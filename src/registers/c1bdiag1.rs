use bitbybit::bitfield;

use crate::impl_register;

impl_register!(C1BDIAG1, BusDiagnostic1, 0x003C, 4, u32);

#[bitfield(u32, default = 0)]
pub struct C1BDIAG1 {
    #[bits(0..=15, rw)]
    efmsgcnt: u16,

    #[bits(16..=16, rw)]
    nbit0err: bool,

    #[bits(17..=17, rw)]
    nbit1err: bool,

    #[bits(18..=18, rw)]
    nackerr: bool,

    #[bits(19..=19, rw)]
    nformerr: bool,

    #[bits(20..=20, rw)]
    nstuferr: bool,

    #[bits(21..=21, rw)]
    ncrcerr: bool,

    #[bits(23..=23, rw)]
    txboerr: bool,

    #[bits(24..=24, rw)]
    dbit0err: bool,

    #[bits(25..=25, rw)]
    dbit1err: bool,

    #[bits(27..=27, rw)]
    dformerr: bool,

    #[bits(28..=28, rw)]
    dstuferr: bool,

    #[bits(29..=29, rw)]
    dcrcerr: bool,

    #[bits(30..=30, rw)]
    esi: bool,

    #[bits(32..=32, rw)]
    dlcmm: bool,
}