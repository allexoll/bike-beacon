///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Control register
    pub cr: crate::Reg<cr::CR_SPEC>,
    ///0x04 - Configuration register
    pub cfr: crate::Reg<cfr::CFR_SPEC>,
    ///0x08 - Status register
    pub sr: crate::Reg<sr::SR_SPEC>,
}
///CR register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///Control register
pub mod cr;
///CFR register accessor: an alias for `Reg<CFR_SPEC>`
pub type CFR = crate::Reg<cfr::CFR_SPEC>;
///Configuration register
pub mod cfr;
///SR register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///Status register
pub mod sr;
