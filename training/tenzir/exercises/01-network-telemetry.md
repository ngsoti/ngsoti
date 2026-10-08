# Exercise 1: Acquire and normalize network telemetry

**Time:** 45 minutes. **Solutions:** [`tests/01-acquire`](../tests/01-acquire).

## Goal

Read Suricata and Zeek logs, normalize both to OCSF, and see what a common
schema buys you.

## Background

Every sensor speaks its own language. Suricata calls a source address
`src_ip`, and Zeek calls it `id.orig_h`. A detection that works on one sensor
does not work on the other. Tenzir pipelines solve this in two steps:

1. **Parse** bytes into events that keep the sensor's field names.
2. **Map** those events to a shared schema. We use
   [OCSF](https://ocsf.io), the Open Cybersecurity Schema Framework.

Read [Normalization](https://tenzir.com/docs/explanations/normalization) to see
why Tenzir maps everything to OCSF first.

## Steps

1. Look at the raw data. `data/suricata/eve.json` has three events, and
   `data/zeek/conn.log` has two connections in Zeek's native TSV format:

   ```sh
   cat data/suricata/eve.json | head -c 600
   cat data/zeek/conn.log
   ```

2. Parse the Suricata log without any mapping:

   ```tql
   from_file "data/suricata/eve.json" {
     read_ndjson
   }
   ```

   Save the pipeline to `suricata.tql` and run `tenzir -f suricata.tql`. Which
   field holds the source address? Which fields does only the `alert` event
   have? ([`from_file`](https://tenzir.com/docs/reference/operators/from_file),
   [`read_ndjson`](https://tenzir.com/docs/reference/operators/read_ndjson))

3. Add the mapping from the `suricata` package of the Tenzir Library, then
   derive the human-readable names and validate against the schema:

   ```tql
   suricata::ocsf::normalize
   ocsf_derive
   ocsf_cast
   ```

   Compare `src_ip` with `src_endpoint.ip`, and notice the new fields
   `class_name` and `activity_name`. Which OCSF class does the `alert` become?
   ([`ocsf_derive`](https://tenzir.com/docs/reference/operators/ocsf_derive),
   [`ocsf_cast`](https://tenzir.com/docs/reference/operators/ocsf_cast))

4. Do the same for Zeek. Read the TSV log with
   [`read_zeek_tsv`](https://tenzir.com/docs/reference/operators/read_zeek_tsv)
   and map it with `zeek::ocsf::normalize`.

   > The `zeek` package expects the connection duration in seconds, but
   > `read_zeek_tsv` yields a duration. Convert it first with
   > `duration = duration?.count_seconds()`.

5. Select the same fields from both sources:

   ```tql
   select time, class_name, src_endpoint.ip, dst_endpoint.ip, dst_endpoint.port
   ```

   You wrote one `select` for two sensors. That is the point of normalization.

## Questions

- Why does `ocsf_cast` warn when a field does not belong to the class? What
  would you do with such fields? (Hint: `unmapped`)
- A new sensor arrives next month. What do you need to write so that all your
  existing detections work on it?

## Docs

- [Onboard a data source](https://tenzir.com/docs/tutorials/onboard-a-data-source)
- [Map to OCSF](https://tenzir.com/docs/guides/normalize/map-to-ocsf)
- [Read and watch files](https://tenzir.com/docs/guides/collect/read-and-watch-files)
- [Suricata integration](https://tenzir.com/integrations/suricata) and
  [Zeek integration](https://tenzir.com/integrations/zeek)
- [OCSF schema browser](https://schema.ocsf.io)
