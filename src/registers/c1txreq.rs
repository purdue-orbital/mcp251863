use bitbybit::bitfield;

use crate::impl_register;

impl_register!(C1TXREQ, TransmitRequest, 0x0030, 4, u32);

#[bitfield(u32, default = 0)]
pub struct C1TXREQ {
    #[bits(0..=31)]
	txreq: u32,
}