///Register `ICR` writer
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
///Direction change to down Clear Flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOWNCF_AW {
    ///1: Direction change to down Clear Flag
    CLEAR = 1,
}
impl From<DOWNCF_AW> for bool {
    #[inline(always)]
    fn from(variant: DOWNCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `DOWNCF` writer - Direction change to down Clear Flag
pub struct DOWNCF_W<'a> {
    w: &'a mut W,
}
impl<'a> DOWNCF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DOWNCF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Direction change to down Clear Flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DOWNCF_AW::CLEAR)
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
///Direction change to UP Clear Flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPCF_AW {
    ///1: Direction change to up Clear Flag
    CLEAR = 1,
}
impl From<UPCF_AW> for bool {
    #[inline(always)]
    fn from(variant: UPCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `UPCF` writer - Direction change to UP Clear Flag
pub struct UPCF_W<'a> {
    w: &'a mut W,
}
impl<'a> UPCF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: UPCF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Direction change to up Clear Flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(UPCF_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
///Autoreload register update OK Clear Flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARROKCF_AW {
    ///1: Autoreload register update OK Clear Flag
    CLEAR = 1,
}
impl From<ARROKCF_AW> for bool {
    #[inline(always)]
    fn from(variant: ARROKCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `ARROKCF` writer - Autoreload register update OK Clear Flag
pub struct ARROKCF_W<'a> {
    w: &'a mut W,
}
impl<'a> ARROKCF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ARROKCF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Autoreload register update OK Clear Flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ARROKCF_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
///Compare register update OK Clear Flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPOKCF_AW {
    ///1: Compare register update OK Clear Flag
    CLEAR = 1,
}
impl From<CMPOKCF_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPOKCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPOKCF` writer - Compare register update OK Clear Flag
pub struct CMPOKCF_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPOKCF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CMPOKCF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Compare register update OK Clear Flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CMPOKCF_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
///External trigger valid edge Clear Flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTTRIGCF_AW {
    ///1: External trigger valid edge Clear Flag
    CLEAR = 1,
}
impl From<EXTTRIGCF_AW> for bool {
    #[inline(always)]
    fn from(variant: EXTTRIGCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `EXTTRIGCF` writer - External trigger valid edge Clear Flag
pub struct EXTTRIGCF_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTTRIGCF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EXTTRIGCF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///External trigger valid edge Clear Flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EXTTRIGCF_AW::CLEAR)
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
///Autoreload match Clear Flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARRMCF_AW {
    ///1: Autoreload match Clear Flag
    CLEAR = 1,
}
impl From<ARRMCF_AW> for bool {
    #[inline(always)]
    fn from(variant: ARRMCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `ARRMCF` writer - Autoreload match Clear Flag
pub struct ARRMCF_W<'a> {
    w: &'a mut W,
}
impl<'a> ARRMCF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ARRMCF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Autoreload match Clear Flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ARRMCF_AW::CLEAR)
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
///compare match Clear Flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPMCF_AW {
    ///1: Compare match Clear Flag
    CLEAR = 1,
}
impl From<CMPMCF_AW> for bool {
    #[inline(always)]
    fn from(variant: CMPMCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPMCF` writer - compare match Clear Flag
pub struct CMPMCF_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPMCF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CMPMCF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Compare match Clear Flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CMPMCF_AW::CLEAR)
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
impl W {
    ///Bit 6 - Direction change to down Clear Flag
    #[inline(always)]
    pub fn downcf(&mut self) -> DOWNCF_W {
        DOWNCF_W { w: self }
    }
    ///Bit 5 - Direction change to UP Clear Flag
    #[inline(always)]
    pub fn upcf(&mut self) -> UPCF_W {
        UPCF_W { w: self }
    }
    ///Bit 4 - Autoreload register update OK Clear Flag
    #[inline(always)]
    pub fn arrokcf(&mut self) -> ARROKCF_W {
        ARROKCF_W { w: self }
    }
    ///Bit 3 - Compare register update OK Clear Flag
    #[inline(always)]
    pub fn cmpokcf(&mut self) -> CMPOKCF_W {
        CMPOKCF_W { w: self }
    }
    ///Bit 2 - External trigger valid edge Clear Flag
    #[inline(always)]
    pub fn exttrigcf(&mut self) -> EXTTRIGCF_W {
        EXTTRIGCF_W { w: self }
    }
    ///Bit 1 - Autoreload match Clear Flag
    #[inline(always)]
    pub fn arrmcf(&mut self) -> ARRMCF_W {
        ARRMCF_W { w: self }
    }
    ///Bit 0 - compare match Clear Flag
    #[inline(always)]
    pub fn cmpmcf(&mut self) -> CMPMCF_W {
        CMPMCF_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Interrupt Clear Register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [icr](index.html) module
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [icr::W](W) writer structure
impl crate::Writable for ICR_SPEC {
    type Writer = W;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
