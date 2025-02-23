use arbitrary_int::u5;
use bitbybit::bitfield;
use crate::impl_register;

impl_register!(C1FIFOSTA1,  FIFOStatus1,  0x60,  4, u32);
impl_register!(C1FIFOSTA2,  FIFOStatus2,  0x6C,  4, u32);
impl_register!(C1FIFOSTA3,  FIFOStatus3,  0x78,  4, u32);
impl_register!(C1FIFOSTA4,  FIFOStatus4,  0x84,  4, u32);
impl_register!(C1FIFOSTA5,  FIFOStatus5,  0x90,  4, u32);
impl_register!(C1FIFOSTA6,  FIFOStatus6,  0x9C,  4, u32);
impl_register!(C1FIFOSTA7,  FIFOStatus7,  0xA8,  4, u32);
impl_register!(C1FIFOSTA8,  FIFOStatus8,  0xB4,  4, u32);
impl_register!(C1FIFOSTA9,  FIFOStatus9,  0xC0,  4, u32);
impl_register!(C1FIFOSTA10, FIFOStatus10, 0xCC,  4, u32);
impl_register!(C1FIFOSTA11, FIFOStatus11, 0xD8,  4, u32);
impl_register!(C1FIFOSTA12, FIFOStatus12, 0xE4,  4, u32);
impl_register!(C1FIFOSTA13, FIFOStatus13, 0xF0,  4, u32);
impl_register!(C1FIFOSTA14, FIFOStatus14, 0xFC,  4, u32);
impl_register!(C1FIFOSTA15, FIFOStatus15, 0x108, 4, u32);
impl_register!(C1FIFOSTA16, FIFOStatus16, 0x114, 4, u32);
impl_register!(C1FIFOSTA17, FIFOStatus17, 0x120, 4, u32);
impl_register!(C1FIFOSTA18, FIFOStatus18, 0x12C, 4, u32);
impl_register!(C1FIFOSTA19, FIFOStatus19, 0x138, 4, u32);
impl_register!(C1FIFOSTA20, FIFOStatus20, 0x144, 4, u32);
impl_register!(C1FIFOSTA21, FIFOStatus21, 0x150, 4, u32);
impl_register!(C1FIFOSTA22, FIFOStatus22, 0x15C, 4, u32);
impl_register!(C1FIFOSTA23, FIFOStatus23, 0x168, 4, u32);
impl_register!(C1FIFOSTA24, FIFOStatus24, 0x174, 4, u32);
impl_register!(C1FIFOSTA25, FIFOStatus25, 0x180, 4, u32);
impl_register!(C1FIFOSTA26, FIFOStatus26, 0x18C, 4, u32);
impl_register!(C1FIFOSTA27, FIFOStatus27, 0x198, 4, u32);
impl_register!(C1FIFOSTA28, FIFOStatus28, 0x1A4, 4, u32);
impl_register!(C1FIFOSTA29, FIFOStatus29, 0x1B0, 4, u32);
impl_register!(C1FIFOSTA30, FIFOStatus30, 0x1BC, 4, u32);
impl_register!(C1FIFOSTA31, FIFOStatus31, 0x1C8, 4, u32);

#[bitfield(u32, default = 0)]
pub struct C1FIFOSTA1 {
    #[bit(0, r)]
    tfnrfnif: bool,

    #[bit(1, r)]
    tfhrfhif: bool,

    #[bit(2, r)]
    tferffif: bool,

    #[bit(3)]
    rxovif: bool,

    #[bit(4)]
    txatif: bool,

    #[bit(5)]
    txerr: bool,

    #[bit(6)]
    txlarb: bool,

    #[bit(7)]
    txabt: bool,

    #[bits(8..=12, r)]
    fifoci: u5,
}

#[bitfield(u32, default = 0)]
pub struct C1FIFOSTA2 {
    #[bit(0, r)]
    tfnrfnif: bool,

    #[bit(1, r)]
    tfhrfhif: bool,

    #[bit(2, r)]
    tferffif: bool,

    #[bit(3)]
    rxovif: bool,

    #[bit(4)]
    txatif: bool,

    #[bit(5)]
    txerr: bool,

    #[bit(6)]
    txlarb: bool,

    #[bit(7)]
    txabt: bool,

    #[bits(8..=12, r)]
    fifoci: u5,
}

#[bitfield(u32, default = 0)]
pub struct C1FIFOSTA3 {
    #[bit(0, r)]
    tfnrfnif: bool,

    #[bit(1, r)]
    tfhrfhif: bool,

    #[bit(2, r)]
    tferffif: bool,

    #[bit(3)]
    rxovif: bool,

    #[bit(4)]
    txatif: bool,

    #[bit(5)]
    txerr: bool,

    #[bit(6)]
    txlarb: bool,

    #[bit(7)]
    txabt: bool,

    #[bits(8..=12, r)]
    fifoci: u5,
}

#[bitfield(u32, default = 0)]
pub struct C1FIFOSTA4 {
    #[bit(0, r)]
    tfnrfnif: bool,

    #[bit(1, r)]
    tfhrfhif: bool,

    #[bit(2, r)]
    tferffif: bool,

    #[bit(3)]
    rxovif: bool,

    #[bit(4)]
    txatif: bool,

    #[bit(5)]
    txerr: bool,

    #[bit(6)]
    txlarb: bool,

    #[bit(7)]
    txabt: bool,

    #[bits(8..=12, r)]
    fifoci: u5,
}

#[bitfield(u32, default = 0)]
pub struct C1FIFOSTA5 {
    #[bit(0, r)]
    tfnrfnif: bool,

    #[bit(1, r)]
    tfhrfhif: bool,

    #[bit(2, r)]
    tferffif: bool,

    #[bit(3)]
    rxovif: bool,

    #[bit(4)]
    txatif: bool,

    #[bit(5)]
    txerr: bool,

    #[bit(6)]
    txlarb: bool,

    #[bit(7)]
    txabt: bool,

    #[bits(8..=12, r)]
    fifoci: u5,
}

#[bitfield(u32, default = 0)]
pub struct C1FIFOSTA6 {
    #[bit(0, r)]
    tfnrfnif: bool,

    #[bit(1, r)]
    tfhrfhif: bool,

    #[bit(2, r)]
    tferffif: bool,

    #[bit(3)]
    rxovif: bool,

    #[bit(4)]
    txatif: bool,

    #[bit(5)]
    txerr: bool,

    #[bit(6)]
    txlarb: bool,

    #[bit(7)]
    txabt: bool,

    #[bits(8..=12, r)]
    fifoci: u5,
}

#[bitfield(u32, default = 0)]
pub struct C1FIFOSTA7 {
    #[bit(0, r)]
    tfnrfnif: bool,

    #[bit(1, r)]
    tfhrfhif: bool,

    #[bit(2, r)]
    tferffif: bool,

    #[bit(3)]
    rxovif: bool,

    #[bit(4)]
    txatif: bool,

    #[bit(5)]
    txerr: bool,

    #[bit(6)]
    txlarb: bool,

    #[bit(7)]
    txabt: bool,

    #[bits(8..=12, r)]
    fifoci: u5,
}

#[bitfield(u32, default = 0)]
pub struct C1FIFOSTA8 {
    #[bit(0, r)]
    tfnrfnif: bool,

    #[bit(1, r)]
    tfhrfhif: bool,

    #[bit(2, r)]
    tferffif: bool,

    #[bit(3)]
    rxovif: bool,

    #[bit(4)]
    txatif: bool,

    #[bit(5)]
    txerr: bool,

    #[bit(6)]
    txlarb: bool,

    #[bit(7)]
    txabt: bool,

    #[bits(8..=12, r)]
    fifoci: u5,
}

#[bitfield(u32, default = 0)]
pub struct C1FIFOSTA9 {
    #[bit(0, r)]
    tfnrfnif: bool,

    #[bit(1, r)]
    tfhrfhif: bool,

    #[bit(2, r)]
    tferffif: bool,

    #[bit(3)]
    rxovif: bool,

    #[bit(4)]
    txatif: bool,

    #[bit(5)]
    txerr: bool,

    #[bit(6)]
    txlarb: bool,

    #[bit(7)]
    txabt: bool,

    #[bits(8..=12, r)]
    fifoci: u5,
}

#[bitfield(u32, default = 0)]
pub struct C1FIFOSTA10 {
    #[bit(0, r)]
    tfnrfnif: bool,

    #[bit(1, r)]
    tfhrfhif: bool,

    #[bit(2, r)]
    tferffif: bool,

    #[bit(3)]
    rxovif: bool,

    #[bit(4)]
    txatif: bool,

    #[bit(5)]
    txerr: bool,

    #[bit(6)]
    txlarb: bool,

    #[bit(7)]
    txabt: bool,

    #[bits(8..=12, r)]
    fifoci: u5,
}

#[bitfield(u32, default = 0)]
pub struct C1FIFOSTA11 {
    #[bit(0, r)]
    tfnrfnif: bool,

    #[bit(1, r)]
    tfhrfhif: bool,

    #[bit(2, r)]
    tferffif: bool,

    #[bit(3)]
    rxovif: bool,

    #[bit(4)]
    txatif: bool,

    #[bit(5)]
    txerr: bool,

    #[bit(6)]
    txlarb: bool,

    #[bit(7)]
    txabt: bool,

    #[bits(8..=12, r)]
    fifoci: u5,
}

#[bitfield(u32, default = 0)]
pub struct C1FIFOSTA12 {
    #[bit(0, r)]
    tfnrfnif: bool,

    #[bit(1, r)]
    tfhrfhif: bool,

    #[bit(2, r)]
    tferffif: bool,

    #[bit(3)]
    rxovif: bool,

    #[bit(4)]
    txatif: bool,

    #[bit(5)]
    txerr: bool,

    #[bit(6)]
    txlarb: bool,

    #[bit(7)]
    txabt: bool,

    #[bits(8..=12, r)]
    fifoci: u5,
}

#[bitfield(u32, default = 0)]
pub struct C1FIFOSTA13 {
    #[bit(0, r)]
    tfnrfnif: bool,

    #[bit(1, r)]
    tfhrfhif: bool,

    #[bit(2, r)]
    tferffif: bool,

    #[bit(3)]
    rxovif: bool,

    #[bit(4)]
    txatif: bool,

    #[bit(5)]
    txerr: bool,

    #[bit(6)]
    txlarb: bool,

    #[bit(7)]
    txabt: bool,

    #[bits(8..=12, r)]
    fifoci: u5,
}

#[bitfield(u32, default = 0)]
pub struct C1FIFOSTA14 {
    #[bit(0, r)]
    tfnrfnif: bool,

    #[bit(1, r)]
    tfhrfhif: bool,

    #[bit(2, r)]
    tferffif: bool,

    #[bit(3)]
    rxovif: bool,

    #[bit(4)]
    txatif: bool,

    #[bit(5)]
    txerr: bool,

    #[bit(6)]
    txlarb: bool,

    #[bit(7)]
    txabt: bool,

    #[bits(8..=12, r)]
    fifoci: u5,
}

#[bitfield(u32, default = 0)]
pub struct C1FIFOSTA15 {
    #[bit(0, r)]
    tfnrfnif: bool,

    #[bit(1, r)]
    tfhrfhif: bool,

    #[bit(2, r)]
    tferffif: bool,

    #[bit(3)]
    rxovif: bool,

    #[bit(4)]
    txatif: bool,

    #[bit(5)]
    txerr: bool,

    #[bit(6)]
    txlarb: bool,

    #[bit(7)]
    txabt: bool,

    #[bits(8..=12, r)]
    fifoci: u5,
}

#[bitfield(u32, default = 0)]
pub struct C1FIFOSTA16 {
    #[bit(0, r)]
    tfnrfnif: bool,

    #[bit(1, r)]
    tfhrfhif: bool,

    #[bit(2, r)]
    tferffif: bool,

    #[bit(3)]
    rxovif: bool,

    #[bit(4)]
    txatif: bool,

    #[bit(5)]
    txerr: bool,

    #[bit(6)]
    txlarb: bool,

    #[bit(7)]
    txabt: bool,

    #[bits(8..=12, r)]
    fifoci: u5,
}

#[bitfield(u32, default = 0)]
pub struct C1FIFOSTA17 {
    #[bit(0, r)]
    tfnrfnif: bool,

    #[bit(1, r)]
    tfhrfhif: bool,

    #[bit(2, r)]
    tferffif: bool,

    #[bit(3)]
    rxovif: bool,

    #[bit(4)]
    txatif: bool,

    #[bit(5)]
    txerr: bool,

    #[bit(6)]
    txlarb: bool,

    #[bit(7)]
    txabt: bool,

    #[bits(8..=12, r)]
    fifoci: u5,
}

#[bitfield(u32, default = 0)]
pub struct C1FIFOSTA18 {
    #[bit(0, r)]
    tfnrfnif: bool,

    #[bit(1, r)]
    tfhrfhif: bool,

    #[bit(2, r)]
    tferffif: bool,

    #[bit(3)]
    rxovif: bool,

    #[bit(4)]
    txatif: bool,

    #[bit(5)]
    txerr: bool,

    #[bit(6)]
    txlarb: bool,

    #[bit(7)]
    txabt: bool,

    #[bits(8..=12, r)]
    fifoci: u5,
}

#[bitfield(u32, default = 0)]
pub struct C1FIFOSTA19 {
    #[bit(0, r)]
    tfnrfnif: bool,

    #[bit(1, r)]
    tfhrfhif: bool,

    #[bit(2, r)]
    tferffif: bool,

    #[bit(3)]
    rxovif: bool,

    #[bit(4)]
    txatif: bool,

    #[bit(5)]
    txerr: bool,

    #[bit(6)]
    txlarb: bool,

    #[bit(7)]
    txabt: bool,

    #[bits(8..=12, r)]
    fifoci: u5,
}

#[bitfield(u32, default = 0)]
pub struct C1FIFOSTA20 {
    #[bit(0, r)]
    tfnrfnif: bool,

    #[bit(1, r)]
    tfhrfhif: bool,

    #[bit(2, r)]
    tferffif: bool,

    #[bit(3)]
    rxovif: bool,

    #[bit(4)]
    txatif: bool,

    #[bit(5)]
    txerr: bool,

    #[bit(6)]
    txlarb: bool,

    #[bit(7)]
    txabt: bool,

    #[bits(8..=12, r)]
    fifoci: u5,
}

#[bitfield(u32, default = 0)]
pub struct C1FIFOSTA21 {
    #[bit(0, r)]
    tfnrfnif: bool,

    #[bit(1, r)]
    tfhrfhif: bool,

    #[bit(2, r)]
    tferffif: bool,

    #[bit(3)]
    rxovif: bool,

    #[bit(4)]
    txatif: bool,

    #[bit(5)]
    txerr: bool,

    #[bit(6)]
    txlarb: bool,

    #[bit(7)]
    txabt: bool,

    #[bits(8..=12, r)]
    fifoci: u5,
}

#[bitfield(u32, default = 0)]
pub struct C1FIFOSTA22 {
    #[bit(0, r)]
    tfnrfnif: bool,

    #[bit(1, r)]
    tfhrfhif: bool,

    #[bit(2, r)]
    tferffif: bool,

    #[bit(3)]
    rxovif: bool,

    #[bit(4)]
    txatif: bool,

    #[bit(5)]
    txerr: bool,

    #[bit(6)]
    txlarb: bool,

    #[bit(7)]
    txabt: bool,

    #[bits(8..=12, r)]
    fifoci: u5,
}

#[bitfield(u32, default = 0)]
pub struct C1FIFOSTA23 {
    #[bit(0, r)]
    tfnrfnif: bool,

    #[bit(1, r)]
    tfhrfhif: bool,

    #[bit(2, r)]
    tferffif: bool,

    #[bit(3)]
    rxovif: bool,

    #[bit(4)]
    txatif: bool,

    #[bit(5)]
    txerr: bool,

    #[bit(6)]
    txlarb: bool,

    #[bit(7)]
    txabt: bool,

    #[bits(8..=12, r)]
    fifoci: u5,
}

#[bitfield(u32, default = 0)]
pub struct C1FIFOSTA24 {
    #[bit(0, r)]
    tfnrfnif: bool,

    #[bit(1, r)]
    tfhrfhif: bool,

    #[bit(2, r)]
    tferffif: bool,

    #[bit(3)]
    rxovif: bool,

    #[bit(4)]
    txatif: bool,

    #[bit(5)]
    txerr: bool,

    #[bit(6)]
    txlarb: bool,

    #[bit(7)]
    txabt: bool,

    #[bits(8..=12, r)]
    fifoci: u5,
}

#[bitfield(u32, default = 0)]
pub struct C1FIFOSTA25 {
    #[bit(0, r)]
    tfnrfnif: bool,

    #[bit(1, r)]
    tfhrfhif: bool,

    #[bit(2, r)]
    tferffif: bool,

    #[bit(3)]
    rxovif: bool,

    #[bit(4)]
    txatif: bool,

    #[bit(5)]
    txerr: bool,

    #[bit(6)]
    txlarb: bool,

    #[bit(7)]
    txabt: bool,

    #[bits(8..=12, r)]
    fifoci: u5,
}

#[bitfield(u32, default = 0)]
pub struct C1FIFOSTA26 {
    #[bit(0, r)]
    tfnrfnif: bool,

    #[bit(1, r)]
    tfhrfhif: bool,

    #[bit(2, r)]
    tferffif: bool,

    #[bit(3)]
    rxovif: bool,

    #[bit(4)]
    txatif: bool,

    #[bit(5)]
    txerr: bool,

    #[bit(6)]
    txlarb: bool,

    #[bit(7)]
    txabt: bool,

    #[bits(8..=12, r)]
    fifoci: u5,
}

#[bitfield(u32, default = 0)]
pub struct C1FIFOSTA27 {
    #[bit(0, r)]
    tfnrfnif: bool,

    #[bit(1, r)]
    tfhrfhif: bool,

    #[bit(2, r)]
    tferffif: bool,

    #[bit(3)]
    rxovif: bool,

    #[bit(4)]
    txatif: bool,

    #[bit(5)]
    txerr: bool,

    #[bit(6)]
    txlarb: bool,

    #[bit(7)]
    txabt: bool,

    #[bits(8..=12, r)]
    fifoci: u5,
}

#[bitfield(u32, default = 0)]
pub struct C1FIFOSTA28 {
    #[bit(0, r)]
    tfnrfnif: bool,

    #[bit(1, r)]
    tfhrfhif: bool,

    #[bit(2, r)]
    tferffif: bool,

    #[bit(3)]
    rxovif: bool,

    #[bit(4)]
    txatif: bool,

    #[bit(5)]
    txerr: bool,

    #[bit(6)]
    txlarb: bool,

    #[bit(7)]
    txabt: bool,

    #[bits(8..=12, r)]
    fifoci: u5,
}

#[bitfield(u32, default = 0)]
pub struct C1FIFOSTA29 {
    #[bit(0, r)]
    tfnrfnif: bool,

    #[bit(1, r)]
    tfhrfhif: bool,

    #[bit(2, r)]
    tferffif: bool,

    #[bit(3)]
    rxovif: bool,

    #[bit(4)]
    txatif: bool,

    #[bit(5)]
    txerr: bool,

    #[bit(6)]
    txlarb: bool,

    #[bit(7)]
    txabt: bool,

    #[bits(8..=12, r)]
    fifoci: u5,
}

#[bitfield(u32, default = 0)]
pub struct C1FIFOSTA30 {
    #[bit(0, r)]
    tfnrfnif: bool,

    #[bit(1, r)]
    tfhrfhif: bool,

    #[bit(2, r)]
    tferffif: bool,

    #[bit(3)]
    rxovif: bool,

    #[bit(4)]
    txatif: bool,

    #[bit(5)]
    txerr: bool,

    #[bit(6)]
    txlarb: bool,

    #[bit(7)]
    txabt: bool,

    #[bits(8..=12, r)]
    fifoci: u5,
}

#[bitfield(u32, default = 0)]
pub struct C1FIFOSTA31 {
    #[bit(0, r)]
    tfnrfnif: bool,

    #[bit(1, r)]
    tfhrfhif: bool,

    #[bit(2, r)]
    tferffif: bool,

    #[bit(3)]
    rxovif: bool,

    #[bit(4)]
    txatif: bool,

    #[bit(5)]
    txerr: bool,

    #[bit(6)]
    txlarb: bool,

    #[bit(7)]
    txabt: bool,

    #[bits(8..=12, r)]
    fifoci: u5,
}

