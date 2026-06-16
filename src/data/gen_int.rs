use bitfield::bitfield;

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct GenIntRaw(u32);
    impl Debug;
    pub pgaovv_int, _: 8;
    pub hvdd_int, _: 7;
    pub hvddo_int, _: 6;
    pub thshdn_int, _: 5;
    pub thwrng_int, _: 4;
    pub ovc_int, _: 3;
    pub cnfg_int, _: 2;
    pub crc_int, _: 1;
    pub gpi_int, _: 0;
}

impl_u24_register!(GenIntRaw);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GenInt {
    pub pga_overvoltage: bool,
    pub hvdd_undervoltage: bool,
    pub hvddo_undervoltage: bool,
    pub thermal_shutdown: bool,
    pub thermal_warning: bool,
    pub overcurrent: bool,
    pub configuration: bool,
    pub crc: bool,
    pub gpi: bool,
}

impl GenInt {
    pub fn is_error(&self) -> bool {
        self.pga_overvoltage
            || self.hvdd_undervoltage
            || self.hvddo_undervoltage
            || self.thermal_shutdown
            || self.thermal_warning
            || self.overcurrent
            || self.configuration
            || self.crc
    }
}

impl From<GenIntRaw> for GenInt {
    fn from(raw: GenIntRaw) -> Self {
        Self {
            pga_overvoltage: raw.pgaovv_int(),
            hvdd_undervoltage: raw.hvdd_int(),
            hvddo_undervoltage: raw.hvddo_int(),
            thermal_shutdown: raw.thshdn_int(),
            thermal_warning: raw.thwrng_int(),
            overcurrent: raw.ovc_int(),
            configuration: raw.cnfg_int(),
            crc: raw.crc_int(),
            gpi: raw.gpi_int(),
        }
    }
}

impl TryFrom<&[u8]> for GenInt {
    type Error = crate::MaxError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        Ok(GenIntRaw::try_from(data)?.into())
    }
}
