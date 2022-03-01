///Register `AHBRSTR` reader
pub struct R(crate::R<AHBRSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBRSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBRSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBRSTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AHBRSTR` writer
pub struct W(crate::W<AHBRSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBRSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<AHBRSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBRSTR_SPEC>) -> Self {
        W(writer)
    }
}
///Crypto module reset
pub type CRYPRST_A = DMARST_A;
///Field `CRYPRST` reader - Crypto module reset
pub type CRYPRST_R = DMARST_R;
///Field `CRYPRST` writer - Crypto module reset
pub struct CRYPRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPRST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CRYPRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Reset the module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CRYPRST_A::RESET)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
///Test integration module reset
pub type CRCRST_A = DMARST_A;
///Field `CRCRST` reader - Test integration module reset
pub type CRCRST_R = DMARST_R;
///Field `CRCRST` writer - Test integration module reset
pub struct CRCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCRST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CRCRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Reset the module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CRCRST_A::RESET)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
///Memory interface reset
pub type MIFRST_A = DMARST_A;
///Field `MIFRST` reader - Memory interface reset
pub type MIFRST_R = DMARST_R;
///Field `MIFRST` writer - Memory interface reset
pub struct MIFRST_W<'a> {
    w: &'a mut W,
}
impl<'a> MIFRST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MIFRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Reset the module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(MIFRST_A::RESET)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
///DMA reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMARST_A {
    ///1: Reset the module
    RESET = 1,
}
impl From<DMARST_A> for bool {
    #[inline(always)]
    fn from(variant: DMARST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DMARST` reader - DMA reset
pub struct DMARST_R(crate::FieldReader<bool, DMARST_A>);
impl DMARST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMARST_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<DMARST_A> {
        match self.bits {
            true => Some(DMARST_A::RESET),
            _ => None,
        }
    }
    ///Checks if the value of the field is `RESET`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == DMARST_A::RESET
    }
}
impl core::ops::Deref for DMARST_R {
    type Target = crate::FieldReader<bool, DMARST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DMARST` writer - DMA reset
pub struct DMARST_W<'a> {
    w: &'a mut W,
}
impl<'a> DMARST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DMARST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Reset the module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DMARST_A::RESET)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    ///Bit 24 - Crypto module reset
    #[inline(always)]
    pub fn cryprst(&self) -> CRYPRST_R {
        CRYPRST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 12 - Test integration module reset
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 8 - Memory interface reset
    #[inline(always)]
    pub fn mifrst(&self) -> MIFRST_R {
        MIFRST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 0 - DMA reset
    #[inline(always)]
    pub fn dmarst(&self) -> DMARST_R {
        DMARST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 24 - Crypto module reset
    #[inline(always)]
    pub fn cryprst(&mut self) -> CRYPRST_W {
        CRYPRST_W { w: self }
    }
    ///Bit 12 - Test integration module reset
    #[inline(always)]
    pub fn crcrst(&mut self) -> CRCRST_W {
        CRCRST_W { w: self }
    }
    ///Bit 8 - Memory interface reset
    #[inline(always)]
    pub fn mifrst(&mut self) -> MIFRST_W {
        MIFRST_W { w: self }
    }
    ///Bit 0 - DMA reset
    #[inline(always)]
    pub fn dmarst(&mut self) -> DMARST_W {
        DMARST_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AHB peripheral reset register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahbrstr](index.html) module
pub struct AHBRSTR_SPEC;
impl crate::RegisterSpec for AHBRSTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ahbrstr::R](R) reader structure
impl crate::Readable for AHBRSTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ahbrstr::W](W) writer structure
impl crate::Writable for AHBRSTR_SPEC {
    type Writer = W;
}
///`reset()` method sets AHBRSTR to value 0
impl crate::Resettable for AHBRSTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
