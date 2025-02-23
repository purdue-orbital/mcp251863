use bitbybit::bitfield;
use crate::impl_register;

impl_register!(TEFUA, TransmitEventFIFOUserAddress, 0x0048, 4, u32);

#[bitfield(u32)]
pub struct TEFUA {
    #[bits(0..=31, r)]
    tefua: u32,
}