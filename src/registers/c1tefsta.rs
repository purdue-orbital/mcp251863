use bitbybit::bitfield;

use crate::impl_register;

impl_register!(C1TEFSTA, TransmitEventFIFOStatus, 0x0044, 4, u32);

#[bitfield(u32, default = 0x00_00_00_00)]
pub struct C1TEFSTA {
    #[bits(0..=0, r)]
    tefneif: bool,

    #[bits(1..=1, r)]
    tefhif: bool,

    #[bits(2..=2, r)]
    teffif: bool,

    #[bits(3..=3)]
    tefovif: bool,
}