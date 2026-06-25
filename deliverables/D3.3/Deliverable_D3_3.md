---
tags: ngsoti
subtitle: "NGSOTI Project: 101127921 DIGITAL-ECCC-2022-CYBER-03   D3.3"
title: "Reports on NGSOTI training experience and data set"
page-title: "Project: 101127921 NGSOTI DIGITAL-ECCC-2022-CYBER-03"
author: [UNILU/NGSOTI]
date: "2026-06-30"
tlp: "CLEAR"
keywords: [training experience, Kirkpatrick model, survey, learning outcomes, cyber threat intelligence]
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
lof: true
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
# NGSOTI training experience and data

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

The identifier of the deliverable is D3.3 and it adheres to the
definition outlined in the grant agreement **Technical reports and data
sets on training experience data that will be useful for research** .
The deliverable name is **Reports on NGSOTI training experience and data
set** and the overall objective/alignment is described in the executive
summary.

## Executive Summary

This report evaluates the NGSOTI training delivered to a cohort of
graduate-level cybersecurity students and documents the anonymised data
collected through the accompanying post-training survey. The cohort
comprises seventeen participants (n = 17), each attending one or more of
seven hands-on sessions on MISP (CTI fundamentals and automation),
Lookyloo, SkillAegis, Kunai, the AIL Framework, and digital forensics.
Participants complete a structured evaluation questionnaire that
combines **retrospective pre/post self-assessments** with reaction-level
and behaviour-level items, designed around **Kirkpatrick's four-level
training evaluation model** (Reaction, Learning, Behavior, Results);
this report follows the same structure to present its findings.

Across the five tools, self-reported familiarity rises by an average of
**+1.46 points on a 5-point scale** (from a pre-training mean of 1.52 to
a post-training mean of 2.97), and confidence in understanding and
applying the tools improves by **+0.89 to +1.41 points** depending on
the tool and dimension. Participants rate the training positively on
usefulness (mean 4.06/5) and clarity (3.88/5), and a majority report
concrete plans to continue working with the tools (independently,
academically, or professionally) within the following one to three
months. The most frequently cited obstacle to applying what they learn
is **insufficient personal knowledge to proceed independently**, which
indicates that the appetite for the material extends beyond a single
training cycle and supports the case for continued and expanded
delivery.

# Methodology

## Training context

The evaluated training is delivered to a cohort of students of the
Master in Cybersecurity and Cyber Defence (MCYSD) at the University of
Luxembourg, as part of the academic activities supported under NGSOTI
Trainings. The cohort consists of graduate-level students specialising
in cybersecurity and cyber defence, a characteristic that shapes both
the level of prior knowledge participants bring to the sessions and the
kinds of follow-on activities (university projects, theses, internships)
that the Behavior-level findings describe. The programme combines
instructor-led explanation, live demonstrations, and hands-on exercises
across seven tracks:

- **MISP CTI**: fundamentals of the MISP Threat Sharing Platform and
  threat intelligence workflows
- **MISP Automation**: scripting, automation, and integration around
  MISP
- **CTI at Large**: broader cyber threat intelligence concepts and
  practice
- **AIL Framework**: information-leak analysis and OSINT pivoting
- **Forensics**: digital forensics tooling, disk imaging, and log/host
  analysis
- **Kunai**: runtime threat detection and monitoring

All tools used in the training are open-source components of the NGSOTI
technical stack (MISP, AIL, Lookyloo, Kunai, SkillAegis), giving
participants direct exposure to the same software used operationally by
SOC teams and CSIRTs in the consortium and beyond.

## The Kirkpatrick model as the evaluation framework

The post-training questionnaire is designed around **Kirkpatrick's
Four-Level Training Evaluation Model**, a widely used framework for
assessing the effectiveness of learning programmes. The model examines
training outcomes at four progressively deeper levels:

1.  **Reaction**: how participants experience the training: whether they
    find it engaging, clear, and relevant
2.  **Learning**: what knowledge, skills, and confidence participants
    acquire
3.  **Behavior**: whether participants intend to, or already, apply what
    they learned in their studies, projects, or professional context
4.  **Results**: the longer-term, organisational-level outcomes
    attributable to the training (for example, measurable improvements
    in CTI capability across the wider ecosystem)

The first three levels map directly onto sections of the questionnaire:
Reaction items cover satisfaction, clarity, usefulness, difficulty
match, and engagement; Learning items cover the pre/post familiarity and
confidence batteries and the self-rated increase in CTI understanding;
and behavior items cover planned next steps and barriers to application.
Level 4 (Results) sits beyond the scope of a single post-event survey as
it requires longitudinal observation across cohorts, which is the
purpose the collected data described in this report, as it is designed
to serve a structured, comparable baseline to track over time as NGSOTI
training scales up.

## Instrument and data collection

Data collection takes place through a single, anonymous,
self-administered online questionnaire (Microsoft Forms), completed
after the training. The questionnaire comprises:

- **Retrospective pre/post items**: for each of five tools (MISP,
  Lookyloo, SkillAegis, Kunai, AIL), participants rate their
  *familiarity* and their confidence in understanding and applying the
  tool both **before** and **after** the training, on 5-point Likert
  scales. This retrospective design avoids the response-shift bias that
  affects traditional pre-test/post-test comparisons, where participants
  tend to overestimate their initial competence before they know what
  they don't know, and keeps the questionnaire short enough to complete
  in a single sitting.
- **Reaction items**: 5-point ratings of overall satisfaction, clarity
  and usefulness of explanations and demonstrations, match between
  training difficulty and participants' level, motivation to explore
  further, and open questions on the most engaging element and on what
  could be improved.
- **Learning items**: the pre/post batteries above, plus a direct
  5-point self-rating of how much the training increases participants'
  understanding of cyber threat intelligence, and open questions on the
  most valuable tool or activity and on topics that deserve deeper
  coverage.
- **Behavior items**: a multiple-choice question on concrete actions
  participants plan to take in the following one to three months (for
  example, setting up a tool independently, using it in a university
  project, applying for an internship), and a question on perceived
  barriers to doing so.
- **Demographic items**: age, gender, highest level of education,
  nationality, and professional or educational background, collected to
  characterise the sample rather than to identify individuals.

The questionnaire's sections are themselves labeled after Kirkpatrick's
first three levels (Reaction, Learning, Behavior) giving a solid
scientific basis for the training experience evaluation. All figures and
statistics in this report are drawn from the data collected through the
questionnaire.

# Participant Profile

The cohort consists of 17 participants with reasonably diverse
backgrounds. Median age is 26 years (range 23--40). The group skews
male, consists predominantly of students at Master's level, and combines
EU and non-EU nationalities. Participants' professional and educational
backgrounds cluster around computer science/software engineering and
cybersecurity/information security, consistent with the target MCYSD
audience, with smaller numbers reporting network engineering or
data-science backgrounds.

![Participant profile: age distribution, gender, nationality, and
highest education level (n =
17)](./picts/charts/fig07_demographics.png){width="13cm" height="4.5cm"}

![Professional and educational background of participants (multiple
selections allowed)](./picts/charts/fig08_background.png){width="13cm"
height="7cm"}

Attendance is high and broadly even across the seven tracks: every
session is attended by at least 15 of the 17 respondents, with MISP CTI
drawing full attendance (17/17). This even spread means that the
pre/post comparisons below reflect the cohort's overall experience of
the programme rather than the experience of a narrow sub-group.

![Session attendance across the seven NGSOTI training tracks (n =
17)](./picts/charts/fig05_session_attendance.png){width="13cm"
height="5.5cm"}

# Reaction (Level 1): How participants experience the training

Reaction-level results are consistently positive. Usefulness of the
explanations and demonstrations is the highest-rated dimension (mean
4.06/5), while difficulty match sits closest to the scale midpoint
(3.35/5), indicating that the material is pitched neither too easily nor
too demandingly for this cohort.

  --------------------------------------------------------------------------
  Dimension            Mean (1--5)       Median            Distribution
                                                           (rating: count)
  -------------------- ----------------- ----------------- -----------------
  Usefulness of        4.06              4                 2:2, 3:3, 4:4,
  explanations/demos                                       5:8

  Clarity of           3.88              4                 2:2, 3:3, 4:7,
  explanations/demos                                       5:5

  Overall satisfaction 3.59              4                 2:2, 3:6, 4:6,
                                                           5:3

  Motivation to        3.59              4                 2:3, 3:4, 4:7,
  explore further                                          5:3

  Difficulty match     3.35              3                 2:1, 3:10, 4:5,
  (1=too easy, 5=too                                       5:1
  difficult)                                               
  --------------------------------------------------------------------------

![Training quality --- mean ratings across all evaluation dimensions (n
= 17)](./picts/charts/fig04_quality_ratings.png){width="13cm"
height="5.5cm"}

Two-thirds of respondents (12/17) rate overall satisfaction at 4 or 5
out of 5, and the same proportion rates usefulness at 4 or 5. The
difficulty-match distribution clusters tightly around "3 -- about right"
(10 of 17 respondents), indicating that the level of the material is
generally well matched to a graduate cybersecurity audience, with a
small minority finding it on the easier or harder side.

Asked which element of the training they find most engaging,
participants point overwhelmingly to active, applied formats:

![Most engaging training elements, as rated by
participants](./picts/charts/fig06_engaging_elements.png){width="13cm"
height="5.5cm"}

"Hands-on exercise" (11/17) and "Live demonstration" (9/17) are selected
far more often than "Instructor explanation/lecture" (5/17), "Discussion
and Q&A" (4/17), or "Course materials and documentation" (2/17). The
same pattern recurs in the open-text answers: asked what could be
improved, the most frequent suggestion, present in roughly half of all
responses, is simply **more hands-on practice and more real-world
application**, for example:

> "More practical content, how to apply in real situations"

> "I would prefer shorter lecture explanations and more hands-on
> exercises"

> "Take away homework to \[practice\] what we have learned"

# Learning (Level 2): What participants gain

Across **all five tools and all three dimensions measured**
(familiarity, confidence in understanding, confidence in applying), the
post-training mean exceeds the pre-training mean, which is a consistent
signal of self-perceived learning gain that the retrospective pre/post
design is intended to capture.

## Familiarity with the tools

  Tool         Pre (mean)   Post (mean)   delta (Δ)
  ------------ ------------ ------------- -----------
  Kunai        1.65         3.18          **+1.53**
  Lookyloo     1.47         2.94          **+1.47**
  SkillAegis   1.06         2.53          **+1.47**
  MISP         1.94         3.35          **+1.41**
  AIL          1.47         2.82          **+1.35**

![Tool familiarity before vs. after the training (mean rating, n =
17)](./picts/charts/fig01_familiarity_pre_post.png){width="13cm"
height="6.5cm"}

The largest relative gain occurs for SkillAegis, which starts from the
lowest pre-training baseline (1.06, i.e. participants report being
essentially unfamiliar with it) and still records one of the largest
absolute increases. MISP reaches the highest post-training familiarity
in absolute terms (3.35/5), consistent with its two dedicated sessions
(MISP CTI and MISP Automation) and its presence across other tracks such
as CTI at Large and AIL.

## Confidence in understanding and applying the tools

  Tool         Understanding Δ   Applying Δ
  ------------ ----------------- ------------
  MISP         +1.29             **+1.41**
  Kunai        **+1.35**         +1.18
  AIL          +0.94             +1.12
  Lookyloo     +1.12             +1.06
  SkillAegis   +0.76             +0.71

![Confidence in UNDERSTANDING each tool, before vs. after (mean rating,
n =
17)](./picts/charts/fig02_confidence_understanding_pre_post.png){width="13cm"
height="6.5cm"}

![Confidence in APPLYING each tool, before vs. after (mean rating, n =
17)](./picts/charts/fig03_confidence_applying_pre_post.png){width="13cm"
height="6.5cm"}

Participants show increased confidence after the sessions, with MISP and
Kunai show the strongest improvements on every dimension, while
SkillAegis shows the smallest - though still positive --- gains in
confidence despite a strong familiarity gain. This is consistent with a
tool that participants now recognise but have not yet had enough
hands-on time to feel they could operate independently, precisely the
gap that additional exercise time (participants' own leading suggestion;
see Reaction) would be expected to close. For four of the five tools,
gains in "applying" confidence are at least as large as gains in
"understanding" confidence, indicating that the training succeeds not
only in explaining concepts but in giving participants enough practical
exposure to put the tools to work rather than merely describe and
present them.

## Self-rated learning increase and open feedback

Asked directly how much the training increases their understanding of
cyber threat intelligence, participants report a median of 4/5 and a
mean of 3.47/5; 9 of 17 rate the increase at 4 or 5. Open-text answers
about the most valuable activity converge heavily on **MISP** (named
directly by roughly half of respondents) and on **hands-on,
scenario-based exercises**:

> "MISP, we had live demonstration and hands-on real-world incident to
> apply what we had learnt"

> "The hands-on during the forensics classes where we created disk
> images and did forensic analysis on them ... it improved my knowledge"

> "Kunai and threat intel" / "kunai, it was completely new and
> challenging for me so I had to study hard to get into it"

Asked which topic deserves deeper coverage, **CTI** and **MISP** are the
two most frequently named, alongside several requests for more coverage
of "the code", "the project architecture", and end-to-end real-case
workflows, which is a concrete pointer for refining future iterations of
the curriculum.

# Behavior (Level 3): What participants intend to do next

Looking forward, participants report concrete intentions to continue
engaging with the tools and material covered in the training, alongside
the obstacles they expect to encounter.

![Behaviour: intentions to apply the training and the obstacles
ahead](./picts/charts/fig09_plans_barriers.png){width="13cm"
height="5.5cm"}

Eight of seventeen respondents (47%) plan to **continue independently
exploring the tools**, and the same number plan to **set up and explore
a tool in a lab or on their own machine**, in the next one to three
months; eight also intend to **apply for an internship or project
involving these tools**, and seven plan to **use a tool in a university
project, thesis, or assignment**. Most respondents select more than one
of these options, indicating a generally high level of post-training
engagement rather than a binary interested/not-interested split.

Among barriers, the most frequently cited by a clear margin is
**"insufficient personal knowledge to proceed independently"** (5
mentions), followed by **"limited institutional support or permission"**
and **"lack of access to appropriate data"** (4 mentions each), and
**"tools not applicable to my current role"** (3 mentions). Three
respondents report perceiving no barrier at all ("Nothing", "I don't
think so", "Nothing like that").

Reading through the Learning results, the leading barrier "insufficient
personal knowledge" is not a contradiction but a natural consequence of
a successful introductory training: participants now know enough to
recognise what they do not yet know, and to want to close that gap (the
second most common open-text request, after more hands-on time, amounts
to "more of this"). This combination of rising familiarity, rising
confidence, and self-aware appetite for more, is the trajectory a
training programme aims to produce on first encounter with a cohort, and
it points to continuity (follow-up sessions, lab access, mentoring) as
the most effective direction for converting intention into applied
behaviour.

# Discussion

The four levels of the Kirkpatrick model, taken together, describe a
coherent pattern for this cohort. Reaction is solidly positive, with the
clearest signal being a strong preference for hands-on, applied formats
over lecture-style delivery, paired with an equally clear request for
more of the same. Learning gains are consistent and broad-based: every
tool, on every dimension measured, moves in the expected direction, with
the largest gains concentrated on the tools that receive the most
session time and hands-on exposure (MISP, Kunai). Behavior intentions
are concrete and largely academic or professional in nature (lab
exploration, university projects, internships) the kind of
follow-through expected from a graduate cybersecurity cohort, and one
that feeds directly into the longer-term, ecosystem-level outcomes that
**Results (Level 4)** addresses.

As a main recommendation that follow directly from the data, we report
the **Increase the proportion of hands-on exercise time.** This is the
most consistent request across both the closed-ended "engaging element"
question and the open-text improvement suggestions, and it coincides
with where the largest remaining confidence gaps sit (e.g. SkillAegis
"applying" confidence).

# Data Description

**Field groups.**

  --------------------------------------------------------------------------
  Group                   Fields                     Notes
  ----------------------- -------------------------- -----------------------
  Demographics            age, gender, education     Categorical / ordinal;
                          level, nationality,        used to characterise,
                          professional/educational   not identify, the
                          background                 sample

  Attendance              session(s) attended        Multi-select across 7
                                                     tracks

  Pre-training            familiarity and confidence 5-point Likert,
  self-assessment         (understanding /           retrospective
                          applying), per tool, for 5 pre-measure
                          tools                      

  Post-training           familiarity and confidence 5-point Likert
  self-assessment         (understanding /           
                          applying), per tool, for 5 
                          tools; self-rated          
                          CTI-understanding increase 

  Reaction                satisfaction, clarity,     5-point Likert +
                          usefulness, difficulty     multiple-choice
                          match, motivation to       
                          explore further, most      
                          engaging element           

  Behavior                planned actions in the     Multiple-choice,
                          next 1--3 months;          multi-select
                          perceived barriers         

  Open-text               improvement suggestions    Free text; useful for
                          (clarity / delivery),      thematic/qualitative
                          highest-value session,     analysis as illustrated
                          most valuable              in this report
                          tool/activity, topic for   
                          deeper coverage, final     
                          feedback                   
  --------------------------------------------------------------------------

**Future reuse.** The structure of this data is designed to be
**repeatable**: future training cohorts can be surveyed with the same
instrument, and their anonymised responses appended to or compared
against this baseline, supporting longitudinal tracking of learning-gain
trends, and cross-cohort comparisons.

# Annex: Survey Instrument

The questionnaire fielded for this cohort comprises the following
sections and item types (verbatim question wording is preserved in the
released data's column headers):

1.  **Demographics**: age, gender, highest level of education,
    nationality, professional/educational background, session(s)
    attended.
2.  **Pre-training self-assessment**: for each of MISP, Lookyloo,
    SkillAegis, Kunai, and AIL: familiarity (5-point scale), confidence
    in understanding (5-point scale), confidence in applying (5-point
    scale).
3.  **Reaction**: overall satisfaction, clarity of
    explanations/demonstrations, usefulness of explanations/
    demonstrations, match between difficulty and professional/knowledge
    level, most engaging training element (select up to 2), open
    questions on what could be improved (clarity, delivery), and which
    session had the highest added value.
4.  **Post-training self-assessment**: the same familiarity and
    confidence batteries as the pre-training section, repeated for the
    same five tools, plus a direct self-rating of how much the training
    increases understanding of cyber threat intelligence, an open
    question on the most valuable tool/activity, and an open question on
    which concept or skill deserves deeper coverage.
5.  **Behavior**: motivation to explore the topics/tools further,
    planned actions in the next one to three months (select all that
    apply), perceived barriers to applying what was learned (select all
    that apply), and final open-ended feedback.
