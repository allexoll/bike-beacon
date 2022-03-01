///Register `DIER` reader
pub struct R(crate::R<DIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DIER` writer
pub struct W(crate::W<DIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIER_SPEC>;
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
impl From<crate::W<DIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIER_SPEC>) -> Self {
        W(writer)
    }
}
///Trigger interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIE_A {
    ///0: Trigger interrupt disabled
    DISABLED = 0,
    ///1: Trigger interrupt enabled
    ENABLED = 1,
}
impl From<TIE_A> for bool {
    #[inline(always)]
    fn from(variant: TIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TIE` reader - Trigger interrupt enable
pub struct TIE_R(crate::FieldReader<bool, TIE_A>);
impl TIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TIE_A {
        match self.bits {
            false => TIE_A::DISABLED,
            true => TIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TIE_A::ENABLED
    }
}
impl core::ops::Deref for TIE_R {
    type Target = crate::FieldReader<bool, TIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TIE` writer - Trigger interrupt enable
pub struct TIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Trigger interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIE_A::DISABLED)
    }
    ///Trigger interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
///Capture/Compare 2 interrupt enable
pub type CC2IE_A = CC1IE_A;
///Field `CC2IE` reader - Capture/Compare 2 interrupt enable
pub type CC2IE_R = CC1IE_R;
///Field `CC2IE` writer - Capture/Compare 2 interrupt enable
pub struct CC2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2IE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC2IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///CCx interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC2IE_A::DISABLED)
    }
    ///CCx interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC2IE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
///Capture/Compare 1 interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1IE_A {
    ///0: CCx interrupt disabled
    DISABLED = 0,
    ///1: CCx interrupt enabled
    ENABLED = 1,
}
impl From<CC1IE_A> for bool {
    #[inline(always)]
    fn from(variant: CC1IE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1IE` reader - Capture/Compare 1 interrupt enable
pub struct CC1IE_R(crate::FieldReader<bool, CC1IE_A>);
impl CC1IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CC1IE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CC1IE_A {
        match self.bits {
            false => CC1IE_A::DISABLED,
            true => CC1IE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CC1IE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CC1IE_A::ENABLED
    }
}
impl core::ops::Deref for CC1IE_R {
    type Target = crate::FieldReader<bool, CC1IE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CC1IE` writer - Capture/Compare 1 interrupt enable
pub struct CC1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1IE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC1IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///CCx interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC1IE_A::DISABLED)
    }
    ///CCx interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC1IE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
///Update interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UIE_A {
    ///0: Update interrupt disabled
    DISABLED = 0,
    ///1: Update interrupt enabled
    ENABLED = 1,
}
impl From<UIE_A> for bool {
    #[inline(always)]
    fn from(variant: UIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `UIE` reader - Update interrupt enable
pub struct UIE_R(crate::FieldReader<bool, UIE_A>);
impl UIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> UIE_A {
        match self.bits {
            false => UIE_A::DISABLED,
            true => UIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == UIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == UIE_A::ENABLED
    }
}
impl core::ops::Deref for UIE_R {
    type Target = crate::FieldReader<bool, UIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `UIE` writer - Update interrupt enable
pub struct UIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: UIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Update interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UIE_A::DISABLED)
    }
    ///Update interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    ///Bit 6 - Trigger interrupt enable
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 2 - Capture/Compare 2 interrupt enable
    #[inline(always)]
    pub fn cc2ie(&self) -> CC2IE_R {
        CC2IE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Capture/Compare 1 interrupt enable
    #[inline(always)]
    pub fn cc1ie(&self) -> CC1IE_R {
        CC1IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Update interrupt enable
    #[inline(always)]
    pub fn uie(&self) -> UIE_R {
        UIE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 6 - Trigger interrupt enable
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W {
        TIE_W { w: self }
    }
    ///Bit 2 - Capture/Compare 2 interrupt enable
    #[inline(always)]
    pub fn cc2ie(&mut self) -> CC2IE_W {
        CC2IE_W { w: self }
    }
    ///Bit 1 - Capture/Compare 1 interrupt enable
    #[inline(always)]
    pub fn cc1ie(&mut self) -> CC1IE_W {
        CC1IE_W { w: self }
    }
    ///Bit 0 - Update interrupt enable
    #[inline(always)]
    pub fn uie(&mut self) -> UIE_W {
        UIE_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DMA/Interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dier](index.html) module
pub struct DIER_SPEC;
impl crate::RegisterSpec for DIER_SPEC {
    type Ux = u32;
}
///`read()` method returns [dier::R](R) reader structure
impl crate::Readable for DIER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dier::W](W) writer structure
impl crate::Writable for DIER_SPEC {
    type Writer = W;
}
///`reset()` method sets DIER to value 0
impl crate::Resettable for DIER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
