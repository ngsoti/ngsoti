---
tags: ngsoti
title: "NGSOTI – Project: 101127921 \\ DIGITAL-ECCC-2022-CYBER-03 \\ Learning from Large-Scale IPv4 Blackhole: \\ Behavioral Analysis of the SNMP Ecosystem"
page-title: "Project: 101127921 — NGSOTI — DIGITAL-ECCC-2022-CYBER-03"
author: [Team CIRCL/NGSOTI]
date: "2024-11-24"
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





# Behavioral Analysis of the SNMP Ecosystem

## Disclaimer
Co-funded by the European Union. Views and opinions expressed are however those of the author(s) only and do not necessarily reflect those of the European Union or the European Cybersecurity Competence Centre. Neither the European Union nor the granting authority can be held responsible for them.

## Introduction 

The missions of the european Project **NGSOTI** (Next Generation Security Operator Training Infrastructure), is to empower SOC operators and organisations across Europe with the knowledge, tools, and infrastructure needed to defend effectively against ever-evolving cyber threats. ([restena.lu](https://restena.lu/en/project/ngsoti)). Within this scope the key objective is to enhance SOC operator capabilities.

SOCs carry the crucial mission of monitoring cybersecurity events and escalating any incidents or detections of malicious activity. While the task may seem straightforward, it is in fact increasingly complex. Attackers continuously evolve their techniques, thereby forcing SOC analysts to keep adapting.  

These analysts are confronted with a dual challenge:  
- On one hand, they must be versatile generalists, capable of responding across a wide spectrum of domains.  
- On the other hand, the volume of alerts and information they must process frequently leads to cognitive fatigue.  

This fatigue inevitably results in reduced alertness and motivation, which can lead to miss-detections of critical incidents.  

Our sinkhole-based analysis is aligned with the core ambition of NGSOTI: to develop an open-source training infrastructure powered by real-world data that equips future SOC operators with advanced capabilities in network-related alert handling, incident response, log analysis, security operations management, threat intelligence, and communication. ([restena.lu](https://restena.lu/en/project/ngsoti))  

In this context, we target two complementary goals:  
Primarly, it will provide key insights and enriched data that enable SOC teams to understand threats to be able to detect attacks with finer granularity.

Secondly , our goal is to reduce operator fatigue by optimising the knowledge of background noise . I will help improving alerts prioritisation improving vigilance and training of SOC analysts.

By pursuing these complementary objectives, we aim to enhance both SOC performance and training quality within the NGSOTI framework. 

We thank's the RESTENA foundation for the opportunity to work on this dataset. 


Analysis scope

This report is an analysis by CIRCL of traffic captured on his sinkhole. It is focusing on the Simple Network Management Protocol (SNMP). SNMP is a standardized protocol used for monitoring, managing, and configuring networked devices such as routers, switches, servers, and IoT devices. It enables network administrators to collect information about device performance, network traffic, and operational status, as well as to remotely control certain device parameters.

SNMP operates in a client-server model: managed devices run an SNMP agent that exposes management data, while a network management system (NMS) queries these agents or receives unsolicited notifications (traps) to monitor the network. Data is structured in the form of Object Identifiers (OIDs), which represent specific metrics such as CPU load, interface status, or memory usage. This Traffic is using UDP (Unified Datagram Protocol) therefore it give relevant informations even if no host are listening and responding to.

SNMP supports three versions:

v1 and v2c: Basic functionality with community strings for authentication, but limited security.

v3: Adds cryptographic security with authentication and encryption.

## Dataset 

The dataset is an extract of the SNMP traffic captured by the blackhole network collected by CIRCL from November 1st to October 31th, 2025. Each record of the dataset represents a single SNMP packet received by the blackhole. It includes packet reception timestapm, source and destination IPs and ports, SNMP version, requested OIDs, community string (if applicable), and the reference pcap file. The sinkhole is a /18 containing 16382 IPv4 addresses, on a range which is only 1 bit away from a private RFC1918 range. 
This network allowing not only to capture standard scanning and exploits activity but also to catch misconfigurations or “typo” traffic targeting nearby private network spaces. This dataset provides insights into automated scanning campaigns as well as opportunistic reconnaissance behavior observed in the first half of 2025.

## Datalake setup

The SNMP traffic was extracted from raw pcaps files using [Suricata](https://https://suricata.io/) 7.0.3, an open-source network threat detection engine capable of parsing protocols in real time. Suricata generated structured metadata, including SNMP version, community strings, requested OIDs, source and destination IPs and ports, and timestamps.

Theses extracted metadata were ingested into [ClickHouse](https://clickhouse.com/) 25.9.3.1. Clickhouse is a high-performance columnar database optimized for analytical workloads. Since ClickHouse allows fast aggregation and querying of large datasets, it is ideal for statistical analysis of SNMP traffic, such as tracking scanning patterns, frequent OIDs, and temporal trends in probing activity. 


The final datalake contains the following data structure; 

```
   ┌─name──────┬─compressed_size─┬─uncompressed_size─┬──ratio─┐
1. │ version   │ 331.43 MiB      │ 1.18 GiB          │   3.65 │
2. │ file      │ 465.48 MiB      │ 107.46 GiB        │  236.4 │
3. │ dest_ip   │ 2.43 GiB        │ 8.89 GiB          │   3.66 │
4. │ src_ip    │ 1.71 GiB        │ 8.42 GiB          │   4.92 │
5. │ oids      │ 3.21 GiB        │ 36.20 GiB         │  11.28 │
6. │ src_port  │ 958.68 MiB      │ 1.18 GiB          │   1.26 │
7. │ dest_port │ 11.83 MiB       │ 1.18 GiB          │ 102.23 │
8. │ community │ 187.06 MiB      │ 4.07 GiB          │  22.29 │
9. │ timestamp │ 151.82 MiB      │ 2.36 GiB          │  15.93 │
   └───────────┴─────────────────┴───────────────────┴────────┘

```
* version, is the snmp version could be 1,2 or 3.
* file reference the pcap original file where the packet lie
* dest_ip the destination ipv4
* scr_ip the source ipv4
* oid is an array of requested oids in the snmp frame
* src_port the udp source port
* dest_port the udp destination port
* Community is the SNMP community string
* timestamp the timestamp of the data frame.

## General Statistical analysis
### SNMP Activity
requencies Analysis.ipynb
The year-long analysis reveal that the IPv4 /18 sinkhole was contacted via SNMP by 153,045 distinct IPv4 sources, generating a total of 6.34 billion queries. The diagram below illustrates the daily volume.

![](http://hdoc.csirt-tooling.org/uploads/upload_b7dad7268e928773b19299dfcd84050c.png)

The frequency analysis highlights clear spikes of roughly 4 billion SNMP queries per day on the following dates:
– Late November 2024 and early October 2024
– 14 January 2025
– 2–3 June 2025
– 29 October 2025

### Country repartition.
To visualise the source of the snmp traffic we used two different criterias, packet volume and different ip volume per country.

![](http://hdoc.csirt-tooling.org/uploads/upload_420fb5612baae17175f9130526504f2c.png)


Top 10 of total hits per country.
| Country | distinct IPs | total_packets |
|---------|--------------|------------|
| ID      | 1340         | 64,296,106 |
| CN      | 69,118       | 60,024,273 |
| PS      | 2            | 41,831,687 |
| US      | 4,451        | 22,493,575 |
| CL      | 2,747        | 21,884,264 |
| DE      | 2,306        | 19,192,107 |
| CO      | 1,682        | 8,334,777  |
| BR      | 581          | 6,605,271  |
| GB      | 577          | 6,273,069  |
| MX      | 1,022        | 5,862,589  |

Many interesting informations are revealed by this output.
* Indonesia is the first nation in term of packet emited just before China with 50x less source IP's.
* Palestine emitted 40 Billions of packets using only two IP's. This behaviour is analysed in the chapter Anomalies Investigation / Palestinian traffic.

It is interesting to see that besides Indonesia 

![](http://hdoc.csirt-tooling.org/uploads/upload_e924901d0696f9b1200dcb778722ca1c.png)




| Country | IP's | 
| -------- | -------- | 
|	CN |	69118
| 	US |	4451
| 	CL |	2747
| 	IN 	|2323
| 	DE |	2306
| 	CO |	1682
| 	SG |	1564
| 	ID |	1340
| 	SE |	1323
| 	MX |	1022
| 	TH |	1021
| 	MY |	900
| 	TW |	688
| 	BR |	581
|	GB |	577

### SNMP version repartition

The final dataset represent 328 Billions of SNMP packets, the analysis of snmp version shows a low as of 2% of query requests using the SNMP protocol v3.

![](http://hdoc.csirt-tooling.org/uploads/upload_b9f7ed43fa596c401fac28771be3e619.png)

### SNMP Community


![](http://hdoc.csirt-tooling.org/uploads/upload_246b783153a912046d5747743ea29e41.png)


##### Scanned devices


CANON_ADMIN
https://oip.manual.canon/USRMA-0219-zz-SS-enUS/contents/10040030.html





### Devices scanned.

#### Methodology

To analyze the SNMP sinkhole dataset, we focus on the Object Identifiers (OIDs) queried by external sources. Each SNMP packet may contain multiple OIDs, which we flatten to handle individually. We then extract the vendor-specific prefix of each OID, identified by the `1.3.6.1.4.1.x` pattern, where `x` corresponds to a particular vendor. By counting the frequency of queries for each vendor prefix and ranking them, we can identify which vendors’ devices are most commonly targeted or scanned. This methodology allows us to uncover trends in attacker behavior, highlight reconnaissance activity, and detect potential focus on specific device types in the wild.

#### Devices Scan
![](http://hdoc.csirt-tooling.org/uploads/upload_1aa7548b393a1eeb1d82a6402d7e318f.png)
![](http://hdoc.csirt-tooling.org/uploads/upload_4a7833afac2c61ece4840bf746be9156.png)


#### Device to Community

![](http://hdoc.csirt-tooling.org/uploads/upload_c8f506eec12cfb6c26462dcceb01bcae.png)

This graph depict the relations between the snmp hardware and the requested v1 & v2 community. it cleardy demonstrate that beside public, there is a clear focus on certain oid for certain devices. This reveal abuse for default snmp community  however most of the non public ones discovered in this dataset are not documented

#### Default values
* canon_admin for [Canon](https://oip.manual.canon/USRMA-0219-zz-SS-enUS/contents/10040030.html)

### Vulnerabilities and exploits

TO ANALYSE
![](http://hdoc.csirt-tooling.org/uploads/upload_45efbd2d789d43d5ad308da6194c35ce.png)

MISCONFIG
![](http://hdoc.csirt-tooling.org/uploads/upload_cf8e5f858174412541b121f0b940f2a4.png)


### Anomalies investigation

## Palestinian Traffic

According to the volumetry analysis the Palestinian Autonomous System 12975 have emited 41.8 Billions of packets.
``` 
   ┌─ip────────────┬─as_number─┬─as_name────────────────────────────┬─as_country─┬─ptr─┐
1. │ 213.6.137.78  │     12975 │ PALTEL-AS PALTEL Autonomous System │ PS         │     │
2. │ 213.6.173.227 │     12975 │ PALTEL-AS PALTEL Autonomous System │ PS         │     │
   └───────────────┴───────────┴────────────────────────────────────┴────────────┴─────┘
```

It appear that the traffic was only generated by the host 213.6.137.78
from at least 1st of January to the 19th of May. This IP did not have any PTR associated DNS records and is not known in our passive DNS database.

![](http://hdoc.csirt-tooling.org/uploads/upload_d9c22a5551bb9ae72366451bf1624051.png)

For each destination, two different scans are performed. Each scans are performed 3 times and retried every 3 seconds.

For the first type of scan, it do a SNMP get request and ask the following generic SNMP Oids;
* 1.3.6.1.2.1.2.2.1 ifEntry
* 1.3.6.1.2.1.4.20.1 ipAddrTable
* 1.3.6.1.2.1.4.22.1 ipNetToMediaTable
Then finally the unknown regitered vendor 9999 oid
* 1.3.6.1.2.1.9999.1.1.6.4.1

This last scan may be related to microtiK os devices as documented [here](https://fossies.org/linux/opennms/features/enlinkd/tests/src/test/resources/linkd/nms102/mikrotik-192.168.0.1-walk.txt) And seems to allows determination of internal IP's.
```bash
snmpwalk [redacted] -v1 -c public 1.3.6.1.2.1.9999.1.1.6.4.1 | head
Error: OID not increasing: iso.3.6.1.2.1.9999.1.1.6.4.1.4.192.168.33.99
 >= iso.3.6.1.2.1.9999.1.1.6.4.1.4.172.31.33.144

iso.3.6.1.2.1.9999.1.1.6.4.1.4.192.168.33.99 = INTEGER: 2
iso.3.6.1.2.1.9999.1.1.6.4.1.4.172.31.33.144 = INTEGER: 2
``` 

The second scan is also a SNMP get request on; 
* 1.3.6.1.2.1.25.3.3.1.2 hrProcessorEntry
* 1.3.6.1.2.1.25.2.3.1.2 hrStorageEntry

For all the request, no payload are present.

Example in c0bb49e788964718af4dfea4c0ab898c-2025-04-27-174644
![](http://hdoc.csirt-tooling.org/uploads/upload_a40a19c1ccc3f3e12aacc687e9797daa.png)

Example in c0bb49e788964718af4dfea4c0ab898c-2025-03-16-011212
![](http://hdoc.csirt-tooling.org/uploads/upload_02013d5831e6aec2a922a4f8cbf2299c.png)

This schema is observable for all destination IP's.

## Scanning vendor Traffic
By resolving the pointers record it is convenient to find out some scanning solutions vendors or foundation.

The blackhole was reached by;
* [Cencys](https://search.censys.io/)
* [NetSecScan](http://netsecscan.net/)
* [Shadowserver](https://www.shadowserver.org/)
* [ShadowForce](https://shadowforce.io/)
* [Shodan](https://www.shodan.io/)
* onyphe.net (TODO)
* 

### Censys
Censys scanners are originating from BGP AS;
* 398324
* 398722

We identified in our dataset 65 IP's with a censys DNS related PTR record 
   * 16 scanner-001.hk2.censys-scanner.com
   * 16 scanner-007.ch1.censys-scanner.com
   * 1 scanner-011.ch1.censys-scanner.com
   * 16 scanner-11.ch1.censys-scanner.com
   * 16 scanner-14.ch1.censys-scanner.com
     
And and additional 16 Ip's from these AS responding to the PTR    unused-space.coop.net.

The footprint of these 81 Ip's.
![](http://hdoc.csirt-tooling.org/uploads/upload_51457a1e059fae20afa88292aeec25b6.png)


![](http://hdoc.csirt-tooling.org/uploads/upload_84fba8c831fd439f8b46fb594aca32e1.png)


From this frequencial analysys we could see that Censys never exceed a total of 350 SNMP paquets per hours and spread the load in a really efficient manner keeping a footprint of the dataset very ligth.

Additionnaly during this timeframe, all these request were using SNMPv3 query, which impair us to determine the queried OID and Versions.

![](http://hdoc.csirt-tooling.org/uploads/upload_ca3b433f5168ee673a4a43e7936edd6f.png)

It should be mentionned that besides SNMP, a Single scanners scans for 74 other TCP ports and 12 UDPs 


### NetSecScan

### Shadowserver

### ShadowForce

### Shodan



# Learning experience

Problematique soc operators
Finding soc operators



# TODO - TO REMOVE
->> TODO
* data from oct 24 / oct 25
* Volume of XX bytes of raw network capture
* graph scan vendor
* new oid / month 
* analyse agnov
* chapitre pourquoi pour le soc ?
* snmp community ( grep -v tuple (1ipvs1pt))
* trouver vulns
* todo donner les donnees / notebook.
* outcome -> liste
* grouper par token les srcip(cable, adsl, vpn, scan) parametre.
* Semaines pattern ( sur un byte )
* month sur un byte

idées
* congés pattern
