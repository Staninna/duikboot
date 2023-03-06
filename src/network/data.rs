use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Pod, Zeroable)]
pub struct UserInput {
    pub input: KeyInput,
}

bitflags::bitflags! {
    #[repr(C)]
    #[derive(Pod, Zeroable)]
    pub struct KeyInput: u8 {
        const UP    = 0b0000_0001;
        const LEFT  = 0b0000_0010;
        const DOWN  = 0b0000_0100;
        const RIGHT = 0b0000_1000;
    }
}
