use bitbybit::bitfield;
use arbitrary_int::*;
use crate::instruction::*;
use crate::registers::*;
use crate::MCP251863;

pub mod message_objects;
mod configuration;
mod tx;

pub use message_objects::*;

// Are we using standard or extended FD frames?

const NOMINAL_BIT_RATE: u32 = 0; // TODO: figure out nominal bitrate
const DATA_BIT_RATE: u32 = 0; // TODO: figure out data bitrate. Are we using bitrate switching?