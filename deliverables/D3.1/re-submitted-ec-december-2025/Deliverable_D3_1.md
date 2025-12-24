---
tags: ngsoti
subtitle: "NGSOTI Project: 101127921 DIGITAL-ECCC-2022-CYBER-03"
title: "D3.1 - References of training material updates #1"
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
  - \usepackage{graphicx}
  - \usepackage[table]{xcolor}
  - \usepackage{longtable}
  - \usepackage{pdflscape}
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

Words displayed in red indicate hyperlinks. These links can be clicked
or viewed by hovering over the text.

## Distribution and License

The document is distributed under Creative Common Attribution 4.0
International
[CC-BY](https://creativecommons.org/licenses/by/4.0/deed.en).

The document is distributed as TLP:CLEAR.

## Deliverable Definition

The identifier of the deliverable is D3.1 and it adheres to the
definition outlined in the grant agreement written in bold. **A list of
commits of in public training material git repositories.**

## Executive Summary

In the NGOSTI project, new training materials are developed, or existing
ones are updated. Some of these materials are released under an
open-source license, allowing multiple contributors from various
projects to enhance and extend them. This report focuses on NGOSTI
training programs in the following three domains:

- Incident Response
- MISP
- Cryptography

This report includes references to commits in the public training
material repositories.

# Training Material

## Incident Response

[NGOSTI Incident
Reponse](https://github.com/ngsoti/ngsoti/tree/main/training/incident-response/NGSOTI-Incident-Response)
training is tailored to each organization receiving the training. Thus
it includes many sensitive data about their internal working
organization and infrastructure. A generic incident response training
was distilled and released on the NGOSTI project for this deliverable .
It consists in 71 slides of training material.

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
training materials are available on the NGSOTI [Github
repository](https://github.com/ngsoti/ngsoti/tree/main/training/cryptography/NGSOTI-DLH-Cryptography)
for this deliverable. They consist of training slides and three exercise
worksheets for encryption and decryption.

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
MISP-training [GitHub
repository](https://github.com/MISP/misp-training).

## List of commits

The list of commits related to trainings is shown in the table below.
