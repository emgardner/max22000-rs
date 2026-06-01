use bitfield::bitfield;

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct AoCnfgWrRaw(u32);
    impl Debug;
    pub ao_rb_en, set_ao_rb_en: 19;
}

impl_u24_register!(AoCnfgWrRaw);

impl AoCnfgWrRaw {
    pub const RESET: Self = Self(0);

    pub fn readback(self) -> AnalogOutReadback {
        self.ao_rb_en().into()
    }

    pub fn set_readback(&mut self, readback: AnalogOutReadback) {
        self.set_ao_rb_en(readback.into());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AoConfig {
    pub readback: AnalogOutReadback,
}

impl From<AoCnfgWrRaw> for AoConfig {
    fn from(raw: AoCnfgWrRaw) -> Self {
        Self {
            readback: raw.readback(),
        }
    }
}

impl TryFrom<&[u8]> for AoConfig {
    type Error = crate::MaxError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        Ok(AoCnfgWrRaw::try_from(data)?.into())
    }
}

impl From<AoConfig> for AoCnfgWrRaw {
    fn from(config: AoConfig) -> Self {
        let mut raw = AoCnfgWrRaw::RESET;
        raw.set_readback(config.readback);
        raw
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AnalogOutReadback {
    Disabled = 0,
    Enabled = 1,
}

impl From<bool> for AnalogOutReadback {
    fn from(enabled: bool) -> Self {
        if enabled {
            Self::Enabled
        } else {
            Self::Disabled
        }
    }
}

impl From<AnalogOutReadback> for bool {
    fn from(readback: AnalogOutReadback) -> Self {
        matches!(readback, AnalogOutReadback::Enabled)
    }
}
