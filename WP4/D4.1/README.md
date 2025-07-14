# NGSOTI Architecture and Exploitation from Collected Data

## NGSOTI Training SOC – Architecture Overview

![NGOSTI Diagram overview](https://raw.githubusercontent.com/ngsoti/ngsoti/refs/heads/main/deliverables/D4.1/oss-overview.png)

## Purpose

The NGSOTI stack aims to provide trainees with a full-fidelity, open-source Security Operations Center that looks and behaves like a production environment, yet remains safe, fully reproducible, and easy to extend with new scenarios. To ensure authentic telemetry, it re-uses rich datasets from the D4 Project and other real-world sources, giving students hands-on experience with the kinds of signals a mature detection infrastructure generates.

## Layered design and building blocks

| Layer | Key components | Role |
|-------|---------------|------|
| **Trainee & Instructor** | *[SkillAegis](https://github.com/MISP/SkillAegis)* portal, editor & scoreboard | Build scenarios in [Common Exercise Format (CEXF)](https://github.com/MISP/cexf), launch them, track progress and scoring. |
| **Telemetry** | Isolated lab network, **Kunai** eBPF/Suricata sensor for Linux, Zeek, Suricata, Windows Sysmon | Generate realistic endpoint & network events without risking the production network. |
| **Data pipeline and processing** | **Tenzir** pipelines (collect → normalise → enrich → route), **Poppy** Bloom filters | High-speed streaming, live detections, inline threat-enrichment. |
| **Threat & Vulnerability Intelligence** | **[MISP](https://github.com/MISP/MISP)**, **[Vulnerability-Lookup](https://github.com/vulnerability-lookup/vulnerability-lookup/)** | Contextualise artefacts with IOC sightings and CVE metadata. |
| **Storage** | Cheap object storage (NFS, S3 / MinIO) | Long-term retention and replay of raw & processed events. |
| **Analyst tooling** | Wazuh Dashboards, **FlowIntel** case mgmt, MISP workflow hooks | Alert triage, hunting, automated response, reporting. |

## A sample end-to-end data flow

1. **Instructor** creates an exercise in SkillAegis → injects deploy into cyber-range.  
2. **Kunai / Zeek / Suricata** observe activity → stream logs to **Tenzir**.  
3. **Tenzir** normalises events, joins MISP & Vulnerability-Lookup data, checks Poppy Bloom filters, issues detections.  
4. Filtered events go to **SIEM dashboards**; all raw + enriched data land in object storage.  
5. Alerts automatically open **[FlowIntel](https://github.com/flowintel/flowintel)** cases;   
6. **[SkillAegis](https://github.com/MISP/SkillAegis)** pulls case status & flag submissions to update live leader-boards.

