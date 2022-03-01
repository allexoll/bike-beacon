///Register `GTPR` reader
pub struct R(crate::R<GTPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTPR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `GTPR` writer
pub struct W(crate::W<GTPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTPR_SPEC>;
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
impl From<crate::W<GTPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTPR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `GT` reader - Guard time value
pub struct GT_R(crate::FieldReader<u8, u8>);
impl GT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `GT` writer - Guard time value
pub struct GT_W<'a> {
    w: &'a mut W,
}
impl<'a> GT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
///Field `PSC` reader - Prescaler value
pub struct PSC_R(crate::FieldReader<u8, u8>);
impl PSC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PSC` writer - Prescaler value
pub struct PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> PSC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    ///Bits 8:15 - Guard time value
    #[inline(always)]
    pub fn gt(&self) -> GT_R {
        GT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 0:7 - Prescaler value
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 8:15 - Guard time value
    #[inline(always)]
    pub fn gt(&mut self) -> GT_W {
        GT_W { w: self }
    }
    ///Bits 0:7 - Prescaler value
    #[inline(always)]
    pub fn psc(&mut self) -> PSC_W {
        PSC_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Guard time and prescaler register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gtpr](index.html) module
pub struct GTPR_SPEC;
impl crate::RegisterSpec for GTPR_SPEC {
    type Ux = u32;
}
///`read()` method returns [gtpr::R](R) reader structure
impl crate::Readable for GTPR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [gtpr::W](W) writer structure
impl crate::Writable for GTPR_SPEC {
    type Writer = W;
}
///`reset()` method sets GTPR to value 0
impl crate::Resettable for GTPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
