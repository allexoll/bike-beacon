///Register `SR` reader
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SR` writer
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SEIS` reader - Seed error interrupt status
pub struct SEIS_R(crate::FieldReader<bool, bool>);
impl SEIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SEIS` writer - Seed error interrupt status
pub struct SEIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SEIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
///Field `CEIS` reader - Clock error interrupt status
pub struct CEIS_R(crate::FieldReader<bool, bool>);
impl CEIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CEIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CEIS` writer - Clock error interrupt status
pub struct CEIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CEIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
///Field `SECS` reader - Seed error current status
pub struct SECS_R(crate::FieldReader<bool, bool>);
impl SECS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SECS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CECS` reader - Clock error current status
pub struct CECS_R(crate::FieldReader<bool, bool>);
impl CECS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CECS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CECS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DRDY` reader - Data ready
pub struct DRDY_R(crate::FieldReader<bool, bool>);
impl DRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bit 6 - Seed error interrupt status
    #[inline(always)]
    pub fn seis(&self) -> SEIS_R {
        SEIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - Clock error interrupt status
    #[inline(always)]
    pub fn ceis(&self) -> CEIS_R {
        CEIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 2 - Seed error current status
    #[inline(always)]
    pub fn secs(&self) -> SECS_R {
        SECS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Clock error current status
    #[inline(always)]
    pub fn cecs(&self) -> CECS_R {
        CECS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Data ready
    #[inline(always)]
    pub fn drdy(&self) -> DRDY_R {
        DRDY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 6 - Seed error interrupt status
    #[inline(always)]
    pub fn seis(&mut self) -> SEIS_W {
        SEIS_W { w: self }
    }
    ///Bit 5 - Clock error interrupt status
    #[inline(always)]
    pub fn ceis(&mut self) -> CEIS_W {
        CEIS_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr](index.html) module
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
///`read()` method returns [sr::R](R) reader structure
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [sr::W](W) writer structure
impl crate::Writable for SR_SPEC {
    type Writer = W;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}