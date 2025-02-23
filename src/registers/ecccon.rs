use bitbybit::bitfield;
use arbitrary_int::u7;
use crate::impl_register;

impl_register!(ECCCON, ErrorCodeCaptureControl, 0xE0C, 4, u32);

#[bitfield(u32, default = 0x00_00_00_00)]
pub struct ECCCON {
    #[bit(0, rw)]
    eccen: bool,

    #[bit(1, rw)]
    secie: bool,

    #[bit(2, rw)]
    dedie: bool,

    #[bits(3..=9, rw)]
    parity: u7,
}