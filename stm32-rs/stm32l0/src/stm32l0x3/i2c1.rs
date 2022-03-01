///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Control register 1
    pub cr1: crate::Reg<cr1::CR1_SPEC>,
    ///0x04 - Control register 2
    pub cr2: crate::Reg<cr2::CR2_SPEC>,
    ///0x08 - Own address register 1
    pub oar1: crate::Reg<oar1::OAR1_SPEC>,
    ///0x0c - Own address register 2
    pub oar2: crate::Reg<oar2::OAR2_SPEC>,
    ///0x10 - Timing register
    pub timingr: crate::Reg<timingr::TIMINGR_SPEC>,
    ///0x14 - Status register 1
    pub timeoutr: crate::Reg<timeoutr::TIMEOUTR_SPEC>,
    ///0x18 - Interrupt and Status register
    pub isr: crate::Reg<isr::ISR_SPEC>,
    ///0x1c - Interrupt clear register
    pub icr: crate::Reg<icr::ICR_SPEC>,
    ///0x20 - PEC register
    pub pecr: crate::Reg<pecr::PECR_SPEC>,
    ///0x24 - Receive data register
    pub rxdr: crate::Reg<rxdr::RXDR_SPEC>,
    ///0x28 - Transmit data register
    pub txdr: crate::Reg<txdr::TXDR_SPEC>,
}
///CR1 register accessor: an alias for `Reg<CR1_SPEC>`
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
///Control register 1
pub mod cr1;
///CR2 register accessor: an alias for `Reg<CR2_SPEC>`
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
///Control register 2
pub mod cr2;
///OAR1 register accessor: an alias for `Reg<OAR1_SPEC>`
pub type OAR1 = crate::Reg<oar1::OAR1_SPEC>;
///Own address register 1
pub mod oar1;
///OAR2 register accessor: an alias for `Reg<OAR2_SPEC>`
pub type OAR2 = crate::Reg<oar2::OAR2_SPEC>;
///Own address register 2
pub mod oar2;
///TIMINGR register accessor: an alias for `Reg<TIMINGR_SPEC>`
pub type TIMINGR = crate::Reg<timingr::TIMINGR_SPEC>;
///Timing register
pub mod timingr;
///TIMEOUTR register accessor: an alias for `Reg<TIMEOUTR_SPEC>`
pub type TIMEOUTR = crate::Reg<timeoutr::TIMEOUTR_SPEC>;
///Status register 1
pub mod timeoutr;
///ISR register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///Interrupt and Status register
pub mod isr;
///ICR register accessor: an alias for `Reg<ICR_SPEC>`
pub type ICR = crate::Reg<icr::ICR_SPEC>;
///Interrupt clear register
pub mod icr;
///PECR register accessor: an alias for `Reg<PECR_SPEC>`
pub type PECR = crate::Reg<pecr::PECR_SPEC>;
///PEC register
pub mod pecr;
///RXDR register accessor: an alias for `Reg<RXDR_SPEC>`
pub type RXDR = crate::Reg<rxdr::RXDR_SPEC>;
///Receive data register
pub mod rxdr;
///TXDR register accessor: an alias for `Reg<TXDR_SPEC>`
pub type TXDR = crate::Reg<txdr::TXDR_SPEC>;
///Transmit data register
pub mod txdr;
