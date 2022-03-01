///Register `APB2SMENR` reader
pub struct R(crate::R<APB2SMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2SMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2SMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2SMENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `APB2SMENR` writer
pub struct W(crate::W<APB2SMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2SMENR_SPEC>;
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
impl From<crate::W<APB2SMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2SMENR_SPEC>) -> Self {
        W(writer)
    }
}
///DBG clock enable during sleep mode bit
pub type DBGSMEN_A = SYSCFGSMEN_A;
///Field `DBGSMEN` reader - DBG clock enable during sleep mode bit
pub type DBGSMEN_R = SYSCFGSMEN_R;
///Field `DBGSMEN` writer - DBG clock enable during sleep mode bit
pub struct DBGSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGSMEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DBGSMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DBGSMEN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DBGSMEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
///USART1 clock enable during sleep mode bit
pub type USART1SMEN_A = SYSCFGSMEN_A;
///Field `USART1SMEN` reader - USART1 clock enable during sleep mode bit
pub type USART1SMEN_R = SYSCFGSMEN_R;
///Field `USART1SMEN` writer - USART1 clock enable during sleep mode bit
pub struct USART1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1SMEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: USART1SMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USART1SMEN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USART1SMEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
///SPI1 clock enable during sleep mode bit
pub type SPI1SMEN_A = SYSCFGSMEN_A;
///Field `SPI1SMEN` reader - SPI1 clock enable during sleep mode bit
pub type SPI1SMEN_R = SYSCFGSMEN_R;
///Field `SPI1SMEN` writer - SPI1 clock enable during sleep mode bit
pub struct SPI1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1SMEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SPI1SMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SPI1SMEN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SPI1SMEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
///ADC clock enable during sleep mode bit
pub type ADCSMEN_A = SYSCFGSMEN_A;
///Field `ADCSMEN` reader - ADC clock enable during sleep mode bit
pub type ADCSMEN_R = SYSCFGSMEN_R;
///Field `ADCSMEN` writer - ADC clock enable during sleep mode bit
pub struct ADCSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCSMEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ADCSMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADCSMEN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADCSMEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
///TIM22 timer clock enable during sleep mode bit
pub type TIM22SMEN_A = SYSCFGSMEN_A;
///Field `TIM22SMEN` reader - TIM22 timer clock enable during sleep mode bit
pub type TIM22SMEN_R = SYSCFGSMEN_R;
///Field `TIM22SMEN` writer - TIM22 timer clock enable during sleep mode bit
pub struct TIM22SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM22SMEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TIM22SMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM22SMEN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM22SMEN_A::ENABLED)
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
///TIM21 timer clock enable during sleep mode bit
pub type TIM21SMEN_A = SYSCFGSMEN_A;
///Field `TIM21SMEN` reader - TIM21 timer clock enable during sleep mode bit
pub type TIM21SMEN_R = SYSCFGSMEN_R;
///Field `TIM21SMEN` writer - TIM21 timer clock enable during sleep mode bit
pub struct TIM21SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM21SMEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TIM21SMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM21SMEN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM21SMEN_A::ENABLED)
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
///System configuration controller clock enable during sleep mode bit
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCFGSMEN_A {
    ///0: Clock disabled
    DISABLED = 0,
    ///1: Clock enabled
    ENABLED = 1,
}
impl From<SYSCFGSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGSMEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SYSCFGSMEN` reader - System configuration controller clock enable during sleep mode bit
pub struct SYSCFGSMEN_R(crate::FieldReader<bool, SYSCFGSMEN_A>);
impl SYSCFGSMEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYSCFGSMEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SYSCFGSMEN_A {
        match self.bits {
            false => SYSCFGSMEN_A::DISABLED,
            true => SYSCFGSMEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SYSCFGSMEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SYSCFGSMEN_A::ENABLED
    }
}
impl core::ops::Deref for SYSCFGSMEN_R {
    type Target = crate::FieldReader<bool, SYSCFGSMEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SYSCFGSMEN` writer - System configuration controller clock enable during sleep mode bit
pub struct SYSCFGSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCFGSMEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SYSCFGSMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYSCFGSMEN_A::DISABLED)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SYSCFGSMEN_A::ENABLED)
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
    ///Bit 22 - DBG clock enable during sleep mode bit
    #[inline(always)]
    pub fn dbgsmen(&self) -> DBGSMEN_R {
        DBGSMEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 14 - USART1 clock enable during sleep mode bit
    #[inline(always)]
    pub fn usart1smen(&self) -> USART1SMEN_R {
        USART1SMEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 12 - SPI1 clock enable during sleep mode bit
    #[inline(always)]
    pub fn spi1smen(&self) -> SPI1SMEN_R {
        SPI1SMEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 9 - ADC clock enable during sleep mode bit
    #[inline(always)]
    pub fn adcsmen(&self) -> ADCSMEN_R {
        ADCSMEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 5 - TIM22 timer clock enable during sleep mode bit
    #[inline(always)]
    pub fn tim22smen(&self) -> TIM22SMEN_R {
        TIM22SMEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 2 - TIM21 timer clock enable during sleep mode bit
    #[inline(always)]
    pub fn tim21smen(&self) -> TIM21SMEN_R {
        TIM21SMEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 0 - System configuration controller clock enable during sleep mode bit
    #[inline(always)]
    pub fn syscfgsmen(&self) -> SYSCFGSMEN_R {
        SYSCFGSMEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 22 - DBG clock enable during sleep mode bit
    #[inline(always)]
    pub fn dbgsmen(&mut self) -> DBGSMEN_W {
        DBGSMEN_W { w: self }
    }
    ///Bit 14 - USART1 clock enable during sleep mode bit
    #[inline(always)]
    pub fn usart1smen(&mut self) -> USART1SMEN_W {
        USART1SMEN_W { w: self }
    }
    ///Bit 12 - SPI1 clock enable during sleep mode bit
    #[inline(always)]
    pub fn spi1smen(&mut self) -> SPI1SMEN_W {
        SPI1SMEN_W { w: self }
    }
    ///Bit 9 - ADC clock enable during sleep mode bit
    #[inline(always)]
    pub fn adcsmen(&mut self) -> ADCSMEN_W {
        ADCSMEN_W { w: self }
    }
    ///Bit 5 - TIM22 timer clock enable during sleep mode bit
    #[inline(always)]
    pub fn tim22smen(&mut self) -> TIM22SMEN_W {
        TIM22SMEN_W { w: self }
    }
    ///Bit 2 - TIM21 timer clock enable during sleep mode bit
    #[inline(always)]
    pub fn tim21smen(&mut self) -> TIM21SMEN_W {
        TIM21SMEN_W { w: self }
    }
    ///Bit 0 - System configuration controller clock enable during sleep mode bit
    #[inline(always)]
    pub fn syscfgsmen(&mut self) -> SYSCFGSMEN_W {
        SYSCFGSMEN_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///APB2 peripheral clock enable in sleep mode register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb2smenr](index.html) module
pub struct APB2SMENR_SPEC;
impl crate::RegisterSpec for APB2SMENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [apb2smenr::R](R) reader structure
impl crate::Readable for APB2SMENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [apb2smenr::W](W) writer structure
impl crate::Writable for APB2SMENR_SPEC {
    type Writer = W;
}
///`reset()` method sets APB2SMENR to value 0x0040_5225
impl crate::Resettable for APB2SMENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0040_5225
    }
}