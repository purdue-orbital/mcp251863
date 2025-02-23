use bitbybit::bitfield;
use arbitrary_int::u12;
use crate::impl_register;

impl_register!(ECCSTAT, ErrorCodeCaptureStatus, 0xE10, 4, u32);

#[bitfield(u32, default = 0x00_00_00_00)]
pub struct ECCSTAT {
    #[bit(1, w)]
    secif: bool,

    #[bit(2, w)]
    dedif: bool,

    #[bits(16..=27, r)]
    erraddr: u12,
}