///Register `IER` reader
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IER` writer
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MCEIE` reader - Max count error interrupt enable
pub struct MCEIE_R(crate::FieldReader<bool, bool>);
impl MCEIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MCEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MCEIE` writer - Max count error interrupt enable
pub struct MCEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCEIE_W<'a> {
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
///Field `EOAIE` reader - End of acquisition interrupt enable
pub struct EOAIE_R(crate::FieldReader<bool, bool>);
impl EOAIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EOAIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOAIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EOAIE` writer - End of acquisition interrupt enable
pub struct EOAIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EOAIE_W<'a> {
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
    ///Bit 1 - Max count error interrupt enable
    #[inline(always)]
    pub fn mceie(&self) -> MCEIE_R {
        MCEIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - End of acquisition interrupt enable
    #[inline(always)]
    pub fn eoaie(&self) -> EOAIE_R {
        EOAIE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 1 - Max count error interrupt enable
    #[inline(always)]
    pub fn mceie(&mut self) -> MCEIE_W {
        MCEIE_W { w: self }
    }
    ///Bit 0 - End of acquisition interrupt enable
    #[inline(always)]
    pub fn eoaie(&mut self) -> EOAIE_W {
        EOAIE_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ier](index.html) module
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
///`read()` method returns [ier::R](R) reader structure
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ier::W](W) writer structure
impl crate::Writable for IER_SPEC {
    type Writer = W;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
