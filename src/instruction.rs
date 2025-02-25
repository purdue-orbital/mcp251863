use bitbybit::{bitenum, bitfield};
use arbitrary_int::{u12, u24};
use embedded_hal::spi::{SpiDevice, Operation, Error, ErrorKind};

use crate::registers::{c1int::Interrupt, Register};

use crc;
const CRC_ALGO: crc::Crc<u16> = crc::Crc::<u16>::new(&crc::CRC_16_USB);
const C1INT_SPICRCIF_MASK: u8 = 0b0000_0010;

#[derive(Clone, Copy, Debug)]
pub enum InstructionError {
	External(ErrorKind),
	CRCError,
}

impl Error for InstructionError {
	fn kind(&self) -> ErrorKind {
		match self {
			InstructionError::External(kind) => *kind,
			InstructionError::CRCError => ErrorKind::Other,
		}
	}
}

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

	pub fn reset(bus: &mut impl SpiDevice, addr: u12) -> Result<(), InstructionError> {
		match bus.transaction(&mut [
			Operation::Write(&Command::Reset.with_address(addr).to_bytes())
		]) {
			Ok(_) => Ok(()),
			Err(e) => Err(InstructionError::External(e.kind())),
		}
	}

	pub fn read(bus: &mut impl SpiDevice, addr: u12, buf: &mut [u8]) -> Result<(), InstructionError> {

		match bus.transaction(&mut [
			Operation::Write(&Command::Read.with_address(addr).to_bytes()),
			Operation::Read(buf)
		]) {
			Ok(_) => Ok(()),
			Err(e) => Err(InstructionError::External(e.kind())),
		}
	}

	pub fn write(bus: &mut impl SpiDevice, addr: u12, buf: &[u8]) -> Result<(), InstructionError> {

		match bus.transaction(&mut [
			Operation::Write(&Command::Write.with_address(addr).to_bytes()),
			Operation::Write(buf)
		]) {
			Ok(_) => Ok(()),
			Err(e) => Err(InstructionError::External(e.kind())),
		}
	}

	pub fn read_crc(bus: &mut impl SpiDevice, addr: u12, buf: &mut [u8]) -> Result<(), InstructionError> {

		let cmd = Command::ReadCRC.with_address(addr).to_bytes();
		let num_bytes = buf.len() as u8;

		let read_result = bus.transaction(&mut [
			Operation::Write(&cmd),
			Operation::Write(&[num_bytes]),
			Operation::Read(buf)
		]);

		// check recieved crc checksum for errors
		// checksum calculated on command, address, data length, data (datasheet pg 70)
		let crc_errors_present = {
			let mut crc_digest = CRC_ALGO.digest();
			crc_digest.update(&cmd);
			crc_digest.update(&[num_bytes]);
			crc_digest.update(&buf[..(buf.len() - 2)]);
			u16::from_le_bytes([buf[buf.len() - 2], buf[buf.len() - 1]]) != crc_digest.finalize()
		};

		if crc_errors_present {
			Err(InstructionError::CRCError)
		} else {
			match read_result {
				Ok(_) => Ok(()),
				Err(e) => Err(InstructionError::External(e.kind())),
			}
		}
	}

	pub fn write_crc(bus: &mut impl SpiDevice, addr: u12, buf: &[u8]) -> Result<(), InstructionError> {

		// write data and crc
		let cmd = Command::WriteCRC.with_address(addr).to_bytes();
		let num_bytes = buf.len() as u8;

		let checksum = {
			let mut crc_digest = CRC_ALGO.digest();
			crc_digest.update(&cmd);
			crc_digest.update(&[num_bytes]);
			crc_digest.update(buf);
			crc_digest.finalize().to_le_bytes()
		};

		let read_crc_interrupt_cmd = Command::ReadCRC.with_address(Interrupt::ADDR + u12::from(1u8)).to_bytes();
		let mut crc_interrupt_buf = [0u8; 3];

		let write_result = bus.transaction(&mut [
			// Write data to device
			Operation::Write(&cmd),
			Operation::Write(&[num_bytes]),
			Operation::Write(buf),
			Operation::Write(&checksum),

			// read the CRC register
			Operation::Write(&read_crc_interrupt_cmd),
			Operation::Write(&[1u8]), // number of bytes to read
			Operation::Read(&mut crc_interrupt_buf), // interrupt register + checksum
		]);
		
		// check recieved crc checksuhm for errors
		// checksum calculated on command, address, data length, data (datasheet pg 70)
		let crc_errors_present = {
			let mut crc_digest = CRC_ALGO.digest();
			crc_digest.update(&read_crc_interrupt_cmd);
			crc_digest.update(&[1u8]);
			crc_digest.update(&crc_interrupt_buf[..1]);
			u16::from_le_bytes([crc_interrupt_buf[1], crc_interrupt_buf[2]]) != crc_digest.finalize()
		};

		// check for errors when reading 
		if crc_errors_present {
			Err(InstructionError::CRCError)	
		} 

		// check if CRC interrupt is set
		else if crc_interrupt_buf[0] & C1INT_SPICRCIF_MASK != 0 { // If set, clear the interrupt
			match Self::try_clear_crc_interrupt(bus, crc_interrupt_buf[0], 5) {
				Ok(_) => Err(InstructionError::CRCError),
				Err(e) => Err(InstructionError::External(e.kind())),
			}
		}

		// return result of bus transaction
		else {
			match write_result {
				Ok(_) => Ok(()),
				Err(e) => Err(InstructionError::External(e.kind())),
			}
		}
	}

	/// clears the CRC interrupt flag in the C1INT register, tries again if it encounters a CRCError
	fn try_clear_crc_interrupt(bus: &mut impl SpiDevice, mut crc_interrupt_buf: u8, max_tries: u8) -> Result<(), InstructionError> {

		crc_interrupt_buf ^= C1INT_SPICRCIF_MASK; // clear the spicrcif interrupt

		for _ in 0..max_tries {
			let interrupt_clear_result = Self::write_safe(bus, Interrupt::ADDR + u12::from_u8(1), &[crc_interrupt_buf]);

			match interrupt_clear_result {
				Ok(_) => return Ok(()),
				Err(e) => match e {
					InstructionError::CRCError => continue,
					_ => return Err(e),
				}
			}
		}

		Err(InstructionError::CRCError)
	}

	pub fn write_safe(bus: &mut impl SpiDevice, addr: u12, buf: &[u8; 1]) -> Result<(), InstructionError> {

		// write data and crc
		let cmd = Command::WriteCRC.with_address(addr).to_bytes();

		// checksum calculated on command, address, and data
		let checksum = {
			let mut crc_digest = CRC_ALGO.digest();
			crc_digest.update(&cmd);
			crc_digest.update(buf);
			crc_digest.finalize().to_le_bytes()
		};

		let read_crc_interrupt_cmd = Command::ReadCRC.with_address(Interrupt::ADDR + u12::from(1u8)).to_bytes();
		let mut crc_interrupt_buf = [0u8; 3];

		let write_result = bus.transaction(&mut [
			// Write data to device
			Operation::Write(&cmd),
			Operation::Write(buf),
			Operation::Write(&checksum),

			// read the CRC register
			Operation::Write(&read_crc_interrupt_cmd),
			Operation::Write(&[1u8]), // number of bytes to read
			Operation::Read(&mut crc_interrupt_buf), // interrupt register + checksum
		]);
		
		// check recieved crc checksum for errors
		let crc_errors_present = {
			let mut crc_digest = CRC_ALGO.digest();
			crc_digest.update(&read_crc_interrupt_cmd);
			crc_digest.update(&[1u8]);
			crc_digest.update(&crc_interrupt_buf[..1]);
			u16::from_le_bytes([crc_interrupt_buf[1], crc_interrupt_buf[2]]) != crc_digest.finalize()
		};

		// check for errors when reading 
		if crc_errors_present {
			Err(InstructionError::CRCError)	
		} 

		// check if CRC interrupt is set
		else if crc_interrupt_buf[0] & C1INT_SPICRCIF_MASK != 0 { // If set, clear the interrupt
			match Self::try_clear_crc_interrupt(bus, crc_interrupt_buf[0], 5) {
				Ok(_) => Err(InstructionError::CRCError),
				Err(e) => Err(InstructionError::External(e.kind())),
			}
		}

		// return result of bus transaction
		else {
			match write_result {
				Ok(_) => Ok(()),
				Err(e) => Err(InstructionError::External(e.kind())),
			}
		}
	}
	
	
}