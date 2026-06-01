use bitfield::bitfield;

use super::sign_extend;

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct AoOffsetCorrectionWrRaw(u32);
    impl Debug;
    u32;
    pub ao_offset_w, set_ao_offset_w: 23, 6;
}

impl_u24_register!(AoOffsetCorrectionWrRaw);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AoOffsetCorrectionWr {
    pub code: i32,
}

impl From<AoOffsetCorrectionWrRaw> for AoOffsetCorrectionWr {
    fn from(raw: AoOffsetCorrectionWrRaw) -> Self {
        Self {
            code: sign_extend(raw.ao_offset_w(), 18),
        }
    }
}

impl TryFrom<&[u8]> for AoOffsetCorrectionWr {
    type Error = crate::MaxError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        Ok(AoOffsetCorrectionWrRaw::try_from(data)?.into())
    }
}

impl From<AoOffsetCorrectionWr> for AoOffsetCorrectionWrRaw {
    fn from(offset: AoOffsetCorrectionWr) -> Self {
        let mut raw = AoOffsetCorrectionWrRaw::from([0, 0, 0]);
        raw.set_ao_offset_w(offset.code as u32 & 0x0003_ffff);
        raw
    }
}
