use bitbybit::bitfield;
use arbitrary_int::u12;
use crate::{impl_register, registers::Register};

impl_register!(C1FIFOUA,  FIFOUserAdress,  0x64,  4, u32);

impl C1FIFOUA {
    /// address of the *n*th fifo control register
    pub fn addr_n(n: u8) -> u12 {
        Self::ADDR + u12::from_u8(12 * (n - 1))
    }
}

#[bitfield(u32)]
pub struct C1FIFOUA {
    #[bits(0..=31, r)]
    fifoua: u32,
}
