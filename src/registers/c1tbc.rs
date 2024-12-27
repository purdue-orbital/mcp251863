use crate::registers::Register;

pub use C1TBC as TimeBaseCounter;

/// Register 4-11, Time Base Counter Register
/// 
/// Value is reset on any write to Time Base Counter.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct C1TBC(u32);

impl Register<4> for C1TBC {
	const ADDR_16_BIT: u16 = 0x010;

	fn from_bytes(value: [u8; Self::SIZE]) -> Self {
		Self(u32::from_le_bytes(value))
	}

	fn to_bytes(self) -> [u8; Self::SIZE] {
		self.0.to_le_bytes()
	}
}