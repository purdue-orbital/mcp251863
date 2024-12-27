use bitbybit::{bitenum, bitfield};
use arbitrary_int::{u2, u5};
use embedded_hal::spi::SpiDevice;

use crate::MCP251863;

// note: see CiCON register
#[derive(Debug, PartialEq, Eq)]
#[bitenum(u3, exhaustive = true)]
pub enum OperationMode {
	/// supports mixing of CAN 2.0 and FD
	Normal = 0,
	/// CAN FD frames can become error frames
	NormalCAN2 = 6,
	Config = 4,
	ListenOnly = 3,
	Sleep = 1,
	InternalLoopback = 2,
	ExternalLoopback = 5,
	RestrictedOperation = 7,
}

/// Register 4-7, byte 0
#[bitfield(u8, default = 0)]
pub struct C1CON0 {
	#[bits(0..=4, rw)]
	device_net_filter_bit_number_bits: u5,
	
	/// can only be modified in config mode
	#[bits(5..=5, rw)]
	iso_crc_enable: bool,
	
	/// can only be modified in config mode
	#[bits(6..=6, rw)]
	protocol_exception_event_detection_disable: bool,
	
	// #[bits(7..=7, r)]
	// _ignore: bool,

	/// can only be modified in config mode
	#[bits(8..=8, rw)]
	wake_filter_enable: bool
}

/// Register 4-7, byte 1
#[bitfield(u8, default = 0)]
pub struct C1CON1 {
	/// can only be modified in config mode
	#[bits(0..=1, rw)]
	wake_filter: u2,

	#[bits(2..=2, r)]
	busy: bool,

	#[bits(3..=3, rw)]
	bit_rate_switching_disable: bool

	// other bits are ignored
}

/// Register 4-7, byte 2
#[bitfield(u8, default = 0)]
pub struct C1CON2 {
	/// can only be modified in config mode
	#[bits(0..=0, rw)]
	restrict_retransmission_attempts: bool,

	/// can only be modified in config mode
	#[bits(1..=1, rw)]
	transmit_esi_in_gateway_mode: bool,

	/// can only be modified in config mode
	#[bits(2..=2, rw)]
	serr_2_lom: bool, // todo make enum?

	/// can only be modified in config mode
	#[bits(3..=3, rw)]
	store_in_transit_event_fifo: bool,

	/// can only be modified in config mode
	#[bits(4..=4, rw)]
	transmit_queue_enable: bool,

	/// 
	#[bits(0..=0, rw)]
	restrict_retransmission_attempts: bool,
}


impl MCP251863 {
	pub fn set_operation_mode(&mut self, bus: &mut impl SpiDevice, mode: OperationMode) -> Result<(), ()> {
		todo!()
	}
}
