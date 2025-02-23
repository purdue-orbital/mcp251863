use bitbybit::bitfield;
use crate::impl_register;

impl_register!(IOCON, IOControl, 0xE04, 4, u32);

#[bitfield(u32, default = 0x03_00_00_03)]
pub struct IOCON {
    #[bit(0, rw)]
    tris0: bool,

    #[bit(1, rw)]
    tris1: bool,

    #[bit(6, rw)]
    xstbyen: bool,

    #[bit(8, rw)]
    lat0: bool,

    #[bit(9, rw)]
    lat1: bool,

    #[bit(16, rw)]
    gpio0: bool,

    #[bit(17, rw)]
    gpio1: bool,

    #[bit(24, rw)]
    pm0: bool,

    #[bit(25, rw)]
    pm1: bool,

    #[bit(28, rw)]
    txcanod: bool,

    #[bit(29, rw)]
    sof: bool,

    #[bit(30, rw)]
    intod: bool,
}