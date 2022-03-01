///Register `POL` reader
pub struct R(crate::R<POL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `POL` writer
pub struct W(crate::W<POL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POL_SPEC>;
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
impl From<crate::W<POL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `POL` reader - Programmable polynomial
pub struct POL_R(crate::FieldReader<u32, u32>);
impl POL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `POL` writer - Programmable polynomial
pub struct POL_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    ///Bits 0:31 - Programmable polynomial
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(self.bits as u32)
    }
}
impl W {
    ///Bits 0:31 - Programmable polynomial
    #[inline(always)]
    pub fn pol(&mut self) -> POL_W {
        POL_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub fn bits(&mut self, bits: u32) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
///polynomial
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pol](index.html) module
pub struct POL_SPEC;
impl crate::RegisterSpec for POL_SPEC {
    type Ux = u32;
}
///`read()` method returns [pol::R](R) reader structure
impl crate::Readable for POL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pol::W](W) writer structure
impl crate::Writable for POL_SPEC {
    type Writer = W;
}
///`reset()` method sets POL to value 0x04c1_1db7
impl crate::Resettable for POL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04c1_1db7
    }
}
