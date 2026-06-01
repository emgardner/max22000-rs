use bitfield::bitfield;

use super::DataRate;
use crate::MaxError;

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct DChnlStatRaw(u32);
    impl Debug;
    pub refdet, _: 14;
    pub dor, _: 9;
    pub aor, _: 8;
    u8;
    pub rate, _: 7, 4;
    pub pdstat, _: 3, 2;
    pub mstat, _: 1;
    pub rdy, _: 0;
}

impl_u24_register!(DChnlStatRaw);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DChnlStatus {
    pub reference_detected: bool,
    pub digital_overrange: bool,
    pub analog_overrange: bool,
    pub data_rate: DataRate,
    pub power_down_status: AdcPowerDownStatus,
    pub modulator_converting: bool,
    pub ready: bool,
}

impl TryFrom<DChnlStatRaw> for DChnlStatus {
    type Error = MaxError;

    fn try_from(raw: DChnlStatRaw) -> Result<Self, Self::Error> {
        Ok(Self {
            reference_detected: raw.refdet(),
            digital_overrange: raw.dor(),
            analog_overrange: raw.aor(),
            data_rate: raw.rate().try_into()?,
            power_down_status: raw.pdstat().try_into()?,
            modulator_converting: raw.mstat(),
            ready: raw.rdy(),
        })
    }
}

impl TryFrom<&[u8]> for DChnlStatus {
    type Error = MaxError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        DChnlStatRaw::try_from(data)?.try_into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AdcPowerDownStatus {
    Converting = 0b00,
    Standby = 0b10,
    Reset = 0b11,
}

impl TryFrom<u8> for AdcPowerDownStatus {
    type Error = MaxError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b00 => Ok(Self::Converting),
            0b10 => Ok(Self::Standby),
            0b11 => Ok(Self::Reset),
            value => Err(MaxError::InvalidRegisterValue { value }),
        }
    }
}
