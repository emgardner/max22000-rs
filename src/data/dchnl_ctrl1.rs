use bitfield::bitfield;

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct DChnlCtrl1Raw(u32);
    impl Debug;
    pub dchnl_pd, set_dchnl_pd: 20;
    pub scycle, set_scycle: 17;
    pub contsc, set_contsc: 16;
}

impl_u24_register!(DChnlCtrl1Raw);

impl DChnlCtrl1Raw {
    pub const RESET: Self = Self(0x02_00_00);

    pub fn power_down_state(self) -> AdcPowerDownState {
        self.dchnl_pd().into()
    }

    pub fn set_power_down_state(&mut self, state: AdcPowerDownState) {
        self.set_dchnl_pd(state.into());
    }

    pub fn conversion_cycle_mode(self) -> ConversionCycleMode {
        self.scycle().into()
    }

    pub fn set_conversion_cycle_mode(&mut self, mode: ConversionCycleMode) {
        self.set_scycle(mode.into());
    }

    pub fn single_cycle_run_mode(self) -> SingleCycleRunMode {
        self.contsc().into()
    }

    pub fn set_single_cycle_run_mode(&mut self, mode: SingleCycleRunMode) {
        self.set_contsc(mode.into());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DChnlCtrl1 {
    pub power_down_state: AdcPowerDownState,
    pub conversion_cycle_mode: ConversionCycleMode,
    pub single_cycle_run_mode: SingleCycleRunMode,
}

impl From<DChnlCtrl1Raw> for DChnlCtrl1 {
    fn from(raw: DChnlCtrl1Raw) -> Self {
        Self {
            power_down_state: raw.power_down_state(),
            conversion_cycle_mode: raw.conversion_cycle_mode(),
            single_cycle_run_mode: raw.single_cycle_run_mode(),
        }
    }
}

impl TryFrom<&[u8]> for DChnlCtrl1 {
    type Error = crate::MaxError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        Ok(DChnlCtrl1Raw::try_from(data)?.into())
    }
}

impl From<DChnlCtrl1> for DChnlCtrl1Raw {
    fn from(config: DChnlCtrl1) -> Self {
        let mut raw = DChnlCtrl1Raw::RESET;
        raw.set_power_down_state(config.power_down_state);
        raw.set_conversion_cycle_mode(config.conversion_cycle_mode);
        raw.set_single_cycle_run_mode(config.single_cycle_run_mode);
        raw
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AdcPowerDownState {
    Standby = 0,
    Reset = 1,
}

impl From<bool> for AdcPowerDownState {
    fn from(reset: bool) -> Self {
        if reset { Self::Reset } else { Self::Standby }
    }
}

impl From<AdcPowerDownState> for bool {
    fn from(state: AdcPowerDownState) -> Self {
        matches!(state, AdcPowerDownState::Reset)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ConversionCycleMode {
    Continuous = 0,
    SingleCycle = 1,
}

impl From<bool> for ConversionCycleMode {
    fn from(single_cycle: bool) -> Self {
        if single_cycle {
            Self::SingleCycle
        } else {
            Self::Continuous
        }
    }
}

impl From<ConversionCycleMode> for bool {
    fn from(mode: ConversionCycleMode) -> Self {
        matches!(mode, ConversionCycleMode::SingleCycle)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SingleCycleRunMode {
    SingleConversion = 0,
    Continuous = 1,
}

impl From<bool> for SingleCycleRunMode {
    fn from(continuous: bool) -> Self {
        if continuous {
            Self::Continuous
        } else {
            Self::SingleConversion
        }
    }
}

impl From<SingleCycleRunMode> for bool {
    fn from(mode: SingleCycleRunMode) -> Self {
        matches!(mode, SingleCycleRunMode::Continuous)
    }
}
