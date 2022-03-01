///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register
    pub cr: crate::Reg<cr::CR_SPEC>,
    ///0x04 - interrupt enable register
    pub ier: crate::Reg<ier::IER_SPEC>,
    ///0x08 - interrupt clear register
    pub icr: crate::Reg<icr::ICR_SPEC>,
    ///0x0c - interrupt status register
    pub isr: crate::Reg<isr::ISR_SPEC>,
    ///0x10 - I/O hysteresis control register
    pub iohcr: crate::Reg<iohcr::IOHCR_SPEC>,
    _reserved5: [u8; 0x04],
    ///0x18 - I/O analog switch control register
    pub ioascr: crate::Reg<ioascr::IOASCR_SPEC>,
    _reserved6: [u8; 0x04],
    ///0x20 - I/O sampling control register
    pub ioscr: crate::Reg<ioscr::IOSCR_SPEC>,
    _reserved7: [u8; 0x04],
    ///0x28 - I/O channel control register
    pub ioccr: crate::Reg<ioccr::IOCCR_SPEC>,
    _reserved8: [u8; 0x04],
    ///0x30 - I/O group control status register
    pub iogcsr: crate::Reg<iogcsr::IOGCSR_SPEC>,
    ///0x34 - I/O group x counter register
    pub iog1cr: crate::Reg<iogcr::IOGCR_SPEC>,
    ///0x38 - I/O group x counter register
    pub iog2cr: crate::Reg<iogcr::IOGCR_SPEC>,
    ///0x3c - I/O group x counter register
    pub iog3cr: crate::Reg<iogcr::IOGCR_SPEC>,
    ///0x40 - I/O group x counter register
    pub iog4cr: crate::Reg<iogcr::IOGCR_SPEC>,
    ///0x44 - I/O group x counter register
    pub iog5cr: crate::Reg<iogcr::IOGCR_SPEC>,
    ///0x48 - I/O group x counter register
    pub iog6cr: crate::Reg<iogcr::IOGCR_SPEC>,
    ///0x4c - I/O group x counter register
    pub iog7cr: crate::Reg<iogcr::IOGCR_SPEC>,
    ///0x50 - I/O group x counter register
    pub iog8cr: crate::Reg<iogcr::IOGCR_SPEC>,
}
///CR register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///control register
pub mod cr;
///IER register accessor: an alias for `Reg<IER_SPEC>`
pub type IER = crate::Reg<ier::IER_SPEC>;
///interrupt enable register
pub mod ier;
///ICR register accessor: an alias for `Reg<ICR_SPEC>`
pub type ICR = crate::Reg<icr::ICR_SPEC>;
///interrupt clear register
pub mod icr;
///ISR register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///interrupt status register
pub mod isr;
///IOHCR register accessor: an alias for `Reg<IOHCR_SPEC>`
pub type IOHCR = crate::Reg<iohcr::IOHCR_SPEC>;
///I/O hysteresis control register
pub mod iohcr;
///IOASCR register accessor: an alias for `Reg<IOASCR_SPEC>`
pub type IOASCR = crate::Reg<ioascr::IOASCR_SPEC>;
///I/O analog switch control register
pub mod ioascr;
///IOSCR register accessor: an alias for `Reg<IOSCR_SPEC>`
pub type IOSCR = crate::Reg<ioscr::IOSCR_SPEC>;
///I/O sampling control register
pub mod ioscr;
///IOCCR register accessor: an alias for `Reg<IOCCR_SPEC>`
pub type IOCCR = crate::Reg<ioccr::IOCCR_SPEC>;
///I/O channel control register
pub mod ioccr;
///IOGCSR register accessor: an alias for `Reg<IOGCSR_SPEC>`
pub type IOGCSR = crate::Reg<iogcsr::IOGCSR_SPEC>;
///I/O group control status register
pub mod iogcsr;
///IOGCR register accessor: an alias for `Reg<IOGCR_SPEC>`
pub type IOGCR = crate::Reg<iogcr::IOGCR_SPEC>;
///I/O group x counter register
pub mod iogcr;
