use bitbybit::{bitenum, bitfield};
use arbitrary_int::u12;
use embedded_hal::spi::{SpiDevice, Operation};

use crate::MCP251863;

/// ## See table 5-1 in datasheet
#[derive(Debug)]
#[bitenum(u4, exhaustive = true)]
pub enum Command {
	/// ## See 5.1.1 in datasheet
	/// 
	/// The RESET instruction should only be issued after the device enters Configuration mode. All
	/// SFR and State Machines are reset same as during a Power-on Reset (POR), and the device
	/// transitions immediately to Configuration mode. The Message Memory is not changed.
	/// 
	/// The actual reset happens at the end of the instruction when nCS goes high.
	Reset = 0b0000,

	Read = 0b0011,
	Write = 0b0010,
	ReadCRC = 0b1011,
	WriteCRC = 0b1010,
	WriteSafe = 0b1100,

	// these only exist to make the enum exhastive (so it can be used in `Instruction`)
	_NA01 = 1,
	_NA04 = 4,
	_NA05 = 5,
	_NA06 = 6,
	_NA07 = 7,
	_NA08 = 8,
	_NA09 = 9,
	_NA13 = 13,
	_NA14 = 14,
	_NA15 = 15,
}

impl Command {
	pub const fn with_address(&self, addr: u12) -> Instruction {
		Instruction::builder()
			.with_command(*self)
			.with_address(addr)
			.build()
	}
}

#[bitfield(u16, default = 0)]
pub struct Instruction {
	#[bits(12..=15, rw)]
	command: Command,

	#[bits(0..=11, rw)]
	address: u12,
}

impl Instruction {
	pub const fn to_bytes(&self) -> [u8; 2] {
		self.raw_value().to_le_bytes()
	}
}


impl MCP251863 {
	fn read_byte(bus: &mut impl SpiDevice, addr: u12) -> Option<u8> {
		let mut buf = [0; 1];

		bus.transaction(&mut [
			Operation::Write(&Command::Read.with_address(addr).to_bytes()),
			Operation::Read(&mut buf)
		]).unwrap(); // TODO!!! remove unwrap

		Some(buf[0])
	}

	fn read_byte_crc(bus: &mut impl SpiDevice, addr: u12) -> Option<u8> {
		todo!(); 

		let mut buf = [0; 1];

		Some(buf[0])
	}

	fn write_byte_crc(bus: &mut impl SpiDevice, addr: u12, byte: u8) -> Result<(), ()> {
		todo!()
	}
}