# Exercise 5: Detect and respond

**Time:** 30 minutes. **Solutions:** [`tests/06-detect`](../tests/06-detect),
[`live/`](../live).

## Goal

Detect the incident with Sigma rules and TQL, and route the findings to the
tools that analysts use.

## Background

Normalization pays off in detection. A rule that names `process.file.path` or
`dst_endpoint.port` works on every source that maps to the matching OCSF class:
Kunai on Linux today, and other sensors tomorrow. Tenzir's
[`sigma`](https://tenzir.com/docs/reference/operators/sigma) operator goes one
step further. It translates the field names of stock Sigma rules (`Image`,
`CommandLine`) to OCSF on the fly, so community rules from
[SigmaHQ](https://github.com/SigmaHQ/sigma) or [Rulezet](https://rulezet.org)
apply to normalized telemetry without a mapping per rule.

Tenzir is the executor of the detection content. The rules stay plain Sigma
files that your detection team owns and versions. A pipeline loads them from a
directory, reloads changes without a restart (`refresh_interval`), and turns
every match into an OCSF Detection Finding. Sigma findings therefore share
routing, storage, and triage with findings from YARA and from your own TQL.

Sigma covers three Linux log source families: process creation, network
connection, and file event. Kunai also reports kernel module loads, memory
changes, and debugger attachment. For those, you write TQL.

## Steps

### Part A: Sigma

1. `data/sigma` has four rules. Read them. Which attacker behaviors do they
   cover? Which `logsource` does each declare?
2. Run [`tests/06-detect/sigma-on-kunai.tql`](../tests/06-detect/sigma-on-kunai.tql):

   ```sh
   tenzir -f tests/06-detect/sigma-on-kunai.tql
   ```

   Which events match? Why does the hidden-file rule fire twice? Which MITRE
   ATT&CK technique does each rule reference?
3. Change the rule `linux-exec-from-tmp.yml` so that it also catches `/run/user/`.
   Does the output change? Add a rule of your own for the web shell: a shell
   that `apache2` spawns.
   Rules in a directory reload while the pipeline runs, so you can test a
   change without restarting it.
   To tune a rule without editing it, add a Sigma global filter document to
   the rule file or to the directory. This one suppresses package installers:

   ```yaml
   title: Ignore package managers
   logsource:
     category: process_creation
     product: linux
   filter:
     rules:
       - 0d3c5e77-2b9a-4c1f-8a64-7e1b9f2a3c40
     selection:
       ParentImage|endswith: '/dpkg'
     condition: not selection
   ```
4. **Use the schema.** The four rules above use Sigma's own field names
   (`Image`, `CommandLine`), which Tenzir translates to OCSF for the
   supported log sources. A rule can also name OCSF fields directly, and then
   it applies to every sensor that maps to the same OCSF class. Open
   [`data/sigma-ocsf`](../data/sigma-ocsf) and run
   [`tests/06-detect/sigma-across-sources.tql`](../tests/06-detect/sigma-across-sources.tql).
   One rule about port 4444 fires on Kunai, an endpoint sensor, and on Zeek, a
   network sensor. Why does the Suricata alert not match? (Hint: look at its
   OCSF class.) Which other rules could you write once for all sensors? Read
   [Check Sigma rule compatibility with OCSF](https://tenzir.com/docs/guides/detect/check-sigma-rule-compatibility-with-ocsf)
   to see which log sources have a translation.

### Part B: Detections without Sigma

5. Open [`kunai-beyond-sigma.tql`](../tests/06-detect/kunai-beyond-sigma.tql).
   It finds the rootkit load, the executable memory, the `ptrace` attach to
   `sshd`, and the attack on `auditd`. It uses
   [`ocsf_class_uid`](https://tenzir.com/docs/reference/functions/ocsf_class_uid)
   instead of magic numbers. Why is that more readable?
6. Write a detection that fires when one process performs three of these
   actions within 10 seconds. Use
   [`summarize`](https://tenzir.com/docs/reference/operators/summarize) with the
   process's unique ID (`actor.process.uid`). Sigma correlation rules are not
   part of the `sigma` operator, so you express them in TQL. See
   [Detect over time windows](https://tenzir.com/docs/guides/detect/detect-over-time-windows)
   and [Create multi-stage detectors](https://tenzir.com/docs/guides/detect/create-multi-stage-detectors).

### Part C: Route and respond

7. Read the pipelines in [`live/`](../live). Each sends findings to one
   destination:

   | Destination    | Pipeline                                                |
   | :------------- | :------------------------------------------------------ |
   | OpenSearch     | [`opensearch.tql`](../live/opensearch.tql)              |
   | Wazuh          | [`wazuh.tql`](../live/wazuh.tql)                        |
   | FlowIntel      | [`flowintel-case.tql`](../live/flowintel-case.tql)      |
   | SkillAegis     | [`skillaegis-webhook.tql`](../live/skillaegis-webhook.tql) |
   | Vulnerability-Lookup | [`vulnerability-lookup-sightings.tql`](../live/vulnerability-lookup-sightings.tql) |

   They connect through *topics*: one pipeline
   [`publish`](https://tenzir.com/docs/reference/operators/publish)es, and the
   others [`subscribe`](https://tenzir.com/docs/reference/operators/subscribe).
   Draw the full flow from `kunai.log` to a FlowIntel case. Which pipelines
   run continuously?
8. Suppose OpenSearch is down for ten minutes. What happens to the events in
   the pipelines that publish to it? What would you change so that no
   detection is lost? (Read
   [Send to destinations](https://tenzir.com/docs/guides/route/send-to-destinations).)

## Questions

- Which of the rules is most likely to fire on a developer's laptop? How do
  you tune it without losing the attack?
- Why does NGSOTI put detection upstream of the SIEM? What does the SIEM do
  better?

## Docs

- [Execute Sigma rules](https://tenzir.com/docs/guides/detect/execute-sigma-rules)
- [Match events with TQL](https://tenzir.com/docs/guides/detect/match-events-with-tql)
- [Model detections in OCSF](https://tenzir.com/docs/guides/detect/model-detections-in-ocsf)
- [Detections](https://tenzir.com/docs/explanations/detections)
- [Sigma integration](https://tenzir.com/integrations/sigma) and
  [YARA integration](https://tenzir.com/integrations/yara)
- [OpenSearch](https://tenzir.com/integrations/opensearch),
  [Wazuh](https://tenzir.com/integrations/wazuh)
