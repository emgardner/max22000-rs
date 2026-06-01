use bitfield::bitfield;

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct GenPwrCtrlRaw(u32);
    impl Debug;
    pub aodac_pd, set_aodac_pd: 23;
    pub aodac_rst, set_aodac_rst: 21;
    pub gen_pd, set_gen_pd: 19;
    pub gen_rst, set_gen_rst: 17;
}

impl_u24_register!(GenPwrCtrlRaw);

impl GenPwrCtrlRaw {
    pub const RESET: Self = Self(0);

    pub fn aodac_power_state(self) -> PowerState {
        self.aodac_pd().into()
    }

    pub fn set_aodac_power_state(&mut self, state: PowerState) {
        self.set_aodac_pd(state.into());
    }

    pub fn aodac_reset_state(self) -> ResetState {
        self.aodac_rst().into()
    }

    pub fn set_aodac_reset_state(&mut self, state: ResetState) {
        self.set_aodac_rst(state.into());
    }

    pub fn general_power_state(self) -> PowerState {
        self.gen_pd().into()
    }

    pub fn set_general_power_state(&mut self, state: PowerState) {
        self.set_gen_pd(state.into());
    }

    pub fn general_reset_state(self) -> ResetState {
        self.gen_rst().into()
    }

    pub fn set_general_reset_state(&mut self, state: ResetState) {
        self.set_gen_rst(state.into());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GenPwrCtrl {
    pub aodac_power_state: PowerState,
    pub aodac_reset_state: ResetState,
    pub general_power_state: PowerState,
    pub general_reset_state: ResetState,
}

impl From<GenPwrCtrlRaw> for GenPwrCtrl {
    fn from(raw: GenPwrCtrlRaw) -> Self {
        Self {
            aodac_power_state: raw.aodac_power_state(),
            aodac_reset_state: raw.aodac_reset_state(),
            general_power_state: raw.general_power_state(),
            general_reset_state: raw.general_reset_state(),
        }
    }
}

impl TryFrom<&[u8]> for GenPwrCtrl {
    type Error = crate::MaxError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        Ok(GenPwrCtrlRaw::try_from(data)?.into())
    }
}

impl From<GenPwrCtrl> for GenPwrCtrlRaw {
    fn from(config: GenPwrCtrl) -> Self {
        let mut raw = GenPwrCtrlRaw::RESET;
        raw.set_aodac_power_state(config.aodac_power_state);
        raw.set_aodac_reset_state(config.aodac_reset_state);
        raw.set_general_power_state(config.general_power_state);
        raw.set_general_reset_state(config.general_reset_state);
        raw
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PowerState {
    Normal = 0,
    PoweredDown = 1,
}

impl From<bool> for PowerState {
    fn from(powered_down: bool) -> Self {
        if powered_down {
            Self::PoweredDown
        } else {
            Self::Normal
        }
    }
}

impl From<PowerState> for bool {
    fn from(state: PowerState) -> Self {
        matches!(state, PowerState::PoweredDown)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ResetState {
    Normal = 0,
    Reset = 1,
}

impl From<bool> for ResetState {
    fn from(reset: bool) -> Self {
        if reset { Self::Reset } else { Self::Normal }
    }
}

impl From<ResetState> for bool {
    fn from(state: ResetState) -> Self {
        matches!(state, ResetState::Reset)
    }
}
