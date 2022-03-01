///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - SYSCFG configuration register 1
    pub cfgr1: crate::Reg<cfgr1::CFGR1_SPEC>,
    ///0x04 - SYSCFG configuration register 2
    pub cfgr2: crate::Reg<cfgr2::CFGR2_SPEC>,
    ///0x08 - external interrupt configuration register 1
    pub exticr1: crate::Reg<exticr1::EXTICR1_SPEC>,
    ///0x0c - external interrupt configuration register 2
    pub exticr2: crate::Reg<exticr2::EXTICR2_SPEC>,
    ///0x10 - external interrupt configuration register 3
    pub exticr3: crate::Reg<exticr3::EXTICR3_SPEC>,
    ///0x14 - external interrupt configuration register 4
    pub exticr4: crate::Reg<exticr4::EXTICR4_SPEC>,
    _reserved6: [u8; 0x08],
    ///0x20 - SYSCFG configuration register 3
    pub cfgr3: crate::Reg<cfgr3::CFGR3_SPEC>,
}
///CFGR1 register accessor: an alias for `Reg<CFGR1_SPEC>`
pub type CFGR1 = crate::Reg<cfgr1::CFGR1_SPEC>;
///SYSCFG configuration register 1
pub mod cfgr1;
///CFGR2 register accessor: an alias for `Reg<CFGR2_SPEC>`
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
///SYSCFG configuration register 2
pub mod cfgr2;
///EXTICR1 register accessor: an alias for `Reg<EXTICR1_SPEC>`
pub type EXTICR1 = crate::Reg<exticr1::EXTICR1_SPEC>;
///external interrupt configuration register 1
pub mod exticr1;
///EXTICR2 register accessor: an alias for `Reg<EXTICR2_SPEC>`
pub type EXTICR2 = crate::Reg<exticr2::EXTICR2_SPEC>;
///external interrupt configuration register 2
pub mod exticr2;
///EXTICR3 register accessor: an alias for `Reg<EXTICR3_SPEC>`
pub type EXTICR3 = crate::Reg<exticr3::EXTICR3_SPEC>;
///external interrupt configuration register 3
pub mod exticr3;
///EXTICR4 register accessor: an alias for `Reg<EXTICR4_SPEC>`
pub type EXTICR4 = crate::Reg<exticr4::EXTICR4_SPEC>;
///external interrupt configuration register 4
pub mod exticr4;
///CFGR3 register accessor: an alias for `Reg<CFGR3_SPEC>`
pub type CFGR3 = crate::Reg<cfgr3::CFGR3_SPEC>;
///SYSCFG configuration register 3
pub mod cfgr3;
