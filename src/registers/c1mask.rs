use bitbybit::bitfield;
use crate::{impl_register, registers::Register};
use arbitrary_int::{u11, u12, u18};

impl_register!(C1MASK,  Mask,  0x1F4, 4, u32);

#[bitfield(u32, default = 0)]
pub struct C1MASK {
    #[bits(0..=10, rw)]
    msid: u11,

    #[bits(11..=28, rw)]
    meid: u18,

    #[bit(29, rw)]
    msid11: bool,

    #[bit(30, rw)]
    mide: bool,
}

impl C1MASK {
    pub fn addr_n(n: u8) -> u12 {
        Self::ADDR + u12::from_u8(8 * (n - 1))
    }
}