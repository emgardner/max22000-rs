use bitfield::bitfield;

use super::AdcChannelSelection;
use crate::MaxError;

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct DChnlNSelRaw(u32);
    impl Debug;
    u8;
    pub dchnl_n_sel, set_dchnl_n_sel: 3, 0;
}

impl_u24_register!(DChnlNSelRaw);

impl DChnlNSelRaw {
    pub const RESET: Self = Self(0);

    pub fn adc_channel_selection(self) -> Result<AdcChannelSelection, MaxError> {
        self.dchnl_n_sel().try_into()
    }

    pub fn set_adc_channel_selection(&mut self, selection: AdcChannelSelection) {
        self.set_dchnl_n_sel(selection.into());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DChnlNSel {
    pub adc_channel_selection: AdcChannelSelection,
}

impl TryFrom<DChnlNSelRaw> for DChnlNSel {
    type Error = MaxError;

    fn try_from(raw: DChnlNSelRaw) -> Result<Self, Self::Error> {
        Ok(Self {
            adc_channel_selection: raw.adc_channel_selection()?,
        })
    }
}

impl TryFrom<&[u8]> for DChnlNSel {
    type Error = MaxError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        DChnlNSelRaw::try_from(data)?.try_into()
    }
}

impl From<DChnlNSel> for DChnlNSelRaw {
    fn from(selection: DChnlNSel) -> Self {
        let mut raw = DChnlNSelRaw::RESET;
        raw.set_adc_channel_selection(selection.adc_channel_selection);
        raw
    }
}
