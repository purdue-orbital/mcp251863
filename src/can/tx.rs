//! transmission functions and structures

use bitbybit::bitfield;
use crate::instruction::{Instruction, InstructionError};
use crate::MCP251863;
use embedded_hal::spi::{SpiDevice, Error, ErrorKind};
use crate::registers::prelude::*;
use arbitrary_int::*;
use crate::can::message_objects::{
    TransmitMessageObject,
    TransmitMessageWord0,
    TransmitMessageWord1,
    decode_dlc_u3,
};

/// Default transmit FIFO configuration:
/// NOTE: priority is set to 0 (highest priority). This should usually be changed by application
/// * 64 byte payload size
/// * 32 messages deep
/// * unlimited retransmit attempts
/// * highest priority
/// * all transmit interrupts set
/// * all recieve interrupts disabled
// TODO: figure out configuration for these settings
pub const TX_FIFO_64B_DEFAULT: FIFOControl = FIFOControl::new_with_raw_value(0x00_00_00_00);

pub struct TransmitFIFO {
    fifocon_addr: u12, // which FIFO is this? must be in 1..=31
    payload_size: usize,
    max_spi_retries: i8,
	max_spi_crc_retries: i8,
}


impl TransmitFIFO {

    /// MCP must be in configuration mode when this function is executed
    pub fn new(bus: &mut impl SpiDevice, n: u8, fifocon_reg: FIFOControl) -> Result<Self, InstructionError> {
        let fifocon_addr = FIFOControl::addr_n(n);

        fifocon_reg.write_register_crc(bus)?;

        Ok(TransmitFIFO {
            fifocon_addr: u12::from_u16(0x400) + fifocon_addr * u12::from_u8(n),
            payload_size: decode_dlc_u3(fifocon_reg.plsize()),
            max_spi_retries: -1, // TODO make configurable
            max_spi_crc_retries: -1,
        })
    }

    // TODO: check datasheet for reset preconditions and behavior
    pub fn reset(&self, bus: &mut impl SpiDevice) -> Result<(), InstructionError> {
        self.write_fifocon_byte2(bus, &[0x00])
    }

    // TODO: check datasheet for reset preconditions and behavior
    pub fn reset_crc(&self, bus: &mut impl SpiDevice) -> Result<(), InstructionError> {
        self.write_fifocon_byte2_crc(bus, &[0x00])
    }

    /// Loads a message into the FIFO, does NOT use crc to verify the spi message
    pub fn load_message(&self, bus: &mut impl SpiDevice, msg: &TransmitMessageObject) -> Result<(), InstructionError> {
        // abridge data if it won't fit in the fifo
        let data_to_transmit = {
            if msg.data.len() > self.payload_size {
                &msg.data[..self.payload_size]
            } else {
                &msg.data
            }
        };

        // read fifo control, status, and address registers
        let mut fifo_registers = [0u8; 12];
        let mut result = Instruction::read(bus, self.fifocon_addr, &mut fifo_registers);
        let mut tries = 0;
        while result.is_err() && (self.max_spi_retries > 0 || tries < self.max_spi_retries) {
            result = Instruction::read(bus, self.fifocon_addr, &mut fifo_registers);
            tries += 1;
        }
        result?;

        // check if FIFO is full
        if fifo_registers[4] | 0b1111_1110 != 0b1111_1111 {
            return Err(InstructionError::External(ErrorKind::Overrun.kind()));
        }

        // attempt to write the message to the next fifo position
        let next_message_addr = u12::from_u16(0x0400 + (fifo_registers[10] as u16) << 8 | (fifo_registers[11] as u16));
        result = Instruction::write_tx_message_object(bus, next_message_addr, msg.word0, msg.word1, data_to_transmit); // Replace with crc version
        tries = 0;
        while result.is_err() && (self.max_spi_retries > 0 || tries < self.max_spi_retries) {
            result = Instruction::write_tx_message_object(bus, next_message_addr, msg.word0, msg.word1, data_to_transmit);
            
            tries += 1;
        }
        result?;

        // attempt to set the UINC register to increments the fifo address
        result = Instruction::write(bus, self.fifocon_addr + u12::from_u8(1), &[fifo_registers[1] | 0b0000_0001]);
        tries = 0;
        while result.is_err() && (self.max_spi_retries > 0 || tries < self.max_spi_retries) {
            result = Instruction::write(bus, self.fifocon_addr + u12::from_u8(1), &[fifo_registers[1] | 0b0000_0001]);
            tries += 1;
        }
        result
    }

    /// Loads a message into the FIFO, using crc to verify the spi message
    pub fn load_message_crc(&self, bus: &mut impl SpiDevice, msg: &TransmitMessageObject) -> Result<(), InstructionError> {
        // abridge data if it won't fit in the fifo
        let data_to_transmit = {
            if msg.data.len() > self.payload_size {
                &msg.data[..self.payload_size]
            } else {
                &msg.data
            }
        };

        // read fifo control, status, and address registers
        let mut fifo_registers = [0u8; 12];
        let mut result = Instruction::read_crc(bus, self.fifocon_addr, &mut fifo_registers);
        let mut tries = 0;
        while result.is_err() && (self.max_spi_crc_retries > 0 || tries < self.max_spi_crc_retries) {
            result = Instruction::read_crc(bus, self.fifocon_addr, &mut fifo_registers);
            tries += 1;
        }
        result?;

        // check if FIFO is full
        if fifo_registers[4] | 0b1111_1110 != 0b1111_1111 {
            return Err(InstructionError::External(ErrorKind::Overrun.kind()));
        }

        // attempt to write the message to the next fifo position
        let next_message_addr = u12::from_u16(0x0400 + (fifo_registers[10] as u16) << 8 | (fifo_registers[11] as u16));
        result = Instruction::write_tx_message_object(bus, next_message_addr, msg.word0, msg.word1, data_to_transmit); // Replace with crc version
        tries = 0;
        while result.is_err() && (self.max_spi_crc_retries > 0 || tries < self.max_spi_crc_retries) {
            result = Instruction::write_tx_message_object(bus, next_message_addr, msg.word0, msg.word1, data_to_transmit);
            
            tries += 1;
        }
        result?;

        // attempt to set the UINC register to increments the fifo address
        result = Instruction::write_safe(bus, self.fifocon_addr + u12::from_u8(1), &[fifo_registers[1] | 0b0000_0001]);
        tries = 0;
        while result.is_err() && (self.max_spi_crc_retries > 0 || tries < self.max_spi_crc_retries) {
            result = Instruction::write_safe(bus, self.fifocon_addr + u12::from_u8(1), &[fifo_registers[1] | 0b0000_0001]);
            tries += 1;
        }
        result
    }

    /// Requests a send of the messages in the FIFO. Transmission attempts continue until:
    /// * The FIFO is empty (all messages transmitted)
    /// * The transmission request is aborted
    /// * FIFO is reset
    /// Uses crc to verify the spi message
    pub fn request_transmission_crc(&self, bus: &mut impl SpiDevice) -> Result<(), InstructionError> {

        // read fifo control, status, and address registers
        let mut fifocon_byte2 = [0u8; 1];
        self.read_fifocon_byte2_crc(bus, &mut fifocon_byte2)?;

        // if TXREQ is set, FIFO is alread transmitting
        if fifocon_byte2[0] & 0b0000_0010 != 0 {
            return Ok(());
        }

        // write fifo control register to request transmission
        self.write_fifocon_byte2_crc(bus, &[fifocon_byte2[0] | 0b0000_0010])
    }

    /// Requests a send of the messages in the FIFO. Transmission attempts continue until:
    /// * The FIFO is empty (all messages transmitted)
    /// * The transmission request is aborted
    /// * FIFO is reset
    /// Does NOT use crc to verify the spi message
    pub fn request_transmission(&self, bus: &mut impl SpiDevice) -> Result<(), InstructionError> {

        // read fifo control, status, and address registers
        let mut fifocon_byte2 = [0u8; 1];
        self.read_fifocon_byte2(bus, &mut fifocon_byte2)?;

        // if TXREQ is set, FIFO is alread transmitting
        if fifocon_byte2[0] & 0b0000_0010 != 0 {
            return Ok(());
        }

        // write fifo control register to request transmission
        self.write_fifocon_byte2(bus, &[fifocon_byte2[0] | 0b0000_0010])
    }

    /// Requests that the transmissions of the messages in the FIFO be aborted.
    /// Abortion only happens once the current message is finished transmitting.
    /// Does NOT use crc to verify the spi message
    pub fn request_abort(&self, bus: &mut impl SpiDevice) -> Result<(), InstructionError> {
        // read fifo control byte 2
        let mut fifocon_byte2 = [0u8; 1];
        self.read_fifocon_byte2(bus, &mut fifocon_byte2)?;

        // if TXREQ is not set, FIFO is not transmitting
        if fifocon_byte2[0] & 0b0000_0010 == 0 {
            return Ok(());
        }

        // write fifo control register to request abortion
        self.write_fifocon_byte2(bus, &[fifocon_byte2[0] & 0b1111_1101])
    }

    /// Requests that the transmissions of the messages in the FIFO be aborted.
    /// Abortion only happens once the current message is finished transmitting.
    /// Uses crc to verify the spi message
    pub fn request_abort_crc(&self, bus: &mut impl SpiDevice) -> Result<(), InstructionError> {
        // read fifo control byte 2
        let mut fifocon_byte2 = [0u8; 1];
        self.read_fifocon_byte2_crc(bus, &mut fifocon_byte2)?;

        // if TXREQ is not set, FIFO is not transmitting
        if fifocon_byte2[0] & 0b0000_0010 == 0 {
            return Ok(());
        }

        // write fifo control register to request abortion
        self.write_fifocon_byte2_crc(bus, &[fifocon_byte2[0] & 0b1111_1101])
    }

    fn read_fifocon_byte2(&self, bus: &mut impl SpiDevice, buf: &mut [u8; 1]) -> Result<(), InstructionError> {
        let mut result = Instruction::read(bus, self.fifocon_addr + u12::from_u8(1), buf);
        let mut tries = 0;
        while result.is_err() && (self.max_spi_retries > 0 || tries < self.max_spi_retries) {
            result = Instruction::read(bus, self.fifocon_addr + u12::from_u8(1), buf);
            tries += 1;
        }
        result
    }

    fn read_fifocon_byte2_crc(&self, bus: &mut impl SpiDevice, buf: &mut [u8; 1]) -> Result<(), InstructionError> {
        let mut result = Instruction::read_crc(bus, self.fifocon_addr + u12::from_u8(1), buf);
        let mut tries = 0;
        while result.is_err() && (self.max_spi_crc_retries > 0 || tries < self.max_spi_crc_retries) {
            result = Instruction::read_crc(bus, self.fifocon_addr + u12::from_u8(1), buf);
            tries += 1;
        }
        result
    }

    fn write_fifocon_byte2(&self, bus: &mut impl SpiDevice, buf: &[u8; 1]) -> Result<(), InstructionError> {
        let mut result = Instruction::write(bus, self.fifocon_addr + u12::from_u8(1), buf);
        let mut tries = 0;
        while result.is_err() && (self.max_spi_retries > 0 || tries < self.max_spi_retries) {
            result = Instruction::write(bus, self.fifocon_addr + u12::from_u8(1), buf);
            tries += 1;
        }
        result
    }

    fn write_fifocon_byte2_crc(&self, bus: &mut impl SpiDevice, buf: &[u8; 1]) -> Result<(), InstructionError> {
        let mut result = Instruction::write_safe(bus, self.fifocon_addr + u12::from_u8(1), buf);
        let mut tries = 0;
        while result.is_err() && (self.max_spi_crc_retries > 0 || tries < self.max_spi_crc_retries) {
            result = Instruction::write_safe(bus, self.fifocon_addr + u12::from_u8(1), buf);
            tries += 1;
        }
        result
    }
}