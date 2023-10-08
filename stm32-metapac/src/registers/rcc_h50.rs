
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Rcc",
            extends: None,
            description: Some(
                "Reset and clock controller",
            ),
            items: &[
                BlockItem {
                    name: "cr",
                    description: Some(
                        "RCC clock control register",
                    ),
                    array: None,
                    byte_offset: 0,
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
                    name: "hsicfgr",
                    description: Some(
                        "RCC HSI calibration register",
                    ),
                    array: None,
                    byte_offset: 16,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hsicfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "crrcr",
                    description: Some(
                        "RCC clock recovery RC register",
                    ),
                    array: None,
                    byte_offset: 20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Crrcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "csicfgr",
                    description: Some(
                        "RCC CSI calibration register",
                    ),
                    array: None,
                    byte_offset: 24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Csicfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr",
                    description: Some(
                        "RCC clock configuration register",
                    ),
                    array: None,
                    byte_offset: 28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr2",
                    description: Some(
                        "RCC CPU domain clock configuration register 2",
                    ),
                    array: None,
                    byte_offset: 32,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pllcfgr",
                    description: Some(
                        "RCC PLL clock source selection register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pllcfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "plldivr",
                    description: Some(
                        "RCC PLL1 dividers register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 52,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Plldivr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pllfracr",
                    description: Some(
                        "RCC PLL1 fractional divider register",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    byte_offset: 56,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pllfracr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cier",
                    description: Some(
                        "RCC clock source interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cier",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cifr",
                    description: Some(
                        "RCC clock source interrupt flag register",
                    ),
                    array: None,
                    byte_offset: 84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cifr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cicr",
                    description: Some(
                        "RCC clock source interrupt clear register",
                    ),
                    array: None,
                    byte_offset: 88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cicr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb1rstr",
                    description: Some(
                        "RCC AHB1 reset register",
                    ),
                    array: None,
                    byte_offset: 96,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb1rstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb2rstr",
                    description: Some(
                        "RCC AHB2 peripheral reset register",
                    ),
                    array: None,
                    byte_offset: 100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb2rstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1lrstr",
                    description: Some(
                        "RCC APB1 peripheral low reset register",
                    ),
                    array: None,
                    byte_offset: 116,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1lrstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1hrstr",
                    description: Some(
                        "RCC APB1 peripheral high reset register",
                    ),
                    array: None,
                    byte_offset: 120,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1hrstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2rstr",
                    description: Some(
                        "RCC APB2 peripheral reset register",
                    ),
                    array: None,
                    byte_offset: 124,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2rstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb3rstr",
                    description: Some(
                        "RCC APB3 peripheral reset register",
                    ),
                    array: None,
                    byte_offset: 128,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb3rstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb1enr",
                    description: Some(
                        "RCC AHB1 peripherals clock register",
                    ),
                    array: None,
                    byte_offset: 136,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb1enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb2enr",
                    description: Some(
                        "RCC AHB2 peripheral clock register",
                    ),
                    array: None,
                    byte_offset: 140,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb2enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1lenr",
                    description: Some(
                        "RCC APB1 peripheral clock register",
                    ),
                    array: None,
                    byte_offset: 156,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1lenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1henr",
                    description: Some(
                        "RCC APB1 peripheral clock register",
                    ),
                    array: None,
                    byte_offset: 160,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1henr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2enr",
                    description: Some(
                        "RCC APB2 peripheral clock register",
                    ),
                    array: None,
                    byte_offset: 164,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb3enr",
                    description: Some(
                        "RCC APB3 peripheral clock register",
                    ),
                    array: None,
                    byte_offset: 168,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb3enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb1lpenr",
                    description: Some(
                        "RCC AHB1 sleep clock register",
                    ),
                    array: None,
                    byte_offset: 176,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb1lpenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb2lpenr",
                    description: Some(
                        "RCC AHB2 sleep clock register",
                    ),
                    array: None,
                    byte_offset: 180,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb2lpenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1llpenr",
                    description: Some(
                        "RCC APB1 sleep clock register",
                    ),
                    array: None,
                    byte_offset: 196,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1llpenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1hlpenr",
                    description: Some(
                        "RCC APB1 sleep clock register",
                    ),
                    array: None,
                    byte_offset: 200,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1hlpenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2lpenr",
                    description: Some(
                        "RCC APB2 sleep clock register",
                    ),
                    array: None,
                    byte_offset: 204,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2lpenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb3lpenr",
                    description: Some(
                        "RCC APB3 sleep clock register",
                    ),
                    array: None,
                    byte_offset: 208,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb3lpenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccipr1",
                    description: Some(
                        "RCC kernel clock configuration register",
                    ),
                    array: None,
                    byte_offset: 216,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccipr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccipr2",
                    description: Some(
                        "RCC kernel clock configuration register",
                    ),
                    array: None,
                    byte_offset: 220,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccipr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccipr3",
                    description: Some(
                        "RCC kernel clock configuration register",
                    ),
                    array: None,
                    byte_offset: 224,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccipr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccipr4",
                    description: Some(
                        "RCC kernel clock configuration register",
                    ),
                    array: None,
                    byte_offset: 228,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccipr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccipr5",
                    description: Some(
                        "RCC kernel clock configuration register",
                    ),
                    array: None,
                    byte_offset: 232,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccipr5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bdcr",
                    description: Some(
                        "RCC Backup domain control register",
                    ),
                    array: None,
                    byte_offset: 240,
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
                    name: "rsr",
                    description: Some(
                        "RCC reset status register",
                    ),
                    array: None,
                    byte_offset: 244,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rsr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Crrcr",
            extends: None,
            description: Some(
                "RCC clock recovery RC register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsi48cal",
                    description: Some(
                        "Internal RC 48 MHz clock calibration\r Set by hardware by option-byte loading during system reset NRESET. Read-only.",
                    ),
                    bit_offset: 0,
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pllcfgr",
            extends: None,
            description: Some(
                "RCC PLL clock source selection register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pllsrc",
                    description: Some(
                        "DIVMx and PLLs clock source selection\r Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled.\r In order to save power, when no PLL is used, the value of PLL1SRC must be set to '00'. 00: no clock send to DIVMx divider and PLLs (default after reset).",
                    ),
                    bit_offset: 0,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pllsrc",
                    ),
                },
                Field {
                    name: "pllrge",
                    description: Some(
                        "PLL1 input frequency range\r Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1.",
                    ),
                    bit_offset: 2,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pllrge",
                    ),
                },
                Field {
                    name: "pllfracen",
                    description: Some(
                        "PLL1 fractional latch enable\r Set and reset by software to latch the content of FRACN1 into the sigma-delta modulator.\r In order to latch the FRACN1 value into the sigma-delta modulator, PLL1FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN1 into the modulator.",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllvcosel",
                    description: Some(
                        "PLL1 VCO selection\r Set and reset by software to select the proper VCO frequency range used for PLL1. This bit must be written before enabling the PLL1.",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pllvcosel",
                    ),
                },
                Field {
                    name: "divm",
                    description: Some(
                        "prescaler for PLL1\r Set and cleared by software to configure the prescaler of the PLL1.\r The hardware does not allow any modification of this prescaler when PLL1 is enabled (PLL1ON = 1 or PLL1RDY = 1).\r In order to save power when PLL1 is not used, the value of DIVM1 must be set to 0.\r ...\r ...",
                    ),
                    bit_offset: 8,
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllpen",
                    description: Some(
                        "PLL1 DIVP divider output enable\r Set and reset by software to enable the pll1_p_ck output of the PLL1.\r This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).\r In order to save power, when the pll1_p_ck output of the PLL1 is not used, the pll1_p_ck must be disabled.",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllqen",
                    description: Some(
                        "PLL1 DIVQ divider output enable\r Set and reset by software to enable the pll1_q_ck output of the PLL1.\r In order to save power, when the pll1_q_ck output of the PLL1 is not used, the pll1_q_ck must be disabled.\r This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllren",
                    description: Some(
                        "PLL1 DIVR divider output enable\r Set and reset by software to enable the pll1_r_ck output of the PLL1.\r To save power, DIVR1EN and DIVR1 bits must be set to 0 when the pll1_r_ck is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ccipr1",
            extends: None,
            description: Some(
                "RCC kernel clock configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usart1sel",
                    description: Some(
                        "USART1 kernel clock source selection\r Set and reset by software.\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: 0,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Usartsel",
                    ),
                },
                Field {
                    name: "usart2sel",
                    description: Some(
                        "USART2 kernel clock source selection\r Set and reset by software.\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: 3,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Usartsel",
                    ),
                },
                Field {
                    name: "usart3sel",
                    description: Some(
                        "USART3 kernel clock source selection\r Set and reset by software.\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: 6,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Usartsel",
                    ),
                },
                Field {
                    name: "timicsel",
                    description: Some(
                        "TIM2, TIM3 and LPTIM2 input capture source selection\r Set and reset by software.",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Timicsel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Apb3rstr",
            extends: None,
            description: Some(
                "RCC APB3 peripheral reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sbsrst",
                    description: Some(
                        "SBS block reset\r Set and reset by software.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpuart1rst",
                    description: Some(
                        "LPUART1 block reset\r Set and reset by software.",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i3c2rst",
                    description: Some(
                        "I3C2RST block reset\r Set and reset by software.",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim1rst",
                    description: Some(
                        "LPTIM1 block reset\r Set and reset by software.",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vrefrst",
                    description: Some(
                        "VREF block reset\r Set and reset by software.",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Csicfgr",
            extends: None,
            description: Some(
                "RCC CSI calibration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "csical",
                    description: Some(
                        "CSI clock calibration\r Set by hardware by option byte loading during system reset NRESET. Adjusted by software through trimming bits CSITRIM.\r This field represents the sum of engineering option byte calibration value and CSITRIM bits value.",
                    ),
                    bit_offset: 0,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "csitrim",
                    description: Some(
                        "CSI clock trimming\r Set by software to adjust calibration.\r CSITRIM field is added to the engineering option bytes loaded during reset phase (FLASH_CSI_OPT) in order to form the calibration trimming value.\r CSICAL = CSITRIM + FLASH_CSI_OPT.\r Note: The reset value of the field is 0x20.",
                    ),
                    bit_offset: 16,
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cifr",
            extends: None,
            description: Some(
                "RCC clock source interrupt flag register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsirdyf",
                    description: Some(
                        "LSI ready interrupt flag\r Reset by software by writing LSIRDYC bit.\r Set by hardware when the LSI clock becomes stable and LSIRDYIE is set.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lserdyf",
                    description: Some(
                        "LSE ready interrupt flag\r Reset by software by writing LSERDYC bit.\r Set by hardware when the LSE clock becomes stable and LSERDYIE is set.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "csirdyf",
                    description: Some(
                        "CSI ready interrupt flag\r Reset by software by writing CSIRDYC bit.\r Set by hardware when the CSI clock becomes stable and CSIRDYIE is set.",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsirdyf",
                    description: Some(
                        "HSI ready interrupt flag\r Reset by software by writing HSIRDYC bit.\r Set by hardware when the HSI clock becomes stable and HSIRDYIE is set.",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hserdyf",
                    description: Some(
                        "HSE ready interrupt flag\r Reset by software by writing HSERDYC bit.\r Set by hardware when the HSE clock becomes stable and HSERDYIE is set.",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsi48rdyf",
                    description: Some(
                        "HSI48 ready interrupt flag\r Reset by software by writing HSI48RDYC bit.\r Set by hardware when the HSI48 clock becomes stable and HSI48RDYIE is set.",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllrdyf",
                    description: Some(
                        "PLL1 ready interrupt flag\r Reset by software by writing PLL1RDYC bit.\r Set by hardware when the PLL1 locks and PLL1RDYIE is set.",
                    ),
                    bit_offset: 6,
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
                Field {
                    name: "hsecssf",
                    description: Some(
                        "HSE clock security system interrupt flag\r Reset by software by writing HSECSSC bit.\r Set by hardware in case of HSE clock failure.",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb1hrstr",
            extends: None,
            description: Some(
                "RCC APB1 peripheral high reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dtsrst",
                    description: Some(
                        "DTS block reset\r Set and reset by software.",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim2rst",
                    description: Some(
                        "LPTIM2 block reset\r Set and reset by software.",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fdcan1rst",
                    description: Some(
                        "FDCAN1 block reset\r Set and reset by software.",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb3enr",
            extends: None,
            description: Some(
                "RCC APB3 peripheral clock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sbsen",
                    description: Some(
                        "SBS clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpuart1en",
                    description: Some(
                        "LPUART1 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i3c2en",
                    description: Some(
                        "I3C2EN clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim1en",
                    description: Some(
                        "LPTIM1 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vrefen",
                    description: Some(
                        "VREF clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rtcapben",
                    description: Some(
                        "RTC APB interface clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ccipr5",
            extends: None,
            description: Some(
                "RCC kernel clock configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adcdacsel",
                    description: Some(
                        "ADC and DAC kernel clock source selection\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: 0,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Adcdacsel",
                    ),
                },
                Field {
                    name: "dacsel",
                    description: Some(
                        "DAC hold clock",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dacsel",
                    ),
                },
                Field {
                    name: "rngsel",
                    description: Some(
                        "RNG kernel clock source selection",
                    ),
                    bit_offset: 4,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Rngsel",
                    ),
                },
                Field {
                    name: "fdcan1sel",
                    description: Some(
                        "FDCAN1 kernel clock source selection",
                    ),
                    bit_offset: 8,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Fdcansel",
                    ),
                },
                Field {
                    name: "ckpersel",
                    description: Some(
                        "per_ck clock source selection",
                    ),
                    bit_offset: 30,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Ckpersel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Apb2rstr",
            extends: None,
            description: Some(
                "RCC APB2 peripheral reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim1rst",
                    description: Some(
                        "TIM1 block reset\r Set and reset by software.",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi1rst",
                    description: Some(
                        "SPI1 block reset\r Set and reset by software.",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart1rst",
                    description: Some(
                        "USART1 block reset\r Set and reset by software.",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usbrst",
                    description: Some(
                        "USB block reset\r Set and reset by software.",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb1llpenr",
            extends: None,
            description: Some(
                "RCC APB1 sleep clock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2lpen",
                    description: Some(
                        "TIM2 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim3lpen",
                    description: Some(
                        "TIM3 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim6lpen",
                    description: Some(
                        "TIM6 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim7lpen",
                    description: Some(
                        "TIM7 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wwdglpen",
                    description: Some(
                        "WWDG clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "opamplpen",
                    description: Some(
                        "OPAMP clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi2lpen",
                    description: Some(
                        "SPI2 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi3lpen",
                    description: Some(
                        "SPI3 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "complpen",
                    description: Some(
                        "COMP clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart2lpen",
                    description: Some(
                        "USART2 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart3lpen",
                    description: Some(
                        "USART3 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c1lpen",
                    description: Some(
                        "I2C1 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c2lpen",
                    description: Some(
                        "I2C2 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i3c1lpen",
                    description: Some(
                        "I3C1 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crslpen",
                    description: Some(
                        "CRS clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ccipr3",
            extends: None,
            description: Some(
                "RCC kernel clock configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spi1sel",
                    description: Some(
                        "SPI1 kernel clock source selection\r Set and reset by software.\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: 0,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Spisel",
                    ),
                },
                Field {
                    name: "spi2sel",
                    description: Some(
                        "SPI2 kernel clock source selection\r Set and reset by software.\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: 3,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Spisel",
                    ),
                },
                Field {
                    name: "spi3sel",
                    description: Some(
                        "SPI3 kernel clock source selection\r Set and reset by software.\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: 6,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Spisel",
                    ),
                },
                Field {
                    name: "lpuart1sel",
                    description: Some(
                        "LPUART1 kernel clock source selection\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: 24,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Lpuartsel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Ccipr2",
            extends: None,
            description: Some(
                "RCC kernel clock configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lptim1sel",
                    description: Some(
                        "LPTIM1 kernel clock source selection\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: 8,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Lptimsel",
                    ),
                },
                Field {
                    name: "lptim2sel",
                    description: Some(
                        "LPTIM2 kernel clock source selection\r others: reserved, the kernel clock is disabled",
                    ),
                    bit_offset: 12,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Lptimsel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Apb3lpenr",
            extends: None,
            description: Some(
                "RCC APB3 sleep clock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sbslpen",
                    description: Some(
                        "SBS clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpuart1lpen",
                    description: Some(
                        "LPUART1 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i3c2lpen",
                    description: Some(
                        "I3C2 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim1lpen",
                    description: Some(
                        "LPTIM1 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vreflpen",
                    description: Some(
                        "VREF clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rtcapblpen",
                    description: Some(
                        "RTC APB interface clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb1hlpenr",
            extends: None,
            description: Some(
                "RCC APB1 sleep clock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dtslpen",
                    description: Some(
                        "DTS clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim2lpen",
                    description: Some(
                        "LPTIM2 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fdcan1lpen",
                    description: Some(
                        "FDCAN1 peripheral clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rsr",
            extends: None,
            description: Some(
                "RCC reset status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rmvf",
                    description: Some(
                        "remove reset flag\r Set and reset by software to reset the value of the reset flags.",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pinrstf",
                    description: Some(
                        "pin reset flag (NRST)\r Reset by software by writing the RMVF bit.\r Set by hardware when a reset from pin occurs.",
                    ),
                    bit_offset: 26,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "borrstf",
                    description: Some(
                        "BOR reset flag\r Reset by software by writing the RMVF bit.\r Set by hardware when a BOR reset occurs (pwr_bor_rst).",
                    ),
                    bit_offset: 27,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sftrstf",
                    description: Some(
                        "system reset from CPU reset flag\r Reset by software by writing the RMVF bit.\r Set by hardware when the system reset is due to CPU.The CPU can generate a system reset by writing SYSRESETREQ bit of AIRCR register of the core M33.",
                    ),
                    bit_offset: 28,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iwdgrstf",
                    description: Some(
                        "independent watchdog reset flag\r Reset by software by writing the RMVF bit.\r Set by hardware when an independent watchdog reset occurs.",
                    ),
                    bit_offset: 29,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wwdgrstf",
                    description: Some(
                        "window watchdog reset flag\r Reset by software by writing the RMVF bit.\r Set by hardware when a window watchdog reset occurs.",
                    ),
                    bit_offset: 30,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpwrrstf",
                    description: Some(
                        "Low-power reset flag\r Set by hardware when a reset occurs due to Stop or Standby mode entry, whereas the corresponding nRST_STOP, nRST_STBY option bit is cleared.\r Cleared by writing to the RMVF bit.",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ccipr4",
            extends: None,
            description: Some(
                "RCC kernel clock configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "systicksel",
                    description: Some(
                        "SYSTICK clock source selection\r Note: rcc_hclk frequency must be four times higher than\r lsi_ker_ck/lse_ck (period (LSI/LSE) ≥ 4 * period (HCLK).",
                    ),
                    bit_offset: 2,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Systicksel",
                    ),
                },
                Field {
                    name: "usbsel",
                    description: Some(
                        "USB kernel clock source selection",
                    ),
                    bit_offset: 4,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Usbsel",
                    ),
                },
                Field {
                    name: "i2c1sel",
                    description: Some(
                        "I2C1 kernel clock source selection",
                    ),
                    bit_offset: 16,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Icsel",
                    ),
                },
                Field {
                    name: "i2c2sel",
                    description: Some(
                        "I2C2 kernel clock source selection",
                    ),
                    bit_offset: 18,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Icsel",
                    ),
                },
                Field {
                    name: "i3c1sel",
                    description: Some(
                        "I3C1 kernel clock source selection",
                    ),
                    bit_offset: 24,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Icsel",
                    ),
                },
                Field {
                    name: "i3c2sel",
                    description: Some(
                        "I3C2 kernel clock source selection",
                    ),
                    bit_offset: 26,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Icsel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "RCC clock control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsion",
                    description: Some(
                        "HSI clock enable\r Set and cleared by software.\r Set by hardware to force the HSI to ON when the product leaves Stop mode, if STOPWUCK = 1 or STOPKERWUCK = 1.\r Set by hardware to force the HSI to ON when the product leaves Standby mode or in case of a failure of the HSE which is used as the system clock source.\r This bit cannot be cleared if the HSI is used directly (via SW mux) as system clock, or if the HSI is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1).",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsirdy",
                    description: Some(
                        "HSI clock ready flag\r Set by hardware to indicate that the HSI oscillator is stable.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsikeron",
                    description: Some(
                        "HSI clock enable in Stop mode\r Set and reset by software to force the HSI to ON, even in Stop mode, in order to be quickly available as kernel clock for peripherals. This bit has no effect on the value of HSION.",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsidiv",
                    description: Some(
                        "HSI clock divider\r Set and reset by software.\r These bits allow selecting a division ratio in order to configure the wanted HSI clock frequency. The\r HSIDIV cannot be changed if the HSI is selected as reference clock for at least one enabled PLL (PLLxON bit set to 1). In that case, the new HSIDIV value is ignored.",
                    ),
                    bit_offset: 3,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Hsidiv",
                    ),
                },
                Field {
                    name: "hsidivf",
                    description: Some(
                        "HSI divider flag\r Set and reset by hardware.\r As a write operation to HSIDIV has not an immediate effect on the frequency, this flag indicates the\r current status of the HSI divider. HSIDIVF goes immediately to 0 when HSIDIV value is changed, and is set back to 1 when the output frequency matches the value programmed into HSIDIV.",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "csion",
                    description: Some(
                        "CSI clock enable\r Set and reset by software to enable/disable CSI clock for system and/or peripheral.\r Set by hardware to force the CSI to ON when the system leaves Stop mode, if STOPWUCK = 1 or STOPKERWUCK = 1.\r This bit cannot be cleared if the CSI is used directly (via SW mux) as system clock, or if the CSI is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1).",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "csirdy",
                    description: Some(
                        "CSI clock ready flag\r Set by hardware to indicate that the CSI oscillator is stable. This bit is activated only if the RC is enabled by CSION (it is not activated if the CSI is enabled by CSIKERON or by a peripheral request).",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "csikeron",
                    description: Some(
                        "CSI clock enable in Stop mode\r Set and reset by software to force the CSI to ON, even in Stop mode, in order to be quickly available as kernel clock for some peripherals. This bit has no effect on the value of CSION.",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsi48on",
                    description: Some(
                        "HSI48 clock enable\r Set by software and cleared by software or by the hardware when the system enters to Stop\r or Standby mode.",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsi48rdy",
                    description: Some(
                        "HSI48 clock ready flag\r Set by hardware to indicate that the HSI48 oscillator is stable.",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hseon",
                    description: Some(
                        "HSE clock enable\r Set and cleared by software.\r Cleared by hardware to stop the HSE when entering Stop or Standby mode.\r This bit cannot be cleared if the HSE is used directly (via SW mux) as system clock, or if the\r HSE is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1).",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hserdy",
                    description: Some(
                        "HSE clock ready flag\r Set by hardware to indicate that the HSE oscillator is stable.",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsebyp",
                    description: Some(
                        "HSE clock bypass\r Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit to be used by the device.\r The HSEBYP bit can be written only if the HSE oscillator is disabled.",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsecsson",
                    description: Some(
                        "HSE clock security system enable\r Set by software to enable clock security system on HSE.\r This bit is “set only” (disabled by a system reset or when the system enters in Standby mode). When HSECSSON is set, the clock detector is enabled by hardware when the HSE is ready and disabled by hardware if an oscillator failure is detected.",
                    ),
                    bit_offset: 19,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hseext",
                    description: Some(
                        "external high speed clock type in Bypass mode\r Set and reset by software to select the external clock type (analog or digital).\r The external clock must be enabled with the HSEON bit to be used by the device. The HSEEXT bit can be written only if the HSE oscillator is disabled.",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Hseext",
                    ),
                },
                Field {
                    name: "pllon",
                    description: Some(
                        "PLL1 enable\r Set and cleared by software to enable PLL1.\r Cleared by hardware when entering Stop or Standby mode. Note that the hardware prevents\r writing this bit to 0, if the PLL1 output is used as the system clock.",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 2,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "pllrdy",
                    description: Some(
                        "PLL1 clock ready flag\r Set by hardware to indicate that the PLL1 is locked.",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 2,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb2lpenr",
            extends: None,
            description: Some(
                "RCC APB2 sleep clock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim1lpen",
                    description: Some(
                        "TIM1 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi1lpen",
                    description: Some(
                        "SPI1 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart1lpen",
                    description: Some(
                        "USART1 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usblpen",
                    description: Some(
                        "USB clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Plldivr",
            extends: None,
            description: Some(
                "RCC PLL1 dividers register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "plln",
                    description: Some(
                        "Multiplication factor for PLL1VCO\r Set and reset by software to control the multiplication factor of the VCO.\r These bits can be written only when the PLL is disabled (PLL1ON = 0 and PLL1RDY = 0).\r ...\r ...\r Others: reserved",
                    ),
                    bit_offset: 0,
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllp",
                    description: Some(
                        "PLL1 DIVP division factor\r Set and reset by software to control the frequency of the pll1_p_ck clock.\r These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).\r Note that odd division factors are not allowed.\r ...",
                    ),
                    bit_offset: 9,
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllq",
                    description: Some(
                        "PLL1 DIVQ division factor\r Set and reset by software to control the frequency of the pll1_q_ck clock.\r These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).\r ...",
                    ),
                    bit_offset: 16,
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllr",
                    description: Some(
                        "PLL1 DIVR division factor\r Set and reset by software to control the frequency of the pll1_r_ck clock.\r These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).\r ...",
                    ),
                    bit_offset: 24,
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb1henr",
            extends: None,
            description: Some(
                "RCC APB1 peripheral clock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dtsen",
                    description: Some(
                        "DTS clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim2en",
                    description: Some(
                        "LPTIM2 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fdcan1en",
                    description: Some(
                        "FDCAN1 peripheral clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Hsicfgr",
            extends: None,
            description: Some(
                "RCC HSI calibration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsical",
                    description: Some(
                        "HSI clock calibration\r Set by hardware by option byte loading during system reset nreset. Adjusted by software through trimming bits HSITRIM.\r This field represents the sum of engineering option byte calibration value and HSITRIM bits value.",
                    ),
                    bit_offset: 0,
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsitrim",
                    description: Some(
                        "HSI clock trimming\r Set by software to adjust calibration.\r HSITRIM field is added to the engineering option bytes loaded during reset phase (FLASH_HSI_OPT) in order to form the calibration trimming value.\r HSICAL = HSITRIM + FLASH_HSI_OPT.\r After a change of HSITRIM it takes one system clock cycle before the new HSITRIM value is updated\r Note: The reset value of the field is 0x40.",
                    ),
                    bit_offset: 16,
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb2enr",
            extends: None,
            description: Some(
                "RCC APB2 peripheral clock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim1en",
                    description: Some(
                        "TIM1 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi1en",
                    description: Some(
                        "SPI1 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart1en",
                    description: Some(
                        "USART1 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usben",
                    description: Some(
                        "USB clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb2lpenr",
            extends: None,
            description: Some(
                "RCC AHB2 sleep clock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpioalpen",
                    description: Some(
                        "GPIOA clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpioblpen",
                    description: Some(
                        "GPIOB clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpioclpen",
                    description: Some(
                        "GPIOC clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiodlpen",
                    description: Some(
                        "GPIOD clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiohlpen",
                    description: Some(
                        "GPIOH clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc1lpen",
                    description: Some(
                        "ADC1 peripherals clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dac12lpen",
                    description: Some(
                        "DAC clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hashlpen",
                    description: Some(
                        "HASH clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rnglpen",
                    description: Some(
                        "RNG clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram2lpen",
                    description: Some(
                        "SRAM2 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 30,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cier",
            extends: None,
            description: Some(
                "RCC clock source interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsirdyie",
                    description: Some(
                        "LSI ready interrupt enable\r Set and reset by software to enable/disable interrupt caused by the LSI oscillator stabilization.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lserdyie",
                    description: Some(
                        "LSE ready interrupt enable\r Set and reset by software to enable/disable interrupt caused by the LSE oscillator stabilization.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "csirdyie",
                    description: Some(
                        "CSI ready interrupt enable\r Set and reset by software to enable/disable interrupt caused by the CSI oscillator stabilization.",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsirdyie",
                    description: Some(
                        "HSI ready interrupt enable\r Set and reset by software to enable/disable interrupt caused by the HSI oscillator stabilization.",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hserdyie",
                    description: Some(
                        "HSE ready interrupt enable\r Set and reset by software to enable/disable interrupt caused by the HSE oscillator stabilization.",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsi48rdyie",
                    description: Some(
                        "HSI48 ready interrupt enable\r Set and reset by software to enable/disable interrupt caused by the HSI48 oscillator stabilization.",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllrdyie",
                    description: Some(
                        "PLL1 ready interrupt enable\r Set and reset by software to enable/disable interrupt caused by PLL1 lock.",
                    ),
                    bit_offset: 6,
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
        FieldSet {
            name: "Cfgr2",
            extends: None,
            description: Some(
                "RCC CPU domain clock configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hpre",
                    description: Some(
                        "AHB prescaler\r Set and reset by software to control the division factor of rcc_hclk. Changing\r this division ratio has an impact on the frequency of all bus matrix clocks\r 0xxx: rcc_hclk = sys_ck (default after reset)",
                    ),
                    bit_offset: 0,
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Hpre",
                    ),
                },
                Field {
                    name: "ppre1",
                    description: Some(
                        "APB low-speed prescaler (APB1)\r Set and reset by software to control the division factor of rcc_pclk1.\r The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk after PPRE write.\r 0xx: rcc_pclk1 = rcc_hclk1 (default after reset)",
                    ),
                    bit_offset: 4,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Ppre",
                    ),
                },
                Field {
                    name: "ppre2",
                    description: Some(
                        "APB high-speed prescaler (APB2)\r Set and reset by software to control APB high-speed clocks division factor.\r The clocks are divided with the new prescaler factor from 1 to 16 APB cycles after PPRE2 write.\r 0xx: rcc_pclk2 = rcc_hclk1",
                    ),
                    bit_offset: 8,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Ppre",
                    ),
                },
                Field {
                    name: "ppre3",
                    description: Some(
                        "APB low-speed prescaler (APB3)\r Set and reset by software to control APB low-speed clocks division factor.\r The clocks are divided with the new prescaler factor from 1 to 16 APB cycles after PPRE3 write.\r 0xx: rcc_pclk3 = rcc_hclk1",
                    ),
                    bit_offset: 12,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Ppre",
                    ),
                },
                Field {
                    name: "ahb1dis",
                    description: Some(
                        "AHB1 clock disable\r This bit can be set in order to further reduce power consumption, when none of the AHB1\r peripherals from RCC_AHB1ENR are used and when their clocks are disabled in\r RCC_AHB1ENR. When this bit is set, all the AHB1 peripherals clocks from\r RCC_AHB1ENR are off.\r enable control bits",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ahb2dis",
                    description: Some(
                        "AHB2 clock disable\r This bit can be set in order to further reduce power consumption, when none of the AHB2\r peripherals from RCC_AHB2ENR are used and when their clocks are disabled in\r RCC_AHB2ENR. When this bit is set, all the AHB2 peripherals clocks from\r RCC_AHB2ENR are off.\r enable control bits",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ahb4dis",
                    description: Some(
                        "AHB4 clock disable\r This bit can be set in order to further reduce power consumption, when none of the AHB4\r peripherals from RCC_AHB4ENR are used and when their clocks are disabled in\r RCC_AHB4ENR. When this bit is set, all the AHB4 peripherals clocks from\r RCC_AHB4ENR are off.\r enable control bits",
                    ),
                    bit_offset: 19,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "apb1dis",
                    description: Some(
                        "APB1 clock disable value\r This bit can be set in order to further reduce power consumption, when none of the APB1\r peripherals (except IWDG) are used and when their clocks are disabled in RCC_APB1ENR.\r When this bit is set, all the APB1 peripherals clocks are off, except for IWDG.\r control bits",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "apb2dis",
                    description: Some(
                        "APB2 clock disable value\r This bit can be set in order to further reduce power consumption, when none of the APB2\r peripherals are used and when their clocks are disabled in RCC_APB2ENR. When this bit is\r set, all the APB2 peripherals clocks are off.\r control bits",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "apb3dis",
                    description: Some(
                        "APB3 clock disable value.Set and cleared by software\r This bit can be set in order to further reduce power consumption, when none of the APB3\r peripherals are used and when their clocks are disabled in RCC_APB3ENR. When this bit is\r set, all the APB3 peripherals clocks are off.\r control bits",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb2rstr",
            extends: None,
            description: Some(
                "RCC AHB2 peripheral reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpioarst",
                    description: Some(
                        "GPIOA block reset\r Set and reset by software.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiobrst",
                    description: Some(
                        "GPIOB block reset\r Set and reset by software.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiocrst",
                    description: Some(
                        "GPIOC block reset\r Set and reset by software.",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiodrst",
                    description: Some(
                        "GPIOD block reset\r Set and reset by software.",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiohrst",
                    description: Some(
                        "GPIOH block reset\r Set and reset by software.",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc1rst",
                    description: Some(
                        "ADC1 block reset\r Set and reset by software.",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dac12rst",
                    description: Some(
                        "DAC block reset\r Set and reset by software.",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hashrst",
                    description: Some(
                        "HASH block reset\r Set and reset by software.",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rngrst",
                    description: Some(
                        "RNG block reset\r Set and reset by software.",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb2enr",
            extends: None,
            description: Some(
                "RCC AHB2 peripheral clock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpioaen",
                    description: Some(
                        "GPIOA clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpioben",
                    description: Some(
                        "GPIOB clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiocen",
                    description: Some(
                        "GPIOC clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpioden",
                    description: Some(
                        "GPIOD clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiohen",
                    description: Some(
                        "GPIOH clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc1en",
                    description: Some(
                        "ADC1 peripherals clock enabled\r Set and reset by software.",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dac12en",
                    description: Some(
                        "DAC clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hashen",
                    description: Some(
                        "HASH clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rngen",
                    description: Some(
                        "RNG clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram2en",
                    description: Some(
                        "SRAM2 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 30,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cicr",
            extends: None,
            description: Some(
                "RCC clock source interrupt clear register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsirdyc",
                    description: Some(
                        "LSI ready interrupt clear\r Set by software to clear LSIRDYF.\r Reset by hardware when clear done.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lserdyc",
                    description: Some(
                        "LSE ready interrupt clear\r Set by software to clear LSERDYF.\r Reset by hardware when clear done.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "csirdyc",
                    description: Some(
                        "HSI ready interrupt clear\r Set by software to clear CSIRDYF.\r Reset by hardware when clear done.",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsirdyc",
                    description: Some(
                        "HSI ready interrupt clear\r Set by software to clear HSIRDYF.\r Reset by hardware when clear done.",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hserdyc",
                    description: Some(
                        "HSE ready interrupt clear\r Set by software to clear HSERDYF.\r Reset by hardware when clear done.",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsi48rdyc",
                    description: Some(
                        "HSI48 ready interrupt clear\r Set by software to clear HSI48RDYF.\r Reset by hardware when clear done.",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllrdyc",
                    description: Some(
                        "PLL1 ready interrupt clear\r Set by software to clear PLL1RDYF.\r Reset by hardware when clear done.",
                    ),
                    bit_offset: 6,
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
                Field {
                    name: "hsecssc",
                    description: Some(
                        "HSE clock security system interrupt clear\r Set by software to clear HSECSSF.\r Reset by hardware when clear done.",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cfgr",
            extends: None,
            description: Some(
                "RCC clock configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sw",
                    description: Some(
                        "system clock and trace clock switch\r Set and reset by software to select system clock and trace clock sources (sys_ck).\r Set by hardware in order to:\r -\tforce the selection of the HSI or CSI (depending on STOPWUCK selection) when leaving a system Stop mode\r -\tforce the selection of the HSI in case of failure of the HSE when used directly or indirectly as system clock\r others: reserved",
                    ),
                    bit_offset: 0,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Sw",
                    ),
                },
                Field {
                    name: "sws",
                    description: Some(
                        "system clock switch status\r Set and reset by hardware to indicate which clock source is used as system clock. 000: HSI used as system clock (hsi_ck) (default after reset).\r others: reserved",
                    ),
                    bit_offset: 3,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Sw",
                    ),
                },
                Field {
                    name: "stopwuck",
                    description: Some(
                        "system clock selection after a wakeup from system Stop\r Set and reset by software to select the system wakeup clock from system Stop.\r The selected clock is also used as emergency clock for the clock security system (CSS) on HSE. 0: HSI selected as wakeup clock from system Stop (default after reset)\r STOPWUCK must not be modified when CSS is enabled (by HSECSSON bit) and the system clock is HSE (SWS = 10) or a switch on HSE is requested (SW =10).",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Stopwuck",
                    ),
                },
                Field {
                    name: "stopkerwuck",
                    description: Some(
                        "kernel clock selection after a wakeup from system Stop\r Set and reset by software to select the kernel wakeup clock from system Stop.",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Stopkerwuck",
                    ),
                },
                Field {
                    name: "rtcpre",
                    description: Some(
                        "HSE division factor for RTC clock\r Set and cleared by software to divide the HSE to generate a clock for RTC.\r Caution: The software must set these bits correctly to ensure that the clock supplied to the RTC is lower than 1 MHz. These bits must be configured if needed before selecting the RTC clock source.\r ...",
                    ),
                    bit_offset: 8,
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timpre",
                    description: Some(
                        "timers clocks prescaler selection\r This bit is set and reset by software to control the clock frequency of all the timers connected to APB1 and APB2 domains.",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Timpre",
                    ),
                },
                Field {
                    name: "mco1pre",
                    description: Some(
                        "MCO1 prescaler\r Set and cleared by software to configure the prescaler of the MCO1. Modification of this prescaler may generate glitches on MCO1. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs.\r ...",
                    ),
                    bit_offset: 18,
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Mcopre",
                    ),
                },
                Field {
                    name: "mco1sel",
                    description: Some(
                        "Microcontroller clock output 1\r Set and cleared by software. Clock source selection may generate glitches on MCO1.\r It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs.\r others: reserved",
                    ),
                    bit_offset: 22,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Mco1sel",
                    ),
                },
                Field {
                    name: "mco2pre",
                    description: Some(
                        "MCO2 prescaler\r Set and cleared by software to configure the prescaler of the MCO2. Modification of this prescaler may generate glitches on MCO2. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs.\r ...",
                    ),
                    bit_offset: 25,
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Mcopre",
                    ),
                },
                Field {
                    name: "mco2sel",
                    description: Some(
                        "microcontroller clock output 2\r Set and cleared by software. Clock source selection may generate glitches on MCO2.\r It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs.\r others: reserved",
                    ),
                    bit_offset: 29,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Mco2sel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Ahb1lpenr",
            extends: None,
            description: Some(
                "RCC AHB1 sleep clock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpdma1lpen",
                    description: Some(
                        "GPDMA1 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpdma2lpen",
                    description: Some(
                        "GPDMA2 clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flitflpen",
                    description: Some(
                        "Flash interface (FLITF) clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crclpen",
                    description: Some(
                        "CRC clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ramcfglpen",
                    description: Some(
                        "RAMCFG clock enable during sleep mode\r Set and reset by software.",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bkpramlpen",
                    description: Some(
                        "BKPRAM clock enable during sleep mode\r Set and reset by software",
                    ),
                    bit_offset: 28,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "icachelpen",
                    description: Some(
                        "ICACHE clock enable during sleep mode\r Set and reset by software",
                    ),
                    bit_offset: 29,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram1lpen",
                    description: Some(
                        "SRAM1 clock enable during sleep mode\r Set and reset by software",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb1lenr",
            extends: None,
            description: Some(
                "RCC APB1 peripheral clock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2en",
                    description: Some(
                        "TIM2 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim3en",
                    description: Some(
                        "TIM3 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim6en",
                    description: Some(
                        "TIM6 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim7en",
                    description: Some(
                        "TIM7 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wwdgen",
                    description: Some(
                        "WWDG clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "opampen",
                    description: Some(
                        "OPAMP clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi2en",
                    description: Some(
                        "SPI2 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi3en",
                    description: Some(
                        "SPI3 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "compen",
                    description: Some(
                        "COMP clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart2en",
                    description: Some(
                        "USART2 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart3en",
                    description: Some(
                        "USART3 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c1en",
                    description: Some(
                        "I2C1 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c2en",
                    description: Some(
                        "I2C2 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i3c1en",
                    description: Some(
                        "I3C1 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crsen",
                    description: Some(
                        "CRS clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pllfracr",
            extends: None,
            description: Some(
                "RCC PLL1 fractional divider register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pllfracn",
                    description: Some(
                        "fractional part of the multiplication factor for PLL1 VCO\r Set and reset by software to control the fractional part of the multiplication factor of the VCO. These bits can be written at any time, allowing dynamic fine-tuning of the PLL1 VCO.\r The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is:\r * 128 to 560 MHz if PLL1VCOSEL = 0\r * \t150 to 420 MHz if PLL1VCOSEL = 1\r VCO output frequency = Fref1_ck x (PLL1N + (PLL1FRACN / 213)), with\r * \tPLL1N between 8 and 420\r * \tPLL1FRACN can be between 0 and 213- 1\r * \tThe input frequency Fref1_ck must be between 1 and 16 MHz.\r To change the PLL1FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows:\r * \tSet the bit PLL1FRACEN to 0\r * \tWrite the new fractional value into PLL1FRACN\r * \tSet the bit PLL1FRACEN to 1",
                    ),
                    bit_offset: 3,
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bdcr",
            extends: None,
            description: Some(
                "RCC Backup domain control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lseon",
                    description: Some(
                        "LSE oscillator enabled\r Set and reset by software.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lserdy",
                    description: Some(
                        "LSE oscillator ready\r Set and reset by hardware to indicate when the LSE is stable.\r This bit needs 6 cycles of lse_ck clock to fall down after LSEON has been set to 0.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lsebyp",
                    description: Some(
                        "LSE oscillator bypass\r Set and reset by software to bypass oscillator in debug mode. This bit must not be written when the LSE is enabled (by LSEON) or ready (LSERDY = 1)",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lsedrv",
                    description: Some(
                        "LSE oscillator driving capability\r Set by software to select the driving capability of the LSE oscillator.\r These bit can be written only if LSE oscillator is disabled (LSEON = 0 and LSERDY = 0).",
                    ),
                    bit_offset: 3,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Lsedrv",
                    ),
                },
                Field {
                    name: "lsecsson",
                    description: Some(
                        "LSE clock security system enable\r Set by software to enable the clock security system on 32 kHz oscillator.\r LSECSSON must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware) and after RTCSEL is selected.\r Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD = 1). In that case the software must disable LSECSSON.",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lsecssd",
                    description: Some(
                        "LSE clock security system failure detection\r Set by hardware to indicate when a failure has been detected by the clock security system on the external 32 kHz oscillator.",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lseext",
                    description: Some(
                        "low-speed external clock type in bypass mode\r Set and reset by software to select the external clock type (analog or digital).\r The external clock must be enabled with the LSEON bit, to be used by the device.\r The LSEEXT bit can be written only if the LSE oscillator is disabled.",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Lseext",
                    ),
                },
                Field {
                    name: "rtcsel",
                    description: Some(
                        "RTC clock source selection\r Set by software to select the clock source for the RTC.\r These bits can be written only one time (except in case of failure detection on LSE).\r These bits must be written before LSECSSON is enabled.\r The VSWRST bit can be used to reset them, then it can be written one time again.\r If HSE is selected as RTC clock, this clock is lost when the system is in Stop mode or in case of a pin reset (NRST).",
                    ),
                    bit_offset: 8,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Rtcsel",
                    ),
                },
                Field {
                    name: "rtcen",
                    description: Some(
                        "RTC clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vswrst",
                    description: Some(
                        "VSwitch domain software reset\r Set and reset by software.",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lscoen",
                    description: Some(
                        "Low-speed clock output (LSCO) enable\r Set and cleared by software.",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lscosel",
                    description: Some(
                        "Low-speed clock output selection\r Set and cleared by software.",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Lscosel",
                    ),
                },
                Field {
                    name: "lsion",
                    description: Some(
                        "LSI oscillator enable\r Set and cleared by software.",
                    ),
                    bit_offset: 26,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lsirdy",
                    description: Some(
                        "LSI oscillator ready\r Set and cleared by hardware to indicate when the LSI oscillator is stable.\r After the LSION bit is cleared, LSIRDY goes low after three internal low-speed oscillator clock cycles.\r This bit is set when the LSI is used by IWDG or RTC, even if LSION = 0.",
                    ),
                    bit_offset: 27,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb1enr",
            extends: None,
            description: Some(
                "RCC AHB1 peripherals clock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpdma1en",
                    description: Some(
                        "GPDMA1 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpdma2en",
                    description: Some(
                        "GPDMA2 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flitfen",
                    description: Some(
                        "Flash interface clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crcen",
                    description: Some(
                        "CRC clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ramcfgen",
                    description: Some(
                        "RAMCFG clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bkpramen",
                    description: Some(
                        "BKPRAM clock enable\r Set and reset by software",
                    ),
                    bit_offset: 28,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram1en",
                    description: Some(
                        "SRAM1 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb1lrstr",
            extends: None,
            description: Some(
                "RCC APB1 peripheral low reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2rst",
                    description: Some(
                        "TIM2 block reset\r Set and reset by software.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim3rst",
                    description: Some(
                        "TIM3 block reset\r Set and reset by software.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim6rst",
                    description: Some(
                        "TIM6 block reset\r Set and reset by software.",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim7rst",
                    description: Some(
                        "TIM7 block reset\r Set and reset by software.",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "opamprst",
                    description: Some(
                        "OPAMP block reset\r Set and reset by software.",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi2rst",
                    description: Some(
                        "SPI2 block reset\r Set and reset by software.",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi3rst",
                    description: Some(
                        "SPI3 block reset\r Set and reset by software.",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "comprst",
                    description: Some(
                        "COMP block reset\r Set and reset by software.",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart2rst",
                    description: Some(
                        "USART2 block reset\r Set and reset by software.",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart3rst",
                    description: Some(
                        "USART3 block reset\r Set and reset by software.",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c1rst",
                    description: Some(
                        "I2C1 block reset\r Set and reset by software.",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c2rst",
                    description: Some(
                        "I2C2 block reset\r Set and reset by software.",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i3c1rst",
                    description: Some(
                        "I3C1 block reset\r Set and reset by software.",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crsrst",
                    description: Some(
                        "CRS block reset\r Set and reset by software.",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb1rstr",
            extends: None,
            description: Some(
                "RCC AHB1 reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpdma1rst",
                    description: Some(
                        "GPDMA1 block reset\r Set and reset by software.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpdma2rst",
                    description: Some(
                        "GPDMA2 block reset\r Set and reset by software.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crcrst",
                    description: Some(
                        "CRC block reset Set and reset by software.",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ramcfgrst",
                    description: Some(
                        "RAMCFG block reset\r Set and reset by software.",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Hseext",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ANALOG",
                    description: Some(
                        "HSE in analog mode (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIGITAL",
                    description: Some(
                        "HSE in digital mode",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Dacsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DAC_HOLD",
                    description: Some(
                        "dac_hold_ck selected as kernel clock (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DAC_HOLD_2",
                    description: Some(
                        "dac_hold_ck selected as kernel clock",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Lseext",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ANALOG",
                    description: Some(
                        "LSE in analog mode (default after Backup domain reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIGITAL",
                    description: Some(
                        "LSE in digital mode (do not use if RTC is active).",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Adcdacsel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "HCLK",
                    description: Some(
                        "rcc_hclk selected as kernel clock (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYSCLK",
                    description: Some(
                        "sys_ck selected as kernel clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL2_R",
                    description: Some(
                        "pll2_r_ck selected as kernel clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "hse_ck selected as kernel clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "HSI_KER",
                    description: Some(
                        "hsi_ker_ck selected as kernel clock",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "CSI_KER",
                    description: Some(
                        "csi_ker_ck selected as kernel clock",
                    ),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Ppre",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "rcc_pclk3 = rcc_hclk1 / 1",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "rcc_pclk3 = rcc_hclk1 / 2",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "rcc_pclk3 = rcc_hclk1 / 4",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "rcc_pclk3 = rcc_hclk1 / 8",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "rcc_pclk3 = rcc_hclk1 / 16",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Sw",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI selected as system clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CSI",
                    description: Some(
                        "CSI selected as system clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE selected as system clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "PLL1",
                    description: Some(
                        "PLL1 selected as system clock",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Stopwuck",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CSI",
                    description: Some(
                        "CSI selected as wakeup clock from system Stop",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Hpre",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "sys_ck not divided",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "sys_ck divided by 2",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "sys_ck divided by 4",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "sys_ck divided by 8",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "sys_ck divided by 16",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "DIV64",
                    description: Some(
                        "sys_ck divided by 64",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "DIV128",
                    description: Some(
                        "sys_ck divided by 128",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "DIV256",
                    description: Some(
                        "sys_ck divided by 256",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "DIV512",
                    description: Some(
                        "sys_ck divided by 512",
                    ),
                    value: 15,
                },
            ],
        },
        Enum {
            name: "Pllvcosel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "WIDEVCO",
                    description: Some(
                        "VCO frequency range 192 to 836 MHz",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MEDIUMVCO",
                    description: Some(
                        "VCO frequency range 150 to 420 MHz",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Stopkerwuck",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI selected as wakeup clock from system Stop (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CSI",
                    description: Some(
                        "CSI selected as wakeup clock from system Stop",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Usartsel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "RCC_PCLK2",
                    description: Some(
                        "rcc_pclk2 selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_Q",
                    description: Some(
                        "pll2_q selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI_KER",
                    description: Some(
                        "hsi_ker selected as peripheral clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "CSI_KER",
                    description: Some(
                        "csi_ker selected as peripheral clock",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE selected as peripheral clock",
                    ),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Lscosel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI clock selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE clock selected",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Fdcansel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "hse_ck selected as kernel clock (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL1_Q",
                    description: Some(
                        "pll1_q_ck selected as kernel clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL2_Q",
                    description: Some(
                        "pll2_q_ck selected as kernel clock",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Usbsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DISABLE",
                    description: Some(
                        "Disable the kernel clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL1_Q",
                    description: Some(
                        "pll1_q selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI48",
                    description: Some(
                        "HSI48 selected as peripheral clock",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Lptimsel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "RCC_PCLK3",
                    description: Some(
                        "rcc_pclk3 selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_P",
                    description: Some(
                        "pll2_p selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE selected as peripheral clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI selected as peripheral clock",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "PER",
                    description: Some(
                        "PER selected as peripheral clock",
                    ),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Mcopre",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "Divide by 1",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "Divide by 2",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV3",
                    description: Some(
                        "Divide by 3",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "Divide by 4",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "DIV5",
                    description: Some(
                        "Divide by 5",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DIV6",
                    description: Some(
                        "Divide by 6",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DIV7",
                    description: Some(
                        "Divide by 7",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "Divide by 8",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "DIV9",
                    description: Some(
                        "Divide by 9",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "DIV10",
                    description: Some(
                        "Divide by 10",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "DIV11",
                    description: Some(
                        "Divide by 11",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "DIV12",
                    description: Some(
                        "Divide by 12",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "DIV13",
                    description: Some(
                        "Divide by 13",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "DIV14",
                    description: Some(
                        "Divide by 14",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "DIV15",
                    description: Some(
                        "Divide by 15",
                    ),
                    value: 15,
                },
            ],
        },
        Enum {
            name: "Timpre",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DEFAULTX2",
                    description: Some(
                        "The timers kernel clock is equal to rcc_hclk1 if PPRE1 or PPRE2 corresponds to a division by 1 or 2, else it is equal to 2 x Frcc_pclk1 or 2 x Frcc_pclk2 (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DEFAULTX4",
                    description: Some(
                        "The timers kernel clock is equal to 2 x Frcc_pclk1 or 2 x Frcc_pclk2 if PPRE1 or PPRE2 corresponds to a division by 1, 2 or 4, else it is equal to 4 x Frcc_pclk1 or 4 x Frcc_pclk2",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Icsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "RCC_PCLK1",
                    description: Some(
                        "rcc_pclk1 selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL3_R",
                    description: Some(
                        "pll3_r selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI_KER",
                    description: Some(
                        "hsi_ker selected as peripheral clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "CSI_KER",
                    description: Some(
                        "csi_ker selected as peripheral clock",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Systicksel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "HCLK_DIV_8",
                    description: Some(
                        "rcc_hclk/8 selected as clock source (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSI_KER",
                    description: Some(
                        "lsi_ker_ck[1] selected as clock source",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "lse_ck[1] selected as clock source",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Lsedrv",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "LOW",
                    description: Some(
                        "Low driving capability",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MEDIUMLOW",
                    description: Some(
                        "Medium low driving capability",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "MEDIUMHIGH",
                    description: Some(
                        "Medium high driving capability",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HIGH",
                    description: Some(
                        "High driving capability",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Lpuartsel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "RCC_PCLK3",
                    description: Some(
                        "rcc_pclk3 selected as kernel clock (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_Q",
                    description: Some(
                        "pll2_q_ck selected as kernel clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI_KER",
                    description: Some(
                        "hsi_ker_ck selected as kernel clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "CSI_KER",
                    description: Some(
                        "csi_ker_ck selected as kernel clock",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "lse_ck selected as kernel clock",
                    ),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Rtcsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "NOCLOCK",
                    description: Some(
                        "no clock (default after Backup domain reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE selected as RTC clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI selected as RTC clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSE_DIV_RTCPRE",
                    description: Some(
                        "HSE divided by RTCPRE value selected as RTC clock",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Pllsrc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "NONE",
                    description: Some(
                        "no clock send to DIVMx divider and PLLs (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI selected as PLL clock (hsi_ck)",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "CSI",
                    description: Some(
                        "CSI selected as PLL clock (csi_ck)",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE selected as PLL clock (hse_ck)",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Pllrge",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "RANGE1",
                    description: Some(
                        "Frequency is between 1 and 2 MHz",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RANGE2",
                    description: Some(
                        "Frequency is between 2 and 4 MHz",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "RANGE4",
                    description: Some(
                        "Frequency is between 4 and 8 MHz",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "RANGE8",
                    description: Some(
                        "Frequency is between 8 and 16 MHz",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Mco2sel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "SYSCLK",
                    description: Some(
                        "System clock selected for micro-controller clock output",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_P",
                    description: Some(
                        "pll2_p selected for micro-controller clock output",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE selected for micro-controller clock output",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "PLL1_P",
                    description: Some(
                        "pll1_p selected for micro-controller clock output",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "CSI",
                    description: Some(
                        "CSI selected for micro-controller clock output",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI selected for micro-controller clock output",
                    ),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Rngsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "HSI48_KER",
                    description: Some(
                        "hsi48_ker_ck selected as kernel clock (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL1_Q",
                    description: Some(
                        "pll1_q_ck selected as kernel clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "lse_ck selected as kernel clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "LSI_KER",
                    description: Some(
                        "lsi_ker_ck selected as kernel clock",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Mco1sel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI selected for micro-controller clock output",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE selected for micro-controller clock output",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE selected for micro-controller clock output",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "PLL1_Q",
                    description: Some(
                        "pll1_q selected for micro-controller clock output",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "HSI48",
                    description: Some(
                        "HSI48 selected for micro-controller clock output",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "Timicsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "B_0X0",
                    description: Some(
                        "No internal clock available for timers input capture (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1",
                    description: Some(
                        "hsi_ker_ck/1024, hsi_ker_ck/8 and csi_ker_ck/128 selected for timers input capture",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Spisel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "RCC_PCLK4",
                    description: Some(
                        "rcc_pclk4 selected as peripheral clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_Q",
                    description: Some(
                        "pll2_q selected as peripheral clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI_KER",
                    description: Some(
                        "hsi_ker selected as peripheral clock",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "CSI_KER",
                    description: Some(
                        "csi_ker selected as peripheral clock",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "Hsidiv",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "No division",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "Division by 2",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "Division by 4",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "Division by 8",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Ckpersel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "hsi_ker_ck selected as kernel clock (default after reset)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CSI",
                    description: Some(
                        "csi_ker_ck selected as kernel clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "hse_ck selected as kernel clock",
                    ),
                    value: 2,
                },
            ],
        },
    ],
};
