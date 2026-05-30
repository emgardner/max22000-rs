use crate::MaxError;

macro_rules! impl_u24_register {
    ($ty:ty) => {
        impl $ty {
            pub const fn raw(self) -> u32 {
                self.0
            }

            pub const fn to_be_bytes(self) -> [u8; 3] {
                let bytes = self.0.to_be_bytes();
                [bytes[1], bytes[2], bytes[3]]
            }
        }

        impl From<[u8; 3]> for $ty {
            fn from(data: [u8; 3]) -> Self {
                Self(u32::from_be_bytes([0, data[0], data[1], data[2]]))
            }
        }

        impl From<$ty> for [u8; 3] {
            fn from(register: $ty) -> Self {
                register.to_be_bytes()
            }
        }

        impl TryFrom<&[u8]> for $ty {
            type Error = crate::MaxError;

            fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
                let data: [u8; 3] =
                    data.try_into()
                        .map_err(|_| crate::MaxError::InvalidDataLength {
                            expected: 3,
                            actual: data.len(),
                        })?;

                Ok(data.into())
            }
        }
    };
}

pub mod ao_cnfg_wr;
pub mod dchnl_cmd;
pub mod dchnl_ctrl1;
pub mod dchnl_ctrl2;
pub mod dchnl_n_sel;
pub mod gen_chnl_ctrl;
pub mod gen_cnfg;
pub mod gen_gpi_int;
pub mod gen_gpio_ctrl;
pub mod gen_inten;
pub mod gen_pwr_ctrl;

pub use ao_cnfg_wr::*;
pub use dchnl_cmd::*;
pub use dchnl_ctrl1::*;
pub use dchnl_ctrl2::*;
pub use dchnl_n_sel::*;
pub use gen_chnl_ctrl::*;
pub use gen_cnfg::*;
pub use gen_gpi_int::*;
pub use gen_gpio_ctrl::*;
pub use gen_inten::*;
pub use gen_pwr_ctrl::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DataRate {
    Sps5 = 0b0000,
    Sps10 = 0b0001,
    Sps15 = 0b0010,
    Sps30 = 0b0011,
    Sps50 = 0b0100,
    Sps60 = 0b0101,
    Sps225 = 0b0110,
    Sps450 = 0b0111,
    Sps900 = 0b1000,
    Sps1800 = 0b1001,
    Sps3600 = 0b1010,
    Sps7200 = 0b1011,
    Sps14400 = 0b1100,
    Sps28800 = 0b1101,
    Sps57600 = 0b1110,
    Sps115200 = 0b1111,
}

impl DataRate {
    pub const fn sps(self) -> u32 {
        match self {
            Self::Sps5 => 5,
            Self::Sps10 => 10,
            Self::Sps15 => 15,
            Self::Sps30 => 30,
            Self::Sps50 => 50,
            Self::Sps60 => 60,
            Self::Sps225 => 225,
            Self::Sps450 => 450,
            Self::Sps900 => 900,
            Self::Sps1800 => 1_800,
            Self::Sps3600 => 3_600,
            Self::Sps7200 => 7_200,
            Self::Sps14400 => 14_400,
            Self::Sps28800 => 28_800,
            Self::Sps57600 => 57_600,
            Self::Sps115200 => 115_200,
        }
    }
}

impl TryFrom<u8> for DataRate {
    type Error = MaxError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b0000 => Ok(Self::Sps5),
            0b0001 => Ok(Self::Sps10),
            0b0010 => Ok(Self::Sps15),
            0b0011 => Ok(Self::Sps30),
            0b0100 => Ok(Self::Sps50),
            0b0101 => Ok(Self::Sps60),
            0b0110 => Ok(Self::Sps225),
            0b0111 => Ok(Self::Sps450),
            0b1000 => Ok(Self::Sps900),
            0b1001 => Ok(Self::Sps1800),
            0b1010 => Ok(Self::Sps3600),
            0b1011 => Ok(Self::Sps7200),
            0b1100 => Ok(Self::Sps14400),
            0b1101 => Ok(Self::Sps28800),
            0b1110 => Ok(Self::Sps57600),
            0b1111 => Ok(Self::Sps115200),
            value => Err(MaxError::InvalidRegisterValue { value }),
        }
    }
}

impl From<DataRate> for u8 {
    fn from(rate: DataRate) -> Self {
        rate as Self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AdcModes {
    StandbyPowerDown = 0b01,
    Converting = 0b11,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Registers {
    GenralProduct1 = 0x00,
    GenralRev2 = 0x01,
    GenConfig = 0x02,
    GenChannel = 0x03,
    GenGpio = 0x04,
    GenGpioInt = 0x05,
    GenGpioData = 0x06,
    GenInt = 0x07,
    GenIntEn = 0x08,
    GenPwrCtrl = 0x09,
    DchnlCmd = 0x20,
    DchnlStat = 0x21,
    DchnlCtrl1 = 0x22,
    DchnlCtrl2 = 0x23,
    DchnlData = 0x24,
    DchnlNsel = 0x25,
    DchnlNSoc = 0x26,
    DchnlNSgc = 0x27,
    AoDataWr = 0x40,
    AoOffsetCorrectionWr = 0x41,
    AoGainCorrectionWr = 0x42,
    AoConfigWr = 0x43,
    AoDataRd = 0x44,
    AoOffsetCorrectionRd = 0x45,
    AoGainCorrectionRd = 0x46,
    AoStatusRd = 0x47,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProductInfo {
    pub output_channels: u8,
    pub input_channels: u8,
    pub tc_support: bool,
}

impl From<u8> for ProductInfo {
    fn from(data: u8) -> Self {
        Self {
            output_channels: data >> 5 & 0b111,
            input_channels: data >> 1 & 0b1111,
            tc_support: data & 0x1 > 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Product {
    pub product_id: ProductInfo,
    pub serial_1: u8,
    pub serial_2: u8,
}

impl TryFrom<&[u8]> for Product {
    type Error = MaxError;

    fn try_from(data: &[u8]) -> Result<Self, MaxError> {
        Ok(Product {
            product_id: data[0].into(),
            serial_1: data[1],
            serial_2: data[2],
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rev {
    pub rev_id: u8,
    pub serial_1: u8,
    pub serial_2: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceInfo {
    pub product_id: ProductInfo,
    pub rev_id: u8,
    pub serial: u32,
}

impl TryFrom<&[u8]> for Rev {
    type Error = MaxError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        Ok(Rev {
            rev_id: data[0],
            serial_1: data[1],
            serial_2: data[2],
        })
    }
}

impl DeviceInfo {
    pub fn from_parts(product: &Product, rev: &Rev) -> Self {
        Self {
            product_id: product.product_id,
            rev_id: rev.rev_id,
            serial: u32::from_be_bytes([
                product.serial_1,
                product.serial_2,
                rev.serial_1,
                rev.serial_2,
            ]),
        }
    }
}
