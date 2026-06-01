use bitfield::bitfield;

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct DChnlNSgcRaw(u32);
    impl Debug;
    u32;
    pub dchnl_n_sgc, set_dchnl_n_sgc: 23, 0;
}

impl_u24_register!(DChnlNSgcRaw);

impl DChnlNSgcRaw {
    pub const RESET: Self = Self(0xc0_00_00);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DChnlNSgc {
    pub code: u32,
}

impl From<DChnlNSgcRaw> for DChnlNSgc {
    fn from(raw: DChnlNSgcRaw) -> Self {
        Self {
            code: raw.dchnl_n_sgc(),
        }
    }
}

impl TryFrom<&[u8]> for DChnlNSgc {
    type Error = crate::MaxError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        Ok(DChnlNSgcRaw::try_from(data)?.into())
    }
}

impl From<DChnlNSgc> for DChnlNSgcRaw {
    fn from(gain: DChnlNSgc) -> Self {
        let mut raw = DChnlNSgcRaw::RESET;
        raw.set_dchnl_n_sgc(gain.code & 0x00ff_ffff);
        raw
    }
}
