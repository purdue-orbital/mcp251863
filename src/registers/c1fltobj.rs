use bitbybit::bitfield;
use crate::impl_register;
use arbitrary_int::{u11, u18};

impl_register!(C1FLTOBJ0,  FilterObject0,  0x1F0, 4, u32);
impl_register!(C1FLTOBJ1,  FilterObject1,  0x1F8, 4, u32);
impl_register!(C1FLTOBJ2,  FilterObject2,  0x200, 4, u32);
impl_register!(C1FLTOBJ3,  FilterObject3,  0x208, 4, u32);
impl_register!(C1FLTOBJ4,  FilterObject4,  0x210, 4, u32);
impl_register!(C1FLTOBJ5,  FilterObject5,  0x218, 4, u32);
impl_register!(C1FLTOBJ6,  FilterObject6,  0x220, 4, u32);
impl_register!(C1FLTOBJ7,  FilterObject7,  0x228, 4, u32);
impl_register!(C1FLTOBJ8,  FilterObject8,  0x230, 4, u32);
impl_register!(C1FLTOBJ9,  FilterObject9,  0x238, 4, u32);
impl_register!(C1FLTOBJ10, FilterObject10, 0x240, 4, u32);
impl_register!(C1FLTOBJ11, FilterObject11, 0x248, 4, u32);
impl_register!(C1FLTOBJ12, FilterObject12, 0x250, 4, u32);
impl_register!(C1FLTOBJ13, FilterObject13, 0x258, 4, u32);
impl_register!(C1FLTOBJ14, FilterObject14, 0x260, 4, u32);
impl_register!(C1FLTOBJ15, FilterObject15, 0x268, 4, u32);
impl_register!(C1FLTOBJ16, FilterObject16, 0x270, 4, u32);
impl_register!(C1FLTOBJ17, FilterObject17, 0x278, 4, u32);
impl_register!(C1FLTOBJ18, FilterObject18, 0x280, 4, u32);
impl_register!(C1FLTOBJ19, FilterObject19, 0x288, 4, u32);
impl_register!(C1FLTOBJ20, FilterObject20, 0x290, 4, u32);
impl_register!(C1FLTOBJ21, FilterObject21, 0x298, 4, u32);
impl_register!(C1FLTOBJ22, FilterObject22, 0x2A0, 4, u32);
impl_register!(C1FLTOBJ23, FilterObject23, 0x2A8, 4, u32);
impl_register!(C1FLTOBJ24, FilterObject24, 0x2B0, 4, u32);
impl_register!(C1FLTOBJ25, FilterObject25, 0x2B8, 4, u32);
impl_register!(C1FLTOBJ26, FilterObject26, 0x2C0, 4, u32);
impl_register!(C1FLTOBJ27, FilterObject27, 0x2C8, 4, u32);
impl_register!(C1FLTOBJ28, FilterObject28, 0x2D0, 4, u32);
impl_register!(C1FLTOBJ29, FilterObject29, 0x2D8, 4, u32);
impl_register!(C1FLTOBJ30, FilterObject30, 0x2E0, 4, u32);
impl_register!(C1FLTOBJ31, FilterObject31, 0x2E8, 4, u32);

#[bitfield(u32, default = 0)]
pub struct C1FLTOBJ0 {
    #[bits(0..=10, rw)]
    sid: u11,

    #[bits(11..=28, rw)]
    eid: u18,

    #[bit(29, rw)]
    sid11: bool,

    #[bit(30, rw)]
    exide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1FLTOBJ1 {
    #[bits(0..=10, rw)]
    sid: u11,

    #[bits(11..=28, rw)]
    eid: u18,

    #[bit(29, rw)]
    sid11: bool,

    #[bit(30, rw)]
    exide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1FLTOBJ2 {
    #[bits(0..=10, rw)]
    sid: u11,

    #[bits(11..=28, rw)]
    eid: u18,

    #[bit(29, rw)]
    sid11: bool,

    #[bit(30, rw)]
    exide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1FLTOBJ3 {
    #[bits(0..=10, rw)]
    sid: u11,

    #[bits(11..=28, rw)]
    eid: u18,

    #[bit(29, rw)]
    sid11: bool,

    #[bit(30, rw)]
    exide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1FLTOBJ4 {
    #[bits(0..=10, rw)]
    sid: u11,

    #[bits(11..=28, rw)]
    eid: u18,

    #[bit(29, rw)]
    sid11: bool,

    #[bit(30, rw)]
    exide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1FLTOBJ5 {
    #[bits(0..=10, rw)]
    sid: u11,

    #[bits(11..=28, rw)]
    eid: u18,

    #[bit(29, rw)]
    sid11: bool,

    #[bit(30, rw)]
    exide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1FLTOBJ6 {
    #[bits(0..=10, rw)]
    sid: u11,

    #[bits(11..=28, rw)]
    eid: u18,

    #[bit(29, rw)]
    sid11: bool,

    #[bit(30, rw)]
    exide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1FLTOBJ7 {
    #[bits(0..=10, rw)]
    sid: u11,

    #[bits(11..=28, rw)]
    eid: u18,

    #[bit(29, rw)]
    sid11: bool,

    #[bit(30, rw)]
    exide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1FLTOBJ8 {
    #[bits(0..=10, rw)]
    sid: u11,

    #[bits(11..=28, rw)]
    eid: u18,

    #[bit(29, rw)]
    sid11: bool,

    #[bit(30, rw)]
    exide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1FLTOBJ9 {
    #[bits(0..=10, rw)]
    sid: u11,

    #[bits(11..=28, rw)]
    eid: u18,

    #[bit(29, rw)]
    sid11: bool,

    #[bit(30, rw)]
    exide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1FLTOBJ10 {
    #[bits(0..=10, rw)]
    sid: u11,

    #[bits(11..=28, rw)]
    eid: u18,

    #[bit(29, rw)]
    sid11: bool,

    #[bit(30, rw)]
    exide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1FLTOBJ11 {
    #[bits(0..=10, rw)]
    sid: u11,

    #[bits(11..=28, rw)]
    eid: u18,

    #[bit(29, rw)]
    sid11: bool,

    #[bit(30, rw)]
    exide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1FLTOBJ12 {
    #[bits(0..=10, rw)]
    sid: u11,

    #[bits(11..=28, rw)]
    eid: u18,

    #[bit(29, rw)]
    sid11: bool,

    #[bit(30, rw)]
    exide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1FLTOBJ13 {
    #[bits(0..=10, rw)]
    sid: u11,

    #[bits(11..=28, rw)]
    eid: u18,

    #[bit(29, rw)]
    sid11: bool,

    #[bit(30, rw)]
    exide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1FLTOBJ14 {
    #[bits(0..=10, rw)]
    sid: u11,

    #[bits(11..=28, rw)]
    eid: u18,

    #[bit(29, rw)]
    sid11: bool,

    #[bit(30, rw)]
    exide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1FLTOBJ15 {
    #[bits(0..=10, rw)]
    sid: u11,

    #[bits(11..=28, rw)]
    eid: u18,

    #[bit(29, rw)]
    sid11: bool,

    #[bit(30, rw)]
    exide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1FLTOBJ16 {
    #[bits(0..=10, rw)]
    sid: u11,

    #[bits(11..=28, rw)]
    eid: u18,

    #[bit(29, rw)]
    sid11: bool,

    #[bit(30, rw)]
    exide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1FLTOBJ17 {
    #[bits(0..=10, rw)]
    sid: u11,

    #[bits(11..=28, rw)]
    eid: u18,

    #[bit(29, rw)]
    sid11: bool,

    #[bit(30, rw)]
    exide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1FLTOBJ18 {
    #[bits(0..=10, rw)]
    sid: u11,

    #[bits(11..=28, rw)]
    eid: u18,

    #[bit(29, rw)]
    sid11: bool,

    #[bit(30, rw)]
    exide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1FLTOBJ19 {
    #[bits(0..=10, rw)]
    sid: u11,

    #[bits(11..=28, rw)]
    eid: u18,

    #[bit(29, rw)]
    sid11: bool,

    #[bit(30, rw)]
    exide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1FLTOBJ20 {
    #[bits(0..=10, rw)]
    sid: u11,

    #[bits(11..=28, rw)]
    eid: u18,

    #[bit(29, rw)]
    sid11: bool,

    #[bit(30, rw)]
    exide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1FLTOBJ21 {
    #[bits(0..=10, rw)]
    sid: u11,

    #[bits(11..=28, rw)]
    eid: u18,

    #[bit(29, rw)]
    sid11: bool,

    #[bit(30, rw)]
    exide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1FLTOBJ22 {
    #[bits(0..=10, rw)]
    sid: u11,

    #[bits(11..=28, rw)]
    eid: u18,

    #[bit(29, rw)]
    sid11: bool,

    #[bit(30, rw)]
    exide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1FLTOBJ23 {
    #[bits(0..=10, rw)]
    sid: u11,

    #[bits(11..=28, rw)]
    eid: u18,

    #[bit(29, rw)]
    sid11: bool,

    #[bit(30, rw)]
    exide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1FLTOBJ24 {
    #[bits(0..=10, rw)]
    sid: u11,

    #[bits(11..=28, rw)]
    eid: u18,

    #[bit(29, rw)]
    sid11: bool,

    #[bit(30, rw)]
    exide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1FLTOBJ25 {
    #[bits(0..=10, rw)]
    sid: u11,

    #[bits(11..=28, rw)]
    eid: u18,

    #[bit(29, rw)]
    sid11: bool,

    #[bit(30, rw)]
    exide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1FLTOBJ26 {
    #[bits(0..=10, rw)]
    sid: u11,

    #[bits(11..=28, rw)]
    eid: u18,

    #[bit(29, rw)]
    sid11: bool,

    #[bit(30, rw)]
    exide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1FLTOBJ27 {
    #[bits(0..=10, rw)]
    sid: u11,

    #[bits(11..=28, rw)]
    eid: u18,

    #[bit(29, rw)]
    sid11: bool,

    #[bit(30, rw)]
    exide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1FLTOBJ28 {
    #[bits(0..=10, rw)]
    sid: u11,

    #[bits(11..=28, rw)]
    eid: u18,

    #[bit(29, rw)]
    sid11: bool,

    #[bit(30, rw)]
    exide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1FLTOBJ29 {
    #[bits(0..=10, rw)]
    sid: u11,

    #[bits(11..=28, rw)]
    eid: u18,

    #[bit(29, rw)]
    sid11: bool,

    #[bit(30, rw)]
    exide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1FLTOBJ30 {
    #[bits(0..=10, rw)]
    sid: u11,

    #[bits(11..=28, rw)]
    eid: u18,

    #[bit(29, rw)]
    sid11: bool,

    #[bit(30, rw)]
    exide: bool,
}

#[bitfield(u32, default = 0)]
pub struct C1FLTOBJ31 {
    #[bits(0..=10, rw)]
    sid: u11,

    #[bits(11..=28, rw)]
    eid: u18,

    #[bit(29, rw)]
    sid11: bool,

    #[bit(30, rw)]
    exide: bool,
}

