///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - SysTick control and status register
    pub csr: crate::Reg<csr::CSR_SPEC>,
    ///0x04 - SysTick reload value register
    pub rvr: crate::Reg<rvr::RVR_SPEC>,
    ///0x08 - SysTick current value register
    pub cvr: crate::Reg<cvr::CVR_SPEC>,
    ///0x0c - SysTick calibration value register
    pub calib: crate::Reg<calib::CALIB_SPEC>,
}
///CSR register accessor: an alias for `Reg<CSR_SPEC>`
pub type CSR = crate::Reg<csr::CSR_SPEC>;
///SysTick control and status register
pub mod csr;
///RVR register accessor: an alias for `Reg<RVR_SPEC>`
pub type RVR = crate::Reg<rvr::RVR_SPEC>;
///SysTick reload value register
pub mod rvr;
///CVR register accessor: an alias for `Reg<CVR_SPEC>`
pub type CVR = crate::Reg<cvr::CVR_SPEC>;
///SysTick current value register
pub mod cvr;
///CALIB register accessor: an alias for `Reg<CALIB_SPEC>`
pub type CALIB = crate::Reg<calib::CALIB_SPEC>;
///SysTick calibration value register
pub mod calib;
