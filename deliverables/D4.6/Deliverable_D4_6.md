---
tags: ngsoti
subtitle: "NGSOTI Project: 101127921 DIGITAL-ECCC-2022-CYBER-03"
title: "D4.6 - Research Agenda activity report"
page-title: "Project: 101127921 
 NGSOTI 
 DIGITAL-ECCC-2022-CYBER-03"
author: [University of Luxembourg/NGSOTI]
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
  - \usepackage{graphicx}
  - \usepackage[table]{xcolor}
  - \usepackage{longtable}
  - \usepackage{pdflscape}
  - \definecolor{lightgray}{gray}{0.95}
  - \let\OldLongtable\longtable
  - \let\OldEndLongtable\endlongtable
  - \renewenvironment{longtable}{\rowcolors{1}{white}{lightgray}\OldLongtable}{\OldEndLongtable}
---
# Research Agenda activity report

## Disclaimer

Co-funded by the European Union. Views and opinions expressed are
however those of the author(s) only and do not necessarily reflect those
of the European Union or the European Cybersecurity Competence Centre.
Neither the European Union nor the granting authority can be held
responsible for them.

Words displayed in red indicate hyperlinks. These links can be clicked
or viewed by hovering over the text.

## Distribution and License

The document is distributed under Creative Common Attribution 4.0
International
[CC-BY](https://creativecommons.org/licenses/by/4.0/deed.en).

The document is distributed as TLP:CLEAR.

## Deliverable Definition

The identifier of the deliverable is D4.6 and it adheres to the
definition outlined in the grant agreement written in bold. **Research
Agenda activity report**. The description is: **Projects thesis and
internship reports**.

\newpage

## Executive Summary

This report documents the internships and student research activities
hosted by NGSOTI partners over the course of the project, as a concrete
output of the project's Research Agenda. The Restena Foundation, CIRCL,
and the University of Luxembourg hosted or supported students and
trainees on focused technical projects that build directly on NGSOTI
tools and data sources. The activities range from full Master's theses
combining data analysis and machine learning, to short, hands-on
engineering internships producing working prototypes and code
contributions to existing open source tools.

Each activity is reported using the following structure for consistency:
host institution, academic affiliation, period, context and methodology,
and key outcomes. Altogether, the activities reported in the present
report cover four of the project's core technical areas: passive network
traffic analysis for threat intelligence, security scanning of shared
infrastructure, and vulnerability intelligence tooling and machine
learning support for CTI platforms.

\newpage

## Internships and theses hosted by the Restena Foundation

### Lucas Villière: Final destination blackhole! Building Threat intel from trash!

**Host institution:** Restena Foundation\
**Academic affiliation:** Master in Information and Computer Sciences
(MICS), University of Luxembourg\
**Period:** Master's thesis internship, submitted August 2025

**Context and Methodology**

Restena operates a network telescope, a passive sensor capturing all
traffic sent to an inactive IP space. Existing supervised classifiers,
trained on simulated ICS attacks or the labelled CIC-Darknet2020
dataset, do not transfer to this traffic: blackhole captures carry no
reliable ground truth, are mostly noise, and drift over time. The thesis
instead framed the problem as semi-supervised classification. Only UDP
packets were kept, since TCP traffic to an inactive destination never
completes a handshake, and flows were aggregated using a zero-timeout
strategy to produce the flat feature vectors the classifier needs. Each
flow was vectorised into close to 190 dimensions, combining network
metadata with TF-IDF and Singular Value Decomposition (SVD) features
extracted from the payload. Daily capture volumes, several million flows
once vectorised, were processed on the University of Luxembourg's Iris
HPC cluster. A Random Forest classifier was then trained without ground
truth on flows aggregated over 12-hour windows, found to outperform
24-hour aggregation for surfacing anomalous patterns. As a concrete
illustration of the kind of signal the approach surfaces, the intern
traced a recurring UDP stream on port 514 (syslog) to an internal
monitoring system at a European university, leaking `ATK_ATTACK_REPORT`
alerts naming an internal source IP and an external Microsoft
Corporation IP, evidence of a misconfigured logging destination exposed
to the open Internet.

**Key Outcomes**

- **Structured Dataset:** Produced a novel dataset built for
  semi-supervised classification of UDP blackhole traffic.
- **Scalable Pipeline:** Established an HPC-compatible pipeline turning
  millions of raw daily packets into vectorised, \~190-dimension threat
  indicators.
- **Optimised Detection Window:** Trained a Random Forest model showing
  that 12-hour flow aggregation outperforms 24-hour aggregation for
  surfacing anomalous patterns.
- **Actionable Intelligence:** Isolated a concrete misconfiguration,
  tracing a recurring UDP stream on port 514 back to a European
  university leaking internal security alerts onto the open Internet.

### Sam Kafaï: URL Checker Tools for edu.lu

**Host institution:** Restena Foundation\
**Academic affiliation:** 1st year, BTS CyberSecurity, LGK\
**Period:** Two-week internship, autumn 2025

**Context and Methodology**

Restena hosts edu.lu, a URL shortener for the Luxembourgish educational
community built on the open source SURFshort backend. Because shortened
URLs are a common vector for phishing, malware, and command-and-control,
the internship built a security scanning layer that screens a target
before edu.lu trusts it. The result is a modular Python CLI organised
around an inheritance-based `BaseProvider` class: adding a new
threat-intelligence source means writing one class, with no changes to
the CLI itself. A scan runs a target through three stages: provider
reputation queries, content scanning with YARA rules and redirect-chain
inspection, and a unified scoring and confidence calculation. Nine
providers were integrated, each with a documented role: VirusTotal as
the primary broad-coverage check, Google Safe Browsing as narrower but
reliable, WHOIS as a fast baseline, Whalebone for DNS-based
categorisation, MISP for cross-referencing and reporting, local YARA
rules as a fallback when external providers are unreachable, and
Lookyloo and URLScan.io, evaluated but downweighted for being slow and
inconsistent. An automation mode (`--robot`) writes a human-readable log
and a detailed provider-by-provider trace per session, with MISP event
IDs embedded for traceability. Run against the EICAR test file, the tool
scored it 79/100 and MALICIOUS, flagged by MISP, Whalebone, VirusTotal,
and Google Safe Browsing, and automatically opened a corresponding MISP
event.

**Key Outcomes**

- **Functional Security Scanner:** Deployed a working CLI tool
  integrating nine threat-intelligence providers with unified scoring
  and confidence metrics.
- **Automated Threat Reporting:** Demonstrated end-to-end detection on a
  live malicious-URL test case, with automatic MISP event creation.
- **Provider Evaluation:** Produced a documented, evidence-based
  comparison of provider strengths and weaknesses to guide future
  investment in edu.lu's scanning layer.

## Internships and theses hosted by CIRCL

### Thomas Lacroix: Reinforcement and modernization of the MISP open source application

**Host institution:** CIRCL\
**Academic affiliation:** TELECOM Nancy (INP Lorraine, Université de
Lorraine), final-year engineering internship\
**Period:** Academic year 2024-2025

**Context and Methodology**

MISP, the threat intelligence sharing platform underpinning much of
NGSOTI's CTI tooling, needed both feature work on its core application
and a way to verify that instance-to-instance synchronization keeps
working as the codebase evolves. On the application side, the intern
added filtering for extended events on both the index view and the REST
API, added visual distinction and filtering for the two warning-list
categories (false positive versus known identifier), and introduced a
per-role cap on REST API result counts to close a denial-of-service
path. Smaller fixes followed: `Accept`-header fallback for response
format, a `-k` flag on the auto-generated curl command when SSL
validation is skipped, a shortcut to import attributes directly from a
file, and an attribute count added to the galaxy cluster view. On the
synchronization side, CIRCL had no test environment for this: the intern
built a Forgejo Actions pipeline running inside LXC containers, ported
from the project's existing GitHub Actions CI, and a deployment script
that spins up several MISP instances locally and wires them together
according to a specified topology. Using this setup, a seven-instance
test topology was built covering event, sighting, analyst data, and
galaxy cluster propagation via PUSH and PULL, modification and deletion
of synced data, tag and galaxy scoping, distribution-level changes,
sharing groups, and the rules specific to internal (same-organisation)
instances.

**Key Outcomes**

- **Core Code Contributions:** Merged usability, security, and
  traceability improvements into `misp-core`, including extended-event
  filtering, warning-list filtering, and per-role API limits.
- **Automated CI Infrastructure:** Delivered the project's first Forgejo
  Actions-based synchronization test suite, with reusable multi-instance
  deployment tooling.
- **Test Coverage:** Built a seven-instance topology validating the main
  synchronization scenarios across PUSH, PULL, and sharing-group rules.

**Useful links:**

- https://www.misp-project.org/2025/09/19/misp-synchronisation.html/
- Thesis: https://www.misp-project.org/Internship-report_Lacroix.pdf

### Léa Ulusan: Vulnerability intelligence tooling (vulnerability-lookup and VulnTrain)

**Host institution:** CIRCL c/o Luxembourg House of Cybersecurity
**Academic affiliation:** ECE Engineering School **Period:** Internship,
May 2025 to September 2025

**Context and Methodology**

CIRCL maintains
[Vulnerability-Lookup](https://github.com/vulnerability-lookup/vulnerability-lookup),
a platform for searching and correlating CVE, CWE, and related
advisories, and
[VulnTrain](https://github.com/vulnerability-lookup/VulnTrain), a
companion project that trains machine learning models on vulnerability
data. The internship surfaced CWE intelligence directly in
vulnerability-lookup's interface and built a pipeline predicting CWE
categories from free-text vulnerability descriptions. The internship
focused on extending the Vulnerability-Lookup platform with improved
support for CWE-based vulnerability analysis. Vulnerability-Lookup
already provides a structured environment to collect, correlate and
consult vulnerability information from multiple sources. The work
carried out during the internship aimed to enrich this environment by
introducing dedicated Common Weakness Enumeration (CWE) views,
statistics and navigation features. The methodology followed an
incremental development approach: initial templates and backend
blueprints were created, followed by user interface improvements, API
endpoint preparation, statistical visualisations, filtering
capabilities, and enrichment of CWE pages with related CVEs, mitigations
and CAPEC attack-pattern information. The implementation was
progressively refined through fixes, layout changes and typing
improvements to ensure better usability and maintainability.

**Key Outcomes**

**AI models:**

- **Severity classification of vulnerabilities with description written
  in Chinese**: A RoBERTa-Based Model for Automated Vulnerability
  Severity Classification for Chinese. The model is available on
  [Hugging
  Face](https://huggingface.co/CIRCL/vulnerability-severity-classification-chinese-macbert-base).
- **CWE guessing**: Predict CWE categories from Git commit messages and
  vulnerability descriptions. The model is available on [Hugging
  Face](https://huggingface.co/CIRCL/cwe-parent-vulnerability-classification-roberta-base)
  as well as the [dataset with the
  patches](https://huggingface.co/datasets/CIRCL/vulnerability-cwe-patch).

**New features in Vulnerability-Lookup:**

- **CWE functionality:** Added dedicated CWE statistics functionality,
  including visualisation of the most frequent CWE categories and
  time-based filtering capabilities.
- **CVE and CWE visualisation:** Improved the connection between CVEs
  and CWEs by displaying associated CWEs on vulnerability pages and
  linking CWE entries to related CVEs.
- **Enhanced the usability**: of Vulnerability-Lookup through layout
  improvements, filtering options, API endpoint preparation and fixes to
  ensure clearer analyst-facing views.

### Adrian Maraj: Lookyloo -- Phishing Detection

**Host institution:** CIRCL co Luxembourg House of Cybersecurity
**Academic affiliation:** ISFATES **Period:** Internship, March 2024 to
August 2024

**Context and Methodology** CIRCL maintains
[Lookyloo](https://github.com/lookyloo), is a web interface for
capturing a website page and mapping its browsing journey as a tree of
domains, redirects, and external resources, which makes it useful for
phishing and suspicious-website analysis. In the context of phishing
detection, the work aimed to improve how suspicious URLs can be
submitted, captured, categorised and reviewed by analysts. The
methodology followed an incremental software development approach,
starting from practical improvements to the capture and reporting
workflow and progressively adding features that support operational use.
This included work on simplified URL submission, automatic reporting,
private captures, API-based access to categorised captures, and
usability improvements such as downloadable capture trees. These
contributions helped make the phishing-analysis workflow more
accessible, structured and suitable for repeated operational use.

**Key Outcomes** **Workflow improvement** Improved the suspicious URL
submission workflow by contributing to a simplified capture interface
for phishing analysis. **Automatic reporting** Added support for
automatic reporting of captured URLs, helping streamline the handling of
suspicious websites. **Operational control** Enhanced operational
control by supporting private captures and requiring authentication for
selected capture workflows. \*\* Analyst usability\*\* Improved analyst
usability by adding features such as downloadable capture trees and API
access to captures by category.

### Antonia Koch: Lookyloo -- Phishing Detection

**Host institution:** CIRCL co Luxembourg House of Cybersecurity
**Academic affiliation:** ISFATES **Period:** Internship, March 2024 to
August 2024

**Context and Methodology** The internship focused on contributing to
Lookyloo. Antonia's work supported the improvement of analyst-facing
reporting and classification features, making it easier to process
suspicious captures and prepare them for operational review. The
methodology followed an incremental software development approach,
combining frontend improvements, reporting workflow enhancements and
API-related functionality. Contributions included improvements to the
capture page, report form, administrative menus, auto-reporting options,
capture upload via API, quick categorisation features, and WHOIS lookup
handling. These developments helped strengthen the usability and
operational efficiency of Lookyloo for phishing investigation and
suspicious website analysis.

**Key Outcomes** - **Reporting workflow:** Improved the phishing
reporting workflow by enhancing the capture page and making the report
form easier to use. - **Operational features:** Added operational
features for analysts and administrators, including auto-reporting
options and improved admin-only controls. - **Efficiency on Capture
related data:** Contributed to API-based functionality, including
support for uploading captures and handling capture-related data more
efficiently. - **Streamlining enrichment:** Improved classification and
enrichment capabilities through quick categorisation features and better
handling of WHOIS lookup information.

### Théo Geffe: Rulezet

**Host institution:** CIRCL co Luxembourg House of Cybersecurity
**Academic affiliation:** UFR MIM Metz, Université de Lorraine
**Period:** Internship, April 2025 to July 2025

**Context and Methodology** The internship focused on the development of
[Rulezet Core](https://github.com/rulezet/rulezet-core/), an open-source
platform for managing, validating and sharing cybersecurity detection
rules across formats such as YARA, Sigma, Suricata, Zeek, Wazuh and
Elastic. In the context of detection engineering, the work aimed to
strengthen the platform as a collaborative environment where analysts
can import rules, organise them, validate their syntax, search them, and
use them in operational or training contexts. The methodology followed
an incremental software engineering approach, with contributions
implemented directly in the main codebase through iterative commits. The
work covered backend and frontend development, rule management features,
validation workflows, API-related functionality, documentation,
configuration, and general maintainability improvements. This helped
consolidate Rulezet as a practical tool for detection-rule lifecycle
management and cybersecurity training.

**Key Outcomes**

- **Detection-rule management:** Extended Rulezet's detection-rule
  management capabilities, supporting the handling of multiple rule
  formats used by SOC analysts and detection engineers.
- **Rule validation:** Improved rule validation and import workflows,
  helping users assess the quality and usability of detection rules
  before operational use.
- **Usability:** Contributed to platform usability and maintainability
  through frontend refinements, backend improvements, configuration
  updates and documentation work.
- **Collaborative Detection Engineering:** Strengthened Rulezet as an
  open-source cybersecurity platform suitable for collaborative
  detection engineering, rule sharing, and training activities.

## Thesis supported by the University of Luxembourg

### Njomza Rexhepi: Understanding Network Telescope Traffic Through Exploratory Data Analysis and Transformer-Based Representation Learning

**Host institution:** University of Luxembourg, Interdisciplinary Centre
for Security, Reliability and Trust (SnT)\
**Academic affiliation:** Master's programme, Faculty of Electrical and
Computer Engineering, University of Prishtina, with support from LuxDev
(Luxembourg Development Agency)\
**Period:** Master's thesis in progress, expected submission September
2026 \[Ongoing\]

**Context and Methodology** This thesis investigates how
transformer-based representation learning, the approach behind models
like ET-BERT for encrypted traffic, can be adapted to a setting it was
not designed for: passive network telescope data, where traffic is
unsolicited, single-direction, and largely without application content.
The intern built a full-year extraction pipeline from raw telescope
captures and used it to characterize two traffic types reaching the
telescope, Telnet scanning and DNS queries, as a basis for understanding
what these models would actually be learning from. That characterization
produced a more nuanced picture than initially assumed: known botnet
signatures (Mirai-family) are present but represent a minority of
scanning traffic, and DNS traffic at the telescope is dominated by
misdirected legitimate queries rather than attacks, with malicious
activity forming a smaller, distinct subset. An exploratory look at
whether the same sources show correlated TCP and DNS activity over time
surfaced candidate patterns worth following up, though not yet
conclusive evidence of coordinated behavior. Because the dominant
traffic carries no payload to tokenize in the way ET-BERT originally
assumes, the intern is now adapting that architecture's tokenization and
pretraining approach to work directly from packet header structure
instead.

**Key Outcomes**

- **Data Extraction Pipeline:** Delivered a cluster-scale pipeline
  converting a full year of raw telescope PCAPs into a structured,
  schema-consistent Parquet dataset, with totals cross-validated across
  multiple independent analysis scripts.
- **Behavioral Characterization:** Produced a full-year profiling of
  Telnet scanning activity and DNS query composition, distinguishing the
  minority of traffic attributable to known botnet/scanner tooling from
  the majority that is either unattributed or, in the DNS channel,
  benign misdirected traffic.
- **Cross-Protocol Exploration:** Identified candidate IP cohorts and
  subnets showing temporal co-activity across the TCP and DNS channels,
  currently framed as exploratory findings pending statistical and
  threat-intelligence validation.
- **Architectural Adaptation:** Established structure-based tokenizers
  and an ET-BERT adaptation for single-sided, payload-sparse traffic,
  with pretraining and evaluation ongoing ahead of the September 2026
  submission.

## Appendix

### Internships documents and outcomes

- Lucas Villière, *Final destination blackhole! Building Threat intel
  from trash!*, Master's thesis, University of Luxembourg, August 2025.
- Sam Kafaï, Cédric Renzi, Denim Latic, *URL Checker Tools for edu.lu
  (NGSOTI project)*, presentation, OpenSource Conference 2025.
- Thomas Lacroix, *Reinforcement and modernization of the MISP open
  source application*, Master's thesis, TELECOM Nancy, 2025,
  https://www.misp-project.org/Internship-report_Lacroix.pdf.
- CIRCL, internship records for the vulnerability-lookup/VulnTrain
  activity.
- Njomza Rexhepi, *Understanding Network Telescope Traffic Through
  Exploratory Data Analysis and Transformer-Based Representation
  Learning*, Master's thesis (in progress), University of Prishtina,
  with support from the University of Luxembourg (SnT) and LuxDev,
  expected September 2026.
- Léa Ulusan. *Vulnerability Management and Threat Intelligence*, ECE
  Engineering School, Git commits during internship, 2025,
  https://github.com/vulnerability-lookup/vulnerability-lookup/commits?author=hamartia.eu%40gmail.com
- Adrian Maraj, \*\* Lookyloo Phishing Detection. ISFATES, Git commits
  during internship, 2024,
  https://github.com/Lookyloo/lookyloo/commits?author=adrima01
- Antonia Koch, \*\* Lookyloo Phishing Detection. ISFATES, Git commits
  during internship, 2024,
  https://github.com/Lookyloo/lookyloo/commits?author=AntoniaBK
- Théo Geffe, **Rulezet**,UFR MIM Metz, Université de Lorraine, Git
  commits during internship,
  2025,https://github.com/rulezet/rulezet-core/commits/main/?since=2025-04-07&until=2025-07-25&author=ecrou-exact
- Presentation at BSides Luxembourg from Léa Ulusan, 2025-06-19, "[When
  Data Talks, We Let AI
  Listen](https://pretalx.com/bsidesluxembourg-2025/talk/EHAWQX)"
