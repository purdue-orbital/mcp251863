use bitbybit::bitfield;
use arbitrary_int::u7;

use crate::impl_register;

pub use C1NBTCFG as NominalBitTimeConfiguration;

impl_register!(C1NBTCFG, 0x04, 4, u32);

/// Register 4-8, Nominal Bit Time Configuration Register
/// 
/// This register can only be modified in config mode.
#[bitfield(u32, default = 0b0000_0000_0011_1110_0000_1111_0000_1111)]
pub struct C1NBTCFG {
	#[bits(24..=31, rw)]
	baud_rate_prescaler: u8,

	#[bits(16..=23, rw)]
	time_segment_1: u8,

	#[bits(8..=14, rw)]
	time_segment_2: u7,

	#[bits(0..=6, rw)]
	synchronization_jump_width: u7
}