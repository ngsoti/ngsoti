# Cybersecurity Tool for Threat Intelligence and Forensics

![](http://hdoc.csirt-tooling.org/uploads/upload_13235b6957b475dc731b2a392b2fd021.png)

The course **Cybersecurity Tools for Threat Intelligence and Forensics** is composed of 7 modules that cover the main open-source tools (MISP, AIL Project, FlowIntel, and Kunai) developed by CIRCL. It aims to provide a comprehensive toolbox for security analysts, SOC operators, and security professionals.

|Module Number| Description|
|:---|:---|
|1|MISP - CTI|
|2|MISP Automation|
|3|CTI at Large|
|4|AIL Project|
|5|Forensic|
|6|Forensic|
|7|Kunai|

![](http://hdoc.csirt-tooling.org/uploads/upload_b1ded684907729d19396c6f79e8d90cb.png)

https://link.infini.fr/misp-unilu

## Online Evaluation

- [https://hdoc.csirt-tooling.org/m9ebxZkzSwqIiv0MHkofpw?view](https://hdoc.csirt-tooling.org/m9ebxZkzSwqIiv0MHkofpw?view)
## MISP Infrastructure

https://training5.misp-community.org

Login: user[1-9]@org-admin.5.test
Password: uni_lu_training

## Module 1: MISP - CTI

### Agenda

|Time|Description|Slides| 
|:---|:---|:---|
|14:00-15:00|An Introduction to Information Sharing and MISP the Threat Intelligence Platform|[Slide - MISP Introduction](https://raw.githubusercontent.com/ngsoti/ngsoti/main/training/threat-intelligence-mod1/slides/0-misp-introduction-to-information-sharing.pdf)|
|15:00-15:30|MISP Data sharing and models|[Slide - Data Models](https://raw.githubusercontent.com/ngsoti/ngsoti/main/training/threat-intelligence-mod1/slides/MISP%20Data%20model%20overview-with-analyst-data.pdf)|
|16:00-16:30|Best Practices in Threat Intelligence Gather, document, analyse and contextualise intelligence using MISP|[Slide - Best Practices](https://raw.githubusercontent.com/ngsoti/ngsoti/main/training/threat-intelligence-mod1/slides/b.1-best-practices-in-threat-intelligence.pdf)|
|16:30-17:00|MISP 10 Pillars|[Slide - 10 Pillars](https://raw.githubusercontent.com/ngsoti/ngsoti/main/training/threat-intelligence-mod1/slides/MISP%2010%20Pillars.pdf) [Slide - MISP Collaboration and Sharing](https://raw.githubusercontent.com/ngsoti/ngsoti/main/training/threat-intelligence-mod1/slides/MISP%20Collaboration%20%26%20Sharing%20-%20Rapid%20Fire%20of%20Features.pdf)|
|17:00-18:00|Encoding session|[GRU Example](https://www.foo.be/cours/dess-20192020/pub/gru)|

### Documentation
- [Cheatsheet: Concepts & Data model](https://www.misp-project.org/misp-training/cheatsheet.pdf)
- [MISP Data model overview](https://www.misp-project.org/misp-training/MISP%20Data%20model%20overview-with-analyst-data.pdf)
- [Compliance - GDPR](https://www.misp-project.org/compliance/GDPR/)
- [Best Practices in Threat Intelligence](https://www.misp-project.org/best-practices-in-threat-intelligence.html)
- [CIRCL Open Source tools and SOC/CSIRT eco-system](https://hdoc.csirt-tooling.org/rU6m8Y0BQm6a3C8_Gw7apQ#)
- PyMISP: [https://github.com/MISP/PyMISP/](https://github.com/MISP/PyMISP/)
- [OpenAPI documentation](https://www.misp-project.org/documentation/openapi.html)
- [MISP Maltego integration](https://github.com/MISP/MISP-maltego)
- [MISP playbooks](https://github.com/MISP/misp-playbooks)
- [misp-stix](https://github.com/MISP/misp-stix/) a generic library for MISP standard format to STIX (1.1, 1.2, 2.0 and 2.1): [documentation](https://github.com/MISP/misp-stix/tree/main/documentation)
- [misp-galaxy](https://www.misp-galaxy.org)
- [misp-taxonomies](https://www.misp-project.org/taxonomies.html)
- [misp-objects](https://www.misp-project.org/objects.html)
- [Tooling used in a SOC and how it can be integrated](https://hdoc.csirt-tooling.org/rU6m8Y0BQm6a3C8_Gw7apQ#)

### Resources

- [LXC/LXD images](https://images.misp-project.org/) of MISP. [Bridging the Gap: Introducing MISP Airgap for Secure Environments](https://www.misp-project.org/2024/01/12/MISP-airgap.html/).

#### Ransomlook.io - MISP and AIL Integration usage

![](http://hdoc.csirt-tooling.org/uploads/upload_68d38339e306bb60f5b71be47290eec9.png)

- [AIL project](https://github.com/ail-project/ail-framework)
- [AIL feeder Telegram](https://github.com/ail-project/ail-feeder-telegram)
- [ransomlookup source](https://github.com/RansomLook/RansomLook) - [ransomlook.io](https://ransomlook.io)

### **All slide deck**

   - (source file): [https://github.com/MISP/misp-training](https://github.com/MISP/misp-training)
   - (PDF): [https://www.misp-project.org/misp-training/](https://www.misp-project.org/misp-training/)


### Interesting MISP events/examples

- [Targeted phishing - PDF documents / phishkit - YARA tracking](https://training5.misp-community.org/events/view/5cdd3938-7134-4908-9552-173cc0a8016e) - graph, tracking via YARA rules
- [Investigation Syrian Electronic Army Activities ](https://training5.misp-community.org/events/view/c54869a6-0123-405f-b1a0-0ba3cfd759b9) - graph, timeline usage
- [ATM Vulnerabilities Allow Deposit Forgery Attacks](https://training5.misp-community.org/events/view/848a3172-1301-4cbd-8398-435b00904c20) - Galaxy for finance, eventreport
- [Kobalos - Linux threat to high performance computing infrastructure](https://training5.misp-community.org/events/view/83a7add9-76d7-47ef-9f4b-ebd07fbe880d) - EventReport, EventGraph
- [Decaying Example](https://training.misp-community.org/events/view/e6f83d22-248c-4286-91d2-8dd97b637560)
- [Dirty harry example](https://training5.misp-community.org/events/view/339b8437-13e8-4ae6-97dc-47cf909aa78d) - EventGraph, custom objects and timeline

## Module 2 - MISP Automation

### Agenda

|Time|Description|Content|
|:---|:---|:---|
|14:00-15:00|Automation introduction|[Slides - Automation introduction](https://cra.circl.lu/CTI_unilu/automation_intro.pdf)|
|15:00-16:00|MISP API & PyMISP introduction|MISP API introduction [[notebook](https://github.com/ngsoti/ngsoti/blob/main/training/threat-intelligence/mod2-MISP-automation/PyMISP_API_Introduction.ipynb)/[pdf](https://cra.circl.lu/CTI_unilu/PyMISP_API_Introduction.pdf)] - PyMISP overview [[notebook](https://github.com/ngsoti/ngsoti/blob/main/training/threat-intelligence/mod2-MISP-automation/PyMISP_overview.ipynb)/[pdf](https://cra.circl.lu/CTI_unilu/PyMISP_Overview.pdf)]|
|16:00-17:00|API exercise|[Exercise description](https://hdoc.csirt-tooling.org/NRfVrnInTgyvE3g7ag7cAA#)|
|17:00-18:00|PCAP to MISP automation workshop|Hands-on [[notebook](https://github.com/ngsoti/ngsoti/blob/main/training/threat-intelligence/mod2-MISP-automation/Automation_workshop.ipynb)/[pdf](https://cra.circl.lu/CTI_unilu/Automation_workshop.pdf)]|

## Module 3 - CTI at large

- [Slide deck](https://cra.circl.lu/cti-101.pdf)
- [training material](https://github.com/ngsoti/ngsoti/tree/main/training/threat-intelligence/mod3-CTI-at-large)

## Module 4 - AIL Project

- [The Art of Pivoting](https://cra.circl.lu/misp-lea/2-misp-lea-art-of-pivoting.pdf)
- [AIL Introduction](https://cra.circl.lu/misp-lea/3-misp-lea-ail-introduction.pdf)
 
## Module 5 - Module 6 - Forensic Introduction

- https://github.com/neolea/neolea-training-materials/tree/master/Forensic-course-for-university

## Module 7 - Kunai - EDR
- [Kunai training materials](https://github.com/ngsoti/ngsoti/tree/main/training/kunai/teaching-master)

