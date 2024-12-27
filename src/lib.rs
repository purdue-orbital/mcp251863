#![no_std]

use embedded_hal::spi::{SpiDevice, Operation};
use arbitrary_int::u12;

mod status;
pub mod registers;
mod instruction;

use registers::c1con::OperationMode;
use instruction::Command;

// uses SPI mode 0,0 or 1, 1
#[derive(Debug)]
pub struct MCP251863 {
	mode: OperationMode
}


impl MCP251863 {
	pub fn reset(&mut self, bus: &mut impl SpiDevice) -> Option<()> {
		if self.mode != OperationMode::Config {
			self.set_operation_mode(bus, OperationMode::Config).unwrap(); // TODO!!! remove unwrap
		}

		bus.transaction(&mut [
			Operation::Write(&Command::Reset.with_address(u12::from_u16(0)).to_bytes())
		]).unwrap(); // TODO!!! remove unwrap

		Some(())
	}
}