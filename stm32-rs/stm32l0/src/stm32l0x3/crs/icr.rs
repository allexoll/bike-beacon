///Register `ICR` reader
pub struct R(crate::R<ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ICR` writer
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ESYNCC` reader - Expected SYNC clear flag
pub struct ESYNCC_R(crate::FieldReader<bool, bool>);
impl ESYNCC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ESYNCC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ESYNCC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ESYNCC` writer - Expected SYNC clear flag
pub struct ESYNCC_W<'a> {
    w: &'a mut W,
}
impl<'a> ESYNCC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
///Field `ERRC` reader - Error clear flag
pub struct ERRC_R(crate::FieldReader<bool, bool>);
impl ERRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ERRC` writer - Error clear flag
pub struct ERRC_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
///Field `SYNCWARNC` reader - SYNC warning clear flag
pub struct SYNCWARNC_R(crate::FieldReader<bool, bool>);
impl SYNCWARNC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYNCWARNC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNCWARNC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SYNCWARNC` writer - SYNC warning clear flag
pub struct SYNCWARNC_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCWARNC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
///Field `SYNCOKC` reader - SYNC event OK clear flag
pub struct SYNCOKC_R(crate::FieldReader<bool, bool>);
impl SYNCOKC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYNCOKC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNCOKC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SYNCOKC` writer - SYNC event OK clear flag
pub struct SYNCOKC_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCOKC_W<'a> {
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
    ///Bit 3 - Expected SYNC clear flag
    #[inline(always)]
    pub fn esyncc(&self) -> ESYNCC_R {
        ESYNCC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Error clear flag
    #[inline(always)]
    pub fn errc(&self) -> ERRC_R {
        ERRC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - SYNC warning clear flag
    #[inline(always)]
    pub fn syncwarnc(&self) -> SYNCWARNC_R {
        SYNCWARNC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - SYNC event OK clear flag
    #[inline(always)]
    pub fn syncokc(&self) -> SYNCOKC_R {
        SYNCOKC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 3 - Expected SYNC clear flag
    #[inline(always)]
    pub fn esyncc(&mut self) -> ESYNCC_W {
        ESYNCC_W { w: self }
    }
    ///Bit 2 - Error clear flag
    #[inline(always)]
    pub fn errc(&mut self) -> ERRC_W {
        ERRC_W { w: self }
    }
    ///Bit 1 - SYNC warning clear flag
    #[inline(always)]
    pub fn syncwarnc(&mut self) -> SYNCWARNC_W {
        SYNCWARNC_W { w: self }
    }
    ///Bit 0 - SYNC event OK clear flag
    #[inline(always)]
    pub fn syncokc(&mut self) -> SYNCOKC_W {
        SYNCOKC_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///interrupt flag clear register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [icr](index.html) module
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
///`read()` method returns [icr::R](R) reader structure
impl crate::Readable for ICR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [icr::W](W) writer structure
impl crate::Writable for ICR_SPEC {
    type Writer = W;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
