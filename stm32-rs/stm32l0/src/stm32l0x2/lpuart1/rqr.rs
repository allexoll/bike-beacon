///Register `RQR` writer
pub struct W(crate::W<RQR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RQR_SPEC>;
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
impl From<crate::W<RQR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RQR_SPEC>) -> Self {
        W(writer)
    }
}
///Receive data flush request
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFRQ_AW {
    ///1: clears the RXNE flag. This allows to discard the received data without reading it, and avoid an overrun condition
    DISCARD = 1,
}
impl From<RXFRQ_AW> for bool {
    #[inline(always)]
    fn from(variant: RXFRQ_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `RXFRQ` writer - Receive data flush request
pub struct RXFRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFRQ_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RXFRQ_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///clears the RXNE flag. This allows to discard the received data without reading it, and avoid an overrun condition
    #[inline(always)]
    pub fn discard(self) -> &'a mut W {
        self.variant(RXFRQ_AW::DISCARD)
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
///Mute mode request
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMRQ_AW {
    ///1: Puts the USART in mute mode and sets the RWU flag
    MUTE = 1,
}
impl From<MMRQ_AW> for bool {
    #[inline(always)]
    fn from(variant: MMRQ_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `MMRQ` writer - Mute mode request
pub struct MMRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> MMRQ_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MMRQ_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Puts the USART in mute mode and sets the RWU flag
    #[inline(always)]
    pub fn mute(self) -> &'a mut W {
        self.variant(MMRQ_AW::MUTE)
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
///Send break request
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBKRQ_AW {
    ///1: sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available
    BREAK = 1,
}
impl From<SBKRQ_AW> for bool {
    #[inline(always)]
    fn from(variant: SBKRQ_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `SBKRQ` writer - Send break request
pub struct SBKRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SBKRQ_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SBKRQ_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available
    #[inline(always)]
    pub fn break_(self) -> &'a mut W {
        self.variant(SBKRQ_AW::BREAK)
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
impl W {
    ///Bit 3 - Receive data flush request
    #[inline(always)]
    pub fn rxfrq(&mut self) -> RXFRQ_W {
        RXFRQ_W { w: self }
    }
    ///Bit 2 - Mute mode request
    #[inline(always)]
    pub fn mmrq(&mut self) -> MMRQ_W {
        MMRQ_W { w: self }
    }
    ///Bit 1 - Send break request
    #[inline(always)]
    pub fn sbkrq(&mut self) -> SBKRQ_W {
        SBKRQ_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Request register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rqr](index.html) module
pub struct RQR_SPEC;
impl crate::RegisterSpec for RQR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [rqr::W](W) writer structure
impl crate::Writable for RQR_SPEC {
    type Writer = W;
}
///`reset()` method sets RQR to value 0
impl crate::Resettable for RQR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
