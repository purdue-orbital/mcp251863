use bitbybit::bitfield;
use crate::{impl_register, registers::Register};
use arbitrary_int::{u11, u12, u18};

impl_register!(C1FLTOBJ,  FilterObject,  0x1F0, 4, u32);

#[bitfield(u32, default = 0)]
pub struct C1FLTOBJ {
    #[bits(0..=10, rw)]
    sid: u11,

    #[bits(11..=28, rw)]
    eid: u18,

    #[bit(29, rw)]
    sid11: bool,

    #[bit(30, rw)]
    exide: bool,
}

impl C1FLTOBJ {
    pub fn addr_n(n: u8) -> u12 {
        Self::ADDR + u12::from_u8(8 * (n - 1))
    }
}

