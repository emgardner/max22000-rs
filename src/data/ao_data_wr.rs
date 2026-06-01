use bitfield::bitfield;

use super::sign_extend;

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct AoDataWrRaw(u32);
    impl Debug;
    u32;
    pub ao_data_w, set_ao_data_w: 23, 6;
}

impl_u24_register!(AoDataWrRaw);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AoDataWr {
    pub code: i32,
}

impl From<AoDataWrRaw> for AoDataWr {
    fn from(raw: AoDataWrRaw) -> Self {
        Self {
            code: sign_extend(raw.ao_data_w(), 18),
        }
    }
}

impl TryFrom<&[u8]> for AoDataWr {
    type Error = crate::MaxError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        Ok(AoDataWrRaw::try_from(data)?.into())
    }
}

impl From<AoDataWr> for AoDataWrRaw {
    fn from(data: AoDataWr) -> Self {
        let mut raw = AoDataWrRaw::from([0, 0, 0]);
        raw.set_ao_data_w(data.code as u32 & 0x0003_ffff);
        raw
    }
}
