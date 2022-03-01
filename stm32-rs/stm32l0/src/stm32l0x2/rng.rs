///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register
    pub cr: crate::Reg<cr::CR_SPEC>,
    ///0x04 - status register
    pub sr: crate::Reg<sr::SR_SPEC>,
    ///0x08 - data register
    pub dr: crate::Reg<dr::DR_SPEC>,
}
///CR register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///control register
pub mod cr;
///SR register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///status register
pub mod sr;
///DR register accessor: an alias for `Reg<DR_SPEC>`
pub type DR = crate::Reg<dr::DR_SPEC>;
///data register
pub mod dr;
