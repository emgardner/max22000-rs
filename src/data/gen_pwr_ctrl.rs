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
