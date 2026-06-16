use bitfield::bitfield;

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct AoGainCorrectionWrRaw(u32);
    impl Debug;
    u32;
    pub ao_gain_w, set_ao_gain_w: 23, 6;
}

impl_u24_register!(AoGainCorrectionWrRaw);

impl AoGainCorrectionWrRaw {
    pub const RESET: Self = Self(0xff_ff_c0);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AoGainCorrectionWr {
    pub code: u32,
}

impl AoGainCorrectionWr {
    pub const MIN_CODE: u32 = 0;
    pub const MAX_CODE: u32 = 0x0003_ffff;
    pub const UNITY: Self = Self {
        code: Self::MAX_CODE,
    };

    /// Returns the Table 7 gain decimal, `(AO_GAIN_W + 1) / 2^18`.
    pub fn gain(self) -> f64 {
        (self.code.min(Self::MAX_CODE) + 1) as f64 / 262_144.0
    }
}

impl From<AoGainCorrectionWrRaw> for AoGainCorrectionWr {
    fn from(raw: AoGainCorrectionWrRaw) -> Self {
        Self {
            code: raw.ao_gain_w(),
        }
    }
}

impl TryFrom<&[u8]> for AoGainCorrectionWr {
    type Error = crate::MaxError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        Ok(AoGainCorrectionWrRaw::try_from(data)?.into())
    }
}

impl From<AoGainCorrectionWr> for AoGainCorrectionWrRaw {
    fn from(gain: AoGainCorrectionWr) -> Self {
        let mut raw = AoGainCorrectionWrRaw::RESET;
        raw.set_ao_gain_w(gain.code.min(AoGainCorrectionWr::MAX_CODE));
        raw
    }
}
