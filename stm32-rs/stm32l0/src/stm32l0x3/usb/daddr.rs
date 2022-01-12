///Register `DADDR` reader
pub struct R(crate::R<DADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DADDR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DADDR` writer
pub struct W(crate::W<DADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DADDR_SPEC>;
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
impl From<crate::W<DADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DADDR_SPEC>) -> Self {
        W(writer)
    }
}
///EF
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EF_A {
    ///0: USB device disabled
    DISABLED = 0,
    ///1: USB device enabled
    ENABLED = 1,
}
impl From<EF_A> for bool {
    #[inline(always)]
    fn from(variant: EF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EF` reader - EF
pub struct EF_R(crate::FieldReader<bool, EF_A>);
impl EF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EF_A {
        match self.bits {
            false => EF_A::DISABLED,
            true => EF_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EF_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EF_A::ENABLED
    }
}
impl core::ops::Deref for EF_R {
    type Target = crate::FieldReader<bool, EF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EF` writer - EF
pub struct EF_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///USB device disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EF_A::DISABLED)
    }
    ///USB device enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EF_A::ENABLED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
///Field `ADD` reader - ADD
pub struct ADD_R(crate::FieldReader<u8, u8>);
impl ADD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ADD` writer - ADD
pub struct ADD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    ///Bit 7 - EF
    #[inline(always)]
    pub fn ef(&self) -> EF_R {
        EF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bits 0:6 - ADD
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    ///Bit 7 - EF
    #[inline(always)]
    pub fn ef(&mut self) -> EF_W {
        EF_W { w: self }
    }
    ///Bits 0:6 - ADD
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W {
        ADD_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///device address
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [daddr](index.html) module
pub struct DADDR_SPEC;
impl crate::RegisterSpec for DADDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [daddr::R](R) reader structure
impl crate::Readable for DADDR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [daddr::W](W) writer structure
impl crate::Writable for DADDR_SPEC {
    type Writer = W;
}
///`reset()` method sets DADDR to value 0
impl crate::Resettable for DADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
