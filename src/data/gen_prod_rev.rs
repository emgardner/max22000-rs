use crate::MaxError;

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
