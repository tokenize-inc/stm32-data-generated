
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Pwr",
            extends: None,
            description: Some(
                "Power control",
            ),
            items: &[
                BlockItem {
                    name: "pmcr",
                    description: Some(
                        "PWR power mode control register",
                    ),
                    array: None,
                    byte_offset: 0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pmcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pmsr",
                    description: Some(
                        "PWR status register",
                    ),
                    array: None,
                    byte_offset: 4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pmsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "voscr",
                    description: Some(
                        "PWR voltage scaling control register",
                    ),
                    array: None,
                    byte_offset: 16,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Voscr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vossr",
                    description: Some(
                        "PWR voltage scaling status register",
                    ),
                    array: None,
                    byte_offset: 20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vossr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bdcr",
                    description: Some(
                        "PWR Backup domain control register",
                    ),
                    array: None,
                    byte_offset: 32,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bdcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dbpcr",
                    description: Some(
                        "PWR Backup domain control register",
                    ),
                    array: None,
                    byte_offset: 36,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dbpcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bdsr",
                    description: Some(
                        "PWR Backup domain status register",
                    ),
                    array: None,
                    byte_offset: 40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bdsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ucpdr",
                    description: Some(
                        "PWR USB Type-C power delivery register",
                    ),
                    array: None,
                    byte_offset: 44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ucpdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sccr",
                    description: Some(
                        "PWR supply configuration control register",
                    ),
                    array: None,
                    byte_offset: 48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vmcr",
                    description: Some(
                        "PWR voltage monitor control register",
                    ),
                    array: None,
                    byte_offset: 52,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vmcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usbscr",
                    description: Some(
                        "PWR USB supply control register",
                    ),
                    array: None,
                    byte_offset: 56,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Usbscr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vmsr",
                    description: Some(
                        "PWR voltage monitor status register",
                    ),
                    array: None,
                    byte_offset: 60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vmsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wuscr",
                    description: Some(
                        "PWR wakeup status clear register",
                    ),
                    array: None,
                    byte_offset: 64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wuscr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wusr",
                    description: Some(
                        "PWR wakeup status register",
                    ),
                    array: None,
                    byte_offset: 68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wusr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wucr",
                    description: Some(
                        "PWR wakeup configuration register",
                    ),
                    array: None,
                    byte_offset: 72,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wucr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ioretr",
                    description: Some(
                        "PWR I/O retention register",
                    ),
                    array: None,
                    byte_offset: 80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ioretr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "seccfgr",
                    description: Some(
                        "PWR security configuration register",
                    ),
                    array: None,
                    byte_offset: 256,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Seccfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "privcfgr",
                    description: Some(
                        "PWR privilege configuration register",
                    ),
                    array: None,
                    byte_offset: 260,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Privcfgr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Wuscr",
            extends: None,
            description: Some(
                "PWR wakeup status clear register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cwuf",
                    description: Some(
                        "clear wakeup pin flag for WUFx\r These bits are always read as 0.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Usbscr",
            extends: None,
            description: Some(
                "PWR USB supply control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usb33den",
                    description: Some(
                        "V<sub>DDUSB</sub> voltage level detector enable",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usb33sv",
                    description: Some(
                        "independent USB supply valid\r This bit is used to validate the V<sub>DDUSB</sub> supply for electrical and logical isolation purpose. Setting this bit is mandatory to use the USBFS peripheral. If V<sub>DDUSB</sub> is not always present in the application, the V<sub>DDUSB</sub> voltage monitor can be used to determine whether this supply is ready or not.",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pmsr",
            extends: None,
            description: Some(
                "PWR status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "stopf",
                    description: Some(
                        "Stop flag\r This bit is set by hardware and cleared only by any reset or by setting the CSSF bit.",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sbf",
                    description: Some(
                        "System standby flag\r This bit is set by hardware and cleared only by a POR or by setting the CSSF bit.",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vmcr",
            extends: None,
            description: Some(
                "PWR voltage monitor control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pvde",
                    description: Some(
                        "PVD enable",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pls",
                    description: Some(
                        "programmable voltage detector (PVD) level selection\r These bits select the voltage threshold detected by the PVD.",
                    ),
                    bit_offset: 1,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Pls",
                    ),
                },
                Field {
                    name: "avden",
                    description: Some(
                        "peripheral voltage monitor on V<sub>DDA</sub> enable",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "als",
                    description: Some(
                        "analog voltage detector (AVD) level selection\r These bits select the voltage threshold detected by the AVD.",
                    ),
                    bit_offset: 9,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Als",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Wusr",
            extends: None,
            description: Some(
                "PWR wakeup status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wuf",
                    description: Some(
                        "wakeup pin WUFx flag\r This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Privcfgr",
            extends: None,
            description: Some(
                "PWR privilege configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spriv",
                    description: Some(
                        "PWR secure functions privilege configuration\r Set and reset by software. This bit can be written only by a secure privileged access.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Priv",
                    ),
                },
                Field {
                    name: "nspriv",
                    description: Some(
                        "PWR non-secure functions privilege configuration\r Set and reset by software. This bit can be written only by privileged access, secure or non-secure.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Priv",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Seccfgr",
            extends: None,
            description: Some(
                "PWR security configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wup1sec",
                    description: Some(
                        "WUPx secure protection",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sec",
                    ),
                },
                Field {
                    name: "wup2sec",
                    description: Some(
                        "WUPx secure protection",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sec",
                    ),
                },
                Field {
                    name: "wup3sec",
                    description: Some(
                        "WUPx secure protection",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sec",
                    ),
                },
                Field {
                    name: "wup4sec",
                    description: Some(
                        "WUPx secure protection",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sec",
                    ),
                },
                Field {
                    name: "wup5sec",
                    description: Some(
                        "WUPx secure protection",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sec",
                    ),
                },
                Field {
                    name: "wup6sec",
                    description: Some(
                        "WUPx secure protection",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sec",
                    ),
                },
                Field {
                    name: "wup7sec",
                    description: Some(
                        "WUPx secure protection",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sec",
                    ),
                },
                Field {
                    name: "wup8sec",
                    description: Some(
                        "WUPx secure protection",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sec",
                    ),
                },
                Field {
                    name: "retsec",
                    description: Some(
                        "retention secure protection",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sec",
                    ),
                },
                Field {
                    name: "lpmsec",
                    description: Some(
                        "low-power modes secure protection",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sec",
                    ),
                },
                Field {
                    name: "scmsec",
                    description: Some(
                        "supply configuration and monitoring secure protection.",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sec",
                    ),
                },
                Field {
                    name: "vbsec",
                    description: Some(
                        "backup domain secure protection",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sec",
                    ),
                },
                Field {
                    name: "vusbsec",
                    description: Some(
                        "voltage USB secure protection",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sec",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Vmsr",
            extends: None,
            description: Some(
                "PWR voltage monitor status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "avdo",
                    description: Some(
                        "analog voltage detector output on V<sub>DDA</sub>\r This bit is set and cleared by hardware. It is valid only if AVD on VDDA is enabled by the AVDEN bit.\r Note: Since the AVD is disabled in Standby mode, this bit is equal to 0 after standby or reset until the AVDEN bit is set.",
                    ),
                    bit_offset: 19,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Avdo",
                    ),
                },
                Field {
                    name: "vddio2rdy",
                    description: Some(
                        "voltage detector output on V<sub>DDIO2</sub>\r This bit is set and cleared by hardware.",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pvdo",
                    description: Some(
                        "programmable voltage detect output\r This bit is set and cleared by hardware. It is valid only if the PVD has been enabled by the PVDE bit.\r Note: Since the PVD is disabled in Standby mode, this bit is equal to 0 after Standby or reset until the PVDE bit is set.",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pvdo",
                    ),
                },
                Field {
                    name: "usb33rdy",
                    description: Some(
                        "V<sub>DDUSB</sub> ready",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ucpdr",
            extends: None,
            description: Some(
                "PWR USB Type-C power delivery register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ucpd_dbdis",
                    description: Some(
                        "USB Type-C and power delivery dead battery disable\r After exiting reset, the USB Type-C “dead battery” behavior is enabled, which may have a pull-down effect on CC1 and CC2 pins. It is recommended to disable it in all case, either to stop this pull-down or to hand over control to the UCPD (which should therefore be initialized before doing the disable).",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ucpd_stby",
                    description: Some(
                        "USB Type-c and Power delivery Standby mode\r When set, this bit is used to memorize the UCPD configuration in Standby mode. This bit must be written to 1 just before entering Standby mode when using UCPD, and it must be written to 0 after exiting the standby mode and before writing any UCPD register.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Voscr",
            extends: None,
            description: Some(
                "PWR voltage scaling control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vos",
                    description: Some(
                        "voltage scaling selection according to performance\r These bits control the V<sub>CORE</sub> voltage level and allow to obtain the best trade-off between power consumption and performance:\r - In bypass mode, these bits must also be set according to the external provided core voltage level and related performance.\r - When increasing the performance, the voltage scaling must be changed before increasing the system frequency.\r - When decreasing performance, the system frequency must first be decreased before changing the voltage scaling.",
                    ),
                    bit_offset: 4,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Vos",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Sccr",
            extends: None,
            description: Some(
                "PWR supply configuration control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bypass",
                    description: Some(
                        "power management unit bypass",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ldoen",
                    description: Some(
                        "LDO enable \r The value is set by hardware when the package uses the LDO regulator.",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "smpsen",
                    description: Some(
                        "SMPS enable \r The value is set by hardware when the package uses the SMPS regulator.",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wucr",
            extends: None,
            description: Some(
                "PWR wakeup configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wupen",
                    description: Some(
                        "enable wakeup pin WUPx\r These bits are set and cleared by software.\r Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "wupp",
                    description: Some(
                        "wakeup pin polarity bit for WUPx\r These bits define the polarity used for event detection on WUPx external wakeup pin.",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Wupp",
                    ),
                },
                Field {
                    name: "wuppupd",
                    description: Some(
                        "wakeup pin pull configuration for WKUPx\r These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.",
                    ),
                    bit_offset: 16,
                    bit_size: 2,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 2,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Wuppupd",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Ioretr",
            extends: None,
            description: Some(
                "PWR I/O retention register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ioreten",
                    description: Some(
                        "IO retention enable:\r When entering into standby mode, the output is sampled, and apply to the output IO during the standby power mode. \r Note: the IO state is not retained if the DBG_STANDBY bit is set in DBGMCU_CR register.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jtagioreten",
                    description: Some(
                        "IO retention enable for JTAG IOs\r when entering into standby mode, the output is sampled, and apply to the output IO during the standby power mode",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bdsr",
            extends: None,
            description: Some(
                "PWR Backup domain status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "brrdy",
                    description: Some(
                        "backup regulator ready\r This bit is set by hardware to indicate that the backup regulator is ready.",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vbatl",
                    description: Some(
                        "V<sub>BAT</sub> level monitoring versus low threshold",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vbath",
                    description: Some(
                        "V<sub>BAT</sub> level monitoring versus high threshold",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "templ",
                    description: Some(
                        "temperature level monitoring versus low threshold",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "temph",
                    description: Some(
                        "temperature level monitoring versus high threshold",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vossr",
            extends: None,
            description: Some(
                "PWR voltage scaling status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vosrdy",
                    description: Some(
                        "Ready bit for V<sub>CORE</sub> voltage scaling output selection.",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "actvosrdy",
                    description: Some(
                        "Voltage level ready for currently used VOS",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "actvos",
                    description: Some(
                        "voltage output scaling currently applied to V<sub>CORE</sub>\r This field provides the last VOS value.",
                    ),
                    bit_offset: 14,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Actvos",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Bdcr",
            extends: None,
            description: Some(
                "PWR Backup domain control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bren",
                    description: Some(
                        "Backup RAM retention in Standby and V<sub>BAT</sub> modes\r When this bit set, the backup regulator (used to maintain the backup RAM content in Standby and V<sub>BAT</sub> modes) is enabled.\r If BREN is cleared, the backup regulator is switched off. The backup RAM can still be used in \tRun and Stop modes. However its content is lost in Standby and V<sub>BAT</sub> modes.\r If BREN is set, the application must wait till the backup regulator ready flag (BRRDY) is set to indicate that the data written into the SRAM is maintained in Standby and V<sub>BAT</sub> modes.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "monen",
                    description: Some(
                        "Backup domain voltage and temperature monitoring enable",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vbe",
                    description: Some(
                        "V<sub>BAT</sub> charging enable\r Note: Reset only by POR,.",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vbrs",
                    description: Some(
                        "V<sub>BAT</sub> charging resistor selection",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vbrs",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Pmcr",
            extends: None,
            description: Some(
                "PWR power mode control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lpms",
                    description: Some(
                        "low-power mode selection\r This bit defines the Deepsleep mode.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "svos",
                    description: Some(
                        "system Stop mode voltage scaling selection\r These bits control the V<sub>CORE</sub> voltage level in system Stop mode, to obtain the best trade-off between power consumption and performance.",
                    ),
                    bit_offset: 2,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Svos",
                    ),
                },
                Field {
                    name: "cssf",
                    description: Some(
                        "clear Standby and Stop flags (always read as 0)\r This bit is cleared to 0 by hardware.",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flps",
                    description: Some(
                        "Flash memory low-power mode in Stop mode\r This bit is used to obtain the best trade-off between low-power consumption and restart time when exiting from Stop mode.\r When it is set, the Flash memory enters low-power mode when the CPU domain is in Stop mode.\r Note: When system enters stop mode with SVOS5 enabled, Flash memory is automatically forced in low-power mode.",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "booste",
                    description: Some(
                        "analog switch V<sub>BOOST</sub> control\r This bit enables the booster to guarantee the analog switch AC performance when the V<sub>DD</sub> supply voltage is below 2.7 V (reduction of the total harmonic distortion to have the same switch performance over the full supply voltage range) The V<sub>DD</sub> supply voltage can be monitored through the PVD and the PLS bits.",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "avd_ready",
                    description: Some(
                        "analog voltage ready\r This bit is only used when the analog switch boost needs to be enabled (see BOOSTE bit).\r It must be set by software when the expected V<sub>DDA</sub> analog supply level is available.\r The correct analog supply level is indicated by the AVDO bit (PWR_VMSR register) after setting the AVDEN bit (PWR_VMCR register) and selecting the supply level to be monitored \t(ALS bits).",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ethernetso",
                    description: Some(
                        "ETHERNET RAM shut-off in Stop mode.",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram3so",
                    description: Some(
                        "AHB SRAM3 shut-off in Stop mode.",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram2_16so",
                    description: Some(
                        "AHB SRAM2 16-Kbyte shut-off in Stop mode.",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram2_48so",
                    description: Some(
                        "AHB SRAM2 48-Kbyte shut-off in Stop mode.",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram1so",
                    description: Some(
                        "AHB SRAM1 shut-off in Stop mode",
                    ),
                    bit_offset: 26,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dbpcr",
            extends: None,
            description: Some(
                "PWR Backup domain control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dbp",
                    description: Some(
                        "Disable Backup domain write protection\r In reset state, all registers and SRAM in Backup domain are protected against parasitic write \taccess. This bit must be set to enable write access to these registers.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Vos",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "SCALE3",
                    description: Some(
                        "scale 3 (default)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SCALE2",
                    description: Some(
                        "scale 2",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "SCALE1",
                    description: Some(
                        "scale 1",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "SCALE0",
                    description: Some(
                        "scale 0",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Actvos",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "VOS3 (lowest power)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "VOS2",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "VOS1",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "VOS0 (highest frequency)",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Priv",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Read and write to PWR secure functions can be done by privileged or unprivileged access.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Read and write to PWR secure functions can be done by privileged access only.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pvdo",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "V<sub>DD</sub> is equal or higher than the PVD threshold selected through the PLS[2:0] bits.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "V<sub>DD</sub> is lower than the PVD threshold selected through the PLS[2:0] bits.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pls",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "1.95 V",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "2.1 V",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "2.25 V",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "2.4 V",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "B_0X4",
                    description: Some(
                        "2.55 V",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "B_0X5",
                    description: Some(
                        "2.7 V",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "B_0X6",
                    description: Some(
                        "2.85 V",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "B_0X7",
                    description: Some(
                        "PVD_IN pin",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Wuppupd",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "no pull-up",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "pull-up",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "pull-down",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "reserved",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Avdo",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "V<sub>DDA</sub> is equal or higher than the AVD threshold selected with the ALS[2:0] bits.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "V<sub>DDA</sub> is lower than the AVD threshold selected with the ALS[2:0] bits.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Wupp",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "detection on high level (rising edge)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "detection on low level (falling edge)",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vbrs",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "Charge V<sub>BAT</sub> through a 5 kΩ resistor.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "Charge V<sub>BAT</sub> through a 1.5 kΩ resistor.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Svos",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "reserved",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "SVOS5 scale 5",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "SVOS4 scale 4",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "SVOS3 scale 3 (default).",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Sec",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "PWR_SCCR and PWR_VMCR can be read and written with secure or non-secure access.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "PWR_SCCR and PWR_VMCR can be read and written only with secure access.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Als",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "1.7 V",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "2.1 V",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2",
                    description: Some(
                        "2.5 V",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3",
                    description: Some(
                        "2.8 V",
                    ),
                    value: 3,
                },
            ],
        },
    ],
};
