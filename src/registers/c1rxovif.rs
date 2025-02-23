use bitbybit::{bitfield};
use arbitrary_int::u31;

use crate::impl_register;

impl_register!(C1RXOVIF, RecieveOverflowInterruptStatus, 0x0030, 4, u32);

#[bitfield(u32, default = 0)]
pub struct C1RXOVIF {
    #[bits(1..=31, r)]
	rfovif: u31,
}