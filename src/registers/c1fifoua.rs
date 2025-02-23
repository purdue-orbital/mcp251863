use bitbybit::bitfield;
use crate::impl_register;

impl_register!(C1FIFOUA1,  FIFOUserAdress1,  0x64,  4, u32);
impl_register!(C1FIFOUA2,  FIFOUserAdress2,  0x70,  4, u32);
impl_register!(C1FIFOUA3,  FIFOUserAdress3,  0x7C,  4, u32);
impl_register!(C1FIFOUA4,  FIFOUserAdress4,  0x88,  4, u32);
impl_register!(C1FIFOUA5,  FIFOUserAdress5,  0x94,  4, u32);
impl_register!(C1FIFOUA6,  FIFOUserAdress6,  0xA0,  4, u32);
impl_register!(C1FIFOUA7,  FIFOUserAdress7,  0xAC,  4, u32);
impl_register!(C1FIFOUA8,  FIFOUserAdress8,  0xB8,  4, u32);
impl_register!(C1FIFOUA9,  FIFOUserAdress9,  0xC4,  4, u32);
impl_register!(C1FIFOUA10, FIFOUserAdress10, 0xD0,  4, u32);
impl_register!(C1FIFOUA11, FIFOUserAdress11, 0xDC,  4, u32);
impl_register!(C1FIFOUA12, FIFOUserAdress12, 0xE8,  4, u32);
impl_register!(C1FIFOUA13, FIFOUserAdress13, 0xF4,  4, u32);
impl_register!(C1FIFOUA14, FIFOUserAdress14, 0x100, 4, u32);
impl_register!(C1FIFOUA15, FIFOUserAdress15, 0x10C, 4, u32);
impl_register!(C1FIFOUA16, FIFOUserAdress16, 0x118, 4, u32);
impl_register!(C1FIFOUA17, FIFOUserAdress17, 0x124, 4, u32);
impl_register!(C1FIFOUA18, FIFOUserAdress18, 0x130, 4, u32);
impl_register!(C1FIFOUA19, FIFOUserAdress19, 0x13C, 4, u32);
impl_register!(C1FIFOUA20, FIFOUserAdress20, 0x148, 4, u32);
impl_register!(C1FIFOUA21, FIFOUserAdress21, 0x154, 4, u32);
impl_register!(C1FIFOUA22, FIFOUserAdress22, 0x160, 4, u32);
impl_register!(C1FIFOUA23, FIFOUserAdress23, 0x16C, 4, u32);
impl_register!(C1FIFOUA24, FIFOUserAdress24, 0x178, 4, u32);
impl_register!(C1FIFOUA25, FIFOUserAdress25, 0x184, 4, u32);
impl_register!(C1FIFOUA26, FIFOUserAdress26, 0x190, 4, u32);
impl_register!(C1FIFOUA27, FIFOUserAdress27, 0x19C, 4, u32);
impl_register!(C1FIFOUA28, FIFOUserAdress28, 0x1A8, 4, u32);
impl_register!(C1FIFOUA29, FIFOUserAdress29, 0x1B4, 4, u32);
impl_register!(C1FIFOUA30, FIFOUserAdress30, 0x1C0, 4, u32);
impl_register!(C1FIFOUA31, FIFOUserAdress31, 0x1CC, 4, u32);

#[bitfield(u32)]
pub struct C1FIFOUA1 {
    #[bits(0..=31, r)]
    fifoua: u32,
}

#[bitfield(u32)]
pub struct C1FIFOUA2 {
    #[bits(0..=31, r)]
    fifoua: u32,
}

#[bitfield(u32)]
pub struct C1FIFOUA3 {
    #[bits(0..=31, r)]
    fifoua: u32,
}

#[bitfield(u32)]
pub struct C1FIFOUA4 {
    #[bits(0..=31, r)]
    fifoua: u32,
}

#[bitfield(u32)]
pub struct C1FIFOUA5 {
    #[bits(0..=31, r)]
    fifoua: u32,
}

#[bitfield(u32)]
pub struct C1FIFOUA6 {
    #[bits(0..=31, r)]
    fifoua: u32,
}

#[bitfield(u32)]
pub struct C1FIFOUA7 {
    #[bits(0..=31, r)]
    fifoua: u32,
}

#[bitfield(u32)]
pub struct C1FIFOUA8 {
    #[bits(0..=31, r)]
    fifoua: u32,
}

#[bitfield(u32)]
pub struct C1FIFOUA9 {
    #[bits(0..=31, r)]
    fifoua: u32,
}

#[bitfield(u32)]
pub struct C1FIFOUA10 {
    #[bits(0..=31, r)]
    fifoua: u32,
}

#[bitfield(u32)]
pub struct C1FIFOUA11 {
    #[bits(0..=31, r)]
    fifoua: u32,
}

#[bitfield(u32)]
pub struct C1FIFOUA12 {
    #[bits(0..=31, r)]
    fifoua: u32,
}

#[bitfield(u32)]
pub struct C1FIFOUA13 {
    #[bits(0..=31, r)]
    fifoua: u32,
}

#[bitfield(u32)]
pub struct C1FIFOUA14 {
    #[bits(0..=31, r)]
    fifoua: u32,
}

#[bitfield(u32)]
pub struct C1FIFOUA15 {
    #[bits(0..=31, r)]
    fifoua: u32,
}

#[bitfield(u32)]
pub struct C1FIFOUA16 {
    #[bits(0..=31, r)]
    fifoua: u32,
}

#[bitfield(u32)]
pub struct C1FIFOUA17 {
    #[bits(0..=31, r)]
    fifoua: u32,
}

#[bitfield(u32)]
pub struct C1FIFOUA18 {
    #[bits(0..=31, r)]
    fifoua: u32,
}

#[bitfield(u32)]
pub struct C1FIFOUA19 {
    #[bits(0..=31, r)]
    fifoua: u32,
}

#[bitfield(u32)]
pub struct C1FIFOUA20 {
    #[bits(0..=31, r)]
    fifoua: u32,
}

#[bitfield(u32)]
pub struct C1FIFOUA21 {
    #[bits(0..=31, r)]
    fifoua: u32,
}

#[bitfield(u32)]
pub struct C1FIFOUA22 {
    #[bits(0..=31, r)]
    fifoua: u32,
}

#[bitfield(u32)]
pub struct C1FIFOUA23 {
    #[bits(0..=31, r)]
    fifoua: u32,
}

#[bitfield(u32)]
pub struct C1FIFOUA24 {
    #[bits(0..=31, r)]
    fifoua: u32,
}

#[bitfield(u32)]
pub struct C1FIFOUA25 {
    #[bits(0..=31, r)]
    fifoua: u32,
}

#[bitfield(u32)]
pub struct C1FIFOUA26 {
    #[bits(0..=31, r)]
    fifoua: u32,
}

#[bitfield(u32)]
pub struct C1FIFOUA27 {
    #[bits(0..=31, r)]
    fifoua: u32,
}

#[bitfield(u32)]
pub struct C1FIFOUA28 {
    #[bits(0..=31, r)]
    fifoua: u32,
}

#[bitfield(u32)]
pub struct C1FIFOUA29 {
    #[bits(0..=31, r)]
    fifoua: u32,
}

#[bitfield(u32)]
pub struct C1FIFOUA30 {
    #[bits(0..=31, r)]
    fifoua: u32,
}

#[bitfield(u32)]
pub struct C1FIFOUA31 {
    #[bits(0..=31, r)]
    fifoua: u32,
}
