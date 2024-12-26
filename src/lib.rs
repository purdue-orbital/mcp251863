use embedded_hal::spi::SpiDevice;
use arbitrary_int::u12;

mod status;
mod mode;
pub mod instruction;

// uses SPI mode 0,0 or 1, 1
#[derive(Debug)]
pub struct MCP251863 {
	mode: mode::OperationMode
}

impl MCP251863 {
	fn read_byte(addr: u12) -> Option<u8> {
		
	}
}