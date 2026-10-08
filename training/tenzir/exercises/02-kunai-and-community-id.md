# Exercise 2: Onboard Kunai and correlate with the network

**Time:** 45 minutes. **Solutions:** [`tests/01-acquire/kunai.tql`](../tests/01-acquire/kunai.tql),
[`tests/02-correlate`](../tests/02-correlate).

## Goal

Normalize endpoint telemetry from [Kunai](https://github.com/kunai-project/kunai)
and connect it to the network view through the Community ID.

## Background

Kunai is the Linux counterpart to Sysmon. It uses eBPF to report process
execution, network connections, DNS queries, file activity, memory changes, and
kernel module loads, in chronological order. Kunai writes one JSON object per
line. Each object has a `data` section that depends on the event type and an
`info` section that is the same for all events. See the
[event reference](https://why.kunai.rocks/docs/events/).

The `kunai` package of the [Tenzir Library](https://github.com/tenzir/library)
maps Kunai events to OCSF 1.9. See the
[Kunai integration](https://tenzir.com/integrations/kunai) for how to collect
the events. The mapping produces these OCSF classes:

- Process Activity, File System Activity, Network Activity, DNS Activity
- Kernel Extension Activity, Module Activity, Memory Activity, Kernel Activity
- Application Lifecycle, Application Error
- Detection Finding, for events that Kunai's own rules matched

Unknown events become OCSF Base Events, and everything the mapping does not
consume stays in `unmapped`. Two operators matter for you:
`kunai::ocsf::normalize` normalizes a stream of events, and
`kunai::ocsf::map` maps one structured event and consumes it.

## Steps

1. Read the Kunai events of the incident and count them by type:

   ```tql
   from_file "data/kunai/dropper.jsonl" {
     read_ndjson
   }
   summarize event=info.event.name, count=count()
   sort -count
   ```

2. Normalize them with the package:

   ```tql
   from_file "data/kunai/dropper.jsonl" {
     read_ndjson
   }
   kunai::ocsf::normalize
   ocsf_derive
   ocsf_cast
   select time, class_name, activity_name,
          process=process?.name?.otherwise(actor.process?.name?),
          cmd=process?.cmd_line?.otherwise(actor.process?.cmd_line?)
   ```

   Read the incident from top to bottom. What did the web shell do?

3. Look at one `execve` event in full. Where does the process hash live? Which
   process is the `actor`, and which one is the `process`? Compare with
   Kunai's `task` and `parent_task`. After an `execve`, the task already runs
   the new program.

4. Read the mapper in the
   [`kunai` package](https://github.com/tenzir/library/tree/main/kunai)
   (`operators/ocsf/map.tql` and `operators/ocsf/events/execve.tql`).
   The mapper consumes the source event, so what it cannot map becomes
   `unmapped`. Find an event with a non-empty `unmapped`.

5. **Correlate.** Kunai reports the Community ID for every connection, and
   Zeek and Suricata do too. Read all three sources, normalize them, and group
   by Community ID:

   ```tql
   from_file "data/kunai/dropper.jsonl" { read_ndjson }
   kunai::ocsf::normalize
   merge {
     from_file "data/suricata/eve.json" { read_ndjson }
     suricata::ocsf::normalize
   }
   merge {
     from_file "data/zeek/conn.log" { read_zeek_tsv }
     duration = duration?.count_seconds()
     zeek::ocsf::normalize
   }
   ```

   Continue with `ocsf_derive`, `ocsf_cast`, and a `summarize` by the Community
   ID. [`tests/02-correlate/community-id.tql`](../tests/02-correlate/community-id.tql)
   has the full pipeline. Which process opened the connection that Suricata
   flagged? What did Zeek see about the same flow?

## Questions

- What would break if Kunai and Zeek computed the Community ID with different
  seeds?
- Kunai reports a connection when the process calls `connect`. Zeek reports it
  when the flow ends. Why do they carry different timestamps?

## Docs

- [Kunai integration](https://tenzir.com/integrations/kunai)
- [Kunai documentation](https://why.kunai.rocks/docs/quickstart)
- [`community_id`](https://tenzir.com/docs/reference/functions/community_id)
  and the [Community ID spec](https://github.com/corelight/community-id-spec)
- [`merge`](https://tenzir.com/docs/reference/operators/merge) and
  [`summarize`](https://tenzir.com/docs/reference/operators/summarize)
- [Split and merge streams](https://tenzir.com/docs/guides/route/split-and-merge-streams)
- [Onboard a data source](https://tenzir.com/docs/tutorials/onboard-a-data-source)
