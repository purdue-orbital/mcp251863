use arbitrary_int::{u2, u3, u5};
use bitbybit::bitfield;
use crate::impl_register;

impl_register!(C1FIFOCON1,  FIFOControl1,  0x5C,  4, u32);
impl_register!(C1FIFOCON2,  FIFOControl2,  0x68,  4, u32);
impl_register!(C1FIFOCON3,  FIFOControl3,  0x74,  4, u32);
impl_register!(C1FIFOCON4,  FIFOControl4,  0x80,  4, u32);
impl_register!(C1FIFOCON5,  FIFOControl5,  0x8C,  4, u32);
impl_register!(C1FIFOCON6,  FIFOControl6,  0x98,  4, u32);
impl_register!(C1FIFOCON7,  FIFOControl7,  0xA4,  4, u32);
impl_register!(C1FIFOCON8,  FIFOControl8,  0xB0,  4, u32);
impl_register!(C1FIFOCON9,  FIFOControl9,  0xBC,  4, u32);
impl_register!(C1FIFOCON10, FIFOControl10, 0xC8,  4, u32);
impl_register!(C1FIFOCON11, FIFOControl11, 0xD4,  4, u32);
impl_register!(C1FIFOCON12, FIFOControl12, 0xE0,  4, u32);
impl_register!(C1FIFOCON13, FIFOControl13, 0xEC,  4, u32);
impl_register!(C1FIFOCON14, FIFOControl14, 0xF8,  4, u32);
impl_register!(C1FIFOCON15, FIFOControl15, 0x104, 4, u32);
impl_register!(C1FIFOCON16, FIFOControl16, 0x110, 4, u32);
impl_register!(C1FIFOCON17, FIFOControl17, 0x11C, 4, u32);
impl_register!(C1FIFOCON18, FIFOControl18, 0x128, 4, u32);
impl_register!(C1FIFOCON19, FIFOControl19, 0x134, 4, u32);
impl_register!(C1FIFOCON20, FIFOControl20, 0x140, 4, u32);
impl_register!(C1FIFOCON21, FIFOControl21, 0x14C, 4, u32);
impl_register!(C1FIFOCON22, FIFOControl22, 0x158, 4, u32);
impl_register!(C1FIFOCON23, FIFOControl23, 0x164, 4, u32);
impl_register!(C1FIFOCON24, FIFOControl24, 0x170, 4, u32);
impl_register!(C1FIFOCON25, FIFOControl25, 0x17C, 4, u32);
impl_register!(C1FIFOCON26, FIFOControl26, 0x188, 4, u32);
impl_register!(C1FIFOCON27, FIFOControl27, 0x194, 4, u32);
impl_register!(C1FIFOCON28, FIFOControl28, 0x1A0, 4, u32);
impl_register!(C1FIFOCON29, FIFOControl29, 0x1AC, 4, u32);
impl_register!(C1FIFOCON30, FIFOControl30, 0x1B8, 4, u32);
impl_register!(C1FIFOCON31, FIFOControl31, 0x1C4, 4, u32);

#[bitfield(u32, default = 0x00_60_04_00)]
pub struct C1FIFOCON1 {
    #[bits(0..=0, rw)]
    tfnrfnie: bool,

    #[bits(1..=1, rw)]
    tfhrfhie: bool,

    #[bits(2..=2, rw)]
    tferffie: bool,

    #[bits(3..=3, rw)]
    rxovie: bool,

    #[bits(4..=4, rw)]
    txatie: bool,

    #[bits(5..=5, rw)]
    rxtsen: bool,

    #[bits(6..=6, rw)]
    rtren: bool,

    #[bits(7..=7, rw)]
    txen: bool,

    #[bits(8..=8)]
    uinc: bool,

    #[bits(9..=9, rw)]
    txreq: bool,

    #[bits(10..=10)]
    freset: bool,

    #[bits(16..=20, rw)]
    txpri: u5,

    #[bits(21..=22, rw)]
    txat: u2,

    #[bits(24..=28, rw)]
    fsize: u5,

    #[bits(29..=31, rw)]
    plsize: u3,
}

#[bitfield(u32, default = 0x00_60_04_00)]
pub struct C1FIFOCON2 {
    #[bits(0..=0, rw)]
    tfnrfnie: bool,

    #[bits(1..=1, rw)]
    tfhrfhie: bool,

    #[bits(2..=2, rw)]
    tferffie: bool,

    #[bits(3..=3, rw)]
    rxovie: bool,

    #[bits(4..=4, rw)]
    txatie: bool,

    #[bits(5..=5, rw)]
    rxtsen: bool,

    #[bits(6..=6, rw)]
    rtren: bool,

    #[bits(7..=7, rw)]
    txen: bool,

    #[bits(8..=8)]
    uinc: bool,

    #[bits(9..=9, rw)]
    txreq: bool,

    #[bits(10..=10)]
    freset: bool,

    #[bits(16..=20, rw)]
    txpri: u5,

    #[bits(21..=22, rw)]
    txat: u2,

    #[bits(24..=28, rw)]
    fsize: u5,

    #[bits(29..=31, rw)]
    plsize: u3,
}

#[bitfield(u32, default = 0x00_60_04_00)]
pub struct C1FIFOCON3 {
    #[bits(0..=0, rw)]
    tfnrfnie: bool,

    #[bits(1..=1, rw)]
    tfhrfhie: bool,

    #[bits(2..=2, rw)]
    tferffie: bool,

    #[bits(3..=3, rw)]
    rxovie: bool,

    #[bits(4..=4, rw)]
    txatie: bool,

    #[bits(5..=5, rw)]
    rxtsen: bool,

    #[bits(6..=6, rw)]
    rtren: bool,

    #[bits(7..=7, rw)]
    txen: bool,

    #[bits(8..=8)]
    uinc: bool,

    #[bits(9..=9, rw)]
    txreq: bool,

    #[bits(10..=10)]
    freset: bool,

    #[bits(16..=20, rw)]
    txpri: u5,

    #[bits(21..=22, rw)]
    txat: u2,

    #[bits(24..=28, rw)]
    fsize: u5,

    #[bits(29..=31, rw)]
    plsize: u3,
}

#[bitfield(u32, default = 0x00_60_04_00)]
pub struct C1FIFOCON4 {
    #[bits(0..=0, rw)]
    tfnrfnie: bool,

    #[bits(1..=1, rw)]
    tfhrfhie: bool,

    #[bits(2..=2, rw)]
    tferffie: bool,

    #[bits(3..=3, rw)]
    rxovie: bool,

    #[bits(4..=4, rw)]
    txatie: bool,

    #[bits(5..=5, rw)]
    rxtsen: bool,

    #[bits(6..=6, rw)]
    rtren: bool,

    #[bits(7..=7, rw)]
    txen: bool,

    #[bits(8..=8)]
    uinc: bool,

    #[bits(9..=9, rw)]
    txreq: bool,

    #[bits(10..=10)]
    freset: bool,

    #[bits(16..=20, rw)]
    txpri: u5,

    #[bits(21..=22, rw)]
    txat: u2,

    #[bits(24..=28, rw)]
    fsize: u5,

    #[bits(29..=31, rw)]
    plsize: u3,
}

#[bitfield(u32, default = 0x00_60_04_00)]
pub struct C1FIFOCON5 {
    #[bits(0..=0, rw)]
    tfnrfnie: bool,

    #[bits(1..=1, rw)]
    tfhrfhie: bool,

    #[bits(2..=2, rw)]
    tferffie: bool,

    #[bits(3..=3, rw)]
    rxovie: bool,

    #[bits(4..=4, rw)]
    txatie: bool,

    #[bits(5..=5, rw)]
    rxtsen: bool,

    #[bits(6..=6, rw)]
    rtren: bool,

    #[bits(7..=7, rw)]
    txen: bool,

    #[bits(8..=8)]
    uinc: bool,

    #[bits(9..=9, rw)]
    txreq: bool,

    #[bits(10..=10)]
    freset: bool,

    #[bits(16..=20, rw)]
    txpri: u5,

    #[bits(21..=22, rw)]
    txat: u2,

    #[bits(24..=28, rw)]
    fsize: u5,

    #[bits(29..=31, rw)]
    plsize: u3,
}

#[bitfield(u32, default = 0x00_60_04_00)]
pub struct C1FIFOCON6 {
    #[bits(0..=0, rw)]
    tfnrfnie: bool,

    #[bits(1..=1, rw)]
    tfhrfhie: bool,

    #[bits(2..=2, rw)]
    tferffie: bool,

    #[bits(3..=3, rw)]
    rxovie: bool,

    #[bits(4..=4, rw)]
    txatie: bool,

    #[bits(5..=5, rw)]
    rxtsen: bool,

    #[bits(6..=6, rw)]
    rtren: bool,

    #[bits(7..=7, rw)]
    txen: bool,

    #[bits(8..=8)]
    uinc: bool,

    #[bits(9..=9, rw)]
    txreq: bool,

    #[bits(10..=10)]
    freset: bool,

    #[bits(16..=20, rw)]
    txpri: u5,

    #[bits(21..=22, rw)]
    txat: u2,

    #[bits(24..=28, rw)]
    fsize: u5,

    #[bits(29..=31, rw)]
    plsize: u3,
}

#[bitfield(u32, default = 0x00_60_04_00)]
pub struct C1FIFOCON7 {
    #[bits(0..=0, rw)]
    tfnrfnie: bool,

    #[bits(1..=1, rw)]
    tfhrfhie: bool,

    #[bits(2..=2, rw)]
    tferffie: bool,

    #[bits(3..=3, rw)]
    rxovie: bool,

    #[bits(4..=4, rw)]
    txatie: bool,

    #[bits(5..=5, rw)]
    rxtsen: bool,

    #[bits(6..=6, rw)]
    rtren: bool,

    #[bits(7..=7, rw)]
    txen: bool,

    #[bits(8..=8)]
    uinc: bool,

    #[bits(9..=9, rw)]
    txreq: bool,

    #[bits(10..=10)]
    freset: bool,

    #[bits(16..=20, rw)]
    txpri: u5,

    #[bits(21..=22, rw)]
    txat: u2,

    #[bits(24..=28, rw)]
    fsize: u5,

    #[bits(29..=31, rw)]
    plsize: u3,
}

#[bitfield(u32, default = 0x00_60_04_00)]
pub struct C1FIFOCON8 {
    #[bits(0..=0, rw)]
    tfnrfnie: bool,

    #[bits(1..=1, rw)]
    tfhrfhie: bool,

    #[bits(2..=2, rw)]
    tferffie: bool,

    #[bits(3..=3, rw)]
    rxovie: bool,

    #[bits(4..=4, rw)]
    txatie: bool,

    #[bits(5..=5, rw)]
    rxtsen: bool,

    #[bits(6..=6, rw)]
    rtren: bool,

    #[bits(7..=7, rw)]
    txen: bool,

    #[bits(8..=8)]
    uinc: bool,

    #[bits(9..=9, rw)]
    txreq: bool,

    #[bits(10..=10)]
    freset: bool,

    #[bits(16..=20, rw)]
    txpri: u5,

    #[bits(21..=22, rw)]
    txat: u2,

    #[bits(24..=28, rw)]
    fsize: u5,

    #[bits(29..=31, rw)]
    plsize: u3,
}

#[bitfield(u32, default = 0x00_60_04_00)]
pub struct C1FIFOCON9 {
    #[bits(0..=0, rw)]
    tfnrfnie: bool,

    #[bits(1..=1, rw)]
    tfhrfhie: bool,

    #[bits(2..=2, rw)]
    tferffie: bool,

    #[bits(3..=3, rw)]
    rxovie: bool,

    #[bits(4..=4, rw)]
    txatie: bool,

    #[bits(5..=5, rw)]
    rxtsen: bool,

    #[bits(6..=6, rw)]
    rtren: bool,

    #[bits(7..=7, rw)]
    txen: bool,

    #[bits(8..=8)]
    uinc: bool,

    #[bits(9..=9, rw)]
    txreq: bool,

    #[bits(10..=10)]
    freset: bool,

    #[bits(16..=20, rw)]
    txpri: u5,

    #[bits(21..=22, rw)]
    txat: u2,

    #[bits(24..=28, rw)]
    fsize: u5,

    #[bits(29..=31, rw)]
    plsize: u3,
}

#[bitfield(u32, default = 0x00_60_04_00)]
pub struct C1FIFOCON10 {
    #[bits(0..=0, rw)]
    tfnrfnie: bool,

    #[bits(1..=1, rw)]
    tfhrfhie: bool,

    #[bits(2..=2, rw)]
    tferffie: bool,

    #[bits(3..=3, rw)]
    rxovie: bool,

    #[bits(4..=4, rw)]
    txatie: bool,

    #[bits(5..=5, rw)]
    rxtsen: bool,

    #[bits(6..=6, rw)]
    rtren: bool,

    #[bits(7..=7, rw)]
    txen: bool,

    #[bits(8..=8)]
    uinc: bool,

    #[bits(9..=9, rw)]
    txreq: bool,

    #[bits(10..=10)]
    freset: bool,

    #[bits(16..=20, rw)]
    txpri: u5,

    #[bits(21..=22, rw)]
    txat: u2,

    #[bits(24..=28, rw)]
    fsize: u5,

    #[bits(29..=31, rw)]
    plsize: u3,
}

#[bitfield(u32, default = 0x00_60_04_00)]
pub struct C1FIFOCON11 {
    #[bits(0..=0, rw)]
    tfnrfnie: bool,

    #[bits(1..=1, rw)]
    tfhrfhie: bool,

    #[bits(2..=2, rw)]
    tferffie: bool,

    #[bits(3..=3, rw)]
    rxovie: bool,

    #[bits(4..=4, rw)]
    txatie: bool,

    #[bits(5..=5, rw)]
    rxtsen: bool,

    #[bits(6..=6, rw)]
    rtren: bool,

    #[bits(7..=7, rw)]
    txen: bool,

    #[bits(8..=8)]
    uinc: bool,

    #[bits(9..=9, rw)]
    txreq: bool,

    #[bits(10..=10)]
    freset: bool,

    #[bits(16..=20, rw)]
    txpri: u5,

    #[bits(21..=22, rw)]
    txat: u2,

    #[bits(24..=28, rw)]
    fsize: u5,

    #[bits(29..=31, rw)]
    plsize: u3,
}

#[bitfield(u32, default = 0x00_60_04_00)]
pub struct C1FIFOCON12 {
    #[bits(0..=0, rw)]
    tfnrfnie: bool,

    #[bits(1..=1, rw)]
    tfhrfhie: bool,

    #[bits(2..=2, rw)]
    tferffie: bool,

    #[bits(3..=3, rw)]
    rxovie: bool,

    #[bits(4..=4, rw)]
    txatie: bool,

    #[bits(5..=5, rw)]
    rxtsen: bool,

    #[bits(6..=6, rw)]
    rtren: bool,

    #[bits(7..=7, rw)]
    txen: bool,

    #[bits(8..=8)]
    uinc: bool,

    #[bits(9..=9, rw)]
    txreq: bool,

    #[bits(10..=10)]
    freset: bool,

    #[bits(16..=20, rw)]
    txpri: u5,

    #[bits(21..=22, rw)]
    txat: u2,

    #[bits(24..=28, rw)]
    fsize: u5,

    #[bits(29..=31, rw)]
    plsize: u3,
}

#[bitfield(u32, default = 0x00_60_04_00)]
pub struct C1FIFOCON13 {
    #[bits(0..=0, rw)]
    tfnrfnie: bool,

    #[bits(1..=1, rw)]
    tfhrfhie: bool,

    #[bits(2..=2, rw)]
    tferffie: bool,

    #[bits(3..=3, rw)]
    rxovie: bool,

    #[bits(4..=4, rw)]
    txatie: bool,

    #[bits(5..=5, rw)]
    rxtsen: bool,

    #[bits(6..=6, rw)]
    rtren: bool,

    #[bits(7..=7, rw)]
    txen: bool,

    #[bits(8..=8)]
    uinc: bool,

    #[bits(9..=9, rw)]
    txreq: bool,

    #[bits(10..=10)]
    freset: bool,

    #[bits(16..=20, rw)]
    txpri: u5,

    #[bits(21..=22, rw)]
    txat: u2,

    #[bits(24..=28, rw)]
    fsize: u5,

    #[bits(29..=31, rw)]
    plsize: u3,
}

#[bitfield(u32, default = 0x00_60_04_00)]
pub struct C1FIFOCON14 {
    #[bits(0..=0, rw)]
    tfnrfnie: bool,

    #[bits(1..=1, rw)]
    tfhrfhie: bool,

    #[bits(2..=2, rw)]
    tferffie: bool,

    #[bits(3..=3, rw)]
    rxovie: bool,

    #[bits(4..=4, rw)]
    txatie: bool,

    #[bits(5..=5, rw)]
    rxtsen: bool,

    #[bits(6..=6, rw)]
    rtren: bool,

    #[bits(7..=7, rw)]
    txen: bool,

    #[bits(8..=8)]
    uinc: bool,

    #[bits(9..=9, rw)]
    txreq: bool,

    #[bits(10..=10)]
    freset: bool,

    #[bits(16..=20, rw)]
    txpri: u5,

    #[bits(21..=22, rw)]
    txat: u2,

    #[bits(24..=28, rw)]
    fsize: u5,

    #[bits(29..=31, rw)]
    plsize: u3,
}

#[bitfield(u32, default = 0x00_60_04_00)]
pub struct C1FIFOCON15 {
    #[bits(0..=0, rw)]
    tfnrfnie: bool,

    #[bits(1..=1, rw)]
    tfhrfhie: bool,

    #[bits(2..=2, rw)]
    tferffie: bool,

    #[bits(3..=3, rw)]
    rxovie: bool,

    #[bits(4..=4, rw)]
    txatie: bool,

    #[bits(5..=5, rw)]
    rxtsen: bool,

    #[bits(6..=6, rw)]
    rtren: bool,

    #[bits(7..=7, rw)]
    txen: bool,

    #[bits(8..=8)]
    uinc: bool,

    #[bits(9..=9, rw)]
    txreq: bool,

    #[bits(10..=10)]
    freset: bool,

    #[bits(16..=20, rw)]
    txpri: u5,

    #[bits(21..=22, rw)]
    txat: u2,

    #[bits(24..=28, rw)]
    fsize: u5,

    #[bits(29..=31, rw)]
    plsize: u3,
}

#[bitfield(u32, default = 0x00_60_04_00)]
pub struct C1FIFOCON16 {
    #[bits(0..=0, rw)]
    tfnrfnie: bool,

    #[bits(1..=1, rw)]
    tfhrfhie: bool,

    #[bits(2..=2, rw)]
    tferffie: bool,

    #[bits(3..=3, rw)]
    rxovie: bool,

    #[bits(4..=4, rw)]
    txatie: bool,

    #[bits(5..=5, rw)]
    rxtsen: bool,

    #[bits(6..=6, rw)]
    rtren: bool,

    #[bits(7..=7, rw)]
    txen: bool,

    #[bits(8..=8)]
    uinc: bool,

    #[bits(9..=9, rw)]
    txreq: bool,

    #[bits(10..=10)]
    freset: bool,

    #[bits(16..=20, rw)]
    txpri: u5,

    #[bits(21..=22, rw)]
    txat: u2,

    #[bits(24..=28, rw)]
    fsize: u5,

    #[bits(29..=31, rw)]
    plsize: u3,
}

#[bitfield(u32, default = 0x00_60_04_00)]
pub struct C1FIFOCON17 {
    #[bits(0..=0, rw)]
    tfnrfnie: bool,

    #[bits(1..=1, rw)]
    tfhrfhie: bool,

    #[bits(2..=2, rw)]
    tferffie: bool,

    #[bits(3..=3, rw)]
    rxovie: bool,

    #[bits(4..=4, rw)]
    txatie: bool,

    #[bits(5..=5, rw)]
    rxtsen: bool,

    #[bits(6..=6, rw)]
    rtren: bool,

    #[bits(7..=7, rw)]
    txen: bool,

    #[bits(8..=8)]
    uinc: bool,

    #[bits(9..=9, rw)]
    txreq: bool,

    #[bits(10..=10)]
    freset: bool,

    #[bits(16..=20, rw)]
    txpri: u5,

    #[bits(21..=22, rw)]
    txat: u2,

    #[bits(24..=28, rw)]
    fsize: u5,

    #[bits(29..=31, rw)]
    plsize: u3,
}

#[bitfield(u32, default = 0x00_60_04_00)]
pub struct C1FIFOCON18 {
    #[bits(0..=0, rw)]
    tfnrfnie: bool,

    #[bits(1..=1, rw)]
    tfhrfhie: bool,

    #[bits(2..=2, rw)]
    tferffie: bool,

    #[bits(3..=3, rw)]
    rxovie: bool,

    #[bits(4..=4, rw)]
    txatie: bool,

    #[bits(5..=5, rw)]
    rxtsen: bool,

    #[bits(6..=6, rw)]
    rtren: bool,

    #[bits(7..=7, rw)]
    txen: bool,

    #[bits(8..=8)]
    uinc: bool,

    #[bits(9..=9, rw)]
    txreq: bool,

    #[bits(10..=10)]
    freset: bool,

    #[bits(16..=20, rw)]
    txpri: u5,

    #[bits(21..=22, rw)]
    txat: u2,

    #[bits(24..=28, rw)]
    fsize: u5,

    #[bits(29..=31, rw)]
    plsize: u3,
}

#[bitfield(u32, default = 0x00_60_04_00)]
pub struct C1FIFOCON19 {
    #[bits(0..=0, rw)]
    tfnrfnie: bool,

    #[bits(1..=1, rw)]
    tfhrfhie: bool,

    #[bits(2..=2, rw)]
    tferffie: bool,

    #[bits(3..=3, rw)]
    rxovie: bool,

    #[bits(4..=4, rw)]
    txatie: bool,

    #[bits(5..=5, rw)]
    rxtsen: bool,

    #[bits(6..=6, rw)]
    rtren: bool,

    #[bits(7..=7, rw)]
    txen: bool,

    #[bits(8..=8)]
    uinc: bool,

    #[bits(9..=9, rw)]
    txreq: bool,

    #[bits(10..=10)]
    freset: bool,

    #[bits(16..=20, rw)]
    txpri: u5,

    #[bits(21..=22, rw)]
    txat: u2,

    #[bits(24..=28, rw)]
    fsize: u5,

    #[bits(29..=31, rw)]
    plsize: u3,
}

#[bitfield(u32, default = 0x00_60_04_00)]
pub struct C1FIFOCON20 {
    #[bits(0..=0, rw)]
    tfnrfnie: bool,

    #[bits(1..=1, rw)]
    tfhrfhie: bool,

    #[bits(2..=2, rw)]
    tferffie: bool,

    #[bits(3..=3, rw)]
    rxovie: bool,

    #[bits(4..=4, rw)]
    txatie: bool,

    #[bits(5..=5, rw)]
    rxtsen: bool,

    #[bits(6..=6, rw)]
    rtren: bool,

    #[bits(7..=7, rw)]
    txen: bool,

    #[bits(8..=8)]
    uinc: bool,

    #[bits(9..=9, rw)]
    txreq: bool,

    #[bits(10..=10)]
    freset: bool,

    #[bits(16..=20, rw)]
    txpri: u5,

    #[bits(21..=22, rw)]
    txat: u2,

    #[bits(24..=28, rw)]
    fsize: u5,

    #[bits(29..=31, rw)]
    plsize: u3,
}

#[bitfield(u32, default = 0x00_60_04_00)]
pub struct C1FIFOCON21 {
    #[bits(0..=0, rw)]
    tfnrfnie: bool,

    #[bits(1..=1, rw)]
    tfhrfhie: bool,

    #[bits(2..=2, rw)]
    tferffie: bool,

    #[bits(3..=3, rw)]
    rxovie: bool,

    #[bits(4..=4, rw)]
    txatie: bool,

    #[bits(5..=5, rw)]
    rxtsen: bool,

    #[bits(6..=6, rw)]
    rtren: bool,

    #[bits(7..=7, rw)]
    txen: bool,

    #[bits(8..=8)]
    uinc: bool,

    #[bits(9..=9, rw)]
    txreq: bool,

    #[bits(10..=10)]
    freset: bool,

    #[bits(16..=20, rw)]
    txpri: u5,

    #[bits(21..=22, rw)]
    txat: u2,

    #[bits(24..=28, rw)]
    fsize: u5,

    #[bits(29..=31, rw)]
    plsize: u3,
}

#[bitfield(u32, default = 0x00_60_04_00)]
pub struct C1FIFOCON22 {
    #[bits(0..=0, rw)]
    tfnrfnie: bool,

    #[bits(1..=1, rw)]
    tfhrfhie: bool,

    #[bits(2..=2, rw)]
    tferffie: bool,

    #[bits(3..=3, rw)]
    rxovie: bool,

    #[bits(4..=4, rw)]
    txatie: bool,

    #[bits(5..=5, rw)]
    rxtsen: bool,

    #[bits(6..=6, rw)]
    rtren: bool,

    #[bits(7..=7, rw)]
    txen: bool,

    #[bits(8..=8)]
    uinc: bool,

    #[bits(9..=9, rw)]
    txreq: bool,

    #[bits(10..=10)]
    freset: bool,

    #[bits(16..=20, rw)]
    txpri: u5,

    #[bits(21..=22, rw)]
    txat: u2,

    #[bits(24..=28, rw)]
    fsize: u5,

    #[bits(29..=31, rw)]
    plsize: u3,
}

#[bitfield(u32, default = 0x00_60_04_00)]
pub struct C1FIFOCON23 {
    #[bits(0..=0, rw)]
    tfnrfnie: bool,

    #[bits(1..=1, rw)]
    tfhrfhie: bool,

    #[bits(2..=2, rw)]
    tferffie: bool,

    #[bits(3..=3, rw)]
    rxovie: bool,

    #[bits(4..=4, rw)]
    txatie: bool,

    #[bits(5..=5, rw)]
    rxtsen: bool,

    #[bits(6..=6, rw)]
    rtren: bool,

    #[bits(7..=7, rw)]
    txen: bool,

    #[bits(8..=8)]
    uinc: bool,

    #[bits(9..=9, rw)]
    txreq: bool,

    #[bits(10..=10)]
    freset: bool,

    #[bits(16..=20, rw)]
    txpri: u5,

    #[bits(21..=22, rw)]
    txat: u2,

    #[bits(24..=28, rw)]
    fsize: u5,

    #[bits(29..=31, rw)]
    plsize: u3,
}

#[bitfield(u32, default = 0x00_60_04_00)]
pub struct C1FIFOCON24 {
    #[bits(0..=0, rw)]
    tfnrfnie: bool,

    #[bits(1..=1, rw)]
    tfhrfhie: bool,

    #[bits(2..=2, rw)]
    tferffie: bool,

    #[bits(3..=3, rw)]
    rxovie: bool,

    #[bits(4..=4, rw)]
    txatie: bool,

    #[bits(5..=5, rw)]
    rxtsen: bool,

    #[bits(6..=6, rw)]
    rtren: bool,

    #[bits(7..=7, rw)]
    txen: bool,

    #[bits(8..=8)]
    uinc: bool,

    #[bits(9..=9, rw)]
    txreq: bool,

    #[bits(10..=10)]
    freset: bool,

    #[bits(16..=20, rw)]
    txpri: u5,

    #[bits(21..=22, rw)]
    txat: u2,

    #[bits(24..=28, rw)]
    fsize: u5,

    #[bits(29..=31, rw)]
    plsize: u3,
}

#[bitfield(u32, default = 0x00_60_04_00)]
pub struct C1FIFOCON25 {
    #[bits(0..=0, rw)]
    tfnrfnie: bool,

    #[bits(1..=1, rw)]
    tfhrfhie: bool,

    #[bits(2..=2, rw)]
    tferffie: bool,

    #[bits(3..=3, rw)]
    rxovie: bool,

    #[bits(4..=4, rw)]
    txatie: bool,

    #[bits(5..=5, rw)]
    rxtsen: bool,

    #[bits(6..=6, rw)]
    rtren: bool,

    #[bits(7..=7, rw)]
    txen: bool,

    #[bits(8..=8)]
    uinc: bool,

    #[bits(9..=9, rw)]
    txreq: bool,

    #[bits(10..=10)]
    freset: bool,

    #[bits(16..=20, rw)]
    txpri: u5,

    #[bits(21..=22, rw)]
    txat: u2,

    #[bits(24..=28, rw)]
    fsize: u5,

    #[bits(29..=31, rw)]
    plsize: u3,
}

#[bitfield(u32, default = 0x00_60_04_00)]
pub struct C1FIFOCON26 {
    #[bits(0..=0, rw)]
    tfnrfnie: bool,

    #[bits(1..=1, rw)]
    tfhrfhie: bool,

    #[bits(2..=2, rw)]
    tferffie: bool,

    #[bits(3..=3, rw)]
    rxovie: bool,

    #[bits(4..=4, rw)]
    txatie: bool,

    #[bits(5..=5, rw)]
    rxtsen: bool,

    #[bits(6..=6, rw)]
    rtren: bool,

    #[bits(7..=7, rw)]
    txen: bool,

    #[bits(8..=8)]
    uinc: bool,

    #[bits(9..=9, rw)]
    txreq: bool,

    #[bits(10..=10)]
    freset: bool,

    #[bits(16..=20, rw)]
    txpri: u5,

    #[bits(21..=22, rw)]
    txat: u2,

    #[bits(24..=28, rw)]
    fsize: u5,

    #[bits(29..=31, rw)]
    plsize: u3,
}

#[bitfield(u32, default = 0x00_60_04_00)]
pub struct C1FIFOCON27 {
    #[bits(0..=0, rw)]
    tfnrfnie: bool,

    #[bits(1..=1, rw)]
    tfhrfhie: bool,

    #[bits(2..=2, rw)]
    tferffie: bool,

    #[bits(3..=3, rw)]
    rxovie: bool,

    #[bits(4..=4, rw)]
    txatie: bool,

    #[bits(5..=5, rw)]
    rxtsen: bool,

    #[bits(6..=6, rw)]
    rtren: bool,

    #[bits(7..=7, rw)]
    txen: bool,

    #[bits(8..=8)]
    uinc: bool,

    #[bits(9..=9, rw)]
    txreq: bool,

    #[bits(10..=10)]
    freset: bool,

    #[bits(16..=20, rw)]
    txpri: u5,

    #[bits(21..=22, rw)]
    txat: u2,

    #[bits(24..=28, rw)]
    fsize: u5,

    #[bits(29..=31, rw)]
    plsize: u3,
}

#[bitfield(u32, default = 0x00_60_04_00)]
pub struct C1FIFOCON28 {
    #[bits(0..=0, rw)]
    tfnrfnie: bool,

    #[bits(1..=1, rw)]
    tfhrfhie: bool,

    #[bits(2..=2, rw)]
    tferffie: bool,

    #[bits(3..=3, rw)]
    rxovie: bool,

    #[bits(4..=4, rw)]
    txatie: bool,

    #[bits(5..=5, rw)]
    rxtsen: bool,

    #[bits(6..=6, rw)]
    rtren: bool,

    #[bits(7..=7, rw)]
    txen: bool,

    #[bits(8..=8)]
    uinc: bool,

    #[bits(9..=9, rw)]
    txreq: bool,

    #[bits(10..=10)]
    freset: bool,

    #[bits(16..=20, rw)]
    txpri: u5,

    #[bits(21..=22, rw)]
    txat: u2,

    #[bits(24..=28, rw)]
    fsize: u5,

    #[bits(29..=31, rw)]
    plsize: u3,
}

#[bitfield(u32, default = 0x00_60_04_00)]
pub struct C1FIFOCON29 {
    #[bits(0..=0, rw)]
    tfnrfnie: bool,

    #[bits(1..=1, rw)]
    tfhrfhie: bool,

    #[bits(2..=2, rw)]
    tferffie: bool,

    #[bits(3..=3, rw)]
    rxovie: bool,

    #[bits(4..=4, rw)]
    txatie: bool,

    #[bits(5..=5, rw)]
    rxtsen: bool,

    #[bits(6..=6, rw)]
    rtren: bool,

    #[bits(7..=7, rw)]
    txen: bool,

    #[bits(8..=8)]
    uinc: bool,

    #[bits(9..=9, rw)]
    txreq: bool,

    #[bits(10..=10)]
    freset: bool,

    #[bits(16..=20, rw)]
    txpri: u5,

    #[bits(21..=22, rw)]
    txat: u2,

    #[bits(24..=28, rw)]
    fsize: u5,

    #[bits(29..=31, rw)]
    plsize: u3,
}

#[bitfield(u32, default = 0x00_60_04_00)]
pub struct C1FIFOCON30 {
    #[bits(0..=0, rw)]
    tfnrfnie: bool,

    #[bits(1..=1, rw)]
    tfhrfhie: bool,

    #[bits(2..=2, rw)]
    tferffie: bool,

    #[bits(3..=3, rw)]
    rxovie: bool,

    #[bits(4..=4, rw)]
    txatie: bool,

    #[bits(5..=5, rw)]
    rxtsen: bool,

    #[bits(6..=6, rw)]
    rtren: bool,

    #[bits(7..=7, rw)]
    txen: bool,

    #[bits(8..=8)]
    uinc: bool,

    #[bits(9..=9, rw)]
    txreq: bool,

    #[bits(10..=10)]
    freset: bool,

    #[bits(16..=20, rw)]
    txpri: u5,

    #[bits(21..=22, rw)]
    txat: u2,

    #[bits(24..=28, rw)]
    fsize: u5,

    #[bits(29..=31, rw)]
    plsize: u3,
}

#[bitfield(u32, default = 0x00_60_04_00)]
pub struct C1FIFOCON31 {
    #[bits(0..=0, rw)]
    tfnrfnie: bool,

    #[bits(1..=1, rw)]
    tfhrfhie: bool,

    #[bits(2..=2, rw)]
    tferffie: bool,

    #[bits(3..=3, rw)]
    rxovie: bool,

    #[bits(4..=4, rw)]
    txatie: bool,

    #[bits(5..=5, rw)]
    rxtsen: bool,

    #[bits(6..=6, rw)]
    rtren: bool,

    #[bits(7..=7, rw)]
    txen: bool,

    #[bits(8..=8)]
    uinc: bool,

    #[bits(9..=9, rw)]
    txreq: bool,

    #[bits(10..=10)]
    freset: bool,

    #[bits(16..=20, rw)]
    txpri: u5,

    #[bits(21..=22, rw)]
    txat: u2,

    #[bits(24..=28, rw)]
    fsize: u5,

    #[bits(29..=31, rw)]
    plsize: u3,
}

