///Register `CFGR2` reader
pub struct R(crate::R<CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CFGR2` writer
pub struct W(crate::W<CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR2_SPEC>;
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
impl From<crate::W<CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
///Oversampler Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVSE_A {
    ///0: Oversampler disabled
    DISABLED = 0,
    ///1: Oversampler enabled
    ENABLED = 1,
}
impl From<OVSE_A> for bool {
    #[inline(always)]
    fn from(variant: OVSE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OVSE` reader - Oversampler Enable
pub struct OVSE_R(crate::FieldReader<bool, OVSE_A>);
impl OVSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVSE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OVSE_A {
        match self.bits {
            false => OVSE_A::DISABLED,
            true => OVSE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == OVSE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == OVSE_A::ENABLED
    }
}
impl core::ops::Deref for OVSE_R {
    type Target = crate::FieldReader<bool, OVSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `OVSE` writer - Oversampler Enable
pub struct OVSE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVSE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OVSE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Oversampler disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OVSE_A::DISABLED)
    }
    ///Oversampler enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OVSE_A::ENABLED)
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
///Oversampling ratio
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OVSR_A {
    ///0: 2x
    MUL2 = 0,
    ///1: 4x
    MUL4 = 1,
    ///2: 8x
    MUL8 = 2,
    ///3: 16x
    MUL16 = 3,
    ///4: 32x
    MUL32 = 4,
    ///5: 64x
    MUL64 = 5,
    ///6: 128x
    MUL128 = 6,
    ///7: 256x
    MUL256 = 7,
}
impl From<OVSR_A> for u8 {
    #[inline(always)]
    fn from(variant: OVSR_A) -> Self {
        variant as _
    }
}
///Field `OVSR` reader - Oversampling ratio
pub struct OVSR_R(crate::FieldReader<u8, OVSR_A>);
impl OVSR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OVSR_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OVSR_A {
        match self.bits {
            0 => OVSR_A::MUL2,
            1 => OVSR_A::MUL4,
            2 => OVSR_A::MUL8,
            3 => OVSR_A::MUL16,
            4 => OVSR_A::MUL32,
            5 => OVSR_A::MUL64,
            6 => OVSR_A::MUL128,
            7 => OVSR_A::MUL256,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `MUL2`
    #[inline(always)]
    pub fn is_mul2(&self) -> bool {
        **self == OVSR_A::MUL2
    }
    ///Checks if the value of the field is `MUL4`
    #[inline(always)]
    pub fn is_mul4(&self) -> bool {
        **self == OVSR_A::MUL4
    }
    ///Checks if the value of the field is `MUL8`
    #[inline(always)]
    pub fn is_mul8(&self) -> bool {
        **self == OVSR_A::MUL8
    }
    ///Checks if the value of the field is `MUL16`
    #[inline(always)]
    pub fn is_mul16(&self) -> bool {
        **self == OVSR_A::MUL16
    }
    ///Checks if the value of the field is `MUL32`
    #[inline(always)]
    pub fn is_mul32(&self) -> bool {
        **self == OVSR_A::MUL32
    }
    ///Checks if the value of the field is `MUL64`
    #[inline(always)]
    pub fn is_mul64(&self) -> bool {
        **self == OVSR_A::MUL64
    }
    ///Checks if the value of the field is `MUL128`
    #[inline(always)]
    pub fn is_mul128(&self) -> bool {
        **self == OVSR_A::MUL128
    }
    ///Checks if the value of the field is `MUL256`
    #[inline(always)]
    pub fn is_mul256(&self) -> bool {
        **self == OVSR_A::MUL256
    }
}
impl core::ops::Deref for OVSR_R {
    type Target = crate::FieldReader<u8, OVSR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `OVSR` writer - Oversampling ratio
pub struct OVSR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVSR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OVSR_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///2x
    #[inline(always)]
    pub fn mul2(self) -> &'a mut W {
        self.variant(OVSR_A::MUL2)
    }
    ///4x
    #[inline(always)]
    pub fn mul4(self) -> &'a mut W {
        self.variant(OVSR_A::MUL4)
    }
    ///8x
    #[inline(always)]
    pub fn mul8(self) -> &'a mut W {
        self.variant(OVSR_A::MUL8)
    }
    ///16x
    #[inline(always)]
    pub fn mul16(self) -> &'a mut W {
        self.variant(OVSR_A::MUL16)
    }
    ///32x
    #[inline(always)]
    pub fn mul32(self) -> &'a mut W {
        self.variant(OVSR_A::MUL32)
    }
    ///64x
    #[inline(always)]
    pub fn mul64(self) -> &'a mut W {
        self.variant(OVSR_A::MUL64)
    }
    ///128x
    #[inline(always)]
    pub fn mul128(self) -> &'a mut W {
        self.variant(OVSR_A::MUL128)
    }
    ///256x
    #[inline(always)]
    pub fn mul256(self) -> &'a mut W {
        self.variant(OVSR_A::MUL256)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | ((value as u32 & 0x07) << 2);
        self.w
    }
}
///Field `OVSS` reader - Oversampling shift
pub struct OVSS_R(crate::FieldReader<u8, u8>);
impl OVSS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OVSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVSS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `OVSS` writer - Oversampling shift
pub struct OVSS_W<'a> {
    w: &'a mut W,
}
impl<'a> OVSS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 5)) | ((value as u32 & 0x0f) << 5);
        self.w
    }
}
///Triggered Oversampling
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOVS_A {
    ///0: All oversampled conversions for a channel are done consecutively after a trigger
    TRIGGERALL = 0,
    ///1: Each oversampled conversion for a channel needs a trigger
    TRIGGEREACH = 1,
}
impl From<TOVS_A> for bool {
    #[inline(always)]
    fn from(variant: TOVS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TOVS` reader - Triggered Oversampling
pub struct TOVS_R(crate::FieldReader<bool, TOVS_A>);
impl TOVS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOVS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TOVS_A {
        match self.bits {
            false => TOVS_A::TRIGGERALL,
            true => TOVS_A::TRIGGEREACH,
        }
    }
    ///Checks if the value of the field is `TRIGGERALL`
    #[inline(always)]
    pub fn is_trigger_all(&self) -> bool {
        **self == TOVS_A::TRIGGERALL
    }
    ///Checks if the value of the field is `TRIGGEREACH`
    #[inline(always)]
    pub fn is_trigger_each(&self) -> bool {
        **self == TOVS_A::TRIGGEREACH
    }
}
impl core::ops::Deref for TOVS_R {
    type Target = crate::FieldReader<bool, TOVS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TOVS` writer - Triggered Oversampling
pub struct TOVS_W<'a> {
    w: &'a mut W,
}
impl<'a> TOVS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TOVS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///All oversampled conversions for a channel are done consecutively after a trigger
    #[inline(always)]
    pub fn trigger_all(self) -> &'a mut W {
        self.variant(TOVS_A::TRIGGERALL)
    }
    ///Each oversampled conversion for a channel needs a trigger
    #[inline(always)]
    pub fn trigger_each(self) -> &'a mut W {
        self.variant(TOVS_A::TRIGGEREACH)
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
///ADC clock mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CKMODE_A {
    ///0: ADCCLK (Asynchronous clock mode)
    ADCLK = 0,
    ///1: PCLK/2 (Synchronous clock mode)
    PCLK_DIV2 = 1,
    ///2: PCLK/4 (Synchronous clock mode)
    PCLK_DIV4 = 2,
    ///3: PCLK (Synchronous clock mode)
    PCLK = 3,
}
impl From<CKMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CKMODE_A) -> Self {
        variant as _
    }
}
///Field `CKMODE` reader - ADC clock mode
pub struct CKMODE_R(crate::FieldReader<u8, CKMODE_A>);
impl CKMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CKMODE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CKMODE_A {
        match self.bits {
            0 => CKMODE_A::ADCLK,
            1 => CKMODE_A::PCLK_DIV2,
            2 => CKMODE_A::PCLK_DIV4,
            3 => CKMODE_A::PCLK,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `ADCLK`
    #[inline(always)]
    pub fn is_adclk(&self) -> bool {
        **self == CKMODE_A::ADCLK
    }
    ///Checks if the value of the field is `PCLK_DIV2`
    #[inline(always)]
    pub fn is_pclk_div2(&self) -> bool {
        **self == CKMODE_A::PCLK_DIV2
    }
    ///Checks if the value of the field is `PCLK_DIV4`
    #[inline(always)]
    pub fn is_pclk_div4(&self) -> bool {
        **self == CKMODE_A::PCLK_DIV4
    }
    ///Checks if the value of the field is `PCLK`
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        **self == CKMODE_A::PCLK
    }
}
impl core::ops::Deref for CKMODE_R {
    type Target = crate::FieldReader<u8, CKMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CKMODE` writer - ADC clock mode
pub struct CKMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CKMODE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CKMODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///ADCCLK (Asynchronous clock mode)
    #[inline(always)]
    pub fn adclk(self) -> &'a mut W {
        self.variant(CKMODE_A::ADCLK)
    }
    ///PCLK/2 (Synchronous clock mode)
    #[inline(always)]
    pub fn pclk_div2(self) -> &'a mut W {
        self.variant(CKMODE_A::PCLK_DIV2)
    }
    ///PCLK/4 (Synchronous clock mode)
    #[inline(always)]
    pub fn pclk_div4(self) -> &'a mut W {
        self.variant(CKMODE_A::PCLK_DIV4)
    }
    ///PCLK (Synchronous clock mode)
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(CKMODE_A::PCLK)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    ///Bit 0 - Oversampler Enable
    #[inline(always)]
    pub fn ovse(&self) -> OVSE_R {
        OVSE_R::new((self.bits & 0x01) != 0)
    }
    ///Bits 2:4 - Oversampling ratio
    #[inline(always)]
    pub fn ovsr(&self) -> OVSR_R {
        OVSR_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    ///Bits 5:8 - Oversampling shift
    #[inline(always)]
    pub fn ovss(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    ///Bit 9 - Triggered Oversampling
    #[inline(always)]
    pub fn tovs(&self) -> TOVS_R {
        TOVS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bits 30:31 - ADC clock mode
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    ///Bit 0 - Oversampler Enable
    #[inline(always)]
    pub fn ovse(&mut self) -> OVSE_W {
        OVSE_W { w: self }
    }
    ///Bits 2:4 - Oversampling ratio
    #[inline(always)]
    pub fn ovsr(&mut self) -> OVSR_W {
        OVSR_W { w: self }
    }
    ///Bits 5:8 - Oversampling shift
    #[inline(always)]
    pub fn ovss(&mut self) -> OVSS_W {
        OVSS_W { w: self }
    }
    ///Bit 9 - Triggered Oversampling
    #[inline(always)]
    pub fn tovs(&mut self) -> TOVS_W {
        TOVS_W { w: self }
    }
    ///Bits 30:31 - ADC clock mode
    #[inline(always)]
    pub fn ckmode(&mut self) -> CKMODE_W {
        CKMODE_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///configuration register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfgr2](index.html) module
pub struct CFGR2_SPEC;
impl crate::RegisterSpec for CFGR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [cfgr2::R](R) reader structure
impl crate::Readable for CFGR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cfgr2::W](W) writer structure
impl crate::Writable for CFGR2_SPEC {
    type Writer = W;
}
///`reset()` method sets CFGR2 to value 0
impl crate::Resettable for CFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
