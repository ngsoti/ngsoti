---
tags: ngsoti
subtitle: "NGSOTI Project: 101127921 DIGITAL-ECCC-2022-CYBER-03   D3.1"
title: "References of training material updates #1"
page-title: "Project: 101127921 
 NGSOTI 
 DIGITAL-ECCC-2022-CYBER-03"
author: [Team CIRCL/NGSOTI]
date: "2024-03-14"
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
# References of training material updates \#1

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

The identifier of the deliverable is D3.1 and it adheres to the
definition outlined in the grant agreement **Public report with key
findings of data collected in NGSOTI such as new discoveries, high level
statistics to attacked schools to use NGSOTI**. The deliverable name is
**References of training material updates \#1** and the overall
objective/alignment is described in the executive summary.

## Executive Summary

In the NGOSTI project, new training materials are developed, or existing
ones are updated. Some of these materials are released under an
open-source license, allowing multiple contributors from various
projects to enhance and extend them. This report focuses on NGOSTI
training programs in the following three domains: • Incident Response •
MISP • Cryptography This report includes references to commits in the
public training material repositories.

# Training Material

## Incident Response

NGOSTI Incident Reponse training is tailored to each organization
receiving the training. Thus it includes many sensitive data about their
internal working organization and infrastructure. A generic training was
distilled and released on the NGOSTI project for this deliverable1 . It
consists in 71 slides of training material.

## Cryptography

Cryptography Concepts - Past and Present was designed and delivered by
Georges Kesseler, IT sysadmin & Course Manager at Digital Learning Hub
of Luxembourg (DLH) as part of the NGOSTI training program for Master
students. The course introduces historical and modern cryptographic
methods, including substitution ciphers (Caesar, Pigpen), polyalphabetic
ciphers (Vigenère), mechanical encryption (Enigma), and contemporary
cryptosystems (RSA, Elliptic Curves, symmetric and asymmetric
encryption). It combines conceptual foundations with practical exercises
on the use of cryptography in securing digital communication. The
training materials are available on the NGSOTI Github repository for
this deliverable2 . They consist of training slides and three exercise
worksheets for enryption and decryption.

## MISP

The MISP open-source training materials have been developed over several
years with support from multiple funding sources and external
contributors. To track which parts were co-funded by NGOSTI, the table
below lists the open-source materials, along with their dates, training
repository names, and commit IDs to retrieve the corresponding content.
MISP plays a central role in NGSOTI by enabling integration with
external sources. Throughout 2024 and early 2025, MISP training has been
expanded to align with NGSOTI objectives. The commits listed under
columns with the repository name ’misp-training’ are taken from the
MISP-training GitHub repository3 .

The list of commits related to trainings is shown in the table below including each commit reference in
the GitHub link:

| Date       | Repository      | Title                                                     | Commit |
|------------|-----------------|-----------------------------------------------------------|--------|
| 2025-03-19 | NGSOTI          | Introduction to Incident Response                         | [#1](https://github.com/NGSOTI/NGSOTI/commit/e4691c02f8598bde45d78b11076a3c72dfbf5a02) |
| 2025-03-20 | NGSOTI          | Cryptography - Past and Present                           | [#2](https://github.com/NGSOTI/NGSOTI/commit/10d49a74ee373709d697d70cd3b285dcb03b7bee) |
| 2024-01-31 | misp-training   | a.7-rest                                                  | [#3](https://github.com/MISP/misp-training/commit/ba9d72ad3ca550528cd6d7b5c63b599f7241f672) |
| 2024-01-31 | misp-training   | a.7-restAPI                                               | [#4](https://github.com/MISP/misp-training/commit/3fca4fdc5c26a468d0c6c03bf8e026cdedff7cb8) |
| 2024-04-12 | misp-training   | cheatsheet                                                | [#5](https://github.com/MISP/misp-training/commit/a84e06b1346c2365a1b5eb39fbbd66e010d08128) |
| 2024-04-12 | misp-training   | MISP Data model overview-with-analyst-data                | [#6](https://github.com/MISP/misp-training/commit/fe271814ee846baf0d527a31cff6e09c09121505) |
| 2024-04-15 | misp-training   | jupyter-notebooks                                         | [#7](https://github.com/MISP/misp-training/commit/f5e5a9cdaeb06f89a8ac0c87289be5aef75bbe15) |
| 2024-05-16 | misp-training   | x.17-eu-attack-community                                  | [#8](https://github.com/MISP/misp-training/commit/0561d3524831dff6a3ae1839045b97252b7f75f0) |
| 2024-05-20 | misp-training   | x.17-eu-attack-community                                  | [#9](https://github.com/MISP/misp-training/commit/861cf9a39a703300d02418633cfd028a451a7cf6) |
| 2024-07-09 | misp-training   | 3.1-misp-modules                                          | [#10](https://github.com/MISP/misp-training/commit/ccc0eaa41ebb0a53fb742b5d353888f2d8263028) |
| 2024-07-10 | misp-training   | a.7-rest-API                                              | [#11](https://github.com/MISP/misp-training/commit/91dcefa17bf800b4d993595db47e10af8b2da485) |
| 2024-08-22 | misp-training   | exercises campaign-targeting-multiple-isacs               | [#12](https://github.com/MISP/misp-training/commit/af5e3218edc21bebd3fe57ede4ec2957ce8c2c76) |
| 2024-09-06 | misp-training   | MISP Collaboration & Sharing - Rapid Fire of Features     | [#13](https://github.com/MISP/misp-training/commit/66d23e28ab747917fcddfa29fb6fa45482169915) |
| 2024-10-02 | misp-training   | c-deployment                                              | [#14](https://github.com/MISP/misp-training/commit/2257fd57087d12624d3fa7e86a8cc9599c6b656f) |
| 2024-11-19 | misp-training   | zz-misp-and-isacs                                         | [#15](https://github.com/MISP/misp-training/commit/5dc38486f64792b8f69455908e17947f1df8cea5) |
| 2024-11-19 | misp-training   | zz-misp-and-isacs                                         | [#16](https://github.com/MISP/misp-training/commit/7b0a37acc97334590c2927794d89f6163deb25a7) |
| 2024-11-20 | misp-training   | zz-misp-and-isacs                                         | [#17](https://github.com/MISP/misp-training/commit/34cc81f8e10ecf7fd0f3e31155e3da270f219512) |

