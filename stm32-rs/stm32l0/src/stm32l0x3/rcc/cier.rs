///Register `CIER` reader
pub struct R(crate::R<CIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIER_SPEC>) -> Self {
        R(reader)
    }
}
///LSE CSS interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSSLSE_A {
    ///0: LSE CSS interrupt disabled
    DISABLED = 0,
    ///1: LSE CSS interrupt enabled
    ENABLED = 1,
}
impl From<CSSLSE_A> for bool {
    #[inline(always)]
    fn from(variant: CSSLSE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSSLSE` reader - LSE CSS interrupt flag
pub struct CSSLSE_R(crate::FieldReader<bool, CSSLSE_A>);
impl CSSLSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSSLSE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CSSLSE_A {
        match self.bits {
            false => CSSLSE_A::DISABLED,
            true => CSSLSE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CSSLSE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CSSLSE_A::ENABLED
    }
}
impl core::ops::Deref for CSSLSE_R {
    type Target = crate::FieldReader<bool, CSSLSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///HSI48 ready interrupt flag
pub type HSI48RDYIE_A = LSIRDYIE_A;
///Field `HSI48RDYIE` reader - HSI48 ready interrupt flag
pub type HSI48RDYIE_R = LSIRDYIE_R;
///MSI ready interrupt flag
pub type MSIRDYIE_A = LSIRDYIE_A;
///Field `MSIRDYIE` reader - MSI ready interrupt flag
pub type MSIRDYIE_R = LSIRDYIE_R;
///PLL ready interrupt flag
pub type PLLRDYIE_A = LSIRDYIE_A;
///Field `PLLRDYIE` reader - PLL ready interrupt flag
pub type PLLRDYIE_R = LSIRDYIE_R;
///HSE ready interrupt flag
pub type HSERDYIE_A = LSIRDYIE_A;
///Field `HSERDYIE` reader - HSE ready interrupt flag
pub type HSERDYIE_R = LSIRDYIE_R;
///HSI16 ready interrupt flag
pub type HSI16RDYIE_A = LSIRDYIE_A;
///Field `HSI16RDYIE` reader - HSI16 ready interrupt flag
pub type HSI16RDYIE_R = LSIRDYIE_R;
///LSE ready interrupt flag
pub type LSERDYIE_A = LSIRDYIE_A;
///Field `LSERDYIE` reader - LSE ready interrupt flag
pub type LSERDYIE_R = LSIRDYIE_R;
///LSI ready interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSIRDYIE_A {
    ///0: Ready interrupt disabled
    DISABLED = 0,
    ///1: Ready interrupt enabled
    ENABLED = 1,
}
impl From<LSIRDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LSIRDYIE` reader - LSI ready interrupt flag
pub struct LSIRDYIE_R(crate::FieldReader<bool, LSIRDYIE_A>);
impl LSIRDYIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSIRDYIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSIRDYIE_A {
        match self.bits {
            false => LSIRDYIE_A::DISABLED,
            true => LSIRDYIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == LSIRDYIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == LSIRDYIE_A::ENABLED
    }
}
impl core::ops::Deref for LSIRDYIE_R {
    type Target = crate::FieldReader<bool, LSIRDYIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bit 7 - LSE CSS interrupt flag
    #[inline(always)]
    pub fn csslse(&self) -> CSSLSE_R {
        CSSLSE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 6 - HSI48 ready interrupt flag
    #[inline(always)]
    pub fn hsi48rdyie(&self) -> HSI48RDYIE_R {
        HSI48RDYIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - MSI ready interrupt flag
    #[inline(always)]
    pub fn msirdyie(&self) -> MSIRDYIE_R {
        MSIRDYIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - PLL ready interrupt flag
    #[inline(always)]
    pub fn pllrdyie(&self) -> PLLRDYIE_R {
        PLLRDYIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - HSE ready interrupt flag
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - HSI16 ready interrupt flag
    #[inline(always)]
    pub fn hsi16rdyie(&self) -> HSI16RDYIE_R {
        HSI16RDYIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - LSE ready interrupt flag
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - LSI ready interrupt flag
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new((self.bits & 0x01) != 0)
    }
}
///Clock interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cier](index.html) module
pub struct CIER_SPEC;
impl crate::RegisterSpec for CIER_SPEC {
    type Ux = u32;
}
///`read()` method returns [cier::R](R) reader structure
impl crate::Readable for CIER_SPEC {
    type Reader = R;
}
///`reset()` method sets CIER to value 0
impl crate::Resettable for CIER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
