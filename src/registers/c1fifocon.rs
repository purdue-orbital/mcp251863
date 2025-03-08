use arbitrary_int::{u2, u3, u5, u12};
use bitbybit::bitfield;
use crate::impl_register;
use super::Register;

impl_register!(C1FIFOCON, FIFOControl, 0x5C, 4, u32);

#[bitfield(u32, default = 0x00_60_04_00)]
pub struct C1FIFOCON {
    #[bits(0..=0, rw)]
    tfnrfnie: bool,

    #[bits(1..=1, rw)]
    tfhrfhie: bool,

    #[bits(2..=2, rw)]
    tferffie: bool,

    #[bits(3..=3, rw)]
    rxovie: bool,

    #[bits(4..=4, rw)]
    txatie: bool,

    #[bits(5..=5, rw)]
    rxtsen: bool,

    #[bits(6..=6, rw)]
    rtren: bool,

    #[bits(7..=7, rw)]
    txen: bool,

    #[bits(8..=8, rw)]
    uinc: bool,

    #[bits(9..=9, rw)]
    txreq: bool,

    #[bits(10..=10, rw)]
    freset: bool,

    #[bits(16..=20, rw)]
    txpri: u5,

    #[bits(21..=22, rw)]
    txat: u2,

    #[bits(24..=28, rw)]
    fsize: u5,

    #[bits(29..=31, rw)]
    plsize: u3,
}

impl C1FIFOCON {
    /// address of the *n*th fifo control register
    pub fn addr_n(n: u8) -> u12 {
        Self::ADDR + u12::from_u8(12 * (n - 1))
    }
}