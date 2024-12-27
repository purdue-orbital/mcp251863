use arbitrary_int::{u4, u5};
use bitbybit::bitfield;

use crate::impl_register;

pub use C1DBTCFG as DataBitTimeConfiguration;

impl_register!(C1DBTCFG, 0x008, 4, u32);

/// Register 4-9, Data Bit Time Configuration Register
///
/// This register can only be modified in config mode.
#[bitfield(u32, default = 0b0000_0000_0000_1110_0000_0011_0000_0011)]
pub struct C1DBTCFG {
	#[bits(24..=31, rw)]
	baud_rate_prescaler: u8,

	#[bits(16..=20, rw)]
	time_segment_1: u5,

	#[bits(8..=11, rw)]
	time_segement_2: u4,

	#[bits(0..=3, rw)]
	synchronization_jump_width: u4
}
