#![no_std]

use embedded_hal::spi::{SpiDevice, Operation};
use arbitrary_int::u12;

mod status;
pub mod registers;
mod instruction;
mod can;

use registers::c1con::OperationMode;
use instruction::{Instruction, InstructionError};

// uses SPI mode 0,0 or 1, 1
#[derive(Debug)]
pub struct MCP251863 {
	mode: OperationMode,
	max_spi_retries: i8,
	max_spi_crc_retries: i8,
}



impl MCP251863 {
	pub fn reset(&mut self, bus: &mut impl SpiDevice) -> Result<(), InstructionError> {
		if self.mode != OperationMode::Config {
			self.set_operation_mode(bus, OperationMode::Config).unwrap(); // TODO!!! remove unwrap
		}

		Instruction::reset(bus, u12::from_u16(0))
	}
}