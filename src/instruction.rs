use bitbybit::{bitenum, bitfield};
use arbitrary_int::{u4, u12};

/// ## See table 5-1 in datasheet
#[derive(Debug)]
#[bitenum(u4, exhaustive = true)]
enum Command {
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
	NA01 = 1,
	NA04 = 4,
	NA05 = 5,
	NA06 = 6,
	NA07 = 7,
	NA08 = 8,
	NA09 = 9,
	NA13 = 13,
	NA14 = 14,
	NA15 = 15,
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
