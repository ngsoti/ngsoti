# Capstone: Validate the Kunai mapping, or onboard a new source

**Time:** 4 to 8 hours, as homework or as an internship project.
**Starting point:** the `kunai` package of the
[Tenzir Library](https://github.com/tenzir/library/tree/main/kunai).

## Goal

Contribute to a real open-source package. Pick one of three tracks. A good
contribution has a change, a test, and a short write-up for the maintainers.

## Background

A Tenzir [package](https://tenzir.com/docs/explanations/packages) bundles
operators, pipelines, contexts, and examples. Packages in the
[Tenzir Library](https://github.com/tenzir/library) follow this layout:

```text
kunai/
├── package.yaml
├── operators/ocsf/
│   ├── map.tql             dispatcher: one arm per Kunai event
│   ├── normalize.tql       one call that runs the whole path
│   └── events/*.tql        one leaf per event type
├── examples/*.tql
└── tests/                  inputs and expected output
```

Read [Onboard a data source](https://tenzir.com/docs/tutorials/onboard-a-data-source)
first. It explains why a mapper consumes its input and why `unmapped` exists.

## Track A: Validate the Kunai mapping on real telemetry

The package's test fixtures are synthetic, because Kunai needs eBPF on a real
Linux host. Logs from a real host are the best test of the mapping. You can
provide them.

1. Record Kunai logs. Use the Kunai setup of Module 7, the
   [Kunai sandbox](https://sandbox.kunai.rocks/), or the logs in the NGSOTI
   malware dataset (<https://helga.circl.lu/NGSOTI/malware-dataset>).
2. Run them through the mapping and validate:

   ```tql
   from_file "kunai.log" {
     read_ndjson
   }
   kunai::ocsf::normalize
   ocsf_derive
   ocsf_cast
   ```

   Which events produce warnings? Which events end up as Base Events? Which
   fields stay in `unmapped` on most events, and are they worth mapping?
3. Check the judgment calls in the mapping, such as thread `clone`, the
   signals for `kill`, and `execve_script`. Do they match what you see?
4. Anonymize a small log that shows a problem, and send it with your findings
   as an issue in the Tenzir Library.

## Track B: Detections on Kunai telemetry

1. Write three Sigma rules for Linux behavior that the module data does not
   cover, for example `LD_PRELOAD` hijacking or a reverse shell. Test them on
   the module data and on your own recordings.
2. Find a behavior that Sigma's Linux catalog cannot express, as in
   [`kunai-beyond-sigma.tql`](../tests/06-detect/kunai-beyond-sigma.tql), and
   write the TQL detection.
3. Measure false positives on a clean host. Tune the rules without losing the
   attack.

## Track C: Onboard another source

Pick one of these tasks and build the first version:

| Task | Difficulty | Hint |
| :--- | :--------- | :--- |
| `vulnerability_lookup` package: CVE records to the OCSF vulnerability objects | Medium | Start from `tests/05-vulnerability` |
| `flowintel` package: create a case from a detection | Medium | Use the instance's Swagger UI to verify the API |
| Map Wazuh alerts to OCSF Detection Finding | Medium | Query `wazuh-alerts-*` as in the Wazuh integration |
| OCSF index templates and dashboards for OpenSearch | Easy | One index per OCSF class |
| Load a Poppy Bloom filter into a Tenzir context | Hard | First find out whether the formats are compatible |

Follow the layout above. Add test inputs and expected output with
`uvx tenzir-test`, and make sure that `ocsf_cast` does not warn.

## Assessment

| Criterion | Question |
| :-------- | :------- |
| Correctness | Does `ocsf_cast` accept the output without a warning? |
| Fidelity | Is every source field either mapped or in `unmapped`? |
| Testing | Does the test cover a realistic and an edge-case event? |
| Judgment | Does the write-up cite the OCSF schema for class choices? |
| Collaboration | Did the maintainers receive something they can act on? |
