///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register 1
    pub cr1: crate::Reg<cr1::CR1_SPEC>,
    ///0x04 - control register 2
    pub cr2: crate::Reg<cr2::CR2_SPEC>,
    ///0x08 - slave mode control register
    pub smcr: crate::Reg<smcr::SMCR_SPEC>,
    ///0x0c - DMA/Interrupt enable register
    pub dier: crate::Reg<dier::DIER_SPEC>,
    ///0x10 - status register
    pub sr: crate::Reg<sr::SR_SPEC>,
    ///0x14 - event generation register
    pub egr: crate::Reg<egr::EGR_SPEC>,
    _reserved_6_ccmr1: [u8; 0x04],
    _reserved_7_ccmr2: [u8; 0x04],
    ///0x20 - capture/compare enable register
    pub ccer: crate::Reg<ccer::CCER_SPEC>,
    ///0x24 - TIMx counter
    pub cnt: crate::Reg<cnt::CNT_SPEC>,
    _reserved10: [u8; 0x02],
    ///0x28 - prescaler
    pub psc: crate::Reg<psc::PSC_SPEC>,
    ///0x2c - TIMx auto-reload register
    pub arr: crate::Reg<arr::ARR_SPEC>,
    _reserved12: [u8; 0x06],
    ///0x34 - capture/compare register
    pub ccr1: crate::Reg<ccr::CCR_SPEC>,
    _reserved13: [u8; 0x02],
    ///0x38 - capture/compare register
    pub ccr2: crate::Reg<ccr::CCR_SPEC>,
    _reserved14: [u8; 0x02],
    ///0x3c - capture/compare register
    pub ccr3: crate::Reg<ccr::CCR_SPEC>,
    _reserved15: [u8; 0x02],
    ///0x40 - capture/compare register
    pub ccr4: crate::Reg<ccr::CCR_SPEC>,
    _reserved16: [u8; 0x06],
    ///0x48 - DMA control register
    pub dcr: crate::Reg<dcr::DCR_SPEC>,
    ///0x4c - DMA address for full transfer
    pub dmar: crate::Reg<dmar::DMAR_SPEC>,
    ///0x50 - TIM2 option register
    pub or: crate::Reg<or::OR_SPEC>,
}
impl RegisterBlock {
    ///0x18 - capture/compare mode register 1 (input mode)
    #[inline(always)]
    pub fn ccmr1_input(&self) -> &crate::Reg<ccmr1_input::CCMR1_INPUT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(24usize)
                as *const crate::Reg<ccmr1_input::CCMR1_INPUT_SPEC>)
        }
    }
    ///0x18 - capture/compare mode register 1 (output mode)
    #[inline(always)]
    pub fn ccmr1_output(&self) -> &crate::Reg<ccmr1_output::CCMR1_OUTPUT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(24usize)
                as *const crate::Reg<ccmr1_output::CCMR1_OUTPUT_SPEC>)
        }
    }
    ///0x1c - capture/compare mode register 2 (input mode)
    #[inline(always)]
    pub fn ccmr2_input(&self) -> &crate::Reg<ccmr2_input::CCMR2_INPUT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize)
                as *const crate::Reg<ccmr2_input::CCMR2_INPUT_SPEC>)
        }
    }
    ///0x1c - capture/compare mode register 2 (output mode)
    #[inline(always)]
    pub fn ccmr2_output(&self) -> &crate::Reg<ccmr2_output::CCMR2_OUTPUT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize)
                as *const crate::Reg<ccmr2_output::CCMR2_OUTPUT_SPEC>)
        }
    }
}
///CR1 register accessor: an alias for `Reg<CR1_SPEC>`
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
///control register 1
pub mod cr1;
///CR2 register accessor: an alias for `Reg<CR2_SPEC>`
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
///control register 2
pub mod cr2;
///SMCR register accessor: an alias for `Reg<SMCR_SPEC>`
pub type SMCR = crate::Reg<smcr::SMCR_SPEC>;
///slave mode control register
pub mod smcr;
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
///CCMR1_Output register accessor: an alias for `Reg<CCMR1_OUTPUT_SPEC>`
pub type CCMR1_OUTPUT = crate::Reg<ccmr1_output::CCMR1_OUTPUT_SPEC>;
///capture/compare mode register 1 (output mode)
pub mod ccmr1_output;
///CCMR1_Input register accessor: an alias for `Reg<CCMR1_INPUT_SPEC>`
pub type CCMR1_INPUT = crate::Reg<ccmr1_input::CCMR1_INPUT_SPEC>;
///capture/compare mode register 1 (input mode)
pub mod ccmr1_input;
///CCMR2_Output register accessor: an alias for `Reg<CCMR2_OUTPUT_SPEC>`
pub type CCMR2_OUTPUT = crate::Reg<ccmr2_output::CCMR2_OUTPUT_SPEC>;
///capture/compare mode register 2 (output mode)
pub mod ccmr2_output;
///CCMR2_Input register accessor: an alias for `Reg<CCMR2_INPUT_SPEC>`
pub type CCMR2_INPUT = crate::Reg<ccmr2_input::CCMR2_INPUT_SPEC>;
///capture/compare mode register 2 (input mode)
pub mod ccmr2_input;
///CCER register accessor: an alias for `Reg<CCER_SPEC>`
pub type CCER = crate::Reg<ccer::CCER_SPEC>;
///capture/compare enable register
pub mod ccer;
///PSC register accessor: an alias for `Reg<PSC_SPEC>`
pub type PSC = crate::Reg<psc::PSC_SPEC>;
///prescaler
pub mod psc;
///DCR register accessor: an alias for `Reg<DCR_SPEC>`
pub type DCR = crate::Reg<dcr::DCR_SPEC>;
///DMA control register
pub mod dcr;
///DMAR register accessor: an alias for `Reg<DMAR_SPEC>`
pub type DMAR = crate::Reg<dmar::DMAR_SPEC>;
///DMA address for full transfer
pub mod dmar;
///OR register accessor: an alias for `Reg<OR_SPEC>`
pub type OR = crate::Reg<or::OR_SPEC>;
///TIM2 option register
pub mod or;
///CNT register accessor: an alias for `Reg<CNT_SPEC>`
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
///TIMx counter
pub mod cnt;
///ARR register accessor: an alias for `Reg<ARR_SPEC>`
pub type ARR = crate::Reg<arr::ARR_SPEC>;
///TIMx auto-reload register
pub mod arr;
///CCR register accessor: an alias for `Reg<CCR_SPEC>`
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
///capture/compare register
pub mod ccr;
