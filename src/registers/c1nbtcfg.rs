use bitbybit::{bitenum, bitfield};
use arbitrary_int::{u2, u5, u7};

use crate::registers::Register;


/// Register 4-8, Nominal Bit Time Configuration Register
#[bitfield(u32, default = 0b0000_0000_0011_1110_0000_1111_0000_1111)]
pub struct C1NBTCFG {
	#[bits(24..=31, rw)]
	baud_rate_prescaler: u8,

	#[bits(16..=23, rw)]
	time_segment_1_bits: u8,

	#[bits(8..=14, rw)]
	time_segment_2_bits: u7,

	#[bits(0..=6, rw)]
	synchronization_jump_width_bits: u7
}


impl Register<4> for C1NBTCFG {
	const ADDR_16_BIT: u16 = 4;

	fn from_bytes(value: [u8; Self::SIZE]) -> Self {
		Self::new_with_raw_value(u32::from_le_bytes(value))
	}

	fn to_bytes(self) -> [u8; Self::SIZE] {
		self.raw_value.to_le_bytes()
	}
}g