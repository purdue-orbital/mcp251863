use arbitrary_int::{u5, u7};
use bitbybit::bitfield;

use crate::impl_register;

impl_register!(C1VEC, 0x018, 4, u32);

/// Register 4-13, Interrupt Code Register
#[bitfield(u32, default = 0b0100_0000_0100_0000_0000_0000_0000_0000)]
pub struct C1VEC {
	/// if multiple interupts are pending, only highest priority will be shown (highest value)
	#[bits(24..=30, r)]
	receive_interrupt_flag_code: u7, // TODO: make enum

	/// if multiple interupts are pending, only highest priority will be shown (highest value)
	#[bits(16..=22, r)]
	transmit_interrupt_flag_code: u7, // TODO: make enum

	/// if multiple interupts are pending, only highest priority will be shown (highest value)
	#[bits(8..=12, r)]
	hit_filter_number: u5,

	/// if multiple interupts are pending, only highest priority will be shown (highest value)
	#[bits(0..=6, r)]
	interupt_flag_code: u7 // TODO: make enum
}