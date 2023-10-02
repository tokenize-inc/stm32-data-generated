
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Usbram",
        extends: None,
        description: Some("USB Endpoint memory"),
        items: &[BlockItem {
            name: "mem",
            description: Some("USB Endpoint memory"),
            array: Some(Array::Regular(RegularArray { len: 512, stride: 4 })),
            byte_offset: 0,
            inner: BlockItemInner::Register(Register {
                access: Access::ReadWrite,
                bit_size: 32,
                fieldset: None,
            }),
        }],
    }],
    fieldsets: &[],
    enums: &[],
};
