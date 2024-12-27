use arbitrary_int::u12;
use embedded_hal::spi::SpiDevice;

use crate::instruction::Instruction;

pub mod c1con;
pub mod c1nbtcfg;
pub mod c1dbtcfg;
pub mod c1tdc;
pub mod c1tbc;

pub mod prelude {
	pub use super::c1con::*;
	pub use super::c1nbtcfg::*;
	pub use super::c1dbtcfg::*;
	pub use super::c1tdc::*;
	pub use super::c1tbc::*;
}

// todo: maybe make this derivable with a macro
/// This trait writes the SPI handling code for us for any size register at a given address
pub trait Register<const S: usize>: Sized {
	/// Register address as u16 (converted to u12 at compile time)
	const ADDR_16_BIT: u16;
	/// Register address as u12
	const ADDR: u12 = u12::from_u16(Self::ADDR_16_BIT);
	/// Size of `Self` in bytes
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
	/// 
	/// 
	/// # Examples
	/// 
	/// ``` no_run
	/// CANControl::modify_register_safe(bus, |reg| {
	/// 	reg.with_requested_operation_mode(mode)
	/// })
	/// ```
	fn modify_register_safe<F: Fn(Self) -> Self>(bus: &mut impl SpiDevice, f: F) ->  Result<(), ()> {
		let mut register = Self::get_register_crc(bus).unwrap();
		register = f(register);

		register.set_register_safe(bus).unwrap();
		Ok(())
	}
}


/* Register impl template
impl Register<4> for _____ {
	const ADDR_16_BIT: u16 = 0x___;

	fn from_bytes(value: [u8; Self::SIZE]) -> Self {
		Self::new_with_raw_value(u32::from_le_bytes(value))
	}

	fn to_bytes(self) -> [u8; Self::SIZE] {
		self.raw_value.to_le_bytes()
	}
}
*/