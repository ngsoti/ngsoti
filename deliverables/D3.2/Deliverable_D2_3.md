---
tags: ngsoti
subtitle: "NGSOTI Project: 101127921 DIGITAL-ECCC-2022-CYBER-03   D3.2"
title: "References of training material updates #2"
page-title: "Project: 101127921 NGSOTI DIGITAL-ECCC-2022-CYBER-03"
author: [Team CIRCL/NGSOTI]
date: "2024-06-03"
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
definition outlined in the grant agreement **Public report with key
findings of data collected in NGSOTI such as new discoveries, high level
statistics to attacked schools to use NGSOTI**. The deliverable name is
**References of training material updates #2** and the overall
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

## MISP

The MISP open-source training materials have been developed over several
years with support from multiple funding sources and external
contributors. To track which parts were co-funded by NGOSTI, the table
below lists the open-source materials, along with their dates, training
repository names, and commit IDs to retrieve the corresponding content.
MISP plays a central role in NGSOTI by enabling integration with
external sources. Throughout end of 2024 and early 2026, MISP training
has been expanded to align with NGSOTI objectives. The commits listed
under columns with the repository name 'misp-training' are taken from
the MISP-training GitHub repository.

The list of commits related to trainings is shown in the table below
including each commit reference in the GitHub link:

  ------------------------------------------------------------------------------------------------------------------------------------------------
  Date             Repository       Title                                                                                                   Commit
  ---------------- ---------------- ------------------------------------------------ -------------------------------------------------------------
  2024-12-12       misp-training    chg: \[exercises/spearphishing-exercise\] fixed     [#1](https://github.com/MISP/misp-training/commit/f844a18)
                                    url of samples                                   

  2024-12-15       misp-training    chg: \[exercises/flubot-exercise\] fixed sample     [#2](https://github.com/MISP/misp-training/commit/ccf380c)
                                    url                                              

  2025-04-16       misp-training    chg: \[b.6-automation\] improvements                [#3](https://github.com/MISP/misp-training/commit/04da244)

  2025-04-25       misp-training    new:                                                [#4](https://github.com/MISP/misp-training/commit/469a3f1)
                                    \[events:FIRSTCTI25_MISP_automation_workshop\]   
                                    Added slides, custom modules and SkillAegis      
                                    scenario                                         

  2025-05-02       misp-training    new: \[c.0-current-state\] Added slide deck         [#5](https://github.com/MISP/misp-training/commit/c6b8f97)

  2025-05-05       misp-training    add: \[complementary\] Updated Data model           [#6](https://github.com/MISP/misp-training/commit/3da0404)
                                    overview in light color mode                     

  2025-05-26       misp-training    \- chg \[NATO MUG\] presentation added              [#7](https://github.com/MISP/misp-training/commit/9f00da0)

  2025-12-16       misp-training    Human Operated Ransomware pivot exercice            [#8](https://github.com/MISP/misp-training/commit/637028b)

  2026-03-30       misp-training    new: \[election-guidelines\] First draft            [#9](https://github.com/MISP/misp-training/commit/e428233)

  2026-03-30       misp-training    chg: \[election-guidelines\] Added AIL             [#10](https://github.com/MISP/misp-training/commit/ce0a6ed)

  2026-03-30       misp-training    chg: \[election-guideline\] Added pdf              [#11](https://github.com/MISP/misp-training/commit/c7ddcbd)
  ------------------------------------------------------------------------------------------------------------------------------------------------

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

  ------------------------------------------------------------------------------------------------------------------------
  Date             Repository       Title                                                                           Commit
  ---------------- ---------------- ------------------ -------------------------------------------------------------------
  2024-12-10       ail-training     chg: \[ail-intro\]    [#1](https://github.com/ail-project/ail-training/commit/956e7e2)
                                    features updated   

  2024-12-10       ail-training     chg: \[quick          [#2](https://github.com/ail-project/ail-training/commit/1aa32b7)
                                    intro\] updated    

  2025-02-03       ail-training     chg: \[update\]       [#3](https://github.com/ail-project/ail-training/commit/e5a1244)
                                    small update       

  2025-02-03       ail-training     chg: \[update         [#4](https://github.com/ail-project/ail-training/commit/4d283c0)
                                    introduction\] add 
                                    ocr + qrcodes +    
                                    barcodes           

  2025-02-03       ail-training     chg:                  [#5](https://github.com/ail-project/ail-training/commit/65c8972)
                                    \[introduction\]   
                                    update             
                                    ail-internal       

  2025-05-22       ail-training     chg: \[short          [#6](https://github.com/ail-project/ail-training/commit/8feffa9)
                                    intro\] update     

  2025-05-22       ail-training     chg: \[short          [#7](https://github.com/ail-project/ail-training/commit/196cbaf)
                                    intro\] add search 
                                    screenshot         

  2025-05-22       ail-training     chg: \[short\]        [#8](https://github.com/ail-project/ail-training/commit/61a81a5)
                                    image missing      

  2025-05-22       ail-training     chg: \[training\]     [#9](https://github.com/ail-project/ail-training/commit/6899f8b)
                                    ail-training       
                                    updated            

  2025-07-16       ail-training     chg: \[short         [#10](https://github.com/ail-project/ail-training/commit/90df6d9)
                                    intro\] migate     
                                    latex              

  2025-07-16       ail-training     chg: \[ail short     [#11](https://github.com/ail-project/ail-training/commit/754d9f5)
                                    intro\] add ail    
                                    internal           

  2025-07-16       ail-training     chg: \[ail short     [#12](https://github.com/ail-project/ail-training/commit/560fae8)
                                    intro\] remove old 
                                    slide              

  2025-07-16       ail-training     chg: \[intro\]       [#13](https://github.com/ail-project/ail-training/commit/5bcad57)
                                    migrate latex      

  2025-07-16       ail-training     chg: \[intro\]       [#14](https://github.com/ail-project/ail-training/commit/4fac428)
                                    improve feeders    
                                    section            

  2025-07-17       ail-training     new: \[Art of        [#15](https://github.com/ail-project/ail-training/commit/73376fb)
                                    Pivoting\] New     
                                    version added for  
                                    VSS 2025 training  

  2025-07-17       ail-training     chg: \[short         [#16](https://github.com/ail-project/ail-training/commit/74350f9)
                                    intro\] add onion  
                                    lookup + images    
                                    descriptions       

  2025-07-17       ail-training     chg: \[short         [#17](https://github.com/ail-project/ail-training/commit/f919281)
                                    intro\] Ongoing    
                                    Developments       

  2026-02-24       ail-training     chg: \[short         [#18](https://github.com/ail-project/ail-training/commit/53b24b1)
                                    intro\] update     

  2026-04-21       ail-training     chg: \[intro\]       [#19](https://github.com/ail-project/ail-training/commit/a2be145)
                                    update AIL intro   

  2026-04-21       ail-training     chg: \[intro\]       [#20](https://github.com/ail-project/ail-training/commit/9b406db)
                                    rename short intro 
                                    to                 
                                    ail-introduction   
  ------------------------------------------------------------------------------------------------------------------------
