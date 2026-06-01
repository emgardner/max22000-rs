use bitfield::bitfield;

use crate::MaxError;

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct GenChnlCtrlRaw(u32);
    impl Debug;
    u8;
    pub ai1_test, set_ai1_test: 23, 22;
    pub ai2_test, set_ai2_test: 21, 20;
    pub ai3_test, set_ai3_test: 19, 18;
    pub ai4_test, set_ai4_test: 17, 16;
    pub ai5_test, set_ai5_test: 15, 14;
    pub ai6_test, set_ai6_test: 13, 12;
    pub ai_dchnl_sel, set_ai_dchnl_sel: 11, 8;
}

impl_u24_register!(GenChnlCtrlRaw);

impl GenChnlCtrlRaw {
    pub const RESET: Self = Self(0);

    pub fn ai1_test_config(self) -> Result<AiTestConfig, MaxError> {
        self.ai1_test().try_into()
    }

    pub fn set_ai1_test_config(&mut self, config: AiTestConfig) {
        self.set_ai1_test(config.into());
    }

    pub fn ai2_test_config(self) -> Result<AiTestConfig, MaxError> {
        self.ai2_test().try_into()
    }

    pub fn set_ai2_test_config(&mut self, config: AiTestConfig) {
        self.set_ai2_test(config.into());
    }

    pub fn ai3_test_config(self) -> Result<AiTestConfig, MaxError> {
        self.ai3_test().try_into()
    }

    pub fn set_ai3_test_config(&mut self, config: AiTestConfig) {
        self.set_ai3_test(config.into());
    }

    pub fn ai4_test_config(self) -> Result<AiTestConfig, MaxError> {
        self.ai4_test().try_into()
    }

    pub fn set_ai4_test_config(&mut self, config: AiTestConfig) {
        self.set_ai4_test(config.into());
    }

    pub fn ai5_test_config(self) -> Result<AiTestConfig, MaxError> {
        self.ai5_test().try_into()
    }

    pub fn set_ai5_test_config(&mut self, config: AiTestConfig) {
        self.set_ai5_test(config.into());
    }

    pub fn ai6_test_config(self) -> Result<AiTestConfig, MaxError> {
        self.ai6_test().try_into()
    }

    pub fn set_ai6_test_config(&mut self, config: AiTestConfig) {
        self.set_ai6_test(config.into());
    }

    pub fn adc_channel_selection(self) -> Result<AdcChannelSelection, MaxError> {
        self.ai_dchnl_sel().try_into()
    }

    pub fn set_adc_channel_selection(&mut self, selection: AdcChannelSelection) {
        self.set_ai_dchnl_sel(selection.into());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GenChnlCtrl {
    pub ai1_test: AiTestConfig,
    pub ai2_test: AiTestConfig,
    pub ai3_test: AiTestConfig,
    pub ai4_test: AiTestConfig,
    pub ai5_test: AiTestConfig,
    pub ai6_test: AiTestConfig,
    pub adc_channel_selection: AdcChannelSelection,
}

impl TryFrom<GenChnlCtrlRaw> for GenChnlCtrl {
    type Error = MaxError;

    fn try_from(raw: GenChnlCtrlRaw) -> Result<Self, Self::Error> {
        Ok(Self {
            ai1_test: raw.ai1_test_config()?,
            ai2_test: raw.ai2_test_config()?,
            ai3_test: raw.ai3_test_config()?,
            ai4_test: raw.ai4_test_config()?,
            ai5_test: raw.ai5_test_config()?,
            ai6_test: raw.ai6_test_config()?,
            adc_channel_selection: raw.adc_channel_selection()?,
        })
    }
}

impl TryFrom<&[u8]> for GenChnlCtrl {
    type Error = MaxError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        GenChnlCtrlRaw::try_from(data)?.try_into()
    }
}

impl From<GenChnlCtrl> for GenChnlCtrlRaw {
    fn from(config: GenChnlCtrl) -> Self {
        let mut raw = GenChnlCtrlRaw::RESET;
        raw.set_ai1_test_config(config.ai1_test);
        raw.set_ai2_test_config(config.ai2_test);
        raw.set_ai3_test_config(config.ai3_test);
        raw.set_ai4_test_config(config.ai4_test);
        raw.set_ai5_test_config(config.ai5_test);
        raw.set_ai6_test_config(config.ai6_test);
        raw.set_adc_channel_selection(config.adc_channel_selection);
        raw
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AiTestConfig {
    Disabled = 0b00,
    ResistorToAgnd = 0b01,
    ResistorToHvdd = 0b10,
    ResistorsToHvddAndAgnd = 0b11,
}

impl TryFrom<u8> for AiTestConfig {
    type Error = MaxError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b00 => Ok(Self::Disabled),
            0b01 => Ok(Self::ResistorToAgnd),
            0b10 => Ok(Self::ResistorToHvdd),
            0b11 => Ok(Self::ResistorsToHvddAndAgnd),
            value => Err(MaxError::InvalidRegisterValue { value }),
        }
    }
}

impl From<AiTestConfig> for u8 {
    fn from(config: AiTestConfig) -> Self {
        config as Self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AdcChannelSelection {
    None = 0b0000,
    Ai1SingleEnded = 0b0001,
    Ai2SingleEnded = 0b0010,
    Ai1Ai2Differential = 0b0011,
    Ai3SingleEnded = 0b0100,
    Ai4SingleEnded = 0b0101,
    Ai3Ai4Differential = 0b0110,
    Ai5Ai6Differential12_5V = 0b1001,
    Ai5Ai6DifferentialLowVoltage = 0b1100,
    Aux1SingleEnded = 0b1101,
    Aux2SingleEnded = 0b1110,
    Aux1Aux2Differential = 0b1111,
}

impl TryFrom<u8> for AdcChannelSelection {
    type Error = MaxError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b0000 => Ok(Self::None),
            0b0001 => Ok(Self::Ai1SingleEnded),
            0b0010 => Ok(Self::Ai2SingleEnded),
            0b0011 => Ok(Self::Ai1Ai2Differential),
            0b0100 => Ok(Self::Ai3SingleEnded),
            0b0101 => Ok(Self::Ai4SingleEnded),
            0b0110 => Ok(Self::Ai3Ai4Differential),
            0b1001 => Ok(Self::Ai5Ai6Differential12_5V),
            0b1100 => Ok(Self::Ai5Ai6DifferentialLowVoltage),
            0b1101 => Ok(Self::Aux1SingleEnded),
            0b1110 => Ok(Self::Aux2SingleEnded),
            0b1111 => Ok(Self::Aux1Aux2Differential),
            value => Err(MaxError::InvalidRegisterValue { value }),
        }
    }
}

impl From<AdcChannelSelection> for u8 {
    fn from(selection: AdcChannelSelection) -> Self {
        selection as Self
    }
}
