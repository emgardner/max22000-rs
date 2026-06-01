use bitfield::bitfield;

use super::sign_extend;

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct DChnlNSocRaw(u32);
    impl Debug;
    u32;
    pub dchnl_n_soc, set_dchnl_n_soc: 23, 0;
}

impl_u24_register!(DChnlNSocRaw);

impl DChnlNSocRaw {
    pub const RESET: Self = Self(0);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DChnlNSoc {
    pub code: i32,
}

impl From<DChnlNSocRaw> for DChnlNSoc {
    fn from(raw: DChnlNSocRaw) -> Self {
        Self {
            code: sign_extend(raw.dchnl_n_soc(), 24),
        }
    }
}

impl TryFrom<&[u8]> for DChnlNSoc {
    type Error = crate::MaxError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        Ok(DChnlNSocRaw::try_from(data)?.into())
    }
}

impl From<DChnlNSoc> for DChnlNSocRaw {
    fn from(offset: DChnlNSoc) -> Self {
        let mut raw = DChnlNSocRaw::RESET;
        raw.set_dchnl_n_soc(offset.code as u32 & 0x00ff_ffff);
        raw
    }
}
