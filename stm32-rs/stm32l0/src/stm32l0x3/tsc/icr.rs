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
///Field `MCEIC` reader - Max count error interrupt clear
pub struct MCEIC_R(crate::FieldReader<bool, bool>);
impl MCEIC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MCEIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCEIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MCEIC` writer - Max count error interrupt clear
pub struct MCEIC_W<'a> {
    w: &'a mut W,
}
impl<'a> MCEIC_W<'a> {
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
///Field `EOAIC` reader - End of acquisition interrupt clear
pub struct EOAIC_R(crate::FieldReader<bool, bool>);
impl EOAIC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EOAIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOAIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EOAIC` writer - End of acquisition interrupt clear
pub struct EOAIC_W<'a> {
    w: &'a mut W,
}
impl<'a> EOAIC_W<'a> {
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
    ///Bit 1 - Max count error interrupt clear
    #[inline(always)]
    pub fn mceic(&self) -> MCEIC_R {
        MCEIC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - End of acquisition interrupt clear
    #[inline(always)]
    pub fn eoaic(&self) -> EOAIC_R {
        EOAIC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 1 - Max count error interrupt clear
    #[inline(always)]
    pub fn mceic(&mut self) -> MCEIC_W {
        MCEIC_W { w: self }
    }
    ///Bit 0 - End of acquisition interrupt clear
    #[inline(always)]
    pub fn eoaic(&mut self) -> EOAIC_W {
        EOAIC_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///interrupt clear register
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
