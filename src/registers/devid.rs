use bitbybit::bitfield;
use arbitrary_int::u4;
use crate::impl_register;

impl_register!(DEVID, DeviceID, 0xE14, 4, u32);

#[bitfield(u32, default = 0x00_00_00_00)]
pub struct DEVID {
    #[bits(0..=3, r)]
    rev: u4,

    #[bits(4..=7, r)]
    id: u4,
}