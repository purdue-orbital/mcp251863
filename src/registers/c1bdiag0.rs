use bitbybit::bitfield;

use crate::impl_register;

impl_register!(C1BDIAG0, BusDiagnostic0, 0x0038, 4, u32);

#[bitfield(u32, default = 0)]
pub struct C1BDIAG0 {
    #[bits(0..=7, rw)]
    nrerrcnt: u8,

    #[bits(8..=15, rw)]
    nterrcnt: u8,

    #[bits(16..=23, rw)]
    drerrcnt: u8,

    #[bits(24..=31, rw)]
    dterrcnt: u8,
}