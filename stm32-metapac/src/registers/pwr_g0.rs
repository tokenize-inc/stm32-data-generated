
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
                    name: "cr1",
                    description: Some(
                        "Power control register 1",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr2",
                    description: Some(
                        "Power control register 2",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr3",
                    description: Some(
                        "Power control register 3",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr4",
                    description: Some(
                        "Power control register 4",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sr1",
                    description: Some(
                        "Power status register 1",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Sr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sr2",
                    description: Some(
                        "Power status register 2",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Sr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "scr",
                    description: Some(
                        "Power status clear register",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Scr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pucr",
                    description: Some(
                        "Power Port pull-up control register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 6,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pdcr",
                    description: Some(
                        "Power Port pull-down control register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 6,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pcr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cr1",
            extends: None,
            description: Some(
                "Power control register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lpms",
                    description: Some(
                        "Low-power mode selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fpd_stop",
                    description: Some(
                        "Flash memory powered down during Stop mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fpd_lprun",
                    description: Some(
                        "Flash memory powered down during Low-power run mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fpd_lpslp",
                    description: Some(
                        "Flash memory powered down during Low-power sleep mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dbp",
                    description: Some(
                        "Disable backup domain write protection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vos",
                    description: Some(
                        "Voltage scaling range selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Vos",
                    ),
                },
                Field {
                    name: "lpr",
                    description: Some(
                        "Low-power run",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr2",
            extends: None,
            description: Some(
                "Power control register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pvde",
                    description: Some(
                        "Power voltage detector enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pvdft",
                    description: Some(
                        "Power voltage detector falling threshold selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pvdrt",
                    description: Some(
                        "Power voltage detector rising threshold selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr3",
            extends: None,
            description: Some(
                "Power control register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ewup",
                    description: Some(
                        "Enable Wakeup pin",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 6,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "rrs",
                    description: Some(
                        "SRAM retention in Standby mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ulpen",
                    description: Some(
                        "Enable the periodical sampling mode for PDR detection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "apc",
                    description: Some(
                        "Apply pull-up and pull-down configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eiwul",
                    description: Some(
                        "Enable internal wakeup line",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr4",
            extends: None,
            description: Some(
                "Power control register 4",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wp",
                    description: Some(
                        "Wakeup pin WKUP1 polarity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 6,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "vbe",
                    description: Some(
                        "VBAT battery charging enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vbrs",
                    description: Some(
                        "VBAT battery charging resistor selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pcr",
            extends: None,
            description: Some(
                "Power Port pull control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "p",
                    description: Some(
                        "Port pull bit y (y=0..15)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Scr",
            extends: None,
            description: Some(
                "Power status clear register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cwuf",
                    description: Some(
                        "Clear Wakeup flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 6,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "csbf",
                    description: Some(
                        "Clear standby flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sr1",
            extends: None,
            description: Some(
                "Power status register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wuf",
                    description: Some(
                        "Wakeup flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 6,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "sbf",
                    description: Some(
                        "Standby flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wufi",
                    description: Some(
                        "Wakeup flag internal",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sr2",
            extends: None,
            description: Some(
                "Power status register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "flash_rdy",
                    description: Some(
                        "Flash ready flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "reglps",
                    description: Some(
                        "Low-power regulator started",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "reglpf",
                    description: Some(
                        "Low-power regulator flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vosf",
                    description: Some(
                        "Voltage scaling flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pvdo",
                    description: Some(
                        "Power voltage detector output",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
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
                    name: "RANGE1",
                    description: None,
                    value: 1,
                },
                EnumVariant {
                    name: "RANGE2",
                    description: None,
                    value: 2,
                },
            ],
        },
    ],
};
                