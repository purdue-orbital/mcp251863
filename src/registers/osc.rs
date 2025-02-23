 use bitbybit::bitfield;
 use arbitrary_int::u2;
 use crate::impl_register;

 impl_register!(OSC, OscillatorControl, 0xE00, 4, u32);

 #[bitfield(u32, default = 0x00_00_00_60)]
 pub struct OSC {
    #[bit(0, rw)]
    pllen: bool,

    #[bit(2, w)]
    oscdis: bool,

    #[bit(3, rw)]
    lpmen: bool,

    #[bit(4, rw)]
    sclkdiv: bool,

    #[bits(5..=6, rw)]
    clkodiv: u2,

    #[bit(8, r)]
    pllrdy: bool,

    #[bit(10, r)]
    oscrdy: bool,

    #[bit(12, r)]
    sclkrdy: bool,
 }