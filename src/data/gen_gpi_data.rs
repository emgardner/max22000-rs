use bitfield::bitfield;

use super::bools_from_mask;

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct GenGpiDataRaw(u32);
    impl Debug;
    u8;
    pub gpi_pos_edge_int_sta, _: 21, 16;
    pub gpi_neg_edge_int_sta, _: 13, 8;
    pub gpi_data, _: 5, 0;
}

impl_u24_register!(GenGpiDataRaw);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GenGpiData {
    pub positive_edge_detected: [bool; 6],
    pub negative_edge_detected: [bool; 6],
    pub input_high: [bool; 6],
}

impl From<GenGpiDataRaw> for GenGpiData {
    fn from(raw: GenGpiDataRaw) -> Self {
        Self {
            positive_edge_detected: bools_from_mask(raw.gpi_pos_edge_int_sta()),
            negative_edge_detected: bools_from_mask(raw.gpi_neg_edge_int_sta()),
            input_high: bools_from_mask(raw.gpi_data()),
        }
    }
}

impl TryFrom<&[u8]> for GenGpiData {
    type Error = crate::MaxError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        Ok(GenGpiDataRaw::try_from(data)?.into())
    }
}
