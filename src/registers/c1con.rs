use bitbybit::{bitenum, bitfield};
use arbitrary_int::{u2, u5};

use embedded_hal::spi::SpiDevice;
use crate::MCP251863;

use crate::impl_register;

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

#[derive(Debug, Eq)]
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

impl TransmitGap {
	pub fn gap(&self) -> u16 {
		match self {
			TransmitGap::NoDelay => 0,
			TransmitGap::Delay2 => 2,
			TransmitGap::Delay4 => 4,
			TransmitGap::Delay8 => 8,
			TransmitGap::Delay16 => 16,
			TransmitGap::Delay32 => 32,
			TransmitGap::Delay64 => 64,
			TransmitGap::Delay128 => 128,
			TransmitGap::Delay256 => 256,
			TransmitGap::Delay512 => 512,
			TransmitGap::Delay1024 => 1024,
			TransmitGap::Delay2048 => 2048,
			TransmitGap::Delay4096 => 4096,
			TransmitGap::_NA01 => 4096,
			TransmitGap::_NA02 => 4096,
			TransmitGap::_NA03 => 4096,
		}
	}
}

impl PartialEq for TransmitGap {
	fn eq(&self, other: &Self) -> bool {
		self.gap() == other.gap()
	}
}

pub use C1CON as CANControl; // make an alias for c1con

impl_register!(C1CON, 0x000, 4, u32);

/// Register 4-7, CAN Control Register
#[bitfield(u32, default = 0b0000_0100_1001_1000_0000_0111_0110_0000)]
pub struct C1CON {
	#[bits(0..=4, rw)]
	device_net_filter_bit_number_bits: u5,
	
	/// Can only be modified in config mode
	#[bits(5..=5, rw)]
	iso_crc_enable: bool,
	
	/// Can only be modified in config mode
	#[bits(6..=6, rw)]
	pxedis: bool,

	/// Can only be modified in config mode
	#[bits(8..=8, rw)]
	wake_filter_enable: bool,

	/// Can only be modified in config mode
	#[bits(9..=10, rw)]
	wake_filter: u2,

	#[bits(11..=11, r)]
	busy: bool,

	#[bits(12..=12, rw)]
	bit_rate_switching_disable: bool,

	/// Can only be modified in config mode
	#[bits(16..=16, rw)]
	restrict_retransmission_attempts: bool,

	/// Can only be modified in config mode
	#[bits(17..=17, rw)]
	transmit_esi_in_gateway_mode: bool,

	/// Can only be modified in config mode
	#[bits(18..=18, rw)]
	serr_2_lom: bool, // todo make enum?

	/// Can only be modified in config mode
	#[bits(19..=19, rw)]
	store_in_transit_event_fifo: bool,

	/// Can only be modified in config mode
	#[bits(20..=20, rw)]
	transmit_queue_enable: bool,

	/// see note 2
	#[bits(21..=23, r)]
	operation_mode: OperationMode,

	#[bits(24..=26, rw)]
	requested_operation_mode: OperationMode,

	#[bits(27..=27, rw)]
	abort_pending_transmissions: bool,

	#[bits(28..=31, rw)]
	transmit_gap: TransmitGap,
}

impl MCP251863 {
	pub fn set_operation_mode(&mut self, bus: &mut impl SpiDevice, mode: OperationMode) -> Result<(), ()> {
		CANControl::modify_register_safe(bus, |reg| {
			reg.with_requested_operation_mode(mode)
		})
	}
}