use arbitrary_int::u12;
use embedded_hal::spi::SpiDevice;

use crate::instruction::Instruction;

pub mod c1con;

// todo: maybe make this derivable with a macro
/// This trait writes the SPI handling code for us for any size register at a given address
pub trait Register<const S: usize>: Sized {
	/// Register address
	const ADDR_16_BIT: u16;
	const ADDR: u12 = u12::from_u16(Self::ADDR_16_BIT);
	const SIZE: usize = S;

	/// Convert from bytes to `Self`
	fn from_bytes(value: [u8; S]) -> Self;

	/// Convert from `Self` to bytes
	fn to_bytes(self) -> [u8; S];

	/// Get value of the register
	fn get_register(bus: &mut impl SpiDevice) -> Result<Self, ()> {
		let mut buf = [0; S];

		Instruction::read(bus, Self::ADDR, &mut buf).unwrap();

		Ok(Self::from_bytes(buf))
	}

	/// Get value of register with CRC
	fn get_register_crc(bus: &mut impl SpiDevice) -> Result<Self, ()> {
		todo!()
	}

	/// Set value of register
	fn set_register(self, bus: &mut impl SpiDevice) -> Result<(), ()> {
		let buf = self.to_bytes();

		Instruction::write(bus, Self::ADDR, &buf).unwrap();

		Ok(())
	}

	/// Set value of register with CRC (see also `set_register_safe`)
	fn set_register_crc(self, bus: &mut impl SpiDevice) -> Result<(), ()> {
		todo!()
	}

	/// Set value of register with a safe write (see also `set_register_crc`)
	fn set_register_safe(self, bus: &mut impl SpiDevice) -> Result<(), ()> {
		todo!()
	}

	/// Modify value of register using a closure. Uses a crc read and a safe write
	fn modify_register_safe<F: Fn(Self) -> Self>(self, bus: &mut impl SpiDevice, f: F) ->  Result<(), ()> {
		let mut register = Self::get_register_crc(bus).unwrap();
		register = f(register);

		register.set_register_safe(bus).unwrap();
		Ok(())
	}
}