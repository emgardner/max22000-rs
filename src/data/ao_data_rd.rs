use bitfield::bitfield;

use super::sign_extend;

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct AoDataRdRaw(u32);
    impl Debug;
    u32;
    pub ao_data_r, _: 17, 0;
}

impl_u24_register!(AoDataRdRaw);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AoDataRd {
    pub code: i32,
}

impl From<AoDataRdRaw> for AoDataRd {
    fn from(raw: AoDataRdRaw) -> Self {
        Self {
            code: sign_extend(raw.ao_data_r(), 18),
        }
    }
}

impl TryFrom<&[u8]> for AoDataRd {
    type Error = crate::MaxError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        Ok(AoDataRdRaw::try_from(data)?.into())
    }
}
