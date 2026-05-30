use bitfield::bitfield;

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct DChnlCtrl2Raw(u32);
    impl Debug;
    pub extclk, set_extclk: 23;
    pub sync_mode, set_sync_mode: 21;
    pub nosysg, set_nosysg: 19;
    pub nosyso, set_nosyso: 18;
}

impl_u24_register!(DChnlCtrl2Raw);

impl DChnlCtrl2Raw {
    pub const RESET: Self = Self(0);

    pub fn clock_source(self) -> AdcClockSource {
        self.extclk().into()
    }

    pub fn set_clock_source(&mut self, source: AdcClockSource) {
        self.set_extclk(source.into());
    }

    pub fn external_sync(self) -> ExternalSync {
        self.sync_mode().into()
    }

    pub fn set_external_sync(&mut self, sync: ExternalSync) {
        self.set_sync_mode(sync.into());
    }

    pub fn gain_correction(self) -> CalibrationCorrection {
        self.nosysg().into()
    }

    pub fn set_gain_correction(&mut self, correction: CalibrationCorrection) {
        self.set_nosysg(correction.into());
    }

    pub fn offset_correction(self) -> CalibrationCorrection {
        self.nosyso().into()
    }

    pub fn set_offset_correction(&mut self, correction: CalibrationCorrection) {
        self.set_nosyso(correction.into());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AdcClockSource {
    InternalOscillator = 0,
    ExternalClock = 1,
}

impl From<bool> for AdcClockSource {
    fn from(external: bool) -> Self {
        if external {
            Self::ExternalClock
        } else {
            Self::InternalOscillator
        }
    }
}

impl From<AdcClockSource> for bool {
    fn from(source: AdcClockSource) -> Self {
        matches!(source, AdcClockSource::ExternalClock)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ExternalSync {
    Disabled = 0,
    SyncPin = 1,
}

impl From<bool> for ExternalSync {
    fn from(enabled: bool) -> Self {
        if enabled {
            Self::SyncPin
        } else {
            Self::Disabled
        }
    }
}

impl From<ExternalSync> for bool {
    fn from(sync: ExternalSync) -> Self {
        matches!(sync, ExternalSync::SyncPin)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CalibrationCorrection {
    Enabled = 0,
    Bypassed = 1,
}

impl From<bool> for CalibrationCorrection {
    fn from(bypassed: bool) -> Self {
        if bypassed {
            Self::Bypassed
        } else {
            Self::Enabled
        }
    }
}

impl From<CalibrationCorrection> for bool {
    fn from(correction: CalibrationCorrection) -> Self {
        matches!(correction, CalibrationCorrection::Bypassed)
    }
}
