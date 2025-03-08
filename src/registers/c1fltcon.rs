use bitbybit::bitfield;
use crate::{impl_register, registers::Register};
use arbitrary_int::{u5, u12};

impl_register!(C1FLTCON, FilterControl, 0x1D0, 4, u32);

#[bitfield(u32, default = 0)]
pub struct C1FLTCON {
    #[bits(0..=4, rw)]
    f0bp: u5,

    #[bit(7, rw)]
    flten0: bool,

    #[bits(8..=12, rw)]
    f1bp: u5,

    #[bit(15, rw)]
    flten1: bool,

    #[bits(16..=20, rw)]
    f2bp: u5,

    #[bit(23, rw)]
    flten2: bool,

    #[bits(24..=28, rw)]
    f3bp: u5,

    #[bit(31, rw)]
    flten3: bool,
}

impl C1FLTCON {
    pub fn addr_n(n: u8) -> u12 {
        Self::ADDR + u12::from_u8(n - 1)
    }
}

