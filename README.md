# andes-riscv

SCOUNTEREN (0x106)
MILMB (0x7C0)
MDLMB (0x7C1)
MECC_CODE (0x7C2)
MNVEC (0x7C3)
MXSTATUS (0x7C4)
MPFT_CTL (0x7C5)
MHSP_CTL (0x7C6)
MSP_BOUND (0x7C7)
MSP_BASE (0x7C8)
MDCAUSE (0x7C9)
MCACHE_CTL (0x7CA)
MCCTLBEGINADDR (0x7CB)
MCCTLCOMMAND (0x7CC)
MCCTLDATA (0x7CD)
MCOUNTERWEN (0x7CE)
MCOUNTERINTEN (0x7CF)
MMISC_CTL (0x7D0)
MCOUNTERMASK_M (0x7D1)
MCOUNTERMASK_S (0x7D2)
MCOUNTERMASK_U (0x7D3)
MCOUNTEROVF (0x7D4)
MSLIDELEG (0x7D5)
MCLK_CTL (0x7DF)
DEXC2DBG (0x7E0)
DDCAUSE (0x7E1)
UITB (0x800)
UCODE (0x801)
UDCAUSE (0x809)
UCCTLBEGINADDR (0x80B)
UCCTLCOMMAND (0x80C)
SLIE (0x9C4)
SLIP (0x9C5)
SDCAUSE (0x9C9)
SCCTLDATA (0x9CD)
SCOUNTERINTEN (0x9CF)
SCOUNTERMASK_M (0x9D1)
SCOUNTERMASK_S (0x9D2)
SCOUNTERMASK_U (0x9D3)
SCOUNTEROVF (0x9D4)
SCOUNTINHIBIT (0x9E0)
SHPMEVENT3 (0x9E3)
SHPMEVENT4 (0x9E4)
SHPMEVENT5 (0x9E5)
SHPMEVENT6 (0x9E6)
MICM_CFG (0xFC0)
MDCM_CFG (0xFC1)
MMSC_CFG (0xFC2)
MMSC_CFG2 (0xFC3)

## CSRs

### AndeStar V5 machine mode CSRs

Configuration Registers

micm_cfg
mdcm_cfg
mmsc_cfg
mmsc_cfg2
mvec_cfg
mccache_ctl_base
mrvarch_cfg

| Address | Privilege | Name | Description |
|---------|-----------|------|-------------|
| 0xFC0 | MRO | micm_cfg | Instruction cache/memory configuration. |
| 0xFC1 | MRO | mdcm_cfg | Data cache/memory configuration. |
| 0xFC2 | MRO | mmsc_cfg | Miscellaneous configuration. |
| 0xFC3 | MRO | mmsc_cfg2 | Miscellaneous configuration. (RV32) |
| 0xFC7 | MRO | mvec_cfg | Vector processor configuration. |
| 0xFCF | MRO | mccache_ctl_base | Cluster cache control base address |
| 0xFCA | MRO | mrvarch_cfg | RISC-V Architecture |

Crash Debug CSRs

mcrash_statesave
mstatus_crashsave

| Address | Privilege | Name | Description |
|---------|-----------|-------------------|----------------------------------------|
| 0xFC8 | MRO | mcrash_statesave | Current state save for crash debugging |
| 0xFC9 | MRO | mstatus_crashsave | mstatus state save for crash debugging |

Memory CSRs

milmb
mdlmb
mecccode
mnvec
mcache_ctl
mcctlbeginaddr
mcctlcommand
mcctldata
mppib
mfiob

| Address | Privilege | Name | Description |
|---------|-----------|----------------------|----------------------------------------|
| 0x7C0 | MRW | milmb | Instruction local memory base address. |
| 0x7C1 | MRW | mdlmb | Data local memory base address. |
| 0x7C2 | MRW | mecc_code | ECC code. |
| 0x7C3 | MRW | mnvec | NMI-handler base address. |
| 0x7CA | MRW | mcache_ctl | Cache control |
| 0x7CB | MRW | mcctlbeginaddr | CCTL begin address |
| 0x7CC | MRW | mcctlcommand | CCTL command |
| 0x7CD | MRW | mcctldata | CCTL data |
| 0x7F0 | MRW | mppib | Private peripheral interface base address |
| 0x7F1 | MRW | mfiob | Fast IO interface base address |

Hardware Stack Protection & Recording

mhsp_ctl
msp_bound
msp_base

| Address | Privilege | Name | Description |
|---------|-----------|----------------|--------------------------------|
| 0x7C6 | MRW | mhsp_ctl | Hardware stack protection control |
| 0x7C7 | MRW | msp_bound | SP bound register |
| 0x7C8 | MRW | msp_base | SP base register |

Trap related CSR

| Address | Privilege | Name | Description |
|---------|------------|----------------|----------------------------------------|
| 0x7C4 | MRW | mxstatus | Additional machine mode status |
| 0x7C9 | MRW | mdcause | Detailed exception cause |
| 0x7D5 | MRW | mslidedleg | Supervisor local interrupt delegation |
| 0x7D6 | MRW | msavestatus | Status save register (level 1 & level 2) |
| 0x7D7 | MRW | msaveepc1 | EPC save register (level 1) |
| 0x7D8 | MRW | msavecause1 | Exception cause save register (level 1) |
| 0x7D9 | MRW | msaveepc2 | EPC save register (level 2) |
| 0x7DA | MRW | msavecause2 | Exception cause save register (level 2) |
| 0x7DB | MRW | msavedcause1 | Detailed exception cause save (level 1) |
| 0x7DC | MRW | msavedcause2 | Detailed exception cause save (level 2) |

Control CSRs

| Address | Privilege | Name | Description |
|---------|------------|--------------|--------------------------------|
| 0x7C5 | MRW | mpft_ctl | Performance throttling control |
| 0x7D0 | MRW | mmisc_ctl | Miscellaneous control |
| 0x7DF | MRW | mclk_ctl | Clock control |

Counter related CSRs

| Address | Privilege | Name | Description |
|---------|------------|-----------------|--------------------------------|
| 0x7CE | MRW | mcounterwen | Counter write enable |
| 0x7CF | MRW | mcounteriinten | Counter overflow interrupt enable |
| 0x7D1 | MRW | mcountermask_m | Counter not counting in M-mode |
| 0x7D2 | MRW | mcountermask_s | Counter not counting in S-mode |
| 0x7D3 | MRW | mcountermask_u | Counter not counting in U-mode |
| 0x7D4 | MRW1C | mcounterrovf | Counter overflow status |

Enhanced CLIC CSRs

| Address | Privilege | Name | Description |
|---------|------------|-------------------|-------------------------------|
| 0x7EC | MRW | mirq_entry | Interrupt common entry point |
| 0x7ED | MRW | mintsel_jal | Select interrupt and call ISR |
| 0x7EE | MRW | pushmcause | Store mcause to stack |
| 0x7EF | MRW | pushmepc | Store mepc to stack |
| 0x7EB | MRW | pushmxstatus | Store mxstatus to stack |

Physical Memory Attribute CSRs

| Address | Privilege | Name | Description |
|---------|------------|----------------|----------------------|
| 0xBC0 | MRW | pmacfg0 | PMA configuration |
| 0xBC1 | MRW | pmpcfg1 | PMA configuration (RV32 only) |
| 0xBC2 | MRW | pmpcfg2 | PMA configuration |
| 0xBC3 | MRW | pmpcfg3 | PMA configuration (RV32 only) |
| 0xBD0~BDF | MRW | pmaaddr0~15 | PMA address |

### AndeStar V5 debug mode CSRs

| Address | Privilege | Name | Description |
|---------|------------|------------|----------------------------------------|
| 0x7E0 | DRW | dexc2dbg | Enable exception to enter Halt Mode. |
| 0x7E1 | DRW | ddcause | Detailed exception type information when an exception enters Halt Mode. |

### AndeStar V5 supervisor mode CSRs

Supervisor Trap Related

| Address | Privilege | Name | Description |
|---------|------------|------|--------------------------------|
| 0x9C4 | SRW | slie | Supervisor local interrupt enable |
| 0x9C5 | SRW | slip | Supervisor local interrupt pending |
| 0x9C9 | SRW | sdcause | Detailed exception cause |

Supervisor Counter Related

| Address | Privilege | Name | Description |
|---------|------------|------------------|------------------------------|
| 0x9CF | SRO/SRW | scounteri nten | Counter overflow interrupt enable |
| 0x9D1 | SRO/SRW | scountermask_m | Counter mask for M-mode |
| 0x9D2 | SRO/SRW | scountermask_s | Counter mask for S-mode |
| 0x9D3 | SRO/SRW | scountermask_u | Counter mask for U-mode |
| 0x9D4 | SRO/SRW1C | scounterrovf | Counter overflow status |
| 0x9E0 | SRO/SRW | scountiinhibit | Supervisor counter inhibit |
| 0x9E3~6 | SRO/SRW | shpmevent3~6 | Performance monitoring event selection |

Supervisor Control

| Address | Privilege | Name | Description |
|---------|------------|------------|-------------|
| 0x9CD | SRW | scctldata | CCTL data |
| 0x9D0 | SRW | smisc_ctl | Miscellaneous control |

### AndeStar V5 user mode CSRs

| Address | Privilege | Name | Description |
|---------|------------|------|--------------------------------------|
| 0x800 | URW | uitb | Instruction table base address for CoDense extension. |
| 0x801 | URW | ucode | Contains overflow flag for DSP extension. |
| 0x809 | URW | udcause | User detailed trap cause |
| 0x80B | URW | ucctlbeginaddr | CCTL begin address |
| 0x80C | URW | ucctlcommand | CCTL command |
| 0x810 | URW | wfe | Wait for event control |
| 0x811 | URW | sleepvalue | Sleep value |
| 0x812 | URW | txevt | Transmit event |
