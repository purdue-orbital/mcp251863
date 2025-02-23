use bitbybit::bitfield;

use crate::impl_register;

impl_register!(C1TXIF, TransmitInterruptStatus, 0x0024, 4, u32);

#[bitfield(u32, default = 0)]
pub struct C1TXIF {
    #[bits(0..=31, r)]
	tfif: u32,
}