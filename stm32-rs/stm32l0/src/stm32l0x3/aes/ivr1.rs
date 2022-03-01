///Register `IVR1` reader
pub struct R(crate::R<IVR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IVR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IVR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IVR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IVR1` writer
pub struct W(crate::W<IVR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IVR1_SPEC>;
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
impl From<crate::W<IVR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IVR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `IV1` reader - Initialization Vector Register (IVR \[63:32\])
pub struct IV1_R(crate::FieldReader<u32, u32>);
impl IV1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        IV1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `IV1` writer - Initialization Vector Register (IVR \[63:32\])
pub struct IV1_W<'a> {
    w: &'a mut W,
}
impl<'a> IV1_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    ///Bits 0:31 - Initialization Vector Register (IVR \[63:32\])
    #[inline(always)]
    pub fn iv1(&self) -> IV1_R {
        IV1_R::new(self.bits as u32)
    }
}
impl W {
    ///Bits 0:31 - Initialization Vector Register (IVR \[63:32\])
    #[inline(always)]
    pub fn iv1(&mut self) -> IV1_W {
        IV1_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub fn bits(&mut self, bits: u32) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
///initialization vector register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ivr1](index.html) module
pub struct IVR1_SPEC;
impl crate::RegisterSpec for IVR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [ivr1::R](R) reader structure
impl crate::Readable for IVR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ivr1::W](W) writer structure
impl crate::Writable for IVR1_SPEC {
    type Writer = W;
}
///`reset()` method sets IVR1 to value 0
impl crate::Resettable for IVR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
