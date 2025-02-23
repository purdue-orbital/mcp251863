use bitbybit::bitfield;

use crate::impl_register;

impl_register!(C1TREC, TransmitReceiveErrorCount, 0x0034, 4, u32);

#[bitfield(u32, default = 0x00_20_00_00)]
pub struct C1TREC {
    #[bits(0..=7, r)]
	rec: u8,

    #[bits(8..=15, r)]
	tec: u8,

    #[bits(16..=16, r)]
	ewarn: bool,

    #[bits(17..=17, r)]
	rxwarn: bool,

    #[bits(18..=18, r)]
	txwarn: bool,

    #[bits(19..=19, r)]
	rxbp: bool,

    #[bits(20..=20, r)]
	txbp: bool,

    #[bits(21..=21, r)]
	txbo: bool,
}