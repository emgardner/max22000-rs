use bitfield::bitfield;

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct AoGainCorrectionRdRaw(u32);
    impl Debug;
    u32;
    pub ao_gain_r, _: 17, 0;
}

impl_u24_register!(AoGainCorrectionRdRaw);

impl AoGainCorrectionRdRaw {
    pub const RESET: Self = Self(0x03_ff_ff);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AoGainCorrectionRd {
    pub code: u32,
}

impl From<AoGainCorrectionRdRaw> for AoGainCorrectionRd {
    fn from(raw: AoGainCorrectionRdRaw) -> Self {
        Self {
            code: raw.ao_gain_r(),
        }
    }
}

impl TryFrom<&[u8]> for AoGainCorrectionRd {
    type Error = crate::MaxError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        Ok(AoGainCorrectionRdRaw::try_from(data)?.into())
    }
}
