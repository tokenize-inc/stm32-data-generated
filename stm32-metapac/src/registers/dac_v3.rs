
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Dac",
            extends: None,
            description: Some(
                "Digital-to-analog converter",
            ),
            items: &[
                BlockItem {
                    name: "cr",
                    description: Some(
                        "control register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "swtrigr",
                    description: Some(
                        "software trigger register",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Swtrigr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhr12r",
                    description: Some(
                        "channel 12-bit right-aligned data holding register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 12,
                            },
                        ),
                    ),
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhr12r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhr12l",
                    description: Some(
                        "channel 12-bit left-aligned data holding register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 12,
                            },
                        ),
                    ),
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhr12l",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhr8r",
                    description: Some(
                        "channel 8-bit right-aligned data holding register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 12,
                            },
                        ),
                    ),
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhr8r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhr12rd",
                    description: Some(
                        "dual 12-bit right-aligned data holding register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhr12rd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhr12ld",
                    description: Some(
                        "dual 12-bit left aligned data holding register",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhr12ld",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhr8rd",
                    description: Some(
                        "dual 8-bit right aligned data holding register",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhr8rd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dor",
                    description: Some(
                        "channel data output register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dor",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sr",
                    description: Some(
                        "status register",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccr",
                    description: Some(
                        "calibration control register",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mcr",
                    description: Some(
                        "mode control register",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "shsr",
                    description: Some(
                        "sample and hold sample time register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Shsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "shhr",
                    description: Some(
                        "sample and hold hold time register",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Shhr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "shrr",
                    description: Some(
                        "sample and hold refresh time register",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Shrr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ccr",
            extends: None,
            description: Some(
                "calibration control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "otrim",
                    description: Some(
                        "channel offset trimming value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 16,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "channel enable",
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
                                len: 2,
                                stride: 16,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "ten",
                    description: Some(
                        "channel trigger enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 16,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "tsel",
                    description: Some(
                        "channel trigger selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 3,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 16,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "wave",
                    description: Some(
                        "channel noise/triangle wave generation enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 16,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Wave",
                    ),
                },
                Field {
                    name: "mamp",
                    description: Some(
                        "channel mask/amplitude selector",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 16,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "dmaen",
                    description: Some(
                        "channel DMA enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 16,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "dmaudrie",
                    description: Some(
                        "channel DMA Underrun Interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 16,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "cen",
                    description: Some(
                        "DAC channel calibration enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 16,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhr12l",
            extends: None,
            description: Some(
                "channel 12-bit left-aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhr",
                    description: Some(
                        "channel 12-bit left-aligned data",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhr12ld",
            extends: None,
            description: Some(
                "dual 12-bit left aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhr",
                    description: Some(
                        "channel 12-bit left-aligned data",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 12,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 16,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhr12r",
            extends: None,
            description: Some(
                "channel 12-bit right-aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhr",
                    description: Some(
                        "channel 12-bit right-aligned data",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhr12rd",
            extends: None,
            description: Some(
                "dual 12-bit right-aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhr",
                    description: Some(
                        "channel 12-bit right-aligned data",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 16,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhr8r",
            extends: None,
            description: Some(
                "channel 8-bit right-aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhr",
                    description: Some(
                        "channel 8-bit right-aligned data",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhr8rd",
            extends: None,
            description: Some(
                "dual 8-bit right aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhr",
                    description: Some(
                        "channel 8-bit right-aligned data",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dor",
            extends: None,
            description: Some(
                "channel data output register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dor",
                    description: Some(
                        "channel data output",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mcr",
            extends: None,
            description: Some(
                "mode control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mode",
                    description: Some(
                        "DAC channel mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 16,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Mode",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Shhr",
            extends: None,
            description: Some(
                "sample and hold hold time register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "thold",
                    description: Some(
                        "channel hold time",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 16,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Shrr",
            extends: None,
            description: Some(
                "sample and hold refresh time register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trefresh",
                    description: Some(
                        "channel refresh time",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 16,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Shsr",
            extends: None,
            description: Some(
                "sample and hold sample time register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tsample",
                    description: Some(
                        "channel sample time",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sr",
            extends: None,
            description: Some(
                "status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmaudr",
                    description: Some(
                        "channel DMA underrun flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 16,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "cal_flag",
                    description: Some(
                        "channel calibration offset status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 16,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "bwst",
                    description: Some(
                        "channel busy writing sample time flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 16,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Swtrigr",
            extends: None,
            description: Some(
                "software trigger register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "swtrig",
                    description: Some(
                        "channel software trigger",
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
                                len: 2,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Mode",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "NORMAL_EXT_BUFEN",
                    description: Some(
                        "Normal mode, external pin only, buffer enabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "NORMAL_EXT_INT_BUFEN",
                    description: Some(
                        "Normal mode, external pin and internal peripherals, buffer enabled",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "NORMAL_EXT_BUFDIS",
                    description: Some(
                        "Normal mode, external pin only, buffer disabled",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "NORMAL_INT_BUFDIS",
                    description: Some(
                        "Normal mode, internal peripherals only, buffer disabled",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "SAMPHOLD_EXT_BUFEN",
                    description: Some(
                        "Sample and hold mode, external pin only, buffer enabled",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "SAMPHOLD_EXT_INT_BUFEN",
                    description: Some(
                        "Sample and hold mode, external pin and internal peripherals, buffer enabled",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "SAMPHOLD_EXT_INT_BUFDIS",
                    description: Some(
                        "Sample and hold mode, external pin and internal peripherals, buffer disabled",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "SAMPHOLD_INT_BUFDIS",
                    description: Some(
                        "Sample and hold mode, internal peripherals only, buffer disabled",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Wave",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "Wave generation disabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "NOISE",
                    description: Some(
                        "Noise wave generation enabled",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "TRIANGLE",
                    description: Some(
                        "Triangle wave generation enabled",
                    ),
                    value: 2,
                },
            ],
        },
    ],
};
                