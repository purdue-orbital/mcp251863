use bitbybit::bitfield;
use crate::impl_register;
use arbitrary_int::u5;

impl_register!(C1FLTCON0, FilterControl0, 0x1D0, 4, u32);
impl_register!(C1FLTCON1, FilterControl1, 0x1D4, 4, u32);
impl_register!(C1FLTCON2, FilterControl2, 0x1D8, 4, u32);
impl_register!(C1FLTCON3, FilterControl3, 0x1DC, 4, u32);
impl_register!(C1FLTCON4, FilterControl4, 0x1E0, 4, u32);
impl_register!(C1FLTCON5, FilterControl5, 0x1E4, 4, u32);
impl_register!(C1FLTCON6, FilterControl6, 0x1E8, 4, u32);
impl_register!(C1FLTCON7, FilterControl7, 0x1EC, 4, u32);

#[bitfield(u32, default = 0)]
pub struct C1FLTCON0 {
    #[bits(0..=4, rw)]
    f0bp: u5,

    #[bit(7, rw)]
    flten0: bool,

    #[bits(8..=12, rw)]
    f1bp: u5,

    #[bit(15, rw)]
    flten1: bool,

    #[bits(16..=20, rw)]
    f2bp: u5,

    #[bit(23, rw)]
    flten2: bool,

    #[bits(24..=28, rw)]
    f3bp: u5,

    #[bit(31, rw)]
    flten3: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1FLTCON1 {
    #[bits(0..=4, rw)]
    f0bp: u5,

    #[bit(7, rw)]
    flten0: bool,

    #[bits(8..=12, rw)]
    f1bp: u5,

    #[bit(15, rw)]
    flten1: bool,

    #[bits(16..=20, rw)]
    f2bp: u5,

    #[bit(23, rw)]
    flten2: bool,

    #[bits(24..=28, rw)]
    f3bp: u5,

    #[bit(31, rw)]
    flten3: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1FLTCON2 {
    #[bits(0..=4, rw)]
    f0bp: u5,

    #[bit(7, rw)]
    flten0: bool,

    #[bits(8..=12, rw)]
    f1bp: u5,

    #[bit(15, rw)]
    flten1: bool,

    #[bits(16..=20, rw)]
    f2bp: u5,

    #[bit(23, rw)]
    flten2: bool,

    #[bits(24..=28, rw)]
    f3bp: u5,

    #[bit(31, rw)]
    flten3: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1FLTCON3 {
    #[bits(0..=4, rw)]
    f0bp: u5,

    #[bit(7, rw)]
    flten0: bool,

    #[bits(8..=12, rw)]
    f1bp: u5,

    #[bit(15, rw)]
    flten1: bool,

    #[bits(16..=20, rw)]
    f2bp: u5,

    #[bit(23, rw)]
    flten2: bool,

    #[bits(24..=28, rw)]
    f3bp: u5,

    #[bit(31, rw)]
    flten3: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1FLTCON4 {
    #[bits(0..=4, rw)]
    f0bp: u5,

    #[bit(7, rw)]
    flten0: bool,

    #[bits(8..=12, rw)]
    f1bp: u5,

    #[bit(15, rw)]
    flten1: bool,

    #[bits(16..=20, rw)]
    f2bp: u5,

    #[bit(23, rw)]
    flten2: bool,

    #[bits(24..=28, rw)]
    f3bp: u5,

    #[bit(31, rw)]
    flten3: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1FLTCON5 {
    #[bits(0..=4, rw)]
    f0bp: u5,

    #[bit(7, rw)]
    flten0: bool,

    #[bits(8..=12, rw)]
    f1bp: u5,

    #[bit(15, rw)]
    flten1: bool,

    #[bits(16..=20, rw)]
    f2bp: u5,

    #[bit(23, rw)]
    flten2: bool,

    #[bits(24..=28, rw)]
    f3bp: u5,

    #[bit(31, rw)]
    flten3: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1FLTCON6 {
    #[bits(0..=4, rw)]
    f0bp: u5,

    #[bit(7, rw)]
    flten0: bool,

    #[bits(8..=12, rw)]
    f1bp: u5,

    #[bit(15, rw)]
    flten1: bool,

    #[bits(16..=20, rw)]
    f2bp: u5,

    #[bit(23, rw)]
    flten2: bool,

    #[bits(24..=28, rw)]
    f3bp: u5,

    #[bit(31, rw)]
    flten3: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1FLTCON7 {
    #[bits(0..=4, rw)]
    f0bp: u5,

    #[bit(7, rw)]
    flten0: bool,

    #[bits(8..=12, rw)]
    f1bp: u5,

    #[bit(15, rw)]
    flten1: bool,

    #[bits(16..=20, rw)]
    f2bp: u5,

    #[bit(23, rw)]
    flten2: bool,

    #[bits(24..=28, rw)]
    f3bp: u5,

    #[bit(31, rw)]
    flten3: bool,
}

