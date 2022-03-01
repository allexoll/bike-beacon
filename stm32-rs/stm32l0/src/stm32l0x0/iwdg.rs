///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Key register
    pub kr: crate::Reg<kr::KR_SPEC>,
    ///0x04 - Prescaler register
    pub pr: crate::Reg<pr::PR_SPEC>,
    ///0x08 - Reload register
    pub rlr: crate::Reg<rlr::RLR_SPEC>,
    ///0x0c - Status register
    pub sr: crate::Reg<sr::SR_SPEC>,
    ///0x10 - Window register
    pub winr: crate::Reg<winr::WINR_SPEC>,
}
///KR register accessor: an alias for `Reg<KR_SPEC>`
pub type KR = crate::Reg<kr::KR_SPEC>;
///Key register
pub mod kr;
///PR register accessor: an alias for `Reg<PR_SPEC>`
pub type PR = crate::Reg<pr::PR_SPEC>;
///Prescaler register
pub mod pr;
///RLR register accessor: an alias for `Reg<RLR_SPEC>`
pub type RLR = crate::Reg<rlr::RLR_SPEC>;
///Reload register
pub mod rlr;
///SR register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///Status register
pub mod sr;
///WINR register accessor: an alias for `Reg<WINR_SPEC>`
pub type WINR = crate::Reg<winr::WINR_SPEC>;
///Window register
pub mod winr;
