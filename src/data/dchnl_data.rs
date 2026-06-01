use bitfield::bitfield;

use super::sign_extend;
use crate::MaxError;

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct DChnlDataRaw(u32);
    impl Debug;
    u32;
    pub dchnl_data, _: 23, 0;
}

impl_u24_register!(DChnlDataRaw);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DChnlData {
    pub code: i32,
}

impl TryFrom<DChnlDataRaw> for DChnlData {
    type Error = MaxError;

    fn try_from(raw: DChnlDataRaw) -> Result<Self, Self::Error> {
        match raw.dchnl_data() {
            0x7f_ffff => Err(MaxError::AdcOverflow),
            0x80_0000 => Err(MaxError::AdcUnderflow),
            _ => Ok(Self {
                code: sign_extend(raw.dchnl_data(), 24),
            }),
        }
    }
}

impl TryFrom<&[u8]> for DChnlData {
    type Error = crate::MaxError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        DChnlDataRaw::try_from(data)?.try_into()
    }
}
