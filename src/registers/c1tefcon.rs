use bitbybit::bitfield;
use arbitrary_int::u5;

use crate::impl_register;

impl_register!(C1TEFCON, TransmitEventFIFOControl, 0x0040, 4, u32);

#[bitfield(u32, default = 0x00_00_04_00)]
pub struct C1TEFCON {
    #[bits(0..=0, rw)]
    tefneie: bool,

    #[bits(1..=1, rw)]
    tefhie: bool,

    #[bits(2..=2, rw)]
    teffie: bool,

    #[bits(3..=3, rw)]
    tefovie: bool,

    #[bits(5..=5, rw)]
    teftsen: bool,

    #[bits(8..=8)]
    uinc: bool,

    #[bits(9..=9)]
    freset: bool,

    #[bits(24..=28, rw)]
    fsize: u5,
}