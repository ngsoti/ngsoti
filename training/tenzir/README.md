# Module 8: Security data pipelines and detection engineering with Tenzir

This module teaches SOC operators how to acquire, normalize, enrich, and route
security telemetry with [Tenzir](https://tenzir.com), and how to write
detections on top of it. It is the missing link between the sensors of the
NGSOTI training SOC (Kunai, Zeek, Suricata, D4) and its analyst tools (MISP,
Vulnerability-Lookup, OpenSearch, Wazuh, FlowIntel, SkillAegis).

```text
sensors            Tenzir pipelines                         analyst tools
Kunai   ─┐                                                ┌─ OpenSearch / Wazuh
Zeek    ─┼─► acquire ─► normalize (OCSF) ─► enrich ─► detect ─► route ─┼─ FlowIntel
Suricata ┤                      ▲               ▲                    └─ SkillAegis
D4      ─┘                      │               │
                           MISP · Vulnerability-Lookup · hashlookup · Rulezet
```

## Audience and prerequisites

SOC operators, students, and engineers with basic Linux and JSON skills.
Modules 1 to 3 (MISP) and 7 (Kunai) help but are not required. You need no
prior Tenzir knowledge.

## Learning objectives

After this module, you can:

1. Read and parse logs, packet captures, and API responses with pipelines.
2. Normalize events from different sensors to
   [OCSF](https://ocsf.io) and explain why a common schema matters.
3. Correlate endpoint and network telemetry with the Community ID.
4. Turn MISP indicators into live detections, and report sightings back.
5. Enrich alerts with vulnerability context and detection rules.
6. Run [Sigma](https://sigmahq.io) rules on normalized telemetry, and write TQL
   detections where Sigma has no coverage.
7. Route findings to a SIEM, a case management system, and an exercise
   platform.
8. Build and test a Tenzir package that onboards a new data source.

## Agenda (4 hours)

| Time        | Content                                                         | Material |
| :---------- | :-------------------------------------------------------------- | :------- |
| 14:00-14:30 | Tenzir in the NGSOTI architecture, the data lifecycle, TQL      | [Tenzir overview](https://tenzir.com/docs/tutorials/learn-the-data-lifecycle), [idiomatic TQL](https://tenzir.com/docs/tutorials/learn-idiomatic-tql) |
| 14:30-15:15 | **Exercise 1**: Acquire and normalize network telemetry         | [01-network-telemetry.md](exercises/01-network-telemetry.md) |
| 15:15-16:00 | **Exercise 2**: Onboard Kunai and correlate with the network    | [02-kunai-and-community-id.md](exercises/02-kunai-and-community-id.md) |
| 16:00-16:15 | Break                                                           |          |
| 16:15-17:00 | **Exercise 3**: MISP intelligence in your pipelines             | [03-misp-intelligence.md](exercises/03-misp-intelligence.md) |
| 17:00-17:30 | **Exercise 4**: D4 blackhole traffic and vulnerability context  | [04-d4-and-vulnerabilities.md](exercises/04-d4-and-vulnerabilities.md) |
| 17:30-18:00 | **Exercise 5**: Detect and respond                              | [05-detect-and-respond.md](exercises/05-detect-and-respond.md) |
| Homework    | **Capstone**: Validate the Kunai mapping, or onboard a source   | [06-capstone-package.md](exercises/06-capstone-package.md) |

## The scenario

All exercises follow one incident on the host `web-01` on 14 September 2026.
It starts with a single Suricata alert at 08:12:04 UTC: `web-01` (10.0.0.15)
connects to 198.51.100.7 on port 4444, a port that cryptocurrency mining pools
use. A miner on a web server looks like a routine case: kill the process,
reimage the host, and close the ticket.

Don't close it yet. Each exercise answers the question that an analyst would
ask next, and together they show that the miner is the loudest part of the
attack, not its purpose:

| Exercise | The question |
| :------- | :----------- |
| 1 | What else did `web-01` do on the network around the alert? |
| 2 | Which process opened the connection, how did it get there, and as which user? |
| 3 | Did the community already know this campaign, and how fast would its indicators have caught it? |
| 4 | A separate case: what does traffic to unused address space reveal, and which vulnerability matters most? |
| 5 | What did the attacker actually want, and how do a dozen alerts become one case? |

Each sensor sees part of the story:

| Sensor        | What it sees                                            | Data |
| :------------ | :------------------------------------------------------ | :--- |
| Kunai         | Processes, files, sockets, memory, kernel modules       | [`data/kunai`](data/kunai) |
| Suricata/Zeek | DNS, HTTP, flows, and an alert for the mining pool      | [`data/suricata`](data/suricata), [`data/zeek`](data/zeek) |
| MISP          | An event with the indicators of the campaign            | [`data/misp`](data/misp) |
| D4            | Blackhole traffic with a Zyxel CVE-2023-28771 exploit attempt | [`data/d4`](data/d4) |

See [`data/README.md`](data/README.md) for how we built the data. Parts of it
are real API responses from Vulnerability-Lookup, hashlookup, and Rulezet. The
rest is synthetic and uses documentation address ranges (RFC 5737).

## Set up

1. [Install Tenzir](https://tenzir.com/docs/guides/installation).
2. [Install the packages](https://tenzir.com/docs/guides/packages/install-a-package)
   `suricata`, `zeek`, `misp`, and `kunai` from the
   [Tenzir Library](https://github.com/tenzir/library).
3. From this directory, run a pipeline to check your setup:

   ```sh
   tenzir -f tests/01-acquire/kunai.tql
   ```

Pipelines that use contexts (lookup tables and Bloom filters) need a running
node. Exercises 3 and 4 explain how to start one. Pipelines in [`live/`](live)
talk to external services and use placeholder host names.

## Repository layout

| Path                      | Content |
| :------------------------ | :------ |
| [`exercises/`](exercises) | Instructions, one file per exercise |
| [`tests/`](tests)         | Reference solutions with the expected output (`.txt`) next to each `.tql` file |
| [`live/`](live)           | Pipelines for real deployments that need MISP, OpenSearch, and other services |
| [`data/`](data)           | Sample data |

The reference solutions are executable tests. Compare your output with the
`.txt` file next to a solution, or run them all with the
[test framework](https://tenzir.com/docs/reference/test-framework).

## Teaching notes

- The full story, for instructors only: a web shell in `apache2`, running as
  `www-data`, downloads `x.sh`, which drops `.kworker-ng` into `/tmp`. The
  binary mines loudly, which trips the Suricata alert. Then, without any
  `sudo` or `su` in the telemetry, the attacker is `root` and loads the
  Diamorphine rootkit. The miner attaches to `sshd` with `ptrace`, a known way
  to harvest SSH credentials, deletes the dropper, and kills `auditd`. The
  miner is the decoy. A response that reimages `web-01` without rotating the
  credentials that passed through `sshd`, and without finding the privilege
  escalation path on hosts of the same build, misses what the attacker came
  for. Save this reveal for Exercise 5.
- Exercises 1 and 2 teach the vocabulary (events, operators, schemas). Do not
  skip them in a mixed audience.
- Exercise 3 needs a node. Start one on the instructor machine and share it, or
  let every trainee run their own with `tenzir-node`.
- For Exercise 5, trainees can write rules against the live NGSOTI MISP
  instance (`training5.misp-community.org`, see the
  [training README](../README.md)) if you want an exercise with more noise.
- The capstone fits a student project or an internship.

## Acknowledgements

NGSOTI is co-funded by the European Union. Kunai, MISP, D4,
Vulnerability-Lookup, hashlookup, and Rulezet are open-source projects that
NGSOTI partners develop or support.
