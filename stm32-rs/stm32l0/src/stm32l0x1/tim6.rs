///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register 1
    pub cr1: crate::Reg<cr1::CR1_SPEC>,
    ///0x04 - control register 2
    pub cr2: crate::Reg<cr2::CR2_SPEC>,
    _reserved2: [u8; 0x04],
    ///0x0c - DMA/Interrupt enable register
    pub dier: crate::Reg<dier::DIER_SPEC>,
    ///0x10 - status register
    pub sr: crate::Reg<sr::SR_SPEC>,
    ///0x14 - event generation register
    pub egr: crate::Reg<egr::EGR_SPEC>,
    _reserved5: [u8; 0x0c],
    ///0x24 - counter
    pub cnt: crate::Reg<cnt::CNT_SPEC>,
    _reserved6: [u8; 0x02],
    ///0x28 - prescaler
    pub psc: crate::Reg<psc::PSC_SPEC>,
    ///0x2c - auto-reload register
    pub arr: crate::Reg<arr::ARR_SPEC>,
}
///CR1 register accessor: an alias for `Reg<CR1_SPEC>`
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
///control register 1
pub mod cr1;
///CR2 register accessor: an alias for `Reg<CR2_SPEC>`
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
///control register 2
pub mod cr2;
///DIER register accessor: an alias for `Reg<DIER_SPEC>`
pub type DIER = crate::Reg<dier::DIER_SPEC>;
///DMA/Interrupt enable register
pub mod dier;
///SR register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///status register
pub mod sr;
///EGR register accessor: an alias for `Reg<EGR_SPEC>`
pub type EGR = crate::Reg<egr::EGR_SPEC>;
///event generation register
pub mod egr;
///CNT register accessor: an alias for `Reg<CNT_SPEC>`
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
///counter
pub mod cnt;
///PSC register accessor: an alias for `Reg<PSC_SPEC>`
pub type PSC = crate::Reg<psc::PSC_SPEC>;
///prescaler
pub mod psc;
///ARR register accessor: an alias for `Reg<ARR_SPEC>`
pub type ARR = crate::Reg<arr::ARR_SPEC>;
///auto-reload register
pub mod arr;
