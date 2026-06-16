# Datacenter Supply Chain: Publicly Traded Companies by Component Category

A comprehensive map of every major layer of the modern datacenter stack and the publicly traded companies that supply each layer. Organized from silicon to building shell.

> **Disclaimer:** This is for informational/research purposes only — not investment advice. Tickers and company structures change; verify before acting.

---

## 1. Compute — CPUs, GPUs & AI Accelerators

The core processing layer: GPUs dominate AI training/inference, CPUs handle general workloads.

| Company | Ticker | What They Supply |
|---|---|---|
| NVIDIA | NVDA | GPUs, AI accelerators (H100, H200, Blackwell), NVLink |
| AMD | AMD | CPUs (EPYC), GPUs, MI300/MI350 AI accelerators |
| Intel | INTC | Xeon CPUs, Gaudi 3 AI accelerators, FPGAs |
| Super Micro Computer | SMCI | Full server systems, AI rack solutions |
| Dell Technologies | DELL | PowerEdge servers, rack systems |
| Hewlett Packard Enterprise | HPE | ProLiant servers, Cray supercomputers |
| Lenovo | LNVGY | Server systems (ThinkSystem) |

---

## 2. Custom ASICs & Accelerators

Hyperscalers (Google, Meta, Amazon, Microsoft) are building custom chips; these are the two dominant co-design partners.

| Company | Ticker | What They Supply |
|---|---|---|
| Broadcom | AVGO | Custom AI ASICs (Google TPU, Meta MTIA), networking silicon — ~70% market share in custom AI silicon |
| Marvell Technology | MRVL | Custom ASICs (Amazon Trainium, Microsoft Maia), DPUs, connectivity |
| Astera Labs | ALAB | PCIe retimers, CXL memory controllers, Ethernet smart cable modules |
| Credo Technology | CRDO | Active electrical cables (AEC), optical connectivity ICs |

---

## 3. Memory — DRAM, HBM & High-Bandwidth Memory

AI accelerators consume enormous memory bandwidth; HBM is now a critical bottleneck.

| Company | Ticker | What They Supply |
|---|---|---|
| Micron Technology | MU | DRAM, NAND flash, HBM3E (high-bandwidth memory) |
| SK Hynix | 000660.KS | HBM market leader, DRAM — listed on Korea Stock Exchange |
| Samsung Electronics | 005930.KS | DRAM, NAND, HBM — listed on Korea Stock Exchange |
| Western Digital | WDC | NAND flash, SSDs, HDDs |
| Seagate Technology | STX | Hard disk drives (HDDs), mass storage |
| Pure Storage | PSTG | All-flash enterprise storage arrays |
| NetApp | NTAP | Storage systems, cloud data services |

---

## 4. Networking — Switches, Routers & NICs

The interconnect fabric that ties thousands of GPUs into coherent training clusters.

| Company | Ticker | What They Supply |
|---|---|---|
| Cisco Systems | CSCO | Enterprise switches, routers, network management |
| Arista Networks | ANET | High-performance ethernet switches for hyperscale AI clusters |
| Broadcom | AVGO | Switching silicon (Tomahawk, Trident series), PAM4 DSPs |
| Marvell Technology | MRVL | Networking chips, Ethernet controllers, DPUs |
| Juniper Networks | JNPR | Routers and switches (being acquired by HPE) |
| Mellanox/NVIDIA | NVDA | InfiniBand networking, ConnectX NICs (now part of NVIDIA) |

---

## 5. Optical Transceivers & Components

800G and 1.6T optical modules are the new bottleneck as AI clusters scale. The datacom optics market exceeded $16B in 2025.

| Company | Ticker | What They Supply |
|---|---|---|
| Coherent Corp | COHR | Optical transceivers, lasers, modulators, 800G/1.6T modules |
| Lumentum Holdings | LITE | Laser chips, optical components for transceivers |
| Ciena Corporation | CIEN | Coherent optical networking platforms, photonic systems |
| Fabrinet | FN | Contract manufacturer for optical modules (makes parts for many brands) |
| Applied Optoelectronics | AAOI | Optical transceivers for hyperscalers |
| II-VI / Coherent | COHR | (Same as Coherent after merger) |

---

## 6. Fiber Optic Cable & Connectivity

Physical fiber that connects servers, racks, and buildings — demand exploding with AI buildout.

| Company | Ticker | What They Supply |
|---|---|---|
| Corning | GLW | Optical fiber, cable, connectors, fiber-to-the-chip; multiyear partnership with NVIDIA (announced May 2026) |
| CommScope Holdings | COMM | Fiber cables, network connectivity hardware |
| Prysmian Group | PRY.MI | Fiber and power cables — listed on Milan Stock Exchange |
| Belden | BDC | Signal transmission cables, connectivity solutions |

---

## 7. Electrical Connectors & PCB Interconnects

Every board, backplane, and cable assembly inside a server relies on precision connectors.

| Company | Ticker | What They Supply |
|---|---|---|
| Amphenol | APH | Connectors, cable assemblies, high-speed backplane connectors for AI servers (up ~98% YTD as of 2026) |
| TE Connectivity | TEL | Connectors, sensors, PCB-level interconnects |
| Molex | Private | (Koch Industries subsidiary — not publicly traded) |

---

## 8. Power Distribution & UPS

Uninterruptible power supplies, power distribution units, and switchgear that keep servers running through grid events.

| Company | Ticker | What They Supply |
|---|---|---|
| Vertiv Holdings | VRT | UPS systems, PDUs, switchgear, liquid cooling — full power+cooling ecosystem; guided ~$13.5B revenue for 2026 |
| Eaton Corporation | ETN | UPS systems, PDUs, circuit protection, switchgear; acquired Resilient Power Systems (2025) |
| Schneider Electric | SBGSY / SU.PA | EcoStruxure UPS, modular power, DCIM software |
| ABB Ltd | ABB | Transformers, switchgear, power automation |
| Legrand | LGRVF / LR.PA | PDUs, racks, structured cabling for datacenters |
| Siemens | SIEGY | Electrical distribution, transformers, automation |

---

## 9. Backup Power — Generators & Fuel Cells

When grid power fails, datacenters need seconds-to-minutes of bridge power (UPS) and then long-duration generators.

| Company | Ticker | What They Supply |
|---|---|---|
| Caterpillar | CAT | Diesel generators, generator sets for datacenters |
| Cummins | CMI | Diesel and natural gas generator sets |
| Generac Holdings | GNRC | Backup generators; acquiring Enercon (switchgear) |
| Bloom Energy | BE | Solid-oxide fuel cells for always-on on-site power |
| Constellation Energy | CEG | Grid power supply; largest private US power producer post-Calpine acquisition |

---

## 10. Power Semiconductors — GaN & SiC

Wide-bandgap semiconductors enable smaller, more efficient power conversion throughout the facility (from wall power to the chip).

| Company | Ticker | What They Supply |
|---|---|---|
| ON Semiconductor | ON | SiC MOSFETs and diodes for power conversion |
| Wolfspeed | WOLF | SiC wafers and power devices |
| STMicroelectronics | STM | SiC and GaN power devices |
| Infineon Technologies | IFNNY | GaN and SiC power semiconductors (acquired GaN Systems) |
| Navitas Semiconductor | NVTS | GaN integrated circuits for power conversion |

---

## 11. Cooling & HVAC

AI GPUs at 700W+ per chip make cooling the hardest engineering problem in modern datacenters.

| Company | Ticker | What They Supply |
|---|---|---|
| Vertiv Holdings | VRT | Liquid cooling (CDUs), rear-door heat exchangers, precision air cooling |
| Modine Manufacturing | MOD | Direct-to-chip and immersion liquid cooling (Airedale division), thermal management |
| Comfort Systems USA | FIX | HVAC design, prefabricated modules, on-site construction for mission-critical facilities; revenues surged 56% YoY Q1 2026 |
| Ecolab | ECL | Water treatment chemicals and services for cooling towers; acquiring CoolIT Systems (closes Q3 2026) |

---

## 12. Semiconductor Manufacturing Equipment

Not inside the datacenter, but the upstream equipment that manufactures every chip inside it.

| Company | Ticker | What They Supply |
|---|---|---|
| ASML Holding | ASML | EUV/DUV lithography machines — the sole supplier of EUV |
| Applied Materials | AMAT | Deposition, etch, CMP equipment |
| Lam Research | LRCX | Etch and deposition systems |
| KLA Corporation | KLAC | Process control, wafer inspection |
| Tokyo Electron | TOELY | Coater/developers, etch systems |

---

## 13. EDA & Chip Design Software

Without design software, no chip gets manufactured.

| Company | Ticker | What They Supply |
|---|---|---|
| Synopsys | SNPS | EDA software, silicon IP (being acquired by ANSYS/Cadence process ongoing) |
| Cadence Design Systems | CDNS | EDA tools, physical verification, IP |
| Ansys | ANSS | Simulation software for chip and system design |

---

## 14. Construction & Installation Services

The skilled trades that physically build, wire, and commission datacenter facilities.

| Company | Ticker | What They Supply |
|---|---|---|
| Quanta Services | PWR | Power generation, high-voltage transmission, grid interconnections for datacenters; $48.5B backlog |
| EMCOR Group | EME | Mechanical and electrical construction, HVAC installation, mission-critical facilities |
| MasTec | MTZ | Infrastructure construction, electrical, telecom |
| Comfort Systems USA | FIX | HVAC and mechanical construction (also listed under Cooling) |

---

## 15. Datacenter REITs & Operators

Companies that own and operate the physical buildings where datacenters live.

| Company | Ticker | What They Supply |
|---|---|---|
| Equinix | EQIX | Global colocation, interconnection — 260+ datacenters worldwide |
| Digital Realty Trust | DLR | Hyperscale and colocation facilities (PlatformDIGITAL) |
| Iron Mountain | IRM | Datacenter REIT; 450MW operating + 900MW pipeline; DC revenue approaching $1B+ in 2026 |

---

## 16. Cybersecurity

Every datacenter requires network security, access control, and threat detection.

| Company | Ticker | What They Supply |
|---|---|---|
| Palo Alto Networks | PANW | Next-gen firewalls, SASE, cloud security, SIEM |
| Fortinet | FTNT | FortiGate firewalls, unified security fabric |
| Check Point Software | CHKP | Network security gateways, threat prevention |
| CrowdStrike | CRWD | Endpoint detection & response, cloud workload protection |
| Zscaler | ZS | Zero-trust network access, cloud security |
| Cisco Systems | CSCO | Network security (also listed in Networking) |

---

## 17. Cloud Hyperscalers (Operators & Buyers)

Not suppliers — but the primary buyers of everything above. Their capex drives the entire supply chain.

| Company | Ticker | Datacenter Role |
|---|---|---|
| Microsoft (Azure) | MSFT | Hyperscaler, massive AI infrastructure investor |
| Amazon (AWS) | AMZN | Largest cloud provider, custom silicon (Trainium, Inferentia) |
| Alphabet (Google Cloud) | GOOGL | Hyperscaler, custom TPU ASICs |
| Meta Platforms | META | Social-scale AI training, custom MTIA silicon |
| Oracle Cloud | ORCL | GPU clusters, cloud infrastructure |

---

## Quick-Reference: Ticker Summary

```
COMPUTE/SERVERS:    NVDA  AMD   INTC  SMCI  DELL  HPE   LNVGY
CUSTOM ASICs:       AVGO  MRVL  ALAB  CRDO
MEMORY/STORAGE:     MU    WDC   STX   PSTG  NTAP  (SKHynix: 000660.KS) (Samsung: 005930.KS)
NETWORKING:         CSCO  ANET  AVGO  MRVL  JNPR
OPTICS:             COHR  LITE  CIEN  FN    AAOI
FIBER/CABLE:        GLW   COMM  BDC   (Prysmian: PRY.MI)
CONNECTORS:         APH   TEL
POWER DIST/UPS:     VRT   ETN   SBGSY ABB   LGRVF SIEGY
GENERATORS:         CAT   CMI   GNRC  BE    CEG
POWER SEMIS:        ON    WOLF  STM   IFNNY NVTS
COOLING/HVAC:       VRT   MOD   FIX   ECL
SEMI EQUIPMENT:     ASML  AMAT  LRCX  KLAC  TOELY
EDA SOFTWARE:       SNPS  CDNS  ANSS
CONSTRUCTION:       PWR   EME   MTZ   FIX
DC REITS:           EQIX  DLR   IRM
CYBERSECURITY:      PANW  FTNT  CHKP  CRWD  ZS    CSCO
HYPERSCALERS:       MSFT  AMZN  GOOGL META  ORCL
```

---

## Sources

- [Data Center Stocks 2026: 40+ Companies | StockTitan](https://www.stocktitan.net/stocks/themes/data-center-stocks)
- [Best Optical Networking Stocks for May 2026 | TickerSpark](https://tickerspark.ai/top-stocks/best-optical-networking-stocks)
- [AI Data Center Boom: Top Stocks & ETFs for 2026 | Tickeron](https://tickeron.com/trading-investing-101/the-data-center-power-boom-10-stocks-and-10-etfs-positioned-for-the-ai-infrastructure-supercycle/)
- [Top AI Infrastructure Stocks 2026 | ExoSwan](https://exoswan.com/ai-infrastructure-stocks/)
- [11 AI Data Center Cooling Stocks | MarketWise](https://marketwise.com/investing/ai-data-center-cooling-stocks-ecolab-coolit/)
- [Broadcom vs Marvell: Custom AI Silicon Battle 2026](https://www.heygotrade.com/en/blog/broadcom-vs-marvell-custom-ai-silicon-battle-2026/)
- [Top Data Center REITs: EQIX, DLR, or IRM?](https://www.heygotrade.com/en/blog/Top-Data-Center-REITs-EQIX-DLR-IRM/)
- [3 Infrastructure Stocks Fueling the Data Center Building Boom | Investing.com](https://www.investing.com/analysis/3-infrastructure-stocks-fueling-the-data-center-building-boom-200680492)
- [Fiber Optics for Data Centers 2025 | Introl Blog](https://introl.com/blog/fiber-optics-data-center-state-of-art-optical-interconnect-2025)
- [Memory Chip Market Forecast 2026 | Ampheo](https://www.ampheo.com/blog/memory-chip-market-forecast-2026-dram-nand-hbm-and-nor-flash)
- [Wide Bandgap Semiconductor Market | GM Insights](https://www.gminsights.com/industry-analysis/wide-bandgap-semiconductors-market)
