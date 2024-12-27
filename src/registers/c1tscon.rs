use arbitrary_int::u10;
use bitbybit::bitfield;

use crate::registers::Register;

pub use C1TSCON as TimeStampController;

#[bitfield(u32, default = 0)]
pub struct C1TSCON {
	#[bits(18..=18, rw)]
	time_stamp_resolution: bool,

	#[bits(17..=17, rw)]
	time_stamp_end_of_frame: bool,

	#[bits(16..=16, rw)]
	time_base_counter: bool,

	/// prescaler divides by `time_base_counter_prescaler` + 1
	#[bits(0..=9, rw)]
	time_base_counter_prescaler: u10,
}

impl Register<4> for C1TSCON {
	const ADDR_16_BIT: u16 = 0x014;

	fn from_bytes(value: [u8; Self::SIZE]) -> Self {
		Self::new_with_raw_value(u32::from_le_bytes(value))
	}

	fn to_bytes(self) -> [u8; Self::SIZE] {
		self.raw_value.to_le_bytes()
	}
}