///Register `AHBENR` reader
pub struct R(crate::R<AHBENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AHBENR` writer
pub struct W(crate::W<AHBENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBENR_SPEC>;
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
impl From<crate::W<AHBENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBENR_SPEC>) -> Self {
        W(writer)
    }
}
///Crypto clock enable bit
pub type CRYPEN_A = DMAEN_A;
///Field `CRYPEN` reader - Crypto clock enable bit
pub type CRYPEN_R = DMAEN_R;
///Field `CRYPEN` writer - Crypto clock enable bit
pub struct CRYPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CRYPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRYPEN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRYPEN_A::ENABLED)
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
///CRC clock enable bit
pub type CRCEN_A = DMAEN_A;
///Field `CRCEN` reader - CRC clock enable bit
pub type CRCEN_R = DMAEN_R;
///Field `CRCEN` writer - CRC clock enable bit
pub struct CRCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CRCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRCEN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRCEN_A::ENABLED)
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
///NVM interface clock enable bit
pub type MIFEN_A = DMAEN_A;
///Field `MIFEN` reader - NVM interface clock enable bit
pub type MIFEN_R = DMAEN_R;
///Field `MIFEN` writer - NVM interface clock enable bit
pub struct MIFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MIFEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MIFEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MIFEN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MIFEN_A::ENABLED)
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
///DMA clock enable bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN_A {
    ///0: Clock disabled
    DISABLED = 0,
    ///1: Clock enabled
    ENABLED = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAEN` reader - DMA clock enable bit
pub struct DMAEN_R(crate::FieldReader<bool, DMAEN_A>);
impl DMAEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMAEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::DISABLED,
            true => DMAEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DMAEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DMAEN_A::ENABLED
    }
}
impl core::ops::Deref for DMAEN_R {
    type Target = crate::FieldReader<bool, DMAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DMAEN` writer - DMA clock enable bit
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DMAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAEN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAEN_A::ENABLED)
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
    ///Bit 24 - Crypto clock enable bit
    #[inline(always)]
    pub fn crypen(&self) -> CRYPEN_R {
        CRYPEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 12 - CRC clock enable bit
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 8 - NVM interface clock enable bit
    #[inline(always)]
    pub fn mifen(&self) -> MIFEN_R {
        MIFEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 0 - DMA clock enable bit
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 24 - Crypto clock enable bit
    #[inline(always)]
    pub fn crypen(&mut self) -> CRYPEN_W {
        CRYPEN_W { w: self }
    }
    ///Bit 12 - CRC clock enable bit
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W {
        CRCEN_W { w: self }
    }
    ///Bit 8 - NVM interface clock enable bit
    #[inline(always)]
    pub fn mifen(&mut self) -> MIFEN_W {
        MIFEN_W { w: self }
    }
    ///Bit 0 - DMA clock enable bit
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AHB peripheral clock enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahbenr](index.html) module
pub struct AHBENR_SPEC;
impl crate::RegisterSpec for AHBENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ahbenr::R](R) reader structure
impl crate::Readable for AHBENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ahbenr::W](W) writer structure
impl crate::Writable for AHBENR_SPEC {
    type Writer = W;
}
///`reset()` method sets AHBENR to value 0x0100
impl crate::Resettable for AHBENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100
    }
}
