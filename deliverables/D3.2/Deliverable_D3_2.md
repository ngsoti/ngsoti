---
tags: ngsoti
subtitle: "NGSOTI Project: 101127921 DIGITAL-ECCC-2022-CYBER-03   D3.2"
title: "References of training material updates #2"
page-title: "Project: 101127921 NGSOTI DIGITAL-ECCC-2022-CYBER-03"
author: [Team CIRCL/NGSOTI]
date: "2026-06-30"
tlp: "CLEAR"
keywords: [threat intelligence, blackhole analysis, snmp]
titlepage: true
titlepage-logo-c: "./eu_template/logo-eufunded_eccc.png"
titlepage-logo-a: "./eu_template/logo-ngsoti.png"
titlepage-logo-b: "./eu_template/logo-partners.png"
logo-width: "35mm"
logo-widthb: "126mm"
logo-widthc: "126mm"
footnotes-pretty: true
toc-own-page: true
colorlinks: true
lof: false
header-includes:
  - \usepackage[table]{xcolor}
  - \usepackage{pdflscape}
  - \usepackage{booktabs}
  - \usepackage{longtable}
  - \usepackage{graphicx}
  - \definecolor{lightgray}{gray}{0.95}
  - \let\OldLongtable\longtable
  - \let\OldEndLongtable\endlongtable
  - \renewenvironment{longtable}{\rowcolors{1}{white}{lightgray}\OldLongtable}{\OldEndLongtable}
---
# References of training material updates #2

## Disclaimer

Co-funded by the European Union. Views and opinions expressed are
however those of the author(s) only and do not necessarily reflect those
of the European Union or the European Cybersecurity Competence Centre.
Neither the European Union nor the granting authority can be held
responsible for them.

## Distribution and License

The document is distributed under Creative Common Attribution 4.0
International
[CC-BY](https://creativecommons.org/licenses/by/4.0/deed.en).

The document is distributed as TLP:CLEAR.

## Deliverable Definition

The identifier of the deliverable is D3.2 and it adheres to the
definition outlined in the grant agreement **A list of commits of in
public training material git repositories**. The deliverable name is
**References of training material updates #2**.

## Executive Summary

In the NGSOTI project, new training materials are developed, or existing
ones are updated. Some of these materials are released under an
open-source license, allowing multiple contributors from various
projects to enhance and extend them. This report focuses on NGSOTI
training programs in the following four domains:

- MISP
- AIL
- NEO-LEA
- NGSOTI (expert invited lectures)

In addition, Tenzir contributes cross-cutting training material on
detection engineering, presented in a separate section at the end of
this report.

This report includes references to commits in the public training
material repositories. For the Tenzir material, the URLs are provided.
The text in red includes hyperlinks that can be accessed online for
verification. This deliverable is a follow-up to Deliverable D3.1. It
covers the period from 1 April 2025 to 30 June 2026.

# Training Material

## MISP

The MISP open-source training materials have been developed over several
years with support from multiple funding sources and external
contributors. To track which parts were related to NGSOTI, the table
below lists the open-source materials, along with their dates, training
repository names, and commit IDs to retrieve the corresponding content.
MISP plays a central role in NGSOTI by enabling integration with
external sources. Throughout end of 2024 and early 2026, MISP training
has been expanded to align with NGSOTI objectives. The commits listed
under columns with the repository name 'misp-training' are taken from
the [MISP-training](https://github.com/misp/misp-training) GitHub
repository.

The list of commits related to trainings is shown in the table below
including each commit reference in the GitHub link:

\begingroup
\small
\setlength{\tabcolsep}{4pt}
\begin{center}
\begin{tabular}{p{2.2cm}p{2.8cm}p{8.1cm}p{1.2cm}}
\hline
Date & Repository & Title & Commit \\
\hline
2025-04-16 & misp-training & chg: [b.6-automation] improvements & \href{https://github.com/MISP/misp-training/commit/04da244}{\#3} \\
2025-04-25 & misp-training & new: [events:FIRSTCTI25\_MISP\_automation\_workshop] Added slides, custom modules and SkillAegis scenario & \href{https://github.com/MISP/misp-training/commit/469a3f1}{\#4} \\
2025-05-02 & misp-training & new: [c.0-current-state] Added slide deck & \href{https://github.com/MISP/misp-training/commit/c6b8f97}{\#5} \\
2025-05-05 & misp-training & add: [complementary] Updated Data model overview in light color mode & \href{https://github.com/MISP/misp-training/commit/3da0404}{\#6} \\
2025-05-26 & misp-training & - chg [NATO MUG] presentation added & \href{https://github.com/MISP/misp-training/commit/9f00da0}{\#7} \\
2025-12-16 & misp-training & Human Operated Ransomware pivot exercise & \href{https://github.com/MISP/misp-training/commit/637028b}{\#8} \\
2026-03-30 & misp-training & new: [election-guidelines] First draft & \href{https://github.com/MISP/misp-training/commit/e428233}{\#9} \\
2026-03-30 & misp-training & chg: [election-guidelines] Added AIL & \href{https://github.com/MISP/misp-training/commit/ce0a6ed}{\#10} \\
2026-03-30 & misp-training & chg: [election-guideline] Added pdf & \href{https://github.com/MISP/misp-training/commit/c7ddcbd}{\#11} \\
\hline
\end{tabular}
\end{center}
\endgroup

## AIL

AIL training material is maintained as an open-source resource and is
regularly adjusted to reflect practical needs observed in information
leak analysis, OSINT investigation, and threat intelligence workflows.
For D3.2, the table below identifies the relevant updates supported by
NGSOTI, with dates, repository references, short change descriptions,
and commit links that allow the corresponding material to be retrieved.
AIL supports NGSOTI activities by helping analysts collect, process, and
pivot across information leaks and related datasets. Between the end of
2024 and early 2026, the training content was updated with revised
introductions, new pivoting material, refreshed screenshots, migration
work, and updated links. The commit references listed for the
`ail-training` repository come from the public AIL-training GitHub
repository.

The list of commits related to trainings is shown in the table below,
including each commit reference in the GitHub link:

\begingroup
\scriptsize
\renewcommand{\arraystretch}{0.9}
\setlength{\tabcolsep}{3pt}
\begin{center}
\begin{tabular}{p{2.0cm}p{2.5cm}p{8.3cm}p{1.1cm}}
\hline
Date & Repository & Title & Commit \\
\hline
2024-12-10 & ail-training & chg: [ail-intro] features updated & \href{https://github.com/ail-project/ail-training/commit/956e7e2}{\#1} \\
2024-12-10 & ail-training & chg: [quick intro] updated & \href{https://github.com/ail-project/ail-training/commit/1aa32b7}{\#2} \\
2025-02-03 & ail-training & chg: [update] small update & \href{https://github.com/ail-project/ail-training/commit/e5a1244}{\#3} \\
2026-02-03 & ail-training & chg: [introduction] update ail-internal & \href{https://github.com/ail-project/ail-training/commit/65c8972}{\#5} \\
2025-05-22 & ail-training & chg: [short intro] update & \href{https://github.com/ail-project/ail-training/commit/8feffa9}{\#6} \\
2025-05-22 & ail-training & chg: [short intro] add search screenshot & \href{https://github.com/ail-project/ail-training/commit/196cbaf}{\#7} \\
2025-05-22 & ail-training & chg: [short] image missing & \href{https://github.com/ail-project/ail-training/commit/61a81a5}{\#8} \\
2025-05-22 & ail-training & chg: [training] ail-training updated & \href{https://github.com/ail-project/ail-training/commit/6899f8b}{\#9} \\
2025-07-16 & ail-training & chg: [short intro] migate latex & \href{https://github.com/ail-project/ail-training/commit/90df6d9}{\#10} \\
2025-07-16 & ail-training & chg: [ail short intro] add ail internal & \href{https://github.com/ail-project/ail-training/commit/754d9f5}{\#11} \\
2025-07-16 & ail-training & chg: [ail short intro] remove old slide & \href{https://github.com/ail-project/ail-training/commit/560fae8}{\#12} \\
2025-07-16 & ail-training & chg: [intro] migrate latex & \href{https://github.com/ail-project/ail-training/commit/5bcad57}{\#13} \\
2025-07-16 & ail-training & chg: [intro] improve feeders section & \href{https://github.com/ail-project/ail-training/commit/4fac428}{\#14} \\
2025-07-17 & ail-training & new: [Art of Pivoting] New version added for VSS 2025 training & \href{https://github.com/ail-project/ail-training/commit/73376fb}{\#15} \\
2025-07-17 & ail-training & chg: [short intro] add onion lookup + images descriptions & \href{https://github.com/ail-project/ail-training/commit/74350f9}{\#16} \\
2025-07-17 & ail-training & chg: [short intro] Ongoing Developments & \href{https://github.com/ail-project/ail-training/commit/f919281}{\#17} \\
2026-02-24 & ail-training & chg: [short intro] update & \href{https://github.com/ail-project/ail-training/commit/53b24b1}{\#18} \\
2026-04-21 & ail-training & chg: [intro] update AIL intro & \href{https://github.com/ail-project/ail-training/commit/a2be145}{\#19} \\
2026-04-21 & ail-training & chg: [intro] rename short intro to ail-introduction & \href{https://github.com/ail-project/ail-training/commit/9b406db}{\#20} \\
\hline
\end{tabular}
\end{center}
\endgroup

## NEO-LEA updates

The NEO-LEA repository contains training material focusing on artefact
analysis using forensic techniques used by SOC operators as well as law
enforcement agencies. Analysis techniques for digital signatures have
been added to strengthen capabilities for verifying and analysing
digitally signed documents.

\begingroup
\scriptsize
\renewcommand{\arraystretch}{0.9}
\setlength{\tabcolsep}{3pt}
\begin{center}
\begin{tabular}{p{2.0cm}p{2.5cm}p{8.3cm}p{1.1cm}}
\hline
Date & Repository & Title & Commit \\
\hline
2026-05-29 & e.200-dfir-pdf-analysis & added pdfsig tool to handle electronic signatures  & \href{https://github.com/neolea/neolea-training-materials/commit/03484f75d3f413ac12617897ae572df153a0c11c}{\#1} \\
2026-05-29 & e.200-dfir-pdf-analysis &  added script to extract tool indicators that were used to sign & \href{https://github.com/neolea/neolea-training-materials/commit/9a862738fbfb6cfe0b2771f6c5310e941690af03}{\#2}\\
2026-05-29 & e.200-dfir-pdf-analysis &  get indications on the tools that were used to sign  & \href{https://github.com/neolea/neolea-training-materials/commit/09f913b39e2eec02c7d13e662a7969097769f4d3}{\#3}\\
2026-05-29 & e.200-dfir-pdf-analysis &  added example output of pdfsig  & \href{https://github.com/neolea/neolea-training-materials/commit/7e3943f3663d93ce36386e00f138d580c63915b1}{\#4}\\
2026-05-29 & e.200-dfir-pdf-analysis &  added example of openssl  & \href{https://github.com/neolea/neolea-training-materials/commit/02bc87f50168d7d749f4b6912e42281174b59f7a}{\#5}\\

\end{tabular}
\end{center}
\endgroup

## NGSOTI: Expert lectures

The NGSOTI repository hosts training materials developed within the
project and contributed directly by the partners. Three modules were
added covering complementary aspects of network security and digital
trust infrastructure, and cybersecurity awareness through game-based
learning. The first addresses DNS as an attack vector, covering
look-alike domains, data exfiltration over DNS, and tunneling tools such
as iodine and dnscat2, with hands-on detection using Wireshark, Snort,
and Suricata signatures. The second examines long-term digital
preservation from a cybersecurity perspective, grounding the topic in
CEN/TS 18170:2025 and eIDAS 2: it covers the OAIS model (SIP/AIP/DIP),
integrity chains, cryptographic audit trails, storage security
requirements, and the implications of algorithm lifecycle management for
archiving trust service providers. The third covers how to design
educational games by defining a target audience, vulgarisation
techniques such as analogy, storytelling, and visualisation, building a
gameplay loop, choosing game elements, and running playtest iterations,
illustrated with a phishing-detection email exercise and a survey of
existing cybersecurity games across board, digital, and simulation
formats.

\begingroup
\scriptsize
\renewcommand{\arraystretch}{0.9}
\setlength{\tabcolsep}{3pt}
\begin{center}
\begin{tabular}{p{2.0cm}p{2.5cm}p{8.3cm}p{1.1cm}}
\hline
Date & Repository & Title & Commit \\
\hline
2026-06-26 & training/UNILU-NC3-Cybersecurity-and-Games & Add [cyber-and-games] NC3 CyberAndGames BenjaminJoly-OmarRamadan & \href{https://github.com/ngsoti/ngsoti/commit/292efed}{\#3} \\
2026-06-16 & training/UNILU-Network-Security & Add [network-security-dns] Network Security Wireshark & \href{https://github.com/ngsoti/ngsoti/commit/81ab6fb}{\#1} \\
2026-06-16 & training/UNILU-Electronic-Archiving & Add [electronic-archiving] National Archives cybersecurity perspective & \href{https://github.com/ngsoti/ngsoti/commit/49db9b6}{\#2} \\
\hline
\end{tabular}
\end{center}
\endgroup

## Tenzir

Tenzir contributes open-source training material on detection
engineering, published as documentation at https://docs.tenzir.com
(source repository: https://github.com/tenzir/docs). Within NGSOTI,
Tenzir sits upstream of the SIEM to acquire, normalize, and route
high-volume security telemetry, with a focus on analytical workloads
such as detection engineering and threat hunting.

The material teaches SOC operators to express detections in the Tenzir
Query Language (TQL) over real data. A key part is normalization:
mapping raw events into common security data models such as OCSF, ECS,
Microsoft Sentinel ASIM, Splunk CIM, and Google SecOps UDM, so that
detections run against consistent, schema-aligned fields. On that
normalized data, the material progresses from simple threshold rules to
statistical anomaly detection. The relevant guides are:

- Map to OCSF --- normalize events into the Open Cybersecurity Schema
  Framework: https://docs.tenzir.com/guides/normalization/map-to-ocsf
- Aggregate event streams --- windowed threshold and statistical
  detections, including brute-force login detection and a traffic-spike
  detector (mean + 2 standard deviations) adapted from the open-source
  Splunk security_content project:
  https://docs.tenzir.com/guides/analytics/aggregate-event-streams
- Execute Sigma rules --- running Sigma signatures on normalized
  telemetry:
  https://docs.tenzir.com/guides/enrichment/execute-sigma-rules
- Enrich with threat intelligence --- intel-driven detection using
  lookup tables and indicators of compromise (e.g. from MISP):
  https://docs.tenzir.com/guides/enrichment/enrich-with-threat-intel
