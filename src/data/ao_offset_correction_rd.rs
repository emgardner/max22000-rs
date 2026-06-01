use bitfield::bitfield;

use super::sign_extend;

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct AoOffsetCorrectionRdRaw(u32);
    impl Debug;
    u32;
    pub ao_offset_r, _: 17, 0;
}

impl_u24_register!(AoOffsetCorrectionRdRaw);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AoOffsetCorrectionRd {
    pub code: i32,
}

impl From<AoOffsetCorrectionRdRaw> for AoOffsetCorrectionRd {
    fn from(raw: AoOffsetCorrectionRdRaw) -> Self {
        Self {
            code: sign_extend(raw.ao_offset_r(), 18),
        }
    }
}

impl TryFrom<&[u8]> for AoOffsetCorrectionRd {
    type Error = crate::MaxError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        Ok(AoOffsetCorrectionRdRaw::try_from(data)?.into())
    }
}
