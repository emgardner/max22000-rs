use bitfield::bitfield;

use crate::MaxError;

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct GenConfigRaw(u32);
    impl Debug;
    pub crc_en, set_crc_en: 23;
    pub dacref_sel, set_dacref_sel: 22;
    pub adcref_sel, set_adcref_sel: 21;
    pub line_cnfg, set_line_cnfg: 20;
    u8;
    pub ao_cnfg, set_ao_cnfg: 19, 16;
    pub ai1_2_cnfg, set_ai1_2_cnfg: 15, 13;
    pub ai3_cnfg, set_ai3_cnfg: 12;
    pub ai4_cnfg, set_ai4_cnfg: 11;
    pub ai5_6_cnfg, set_ai5_6_cnfg: 10, 8;
    pub ai5_df_gain, set_ai5_df_gain: 7, 6;
    pub ovc_ctrl, set_ovc_ctrl: 3;
}

impl_u24_register!(GenConfigRaw);

impl GenConfigRaw {
    pub const RESET: Self = Self(0x10_00_00);

    pub fn analog_out_config(self) -> Result<AnalogOutConfig, MaxError> {
        self.ao_cnfg().try_into()
    }

    pub fn set_analog_out_config(&mut self, config: AnalogOutConfig) {
        self.set_ao_cnfg(config.into());
    }

    pub fn ai1_2_config(self) -> Result<Ai1_2Config, MaxError> {
        self.ai1_2_cnfg().try_into()
    }

    pub fn set_ai1_2_config(&mut self, config: Ai1_2Config) {
        self.set_ai1_2_cnfg(config.into());
    }

    pub fn ai3_config(self) -> Ai3Config {
        self.ai3_cnfg().into()
    }

    pub fn set_ai3_config(&mut self, config: Ai3Config) {
        self.set_ai3_cnfg(config.into());
    }

    pub fn ai4_config(self) -> Ai4Config {
        self.ai4_cnfg().into()
    }

    pub fn set_ai4_config(&mut self, config: Ai4Config) {
        self.set_ai4_cnfg(config.into());
    }

    pub fn ai5_6_config(self) -> Result<Ai5_6Config, MaxError> {
        self.ai5_6_cnfg().try_into()
    }

    pub fn set_ai5_6_config(&mut self, config: Ai5_6Config) {
        self.set_ai5_6_cnfg(config.into());
    }

    pub fn ai5_differential_gain(self) -> Result<Ai5DifferentialGain, MaxError> {
        self.ai5_df_gain().try_into()
    }

    pub fn set_ai5_differential_gain(&mut self, gain: Ai5DifferentialGain) {
        self.set_ai5_df_gain(gain.into());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GenConfig {
    pub crc_en: bool,
    pub dac_ref_sel: bool,
    pub adc_ref_sel: bool,
    pub line_cnfg: bool,
    pub ao_cnfg: AnalogOutConfig,
    pub ai1_2_config: Ai1_2Config,
    pub ai3_config: Ai3Config,
    pub ai4_config: Ai4Config,
    pub ai5_6_config: Ai5_6Config,
    pub ai5_df_gain: Ai5DifferentialGain,
    pub ovc_ctrl: bool,
}

impl TryFrom<GenConfigRaw> for GenConfig {
    type Error = MaxError;

    fn try_from(raw: GenConfigRaw) -> Result<Self, Self::Error> {
        Ok(Self {
            crc_en: raw.crc_en(),
            dac_ref_sel: raw.dacref_sel(),
            adc_ref_sel: raw.adcref_sel(),
            line_cnfg: raw.line_cnfg(),
            ao_cnfg: raw.analog_out_config()?,
            ai1_2_config: raw.ai1_2_config()?,
            ai3_config: raw.ai3_config(),
            ai4_config: raw.ai4_config(),
            ai5_6_config: raw.ai5_6_config()?,
            ai5_df_gain: raw.ai5_differential_gain()?,
            ovc_ctrl: raw.ovc_ctrl(),
        })
    }
}

impl TryFrom<&[u8]> for GenConfig {
    type Error = MaxError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        GenConfigRaw::try_from(data)?.try_into()
    }
}

impl From<GenConfig> for GenConfigRaw {
    fn from(config: GenConfig) -> Self {
        let mut raw = GenConfigRaw::RESET;
        raw.set_crc_en(config.crc_en);
        raw.set_dacref_sel(config.dac_ref_sel);
        raw.set_adcref_sel(config.adc_ref_sel);
        raw.set_line_cnfg(config.line_cnfg);
        raw.set_analog_out_config(config.ao_cnfg);
        raw.set_ai1_2_config(config.ai1_2_config);
        raw.set_ai3_config(config.ai3_config);
        raw.set_ai4_config(config.ai4_config);
        raw.set_ai5_6_config(config.ai5_6_config);
        raw.set_ai5_differential_gain(config.ai5_df_gain);
        raw.set_ovc_ctrl(config.ovc_ctrl);
        raw
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AnalogOutConfig {
    HighImpedance = 0b0000,
    AnalogOut25V = 0b0001,
    AnalogOut12_5V = 0b0010,
    AnalogOut25ma = 0b0110,
    AnalogOut2_5ma = 0b1000,
}

impl TryFrom<u8> for AnalogOutConfig {
    type Error = MaxError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b0000 => Ok(Self::HighImpedance),
            0b0001 => Ok(Self::AnalogOut25V),
            0b0010 => Ok(Self::AnalogOut12_5V),
            0b0110 => Ok(Self::AnalogOut25ma),
            0b1000 => Ok(Self::AnalogOut2_5ma),
            value => Err(MaxError::InvalidRegisterValue { value }),
        }
    }
}

impl From<AnalogOutConfig> for u8 {
    fn from(config: AnalogOutConfig) -> Self {
        config as Self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Ai1_2Config {
    PoweredDown = 0b000,
    Ai1SingleEnded = 0b001,
    Ai2SingleEnded = 0b010,
    BothSingleEnded = 0b011,
    DifferentialPairCsa = 0b100,
}

impl TryFrom<u8> for Ai1_2Config {
    type Error = MaxError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b000 => Ok(Self::PoweredDown),
            0b001 => Ok(Self::Ai1SingleEnded),
            0b010 => Ok(Self::Ai2SingleEnded),
            0b011 => Ok(Self::BothSingleEnded),
            0b100 => Ok(Self::DifferentialPairCsa),
            value => Err(MaxError::InvalidRegisterValue { value }),
        }
    }
}

impl From<Ai1_2Config> for u8 {
    fn from(config: Ai1_2Config) -> Self {
        config as Self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Ai3Config {
    PoweredDown = 0,
    SingleEnded = 1,
}

impl From<bool> for Ai3Config {
    fn from(enabled: bool) -> Self {
        if enabled {
            Self::SingleEnded
        } else {
            Self::PoweredDown
        }
    }
}

impl From<Ai3Config> for bool {
    fn from(config: Ai3Config) -> Self {
        matches!(config, Ai3Config::SingleEnded)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Ai4Config {
    PoweredDown = 0,
    SingleEnded = 1,
}

impl From<bool> for Ai4Config {
    fn from(enabled: bool) -> Self {
        if enabled {
            Self::SingleEnded
        } else {
            Self::PoweredDown
        }
    }
}

impl From<Ai4Config> for bool {
    fn from(config: Ai4Config) -> Self {
        matches!(config, Ai4Config::SingleEnded)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Ai5_6Config {
    PoweredDown = 0b000,
    DifferentialPairPga = 0b100,
}

impl TryFrom<u8> for Ai5_6Config {
    type Error = MaxError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b000 => Ok(Self::PoweredDown),
            0b100 => Ok(Self::DifferentialPairPga),
            value => Err(MaxError::InvalidRegisterValue { value }),
        }
    }
}

impl From<Ai5_6Config> for u8 {
    fn from(config: Ai5_6Config) -> Self {
        config as Self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Ai5DifferentialGain {
    DefaultRange = 0b00,
    Range500Mv = 0b01,
    Range250Mv = 0b10,
    Range125Mv = 0b11,
}

impl TryFrom<u8> for Ai5DifferentialGain {
    type Error = MaxError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b00 => Ok(Self::DefaultRange),
            0b01 => Ok(Self::Range500Mv),
            0b10 => Ok(Self::Range250Mv),
            0b11 => Ok(Self::Range125Mv),
            value => Err(MaxError::InvalidRegisterValue { value }),
        }
    }
}

impl From<Ai5DifferentialGain> for u8 {
    fn from(gain: Ai5DifferentialGain) -> Self {
        gain as Self
    }
}
