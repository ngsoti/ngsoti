# Exercise 4: D4 blackhole traffic and vulnerability context

**Time:** 30 minutes. **Solutions:** [`tests/04-d4`](../tests/04-d4),
[`tests/05-vulnerability`](../tests/05-vulnerability).

## Goal

Triage packet captures from a D4 sensor, detect an exploit attempt, and add
vulnerability context to the alert.

## Background

[D4](https://www.d4-project.org/) is a framework to collect network data on
many sensors and to centralize it. NGSOTI uses it to capture *blackhole*
traffic: packets that reach unused address space. Nobody should send packets
there, so everything in the capture is a scan, an attack, or a
misconfiguration. NGSOTI's blackhole sensors captured terabytes of such traffic
in 2024, including exploitation of the Zyxel vulnerability CVE-2023-28771 and
devices that send Syslog to the wrong network.

A D4 sensor encapsulates a byte stream and sends it to a D4 server. The server
dispatches the stream to analyzers. Our analyzer is a Tenzir pipeline that reads
the PCAP bytes.

`data/d4/blackhole.pcap` is a synthetic capture that models these
observations: Telnet scans, a misdirected Syslog message, a DNS query for
`cloud.mikrotik.com`, an SNMP probe, and one IKE packet with a shell command.

## Steps

### Part A: Triage

1. Read the capture and decode the packets:

   ```tql
   from_file "data/d4/blackhole.pcap" {
     read_pcap
   }
   this = {...this, ...decapsulate(this)}
   ```

   `read_pcap` turns bytes into packet events. `decapsulate` parses link,
   network, and transport headers and also computes the Community ID.

2. Summarize by transport protocol and destination port:

   ```tql
   transport = "tcp" if tcp != null else "udp"
   dst_port = tcp?.dst_port?.otherwise(udp?.dst_port?)
   summarize transport, dst_port, packets=count(), sources=distinct(ip.src)
   sort dst_port
   ```

   Which ports does the blackhole see? Which of them point to a
   misconfiguration, and which to a scan? Compare with
   [`tests/04-d4/blackhole-triage.tql`](../tests/04-d4/blackhole-triage.tql).

### Part B: Detect exploitation

3. Open [`detect-ike-command-injection.tql`](../tests/04-d4/detect-ike-command-injection.tql).
   The exploit for CVE-2023-28771 sends shell metacharacters followed by a
   command to UDP port 500. Packet payloads are binary, so the pipeline matches
   their hexadecimal form. Run it. Which source sent the exploit?

4. Find a Suricata rule for this CVE. Vulnerability-Lookup links CVEs to
   detection rules in [Rulezet](https://rulezet.org). We saved the response in
   `data/rulezet/CVE-2023-28771.json`. Which rule formats exist? Read the
   Suricata rule and compare its logic with ours. Which is more precise?

### Part C: Vulnerability context

5. Start a node (see Exercise 3), and run the three steps in
   [`tests/05-vulnerability/cve-context`](../tests/05-vulnerability/cve-context):

   ```sh
   for f in tests/05-vulnerability/cve-context/0*.tql; do tenzir -f "$f"; done
   ```

   The first step creates a lookup table, the second loads the CVE record from
   [Vulnerability-Lookup](https://vulnerability.circl.lu) (`data/vulnerability-lookup/CVE-2023-28771.json`),
   and the third enriches the detection. What is the severity? The
   [EPSS score](https://www.first.org/epss/) in
   `data/vulnerability-lookup/epss-CVE-2023-28771.json` estimates the
   probability of exploitation. How would you add it to the table?

6. Read [`live/vulnerability-lookup.tql`](../live/vulnerability-lookup.tql) and
   [`live/vulnerability-lookup-sightings.tql`](../live/vulnerability-lookup-sightings.tql).
   Vulnerability-Lookup asks clients to identify themselves, to use `since=` for
   incremental pulls, and not to enumerate its API. Where do the pipelines
   follow this?

### Part D: A live D4 analyzer

7. Read [`live/d4-pcap-stream.tql`](../live/d4-pcap-stream.tql). It reads from a
   named pipe that a D4 analyzer feeds. Draw the path of a packet from the
   sensor to the `blackhole` topic. Which component handles the D4 protocol
   header?

## Questions

- The blackhole only sees unsolicited traffic, so no flow has a legitimate
  response. Which detections become simpler because of that?
- The exploit packet is one of 7. How does a detection stay cheap when you see
  billions of packets per day? (Hint: look at the order of the two `where`
  statements in `detect-ike-command-injection.tql`.)

## Docs

- [`read_pcap`](https://tenzir.com/docs/reference/operators/read_pcap),
  [`decapsulate`](https://tenzir.com/docs/reference/functions/decapsulate),
  [`encode_hex`](https://tenzir.com/docs/reference/functions/encode_hex),
  [`match_regex`](https://tenzir.com/docs/reference/functions/match_regex)
- [Get data from the network](https://tenzir.com/docs/guides/collect/get-data-from-the-network)
- [Enrich with asset inventory](https://tenzir.com/docs/guides/enrich/enrich-with-asset-inventory)
- [Vulnerability-Lookup API](https://www.vulnerability-lookup.org/documentation/api.html)
- [D4 project](https://www.d4-project.org/) and
  [d4-core](https://github.com/D4-project/d4-core)
