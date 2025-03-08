use arbitrary_int::{u5, u12};
use bitbybit::bitfield;
use crate::{impl_register, registers::Register};

impl_register!(C1FIFOSTA,  FIFOStatus,  0x60,  4, u32);

impl C1FIFOSTA {
    /// address of the *n*th fifo control register
    pub fn addr_n(n: u8) -> u12 {
        Self::ADDR + u12::from_u8(12 * (n - 1))
    }
}

#[bitfield(u32, default = 0)]
pub struct C1FIFOSTA {
    #[bit(0, r)]
    tfnrfnif: bool,

    #[bit(1, r)]
    tfhrfhif: bool,

    #[bit(2, r)]
    tferffif: bool,

    #[bit(3)]
    rxovif: bool,

    #[bit(4)]
    txatif: bool,

    #[bit(5)]
    txerr: bool,

    #[bit(6)]
    txlarb: bool,

    #[bit(7)]
    txabt: bool,

    #[bits(8..=12, r)]
    fifoci: u5,
}
