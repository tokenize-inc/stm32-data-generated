include!("../metadata_0749.rs");
pub static METADATA: Metadata = Metadata {
    name: "STM32WB30CE",
    family: "STM32WB",
    line: "STM32WBx0 Value Line",
    memory: &[
        MemoryRegion {
            name: "BANK_1",
            kind: MemoryRegionKind::Flash,
            address: 134217728,
            size: 524288,
            settings: Some(FlashSettings {
                erase_size: 4096,
                write_size: 8,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "SRAM",
            kind: MemoryRegionKind::Ram,
            address: 536870912,
            size: 98304,
            settings: None,
        },
    ],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(4),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
};
