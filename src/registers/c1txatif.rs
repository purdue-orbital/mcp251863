use bitbybit::bitfield;

use crate::impl_register;

impl_register!(C1TXATIF, TransmitAttemptInterruptStatus, 0x002C, 4, u32);

#[bitfield(u32, default = 0)]
pub struct C1TXATIF {
    #[bits(0..=31, r)]
	tfatif: u32,
}