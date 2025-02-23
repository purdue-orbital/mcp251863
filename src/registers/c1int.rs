use bitbybit::bitfield;

use crate::impl_register;

impl_register!(C1INT, Interrupt, 0x001C, 4, u32);

#[bitfield(u32, default = 0)]
pub struct C1INT {
    #[bits(0..=0, r)]
	txif: bool,

    #[bits(1..=1, r)]
	rxif: bool,

    #[bits(2..=2)]
	tbcif: bool,
    
    #[bits(3..=3)]
	modif: bool,
    
    #[bits(4..=4, r)]
	tefif: bool,
    
    #[bits(8..=8, r)]
	eccif: bool,
    
    #[bits(9..=9, r)]
	spicrcif: bool,
    
    #[bits(10..=10, r)]
	txatif: bool,
    
    #[bits(11..=11, r)]
	rxovif: bool,

    #[bits(12..=12)]
	serrif: bool,

    #[bits(13..=13)]
	cerrif: bool,

    #[bits(14..=14)]
	wakif: bool,

    #[bits(15..=15)]
	ivmif: bool,

    #[bits(16..=16, rw)]
	txie: bool,

    #[bits(17..=17, rw)]
	rxie: bool,

    #[bits(18..=18, rw)]
	tbcie: bool,

    #[bits(19..=19, rw)]
	modie: bool,

    #[bits(20..=20, rw)]
	tefie: bool,

    #[bits(24..=24, rw)]
	eccie: bool,

    #[bits(25..=25, rw)]
	spicrcie: bool,

    #[bits(26..=26, rw)]
	txatie: bool,

    #[bits(27..=27, rw)]
	rxovie: bool,

    #[bits(28..=28, rw)]
	serrie: bool,

    #[bits(29..=29, rw)]
	cerrie: bool,

    #[bits(30..=30, rw)]
	wakie: bool,

    #[bits(31..=31, rw)]
	ivmie: bool,
}
