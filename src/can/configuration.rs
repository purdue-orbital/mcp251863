use embedded_hal::spi::{SpiDevice, Error};
use crate::registers::prelude::{
    OscillatorControl,
    IOControl,
    CANControl,
    OperationMode,
    NominalBitTimeConfiguration,
    DataBitTimeConfiguration,
    *,
};
use crate::can::tx::TransmitFIFO;
use crate::instruction::InstructionError;
use crate::MCP251863;

/// Configuration for 40MHz system clock
pub const CLK_40MHZ: OscillatorControl = OscillatorControl::new_with_raw_value(0x00_00_00_61);
/// Configuration for 20MHz system clock
pub const CLK_20MHZ: OscillatorControl = OscillatorControl::new_with_raw_value(0x00_00_00_71);

/// Configuration for IO pins as interrupts
pub const IO_PIN_INT: IOControl = IOControl::new_with_raw_value(0x00_00_00_03);

/// CAN Configuration Settings:
/// * 1 bit delay between consecutive transmissions
/// * Operation mode: Configuration mode
/// * Disabled TXQ
/// * Disabled TEF
/// * Infinite retransmission attempts
/// * Bit rate switching enabled
/// * System erros trigger listen only mode
/// * Disabled wake filter
/// * Non-ISO CRC
/// * Data bits not used for reception-filters
pub const CAN_BARE_MIN: CANControl = CANControl::new_with_raw_value(0x14_87_07_40);

/// Nominal Bit Time Configuration Settings:
/// * 500Kb/s nominal bitrate
// TODO: add more and change value
// TODO: add data bitrate settings & value
pub const NOMINAL_BITRATE_500K: NominalBitTimeConfiguration = NominalBitTimeConfiguration::new_with_raw_value(0x00_00_00_00); 
pub const DATA_BIT_RATE_4M: DataBitTimeConfiguration = DataBitTimeConfiguration::new_with_raw_value(0x00_00_00_00);

impl MCP251863 {

    /// Set operation mode to configuration mode
    pub fn set_configuration_mode(&mut self, bus: &mut impl SpiDevice) -> Result<(), InstructionError> {
        let mut result = self.set_operation_mode(bus, OperationMode::Config);
        let mut tries = 0;
        while result.is_err() && (self.max_spi_crc_retries > 0 || tries < self.max_spi_crc_retries) {
            result = self.set_operation_mode(bus, OperationMode::Config);
            tries += 1;
        }
        result
    }

    /// Configures the oscillator control register
    /// 
    /// # Example
    /// '''no_run
    /// // configure SPI connection
    /// let mut spi = Spi::new(/* ... */);
    /// 
    /// // instantiate MCP251863
    /// let mut mcp = MCP251863::new();
    /// 
    /// // configure oscillator
    /// // 40MHz or 20MHz recommended
    /// mcp.configure_osc(&mut spi, CLK_40MHZ).unwrap();
    /// '''
    pub fn configure_osc(&mut self, bus: &mut impl SpiDevice, osc_reg: OscillatorControl) -> Result<(), InstructionError> {
        if self.mode != OperationMode::Config {
            self.set_configuration_mode(bus)?;
        }

        let mut result = osc_reg.write_register_safe(bus, 0);
        let mut tries = 0;
        while result.is_err() && (self.max_spi_crc_retries > 0 || tries < self.max_spi_crc_retries) {
            result = osc_reg.write_register_safe(bus, 0);
            tries += 1;
        }

        result
    }

    /// Configures the io control register
    /// 
    /// # Example
    /// '''no_run
    /// // hardware dependent - configure SPI connection
    /// let mut spi = Spi::new(/* ... */);
    /// 
    /// // instantiate MCP251863
    /// let mut mcp = MCP251863::new();
    /// 
    /// // configure io pins
    /// // pins configured as interrupts
    /// mcp.configure_io(&mut spi, IO_PIN_INT).unwrap();
    /// '''
    pub fn configure_io(&mut self, bus: &mut impl SpiDevice, iocon_reg: IOControl) -> Result<(), InstructionError> {
        if self.mode != OperationMode::Config {
            self.set_configuration_mode(bus)?;
        }

        let mut result = iocon_reg.write_register_crc(bus);
        let mut tries = 0;
        while result.is_err() && (self.max_spi_crc_retries > 0 || tries < self.max_spi_crc_retries) {
            result = iocon_reg.write_register_crc(bus);
            tries += 1;
        }

        result
    }

    /// Configures the CAN control register
    /// 
    /// # Example
    /// '''no_run
    /// // hardware dependent - configure SPI connection
    /// let mut spi = Spi::new(/* ... */);
    /// 
    /// // instantiate MCP251863
    /// let mut mcp = MCP251863::new();
    /// 
    /// // configure can control
    /// // pins configured as interrupts
    /// mcp.configure_can(&mut spi, CAN_BARE_MIN).unwrap();
    /// '''
    pub fn configure_can(&mut self, bus: &mut impl SpiDevice, cancon_reg: CANControl) -> Result<(), InstructionError> {
        if self.mode != OperationMode::Config {
            self.set_configuration_mode(bus)?;
        }

        let mut result = cancon_reg.write_register_crc(bus);
        let mut tries = 0;
        while result.is_err() && (self.max_spi_crc_retries > 0 || tries < self.max_spi_crc_retries) {
            result = cancon_reg.write_register_crc(bus);
            tries += 1;
        }

        result
    }

    /// Configures the CAN control register
    /// 
    /// # Example
    /// '''no_run
    /// // hardware dependent - configure SPI connection
    /// let mut spi = Spi::new(/* ... */);
    /// 
    /// // instantiate MCP251863
    /// let mut mcp = MCP251863::new();
    /// 
    /// // configure bit time
    /// // this uses 500k nominal bitrate and 4M data bitrate
    /// mcp.configure_bit_time(&mut spi, NOMINAL_BITRATE_500K, DATA_BITRATE_4M).unwrap();
    /// '''
    pub fn configure_bit_time(&mut self, bus: &mut impl SpiDevice, nbtcfg_reg: NominalBitTimeConfiguration, dbtcfg_reg: DataBitTimeConfiguration) -> Result<(), InstructionError> {
        if self.mode != OperationMode::Config {
            self.set_configuration_mode(bus)?;
        }

        let mut result = nbtcfg_reg.write_register_crc(bus);
        let mut tries = 0;
        while result.is_err() && (self.max_spi_crc_retries > 0 || tries < self.max_spi_crc_retries) {
            result = nbtcfg_reg.write_register_crc(bus);
            tries += 1;
        }

        result = dbtcfg_reg.write_register_crc(bus);
        tries = 0;
        while result.is_err() && (self.max_spi_crc_retries > 0 || tries < self.max_spi_crc_retries) {
            result = dbtcfg_reg.write_register_crc(bus);
            tries += 1;
        }

        result
    }

    /// configures nominal bit time separately from data bit time
    /// see also `configure_bit_time`
    pub fn configure_nominal_bit_time(&mut self, bus: &mut impl SpiDevice, nbtcfg_reg: NominalBitTimeConfiguration) -> Result<(), InstructionError> {
        if self.mode != OperationMode::Config {
            self.set_configuration_mode(bus)?;
        }

        let mut result = nbtcfg_reg.write_register_crc(bus);
        let mut tries = 0;
        while result.is_err() && (self.max_spi_crc_retries > 0 || tries < self.max_spi_crc_retries) {
            result = nbtcfg_reg.write_register_crc(bus);
            tries += 1;
        }

        result
    }

    /// configures data bit time separately from nominal bit time
    /// see also `configure_bit_time`
    pub fn configure_data_bit_time(&mut self, bus: &mut impl SpiDevice, dbtcfg_reg: DataBitTimeConfiguration) -> Result<(), InstructionError> {
        if self.mode != OperationMode::Config {
            self.set_configuration_mode(bus)?;
        }

        let mut result = dbtcfg_reg.write_register_crc(bus);
        let mut tries = 0;
        while result.is_err() && (self.max_spi_crc_retries > 0 || tries < self.max_spi_crc_retries) {
            result = dbtcfg_reg.write_register_crc(bus);
            tries += 1;
        }

        result
    }

    /// Configures fifo specified by 'channel' as a transmit fifo
    /// If self.max_spi_crc_retries == -1 (infinite retries) it is safe to unwrap
    pub fn new_transmit_fifo(&mut self, bus: &mut impl SpiDevice, n: u8, fifocon_reg: FIFOControl) -> Result<TransmitFIFO, InstructionError> {
        if self.mode != OperationMode::Config {
            self.set_configuration_mode(bus)?;
        }

        let mut result = TransmitFIFO::new(bus, n, fifocon_reg);
        let mut tries = 0;
        while result.is_err() && (self.max_spi_crc_retries > 0 || tries < self.max_spi_crc_retries) {
            result = TransmitFIFO::new(bus, n, fifocon_reg);
            tries += 1;
        }

        result
    }
}