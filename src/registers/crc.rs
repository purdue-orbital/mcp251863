use bitbybit::bitfield;
use crate::impl_register;

impl_register!(CRC, CycleRedundancyCheck, 0xE08, 4, u32);

#[bitfield(u32, default = 0x00_00_00_00)]
pub struct CRC {
    #[bits(0..=15, r)]
    crc: u16,

    #[bit(16, w)]
    crcerrif: bool,

    #[bit(17, w)]
    ferrif: bool,

    #[bit(24, rw)]
    crcerrie: bool,

    #[bit(25, rw)]
    ferrie: bool,
}