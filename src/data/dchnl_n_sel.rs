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
