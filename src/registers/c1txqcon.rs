use bitbybit::bitfield;
use arbitrary_int::{u2, u3, u5};
use crate::impl_register;

impl_register!(C1TXQCON, TransmitQueueControl, 0x04C, 4, u32);

#[bitfield(u32, default = 0x00_60_04_80)]
pub struct C1TXQCON {
    #[bits(0..=0, rw)]
    txqnie: bool,

    #[bits(2..=2, rw)]
    txqeie: bool,

    #[bits(4..=4, rw)]
    txatie: bool,

    #[bits(7..=7, r)]
    txen: bool,

    #[bits(8..=8)]
    uinc: bool,

    #[bits(9..=9, rw)]
    txreq: bool,

    #[bits(10..=10)]
    freset: bool,

    #[bits(16..=20, rw)]
    txpri: u5,

    #[bits(21..=22, rw)]
    txat: u2,

    #[bits(24..=28, rw)]
    fsize: u5,

    #[bits(29..=31, rw)]
    plsize: u3
}