use arbitrary_int::{u4, u5};
use bitbybit::bitfield;

use crate::registers::Register;

/// Register 4-9, Data Bit Time Configuration Register
///
/// This register can only be modified in config mode.
#[bitfield(u32, default = 0b0000_0000_0000_1110_0000_0011_0000_0011)]
struct C1DBTCFG {
	#[bits(24..=31, rw)]
	baud_rate_prescaler: u8,

	#[bits(16..=20, rw)]
	time_segment_1: u5,

	#[bits(8..=11, rw)]
	time_segement_2: u4,

	#[bits(0..=3, rw)]
	synchronization_jump_width: u4
}

impl Register<4> for C1DBTCFG {
	const ADDR_16_BIT: u16 = 0x008;

	fn from_bytes(value: [u8; Self::SIZE]) -> Self {
		Self::new_with_raw_value(u32::from_le_bytes(value))
	}

	fn to_bytes(self) -> [u8; Self::SIZE] {
		self.raw_value.to_le_bytes()
	}
}