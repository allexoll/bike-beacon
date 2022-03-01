///Register `OPTKEYR` writer
pub struct W(crate::W<OPTKEYR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPTKEYR_SPEC>;
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
impl From<crate::W<OPTKEYR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPTKEYR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OPTKEYR` writer - Option byte key
pub struct OPTKEYR_W<'a> {
    w: &'a mut W,
}
impl<'a> OPTKEYR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl W {
    ///Bits 0:31 - Option byte key
    #[inline(always)]
    pub fn optkeyr(&mut self) -> OPTKEYR_W {
        OPTKEYR_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub fn bits(&mut self, bits: u32) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
///Option byte key register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [optkeyr](index.html) module
pub struct OPTKEYR_SPEC;
impl crate::RegisterSpec for OPTKEYR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [optkeyr::W](W) writer structure
impl crate::Writable for OPTKEYR_SPEC {
    type Writer = W;
}
///`reset()` method sets OPTKEYR to value 0
impl crate::Resettable for OPTKEYR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
