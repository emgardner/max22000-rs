use bitfield::bitfield;

use super::{GpioPin, bools_from_mask, mask_from_bools};

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct GenGpiIntRaw(u32);
    impl Debug;
    u8;
    pub gpi_pos_edge_int, set_gpi_pos_edge_int: 21, 16;
    pub gpi_neg_edge_int, set_gpi_neg_edge_int: 13, 8;
}

impl_u24_register!(GenGpiIntRaw);

impl GenGpiIntRaw {
    pub const RESET: Self = Self(0);

    pub fn edge_enabled(self, pin: GpioPin, edge: GpiEdge) -> bool {
        match edge {
            GpiEdge::Positive => self.gpi_pos_edge_int() & pin.mask() != 0,
            GpiEdge::Negative => self.gpi_neg_edge_int() & pin.mask() != 0,
        }
    }

    pub fn set_edge_enabled(&mut self, pin: GpioPin, edge: GpiEdge, enabled: bool) {
        match edge {
            GpiEdge::Positive => {
                let value = if enabled {
                    self.gpi_pos_edge_int() | pin.mask()
                } else {
                    self.gpi_pos_edge_int() & !pin.mask()
                };
                self.set_gpi_pos_edge_int(value);
            }
            GpiEdge::Negative => {
                let value = if enabled {
                    self.gpi_neg_edge_int() | pin.mask()
                } else {
                    self.gpi_neg_edge_int() & !pin.mask()
                };
                self.set_gpi_neg_edge_int(value);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GenGpiInt {
    pub positive_edge_enabled: [bool; 6],
    pub negative_edge_enabled: [bool; 6],
}

impl From<GenGpiIntRaw> for GenGpiInt {
    fn from(raw: GenGpiIntRaw) -> Self {
        Self {
            positive_edge_enabled: bools_from_mask(raw.gpi_pos_edge_int()),
            negative_edge_enabled: bools_from_mask(raw.gpi_neg_edge_int()),
        }
    }
}

impl TryFrom<&[u8]> for GenGpiInt {
    type Error = crate::MaxError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        Ok(GenGpiIntRaw::try_from(data)?.into())
    }
}

impl From<GenGpiInt> for GenGpiIntRaw {
    fn from(config: GenGpiInt) -> Self {
        let mut raw = GenGpiIntRaw::RESET;
        raw.set_gpi_pos_edge_int(mask_from_bools(config.positive_edge_enabled));
        raw.set_gpi_neg_edge_int(mask_from_bools(config.negative_edge_enabled));
        raw
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpiEdge {
    Positive,
    Negative,
}
