use bitbybit::{bitenum, bitfield};
use arbitrary_int::{u2, u5};
use embedded_hal::spi::SpiDevice;

use crate::MCP251863;

use crate::registers::Register;

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
#[bitfield(u8, default = 0b0110_0000)]
pub struct C1CON0 {
	#[bits(0..=4, rw)]
	device_net_filter_bit_number_bits: u5,
	
	/// can only be modified in config mode
	#[bits(5..=5, rw)]
	iso_crc_enable: bool,
	
	/// can only be modified in config mode
	#[bits(6..=6, rw)]
	pxedis: bool,
	
	// other bits are ignored
}

/// Register 4-7, byte 1
#[bitfield(u8, default = 0b0000_0111)]
pub struct C1CON1 {
	/// can only be modified in config mode
	#[bits(0..=0, rw)]
	wake_filter_enable: bool,

	/// can only be modified in config mode
	#[bits(1..=2, rw)]
	wake_filter: u2,

	#[bits(3..=3, r)]
	busy: bool,

	#[bits(4..=4, rw)]
	bit_rate_switching_disable: bool

	// other bits are ignored
}

/// Register 4-7, byte 2
#[bitfield(u8, default = 0b1001_1000)]
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

	/// see note 2
	#[bits(5..=7, r)]
	operation_mode: OperationMode,
}


#[bitenum(u4, exhaustive = true)]
pub enum TransmitGap {
	NoDelay = 0,
	Delay2 = 1,
	Delay4 = 2,
	Delay8 = 3,
	Delay16 = 4,
	Delay32 = 5,
	Delay64 = 6,
	Delay128 = 7,
	Delay256 = 8,
	Delay512 = 9,
	Delay1024 = 10,
	Delay2048 = 11,
	Delay4096 = 12,

	// these exist to make it exhaustive, they are equivelant to `Delay4096`
	_NA01 = 13,
	_NA02 = 14,
	_NA03 = 15,
}

/// Register 4-7, byte 3
#[bitfield(u8, default = 0b0000_0100)]
pub struct C1CON3 {
	#[bits(0..=2, rw)]
	requested_operation_mode: OperationMode,

	#[bits(3..=3, rw)]
	abort_pending_transmissions: bool,

	#[bits(4..=7, rw)]
	transmit_gap: TransmitGap,
}

impl MCP251863 {
	pub fn set_operation_mode(&mut self, bus: &mut impl SpiDevice, mode: OperationMode) -> Result<(), ()> {
		todo!()
	}
}

impl Register<1> for C1CON0 {
	const ADDR_16_BIT: u16 = 0;

	fn from_bytes(value: [u8; Self::SIZE]) -> Self {
		Self::new_with_raw_value(u8::from_le_bytes(value))
	}

	fn to_bytes(self) -> [u8; Self::SIZE] {
		self.raw_value.to_le_bytes()
	}
}

impl Register<1> for C1CON1 {
	const ADDR_16_BIT: u16 = 0;

	fn from_bytes(value: [u8; Self::SIZE]) -> Self {
		Self::new_with_raw_value(u8::from_le_bytes(value))
	}

	fn to_bytes(self) -> [u8; Self::SIZE] {
		self.raw_value.to_le_bytes()
	}
}

impl Register<1> for C1CON2 {
	const ADDR_16_BIT: u16 = 0;

	fn from_bytes(value: [u8; Self::SIZE]) -> Self {
		Self::new_with_raw_value(u8::from_le_bytes(value))
	}

	fn to_bytes(self) -> [u8; Self::SIZE] {
		self.raw_value.to_le_bytes()
	}
}

impl Register<1> for C1CON3 {
	const ADDR_16_BIT: u16 = 0;

	fn from_bytes(value: [u8; Self::SIZE]) -> Self {
		Self::new_with_raw_value(u8::from_le_bytes(value))
	}

	fn to_bytes(self) -> [u8; Self::SIZE] {
		self.raw_value.to_le_bytes()
	}
}
