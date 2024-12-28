use arbitrary_int::u10;
use bitbybit::bitfield;

use crate::impl_register;

impl_register!(C1TSCON, TimeStampController, 0x014, 4, u32);

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