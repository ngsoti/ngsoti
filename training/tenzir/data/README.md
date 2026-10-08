# Sample data

The data supports the incident story in the [module README](../README.md). It
uses documentation address ranges (RFC 5737) and the reserved name
`evil-cdn.example`, so nothing points at real infrastructure.

| Path | Origin |
| :--- | :----- |
| `kunai/`, `suricata/`, `zeek/`, `misp/`, `d4/` | Synthetic, in the formats of the respective tools |
| `sigma/`, `sigma-ocsf/` | Rules written for this module |
| `vulnerability-lookup/`, `rulezet/`, `hashlookup/` | Real responses of the public CIRCL services |

The Kunai, Suricata, and Zeek events share addresses and Community IDs, so that
Exercise 2 can correlate them.

## Real data for your own experiments

- The [Kunai sandbox](https://sandbox.kunai.rocks/)
- The NGSOTI malware dataset, which includes Kunai logs and PCAPs:
  <https://helga.circl.lu/NGSOTI/malware-dataset>
- More NGSOTI datasets: [`datasets.md`](../../../datasets.md)
