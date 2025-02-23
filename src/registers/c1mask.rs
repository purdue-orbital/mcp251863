use bitbybit::bitfield;
use crate::impl_register;
use arbitrary_int::{u11, u18};

impl_register!(C1MASK0,  Mask0,  0x1F4, 4, u32);
impl_register!(C1MASK1,  Mask1,  0x1FC, 4, u32);
impl_register!(C1MASK2,  Mask2,  0x204, 4, u32);
impl_register!(C1MASK3,  Mask3,  0x20C, 4, u32);
impl_register!(C1MASK4,  Mask4,  0x214, 4, u32);
impl_register!(C1MASK5,  Mask5,  0x21C, 4, u32);
impl_register!(C1MASK6,  Mask6,  0x224, 4, u32);
impl_register!(C1MASK7,  Mask7,  0x22C, 4, u32);
impl_register!(C1MASK8,  Mask8,  0x234, 4, u32);
impl_register!(C1MASK9,  Mask9,  0x23C, 4, u32);
impl_register!(C1MASK10, Mask10, 0x244, 4, u32);
impl_register!(C1MASK11, Mask11, 0x24C, 4, u32);
impl_register!(C1MASK12, Mask12, 0x254, 4, u32);
impl_register!(C1MASK13, Mask13, 0x25C, 4, u32);
impl_register!(C1MASK14, Mask14, 0x264, 4, u32);
impl_register!(C1MASK15, Mask15, 0x26C, 4, u32);
impl_register!(C1MASK16, Mask16, 0x274, 4, u32);
impl_register!(C1MASK17, Mask17, 0x27C, 4, u32);
impl_register!(C1MASK18, Mask18, 0x284, 4, u32);
impl_register!(C1MASK19, Mask19, 0x28C, 4, u32);
impl_register!(C1MASK20, Mask20, 0x294, 4, u32);
impl_register!(C1MASK21, Mask21, 0x29C, 4, u32);
impl_register!(C1MASK22, Mask22, 0x2A4, 4, u32);
impl_register!(C1MASK23, Mask23, 0x2AC, 4, u32);
impl_register!(C1MASK24, Mask24, 0x2B4, 4, u32);
impl_register!(C1MASK25, Mask25, 0x2BC, 4, u32);
impl_register!(C1MASK26, Mask26, 0x2C4, 4, u32);
impl_register!(C1MASK27, Mask27, 0x2CC, 4, u32);
impl_register!(C1MASK28, Mask28, 0x2D4, 4, u32);
impl_register!(C1MASK29, Mask29, 0x2DC, 4, u32);
impl_register!(C1MASK30, Mask30, 0x2E4, 4, u32);
impl_register!(C1MASK31, Mask31, 0x2EC, 4, u32);

#[bitfield(u32, default = 0)]
pub struct C1MASK0 {
    #[bits(0..=10, rw)]
    msid: u11,

    #[bits(11..=28, rw)]
    meid: u18,

    #[bit(29, rw)]
    msid11: bool,

    #[bit(30, rw)]
    mide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1MASK1 {
    #[bits(0..=10, rw)]
    msid: u11,

    #[bits(11..=28, rw)]
    meid: u18,

    #[bit(29, rw)]
    msid11: bool,

    #[bit(30, rw)]
    mide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1MASK2 {
    #[bits(0..=10, rw)]
    msid: u11,

    #[bits(11..=28, rw)]
    meid: u18,

    #[bit(29, rw)]
    msid11: bool,

    #[bit(30, rw)]
    mide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1MASK3 {
    #[bits(0..=10, rw)]
    msid: u11,

    #[bits(11..=28, rw)]
    meid: u18,

    #[bit(29, rw)]
    msid11: bool,

    #[bit(30, rw)]
    mide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1MASK4 {
    #[bits(0..=10, rw)]
    msid: u11,

    #[bits(11..=28, rw)]
    meid: u18,

    #[bit(29, rw)]
    msid11: bool,

    #[bit(30, rw)]
    mide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1MASK5 {
    #[bits(0..=10, rw)]
    msid: u11,

    #[bits(11..=28, rw)]
    meid: u18,

    #[bit(29, rw)]
    msid11: bool,

    #[bit(30, rw)]
    mide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1MASK6 {
    #[bits(0..=10, rw)]
    msid: u11,

    #[bits(11..=28, rw)]
    meid: u18,

    #[bit(29, rw)]
    msid11: bool,

    #[bit(30, rw)]
    mide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1MASK7 {
    #[bits(0..=10, rw)]
    msid: u11,

    #[bits(11..=28, rw)]
    meid: u18,

    #[bit(29, rw)]
    msid11: bool,

    #[bit(30, rw)]
    mide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1MASK8 {
    #[bits(0..=10, rw)]
    msid: u11,

    #[bits(11..=28, rw)]
    meid: u18,

    #[bit(29, rw)]
    msid11: bool,

    #[bit(30, rw)]
    mide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1MASK9 {
    #[bits(0..=10, rw)]
    msid: u11,

    #[bits(11..=28, rw)]
    meid: u18,

    #[bit(29, rw)]
    msid11: bool,

    #[bit(30, rw)]
    mide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1MASK10 {
    #[bits(0..=10, rw)]
    msid: u11,

    #[bits(11..=28, rw)]
    meid: u18,

    #[bit(29, rw)]
    msid11: bool,

    #[bit(30, rw)]
    mide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1MASK11 {
    #[bits(0..=10, rw)]
    msid: u11,

    #[bits(11..=28, rw)]
    meid: u18,

    #[bit(29, rw)]
    msid11: bool,

    #[bit(30, rw)]
    mide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1MASK12 {
    #[bits(0..=10, rw)]
    msid: u11,

    #[bits(11..=28, rw)]
    meid: u18,

    #[bit(29, rw)]
    msid11: bool,

    #[bit(30, rw)]
    mide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1MASK13 {
    #[bits(0..=10, rw)]
    msid: u11,

    #[bits(11..=28, rw)]
    meid: u18,

    #[bit(29, rw)]
    msid11: bool,

    #[bit(30, rw)]
    mide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1MASK14 {
    #[bits(0..=10, rw)]
    msid: u11,

    #[bits(11..=28, rw)]
    meid: u18,

    #[bit(29, rw)]
    msid11: bool,

    #[bit(30, rw)]
    mide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1MASK15 {
    #[bits(0..=10, rw)]
    msid: u11,

    #[bits(11..=28, rw)]
    meid: u18,

    #[bit(29, rw)]
    msid11: bool,

    #[bit(30, rw)]
    mide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1MASK16 {
    #[bits(0..=10, rw)]
    msid: u11,

    #[bits(11..=28, rw)]
    meid: u18,

    #[bit(29, rw)]
    msid11: bool,

    #[bit(30, rw)]
    mide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1MASK17 {
    #[bits(0..=10, rw)]
    msid: u11,

    #[bits(11..=28, rw)]
    meid: u18,

    #[bit(29, rw)]
    msid11: bool,

    #[bit(30, rw)]
    mide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1MASK18 {
    #[bits(0..=10, rw)]
    msid: u11,

    #[bits(11..=28, rw)]
    meid: u18,

    #[bit(29, rw)]
    msid11: bool,

    #[bit(30, rw)]
    mide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1MASK19 {
    #[bits(0..=10, rw)]
    msid: u11,

    #[bits(11..=28, rw)]
    meid: u18,

    #[bit(29, rw)]
    msid11: bool,

    #[bit(30, rw)]
    mide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1MASK20 {
    #[bits(0..=10, rw)]
    msid: u11,

    #[bits(11..=28, rw)]
    meid: u18,

    #[bit(29, rw)]
    msid11: bool,

    #[bit(30, rw)]
    mide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1MASK21 {
    #[bits(0..=10, rw)]
    msid: u11,

    #[bits(11..=28, rw)]
    meid: u18,

    #[bit(29, rw)]
    msid11: bool,

    #[bit(30, rw)]
    mide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1MASK22 {
    #[bits(0..=10, rw)]
    msid: u11,

    #[bits(11..=28, rw)]
    meid: u18,

    #[bit(29, rw)]
    msid11: bool,

    #[bit(30, rw)]
    mide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1MASK23 {
    #[bits(0..=10, rw)]
    msid: u11,

    #[bits(11..=28, rw)]
    meid: u18,

    #[bit(29, rw)]
    msid11: bool,

    #[bit(30, rw)]
    mide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1MASK24 {
    #[bits(0..=10, rw)]
    msid: u11,

    #[bits(11..=28, rw)]
    meid: u18,

    #[bit(29, rw)]
    msid11: bool,

    #[bit(30, rw)]
    mide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1MASK25 {
    #[bits(0..=10, rw)]
    msid: u11,

    #[bits(11..=28, rw)]
    meid: u18,

    #[bit(29, rw)]
    msid11: bool,

    #[bit(30, rw)]
    mide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1MASK26 {
    #[bits(0..=10, rw)]
    msid: u11,

    #[bits(11..=28, rw)]
    meid: u18,

    #[bit(29, rw)]
    msid11: bool,

    #[bit(30, rw)]
    mide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1MASK27 {
    #[bits(0..=10, rw)]
    msid: u11,

    #[bits(11..=28, rw)]
    meid: u18,

    #[bit(29, rw)]
    msid11: bool,

    #[bit(30, rw)]
    mide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1MASK28 {
    #[bits(0..=10, rw)]
    msid: u11,

    #[bits(11..=28, rw)]
    meid: u18,

    #[bit(29, rw)]
    msid11: bool,

    #[bit(30, rw)]
    mide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1MASK29 {
    #[bits(0..=10, rw)]
    msid: u11,

    #[bits(11..=28, rw)]
    meid: u18,

    #[bit(29, rw)]
    msid11: bool,

    #[bit(30, rw)]
    mide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1MASK30 {
    #[bits(0..=10, rw)]
    msid: u11,

    #[bits(11..=28, rw)]
    meid: u18,

    #[bit(29, rw)]
    msid11: bool,

    #[bit(30, rw)]
    mide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1MASK31 {
    #[bits(0..=10, rw)]
    msid: u11,

    #[bits(11..=28, rw)]
    meid: u18,

    #[bit(29, rw)]
    msid11: bool,

    #[bit(30, rw)]
    mide: bool,
}