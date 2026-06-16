pub mod data;

use embedded_hal::spi::{Error as SpiError, ErrorKind as SpiErrorKind, SpiDevice};
use thiserror::Error;

use crate::data::{DeviceInfo, Registers};

const CRC8: crc::Crc<u8> = crc::Crc::<u8>::new(&crc::CRC_8_MAXIM_DOW);

pub struct Max22000<SPI> {
    spi: SPI,
    crc_enabled: bool,
}

#[derive(Debug, Clone, Copy, Error)]
pub enum MaxError {
    #[error("Crc Error")]
    CrcError,
    #[error("Spi Error: {0}")]
    SpiError(SpiErrorKind),
    #[error("Invalid data length: expected {expected} bytes, got {actual}")]
    InvalidDataLength { expected: usize, actual: usize },
    #[error("Invalid register value: {value}")]
    InvalidRegisterValue { value: u8 },
    #[error("AdcOverflow")]
    AdcOverflow,
    #[error("AdcUnderflow")]
    AdcUnderflow,
}

pub type MaxResult<T> = Result<T, MaxError>;

fn spi_error<E: SpiError>(error: E) -> MaxError {
    MaxError::SpiError(error.kind())
}

impl<SPI> Max22000<SPI> {
    pub fn new(spi: SPI) -> Self {
        Self {
            spi,
            crc_enabled: false,
        }
    }

    pub fn release(self) -> SPI {
        self.spi
    }

    pub fn crc_enabled(&self) -> bool {
        self.crc_enabled
    }

    pub fn set_crc(&mut self, enabled: bool) {
        self.crc_enabled = enabled;
    }

    pub fn crc_buffer(bytes: &[u8]) -> u8 {
        CRC8.checksum(bytes)
    }
}

impl<SPI> Max22000<SPI>
where
    SPI: SpiDevice<u8>,
{
    pub fn enable_crc(&mut self) -> MaxResult<()> {
        let mut config = self.read_gen_config()?;
        config.crc_en = true;
        self.write_gen_config(config)?;
        self.crc_enabled = true;
        Ok(())
    }

    pub fn check_crc(&mut self) -> MaxResult<bool> {
        let config = self.read_gen_config()?;
        Ok(config.crc_en)
    }

    pub fn read_register(&mut self, register: Registers) -> MaxResult<[u8; 3]> {
        let operation = Operation::Read(register).as_u8();

        if self.crc_enabled {
            let mut frame = [operation, 0, 0, 0, 0];
            frame[4] = Self::crc_buffer(&frame[..4]);

            self.spi.transfer_in_place(&mut frame).map_err(spi_error)?;

            if Self::crc_buffer(&frame[..4]) != frame[4] {
                return Err(MaxError::CrcError);
            }

            Ok([frame[1], frame[2], frame[3]])
        } else {
            let mut frame = [operation, 0, 0, 0];
            self.spi.transfer_in_place(&mut frame).map_err(spi_error)?;
            Ok([frame[1], frame[2], frame[3]])
        }
    }

    pub fn write_register(&mut self, register: Registers, payload: [u8; 3]) -> MaxResult<()> {
        let operation = Operation::Write(register).as_u8();
        let frame = [operation, payload[0], payload[1], payload[2]];
        if register == Registers::AoDataWr {
            println!("AO DATA: {:02X?}", frame);
        }
        if register == Registers::AoGainCorrectionWr {
            println!("AO GAIN: {:02X?}", frame);
        }
        if self.crc_enabled {
            let mut frame_with_crc = [0; 5];
            frame_with_crc[..4].copy_from_slice(&frame);
            frame_with_crc[4] = Self::crc_buffer(&frame);
            self.spi.write(&frame_with_crc).map_err(spi_error)
        } else {
            self.spi.write(&frame).map_err(spi_error)
        }
    }

    fn read_register_as<T>(&mut self, register: Registers) -> MaxResult<T>
    where
        for<'data> T: TryFrom<&'data [u8], Error = MaxError>,
    {
        let data = self.read_register(register)?;
        T::try_from(data.as_slice())
    }

    fn write_register_as<T>(&mut self, register: Registers, payload: T) -> MaxResult<()>
    where
        T: Into<[u8; 3]>,
    {
        self.write_register(register, payload.into())
    }

    pub fn read_gen_product(&mut self) -> MaxResult<data::Product> {
        self.read_register_as(Registers::GenralProduct1)
    }

    pub fn read_gen_rev(&mut self) -> MaxResult<data::Rev> {
        self.read_register_as(Registers::GenralRev2)
    }

    pub fn read_gen_config_raw(&mut self) -> MaxResult<data::GenConfigRaw> {
        self.read_register_as(Registers::GenConfig)
    }

    pub fn read_gen_config(&mut self) -> MaxResult<data::GenConfig> {
        self.read_register_as(Registers::GenConfig)
    }

    pub fn write_gen_config_raw(&mut self, config: data::GenConfigRaw) -> MaxResult<()> {
        self.write_register_as(Registers::GenConfig, config)
    }

    pub fn write_gen_config(&mut self, config: data::GenConfig) -> MaxResult<()> {
        self.write_gen_config_raw(config.into())
    }

    pub fn read_gen_chnl_ctrl_raw(&mut self) -> MaxResult<data::GenChnlCtrlRaw> {
        self.read_register_as(Registers::GenChannel)
    }

    pub fn read_gen_chnl_ctrl(&mut self) -> MaxResult<data::GenChnlCtrl> {
        self.read_register_as(Registers::GenChannel)
    }

    pub fn write_gen_chnl_ctrl_raw(&mut self, config: data::GenChnlCtrlRaw) -> MaxResult<()> {
        self.write_register_as(Registers::GenChannel, config)
    }

    pub fn write_gen_chnl_ctrl(&mut self, config: data::GenChnlCtrl) -> MaxResult<()> {
        self.write_gen_chnl_ctrl_raw(config.into())
    }

    pub fn read_gen_gpio_ctrl_raw(&mut self) -> MaxResult<data::GenGpioCtrlRaw> {
        self.read_register_as(Registers::GenGpio)
    }

    pub fn read_gen_gpio_ctrl(&mut self) -> MaxResult<data::GenGpioCtrl> {
        self.read_register_as(Registers::GenGpio)
    }

    pub fn write_gen_gpio_ctrl_raw(&mut self, config: data::GenGpioCtrlRaw) -> MaxResult<()> {
        self.write_register_as(Registers::GenGpio, config)
    }

    pub fn write_gen_gpio_ctrl(&mut self, config: data::GenGpioCtrl) -> MaxResult<()> {
        self.write_gen_gpio_ctrl_raw(config.into())
    }

    pub fn read_gen_gpi_int_raw(&mut self) -> MaxResult<data::GenGpiIntRaw> {
        self.read_register_as(Registers::GenGpioInt)
    }

    pub fn read_gen_gpi_int(&mut self) -> MaxResult<data::GenGpiInt> {
        self.read_register_as(Registers::GenGpioInt)
    }

    pub fn write_gen_gpi_int_raw(&mut self, config: data::GenGpiIntRaw) -> MaxResult<()> {
        self.write_register_as(Registers::GenGpioInt, config)
    }

    pub fn write_gen_gpi_int(&mut self, config: data::GenGpiInt) -> MaxResult<()> {
        self.write_gen_gpi_int_raw(config.into())
    }

    pub fn read_gen_gpi_data_raw(&mut self) -> MaxResult<data::GenGpiDataRaw> {
        self.read_register_as(Registers::GenGpioData)
    }

    pub fn read_gen_gpi_data(&mut self) -> MaxResult<data::GenGpiData> {
        self.read_register_as(Registers::GenGpioData)
    }

    pub fn read_gen_int_raw(&mut self) -> MaxResult<data::GenIntRaw> {
        self.read_register_as(Registers::GenInt)
    }

    pub fn read_gen_int(&mut self) -> MaxResult<data::GenInt> {
        self.read_register_as(Registers::GenInt)
    }

    pub fn read_gen_inten_raw(&mut self) -> MaxResult<data::GenIntEnRaw> {
        self.read_register_as(Registers::GenIntEn)
    }

    pub fn read_gen_inten(&mut self) -> MaxResult<data::GenIntEn> {
        self.read_register_as(Registers::GenIntEn)
    }

    pub fn write_gen_inten_raw(&mut self, config: data::GenIntEnRaw) -> MaxResult<()> {
        self.write_register_as(Registers::GenIntEn, config)
    }

    pub fn write_gen_inten(&mut self, config: data::GenIntEn) -> MaxResult<()> {
        self.write_gen_inten_raw(config.into())
    }

    pub fn read_gen_pwr_ctrl_raw(&mut self) -> MaxResult<data::GenPwrCtrlRaw> {
        self.read_register_as(Registers::GenPwrCtrl)
    }

    pub fn read_gen_pwr_ctrl(&mut self) -> MaxResult<data::GenPwrCtrl> {
        self.read_register_as(Registers::GenPwrCtrl)
    }

    pub fn write_gen_pwr_ctrl_raw(&mut self, config: data::GenPwrCtrlRaw) -> MaxResult<()> {
        self.write_register_as(Registers::GenPwrCtrl, config)
    }

    pub fn write_gen_pwr_ctrl(&mut self, config: data::GenPwrCtrl) -> MaxResult<()> {
        self.write_gen_pwr_ctrl_raw(config.into())
    }

    pub fn read_dchnl_mode_raw(&mut self) -> MaxResult<data::DChnlMode> {
        self.read_register_as(Registers::DchnlMode)
    }

    pub fn read_dchnl_mode(&mut self) -> MaxResult<data::DChnlMode> {
        self.read_register_as(Registers::DchnlMode)
    }

    pub fn write_dchnl_mode_raw(&mut self, command: data::DChnlModeRaw) -> MaxResult<()> {
        self.write_register_as(Registers::DchnlMode, command)
    }

    pub fn write_dchnl_mode(&mut self, command: data::DChnlMode) -> MaxResult<()> {
        self.write_dchnl_mode_raw(command.into())
    }

    pub fn read_dchnl_stat_raw(&mut self) -> MaxResult<data::DChnlStatRaw> {
        self.read_register_as(Registers::DchnlStat)
    }

    pub fn read_dchnl_stat(&mut self) -> MaxResult<data::DChnlStatus> {
        self.read_register_as(Registers::DchnlStat)
    }

    pub fn read_dchnl_ctrl1_raw(&mut self) -> MaxResult<data::DChnlCtrl1Raw> {
        self.read_register_as(Registers::DchnlCtrl1)
    }

    pub fn read_dchnl_ctrl1(&mut self) -> MaxResult<data::DChnlCtrl1> {
        self.read_register_as(Registers::DchnlCtrl1)
    }

    pub fn write_dchnl_ctrl1_raw(&mut self, config: data::DChnlCtrl1Raw) -> MaxResult<()> {
        self.write_register_as(Registers::DchnlCtrl1, config)
    }

    pub fn write_dchnl_ctrl1(&mut self, config: data::DChnlCtrl1) -> MaxResult<()> {
        self.write_dchnl_ctrl1_raw(config.into())
    }

    pub fn read_dchnl_ctrl2_raw(&mut self) -> MaxResult<data::DChnlCtrl2Raw> {
        self.read_register_as(Registers::DchnlCtrl2)
    }

    pub fn read_dchnl_ctrl2(&mut self) -> MaxResult<data::DChnlCtrl2> {
        self.read_register_as(Registers::DchnlCtrl2)
    }

    pub fn write_dchnl_ctrl2_raw(&mut self, config: data::DChnlCtrl2Raw) -> MaxResult<()> {
        self.write_register_as(Registers::DchnlCtrl2, config)
    }

    pub fn write_dchnl_ctrl2(&mut self, config: data::DChnlCtrl2) -> MaxResult<()> {
        self.write_dchnl_ctrl2_raw(config.into())
    }

    pub fn read_dchnl_data_raw(&mut self) -> MaxResult<data::DChnlDataRaw> {
        self.read_register_as(Registers::DchnlData)
    }

    pub fn read_dchnl_data(&mut self) -> MaxResult<data::DChnlData> {
        self.read_register_as(Registers::DchnlData)
    }

    pub fn read_dchnl_n_sel_raw(&mut self) -> MaxResult<data::DChnlNSelRaw> {
        self.read_register_as(Registers::DchnlNsel)
    }

    pub fn read_dchnl_n_sel(&mut self) -> MaxResult<data::DChnlNSel> {
        self.read_register_as(Registers::DchnlNsel)
    }

    pub fn write_dchnl_n_sel_raw(&mut self, selection: data::DChnlNSelRaw) -> MaxResult<()> {
        self.write_register_as(Registers::DchnlNsel, selection)
    }

    pub fn write_dchnl_n_sel(&mut self, selection: data::DChnlNSel) -> MaxResult<()> {
        self.write_dchnl_n_sel_raw(selection.into())
    }

    pub fn read_dchnl_n_soc_raw(&mut self) -> MaxResult<data::DChnlNSocRaw> {
        self.read_register_as(Registers::DchnlNSoc)
    }

    pub fn read_dchnl_n_soc(&mut self) -> MaxResult<data::DChnlNSoc> {
        self.read_register_as(Registers::DchnlNSoc)
    }

    pub fn write_dchnl_n_soc_raw(&mut self, offset: data::DChnlNSocRaw) -> MaxResult<()> {
        self.write_register_as(Registers::DchnlNSoc, offset)
    }

    pub fn write_dchnl_n_soc(&mut self, offset: data::DChnlNSoc) -> MaxResult<()> {
        self.write_dchnl_n_soc_raw(offset.into())
    }

    pub fn read_dchnl_n_sgc_raw(&mut self) -> MaxResult<data::DChnlNSgcRaw> {
        self.read_register_as(Registers::DchnlNSgc)
    }

    pub fn read_dchnl_n_sgc(&mut self) -> MaxResult<data::DChnlNSgc> {
        self.read_register_as(Registers::DchnlNSgc)
    }

    pub fn write_dchnl_n_sgc_raw(&mut self, gain: data::DChnlNSgcRaw) -> MaxResult<()> {
        self.write_register_as(Registers::DchnlNSgc, gain)
    }

    pub fn write_dchnl_n_sgc(&mut self, gain: data::DChnlNSgc) -> MaxResult<()> {
        self.write_dchnl_n_sgc_raw(gain.into())
    }

    pub fn write_ao_data_wr_raw(&mut self, data: data::AoDataWrRaw) -> MaxResult<()> {
        self.write_register_as(Registers::AoDataWr, data)
    }

    pub fn write_ao_data_wr(&mut self, data: data::AoDataWr) -> MaxResult<()> {
        self.write_ao_data_wr_raw(data.into())
    }

    pub fn write_ao_offset_correction_wr_raw(
        &mut self,
        offset: data::AoOffsetCorrectionWrRaw,
    ) -> MaxResult<()> {
        self.write_register_as(Registers::AoOffsetCorrectionWr, offset)
    }

    pub fn write_ao_offset_correction_wr(
        &mut self,
        offset: data::AoOffsetCorrectionWr,
    ) -> MaxResult<()> {
        self.write_ao_offset_correction_wr_raw(offset.into())
    }

    pub fn write_ao_gain_correction_wr_raw(
        &mut self,
        gain: data::AoGainCorrectionWrRaw,
    ) -> MaxResult<()> {
        self.write_register_as(Registers::AoGainCorrectionWr, gain)
    }

    pub fn write_ao_gain_correction_wr(&mut self, gain: data::AoGainCorrectionWr) -> MaxResult<()> {
        self.write_ao_gain_correction_wr_raw(gain.into())
    }

    pub fn read_ao_cnfg_wr_raw(&mut self) -> MaxResult<data::AoCnfgWrRaw> {
        self.read_register_as(Registers::AoConfigWr)
    }

    pub fn read_ao_cnfg_wr(&mut self) -> MaxResult<data::AoConfig> {
        self.read_register_as(Registers::AoConfigWr)
    }

    pub fn write_ao_cnfg_wr_raw(&mut self, config: data::AoCnfgWrRaw) -> MaxResult<()> {
        self.write_register_as(Registers::AoConfigWr, config)
    }

    pub fn write_ao_cnfg_wr(&mut self, config: data::AoConfig) -> MaxResult<()> {
        self.write_ao_cnfg_wr_raw(config.into())
    }

    pub fn read_ao_data_rd_raw(&mut self) -> MaxResult<data::AoDataRdRaw> {
        self.read_register_as(Registers::AoDataRd)
    }

    pub fn read_ao_data_rd(&mut self) -> MaxResult<data::AoDataRd> {
        self.read_register_as(Registers::AoDataRd)
    }

    pub fn read_ao_offset_correction_rd_raw(&mut self) -> MaxResult<data::AoOffsetCorrectionRdRaw> {
        self.read_register_as(Registers::AoOffsetCorrectionRd)
    }

    pub fn read_ao_offset_correction_rd(&mut self) -> MaxResult<data::AoOffsetCorrectionRd> {
        self.read_register_as(Registers::AoOffsetCorrectionRd)
    }

    pub fn read_ao_gain_correction_rd_raw(&mut self) -> MaxResult<data::AoGainCorrectionRdRaw> {
        self.read_register_as(Registers::AoGainCorrectionRd)
    }

    pub fn read_ao_gain_correction_rd(&mut self) -> MaxResult<data::AoGainCorrectionRd> {
        self.read_register_as(Registers::AoGainCorrectionRd)
    }

    pub fn read_ao_status_rd_raw(&mut self) -> MaxResult<data::AoStatusRdRaw> {
        self.read_register_as(Registers::AoStatusRd)
    }

    pub fn read_ao_status_rd(&mut self) -> MaxResult<data::AoStatus> {
        self.read_register_as(Registers::AoStatusRd)
    }

    pub fn read_info(&mut self) -> MaxResult<data::DeviceInfo> {
        let product = self.read_gen_product()?;
        let rev = self.read_gen_rev()?;
        Ok(DeviceInfo::from_parts(&product, &rev))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Read(data::Registers),
    Write(data::Registers),
}

impl Operation {
    pub fn as_u8(&self) -> u8 {
        match self {
            Self::Read(register) => (*register as u8) << 1 | 0x01,
            Self::Write(register) => (*register as u8) << 1,
        }
    }
}
