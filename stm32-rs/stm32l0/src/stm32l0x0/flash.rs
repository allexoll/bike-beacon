///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Access control register
    pub acr: crate::Reg<acr::ACR_SPEC>,
    ///0x04 - Program/erase control register
    pub pecr: crate::Reg<pecr::PECR_SPEC>,
    ///0x08 - Power down key register
    pub pdkeyr: crate::Reg<pdkeyr::PDKEYR_SPEC>,
    ///0x0c - Program/erase key register
    pub pekeyr: crate::Reg<pekeyr::PEKEYR_SPEC>,
    ///0x10 - Program memory key register
    pub prgkeyr: crate::Reg<prgkeyr::PRGKEYR_SPEC>,
    ///0x14 - Option byte key register
    pub optkeyr: crate::Reg<optkeyr::OPTKEYR_SPEC>,
    ///0x18 - Status register
    pub sr: crate::Reg<sr::SR_SPEC>,
    ///0x1c - Option byte register
    pub optr: crate::Reg<optr::OPTR_SPEC>,
    ///0x20 - Write protection register
    pub wrprot1: crate::Reg<wrprot1::WRPROT1_SPEC>,
    _reserved9: [u8; 0x5c],
    ///0x80 - Write protection register
    pub wrprot2: crate::Reg<wrprot2::WRPROT2_SPEC>,
}
///ACR register accessor: an alias for `Reg<ACR_SPEC>`
pub type ACR = crate::Reg<acr::ACR_SPEC>;
///Access control register
pub mod acr;
///PECR register accessor: an alias for `Reg<PECR_SPEC>`
pub type PECR = crate::Reg<pecr::PECR_SPEC>;
///Program/erase control register
pub mod pecr;
///PDKEYR register accessor: an alias for `Reg<PDKEYR_SPEC>`
pub type PDKEYR = crate::Reg<pdkeyr::PDKEYR_SPEC>;
///Power down key register
pub mod pdkeyr;
///PEKEYR register accessor: an alias for `Reg<PEKEYR_SPEC>`
pub type PEKEYR = crate::Reg<pekeyr::PEKEYR_SPEC>;
///Program/erase key register
pub mod pekeyr;
///PRGKEYR register accessor: an alias for `Reg<PRGKEYR_SPEC>`
pub type PRGKEYR = crate::Reg<prgkeyr::PRGKEYR_SPEC>;
///Program memory key register
pub mod prgkeyr;
///OPTKEYR register accessor: an alias for `Reg<OPTKEYR_SPEC>`
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYR_SPEC>;
///Option byte key register
pub mod optkeyr;
///SR register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///Status register
pub mod sr;
///OPTR register accessor: an alias for `Reg<OPTR_SPEC>`
pub type OPTR = crate::Reg<optr::OPTR_SPEC>;
///Option byte register
pub mod optr;
///WRPROT1 register accessor: an alias for `Reg<WRPROT1_SPEC>`
pub type WRPROT1 = crate::Reg<wrprot1::WRPROT1_SPEC>;
///Write protection register
pub mod wrprot1;
///WRPROT2 register accessor: an alias for `Reg<WRPROT2_SPEC>`
pub type WRPROT2 = crate::Reg<wrprot2::WRPROT2_SPEC>;
///Write protection register
pub mod wrprot2;
