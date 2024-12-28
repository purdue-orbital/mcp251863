use arbitrary_int::u31;
use bitbybit::bitfield;

use crate::impl_register;

impl_register!(C1RXIF, RXInterrupStatusRegister, 0x020, 4, u32);

/// Register 4-15, Receive Interrupt Status Register
#[bitfield(u32, default = 0)]
pub struct C1RXIF {
	#[bits(1..=31, r)]
	rx_fifo_pending_interupts: u31
}