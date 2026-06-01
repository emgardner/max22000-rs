use bitfield::bitfield;

use super::DataRate;
use crate::MaxError;

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct DChnlCmdRaw(u32);
    impl Debug;
    u8;
    pub dchnl_mode, set_dchnl_mode: 21, 20;
    pub dchnl_rate, set_dchnl_rate: 19, 16;
}

impl_u24_register!(DChnlCmdRaw);

impl DChnlCmdRaw {
    pub const RESET: Self = Self(0x10_00_00);

    pub fn mode(self) -> Result<DChnlMode, MaxError> {
        self.dchnl_mode().try_into()
    }

    pub fn set_mode(&mut self, mode: DChnlMode) {
        self.set_dchnl_mode(mode.into());
    }

    pub fn data_rate(self) -> Result<DataRate, MaxError> {
        self.dchnl_rate().try_into()
    }

    pub fn set_data_rate(&mut self, rate: DataRate) {
        self.set_dchnl_rate(rate.into());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DChnlCmd {
    pub mode: DChnlMode,
    pub data_rate: DataRate,
}

impl TryFrom<DChnlCmdRaw> for DChnlCmd {
    type Error = MaxError;

    fn try_from(raw: DChnlCmdRaw) -> Result<Self, Self::Error> {
        Ok(Self {
            mode: raw.mode()?,
            data_rate: raw.data_rate()?,
        })
    }
}

impl TryFrom<&[u8]> for DChnlCmd {
    type Error = MaxError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        DChnlCmdRaw::try_from(data)?.try_into()
    }
}

impl From<DChnlCmd> for DChnlCmdRaw {
    fn from(command: DChnlCmd) -> Self {
        let mut raw = DChnlCmdRaw::RESET;
        raw.set_mode(command.mode);
        raw.set_data_rate(command.data_rate);
        raw
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DChnlMode {
    PowerDown = 0b01,
    Conversion = 0b11,
}

impl TryFrom<u8> for DChnlMode {
    type Error = MaxError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b01 => Ok(Self::PowerDown),
            0b11 => Ok(Self::Conversion),
            value => Err(MaxError::InvalidRegisterValue { value }),
        }
    }
}

impl From<DChnlMode> for u8 {
    fn from(mode: DChnlMode) -> Self {
        mode as Self
    }
}
