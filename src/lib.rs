pub mod data;

use thiserror::Error;

const CRC8: crc::Crc<u8> = crc::Crc::<u8>::new(&crc::CRC_8_MAXIM_DOW);

pub struct Max22000 {
    crc_enabled: bool,
}

#[derive(Debug, Clone, Copy, Error)]
pub enum MaxError {
    #[error("Crc Error")]
    CrcError,
    #[error("Invalid data length: expected {expected} bytes, got {actual}")]
    InvalidDataLength { expected: usize, actual: usize },
    #[error("Invalid register value: {value}")]
    InvalidRegisterValue { value: u8 },
    #[error("AdcOverflow")]
    AdcOverflow,
    #[error("AdcUnderflow")]
    AdcUnderflow,
}

const ADC_MAX: u32 = 0x7F_FF_FF;
const ADC_MIN: u32 = 0x80_00_00;

pub type MaxResult<T> = Result<T, MaxError>;

fn check_adc_value(raw_value: u32) -> MaxResult<u32> {
    if raw_value == ADC_MAX {
        Err(MaxError::AdcOverflow)
    } else if raw_value == ADC_MIN {
        Err(MaxError::AdcUnderflow)
    } else {
        Ok(raw_value)
    }
}

impl Max22000 {
    pub fn new() -> Self {
        Self { crc_enabled: false }
    }

    pub fn crc_enabled(&self) -> bool {
        self.crc_enabled
    }

    pub fn enable_crc(&mut self) -> MaxResult<()> {
        // TODO
        Ok(())
    }

    pub fn set_crc(&mut self, enabled: bool) {
        self.crc_enabled = enabled;
    }

    pub fn check_crc(&mut self) -> MaxResult<()> {
        // TODO
        Ok(())
    }

    pub fn crc_buffer(bytes: &[u8]) -> u8 {
        CRC8.checksum(bytes)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Read(data::Registers),
    Write(data::Registers),
}

impl Operation {
    pub fn as_u8(&self) -> u8 {
        match self {
            Self::Read(register) => (*register as u8) << 1,
            Self::Write(register) => (*register as u8) << 1 | 0x01,
        }
    }
}
