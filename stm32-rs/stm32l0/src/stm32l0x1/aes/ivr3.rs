///Register `IVR3` reader
pub struct R(crate::R<IVR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IVR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IVR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IVR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IVR3` writer
pub struct W(crate::W<IVR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IVR3_SPEC>;
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
impl From<crate::W<IVR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IVR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `IV3` reader - Initialization Vector Register (MSB IVR \[127:96\])
pub struct IV3_R(crate::FieldReader<u32, u32>);
impl IV3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        IV3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IV3_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `IV3` writer - Initialization Vector Register (MSB IVR \[127:96\])
pub struct IV3_W<'a> {
    w: &'a mut W,
}
impl<'a> IV3_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    ///Bits 0:31 - Initialization Vector Register (MSB IVR \[127:96\])
    #[inline(always)]
    pub fn iv3(&self) -> IV3_R {
        IV3_R::new(self.bits as u32)
    }
}
impl W {
    ///Bits 0:31 - Initialization Vector Register (MSB IVR \[127:96\])
    #[inline(always)]
    pub fn iv3(&mut self) -> IV3_W {
        IV3_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub fn bits(&mut self, bits: u32) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
///initialization vector register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ivr3](index.html) module
pub struct IVR3_SPEC;
impl crate::RegisterSpec for IVR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [ivr3::R](R) reader structure
impl crate::Readable for IVR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ivr3::W](W) writer structure
impl crate::Writable for IVR3_SPEC {
    type Writer = W;
}
///`reset()` method sets IVR3 to value 0
impl crate::Resettable for IVR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
