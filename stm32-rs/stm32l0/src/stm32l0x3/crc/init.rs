///Register `INIT` reader
pub struct R(crate::R<INIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INIT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `INIT` writer
pub struct W(crate::W<INIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INIT_SPEC>;
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
impl From<crate::W<INIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INIT_SPEC>) -> Self {
        W(writer)
    }
}
///Field `INIT` reader - Programmable initial CRC value
pub struct INIT_R(crate::FieldReader<u32, u32>);
impl INIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        INIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INIT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `INIT` writer - Programmable initial CRC value
pub struct INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    ///Bits 0:31 - Programmable initial CRC value
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(self.bits as u32)
    }
}
impl W {
    ///Bits 0:31 - Programmable initial CRC value
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W {
        INIT_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub fn bits(&mut self, bits: u32) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
///Initial CRC value
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [init](index.html) module
pub struct INIT_SPEC;
impl crate::RegisterSpec for INIT_SPEC {
    type Ux = u32;
}
///`read()` method returns [init::R](R) reader structure
impl crate::Readable for INIT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [init::W](W) writer structure
impl crate::Writable for INIT_SPEC {
    type Writer = W;
}
///`reset()` method sets INIT to value 0xffff_ffff
impl crate::Resettable for INIT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
