///Register `CCIPR` reader
pub struct R(crate::R<CCIPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCIPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCIPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCIPR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCIPR` writer
pub struct W(crate::W<CCIPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCIPR_SPEC>;
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
impl From<crate::W<CCIPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCIPR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `HSI48MSEL` reader - 48 MHz HSI48 clock source selection bit
pub struct HSI48MSEL_R(crate::FieldReader<bool, bool>);
impl HSI48MSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HSI48MSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSI48MSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `HSI48MSEL` writer - 48 MHz HSI48 clock source selection bit
pub struct HSI48MSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSI48MSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
///I2C3 clock source selection bits
pub type I2C3SEL_A = I2C1SEL_A;
///Field `I2C3SEL` reader - I2C3 clock source selection bits
pub type I2C3SEL_R = I2C1SEL_R;
///Field `I2C3SEL` writer - I2C3 clock source selection bits
pub struct I2C3SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3SEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: I2C3SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///APB clock selected as peripheral clock
    #[inline(always)]
    pub fn apb(self) -> &'a mut W {
        self.variant(I2C3SEL_A::APB)
    }
    ///System clock selected as peripheral clock
    #[inline(always)]
    pub fn system(self) -> &'a mut W {
        self.variant(I2C3SEL_A::SYSTEM)
    }
    ///HSI16 clock selected as peripheral clock
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(I2C3SEL_A::HSI16)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
///Low Power Timer clock source selection bits
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPTIM1SEL_A {
    ///0: APB clock selected as Timer clock
    APB = 0,
    ///1: LSI clock selected as Timer clock
    LSI = 1,
    ///2: HSI16 clock selected as Timer clock
    HSI16 = 2,
    ///3: LSE clock selected as Timer clock
    LSE = 3,
}
impl From<LPTIM1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LPTIM1SEL_A) -> Self {
        variant as _
    }
}
///Field `LPTIM1SEL` reader - Low Power Timer clock source selection bits
pub struct LPTIM1SEL_R(crate::FieldReader<u8, LPTIM1SEL_A>);
impl LPTIM1SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LPTIM1SEL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LPTIM1SEL_A {
        match self.bits {
            0 => LPTIM1SEL_A::APB,
            1 => LPTIM1SEL_A::LSI,
            2 => LPTIM1SEL_A::HSI16,
            3 => LPTIM1SEL_A::LSE,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `APB`
    #[inline(always)]
    pub fn is_apb(&self) -> bool {
        **self == LPTIM1SEL_A::APB
    }
    ///Checks if the value of the field is `LSI`
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        **self == LPTIM1SEL_A::LSI
    }
    ///Checks if the value of the field is `HSI16`
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        **self == LPTIM1SEL_A::HSI16
    }
    ///Checks if the value of the field is `LSE`
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        **self == LPTIM1SEL_A::LSE
    }
}
impl core::ops::Deref for LPTIM1SEL_R {
    type Target = crate::FieldReader<u8, LPTIM1SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LPTIM1SEL` writer - Low Power Timer clock source selection bits
pub struct LPTIM1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM1SEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LPTIM1SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///APB clock selected as Timer clock
    #[inline(always)]
    pub fn apb(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::APB)
    }
    ///LSI clock selected as Timer clock
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::LSI)
    }
    ///HSI16 clock selected as Timer clock
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::HSI16)
    }
    ///LSE clock selected as Timer clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::LSE)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
///I2C1 clock source selection bits
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum I2C1SEL_A {
    ///0: APB clock selected as peripheral clock
    APB = 0,
    ///1: System clock selected as peripheral clock
    SYSTEM = 1,
    ///2: HSI16 clock selected as peripheral clock
    HSI16 = 2,
}
impl From<I2C1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: I2C1SEL_A) -> Self {
        variant as _
    }
}
///Field `I2C1SEL` reader - I2C1 clock source selection bits
pub struct I2C1SEL_R(crate::FieldReader<u8, I2C1SEL_A>);
impl I2C1SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        I2C1SEL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<I2C1SEL_A> {
        match self.bits {
            0 => Some(I2C1SEL_A::APB),
            1 => Some(I2C1SEL_A::SYSTEM),
            2 => Some(I2C1SEL_A::HSI16),
            _ => None,
        }
    }
    ///Checks if the value of the field is `APB`
    #[inline(always)]
    pub fn is_apb(&self) -> bool {
        **self == I2C1SEL_A::APB
    }
    ///Checks if the value of the field is `SYSTEM`
    #[inline(always)]
    pub fn is_system(&self) -> bool {
        **self == I2C1SEL_A::SYSTEM
    }
    ///Checks if the value of the field is `HSI16`
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        **self == I2C1SEL_A::HSI16
    }
}
impl core::ops::Deref for I2C1SEL_R {
    type Target = crate::FieldReader<u8, I2C1SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `I2C1SEL` writer - I2C1 clock source selection bits
pub struct I2C1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1SEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: I2C1SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///APB clock selected as peripheral clock
    #[inline(always)]
    pub fn apb(self) -> &'a mut W {
        self.variant(I2C1SEL_A::APB)
    }
    ///System clock selected as peripheral clock
    #[inline(always)]
    pub fn system(self) -> &'a mut W {
        self.variant(I2C1SEL_A::SYSTEM)
    }
    ///HSI16 clock selected as peripheral clock
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(I2C1SEL_A::HSI16)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
///LPUART1 clock source selection bits
pub type LPUART1SEL_A = USART1SEL_A;
///Field `LPUART1SEL` reader - LPUART1 clock source selection bits
pub type LPUART1SEL_R = USART1SEL_R;
///Field `LPUART1SEL` writer - LPUART1 clock source selection bits
pub struct LPUART1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1SEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LPUART1SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///APB clock selected as peripheral clock
    #[inline(always)]
    pub fn apb(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::APB)
    }
    ///System clock selected as peripheral clock
    #[inline(always)]
    pub fn system(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::SYSTEM)
    }
    ///HSI16 clock selected as peripheral clock
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::HSI16)
    }
    ///LSE clock selected as peripheral clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::LSE)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
///USART2 clock source selection bits
pub type USART2SEL_A = USART1SEL_A;
///Field `USART2SEL` reader - USART2 clock source selection bits
pub type USART2SEL_R = USART1SEL_R;
///Field `USART2SEL` writer - USART2 clock source selection bits
pub struct USART2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2SEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: USART2SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///APB clock selected as peripheral clock
    #[inline(always)]
    pub fn apb(self) -> &'a mut W {
        self.variant(USART2SEL_A::APB)
    }
    ///System clock selected as peripheral clock
    #[inline(always)]
    pub fn system(self) -> &'a mut W {
        self.variant(USART2SEL_A::SYSTEM)
    }
    ///HSI16 clock selected as peripheral clock
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(USART2SEL_A::HSI16)
    }
    ///LSE clock selected as peripheral clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(USART2SEL_A::LSE)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
///USART1 clock source selection bits
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USART1SEL_A {
    ///0: APB clock selected as peripheral clock
    APB = 0,
    ///1: System clock selected as peripheral clock
    SYSTEM = 1,
    ///2: HSI16 clock selected as peripheral clock
    HSI16 = 2,
    ///3: LSE clock selected as peripheral clock
    LSE = 3,
}
impl From<USART1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: USART1SEL_A) -> Self {
        variant as _
    }
}
///Field `USART1SEL` reader - USART1 clock source selection bits
pub struct USART1SEL_R(crate::FieldReader<u8, USART1SEL_A>);
impl USART1SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USART1SEL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> USART1SEL_A {
        match self.bits {
            0 => USART1SEL_A::APB,
            1 => USART1SEL_A::SYSTEM,
            2 => USART1SEL_A::HSI16,
            3 => USART1SEL_A::LSE,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `APB`
    #[inline(always)]
    pub fn is_apb(&self) -> bool {
        **self == USART1SEL_A::APB
    }
    ///Checks if the value of the field is `SYSTEM`
    #[inline(always)]
    pub fn is_system(&self) -> bool {
        **self == USART1SEL_A::SYSTEM
    }
    ///Checks if the value of the field is `HSI16`
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        **self == USART1SEL_A::HSI16
    }
    ///Checks if the value of the field is `LSE`
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        **self == USART1SEL_A::LSE
    }
}
impl core::ops::Deref for USART1SEL_R {
    type Target = crate::FieldReader<u8, USART1SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `USART1SEL` writer - USART1 clock source selection bits
pub struct USART1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1SEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: USART1SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///APB clock selected as peripheral clock
    #[inline(always)]
    pub fn apb(self) -> &'a mut W {
        self.variant(USART1SEL_A::APB)
    }
    ///System clock selected as peripheral clock
    #[inline(always)]
    pub fn system(self) -> &'a mut W {
        self.variant(USART1SEL_A::SYSTEM)
    }
    ///HSI16 clock selected as peripheral clock
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(USART1SEL_A::HSI16)
    }
    ///LSE clock selected as peripheral clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(USART1SEL_A::LSE)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    ///Bit 26 - 48 MHz HSI48 clock source selection bit
    #[inline(always)]
    pub fn hsi48msel(&self) -> HSI48MSEL_R {
        HSI48MSEL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bits 16:17 - I2C3 clock source selection bits
    #[inline(always)]
    pub fn i2c3sel(&self) -> I2C3SEL_R {
        I2C3SEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    ///Bits 18:19 - Low Power Timer clock source selection bits
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    ///Bits 12:13 - I2C1 clock source selection bits
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2C1SEL_R {
        I2C1SEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    ///Bits 10:11 - LPUART1 clock source selection bits
    #[inline(always)]
    pub fn lpuart1sel(&self) -> LPUART1SEL_R {
        LPUART1SEL_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    ///Bits 2:3 - USART2 clock source selection bits
    #[inline(always)]
    pub fn usart2sel(&self) -> USART2SEL_R {
        USART2SEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    ///Bits 0:1 - USART1 clock source selection bits
    #[inline(always)]
    pub fn usart1sel(&self) -> USART1SEL_R {
        USART1SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    ///Bit 26 - 48 MHz HSI48 clock source selection bit
    #[inline(always)]
    pub fn hsi48msel(&mut self) -> HSI48MSEL_W {
        HSI48MSEL_W { w: self }
    }
    ///Bits 16:17 - I2C3 clock source selection bits
    #[inline(always)]
    pub fn i2c3sel(&mut self) -> I2C3SEL_W {
        I2C3SEL_W { w: self }
    }
    ///Bits 18:19 - Low Power Timer clock source selection bits
    #[inline(always)]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W {
        LPTIM1SEL_W { w: self }
    }
    ///Bits 12:13 - I2C1 clock source selection bits
    #[inline(always)]
    pub fn i2c1sel(&mut self) -> I2C1SEL_W {
        I2C1SEL_W { w: self }
    }
    ///Bits 10:11 - LPUART1 clock source selection bits
    #[inline(always)]
    pub fn lpuart1sel(&mut self) -> LPUART1SEL_W {
        LPUART1SEL_W { w: self }
    }
    ///Bits 2:3 - USART2 clock source selection bits
    #[inline(always)]
    pub fn usart2sel(&mut self) -> USART2SEL_W {
        USART2SEL_W { w: self }
    }
    ///Bits 0:1 - USART1 clock source selection bits
    #[inline(always)]
    pub fn usart1sel(&mut self) -> USART1SEL_W {
        USART1SEL_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Clock configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccipr](index.html) module
pub struct CCIPR_SPEC;
impl crate::RegisterSpec for CCIPR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccipr::R](R) reader structure
impl crate::Readable for CCIPR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccipr::W](W) writer structure
impl crate::Writable for CCIPR_SPEC {
    type Writer = W;
}
///`reset()` method sets CCIPR to value 0
impl crate::Resettable for CCIPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}