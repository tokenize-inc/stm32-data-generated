
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Jpeg",
            extends: None,
            description: Some(
                "JPEG codec",
            ),
            items: &[
                BlockItem {
                    name: "jpeg_confr0",
                    description: Some(
                        "JPEG codec configuration register 0",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "JpegConfr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "jpeg_confr1",
                    description: Some(
                        "JPEG codec configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "JpegConfr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "jpeg_confr2",
                    description: Some(
                        "JPEG codec configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "JpegConfr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "jpeg_confr3",
                    description: Some(
                        "JPEG codec configuration register 3",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "JpegConfr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "jpeg_confr4",
                    description: Some(
                        "JPEG codec configuration register 4",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "JpegConfr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "jpeg_confr5",
                    description: Some(
                        "JPEG codec configuration register 5",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "JpegConfr5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "jpeg_confr6",
                    description: Some(
                        "JPEG codec configuration register 6",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "JpegConfr6",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "jpeg_confr7",
                    description: Some(
                        "JPEG codec configuration register 7",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "JpegConfr7",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "jpeg_cr",
                    description: Some(
                        "JPEG control register",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "JpegCr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "jpeg_sr",
                    description: Some(
                        "JPEG status register",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "JpegSr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "jpeg_cfr",
                    description: Some(
                        "JPEG clear flag register",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "JpegCfr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "jpeg_dir",
                    description: Some(
                        "JPEG data input register",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "JpegDir",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "jpeg_dor",
                    description: Some(
                        "JPEG data output register",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "JpegDor",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem0_0",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem00",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem0_1",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem01",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem0_2",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem02",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem0_3",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem03",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem0_4",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem04",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem0_5",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem05",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem0_6",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem06",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem0_7",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem07",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem0_8",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem08",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem0_9",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem09",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem0_10",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem010",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem0_11",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x7c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem011",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem0_12",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem012",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem0_13",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem013",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem0_14",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem014",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem0_15",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem015",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem1_0",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem10",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem1_1",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem11",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem1_2",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x98,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem12",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem1_3",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x9c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem13",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem1_4",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem14",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem1_5",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0xa4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem15",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem1_6",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem16",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem1_7",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0xac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem17",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem1_8",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem18",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem1_9",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0xb4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem19",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem1_10",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0xb8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem110",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem1_11",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0xbc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem111",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem1_12",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem112",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem1_13",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem113",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem1_14",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0xc8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem114",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem1_15",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0xcc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem115",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem2_0",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0xd0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem20",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem2_1",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0xd4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem21",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem2_2",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0xd8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem22",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem2_3",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0xdc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem23",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem2_4",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0xe0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem24",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem2_5",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0xe4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem25",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem2_6",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0xe8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem26",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem2_7",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0xec,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem27",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem2_8",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0xf0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem28",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem2_9",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0xf4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem29",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem2_10",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0xf8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem210",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem2_11",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0xfc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem211",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem2_12",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem212",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem2_13",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem213",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem2_14",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem214",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem2_15",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x10c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem215",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem3_0",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem30",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem3_1",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x114,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem31",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem3_2",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x118,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem32",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem3_3",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x11c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem33",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem3_4",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x120,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem34",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem3_5",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x124,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem35",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem3_6",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x128,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem36",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem3_7",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x12c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem37",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem3_8",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x130,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem38",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem3_9",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x134,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem39",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem3_10",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x138,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem310",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem3_11",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x13c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem311",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem3_12",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x140,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem312",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem3_13",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x144,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem313",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem3_14",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x148,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem314",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "qmem3_15",
                    description: Some(
                        "JPEG quantization tables",
                    ),
                    array: None,
                    byte_offset: 0x14c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Qmem315",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffmin_0",
                    description: Some(
                        "JPEG HuffMin tables",
                    ),
                    array: None,
                    byte_offset: 0x150,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffmin0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffmin_1",
                    description: Some(
                        "JPEG HuffMin tables",
                    ),
                    array: None,
                    byte_offset: 0x154,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffmin1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffmin_2",
                    description: Some(
                        "JPEG HuffMin tables",
                    ),
                    array: None,
                    byte_offset: 0x158,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffmin2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffmin_3",
                    description: Some(
                        "JPEG HuffMin tables",
                    ),
                    array: None,
                    byte_offset: 0x15c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffmin3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffmin_4",
                    description: Some(
                        "JPEG HuffMin tables",
                    ),
                    array: None,
                    byte_offset: 0x160,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffmin4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffmin_5",
                    description: Some(
                        "JPEG HuffMin tables",
                    ),
                    array: None,
                    byte_offset: 0x164,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffmin5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffmin_6",
                    description: Some(
                        "JPEG HuffMin tables",
                    ),
                    array: None,
                    byte_offset: 0x168,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffmin6",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffmin_7",
                    description: Some(
                        "JPEG HuffMin tables",
                    ),
                    array: None,
                    byte_offset: 0x16c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffmin7",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffmin_8",
                    description: Some(
                        "JPEG HuffMin tables",
                    ),
                    array: None,
                    byte_offset: 0x170,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffmin8",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffmin_9",
                    description: Some(
                        "JPEG HuffMin tables",
                    ),
                    array: None,
                    byte_offset: 0x174,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffmin9",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffmin_10",
                    description: Some(
                        "JPEG HuffMin tables",
                    ),
                    array: None,
                    byte_offset: 0x178,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffmin10",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffmin_11",
                    description: Some(
                        "JPEG HuffMin tables",
                    ),
                    array: None,
                    byte_offset: 0x17c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffmin11",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffmin_12",
                    description: Some(
                        "JPEG HuffMin tables",
                    ),
                    array: None,
                    byte_offset: 0x180,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffmin12",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffmin_13",
                    description: Some(
                        "JPEG HuffMin tables",
                    ),
                    array: None,
                    byte_offset: 0x184,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffmin13",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffmin_14",
                    description: Some(
                        "JPEG HuffMin tables",
                    ),
                    array: None,
                    byte_offset: 0x188,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffmin14",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffmin_15",
                    description: Some(
                        "JPEG HuffMin tables",
                    ),
                    array: None,
                    byte_offset: 0x18c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffmin15",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffbase0",
                    description: Some(
                        "JPEG HuffSymb tables",
                    ),
                    array: None,
                    byte_offset: 0x190,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffbase0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffbase1",
                    description: Some(
                        "JPEG HuffSymb tables",
                    ),
                    array: None,
                    byte_offset: 0x194,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffbase1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffbase2",
                    description: Some(
                        "JPEG HuffSymb tables",
                    ),
                    array: None,
                    byte_offset: 0x198,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffbase2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffbase3",
                    description: Some(
                        "JPEG HuffSymb tables",
                    ),
                    array: None,
                    byte_offset: 0x19c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffbase3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffbase4",
                    description: Some(
                        "JPEG HuffSymb tables",
                    ),
                    array: None,
                    byte_offset: 0x1a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffbase4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffbase5",
                    description: Some(
                        "JPEG HuffSymb tables",
                    ),
                    array: None,
                    byte_offset: 0x1a4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffbase5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffbase6",
                    description: Some(
                        "JPEG HuffSymb tables",
                    ),
                    array: None,
                    byte_offset: 0x1a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffbase6",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffbase7",
                    description: Some(
                        "JPEG HuffSymb tables",
                    ),
                    array: None,
                    byte_offset: 0x1ac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffbase7",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffbase8",
                    description: Some(
                        "JPEG HuffSymb tables",
                    ),
                    array: None,
                    byte_offset: 0x1b0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffbase8",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffbase9",
                    description: Some(
                        "JPEG HuffSymb tables",
                    ),
                    array: None,
                    byte_offset: 0x1b4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffbase9",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffbase10",
                    description: Some(
                        "JPEG HuffSymb tables",
                    ),
                    array: None,
                    byte_offset: 0x1b8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffbase10",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffbase11",
                    description: Some(
                        "JPEG HuffSymb tables",
                    ),
                    array: None,
                    byte_offset: 0x1bc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffbase11",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffbase12",
                    description: Some(
                        "JPEG HuffSymb tables",
                    ),
                    array: None,
                    byte_offset: 0x1c0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffbase12",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffbase13",
                    description: Some(
                        "JPEG HuffSymb tables",
                    ),
                    array: None,
                    byte_offset: 0x1c4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffbase13",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffbase14",
                    description: Some(
                        "JPEG HuffSymb tables",
                    ),
                    array: None,
                    byte_offset: 0x1c8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffbase14",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffbase15",
                    description: Some(
                        "JPEG HuffSymb tables",
                    ),
                    array: None,
                    byte_offset: 0x1cc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffbase15",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffbase16",
                    description: Some(
                        "JPEG HuffSymb tables",
                    ),
                    array: None,
                    byte_offset: 0x1d0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffbase16",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffbase17",
                    description: Some(
                        "JPEG HuffSymb tables",
                    ),
                    array: None,
                    byte_offset: 0x1d4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffbase17",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffbase18",
                    description: Some(
                        "JPEG HuffSymb tables",
                    ),
                    array: None,
                    byte_offset: 0x1d8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffbase18",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffbase19",
                    description: Some(
                        "JPEG HuffSymb tables",
                    ),
                    array: None,
                    byte_offset: 0x1dc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffbase19",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffbase20",
                    description: Some(
                        "JPEG HuffSymb tables",
                    ),
                    array: None,
                    byte_offset: 0x1e0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffbase20",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffbase21",
                    description: Some(
                        "JPEG HuffSymb tables",
                    ),
                    array: None,
                    byte_offset: 0x1e4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffbase21",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffbase22",
                    description: Some(
                        "JPEG HuffSymb tables",
                    ),
                    array: None,
                    byte_offset: 0x1e8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffbase22",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffbase23",
                    description: Some(
                        "JPEG HuffSymb tables",
                    ),
                    array: None,
                    byte_offset: 0x1ec,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffbase23",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffbase24",
                    description: Some(
                        "JPEG HuffSymb tables",
                    ),
                    array: None,
                    byte_offset: 0x1f0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffbase24",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffbase25",
                    description: Some(
                        "JPEG HuffSymb tables",
                    ),
                    array: None,
                    byte_offset: 0x1f4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffbase25",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffbase26",
                    description: Some(
                        "JPEG HuffSymb tables",
                    ),
                    array: None,
                    byte_offset: 0x1f8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffbase26",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffbase27",
                    description: Some(
                        "JPEG HuffSymb tables",
                    ),
                    array: None,
                    byte_offset: 0x1fc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffbase27",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffbase28",
                    description: Some(
                        "JPEG HuffSymb tables",
                    ),
                    array: None,
                    byte_offset: 0x200,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffbase28",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffbase29",
                    description: Some(
                        "JPEG HuffSymb tables",
                    ),
                    array: None,
                    byte_offset: 0x204,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffbase29",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffbase30",
                    description: Some(
                        "JPEG HuffSymb tables",
                    ),
                    array: None,
                    byte_offset: 0x208,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffbase30",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffbase31",
                    description: Some(
                        "JPEG HuffSymb tables",
                    ),
                    array: None,
                    byte_offset: 0x20c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffbase31",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb0",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x210,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb1",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x214,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb2",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x218,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb3",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x21c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb4",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x220,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb5",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x224,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb6",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x228,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb6",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb7",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x22c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb7",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb8",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x230,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb8",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb9",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x234,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb9",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb10",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x238,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb10",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb11",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x23c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb11",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb12",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x240,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb12",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb13",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x244,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb13",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb14",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x248,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb14",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb15",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x24c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb15",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb16",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x250,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb16",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb17",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x254,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb17",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb18",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x258,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb18",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb19",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x25c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb19",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb20",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x260,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb20",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb21",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x264,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb21",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb22",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x268,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb22",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb23",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x26c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb23",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb24",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x270,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb24",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb25",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x274,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb25",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb26",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x278,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb26",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb27",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x27c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb27",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb28",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x280,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb28",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb29",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x284,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb29",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb30",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x288,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb30",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb31",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x28c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb31",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb32",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x290,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb32",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb33",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x294,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb33",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb34",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x298,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb34",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb35",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x29c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb35",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb36",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x2a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb36",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb37",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x2a4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb37",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb38",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x2a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb38",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb39",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x2ac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb39",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb40",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x2b0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb40",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb41",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x2b4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb41",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb42",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x2b8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb42",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb43",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x2bc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb43",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb44",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x2c0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb44",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb45",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x2c4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb45",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb46",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x2c8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb46",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb47",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x2cc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb47",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb48",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x2d0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb48",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb49",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x2d4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb49",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb50",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x2d8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb50",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb51",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x2dc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb51",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb52",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x2e0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb52",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb53",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x2e4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb53",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb54",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x2e8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb54",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb55",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x2ec,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb55",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb56",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x2f0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb56",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb57",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x2f4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb57",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb58",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x2f8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb58",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb59",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x2fc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb59",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb60",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x300,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb60",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb61",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x304,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb61",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb62",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x308,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb62",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb63",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x30c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb63",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb64",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x310,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb64",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb65",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x314,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb65",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb66",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x318,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb66",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb67",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x31c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb67",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb68",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x320,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb68",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb69",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x324,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb69",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb70",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x328,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb70",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb71",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x32c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb71",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb72",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x330,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb72",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb73",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x334,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb73",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb74",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x338,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb74",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb75",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x33c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb75",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb76",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x340,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb76",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb77",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x344,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb77",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb78",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x348,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb78",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb79",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x34c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb79",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb80",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x350,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb80",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb81",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x354,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb81",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb82",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x358,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb82",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffsymb83",
                    description: Some(
                        "JPEG HUFFSYMB tables",
                    ),
                    array: None,
                    byte_offset: 0x35c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Huffsymb83",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem0",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x360,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem2",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x364,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem3",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x368,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem4",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x36c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem5",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x370,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem6",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x374,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem6",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem7",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x378,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem7",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem8",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x37c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem8",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem9",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x380,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem9",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem10",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x384,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem10",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem11",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x388,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem11",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem12",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x38c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem12",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem13",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x390,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem13",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem14",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x394,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem14",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem15",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x398,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem15",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem16",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x39c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem16",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem17",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x3a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem17",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem18",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x3a4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem18",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem19",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x3a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem19",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem20",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x3ac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem20",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem21",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x3b0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem21",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem22",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x3b4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem22",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem23",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x3b8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem23",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem24",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x3bc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem24",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem25",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x3c0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem25",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem26",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x3c4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem26",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem27",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x3c8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem27",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem28",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x3cc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem28",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem29",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x3d0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem29",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem30",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x3d4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem30",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem31",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x3d8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem31",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem32",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x3dc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem32",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem33",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x3e0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem33",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem34",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x3e4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem34",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem35",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x3e8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem35",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem36",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x3ec,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem36",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem37",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x3f0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem37",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem38",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x3f4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem38",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem39",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x3f8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem39",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem40",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x3fc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem40",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem41",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x400,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem41",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem42",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x404,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem42",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem43",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x408,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem43",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem44",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x40c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem44",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem45",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x410,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem45",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem46",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x414,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem46",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem47",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x418,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem47",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem48",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x41c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem48",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem49",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x420,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem49",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem50",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x424,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem50",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem51",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x428,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem51",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem52",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x42c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem52",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem53",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x430,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem53",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem54",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x434,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem54",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem55",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x438,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem55",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem56",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x43c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem56",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem57",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x440,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem57",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem58",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x444,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem58",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem59",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x448,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem59",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem60",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x44c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem60",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem61",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x450,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem61",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem62",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x454,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem62",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem63",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x458,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem63",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem64",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x45c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem64",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem65",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x460,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem65",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem66",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x464,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem66",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem67",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x468,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem67",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem68",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x46c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem68",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem69",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x470,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem69",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem70",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x474,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem70",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem71",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x478,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem71",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem72",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x47c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem72",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem73",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x480,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem73",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem74",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x484,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem74",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem75",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x488,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem75",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem76",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x48c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem76",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem77",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x490,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem77",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem78",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x494,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem78",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem79",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x498,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem79",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem80",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x49c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem80",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem81",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x4a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem81",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem82",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x4a4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem82",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem83",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x4a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem83",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem84",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x4ac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem84",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem85",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x4b0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem85",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem86",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x4b4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem86",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem87",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x4b8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem87",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem88",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x4bc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem88",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem89",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x4c0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem89",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem90",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x4c4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem90",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem91",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x4c8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem91",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem92",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x4cc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem92",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem93",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x4d0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem93",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem94",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x4d4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem94",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem95",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x4d8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem95",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem96",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x4dc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem96",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem97",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x4e0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem97",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem98",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x4e4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem98",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem99",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x4e8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem99",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem100",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x4ec,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem100",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem101",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x4f0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem101",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem102",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x4f4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem102",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dhtmem103",
                    description: Some(
                        "JPEG DHTMem tables",
                    ),
                    array: None,
                    byte_offset: 0x4f8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dhtmem103",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_0",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x500,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc00",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_1",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x504,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc01",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_2",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x508,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc02",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_3",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x50c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc03",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_4",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x510,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc04",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_5",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x514,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc05",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_6",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x518,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc06",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_7",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x51c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc07",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_8",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x520,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc08",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_9",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x524,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc09",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_10",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x528,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc010",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_11",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x52c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc011",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_12",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x530,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc012",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_13",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x534,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc013",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_14",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x538,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc014",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_15",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x53c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc015",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_16",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x540,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc016",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_17",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x544,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc017",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_18",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x548,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc018",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_19",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x54c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc019",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_20",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x550,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc020",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_21",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x554,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc021",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_22",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x558,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc022",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_23",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x55c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc023",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_24",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x560,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc024",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_25",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x564,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc025",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_26",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x568,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc026",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_27",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x56c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc027",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_28",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x570,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc028",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_29",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x574,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc029",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_30",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x578,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc030",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_31",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x57c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc031",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_32",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x580,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc032",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_33",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x584,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc033",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_34",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x588,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc034",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_35",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x58c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc035",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_36",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x590,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc036",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_37",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x594,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc037",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_38",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x598,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc038",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_39",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x59c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc039",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_40",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x5a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc040",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_41",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x5a4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc041",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_42",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x5a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc042",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_43",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x5ac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc043",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_44",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x5b0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc044",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_45",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x5b4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc045",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_46",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x5b8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc046",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_47",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x5bc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc047",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_48",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x5c0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc048",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_49",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x5c4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc049",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_50",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x5c8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc050",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_51",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x5cc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc051",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_52",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x5d0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc052",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_53",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x5d4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc053",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_54",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x5d8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc054",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_55",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x5dc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc055",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_56",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x5e0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc056",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_57",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x5e4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc057",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_58",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x5e8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc058",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_59",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x5ec,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc059",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_60",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x5f0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc060",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_61",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x5f4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc061",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_62",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x5f8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc062",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_63",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x5fc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc063",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_64",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x600,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc064",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_65",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x604,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc065",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_66",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x608,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc066",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_67",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x60c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc067",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_68",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x610,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc068",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_69",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x614,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc069",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_70",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x618,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc070",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_71",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x61c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc071",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_72",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x620,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc072",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_73",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x624,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc073",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_74",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x628,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc074",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_75",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x62c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc075",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_76",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x630,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc076",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_77",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x634,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc077",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_78",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x638,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc078",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_79",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x63c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc079",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_80",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x640,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc080",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_81",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x644,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc081",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_82",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x648,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc082",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_83",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x64c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc083",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_84",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x650,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc084",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_85",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x654,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc085",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_86",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x658,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc086",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac0_87",
                    description: Some(
                        "JPEG encoder, AC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x65c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc087",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_0",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x660,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc10",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_1",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x664,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc11",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_2",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x668,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc12",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_3",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x66c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc13",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_4",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x670,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc14",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_5",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x674,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc15",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_6",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x678,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc16",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_7",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x67c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc17",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_8",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x680,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc18",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_9",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x684,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc19",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_10",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x688,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc110",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_11",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x68c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc111",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_12",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x690,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc112",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_13",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x694,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc113",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_14",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x698,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc114",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_15",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x69c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc115",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_16",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x6a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc116",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_17",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x6a4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc117",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_18",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x6a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc118",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_19",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x6ac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc119",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_20",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x6b0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc120",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_21",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x6b4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc121",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_22",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x6b8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc122",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_23",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x6bc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc123",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_24",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x6c0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc124",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_25",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x6c4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc125",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_26",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x6c8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc126",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_27",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x6cc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc127",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_28",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x6d0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc128",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_29",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x6d4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc129",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_30",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x6d8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc130",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_31",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x6dc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc131",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_32",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x6e0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc132",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_33",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x6e4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc133",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_34",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x6e8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc134",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_35",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x6ec,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc135",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_36",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x6f0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc136",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_37",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x6f4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc137",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_38",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x6f8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc138",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_39",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x6fc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc139",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_40",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x700,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc140",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_41",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x704,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc141",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_42",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x708,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc142",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_43",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x70c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc143",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_44",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x710,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc144",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_45",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x714,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc145",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_46",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x718,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc146",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_47",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x71c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc147",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_48",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x720,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc148",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_49",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x724,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc149",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_50",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x728,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc150",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_51",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x72c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc151",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_52",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x730,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc152",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_53",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x734,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc153",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_54",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x738,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc154",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_55",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x73c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc155",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_56",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x740,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc156",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_57",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x744,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc157",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_58",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x748,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc158",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_59",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x74c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc159",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_60",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x750,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc160",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_61",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x754,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc161",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_62",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x758,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc162",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_63",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x75c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc163",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_64",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x760,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc164",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_65",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x764,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc165",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_66",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x768,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc166",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_67",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x76c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc167",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_68",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x770,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc168",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_69",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x774,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc169",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_70",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x778,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc170",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_71",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x77c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc171",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_72",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x780,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc172",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_73",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x784,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc173",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_74",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x788,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc174",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_75",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x78c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc175",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_76",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x790,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc176",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_77",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x794,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc177",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_78",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x798,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc178",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_79",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x79c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc179",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_80",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x7a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc180",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_81",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x7a4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc181",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_82",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x7a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc182",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_83",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x7ac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc183",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_84",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x7b0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc184",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_85",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x7b4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc185",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_86",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x7b8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc186",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_ac1_87",
                    description: Some(
                        "JPEG encoder, AC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x7bc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencAc187",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_dc0_0",
                    description: Some(
                        "JPEG encoder, DC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x7c0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencDc00",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_dc0_1",
                    description: Some(
                        "JPEG encoder, DC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x7c4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencDc01",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_dc0_2",
                    description: Some(
                        "JPEG encoder, DC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x7c8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencDc02",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_dc0_3",
                    description: Some(
                        "JPEG encoder, DC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x7cc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencDc03",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_dc0_4",
                    description: Some(
                        "JPEG encoder, DC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x7d0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencDc04",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_dc0_5",
                    description: Some(
                        "JPEG encoder, DC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x7d4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencDc05",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_dc0_6",
                    description: Some(
                        "JPEG encoder, DC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x7d8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencDc06",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_dc0_7",
                    description: Some(
                        "JPEG encoder, DC Huffman table 0",
                    ),
                    array: None,
                    byte_offset: 0x7dc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencDc07",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_dc1_0",
                    description: Some(
                        "JPEG encoder, DC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x7e0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencDc10",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_dc1_1",
                    description: Some(
                        "JPEG encoder, DC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x7e4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencDc11",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_dc1_2",
                    description: Some(
                        "JPEG encoder, DC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x7e8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencDc12",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_dc1_3",
                    description: Some(
                        "JPEG encoder, DC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x7ec,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencDc13",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_dc1_4",
                    description: Some(
                        "JPEG encoder, DC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x7f0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencDc14",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_dc1_5",
                    description: Some(
                        "JPEG encoder, DC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x7f4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencDc15",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_dc1_6",
                    description: Some(
                        "JPEG encoder, DC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x7f8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencDc16",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huffenc_dc1_7",
                    description: Some(
                        "JPEG encoder, DC Huffman table 1",
                    ),
                    array: None,
                    byte_offset: 0x7fc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HuffencDc17",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Dhtmem0",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem10",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem100",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem101",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem102",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem103",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem11",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem12",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem13",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem14",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem15",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem16",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem17",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem18",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem19",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem2",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem20",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem21",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem22",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem23",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem24",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem25",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem26",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem27",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem28",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem29",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem3",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem30",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem31",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem32",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem33",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem34",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem35",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem36",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem37",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem38",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem39",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem4",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem40",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem41",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem42",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem43",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem44",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem45",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem46",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem47",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem48",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem49",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem5",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem50",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem51",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem52",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem53",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem54",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem55",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem56",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem57",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem58",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem59",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem6",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem60",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem61",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem62",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem63",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem64",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem65",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem66",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem67",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem68",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem69",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem7",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem70",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem71",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem72",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem73",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem74",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem75",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem76",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem77",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem78",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem79",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem8",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem80",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem81",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem82",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem83",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem84",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem85",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem86",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem87",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem88",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem89",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem9",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem90",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem91",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem92",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem93",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem94",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem95",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem96",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem97",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem98",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dhtmem99",
            extends: None,
            description: Some(
                "JPEG DHTMem tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffbase0",
            extends: None,
            description: Some(
                "JPEG HuffSymb tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_base_ram_0",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "huff_base_ram_1",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffbase1",
            extends: None,
            description: Some(
                "JPEG HuffSymb tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_base_ram_0",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "huff_base_ram_1",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffbase10",
            extends: None,
            description: Some(
                "JPEG HuffSymb tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_base_ram_0",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "huff_base_ram_1",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffbase11",
            extends: None,
            description: Some(
                "JPEG HuffSymb tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_base_ram_0",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "huff_base_ram_1",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffbase12",
            extends: None,
            description: Some(
                "JPEG HuffSymb tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_base_ram_0",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "huff_base_ram_1",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffbase13",
            extends: None,
            description: Some(
                "JPEG HuffSymb tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_base_ram_0",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "huff_base_ram_1",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffbase14",
            extends: None,
            description: Some(
                "JPEG HuffSymb tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_base_ram_0",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "huff_base_ram_1",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffbase15",
            extends: None,
            description: Some(
                "JPEG HuffSymb tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_base_ram_0",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "huff_base_ram_1",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffbase16",
            extends: None,
            description: Some(
                "JPEG HuffSymb tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_base_ram_0",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "huff_base_ram_1",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffbase17",
            extends: None,
            description: Some(
                "JPEG HuffSymb tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_base_ram_0",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "huff_base_ram_1",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffbase18",
            extends: None,
            description: Some(
                "JPEG HuffSymb tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_base_ram_0",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "huff_base_ram_1",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffbase19",
            extends: None,
            description: Some(
                "JPEG HuffSymb tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_base_ram_0",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "huff_base_ram_1",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffbase2",
            extends: None,
            description: Some(
                "JPEG HuffSymb tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_base_ram_0",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "huff_base_ram_1",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffbase20",
            extends: None,
            description: Some(
                "JPEG HuffSymb tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_base_ram_0",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "huff_base_ram_1",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffbase21",
            extends: None,
            description: Some(
                "JPEG HuffSymb tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_base_ram_0",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "huff_base_ram_1",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffbase22",
            extends: None,
            description: Some(
                "JPEG HuffSymb tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_base_ram_0",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "huff_base_ram_1",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffbase23",
            extends: None,
            description: Some(
                "JPEG HuffSymb tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_base_ram_0",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "huff_base_ram_1",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffbase24",
            extends: None,
            description: Some(
                "JPEG HuffSymb tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_base_ram_0",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "huff_base_ram_1",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffbase25",
            extends: None,
            description: Some(
                "JPEG HuffSymb tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_base_ram_0",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "huff_base_ram_1",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffbase26",
            extends: None,
            description: Some(
                "JPEG HuffSymb tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_base_ram_0",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "huff_base_ram_1",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffbase27",
            extends: None,
            description: Some(
                "JPEG HuffSymb tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_base_ram_0",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "huff_base_ram_1",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffbase28",
            extends: None,
            description: Some(
                "JPEG HuffSymb tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_base_ram_0",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "huff_base_ram_1",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffbase29",
            extends: None,
            description: Some(
                "JPEG HuffSymb tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_base_ram_0",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "huff_base_ram_1",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffbase3",
            extends: None,
            description: Some(
                "JPEG HuffSymb tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_base_ram_0",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "huff_base_ram_1",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffbase30",
            extends: None,
            description: Some(
                "JPEG HuffSymb tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_base_ram_0",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "huff_base_ram_1",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffbase31",
            extends: None,
            description: Some(
                "JPEG HuffSymb tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_base_ram_0",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "huff_base_ram_1",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffbase4",
            extends: None,
            description: Some(
                "JPEG HuffSymb tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_base_ram_0",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "huff_base_ram_1",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffbase5",
            extends: None,
            description: Some(
                "JPEG HuffSymb tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_base_ram_0",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "huff_base_ram_1",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffbase6",
            extends: None,
            description: Some(
                "JPEG HuffSymb tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_base_ram_0",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "huff_base_ram_1",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffbase7",
            extends: None,
            description: Some(
                "JPEG HuffSymb tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_base_ram_0",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "huff_base_ram_1",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffbase8",
            extends: None,
            description: Some(
                "JPEG HuffSymb tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_base_ram_0",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "huff_base_ram_1",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffbase9",
            extends: None,
            description: Some(
                "JPEG HuffSymb tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_base_ram_0",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "huff_base_ram_1",
                    description: Some(
                        "HuffBase RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc00",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc01",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc010",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc011",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc012",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc013",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc014",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc015",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc016",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc017",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc018",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc019",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc02",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc020",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc021",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc022",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc023",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc024",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc025",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc026",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc027",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc028",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc029",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc03",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc030",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc031",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc032",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc033",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc034",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc035",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc036",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc037",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc038",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc039",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc04",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc040",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc041",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc042",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc043",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc044",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc045",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc046",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc047",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc048",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc049",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc05",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc050",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc051",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc052",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc053",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc054",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc055",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc056",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc057",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc058",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc059",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc06",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc060",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc061",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc062",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc063",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc064",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc065",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc066",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc067",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc068",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc069",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc07",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc070",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc071",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc072",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc073",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc074",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc075",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc076",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc077",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc078",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc079",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc08",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc080",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc081",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc082",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc083",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc084",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc085",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc086",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc087",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc09",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc10",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc11",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc110",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc111",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc112",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc113",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc114",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc115",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc116",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc117",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc118",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc119",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc12",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc120",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc121",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc122",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc123",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc124",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc125",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc126",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc127",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc128",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc129",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc13",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc130",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc131",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc132",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc133",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc134",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc135",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc136",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc137",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc138",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc139",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc14",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc140",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc141",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc142",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc143",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc144",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc145",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc146",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc147",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc148",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc149",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc15",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc150",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc151",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc152",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc153",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc154",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc155",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc156",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc157",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc158",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc159",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc16",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc160",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc161",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc162",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc163",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc164",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc165",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc166",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc167",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc168",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc169",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc17",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc170",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc171",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc172",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc173",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc174",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc175",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc176",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc177",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc178",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc179",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc18",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc180",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc181",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc182",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc183",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc184",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc185",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc186",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc187",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencAc19",
            extends: None,
            description: Some(
                "JPEG encoder, AC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencDc00",
            extends: None,
            description: Some(
                "JPEG encoder, DC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencDc01",
            extends: None,
            description: Some(
                "JPEG encoder, DC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencDc02",
            extends: None,
            description: Some(
                "JPEG encoder, DC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencDc03",
            extends: None,
            description: Some(
                "JPEG encoder, DC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencDc04",
            extends: None,
            description: Some(
                "JPEG encoder, DC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencDc05",
            extends: None,
            description: Some(
                "JPEG encoder, DC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencDc06",
            extends: None,
            description: Some(
                "JPEG encoder, DC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencDc07",
            extends: None,
            description: Some(
                "JPEG encoder, DC Huffman table 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencDc10",
            extends: None,
            description: Some(
                "JPEG encoder, DC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencDc11",
            extends: None,
            description: Some(
                "JPEG encoder, DC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencDc12",
            extends: None,
            description: Some(
                "JPEG encoder, DC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencDc13",
            extends: None,
            description: Some(
                "JPEG encoder, DC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencDc14",
            extends: None,
            description: Some(
                "JPEG encoder, DC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencDc15",
            extends: None,
            description: Some(
                "JPEG encoder, DC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencDc16",
            extends: None,
            description: Some(
                "JPEG encoder, DC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HuffencDc17",
            extends: None,
            description: Some(
                "JPEG encoder, DC Huffman table 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dhtmem_ram",
                    description: Some(
                        "DHTMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffmin0",
            extends: None,
            description: Some(
                "JPEG HuffMin tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_min_ram",
                    description: Some(
                        "HuffMin RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffmin1",
            extends: None,
            description: Some(
                "JPEG HuffMin tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_min_ram",
                    description: Some(
                        "HuffMin RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffmin10",
            extends: None,
            description: Some(
                "JPEG HuffMin tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_min_ram",
                    description: Some(
                        "HuffMin RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffmin11",
            extends: None,
            description: Some(
                "JPEG HuffMin tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_min_ram",
                    description: Some(
                        "HuffMin RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffmin12",
            extends: None,
            description: Some(
                "JPEG HuffMin tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_min_ram",
                    description: Some(
                        "HuffMin RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffmin13",
            extends: None,
            description: Some(
                "JPEG HuffMin tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_min_ram",
                    description: Some(
                        "HuffMin RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffmin14",
            extends: None,
            description: Some(
                "JPEG HuffMin tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_min_ram",
                    description: Some(
                        "HuffMin RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffmin15",
            extends: None,
            description: Some(
                "JPEG HuffMin tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_min_ram",
                    description: Some(
                        "HuffMin RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffmin2",
            extends: None,
            description: Some(
                "JPEG HuffMin tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_min_ram",
                    description: Some(
                        "HuffMin RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffmin3",
            extends: None,
            description: Some(
                "JPEG HuffMin tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_min_ram",
                    description: Some(
                        "HuffMin RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffmin4",
            extends: None,
            description: Some(
                "JPEG HuffMin tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_min_ram",
                    description: Some(
                        "HuffMin RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffmin5",
            extends: None,
            description: Some(
                "JPEG HuffMin tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_min_ram",
                    description: Some(
                        "HuffMin RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffmin6",
            extends: None,
            description: Some(
                "JPEG HuffMin tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_min_ram",
                    description: Some(
                        "HuffMin RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffmin7",
            extends: None,
            description: Some(
                "JPEG HuffMin tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_min_ram",
                    description: Some(
                        "HuffMin RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffmin8",
            extends: None,
            description: Some(
                "JPEG HuffMin tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_min_ram",
                    description: Some(
                        "HuffMin RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffmin9",
            extends: None,
            description: Some(
                "JPEG HuffMin tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_min_ram",
                    description: Some(
                        "HuffMin RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb0",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb1",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb10",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb11",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb12",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb13",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb14",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb15",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb16",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb17",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb18",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb19",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb2",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb20",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb21",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb22",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb23",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb24",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb25",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb26",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb27",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb28",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb29",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb3",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb30",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb31",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb32",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb33",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb34",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb35",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb36",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb37",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb38",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb39",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb4",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb40",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb41",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb42",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb43",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb44",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb45",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb46",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb47",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb48",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb49",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb5",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb50",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb51",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb52",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb53",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb54",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb55",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb56",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb57",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb58",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb59",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb6",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb60",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb61",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb62",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb63",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb64",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb65",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb66",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb67",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb68",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb69",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb7",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb70",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb71",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb72",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb73",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb74",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb75",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb76",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb77",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb78",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb79",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb8",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb80",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb81",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb82",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb83",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Huffsymb9",
            extends: None,
            description: Some(
                "JPEG HUFFSYMB tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "huff_symb_ram",
                    description: Some(
                        "DHTSymb RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "JpegCfr",
            extends: None,
            description: Some(
                "JPEG clear flag register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ceocf",
                    description: Some(
                        "Clear End of Conversion Flag",
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
                    name: "chpdf",
                    description: Some(
                        "Clear Header Parsing Done Flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "JpegConfr0",
            extends: None,
            description: Some(
                "JPEG codec configuration register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "start",
                    description: Some(
                        "Start",
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
            ],
        },
        FieldSet {
            name: "JpegConfr1",
            extends: None,
            description: Some(
                "JPEG codec configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nf",
                    description: Some(
                        "Number of color components",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "de",
                    description: Some(
                        "Decoding Enable",
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
                    name: "colorspace",
                    description: Some(
                        "Color Space",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ns",
                    description: Some(
                        "Number of components for Scan",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hdr",
                    description: Some(
                        "Header Processing",
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
                    name: "ysize",
                    description: Some(
                        "Y Size",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "JpegConfr2",
            extends: None,
            description: Some(
                "JPEG codec configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nmcu",
                    description: Some(
                        "Number of MCU",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 26,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "JpegConfr3",
            extends: None,
            description: Some(
                "JPEG codec configuration register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "xsize",
                    description: Some(
                        "X size",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "JpegConfr4",
            extends: None,
            description: Some(
                "JPEG codec configuration register 4",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hd",
                    description: Some(
                        "Huffman DC",
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
                    name: "ha",
                    description: Some(
                        "Huffman AC",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "qt",
                    description: Some(
                        "Quantization Table",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nb",
                    description: Some(
                        "Number of Block",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vsf",
                    description: Some(
                        "Vertical Sampling Factor",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsf",
                    description: Some(
                        "Horizontal Sampling Factor",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "JpegConfr5",
            extends: None,
            description: Some(
                "JPEG codec configuration register 5",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hd",
                    description: Some(
                        "Huffman DC",
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
                    name: "ha",
                    description: Some(
                        "Huffman AC",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "qt",
                    description: Some(
                        "Quantization Table",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nb",
                    description: Some(
                        "Number of Block",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vsf",
                    description: Some(
                        "Vertical Sampling Factor",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsf",
                    description: Some(
                        "Horizontal Sampling Factor",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "JpegConfr6",
            extends: None,
            description: Some(
                "JPEG codec configuration register 6",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hd",
                    description: Some(
                        "Huffman DC",
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
                    name: "ha",
                    description: Some(
                        "Huffman AC",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "qt",
                    description: Some(
                        "Quantization Table",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nb",
                    description: Some(
                        "Number of Block",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vsf",
                    description: Some(
                        "Vertical Sampling Factor",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsf",
                    description: Some(
                        "Horizontal Sampling Factor",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "JpegConfr7",
            extends: None,
            description: Some(
                "JPEG codec configuration register 7",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hd",
                    description: Some(
                        "Huffman DC",
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
                    name: "ha",
                    description: Some(
                        "Huffman AC",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "qt",
                    description: Some(
                        "Quantization Table",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nb",
                    description: Some(
                        "Number of Block",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vsf",
                    description: Some(
                        "Vertical Sampling Factor",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsf",
                    description: Some(
                        "Horizontal Sampling Factor",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "JpegCr",
            extends: None,
            description: Some(
                "JPEG control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "jcen",
                    description: Some(
                        "JPEG Core Enable",
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
                    name: "iftie",
                    description: Some(
                        "Input FIFO Threshold Interrupt Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ifnfie",
                    description: Some(
                        "Input FIFO Not Full Interrupt Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "oftie",
                    description: Some(
                        "Output FIFO Threshold Interrupt Enable",
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
                    name: "ofneie",
                    description: Some(
                        "Output FIFO Not Empty Interrupt Enable",
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
                    name: "eocie",
                    description: Some(
                        "End of Conversion Interrupt Enable",
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
                    name: "hpdie",
                    description: Some(
                        "Header Parsing Done Interrupt Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "idmaen",
                    description: Some(
                        "Input DMA Enable",
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
                Field {
                    name: "odmaen",
                    description: Some(
                        "Output DMA Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iff",
                    description: Some(
                        "Input FIFO Flush",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "off",
                    description: Some(
                        "Output FIFO Flush",
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
            name: "JpegDir",
            extends: None,
            description: Some(
                "JPEG data input register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "datain",
                    description: Some(
                        "Data Input FIFO",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "JpegDor",
            extends: None,
            description: Some(
                "JPEG data output register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dataout",
                    description: Some(
                        "Data Output FIFO",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "JpegSr",
            extends: None,
            description: Some(
                "JPEG status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iftf",
                    description: Some(
                        "Input FIFO Threshold Flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ifnff",
                    description: Some(
                        "Input FIFO Not Full Flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "oftf",
                    description: Some(
                        "Output FIFO Threshold Flag",
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
                    name: "ofnef",
                    description: Some(
                        "Output FIFO Not Empty Flag",
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
                    name: "eocf",
                    description: Some(
                        "End of Conversion Flag",
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
                    name: "hpdf",
                    description: Some(
                        "Header Parsing Done Flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cof",
                    description: Some(
                        "Codec Operation Flag",
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
            ],
        },
        FieldSet {
            name: "Qmem00",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem01",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem010",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem011",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem012",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem013",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem014",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem015",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem02",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem03",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem04",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem05",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem06",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem07",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem08",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem09",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem10",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem11",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem110",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem111",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem112",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem113",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem114",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem115",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem12",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem13",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem14",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem15",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem16",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem17",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem18",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem19",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem20",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem21",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem210",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem211",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem212",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem213",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem214",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem215",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem22",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem23",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem24",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem25",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem26",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem27",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem28",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem29",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem30",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem31",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem310",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem311",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem312",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem313",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem314",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem315",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem32",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem33",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem34",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem35",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem36",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem37",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem38",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Qmem39",
            extends: None,
            description: Some(
                "JPEG quantization tables",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "qmem_ram",
                    description: Some(
                        "QMem RAM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
                