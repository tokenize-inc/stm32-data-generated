
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Rcc",
            extends: None,
            description: Some(
                "Reset and clock control",
            ),
            items: &[
                BlockItem {
                    name: "cr",
                    description: Some(
                        "Clock control register",
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
                    name: "cfgr",
                    description: Some(
                        "Clock configuration register (RCC_CFGR)",
                    ),
                    array: None,
                    byte_offset: 0x4,
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
                    name: "cir",
                    description: Some(
                        "Clock interrupt register (RCC_CIR)",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cir",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2rstr",
                    description: Some(
                        "APB2 peripheral reset register (RCC_APB2RSTR)",
                    ),
                    array: None,
                    byte_offset: 0xc,
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
                    name: "apb1rstr",
                    description: Some(
                        "APB1 peripheral reset register (RCC_APB1RSTR)",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1rstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahbenr",
                    description: Some(
                        "AHB Peripheral Clock enable register (RCC_AHBENR)",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahbenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2enr",
                    description: Some(
                        "APB2 peripheral clock enable register (RCC_APB2ENR)",
                    ),
                    array: None,
                    byte_offset: 0x18,
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
                    name: "apb1enr",
                    description: Some(
                        "APB1 peripheral clock enable register (RCC_APB1ENR)",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bdcr",
                    description: Some(
                        "Backup domain control register (RCC_BDCR)",
                    ),
                    array: None,
                    byte_offset: 0x20,
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
                    name: "csr",
                    description: Some(
                        "Control/status register (RCC_CSR)",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Csr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ahbenr",
            extends: None,
            description: Some(
                "AHB Peripheral Clock enable register (RCC_AHBENR)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma1en",
                    description: Some(
                        "DMA1 clock enable",
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
                    name: "dma2en",
                    description: Some(
                        "DMA2 clock enable",
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
                    name: "sramen",
                    description: Some(
                        "SRAM interface clock enable",
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
                    name: "flashen",
                    description: Some(
                        "FLASH clock enable",
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
                    name: "crcen",
                    description: Some(
                        "CRC clock enable",
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
                    name: "fsmcen",
                    description: Some(
                        "FSMC clock enable",
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
                    name: "sdioen",
                    description: Some(
                        "SDIO clock enable",
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
            ],
        },
        FieldSet {
            name: "Apb1enr",
            extends: None,
            description: Some(
                "APB1 peripheral clock enable register (RCC_APB1ENR)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2en",
                    description: Some(
                        "Timer 2 clock enable",
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
                    name: "tim3en",
                    description: Some(
                        "Timer 3 clock enable",
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
                    name: "tim4en",
                    description: Some(
                        "Timer 4 clock enable",
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
                    name: "tim5en",
                    description: Some(
                        "Timer 5 clock enable",
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
                    name: "tim6en",
                    description: Some(
                        "Timer 6 clock enable",
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
                    name: "tim7en",
                    description: Some(
                        "Timer 7 clock enable",
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
                    name: "tim12en",
                    description: Some(
                        "Timer 12 clock enable",
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
                    name: "tim13en",
                    description: Some(
                        "Timer 13 clock enable",
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
                    name: "tim14en",
                    description: Some(
                        "Timer 14 clock enable",
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
                    name: "wwdgen",
                    description: Some(
                        "Window watchdog clock enable",
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
                    name: "spi2en",
                    description: Some(
                        "SPI 2 clock enable",
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
                Field {
                    name: "spi3en",
                    description: Some(
                        "SPI 3 clock enable",
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
                Field {
                    name: "usart2en",
                    description: Some(
                        "USART 2 clock enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart3en",
                    description: Some(
                        "USART 3 clock enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uart4en",
                    description: Some(
                        "UART 4 clock enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uart5en",
                    description: Some(
                        "UART 5 clock enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c1en",
                    description: Some(
                        "I2C 1 clock enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c2en",
                    description: Some(
                        "I2C 2 clock enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usben",
                    description: Some(
                        "USB clock enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "canen",
                    description: Some(
                        "CAN clock enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bkpen",
                    description: Some(
                        "Backup interface clock enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pwren",
                    description: Some(
                        "Power interface clock enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dacen",
                    description: Some(
                        "DAC interface clock enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb1rstr",
            extends: None,
            description: Some(
                "APB1 peripheral reset register (RCC_APB1RSTR)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2rst",
                    description: Some(
                        "Timer 2 reset",
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
                    name: "tim3rst",
                    description: Some(
                        "Timer 3 reset",
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
                    name: "tim4rst",
                    description: Some(
                        "Timer 4 reset",
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
                    name: "tim5rst",
                    description: Some(
                        "Timer 5 reset",
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
                    name: "tim6rst",
                    description: Some(
                        "Timer 6 reset",
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
                    name: "tim7rst",
                    description: Some(
                        "Timer 7 reset",
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
                    name: "tim12rst",
                    description: Some(
                        "Timer 12 reset",
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
                    name: "tim13rst",
                    description: Some(
                        "Timer 13 reset",
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
                    name: "tim14rst",
                    description: Some(
                        "Timer 14 reset",
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
                    name: "wwdgrst",
                    description: Some(
                        "Window watchdog reset",
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
                    name: "spi2rst",
                    description: Some(
                        "SPI2 reset",
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
                Field {
                    name: "spi3rst",
                    description: Some(
                        "SPI3 reset",
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
                Field {
                    name: "usart2rst",
                    description: Some(
                        "USART 2 reset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart3rst",
                    description: Some(
                        "USART 3 reset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uart4rst",
                    description: Some(
                        "USART 4 reset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uart5rst",
                    description: Some(
                        "USART 5 reset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c1rst",
                    description: Some(
                        "I2C1 reset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c2rst",
                    description: Some(
                        "I2C2 reset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usbrst",
                    description: Some(
                        "USB reset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "canrst",
                    description: Some(
                        "CAN reset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bkprst",
                    description: Some(
                        "Backup interface reset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pwrrst",
                    description: Some(
                        "Power interface reset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dacrst",
                    description: Some(
                        "DAC interface reset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb2enr",
            extends: None,
            description: Some(
                "APB2 peripheral clock enable register (RCC_APB2ENR)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "afioen",
                    description: Some(
                        "Alternate function I/O clock enable",
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
                    name: "gpioaen",
                    description: Some(
                        "I/O port A clock enable",
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
                    name: "gpioben",
                    description: Some(
                        "I/O port B clock enable",
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
                    name: "gpiocen",
                    description: Some(
                        "I/O port C clock enable",
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
                    name: "gpioden",
                    description: Some(
                        "I/O port D clock enable",
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
                    name: "gpioeen",
                    description: Some(
                        "I/O port E clock enable",
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
                    name: "gpiofen",
                    description: Some(
                        "I/O port F clock enable",
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
                    name: "gpiogen",
                    description: Some(
                        "I/O port G clock enable",
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
                    name: "adc1en",
                    description: Some(
                        "ADC 1 interface clock enable",
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
                    name: "adc2en",
                    description: Some(
                        "ADC 2 interface clock enable",
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
                    name: "tim1en",
                    description: Some(
                        "TIM1 Timer clock enable",
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
                    name: "spi1en",
                    description: Some(
                        "SPI 1 clock enable",
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
                    name: "tim8en",
                    description: Some(
                        "TIM8 Timer clock enable",
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
                    name: "usart1en",
                    description: Some(
                        "USART1 clock enable",
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
                Field {
                    name: "adc3en",
                    description: Some(
                        "ADC3 interface clock enable",
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
                Field {
                    name: "tim9en",
                    description: Some(
                        "TIM9 Timer clock enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim10en",
                    description: Some(
                        "TIM10 Timer clock enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim11en",
                    description: Some(
                        "TIM11 Timer clock enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb2rstr",
            extends: None,
            description: Some(
                "APB2 peripheral reset register (RCC_APB2RSTR)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "afiorst",
                    description: Some(
                        "Alternate function I/O reset",
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
                    name: "gpioarst",
                    description: Some(
                        "IO port A reset",
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
                    name: "gpiobrst",
                    description: Some(
                        "IO port B reset",
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
                    name: "gpiocrst",
                    description: Some(
                        "IO port C reset",
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
                    name: "gpiodrst",
                    description: Some(
                        "IO port D reset",
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
                    name: "gpioerst",
                    description: Some(
                        "IO port E reset",
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
                    name: "gpiofrst",
                    description: Some(
                        "IO port F reset",
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
                    name: "gpiogrst",
                    description: Some(
                        "IO port G reset",
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
                    name: "adc1rst",
                    description: Some(
                        "ADC 1 interface reset",
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
                    name: "adc2rst",
                    description: Some(
                        "ADC 2 interface reset",
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
                    name: "tim1rst",
                    description: Some(
                        "TIM1 timer reset",
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
                    name: "spi1rst",
                    description: Some(
                        "SPI 1 reset",
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
                    name: "tim8rst",
                    description: Some(
                        "TIM8 timer reset",
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
                    name: "usart1rst",
                    description: Some(
                        "USART1 reset",
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
                Field {
                    name: "adc3rst",
                    description: Some(
                        "ADC 3 interface reset",
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
                Field {
                    name: "tim9rst",
                    description: Some(
                        "TIM9 timer reset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim10rst",
                    description: Some(
                        "TIM10 timer reset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim11rst",
                    description: Some(
                        "TIM11 timer reset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bdcr",
            extends: None,
            description: Some(
                "Backup domain control register (RCC_BDCR)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lseon",
                    description: Some(
                        "External Low Speed oscillator enable",
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
                    name: "lserdy",
                    description: Some(
                        "External Low Speed oscillator ready",
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
                    name: "lsebyp",
                    description: Some(
                        "External Low Speed oscillator bypass",
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
                    name: "rtcsel",
                    description: Some(
                        "RTC clock source selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Rtcsel",
                    ),
                },
                Field {
                    name: "rtcen",
                    description: Some(
                        "RTC clock enable",
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
                Field {
                    name: "bdrst",
                    description: Some(
                        "Backup domain software reset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
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
                "Clock configuration register (RCC_CFGR)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sw",
                    description: Some(
                        "System clock Switch",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Sw",
                    ),
                },
                Field {
                    name: "sws",
                    description: Some(
                        "System Clock Switch Status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Sw",
                    ),
                },
                Field {
                    name: "hpre",
                    description: Some(
                        "AHB prescaler",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Hpre",
                    ),
                },
                Field {
                    name: "ppre1",
                    description: Some(
                        "APB Low speed prescaler (APB1)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Ppre",
                    ),
                },
                Field {
                    name: "ppre2",
                    description: Some(
                        "APB High speed prescaler (APB2)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Ppre",
                    ),
                },
                Field {
                    name: "adcpre",
                    description: Some(
                        "ADC prescaler",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Adcpre",
                    ),
                },
                Field {
                    name: "pllsrc",
                    description: Some(
                        "PLL entry clock source",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pllsrc",
                    ),
                },
                Field {
                    name: "pllxtpre",
                    description: Some(
                        "HSE divider for PLL entry",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pllxtpre",
                    ),
                },
                Field {
                    name: "pllmul",
                    description: Some(
                        "PLL Multiplication Factor",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Pllmul",
                    ),
                },
                Field {
                    name: "usbpre",
                    description: Some(
                        "USB prescaler",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Usbpre",
                    ),
                },
                Field {
                    name: "mcosel",
                    description: Some(
                        "Microcontroller clock output",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Mcosel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cir",
            extends: None,
            description: Some(
                "Clock interrupt register (RCC_CIR)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsirdyf",
                    description: Some(
                        "LSI Ready Interrupt flag",
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
                    name: "lserdyf",
                    description: Some(
                        "LSE Ready Interrupt flag",
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
                    name: "hsirdyf",
                    description: Some(
                        "HSI Ready Interrupt flag",
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
                    name: "hserdyf",
                    description: Some(
                        "HSE Ready Interrupt flag",
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
                    name: "pllrdyf",
                    description: Some(
                        "PLL Ready Interrupt flag",
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
                    name: "cssf",
                    description: Some(
                        "Clock Security System Interrupt flag",
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
                    name: "lsirdyie",
                    description: Some(
                        "LSI Ready Interrupt Enable",
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
                    name: "lserdyie",
                    description: Some(
                        "LSE Ready Interrupt Enable",
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
                    name: "hsirdyie",
                    description: Some(
                        "HSI Ready Interrupt Enable",
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
                    name: "hserdyie",
                    description: Some(
                        "HSE Ready Interrupt Enable",
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
                    name: "pllrdyie",
                    description: Some(
                        "PLL Ready Interrupt Enable",
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
                    name: "lsirdyc",
                    description: Some(
                        "LSI Ready Interrupt Clear",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lserdyc",
                    description: Some(
                        "LSE Ready Interrupt Clear",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsirdyc",
                    description: Some(
                        "HSI Ready Interrupt Clear",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hserdyc",
                    description: Some(
                        "HSE Ready Interrupt Clear",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllrdyc",
                    description: Some(
                        "PLL Ready Interrupt Clear",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cssc",
                    description: Some(
                        "Clock security system interrupt clear",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "Clock control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsion",
                    description: Some(
                        "Internal High Speed clock enable",
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
                    name: "hsirdy",
                    description: Some(
                        "Internal High Speed clock ready flag",
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
                    name: "hsitrim",
                    description: Some(
                        "Internal High Speed clock trimming",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsical",
                    description: Some(
                        "Internal High Speed clock Calibration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hseon",
                    description: Some(
                        "External High Speed clock enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hserdy",
                    description: Some(
                        "External High Speed clock ready flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsebyp",
                    description: Some(
                        "External High Speed clock Bypass",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "csson",
                    description: Some(
                        "Clock Security System enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllon",
                    description: Some(
                        "PLL enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllrdy",
                    description: Some(
                        "PLL clock ready flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Csr",
            extends: None,
            description: Some(
                "Control/status register (RCC_CSR)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsion",
                    description: Some(
                        "Internal low speed oscillator enable",
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
                    name: "lsirdy",
                    description: Some(
                        "Internal low speed oscillator ready",
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
                    name: "rmvf",
                    description: Some(
                        "Remove reset flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pinrstf",
                    description: Some(
                        "PIN reset flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "porrstf",
                    description: Some(
                        "POR/PDR reset flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sftrstf",
                    description: Some(
                        "Software reset flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iwdgrstf",
                    description: Some(
                        "Independent watchdog reset flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wwdgrstf",
                    description: Some(
                        "Window watchdog reset flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpwrrstf",
                    description: Some(
                        "Low-power reset flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
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
            name: "Adcpre",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "PCLK2 divided by 2",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "PCLK2 divided by 4",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV6",
                    description: Some(
                        "PCLK2 divided by 6",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "PCLK2 divided by 8",
                    ),
                    value: 3,
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
                        "SYSCLK not divided",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "SYSCLK divided by 2",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "SYSCLK divided by 4",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "SYSCLK divided by 8",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "SYSCLK divided by 16",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "DIV64",
                    description: Some(
                        "SYSCLK divided by 64",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "DIV128",
                    description: Some(
                        "SYSCLK divided by 128",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "DIV256",
                    description: Some(
                        "SYSCLK divided by 256",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "DIV512",
                    description: Some(
                        "SYSCLK divided by 512",
                    ),
                    value: 15,
                },
            ],
        },
        Enum {
            name: "Mcosel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "DISABLE",
                    description: Some(
                        "MCO output disabled, no clock on MCO",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYS",
                    description: Some(
                        "System clock selected",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI oscillator clock selected",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE oscillator clock selected",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "PLL",
                    description: Some(
                        "PLL clock divided by 2 selected",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Pllmul",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "MUL2",
                    description: Some(
                        "PLL input clock x2",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MUL3",
                    description: Some(
                        "PLL input clock x3",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "MUL4",
                    description: Some(
                        "PLL input clock x4",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "MUL5",
                    description: Some(
                        "PLL input clock x5",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "MUL6",
                    description: Some(
                        "PLL input clock x6",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "MUL7",
                    description: Some(
                        "PLL input clock x7",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "MUL8",
                    description: Some(
                        "PLL input clock x8",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "MUL9",
                    description: Some(
                        "PLL input clock x9",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "MUL10",
                    description: Some(
                        "PLL input clock x10",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "MUL11",
                    description: Some(
                        "PLL input clock x11",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "MUL12",
                    description: Some(
                        "PLL input clock x12",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "MUL13",
                    description: Some(
                        "PLL input clock x13",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "MUL14",
                    description: Some(
                        "PLL input clock x14",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "MUL15",
                    description: Some(
                        "PLL input clock x15",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "MUL16",
                    description: Some(
                        "PLL input clock x16",
                    ),
                    value: 14,
                },
            ],
        },
        Enum {
            name: "Pllsrc",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "HSI_DIV2",
                    description: Some(
                        "HSI divided by 2 selected as PLL input clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HSE_DIV_PREDIV",
                    description: Some(
                        "HSE divided by PREDIV selected as PLL input clock",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pllxtpre",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "HSE clock not divided",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "HSE clock divided by 2",
                    ),
                    value: 1,
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
                        "HCLK not divided",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "HCLK divided by 2",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "HCLK divided by 4",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "HCLK divided by 8",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "HCLK divided by 16",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Rtcsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DISABLE",
                    description: Some(
                        "No clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE oscillator clock used as RTC clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI oscillator clock used as RTC clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE oscillator clock divided by a prescaler used as RTC clock",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Sw",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI selected as system clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE selected as system clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL1_P",
                    description: Some(
                        "PLL selected as system clock",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Usbpre",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DIV1_5",
                    description: Some(
                        "PLL clock is divided by 1.5",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "PLL clock is not divided",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
                