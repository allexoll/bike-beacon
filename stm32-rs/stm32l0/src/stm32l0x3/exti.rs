///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Interrupt mask register (EXTI_IMR)
    pub imr: crate::Reg<imr::IMR_SPEC>,
    ///0x04 - Event mask register (EXTI_EMR)
    pub emr: crate::Reg<emr::EMR_SPEC>,
    ///0x08 - Rising Trigger selection register (EXTI_RTSR)
    pub rtsr: crate::Reg<rtsr::RTSR_SPEC>,
    ///0x0c - Falling Trigger selection register (EXTI_FTSR)
    pub ftsr: crate::Reg<ftsr::FTSR_SPEC>,
    ///0x10 - Software interrupt event register (EXTI_SWIER)
    pub swier: crate::Reg<swier::SWIER_SPEC>,
    ///0x14 - Pending register (EXTI_PR)
    pub pr: crate::Reg<pr::PR_SPEC>,
}
///IMR register accessor: an alias for `Reg<IMR_SPEC>`
pub type IMR = crate::Reg<imr::IMR_SPEC>;
///Interrupt mask register (EXTI_IMR)
pub mod imr;
///EMR register accessor: an alias for `Reg<EMR_SPEC>`
pub type EMR = crate::Reg<emr::EMR_SPEC>;
///Event mask register (EXTI_EMR)
pub mod emr;
///RTSR register accessor: an alias for `Reg<RTSR_SPEC>`
pub type RTSR = crate::Reg<rtsr::RTSR_SPEC>;
///Rising Trigger selection register (EXTI_RTSR)
pub mod rtsr;
///FTSR register accessor: an alias for `Reg<FTSR_SPEC>`
pub type FTSR = crate::Reg<ftsr::FTSR_SPEC>;
///Falling Trigger selection register (EXTI_FTSR)
pub mod ftsr;
///SWIER register accessor: an alias for `Reg<SWIER_SPEC>`
pub type SWIER = crate::Reg<swier::SWIER_SPEC>;
///Software interrupt event register (EXTI_SWIER)
pub mod swier;
///PR register accessor: an alias for `Reg<PR_SPEC>`
pub type PR = crate::Reg<pr::PR_SPEC>;
///Pending register (EXTI_PR)
pub mod pr;
