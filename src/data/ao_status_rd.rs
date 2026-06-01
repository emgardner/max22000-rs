use bitfield::bitfield;

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct AoStatusRdRaw(u32);
    impl Debug;
    pub busy, _: 12;
}

impl_u24_register!(AoStatusRdRaw);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AoStatus {
    pub busy: bool,
}

impl From<AoStatusRdRaw> for AoStatus {
    fn from(raw: AoStatusRdRaw) -> Self {
        Self { busy: raw.busy() }
    }
}

impl TryFrom<&[u8]> for AoStatus {
    type Error = crate::MaxError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        Ok(AoStatusRdRaw::try_from(data)?.into())
    }
}
