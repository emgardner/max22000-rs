use bitfield::bitfield;

use super::{
    ao_gain_correction_wr::AoGainCorrectionWr, ao_offset_correction_wr::AoOffsetCorrectionWr,
    gen_cnfg::AnalogOutConfig, sign_extend,
};

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
    code: u32,
}

impl AoDataWr {
    pub const MIN_CODE: u32 = 0;
    pub const MAX_CODE: u32 = (1 << 18) - 1;
    pub const MIN_SIGNED_CODE: i32 = -(1 << 17);
    pub const MAX_SIGNED_CODE: i32 = (1 << 17) - 1;
    pub const NEGATIVE_FULL_SCALE: Self = Self { code: 1 << 17 };
    pub const POSITIVE_FULL_SCALE: Self = Self {
        code: (1 << 17) - 1,
    };
    pub const ZERO: Self = Self { code: 0 };

    pub fn from_code(code: u32) -> Option<Self> {
        (code <= Self::MAX_CODE).then_some(Self { code })
    }

    pub const fn clamped(code: u32) -> Self {
        Self::from_code_clamped(code)
    }

    pub const fn from_code_clamped(code: u32) -> Self {
        Self {
            code: if code > Self::MAX_CODE {
                Self::MAX_CODE
            } else {
                code
            },
        }
    }

    pub fn from_signed_code(code: i32) -> Option<Self> {
        (Self::MIN_SIGNED_CODE..=Self::MAX_SIGNED_CODE)
            .contains(&code)
            .then_some(Self {
                code: code as u32 & Self::MAX_CODE,
            })
    }

    pub const fn from_signed_code_clamped(code: i32) -> Self {
        let code = if code < Self::MIN_SIGNED_CODE {
            Self::MIN_SIGNED_CODE
        } else if code > Self::MAX_SIGNED_CODE {
            Self::MAX_SIGNED_CODE
        } else {
            code
        };

        Self {
            code: code as u32 & Self::MAX_CODE,
        }
    }

    pub const fn code(self) -> u32 {
        self.code
    }

    pub fn signed_code(self) -> i32 {
        sign_extend(self.code, 18)
    }

    /// Returns the Table 6 signed code fraction, `AO_DATA_W / 2^17`.
    pub fn code_fraction(self) -> f64 {
        self.signed_code() as f64 / 131_072.0
    }

    /// Applies the Table 9 gain and offset correction formula.
    pub fn corrected_fraction(self, gain: AoGainCorrectionWr, offset: AoOffsetCorrectionWr) -> f64 {
        let corrected = self.code_fraction() * gain.gain() + offset.fraction();
        corrected.clamp(-1.0, Self::MAX_SIGNED_CODE as f64 / 131_072.0)
    }

    /// Returns the nominal voltage for `AnalogOut25V` or `AnalogOut12_5V`.
    pub fn nominal_voltage(self, mode: AnalogOutConfig) -> Option<f64> {
        self.nominal_voltage_with_correction(
            mode,
            AoGainCorrectionWr::UNITY,
            AoOffsetCorrectionWr::ZERO,
        )
    }

    pub fn nominal_voltage_with_correction(
        self,
        mode: AnalogOutConfig,
        gain: AoGainCorrectionWr,
        offset: AoOffsetCorrectionWr,
    ) -> Option<f64> {
        let fraction = self.corrected_fraction(gain, offset);

        match mode {
            AnalogOutConfig::AnalogOut25V => Some(12.5 * fraction + 25.0),
            AnalogOutConfig::AnalogOut12_5V => Some(12.5 * fraction),
            _ => None,
        }
    }

    /// Returns the nominal current, in milliamps, for `AnalogOut25ma` or `AnalogOut2_5ma`.
    pub fn nominal_current_ma(self, mode: AnalogOutConfig) -> Option<f64> {
        self.nominal_current_ma_with_correction(
            mode,
            AoGainCorrectionWr::UNITY,
            AoOffsetCorrectionWr::ZERO,
        )
    }

    pub fn nominal_current_ma_with_correction(
        self,
        mode: AnalogOutConfig,
        gain: AoGainCorrectionWr,
        offset: AoOffsetCorrectionWr,
    ) -> Option<f64> {
        let fraction = self.corrected_fraction(gain, offset);

        match mode {
            AnalogOutConfig::AnalogOut25ma => Some(25.0 * fraction),
            AnalogOutConfig::AnalogOut2_5ma => Some(2.5 * fraction),
            _ => None,
        }
    }

    pub fn from_nominal_voltage(mode: AnalogOutConfig, voltage: f64) -> Option<Self> {
        Self::from_nominal_voltage_with_correction(
            mode,
            voltage,
            AoGainCorrectionWr::UNITY,
            AoOffsetCorrectionWr::ZERO,
        )
    }

    pub fn from_nominal_voltage_with_correction(
        mode: AnalogOutConfig,
        voltage: f64,
        gain: AoGainCorrectionWr,
        offset: AoOffsetCorrectionWr,
    ) -> Option<Self> {
        let fraction = match mode {
            AnalogOutConfig::AnalogOut25V => (voltage - 25.0) / 12.5,
            AnalogOutConfig::AnalogOut12_5V => voltage / 12.5,
            _ => return None,
        };

        Self::from_corrected_fraction(fraction, gain, offset)
    }

    pub fn from_nominal_current_ma(mode: AnalogOutConfig, current_ma: f64) -> Option<Self> {
        Self::from_nominal_current_ma_with_correction(
            mode,
            current_ma,
            AoGainCorrectionWr::UNITY,
            AoOffsetCorrectionWr::ZERO,
        )
    }

    pub fn from_nominal_current_ma_with_correction(
        mode: AnalogOutConfig,
        current_ma: f64,
        gain: AoGainCorrectionWr,
        offset: AoOffsetCorrectionWr,
    ) -> Option<Self> {
        let fraction = match mode {
            AnalogOutConfig::AnalogOut25ma => current_ma / 25.0,
            AnalogOutConfig::AnalogOut2_5ma => current_ma / 2.5,
            _ => return None,
        };

        Self::from_corrected_fraction(fraction, gain, offset)
    }

    fn from_corrected_fraction(
        fraction: f64,
        gain: AoGainCorrectionWr,
        offset: AoOffsetCorrectionWr,
    ) -> Option<Self> {
        if !fraction.is_finite() {
            return None;
        }

        let signed_code = ((fraction - offset.fraction()) / gain.gain() * 131_072.0).round() as i32;
        let signed_code = signed_code.clamp(Self::MIN_SIGNED_CODE, Self::MAX_SIGNED_CODE);

        Some(Self {
            code: signed_code as u32 & Self::MAX_CODE,
        })
    }
}

impl From<AoDataWrRaw> for AoDataWr {
    fn from(raw: AoDataWrRaw) -> Self {
        Self {
            code: raw.ao_data_w(),
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
        raw.set_ao_data_w(data.code);
        raw
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < 1e-12,
            "actual {actual} did not match expected {expected}"
        );
    }

    #[test]
    fn packs_encoded_eighteen_bit_code_in_ao_data_w_field() {
        let positive_full_scale = AoDataWrRaw::from(AoDataWr::POSITIVE_FULL_SCALE);
        let negative_full_scale = AoDataWrRaw::from(AoDataWr::NEGATIVE_FULL_SCALE);
        let minus_one = AoDataWrRaw::from(AoDataWr::from_code(0x3ffff).unwrap());

        assert_eq!(positive_full_scale.raw(), 0x7f_ff_c0);
        assert_eq!(negative_full_scale.raw(), 0x80_00_00);
        assert_eq!(minus_one.raw(), 0xff_ff_c0);
        assert_eq!(AoDataWr::from(positive_full_scale).code(), 0x1ffff);
        assert_eq!(AoDataWr::from(negative_full_scale).code(), 0x20000);
        assert_eq!(AoDataWr::from(minus_one).code(), 0x3ffff);
        assert_eq!(AoDataWr::from(minus_one).signed_code(), -1);
    }

    #[test]
    fn construction_rejects_codes_outside_the_eighteen_bit_field() {
        assert_eq!(AoDataWr::from_code(0x00000).unwrap().signed_code(), 0);
        assert_eq!(AoDataWr::from_code(0x1ffff).unwrap().signed_code(), 131_071);
        assert_eq!(
            AoDataWr::from_code(0x20000).unwrap().signed_code(),
            -131_072
        );
        assert_eq!(AoDataWr::from_code(0x3ffff).unwrap().signed_code(), -1);
        assert!(AoDataWr::from_code(0x40000).is_none());
        assert!(AoDataWr::from_signed_code(-131_073).is_none());
        assert!(AoDataWr::from_signed_code(131_072).is_none());
    }

    #[test]
    fn clamped_constructors_saturate_to_valid_ranges() {
        assert_eq!(AoDataWr::clamped(0x40000).code(), 0x3ffff);
        assert_eq!(AoDataWr::from_code_clamped(0x40000).signed_code(), -1);
        assert_eq!(AoDataWr::from_signed_code_clamped(-131_073).code(), 0x20000);
        assert_eq!(AoDataWr::from_signed_code_clamped(131_072).code(), 0x1ffff);
    }

    #[test]
    fn nominal_values_match_table_6() {
        assert_close(
            AoDataWr::NEGATIVE_FULL_SCALE
                .nominal_voltage(AnalogOutConfig::AnalogOut25V)
                .unwrap(),
            12.5,
        );
        assert_close(
            AoDataWr::from_code(0x3ffff)
                .unwrap()
                .nominal_voltage(AnalogOutConfig::AnalogOut25V)
                .unwrap(),
            24.99990463256836,
        );
        assert_close(
            AoDataWr::ZERO
                .nominal_voltage(AnalogOutConfig::AnalogOut25V)
                .unwrap(),
            25.0,
        );
        assert_close(
            AoDataWr::POSITIVE_FULL_SCALE
                .nominal_voltage(AnalogOutConfig::AnalogOut25V)
                .unwrap(),
            37.49990463256836,
        );

        assert_close(
            AoDataWr::NEGATIVE_FULL_SCALE
                .nominal_current_ma(AnalogOutConfig::AnalogOut25ma)
                .unwrap(),
            -25.0,
        );
        assert_close(
            AoDataWr::from_code(0x3ffff)
                .unwrap()
                .nominal_current_ma(AnalogOutConfig::AnalogOut25ma)
                .unwrap(),
            -0.00019073486328125,
        );
        assert_close(
            AoDataWr::POSITIVE_FULL_SCALE
                .nominal_current_ma(AnalogOutConfig::AnalogOut2_5ma)
                .unwrap(),
            2.499980926513672,
        );
    }

    #[test]
    fn gain_and_offset_match_tables_7_and_8() {
        assert_close(AoGainCorrectionWr { code: 0x00000 }.gain(), 1.0 / 262_144.0);
        assert_close(AoGainCorrectionWr { code: 0x0ffff }.gain(), 0.25);
        assert_close(AoGainCorrectionWr { code: 0x1ffff }.gain(), 0.5);
        assert_close(AoGainCorrectionWr { code: 0x2ffff }.gain(), 0.75);
        assert_close(AoGainCorrectionWr::UNITY.gain(), 1.0);

        assert_close(
            AoOffsetCorrectionWr {
                code: AoOffsetCorrectionWr::MIN_CODE,
            }
            .fraction(),
            -1.0,
        );
        assert_close(
            AoOffsetCorrectionWr { code: -1 }.fraction(),
            -1.0 / 131_072.0,
        );
        assert_close(AoOffsetCorrectionWr::ZERO.fraction(), 0.0);
        assert_close(
            AoOffsetCorrectionWr {
                code: AoOffsetCorrectionWr::MAX_CODE,
            }
            .fraction(),
            131_071.0 / 131_072.0,
        );
    }

    #[test]
    fn table_9_formula_applies_gain_offset_and_saturates() {
        let data = AoDataWr::POSITIVE_FULL_SCALE;
        let half_gain = AoGainCorrectionWr { code: 0x1ffff };
        let positive_half_scale_offset = AoOffsetCorrectionWr {
            code: AoOffsetCorrectionWr::MAX_CODE,
        };

        assert_close(
            data.corrected_fraction(half_gain, AoOffsetCorrectionWr::ZERO),
            0.4999961853027344,
        );
        assert_close(
            data.corrected_fraction(AoGainCorrectionWr::UNITY, positive_half_scale_offset),
            131_071.0 / 131_072.0,
        );
    }

    #[test]
    fn inverse_nominal_calculations_use_table_9_scale() {
        assert_eq!(
            AoDataWr::from_nominal_voltage(AnalogOutConfig::AnalogOut12_5V, 12.5)
                .unwrap()
                .code(),
            0x1ffff
        );
        assert_eq!(
            AoDataWr::from_nominal_voltage(AnalogOutConfig::AnalogOut25V, 25.0)
                .unwrap()
                .code(),
            0
        );
        assert_eq!(
            AoDataWr::from_nominal_current_ma(AnalogOutConfig::AnalogOut25ma, -25.0)
                .unwrap()
                .code(),
            0x20000
        );
        assert!(AoDataWr::from_nominal_voltage(AnalogOutConfig::AnalogOut25ma, 1.0).is_none());
        assert!(AoDataWr::from_nominal_current_ma(AnalogOutConfig::AnalogOut12_5V, 1.0).is_none());
    }
}
