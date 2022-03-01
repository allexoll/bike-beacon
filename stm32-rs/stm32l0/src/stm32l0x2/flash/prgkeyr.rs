///Register `PRGKEYR` writer
pub struct W(crate::W<PRGKEYR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRGKEYR_SPEC>;
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
impl From<crate::W<PRGKEYR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRGKEYR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PRGKEYR` writer - Program memory key
pub struct PRGKEYR_W<'a> {
    w: &'a mut W,
}
impl<'a> PRGKEYR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl W {
    ///Bits 0:31 - Program memory key
    #[inline(always)]
    pub fn prgkeyr(&mut self) -> PRGKEYR_W {
        PRGKEYR_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub fn bits(&mut self, bits: u32) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
///Program memory key register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [prgkeyr](index.html) module
pub struct PRGKEYR_SPEC;
impl crate::RegisterSpec for PRGKEYR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [prgkeyr::W](W) writer structure
impl crate::Writable for PRGKEYR_SPEC {
    type Writer = W;
}
///`reset()` method sets PRGKEYR to value 0
impl crate::Resettable for PRGKEYR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
