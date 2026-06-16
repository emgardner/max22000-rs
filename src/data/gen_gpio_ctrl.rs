use bitfield::bitfield;

use super::{bools_from_mask, mask_from_bools};

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct GenGpioCtrlRaw(u32);
    impl Debug;
    u8;
    pub gpio_en, set_gpio_en: 21, 16;
    pub gpio_dir, set_gpio_dir: 13, 8;
    pub gpo_data, set_gpo_data: 5, 0;
}

impl_u24_register!(GenGpioCtrlRaw);

impl GenGpioCtrlRaw {
    pub const RESET: Self = Self(0);

    pub fn gpio_enabled(self, pin: GpioPin) -> bool {
        self.gpio_en() & pin.mask() != 0
    }

    pub fn set_gpio_enabled(&mut self, pin: GpioPin, enabled: bool) {
        let value = if enabled {
            self.gpio_en() | pin.mask()
        } else {
            self.gpio_en() & !pin.mask()
        };

        self.set_gpio_en(value);
    }

    pub fn gpio_direction(self, pin: GpioPin) -> GpioDirection {
        (self.gpio_dir() & pin.mask() != 0).into()
    }

    pub fn set_gpio_direction(&mut self, pin: GpioPin, direction: GpioDirection) {
        let value = if direction.into() {
            self.gpio_dir() | pin.mask()
        } else {
            self.gpio_dir() & !pin.mask()
        };

        self.set_gpio_dir(value);
    }

    pub fn gpo_pin_data(self, pin: GpioPin) -> bool {
        self.gpo_data() & pin.mask() != 0
    }

    pub fn set_gpo_pin_data(&mut self, pin: GpioPin, high: bool) {
        let value = if high {
            self.gpo_data() | pin.mask()
        } else {
            self.gpo_data() & !pin.mask()
        };

        self.set_gpo_data(value);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GenGpioCtrl {
    pub enabled: [bool; 6],
    pub direction: [GpioDirection; 6],
    pub output_high: [bool; 6],
}

impl GenGpioCtrl {
    pub fn enable_input(&mut self, pin: GpioPin) {
        self.enabled[pin as usize] = true;
        self.direction[pin as usize] = GpioDirection::Input;
    }

    pub fn enable_output(&mut self, pin: GpioPin, state: bool) {
        self.enabled[pin as usize] = true;
        self.direction[pin as usize] = GpioDirection::Output;
        self.output_high[pin as usize] = state;
    }

    pub fn set_channel_state(&mut self, pin: GpioPin, state: bool) {
        self.output_high[pin as usize] = state;
    }

    pub fn disable_channel(&mut self, pin: GpioPin) {
        self.enabled[pin as usize] = false;
    }
}

impl From<GenGpioCtrlRaw> for GenGpioCtrl {
    fn from(raw: GenGpioCtrlRaw) -> Self {
        Self {
            enabled: bools_from_mask(raw.gpio_en()),
            direction: [
                raw.gpio_direction(GpioPin::Gpio0),
                raw.gpio_direction(GpioPin::Gpio1),
                raw.gpio_direction(GpioPin::Gpio2),
                raw.gpio_direction(GpioPin::Gpio3),
                raw.gpio_direction(GpioPin::Gpio4),
                raw.gpio_direction(GpioPin::Gpio5),
            ],
            output_high: bools_from_mask(raw.gpo_data()),
        }
    }
}

impl TryFrom<&[u8]> for GenGpioCtrl {
    type Error = crate::MaxError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        Ok(GenGpioCtrlRaw::try_from(data)?.into())
    }
}

impl From<GenGpioCtrl> for GenGpioCtrlRaw {
    fn from(config: GenGpioCtrl) -> Self {
        let direction = config
            .direction
            .iter()
            .enumerate()
            .fold(0, |mask, (index, direction)| {
                mask | (bool::from(*direction) as u8) << index
            });

        let mut raw = GenGpioCtrlRaw::RESET;
        raw.set_gpio_en(mask_from_bools(config.enabled));
        raw.set_gpio_dir(direction);
        raw.set_gpo_data(mask_from_bools(config.output_high));
        raw
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GpioPin {
    Gpio0 = 0,
    Gpio1 = 1,
    Gpio2 = 2,
    Gpio3 = 3,
    Gpio4 = 4,
    Gpio5 = 5,
}

impl GpioPin {
    pub const fn mask(self) -> u8 {
        1 << (self as u8)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GpioDirection {
    Input = 0,
    Output = 1,
}

impl From<bool> for GpioDirection {
    fn from(output: bool) -> Self {
        if output { Self::Output } else { Self::Input }
    }
}

impl From<GpioDirection> for bool {
    fn from(direction: GpioDirection) -> Self {
        matches!(direction, GpioDirection::Output)
    }
}
