# Exercise 3: MISP intelligence in your pipelines

**Time:** 45 minutes. **Solutions:** [`tests/03-misp`](../tests/03-misp),
[`live/misp-*.tql`](../live).

## Goal

Use indicators from [MISP](https://www.misp-project.org) to flag events in real
time, and report what you saw back to the community.

## Background

Four flows connect MISP and Tenzir:

| Flow                      | Direction     | Mechanism                                          |
| :------------------------ | :------------ | :------------------------------------------------- |
| Events as OCSF            | MISP → Tenzir | `/events/restSearch` and the `misp` package        |
| Indicators as contexts    | MISP → Tenzir | `/attributes/restSearch` into a lookup table       |
| Live updates              | MISP → Tenzir | [ZeroMQ](https://www.circl.lu/doc/misp/misp-zmq/) pub/sub |
| Sightings                 | Tenzir → MISP | `/sightings/add`                                   |

Tenzir stores indicators in **contexts**: stateful objects that live in a node.
A **lookup table** maps a key to a value and tells you why an indicator
matters. A **Bloom filter** only tells you whether you have seen a key, but it
stays small with millions of indicators. Read
[Enrichment](https://tenzir.com/docs/explanations/enrichment) first.

## Steps

### Part A: MISP events as OCSF

1. `data/misp/restsearch-events.json` is the response of `/events/restSearch`
   for the campaign event. Normalize it with the `misp` package:

   ```tql
   from_file "data/misp/restsearch-events.json" {
     read_json
   }
   unroll response
   this = response.Event
   misp::event::ocsf::normalize
   ocsf_derive
   ocsf_cast
   ```

   The event becomes an OCSF *OSINT Inventory Info* event. Where do the
   indicators go? Which MISP attributes became part of the event, and which
   ones did the mapping drop?

### Part B: Live updates through ZeroMQ

2. MISP publishes `<topic> <json>` messages. `data/misp/zmq-messages.txt`
   holds a recording. Parse it with
   [`tests/03-misp/zeromq-messages.tql`](../tests/03-misp/zeromq-messages.tql).
   Why does the pipeline split at the *first* space only?

3. In production, replace `from_file` with
   [`from_zmq`](https://tenzir.com/docs/reference/operators/from_zmq). Read
   [`live/misp-zeromq.tql`](../live/misp-zeromq.tql). The `prefix` option
   subscribes to one topic at the socket. Which topic carries a deleted
   attribute, and how would you remove it from the lookup table?

### Part C: Indicators as contexts

4. Start a node in a second terminal. Pipelines that use contexts need it:

   ```sh
   tenzir-node
   ```

5. Create the two contexts, then load the indicators from
   `data/misp/attributes.json`:

   ```sh
   for f in tests/03-misp/ioc-contexts/0[123]*.tql; do tenzir -f "$f"; done
   ```

   Read `03-load-iocs.tql`. Why does it filter on `to_ids`? Why does it
   convert IP addresses with `ip(value)`? What happens to the Bloom filter when
   you load a million indicators?

6. Enrich the Kunai telemetry from Exercise 2:

   ```sh
   tenzir -f tests/03-misp/ioc-contexts/04-enrich-kunai.tql
   ```

   Three events match. Which indicator type triggered each one? What does the
   MISP context tell the analyst that the event alone does not?

7. Inspect and clean up:

   ```tql
   context_inspect "misp_iocs"
   ```

### Part D: Sightings

8. Every match is evidence that an indicator is still active. Read
   [`live/misp-sightings.tql`](../live/misp-sightings.tql). Sightings help the
   community age out stale indicators with MISP's
   [decaying models](https://www.misp-project.org/2019/09/12/Decaying-Of-Indicators.html/).
   What privacy considerations apply before you share a sighting?

## Questions

- A Bloom filter has false positives. How would you combine it with the lookup
  table so that you get both speed and exact answers?
- MISP's warninglists mark benign values such as public DNS resolvers. Where in
  these pipelines would you apply them? (Hint: `enforceWarninglist` in
  `live/misp-fetch-attributes.tql`.)

## Docs

- [Use lookup tables](https://tenzir.com/docs/guides/enrich/use-lookup-tables)
- [Enrich with threat intel](https://tenzir.com/docs/guides/enrich/enrich-with-threat-intel)
- [`context_create_lookup_table`](https://tenzir.com/docs/reference/operators/context_create_lookup_table),
  [`context_create_bloom_filter`](https://tenzir.com/docs/reference/operators/context_create_bloom_filter),
  [`context_update`](https://tenzir.com/docs/reference/operators/context_update),
  [`context_enrich`](https://tenzir.com/docs/reference/operators/context_enrich)
- [`from_http`](https://tenzir.com/docs/reference/operators/from_http)
- [MISP automation and API](https://www.circl.lu/doc/misp/automation/)
- [MISP training](../../threat-intelligence/mod1-MISP-CTI/README.md) (Module 1)
