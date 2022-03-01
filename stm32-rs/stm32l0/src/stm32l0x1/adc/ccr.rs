///Register `CCR` reader
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCR` writer
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
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
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
///ADC prescaler
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESC_A {
    ///0: Input ADC clock not divided
    DIV1 = 0,
    ///1: Input ADC clock divided by 2
    DIV2 = 1,
    ///2: Input ADC clock divided by 4
    DIV4 = 2,
    ///3: Input ADC clock divided by 6
    DIV6 = 3,
    ///4: Input ADC clock divided by 8
    DIV8 = 4,
    ///5: Input ADC clock divided by 10
    DIV10 = 5,
    ///6: Input ADC clock divided by 12
    DIV12 = 6,
    ///7: Input ADC clock divided by 16
    DIV16 = 7,
    ///8: Input ADC clock divided by 32
    DIV32 = 8,
    ///9: Input ADC clock divided by 64
    DIV64 = 9,
    ///10: Input ADC clock divided by 128
    DIV128 = 10,
    ///11: Input ADC clock divided by 256
    DIV256 = 11,
}
impl From<PRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESC_A) -> Self {
        variant as _
    }
}
///Field `PRESC` reader - ADC prescaler
pub struct PRESC_R(crate::FieldReader<u8, PRESC_A>);
impl PRESC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRESC_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<PRESC_A> {
        match self.bits {
            0 => Some(PRESC_A::DIV1),
            1 => Some(PRESC_A::DIV2),
            2 => Some(PRESC_A::DIV4),
            3 => Some(PRESC_A::DIV6),
            4 => Some(PRESC_A::DIV8),
            5 => Some(PRESC_A::DIV10),
            6 => Some(PRESC_A::DIV12),
            7 => Some(PRESC_A::DIV16),
            8 => Some(PRESC_A::DIV32),
            9 => Some(PRESC_A::DIV64),
            10 => Some(PRESC_A::DIV128),
            11 => Some(PRESC_A::DIV256),
            _ => None,
        }
    }
    ///Checks if the value of the field is `DIV1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        **self == PRESC_A::DIV1
    }
    ///Checks if the value of the field is `DIV2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == PRESC_A::DIV2
    }
    ///Checks if the value of the field is `DIV4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == PRESC_A::DIV4
    }
    ///Checks if the value of the field is `DIV6`
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        **self == PRESC_A::DIV6
    }
    ///Checks if the value of the field is `DIV8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == PRESC_A::DIV8
    }
    ///Checks if the value of the field is `DIV10`
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        **self == PRESC_A::DIV10
    }
    ///Checks if the value of the field is `DIV12`
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        **self == PRESC_A::DIV12
    }
    ///Checks if the value of the field is `DIV16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        **self == PRESC_A::DIV16
    }
    ///Checks if the value of the field is `DIV32`
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        **self == PRESC_A::DIV32
    }
    ///Checks if the value of the field is `DIV64`
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        **self == PRESC_A::DIV64
    }
    ///Checks if the value of the field is `DIV128`
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        **self == PRESC_A::DIV128
    }
    ///Checks if the value of the field is `DIV256`
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        **self == PRESC_A::DIV256
    }
}
impl core::ops::Deref for PRESC_R {
    type Target = crate::FieldReader<u8, PRESC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PRESC` writer - ADC prescaler
pub struct PRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PRESC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Input ADC clock not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PRESC_A::DIV1)
    }
    ///Input ADC clock divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PRESC_A::DIV2)
    }
    ///Input ADC clock divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESC_A::DIV4)
    }
    ///Input ADC clock divided by 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PRESC_A::DIV6)
    }
    ///Input ADC clock divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PRESC_A::DIV8)
    }
    ///Input ADC clock divided by 10
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(PRESC_A::DIV10)
    }
    ///Input ADC clock divided by 12
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PRESC_A::DIV12)
    }
    ///Input ADC clock divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PRESC_A::DIV16)
    }
    ///Input ADC clock divided by 32
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PRESC_A::DIV32)
    }
    ///Input ADC clock divided by 64
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PRESC_A::DIV64)
    }
    ///Input ADC clock divided by 128
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(PRESC_A::DIV128)
    }
    ///Input ADC clock divided by 256
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(PRESC_A::DIV256)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | ((value as u32 & 0x0f) << 18);
        self.w
    }
}
///VREFINT enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREFEN_A {
    ///0: VREFINT disabled
    DISABLED = 0,
    ///1: VREFINT enabled
    ENABLED = 1,
}
impl From<VREFEN_A> for bool {
    #[inline(always)]
    fn from(variant: VREFEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VREFEN` reader - VREFINT enable
pub struct VREFEN_R(crate::FieldReader<bool, VREFEN_A>);
impl VREFEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VREFEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> VREFEN_A {
        match self.bits {
            false => VREFEN_A::DISABLED,
            true => VREFEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == VREFEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == VREFEN_A::ENABLED
    }
}
impl core::ops::Deref for VREFEN_R {
    type Target = crate::FieldReader<bool, VREFEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `VREFEN` writer - VREFINT enable
pub struct VREFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: VREFEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///VREFINT disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VREFEN_A::DISABLED)
    }
    ///VREFINT enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VREFEN_A::ENABLED)
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
///Temperature sensor enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSEN_A {
    ///0: Temperature sensor disabled
    DISABLED = 0,
    ///1: Temperature sensor enabled
    ENABLED = 1,
}
impl From<TSEN_A> for bool {
    #[inline(always)]
    fn from(variant: TSEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TSEN` reader - Temperature sensor enable
pub struct TSEN_R(crate::FieldReader<bool, TSEN_A>);
impl TSEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TSEN_A {
        match self.bits {
            false => TSEN_A::DISABLED,
            true => TSEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TSEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TSEN_A::ENABLED
    }
}
impl core::ops::Deref for TSEN_R {
    type Target = crate::FieldReader<bool, TSEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TSEN` writer - Temperature sensor enable
pub struct TSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TSEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Temperature sensor disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TSEN_A::DISABLED)
    }
    ///Temperature sensor enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TSEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
///VLCD enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VLCDEN_A {
    ///0: VLCD reading circuitry disabled
    DISABLED = 0,
    ///1: VLCD reading circuitry enabled
    ENABLED = 1,
}
impl From<VLCDEN_A> for bool {
    #[inline(always)]
    fn from(variant: VLCDEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VLCDEN` reader - VLCD enable
pub struct VLCDEN_R(crate::FieldReader<bool, VLCDEN_A>);
impl VLCDEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VLCDEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> VLCDEN_A {
        match self.bits {
            false => VLCDEN_A::DISABLED,
            true => VLCDEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == VLCDEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == VLCDEN_A::ENABLED
    }
}
impl core::ops::Deref for VLCDEN_R {
    type Target = crate::FieldReader<bool, VLCDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `VLCDEN` writer - VLCD enable
pub struct VLCDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VLCDEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: VLCDEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///VLCD reading circuitry disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VLCDEN_A::DISABLED)
    }
    ///VLCD reading circuitry enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VLCDEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
///Low Frequency Mode enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFMEN_A {
    ///0: Low Frequency Mode disabled
    DISABLED = 0,
    ///1: Low Frequency Mode enabled
    ENABLED = 1,
}
impl From<LFMEN_A> for bool {
    #[inline(always)]
    fn from(variant: LFMEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LFMEN` reader - Low Frequency Mode enable
pub struct LFMEN_R(crate::FieldReader<bool, LFMEN_A>);
impl LFMEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LFMEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LFMEN_A {
        match self.bits {
            false => LFMEN_A::DISABLED,
            true => LFMEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == LFMEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == LFMEN_A::ENABLED
    }
}
impl core::ops::Deref for LFMEN_R {
    type Target = crate::FieldReader<bool, LFMEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LFMEN` writer - Low Frequency Mode enable
pub struct LFMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LFMEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LFMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Low Frequency Mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LFMEN_A::DISABLED)
    }
    ///Low Frequency Mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LFMEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
impl R {
    ///Bits 18:21 - ADC prescaler
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    ///Bit 22 - VREFINT enable
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 23 - Temperature sensor enable
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 24 - VLCD enable
    #[inline(always)]
    pub fn vlcden(&self) -> VLCDEN_R {
        VLCDEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 25 - Low Frequency Mode enable
    #[inline(always)]
    pub fn lfmen(&self) -> LFMEN_R {
        LFMEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    ///Bits 18:21 - ADC prescaler
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W {
        PRESC_W { w: self }
    }
    ///Bit 22 - VREFINT enable
    #[inline(always)]
    pub fn vrefen(&mut self) -> VREFEN_W {
        VREFEN_W { w: self }
    }
    ///Bit 23 - Temperature sensor enable
    #[inline(always)]
    pub fn tsen(&mut self) -> TSEN_W {
        TSEN_W { w: self }
    }
    ///Bit 24 - VLCD enable
    #[inline(always)]
    pub fn vlcden(&mut self) -> VLCDEN_W {
        VLCDEN_W { w: self }
    }
    ///Bit 25 - Low Frequency Mode enable
    #[inline(always)]
    pub fn lfmen(&mut self) -> LFMEN_W {
        LFMEN_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC common configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccr](index.html) module
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccr::R](R) reader structure
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccr::W](W) writer structure
impl crate::Writable for CCR_SPEC {
    type Writer = W;
}
///`reset()` method sets CCR to value 0
impl crate::Resettable for CCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
