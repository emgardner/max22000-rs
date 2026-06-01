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
        raw.set_ao_gain_w(gain.code & 0x0003_ffff);
        raw
    }
}
