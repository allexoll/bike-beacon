///CR register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///channel x configuration register
pub mod cr;
///NDTR register accessor: an alias for `Reg<NDTR_SPEC>`
pub type NDTR = crate::Reg<ndtr::NDTR_SPEC>;
///channel x number of data register
pub mod ndtr;
///PAR register accessor: an alias for `Reg<PAR_SPEC>`
pub type PAR = crate::Reg<par::PAR_SPEC>;
///channel x peripheral address register
pub mod par;
///MAR register accessor: an alias for `Reg<MAR_SPEC>`
pub type MAR = crate::Reg<mar::MAR_SPEC>;
///channel x memory address register
pub mod mar;
