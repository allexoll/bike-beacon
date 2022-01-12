///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register 1
    pub cr1: crate::Reg<cr1::CR1_SPEC>,
    ///0x04 - control register 2
    pub cr2: crate::Reg<cr2::CR2_SPEC>,
    ///0x08 - status register
    pub sr: crate::Reg<sr::SR_SPEC>,
    ///0x0c - data register
    pub dr: crate::Reg<dr::DR_SPEC>,
    ///0x10 - CRC polynomial register
    pub crcpr: crate::Reg<crcpr::CRCPR_SPEC>,
    ///0x14 - RX CRC register
    pub rxcrcr: crate::Reg<rxcrcr::RXCRCR_SPEC>,
    ///0x18 - TX CRC register
    pub txcrcr: crate::Reg<txcrcr::TXCRCR_SPEC>,
    ///0x1c - I2S configuration register
    pub i2scfgr: crate::Reg<i2scfgr::I2SCFGR_SPEC>,
    ///0x20 - I2S prescaler register
    pub i2spr: crate::Reg<i2spr::I2SPR_SPEC>,
}
///CR1 register accessor: an alias for `Reg<CR1_SPEC>`
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
///control register 1
pub mod cr1;
///CR2 register accessor: an alias for `Reg<CR2_SPEC>`
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
///control register 2
pub mod cr2;
///SR register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///status register
pub mod sr;
///DR register accessor: an alias for `Reg<DR_SPEC>`
pub type DR = crate::Reg<dr::DR_SPEC>;
///data register
pub mod dr;
///CRCPR register accessor: an alias for `Reg<CRCPR_SPEC>`
pub type CRCPR = crate::Reg<crcpr::CRCPR_SPEC>;
///CRC polynomial register
pub mod crcpr;
///RXCRCR register accessor: an alias for `Reg<RXCRCR_SPEC>`
pub type RXCRCR = crate::Reg<rxcrcr::RXCRCR_SPEC>;
///RX CRC register
pub mod rxcrcr;
///TXCRCR register accessor: an alias for `Reg<TXCRCR_SPEC>`
pub type TXCRCR = crate::Reg<txcrcr::TXCRCR_SPEC>;
///TX CRC register
pub mod txcrcr;
///I2SCFGR register accessor: an alias for `Reg<I2SCFGR_SPEC>`
pub type I2SCFGR = crate::Reg<i2scfgr::I2SCFGR_SPEC>;
///I2S configuration register
pub mod i2scfgr;
///I2SPR register accessor: an alias for `Reg<I2SPR_SPEC>`
pub type I2SPR = crate::Reg<i2spr::I2SPR_SPEC>;
///I2S prescaler register
pub mod i2spr;
