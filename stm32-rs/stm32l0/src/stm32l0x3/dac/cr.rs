///Register `CR` reader
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR` writer
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
///DAC channel1 DMA Underrun Interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAUDRIE1_A {
    ///0: DAC channel X DMA Underrun Interrupt disabled
    DISABLED = 0,
    ///1: DAC channel X DMA Underrun Interrupt enabled
    ENABLED = 1,
}
impl From<DMAUDRIE1_A> for bool {
    #[inline(always)]
    fn from(variant: DMAUDRIE1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAUDRIE1` reader - DAC channel1 DMA Underrun Interrupt enable
pub struct DMAUDRIE1_R(crate::FieldReader<bool, DMAUDRIE1_A>);
impl DMAUDRIE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMAUDRIE1_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMAUDRIE1_A {
        match self.bits {
            false => DMAUDRIE1_A::DISABLED,
            true => DMAUDRIE1_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DMAUDRIE1_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DMAUDRIE1_A::ENABLED
    }
}
impl core::ops::Deref for DMAUDRIE1_R {
    type Target = crate::FieldReader<bool, DMAUDRIE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DMAUDRIE1` writer - DAC channel1 DMA Underrun Interrupt enable
pub struct DMAUDRIE1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAUDRIE1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DMAUDRIE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///DAC channel X DMA Underrun Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAUDRIE1_A::DISABLED)
    }
    ///DAC channel X DMA Underrun Interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAUDRIE1_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
///DAC channel1 DMA enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN1_A {
    ///0: DAC channel X DMA mode disabled
    DISABLED = 0,
    ///1: DAC channel X DMA mode enabled
    ENABLED = 1,
}
impl From<DMAEN1_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAEN1` reader - DAC channel1 DMA enable
pub struct DMAEN1_R(crate::FieldReader<bool, DMAEN1_A>);
impl DMAEN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMAEN1_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMAEN1_A {
        match self.bits {
            false => DMAEN1_A::DISABLED,
            true => DMAEN1_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DMAEN1_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DMAEN1_A::ENABLED
    }
}
impl core::ops::Deref for DMAEN1_R {
    type Target = crate::FieldReader<bool, DMAEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DMAEN1` writer - DAC channel1 DMA enable
pub struct DMAEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DMAEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///DAC channel X DMA mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAEN1_A::DISABLED)
    }
    ///DAC channel X DMA mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAEN1_A::ENABLED)
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
///Field `MAMP1` reader - DAC channel1 mask/amplitude selector
pub struct MAMP1_R(crate::FieldReader<u8, u8>);
impl MAMP1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MAMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAMP1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MAMP1` writer - DAC channel1 mask/amplitude selector
pub struct MAMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> MAMP1_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
///Field `WAVE1` reader - DAC channel1 noise/triangle wave generation enable
pub struct WAVE1_R(crate::FieldReader<u8, u8>);
impl WAVE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WAVE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAVE1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WAVE1` writer - DAC channel1 noise/triangle wave generation enable
pub struct WAVE1_W<'a> {
    w: &'a mut W,
}
impl<'a> WAVE1_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
///DAC channel1 trigger selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSEL1_A {
    ///0: Timer 6 TRGO event
    TIM6_TRGO = 0,
    ///1: Timer 3 TRGO event
    TIM3_TRGO = 1,
    ///2: Timer 7 TRGO event
    TIM7_TRGO = 2,
    ///3: Timer 15 TRGO event
    TIM15_TRGO = 3,
    ///4: Timer 2 TRGO event
    TIM2_TRGO = 4,
    ///6: EXTI line9
    EXTI9 = 6,
    ///7: Software trigger
    SOFTWARE = 7,
}
impl From<TSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: TSEL1_A) -> Self {
        variant as _
    }
}
///Field `TSEL1` reader - DAC channel1 trigger selection
pub struct TSEL1_R(crate::FieldReader<u8, TSEL1_A>);
impl TSEL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TSEL1_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<TSEL1_A> {
        match self.bits {
            0 => Some(TSEL1_A::TIM6_TRGO),
            1 => Some(TSEL1_A::TIM3_TRGO),
            2 => Some(TSEL1_A::TIM7_TRGO),
            3 => Some(TSEL1_A::TIM15_TRGO),
            4 => Some(TSEL1_A::TIM2_TRGO),
            6 => Some(TSEL1_A::EXTI9),
            7 => Some(TSEL1_A::SOFTWARE),
            _ => None,
        }
    }
    ///Checks if the value of the field is `TIM6_TRGO`
    #[inline(always)]
    pub fn is_tim6_trgo(&self) -> bool {
        **self == TSEL1_A::TIM6_TRGO
    }
    ///Checks if the value of the field is `TIM3_TRGO`
    #[inline(always)]
    pub fn is_tim3_trgo(&self) -> bool {
        **self == TSEL1_A::TIM3_TRGO
    }
    ///Checks if the value of the field is `TIM7_TRGO`
    #[inline(always)]
    pub fn is_tim7_trgo(&self) -> bool {
        **self == TSEL1_A::TIM7_TRGO
    }
    ///Checks if the value of the field is `TIM15_TRGO`
    #[inline(always)]
    pub fn is_tim15_trgo(&self) -> bool {
        **self == TSEL1_A::TIM15_TRGO
    }
    ///Checks if the value of the field is `TIM2_TRGO`
    #[inline(always)]
    pub fn is_tim2_trgo(&self) -> bool {
        **self == TSEL1_A::TIM2_TRGO
    }
    ///Checks if the value of the field is `EXTI9`
    #[inline(always)]
    pub fn is_exti9(&self) -> bool {
        **self == TSEL1_A::EXTI9
    }
    ///Checks if the value of the field is `SOFTWARE`
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        **self == TSEL1_A::SOFTWARE
    }
}
impl core::ops::Deref for TSEL1_R {
    type Target = crate::FieldReader<u8, TSEL1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TSEL1` writer - DAC channel1 trigger selection
pub struct TSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEL1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TSEL1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Timer 6 TRGO event
    #[inline(always)]
    pub fn tim6_trgo(self) -> &'a mut W {
        self.variant(TSEL1_A::TIM6_TRGO)
    }
    ///Timer 3 TRGO event
    #[inline(always)]
    pub fn tim3_trgo(self) -> &'a mut W {
        self.variant(TSEL1_A::TIM3_TRGO)
    }
    ///Timer 7 TRGO event
    #[inline(always)]
    pub fn tim7_trgo(self) -> &'a mut W {
        self.variant(TSEL1_A::TIM7_TRGO)
    }
    ///Timer 15 TRGO event
    #[inline(always)]
    pub fn tim15_trgo(self) -> &'a mut W {
        self.variant(TSEL1_A::TIM15_TRGO)
    }
    ///Timer 2 TRGO event
    #[inline(always)]
    pub fn tim2_trgo(self) -> &'a mut W {
        self.variant(TSEL1_A::TIM2_TRGO)
    }
    ///EXTI line9
    #[inline(always)]
    pub fn exti9(self) -> &'a mut W {
        self.variant(TSEL1_A::EXTI9)
    }
    ///Software trigger
    #[inline(always)]
    pub fn software(self) -> &'a mut W {
        self.variant(TSEL1_A::SOFTWARE)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
///DAC channel1 trigger enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEN1_A {
    ///0: DAC channel X trigger disabled
    DISABLED = 0,
    ///1: DAC channel X trigger enabled
    ENABLED = 1,
}
impl From<TEN1_A> for bool {
    #[inline(always)]
    fn from(variant: TEN1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TEN1` reader - DAC channel1 trigger enable
pub struct TEN1_R(crate::FieldReader<bool, TEN1_A>);
impl TEN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEN1_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TEN1_A {
        match self.bits {
            false => TEN1_A::DISABLED,
            true => TEN1_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TEN1_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TEN1_A::ENABLED
    }
}
impl core::ops::Deref for TEN1_R {
    type Target = crate::FieldReader<bool, TEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TEN1` writer - DAC channel1 trigger enable
pub struct TEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///DAC channel X trigger disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEN1_A::DISABLED)
    }
    ///DAC channel X trigger enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEN1_A::ENABLED)
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
///DAC channel1 output buffer disable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOFF1_A {
    ///0: DAC channel X output buffer enabled
    ENABLED = 0,
    ///1: DAC channel X output buffer disabled
    DISABLED = 1,
}
impl From<BOFF1_A> for bool {
    #[inline(always)]
    fn from(variant: BOFF1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BOFF1` reader - DAC channel1 output buffer disable
pub struct BOFF1_R(crate::FieldReader<bool, BOFF1_A>);
impl BOFF1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BOFF1_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BOFF1_A {
        match self.bits {
            false => BOFF1_A::ENABLED,
            true => BOFF1_A::DISABLED,
        }
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == BOFF1_A::ENABLED
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == BOFF1_A::DISABLED
    }
}
impl core::ops::Deref for BOFF1_R {
    type Target = crate::FieldReader<bool, BOFF1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BOFF1` writer - DAC channel1 output buffer disable
pub struct BOFF1_W<'a> {
    w: &'a mut W,
}
impl<'a> BOFF1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BOFF1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///DAC channel X output buffer enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BOFF1_A::ENABLED)
    }
    ///DAC channel X output buffer disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BOFF1_A::DISABLED)
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
///DAC channel1 enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN1_A {
    ///0: DAC channel X disabled
    DISABLED = 0,
    ///1: DAC channel X enabled
    ENABLED = 1,
}
impl From<EN1_A> for bool {
    #[inline(always)]
    fn from(variant: EN1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EN1` reader - DAC channel1 enable
pub struct EN1_R(crate::FieldReader<bool, EN1_A>);
impl EN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN1_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EN1_A {
        match self.bits {
            false => EN1_A::DISABLED,
            true => EN1_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EN1_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EN1_A::ENABLED
    }
}
impl core::ops::Deref for EN1_R {
    type Target = crate::FieldReader<bool, EN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EN1` writer - DAC channel1 enable
pub struct EN1_W<'a> {
    w: &'a mut W,
}
impl<'a> EN1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///DAC channel X disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN1_A::DISABLED)
    }
    ///DAC channel X enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN1_A::ENABLED)
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
    ///Bit 13 - DAC channel1 DMA Underrun Interrupt enable
    #[inline(always)]
    pub fn dmaudrie1(&self) -> DMAUDRIE1_R {
        DMAUDRIE1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 12 - DAC channel1 DMA enable
    #[inline(always)]
    pub fn dmaen1(&self) -> DMAEN1_R {
        DMAEN1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bits 8:11 - DAC channel1 mask/amplitude selector
    #[inline(always)]
    pub fn mamp1(&self) -> MAMP1_R {
        MAMP1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 6:7 - DAC channel1 noise/triangle wave generation enable
    #[inline(always)]
    pub fn wave1(&self) -> WAVE1_R {
        WAVE1_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    ///Bits 3:5 - DAC channel1 trigger selection
    #[inline(always)]
    pub fn tsel1(&self) -> TSEL1_R {
        TSEL1_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    ///Bit 2 - DAC channel1 trigger enable
    #[inline(always)]
    pub fn ten1(&self) -> TEN1_R {
        TEN1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - DAC channel1 output buffer disable
    #[inline(always)]
    pub fn boff1(&self) -> BOFF1_R {
        BOFF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - DAC channel1 enable
    #[inline(always)]
    pub fn en1(&self) -> EN1_R {
        EN1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 13 - DAC channel1 DMA Underrun Interrupt enable
    #[inline(always)]
    pub fn dmaudrie1(&mut self) -> DMAUDRIE1_W {
        DMAUDRIE1_W { w: self }
    }
    ///Bit 12 - DAC channel1 DMA enable
    #[inline(always)]
    pub fn dmaen1(&mut self) -> DMAEN1_W {
        DMAEN1_W { w: self }
    }
    ///Bits 8:11 - DAC channel1 mask/amplitude selector
    #[inline(always)]
    pub fn mamp1(&mut self) -> MAMP1_W {
        MAMP1_W { w: self }
    }
    ///Bits 6:7 - DAC channel1 noise/triangle wave generation enable
    #[inline(always)]
    pub fn wave1(&mut self) -> WAVE1_W {
        WAVE1_W { w: self }
    }
    ///Bits 3:5 - DAC channel1 trigger selection
    #[inline(always)]
    pub fn tsel1(&mut self) -> TSEL1_W {
        TSEL1_W { w: self }
    }
    ///Bit 2 - DAC channel1 trigger enable
    #[inline(always)]
    pub fn ten1(&mut self) -> TEN1_W {
        TEN1_W { w: self }
    }
    ///Bit 1 - DAC channel1 output buffer disable
    #[inline(always)]
    pub fn boff1(&mut self) -> BOFF1_W {
        BOFF1_W { w: self }
    }
    ///Bit 0 - DAC channel1 enable
    #[inline(always)]
    pub fn en1(&mut self) -> EN1_W {
        EN1_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](index.html) module
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr::R](R) reader structure
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr::W](W) writer structure
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
