///Register `CIFR` reader
pub struct R(crate::R<CIFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIFR_SPEC>) -> Self {
        R(reader)
    }
}
///Clock Security System Interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSSHSEF_A {
    ///0: No clock security interrupt caused by HSE clock failure
    NOCLOCK = 0,
    ///1: Clock security interrupt caused by HSE clock failure
    CLOCK = 1,
}
impl From<CSSHSEF_A> for bool {
    #[inline(always)]
    fn from(variant: CSSHSEF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSSHSEF` reader - Clock Security System Interrupt flag
pub struct CSSHSEF_R(crate::FieldReader<bool, CSSHSEF_A>);
impl CSSHSEF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSSHSEF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CSSHSEF_A {
        match self.bits {
            false => CSSHSEF_A::NOCLOCK,
            true => CSSHSEF_A::CLOCK,
        }
    }
    ///Checks if the value of the field is `NOCLOCK`
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        **self == CSSHSEF_A::NOCLOCK
    }
    ///Checks if the value of the field is `CLOCK`
    #[inline(always)]
    pub fn is_clock(&self) -> bool {
        **self == CSSHSEF_A::CLOCK
    }
}
impl core::ops::Deref for CSSHSEF_R {
    type Target = crate::FieldReader<bool, CSSHSEF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///LSE Clock Security System Interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSSLSEF_A {
    ///0: No failure detected on LSE clock failure
    NOFAILURE = 0,
    ///1: Failure detected on LSE clock failure
    FAILURE = 1,
}
impl From<CSSLSEF_A> for bool {
    #[inline(always)]
    fn from(variant: CSSLSEF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSSLSEF` reader - LSE Clock Security System Interrupt flag
pub struct CSSLSEF_R(crate::FieldReader<bool, CSSLSEF_A>);
impl CSSLSEF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSSLSEF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CSSLSEF_A {
        match self.bits {
            false => CSSLSEF_A::NOFAILURE,
            true => CSSLSEF_A::FAILURE,
        }
    }
    ///Checks if the value of the field is `NOFAILURE`
    #[inline(always)]
    pub fn is_no_failure(&self) -> bool {
        **self == CSSLSEF_A::NOFAILURE
    }
    ///Checks if the value of the field is `FAILURE`
    #[inline(always)]
    pub fn is_failure(&self) -> bool {
        **self == CSSLSEF_A::FAILURE
    }
}
impl core::ops::Deref for CSSLSEF_R {
    type Target = crate::FieldReader<bool, CSSLSEF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///MSI ready interrupt flag
pub type MSIRDYF_A = LSIRDYF_A;
///Field `MSIRDYF` reader - MSI ready interrupt flag
pub type MSIRDYF_R = LSIRDYF_R;
///PLL ready interrupt flag
pub type PLLRDYF_A = LSIRDYF_A;
///Field `PLLRDYF` reader - PLL ready interrupt flag
pub type PLLRDYF_R = LSIRDYF_R;
///HSE ready interrupt flag
pub type HSERDYF_A = LSIRDYF_A;
///Field `HSERDYF` reader - HSE ready interrupt flag
pub type HSERDYF_R = LSIRDYF_R;
///HSI16 ready interrupt flag
pub type HSI16RDYF_A = LSIRDYF_A;
///Field `HSI16RDYF` reader - HSI16 ready interrupt flag
pub type HSI16RDYF_R = LSIRDYF_R;
///LSE ready interrupt flag
pub type LSERDYF_A = LSIRDYF_A;
///Field `LSERDYF` reader - LSE ready interrupt flag
pub type LSERDYF_R = LSIRDYF_R;
///LSI ready interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSIRDYF_A {
    ///0: No clock ready interrupt
    NOTINTERRUPTED = 0,
    ///1: Clock ready interrupt
    INTERRUPTED = 1,
}
impl From<LSIRDYF_A> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LSIRDYF` reader - LSI ready interrupt flag
pub struct LSIRDYF_R(crate::FieldReader<bool, LSIRDYF_A>);
impl LSIRDYF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSIRDYF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSIRDYF_A {
        match self.bits {
            false => LSIRDYF_A::NOTINTERRUPTED,
            true => LSIRDYF_A::INTERRUPTED,
        }
    }
    ///Checks if the value of the field is `NOTINTERRUPTED`
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        **self == LSIRDYF_A::NOTINTERRUPTED
    }
    ///Checks if the value of the field is `INTERRUPTED`
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        **self == LSIRDYF_A::INTERRUPTED
    }
}
impl core::ops::Deref for LSIRDYF_R {
    type Target = crate::FieldReader<bool, LSIRDYF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bit 8 - Clock Security System Interrupt flag
    #[inline(always)]
    pub fn csshsef(&self) -> CSSHSEF_R {
        CSSHSEF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 7 - LSE Clock Security System Interrupt flag
    #[inline(always)]
    pub fn csslsef(&self) -> CSSLSEF_R {
        CSSLSEF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 5 - MSI ready interrupt flag
    #[inline(always)]
    pub fn msirdyf(&self) -> MSIRDYF_R {
        MSIRDYF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - PLL ready interrupt flag
    #[inline(always)]
    pub fn pllrdyf(&self) -> PLLRDYF_R {
        PLLRDYF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - HSE ready interrupt flag
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - HSI16 ready interrupt flag
    #[inline(always)]
    pub fn hsi16rdyf(&self) -> HSI16RDYF_R {
        HSI16RDYF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - LSE ready interrupt flag
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - LSI ready interrupt flag
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 0x01) != 0)
    }
}
///Clock interrupt flag register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cifr](index.html) module
pub struct CIFR_SPEC;
impl crate::RegisterSpec for CIFR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cifr::R](R) reader structure
impl crate::Readable for CIFR_SPEC {
    type Reader = R;
}
///`reset()` method sets CIFR to value 0
impl crate::Resettable for CIFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
