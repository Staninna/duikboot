use crate::settings::player::{FLAG_DOWN, FLAG_LEFT, FLAG_RIGHT, FLAG_UP};
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
        const UP    = FLAG_UP;
        const LEFT  = FLAG_LEFT;
        const DOWN  = FLAG_DOWN;
        const RIGHT = FLAG_RIGHT;
    }
}
