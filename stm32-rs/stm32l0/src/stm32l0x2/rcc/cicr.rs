///Register `CICR` reader
pub struct R(crate::R<CICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CICR_SPEC>) -> Self {
        R(reader)
    }
}
///Clock Security System Interrupt clear
pub type CSSHSEC_A = LSIRDYC_A;
///Field `CSSHSEC` reader - Clock Security System Interrupt clear
pub type CSSHSEC_R = LSIRDYC_R;
///LSE Clock Security System Interrupt clear
pub type CSSLSEC_A = LSIRDYC_A;
///Field `CSSLSEC` reader - LSE Clock Security System Interrupt clear
pub type CSSLSEC_R = LSIRDYC_R;
///HSI48 ready Interrupt clear
pub type HSI48RDYC_A = LSIRDYC_A;
///Field `HSI48RDYC` reader - HSI48 ready Interrupt clear
pub type HSI48RDYC_R = LSIRDYC_R;
///MSI ready Interrupt clear
pub type MSIRDYC_A = LSIRDYC_A;
///Field `MSIRDYC` reader - MSI ready Interrupt clear
pub type MSIRDYC_R = LSIRDYC_R;
///PLL ready Interrupt clear
pub type PLLRDYC_A = LSIRDYC_A;
///Field `PLLRDYC` reader - PLL ready Interrupt clear
pub type PLLRDYC_R = LSIRDYC_R;
///HSE ready Interrupt clear
pub type HSERDYC_A = LSIRDYC_A;
///Field `HSERDYC` reader - HSE ready Interrupt clear
pub type HSERDYC_R = LSIRDYC_R;
///HSI16 ready Interrupt clear
pub type HSI16RDYC_A = LSIRDYC_A;
///Field `HSI16RDYC` reader - HSI16 ready Interrupt clear
pub type HSI16RDYC_R = LSIRDYC_R;
///LSE ready Interrupt clear
pub type LSERDYC_A = LSIRDYC_A;
///Field `LSERDYC` reader - LSE ready Interrupt clear
pub type LSERDYC_R = LSIRDYC_R;
///LSI ready Interrupt clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSIRDYC_A {
    ///1: Clear interrupt flag
    CLEAR = 1,
}
impl From<LSIRDYC_A> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LSIRDYC` reader - LSI ready Interrupt clear
pub struct LSIRDYC_R(crate::FieldReader<bool, LSIRDYC_A>);
impl LSIRDYC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSIRDYC_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<LSIRDYC_A> {
        match self.bits {
            true => Some(LSIRDYC_A::CLEAR),
            _ => None,
        }
    }
    ///Checks if the value of the field is `CLEAR`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == LSIRDYC_A::CLEAR
    }
}
impl core::ops::Deref for LSIRDYC_R {
    type Target = crate::FieldReader<bool, LSIRDYC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bit 8 - Clock Security System Interrupt clear
    #[inline(always)]
    pub fn csshsec(&self) -> CSSHSEC_R {
        CSSHSEC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 7 - LSE Clock Security System Interrupt clear
    #[inline(always)]
    pub fn csslsec(&self) -> CSSLSEC_R {
        CSSLSEC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 6 - HSI48 ready Interrupt clear
    #[inline(always)]
    pub fn hsi48rdyc(&self) -> HSI48RDYC_R {
        HSI48RDYC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - MSI ready Interrupt clear
    #[inline(always)]
    pub fn msirdyc(&self) -> MSIRDYC_R {
        MSIRDYC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - PLL ready Interrupt clear
    #[inline(always)]
    pub fn pllrdyc(&self) -> PLLRDYC_R {
        PLLRDYC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - HSE ready Interrupt clear
    #[inline(always)]
    pub fn hserdyc(&self) -> HSERDYC_R {
        HSERDYC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - HSI16 ready Interrupt clear
    #[inline(always)]
    pub fn hsi16rdyc(&self) -> HSI16RDYC_R {
        HSI16RDYC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - LSE ready Interrupt clear
    #[inline(always)]
    pub fn lserdyc(&self) -> LSERDYC_R {
        LSERDYC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - LSI ready Interrupt clear
    #[inline(always)]
    pub fn lsirdyc(&self) -> LSIRDYC_R {
        LSIRDYC_R::new((self.bits & 0x01) != 0)
    }
}
///Clock interrupt clear register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cicr](index.html) module
pub struct CICR_SPEC;
impl crate::RegisterSpec for CICR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cicr::R](R) reader structure
impl crate::Readable for CICR_SPEC {
    type Reader = R;
}
///`reset()` method sets CICR to value 0
impl crate::Resettable for CICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}