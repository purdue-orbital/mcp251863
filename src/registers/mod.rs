use arbitrary_int::u12;
use embedded_hal::spi::SpiDevice;

pub mod c1con;

pub trait Register: Sized {
	const ADDR: u12;
	const READ_SIZE: usize;

	fn get(bus: &mut impl SpiDevice) -> Option<Self> {
		todo!()
	}
}