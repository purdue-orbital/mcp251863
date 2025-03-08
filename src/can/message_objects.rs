use arbitrary_int::*;
use bitbybit::bitfield;

pub fn encode_dlc(length: usize) -> u4 {
    if length == 1 {
        u4::from_u8(1)
    }
    else if length == 2 {
        u4::from_u8(2)
    }
    else if length == 3 {
        u4::from_u8(3)
    }
    else if length == 4 {
        u4::from_u8(4)
    }
    else if length == 5 {
        u4::from_u8(5)
    }
    else if length == 6 {
        u4::from_u8(6)
    }
    else if length == 7 {
        u4::from_u8(7)
    }
    else if length == 8 {
        u4::from_u8(8)
    }
    else if length == 12 {
        u4::from_u8(9)
    }
    else if length == 16 {
        u4::from_u8(10)
    }
    else if length == 20 {
        u4::from_u8(11)
    }
    else if length == 24 {
        u4::from_u8(12)
    }
    else if length == 32 {
        u4::from_u8(13)
    }
    else if length == 48 {
        u4::from_u8(14)
    }
    else if length == 64 {
        u4::from_u8(15)
    }
    else {
        // if data length is not from valid list (or is 0), encode with 0
        u4::from_u8(0)
    }
}

pub fn decode_dlc(dlc: u4) -> usize {
    match dlc.as_u8() {
        1 => 1,
        2 => 2,
        3 => 3,
        4 => 4,
        5 => 5,
        6 => 6,
        7 => 7,
        8 => 8,
        9 => 12,
        10 => 16,
        11 => 20,
        12 => 24,
        13 => 32,
        14 => 48,
        15 => 64,
        _ => 0,
    }
}

pub fn decode_dlc_u3(dlc: u3) -> usize {
    match dlc.as_u8() {
        0 => 8,
        1 => 12,
        2 => 16,
        3 => 20,
        4 => 24,
        5 => 32,
        6 => 48,
        7 => 64,
        _ => 8,
    }
}

#[bitfield(u32, default = 0)]
pub struct TransmitMessageWord0 {
    #[bits(0..=10, rw)]
    sid: u11,

    #[bits(11..=28, rw)]
    eid: u18,

    #[bit(29, rw)]
    sid11: bool,
}

#[bitfield(u32, default = 0)]
pub struct TransmitMessageWord1 {
    #[bits(0..=3, rw)]
    dlc: u4,

    #[bit(4, rw)]
    ide: bool,

    /// Not used in CAN FD
    #[bit(5, rw)]
    rtr: bool,

    #[bit(6, rw)]
    brs: bool,

    #[bit(7, rw)]
    fdf: bool,

    #[bit(8, rw)]
    esi: bool,

    #[bits(9..=31, rw)]
    seq: u23,
}

impl TransmitMessageWord0 {
    pub fn to_bytes(&self) -> [u8; 4] {
        self.raw_value.to_le_bytes()
    }
}

impl TransmitMessageWord1 {
    pub fn to_bytes(&self) -> [u8; 4] {
        self.raw_value.to_le_bytes()
    }
}

pub struct TransmitMessageObject<'a> {
    pub word0: TransmitMessageWord0,
    pub word1: TransmitMessageWord1,
    pub data: &'a [u8],
}

impl <'a> TransmitMessageObject<'a> {

    /// Creates a new data frame message object
    pub fn new_data_frame(standard_id: u11, id_extension: u18, data: &'a [u8], sequence_number: u23) -> Self {
        Self {
            word0: TransmitMessageWord0::builder()
                .with_sid(standard_id)
                .with_eid(id_extension)
                .with_sid11(false) // TODO: Should SID11 be set or not?
                .build(),

            word1: TransmitMessageWord1::builder()
                .with_dlc(encode_dlc(data.len()))
                .with_ide(true) // recessive for extended id frame format
                .with_rtr(false) // not a remote frame
                .with_brs(true) // recessive brs indicates bit rate switching enabled
                .with_fdf(true) // recessive indicates FD frames are being used (not classic)
                .with_esi(false) // TODO: read interrupts to find ESI value
                .with_seq(sequence_number) // sequence isn't important unless TEF is enabled
                .build(),

            data: data,
        }
    } 

    /*
    pub fn new_error_frame(standard_id: u11, id_extension: u18) -> Self {
        Self {
            word0: TransmitMessageWord0::builder()
                .with_sid(standard_id)
                .with_eid(id_extension)
                .with_sid11(false) // TODO: Should SID11 be set or not?
                .build(),

            word1: TransmitMessageWord1::builder()
                .with_dlc(find_dlc(0))
                .with_ide(true) // recessive for extended id frame format
                .with_rtr(false) // not a remote frame
                .with_brs(true) // recessive brs indicates bit rate switching enabled
                .with_fdf(true) // recessive indicates FD frames are being used (not classic)
                .with_esi(false) // TODO: read interrupts to find ESI value
                .with_seq(u23::from_u32(1)) // sequence isn't important unless TEF is enabled
                .build(),
            
            data: &[],
        }
    }
    */
}