use arbitrary_int::u6;
use bitbybit::{bitfield, bitenum};

use crate::impl_register;

// #[derive(Debug)]
#[bitenum(u2, exhaustive = true)]
pub enum TransmitterDelayCompensationMode {
	Disabled = 0,
	Manual = 1,
	Auto = 2,
	_NA01 = 3
}

impl_register!(C1TDC, TransmitterDelayCompensation, 0x00c, 4, u32);

/// Register 4-10, Transmitter Delay Compensation Register
/// 
/// This register can only be modified in config mode.
#[bitfield(u32, default = 0b0000_0000_0000_0010_0001_0000_0000_0000)]
pub struct C1TDC {
	#[bits(25..=25, rw)]
	bus_integration_edge_filtering: bool,

	/// Enable 12-bit SID in CAN FD Base Format Messages
	#[bits(24..=24, rw)]
	sid_11_enable: bool,

	#[bits(16..=17, rw)]
	mode: TransmitterDelayCompensationMode,

	#[bits(8..=13, rw)]
	offset: u6,

	#[bits(0..=5, rw)]
	value: u6,
}