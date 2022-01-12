///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - interrupt status register
    pub isr: crate::Reg<isr::ISR_SPEC>,
    ///0x04 - interrupt flag clear register
    pub ifcr: crate::Reg<ifcr::IFCR_SPEC>,
    ///0x08..0x18 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
    pub ch1: CH,
    _reserved3: [u8; 0x04],
    ///0x1c..0x2c - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
    pub ch2: CH,
    _reserved4: [u8; 0x04],
    ///0x30..0x40 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
    pub ch3: CH,
    _reserved5: [u8; 0x04],
    ///0x44..0x54 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
    pub ch4: CH,
    _reserved6: [u8; 0x04],
    ///0x58..0x68 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
    pub ch5: CH,
    _reserved7: [u8; 0x04],
    ///0x6c..0x7c - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
    pub ch6: CH,
    _reserved8: [u8; 0x04],
    ///0x80..0x90 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
    pub ch7: CH,
    _reserved9: [u8; 0x18],
    ///0xa8 - channel selection register
    pub cselr: crate::Reg<cselr::CSELR_SPEC>,
}
///Register block
#[repr(C)]
pub struct CH {
    ///0x00 - channel x configuration register
    pub cr: crate::Reg<self::ch::cr::CR_SPEC>,
    ///0x04 - channel x number of data register
    pub ndtr: crate::Reg<self::ch::ndtr::NDTR_SPEC>,
    ///0x08 - channel x peripheral address register
    pub par: crate::Reg<self::ch::par::PAR_SPEC>,
    ///0x0c - channel x memory address register
    pub mar: crate::Reg<self::ch::mar::MAR_SPEC>,
}
///Register block
///Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
pub mod ch;
///ISR register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///interrupt status register
pub mod isr;
///IFCR register accessor: an alias for `Reg<IFCR_SPEC>`
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
///interrupt flag clear register
pub mod ifcr;
///CSELR register accessor: an alias for `Reg<CSELR_SPEC>`
pub type CSELR = crate::Reg<cselr::CSELR_SPEC>;
///channel selection register
pub mod cselr;
