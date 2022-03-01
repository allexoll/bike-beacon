///Register `IVR0` reader
pub struct R(crate::R<IVR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IVR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IVR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IVR0_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IVR0` writer
pub struct W(crate::W<IVR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IVR0_SPEC>;
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
impl From<crate::W<IVR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IVR0_SPEC>) -> Self {
        W(writer)
    }
}
///Field `IV0` reader - initialization vector register (LSB IVR \[31:0\])
pub struct IV0_R(crate::FieldReader<u32, u32>);
impl IV0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        IV0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `IV0` writer - initialization vector register (LSB IVR \[31:0\])
pub struct IV0_W<'a> {
    w: &'a mut W,
}
impl<'a> IV0_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    ///Bits 0:31 - initialization vector register (LSB IVR \[31:0\])
    #[inline(always)]
    pub fn iv0(&self) -> IV0_R {
        IV0_R::new(self.bits as u32)
    }
}
impl W {
    ///Bits 0:31 - initialization vector register (LSB IVR \[31:0\])
    #[inline(always)]
    pub fn iv0(&mut self) -> IV0_W {
        IV0_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub fn bits(&mut self, bits: u32) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
///initialization vector register 0
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ivr0](index.html) module
pub struct IVR0_SPEC;
impl crate::RegisterSpec for IVR0_SPEC {
    type Ux = u32;
}
///`read()` method returns [ivr0::R](R) reader structure
impl crate::Readable for IVR0_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ivr0::W](W) writer structure
impl crate::Writable for IVR0_SPEC {
    type Writer = W;
}
///`reset()` method sets IVR0 to value 0
impl crate::Resettable for IVR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
