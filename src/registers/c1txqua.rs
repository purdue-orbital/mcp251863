use bitbybit::bitfield;

use crate::impl_register;

impl_register!(C1TXQUA, TransmitQueueUserAddress, 0x0050, 4, u32);

#[bitfield(u32)]
pub struct C1TXQUA {
    #[bits(0..=31, r)]
    txqua: u32,
}