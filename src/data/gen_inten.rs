use bitfield::bitfield;

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct GenIntEnRaw(u32);
    impl Debug;
    pub pgaovv_inten, set_pgaovv_inten: 8;
    pub hvdd_inten, set_hvdd_inten: 7;
    pub hvddo_inten, set_hvddo_inten: 6;
    pub thwrng_inten, set_thwrng_inten: 4;
    pub ovc_inten, set_ovc_inten: 3;
    pub cnfg_inten, set_cnfg_inten: 2;
    pub crc_inten, set_crc_inten: 1;
    pub gpi_inten, set_gpi_inten: 0;
}

impl_u24_register!(GenIntEnRaw);

impl GenIntEnRaw {
    pub const RESET: Self = Self(0);

    pub fn interrupt_enabled(self, interrupt: Interrupt) -> bool {
        match interrupt {
            Interrupt::PgaOvervoltage => self.pgaovv_inten(),
            Interrupt::HvddUndervoltage => self.hvdd_inten(),
            Interrupt::HvddoUndervoltage => self.hvddo_inten(),
            Interrupt::ThermalWarning => self.thwrng_inten(),
            Interrupt::Overcurrent => self.ovc_inten(),
            Interrupt::Configuration => self.cnfg_inten(),
            Interrupt::Crc => self.crc_inten(),
            Interrupt::Gpi => self.gpi_inten(),
        }
    }

    pub fn set_interrupt_enabled(&mut self, interrupt: Interrupt, enabled: bool) {
        match interrupt {
            Interrupt::PgaOvervoltage => self.set_pgaovv_inten(enabled),
            Interrupt::HvddUndervoltage => self.set_hvdd_inten(enabled),
            Interrupt::HvddoUndervoltage => self.set_hvddo_inten(enabled),
            Interrupt::ThermalWarning => self.set_thwrng_inten(enabled),
            Interrupt::Overcurrent => self.set_ovc_inten(enabled),
            Interrupt::Configuration => self.set_cnfg_inten(enabled),
            Interrupt::Crc => self.set_crc_inten(enabled),
            Interrupt::Gpi => self.set_gpi_inten(enabled),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interrupt {
    PgaOvervoltage,
    HvddUndervoltage,
    HvddoUndervoltage,
    ThermalWarning,
    Overcurrent,
    Configuration,
    Crc,
    Gpi,
}
