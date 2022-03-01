///Register `CSR` reader
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CSR` writer
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
///Low-power reset flag
pub type LPWRRSTF_A = FWRSTF_A;
///Field `LPWRRSTF` reader - Low-power reset flag
pub type LPWRRSTF_R = FWRSTF_R;
///Field `LPWRRSTF` writer - Low-power reset flag
pub struct LPWRRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> LPWRRSTF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LPWRRSTF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No reset has occured
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(LPWRRSTF_A::NORESET)
    }
    ///A reset has occured
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(LPWRRSTF_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
///Window watchdog reset flag
pub type WWDGRSTF_A = FWRSTF_A;
///Field `WWDGRSTF` reader - Window watchdog reset flag
pub type WWDGRSTF_R = FWRSTF_R;
///Field `WWDGRSTF` writer - Window watchdog reset flag
pub struct WWDGRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDGRSTF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WWDGRSTF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No reset has occured
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(WWDGRSTF_A::NORESET)
    }
    ///A reset has occured
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(WWDGRSTF_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
///Independent watchdog reset flag
pub type IWDGRSTF_A = FWRSTF_A;
///Field `IWDGRSTF` reader - Independent watchdog reset flag
pub type IWDGRSTF_R = FWRSTF_R;
///Field `IWDGRSTF` writer - Independent watchdog reset flag
pub struct IWDGRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> IWDGRSTF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IWDGRSTF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No reset has occured
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(IWDGRSTF_A::NORESET)
    }
    ///A reset has occured
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(IWDGRSTF_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
///Software reset flag
pub type SFTRSTF_A = FWRSTF_A;
///Field `SFTRSTF` reader - Software reset flag
pub type SFTRSTF_R = FWRSTF_R;
///Field `SFTRSTF` writer - Software reset flag
pub struct SFTRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> SFTRSTF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SFTRSTF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No reset has occured
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(SFTRSTF_A::NORESET)
    }
    ///A reset has occured
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SFTRSTF_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
///POR/PDR reset flag
pub type PORRSTF_A = FWRSTF_A;
///Field `PORRSTF` reader - POR/PDR reset flag
pub type PORRSTF_R = FWRSTF_R;
///Field `PORRSTF` writer - POR/PDR reset flag
pub struct PORRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> PORRSTF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PORRSTF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No reset has occured
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(PORRSTF_A::NORESET)
    }
    ///A reset has occured
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(PORRSTF_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
///PIN reset flag
pub type PINRSTF_A = FWRSTF_A;
///Field `PINRSTF` reader - PIN reset flag
pub type PINRSTF_R = FWRSTF_R;
///Field `PINRSTF` writer - PIN reset flag
pub struct PINRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> PINRSTF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PINRSTF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No reset has occured
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(PINRSTF_A::NORESET)
    }
    ///A reset has occured
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(PINRSTF_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
///OBLRSTF
pub type OBLRSTF_A = FWRSTF_A;
///Field `OBLRSTF` reader - OBLRSTF
pub type OBLRSTF_R = FWRSTF_R;
///Field `OBLRSTF` writer - OBLRSTF
pub struct OBLRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> OBLRSTF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OBLRSTF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No reset has occured
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(OBLRSTF_A::NORESET)
    }
    ///A reset has occured
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(OBLRSTF_A::RESET)
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
///Firewall reset flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWRSTF_A {
    ///0: No reset has occured
    NORESET = 0,
    ///1: A reset has occured
    RESET = 1,
}
impl From<FWRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: FWRSTF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FWRSTF` reader - Firewall reset flag
pub struct FWRSTF_R(crate::FieldReader<bool, FWRSTF_A>);
impl FWRSTF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FWRSTF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FWRSTF_A {
        match self.bits {
            false => FWRSTF_A::NORESET,
            true => FWRSTF_A::RESET,
        }
    }
    ///Checks if the value of the field is `NORESET`
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        **self == FWRSTF_A::NORESET
    }
    ///Checks if the value of the field is `RESET`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == FWRSTF_A::RESET
    }
}
impl core::ops::Deref for FWRSTF_R {
    type Target = crate::FieldReader<bool, FWRSTF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `FWRSTF` writer - Firewall reset flag
pub struct FWRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> FWRSTF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FWRSTF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No reset has occured
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(FWRSTF_A::NORESET)
    }
    ///A reset has occured
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(FWRSTF_A::RESET)
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
///RTC software reset bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCRST_A {
    ///1: Resets the RTC peripheral
    RESET = 1,
}
impl From<RTCRST_A> for bool {
    #[inline(always)]
    fn from(variant: RTCRST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RTCRST` reader - RTC software reset bit
pub struct RTCRST_R(crate::FieldReader<bool, RTCRST_A>);
impl RTCRST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTCRST_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<RTCRST_A> {
        match self.bits {
            true => Some(RTCRST_A::RESET),
            _ => None,
        }
    }
    ///Checks if the value of the field is `RESET`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == RTCRST_A::RESET
    }
}
impl core::ops::Deref for RTCRST_R {
    type Target = crate::FieldReader<bool, RTCRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RTCRST` writer - RTC software reset bit
pub struct RTCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCRST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RTCRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Resets the RTC peripheral
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RTCRST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
///RTC clock enable bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCEN_A {
    ///0: RTC clock disabled
    DISABLED = 0,
    ///1: RTC clock enabled
    ENABLED = 1,
}
impl From<RTCEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTCEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RTCEN` reader - RTC clock enable bit
pub struct RTCEN_R(crate::FieldReader<bool, RTCEN_A>);
impl RTCEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTCEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RTCEN_A {
        match self.bits {
            false => RTCEN_A::DISABLED,
            true => RTCEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RTCEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RTCEN_A::ENABLED
    }
}
impl core::ops::Deref for RTCEN_R {
    type Target = crate::FieldReader<bool, RTCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RTCEN` writer - RTC clock enable bit
pub struct RTCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RTCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///RTC clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTCEN_A::DISABLED)
    }
    ///RTC clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RTCEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
///RTC and LCD clock source selection bits
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTCSEL_A {
    ///0: No clock
    NOCLOCK = 0,
    ///1: LSE oscillator clock used as RTC clock
    LSE = 1,
    ///2: LSI oscillator clock used as RTC clock
    LSI = 2,
    ///3: HSE oscillator clock divided by a programmable prescaler (selection through the RTCPRE\[1:0\]
    ///bits in the RCC clock control register (RCC_CR)) used as the RTC clock
    HSE = 3,
}
impl From<RTCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCSEL_A) -> Self {
        variant as _
    }
}
///Field `RTCSEL` reader - RTC and LCD clock source selection bits
pub struct RTCSEL_R(crate::FieldReader<u8, RTCSEL_A>);
impl RTCSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTCSEL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RTCSEL_A {
        match self.bits {
            0 => RTCSEL_A::NOCLOCK,
            1 => RTCSEL_A::LSE,
            2 => RTCSEL_A::LSI,
            3 => RTCSEL_A::HSE,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `NOCLOCK`
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        **self == RTCSEL_A::NOCLOCK
    }
    ///Checks if the value of the field is `LSE`
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        **self == RTCSEL_A::LSE
    }
    ///Checks if the value of the field is `LSI`
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        **self == RTCSEL_A::LSI
    }
    ///Checks if the value of the field is `HSE`
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        **self == RTCSEL_A::HSE
    }
}
impl core::ops::Deref for RTCSEL_R {
    type Target = crate::FieldReader<u8, RTCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RTCSEL` writer - RTC and LCD clock source selection bits
pub struct RTCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCSEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RTCSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///No clock
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut W {
        self.variant(RTCSEL_A::NOCLOCK)
    }
    ///LSE oscillator clock used as RTC clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(RTCSEL_A::LSE)
    }
    ///LSI oscillator clock used as RTC clock
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(RTCSEL_A::LSI)
    }
    ///HSE oscillator clock divided by a programmable prescaler (selection through the RTCPRE\[1:0\]
    ///bits in the RCC clock control register (RCC_CR)) used as the RTC clock
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(RTCSEL_A::HSE)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
///CSS on LSE failure detection flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSSLSED_A {
    ///0: No failure detected on LSE (32 kHz oscillator)
    NOFAILURE = 0,
    ///1: Failure detected on LSE (32 kHz oscillator)
    FAILURE = 1,
}
impl From<CSSLSED_A> for bool {
    #[inline(always)]
    fn from(variant: CSSLSED_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSSLSED` reader - CSS on LSE failure detection flag
pub struct CSSLSED_R(crate::FieldReader<bool, CSSLSED_A>);
impl CSSLSED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSSLSED_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CSSLSED_A {
        match self.bits {
            false => CSSLSED_A::NOFAILURE,
            true => CSSLSED_A::FAILURE,
        }
    }
    ///Checks if the value of the field is `NOFAILURE`
    #[inline(always)]
    pub fn is_no_failure(&self) -> bool {
        **self == CSSLSED_A::NOFAILURE
    }
    ///Checks if the value of the field is `FAILURE`
    #[inline(always)]
    pub fn is_failure(&self) -> bool {
        **self == CSSLSED_A::FAILURE
    }
}
impl core::ops::Deref for CSSLSED_R {
    type Target = crate::FieldReader<bool, CSSLSED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CSSLSED` writer - CSS on LSE failure detection flag
pub struct CSSLSED_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSLSED_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CSSLSED_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No failure detected on LSE (32 kHz oscillator)
    #[inline(always)]
    pub fn no_failure(self) -> &'a mut W {
        self.variant(CSSLSED_A::NOFAILURE)
    }
    ///Failure detected on LSE (32 kHz oscillator)
    #[inline(always)]
    pub fn failure(self) -> &'a mut W {
        self.variant(CSSLSED_A::FAILURE)
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
///CSSLSEON
pub type CSSLSEON_A = LSION_A;
///Field `CSSLSEON` reader - CSSLSEON
pub type CSSLSEON_R = LSION_R;
///Field `CSSLSEON` writer - CSSLSEON
pub struct CSSLSEON_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSLSEON_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CSSLSEON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Oscillator OFF
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CSSLSEON_A::OFF)
    }
    ///Oscillator ON
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(CSSLSEON_A::ON)
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
///LSEDRV
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LSEDRV_A {
    ///0: Lowest drive
    LOW = 0,
    ///1: Medium low drive
    MEDIUMLOW = 1,
    ///2: Medium high drive
    MEDIUMHIGH = 2,
    ///3: Highest drive
    HIGH = 3,
}
impl From<LSEDRV_A> for u8 {
    #[inline(always)]
    fn from(variant: LSEDRV_A) -> Self {
        variant as _
    }
}
///Field `LSEDRV` reader - LSEDRV
pub struct LSEDRV_R(crate::FieldReader<u8, LSEDRV_A>);
impl LSEDRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LSEDRV_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSEDRV_A {
        match self.bits {
            0 => LSEDRV_A::LOW,
            1 => LSEDRV_A::MEDIUMLOW,
            2 => LSEDRV_A::MEDIUMHIGH,
            3 => LSEDRV_A::HIGH,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `LOW`
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == LSEDRV_A::LOW
    }
    ///Checks if the value of the field is `MEDIUMLOW`
    #[inline(always)]
    pub fn is_medium_low(&self) -> bool {
        **self == LSEDRV_A::MEDIUMLOW
    }
    ///Checks if the value of the field is `MEDIUMHIGH`
    #[inline(always)]
    pub fn is_medium_high(&self) -> bool {
        **self == LSEDRV_A::MEDIUMHIGH
    }
    ///Checks if the value of the field is `HIGH`
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == LSEDRV_A::HIGH
    }
}
impl core::ops::Deref for LSEDRV_R {
    type Target = crate::FieldReader<u8, LSEDRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LSEDRV` writer - LSEDRV
pub struct LSEDRV_W<'a> {
    w: &'a mut W,
}
impl<'a> LSEDRV_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LSEDRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Lowest drive
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(LSEDRV_A::LOW)
    }
    ///Medium low drive
    #[inline(always)]
    pub fn medium_low(self) -> &'a mut W {
        self.variant(LSEDRV_A::MEDIUMLOW)
    }
    ///Medium high drive
    #[inline(always)]
    pub fn medium_high(self) -> &'a mut W {
        self.variant(LSEDRV_A::MEDIUMHIGH)
    }
    ///Highest drive
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(LSEDRV_A::HIGH)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | ((value as u32 & 0x03) << 11);
        self.w
    }
}
///External low-speed oscillator bypass bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSEBYP_A {
    ///0: LSE oscillator not bypassed
    NOTBYPASSED = 0,
    ///1: LSE oscillator bypassed
    BYPASSED = 1,
}
impl From<LSEBYP_A> for bool {
    #[inline(always)]
    fn from(variant: LSEBYP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LSEBYP` reader - External low-speed oscillator bypass bit
pub struct LSEBYP_R(crate::FieldReader<bool, LSEBYP_A>);
impl LSEBYP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSEBYP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSEBYP_A {
        match self.bits {
            false => LSEBYP_A::NOTBYPASSED,
            true => LSEBYP_A::BYPASSED,
        }
    }
    ///Checks if the value of the field is `NOTBYPASSED`
    #[inline(always)]
    pub fn is_not_bypassed(&self) -> bool {
        **self == LSEBYP_A::NOTBYPASSED
    }
    ///Checks if the value of the field is `BYPASSED`
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        **self == LSEBYP_A::BYPASSED
    }
}
impl core::ops::Deref for LSEBYP_R {
    type Target = crate::FieldReader<bool, LSEBYP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LSEBYP` writer - External low-speed oscillator bypass bit
pub struct LSEBYP_W<'a> {
    w: &'a mut W,
}
impl<'a> LSEBYP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LSEBYP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///LSE oscillator not bypassed
    #[inline(always)]
    pub fn not_bypassed(self) -> &'a mut W {
        self.variant(LSEBYP_A::NOTBYPASSED)
    }
    ///LSE oscillator bypassed
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(LSEBYP_A::BYPASSED)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
///External low-speed oscillator ready bit
pub type LSERDY_A = LSIRDY_A;
///Field `LSERDY` reader - External low-speed oscillator ready bit
pub type LSERDY_R = LSIRDY_R;
///External low-speed oscillator enable bit
pub type LSEON_A = LSION_A;
///Field `LSEON` reader - External low-speed oscillator enable bit
pub type LSEON_R = LSION_R;
///Field `LSEON` writer - External low-speed oscillator enable bit
pub struct LSEON_W<'a> {
    w: &'a mut W,
}
impl<'a> LSEON_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LSEON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Oscillator OFF
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(LSEON_A::OFF)
    }
    ///Oscillator ON
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(LSEON_A::ON)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
///Internal low-speed oscillator ready bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSIRDY_A {
    ///0: Oscillator not ready
    NOTREADY = 0,
    ///1: Oscillator ready
    READY = 1,
}
impl From<LSIRDY_A> for bool {
    #[inline(always)]
    fn from(variant: LSIRDY_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LSIRDY` reader - Internal low-speed oscillator ready bit
pub struct LSIRDY_R(crate::FieldReader<bool, LSIRDY_A>);
impl LSIRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSIRDY_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSIRDY_A {
        match self.bits {
            false => LSIRDY_A::NOTREADY,
            true => LSIRDY_A::READY,
        }
    }
    ///Checks if the value of the field is `NOTREADY`
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        **self == LSIRDY_A::NOTREADY
    }
    ///Checks if the value of the field is `READY`
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == LSIRDY_A::READY
    }
}
impl core::ops::Deref for LSIRDY_R {
    type Target = crate::FieldReader<bool, LSIRDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Internal low-speed oscillator enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSION_A {
    ///0: Oscillator OFF
    OFF = 0,
    ///1: Oscillator ON
    ON = 1,
}
impl From<LSION_A> for bool {
    #[inline(always)]
    fn from(variant: LSION_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LSION` reader - Internal low-speed oscillator enable
pub struct LSION_R(crate::FieldReader<bool, LSION_A>);
impl LSION_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSION_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSION_A {
        match self.bits {
            false => LSION_A::OFF,
            true => LSION_A::ON,
        }
    }
    ///Checks if the value of the field is `OFF`
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == LSION_A::OFF
    }
    ///Checks if the value of the field is `ON`
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == LSION_A::ON
    }
}
impl core::ops::Deref for LSION_R {
    type Target = crate::FieldReader<bool, LSION_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LSION` writer - Internal low-speed oscillator enable
pub struct LSION_W<'a> {
    w: &'a mut W,
}
impl<'a> LSION_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LSION_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Oscillator OFF
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(LSION_A::OFF)
    }
    ///Oscillator ON
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(LSION_A::ON)
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
///Remove reset flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMVF_A {
    ///1: Clears the reset flag
    CLEAR = 1,
}
impl From<RMVF_A> for bool {
    #[inline(always)]
    fn from(variant: RMVF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RMVF` reader - Remove reset flag
pub struct RMVF_R(crate::FieldReader<bool, RMVF_A>);
impl RMVF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RMVF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<RMVF_A> {
        match self.bits {
            true => Some(RMVF_A::CLEAR),
            _ => None,
        }
    }
    ///Checks if the value of the field is `CLEAR`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == RMVF_A::CLEAR
    }
}
impl core::ops::Deref for RMVF_R {
    type Target = crate::FieldReader<bool, RMVF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RMVF` writer - Remove reset flag
pub struct RMVF_W<'a> {
    w: &'a mut W,
}
impl<'a> RMVF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RMVF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the reset flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RMVF_A::CLEAR)
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
impl R {
    ///Bit 31 - Low-power reset flag
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    ///Bit 30 - Window watchdog reset flag
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 29 - Independent watchdog reset flag
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IWDGRSTF_R {
        IWDGRSTF_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    ///Bit 28 - Software reset flag
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    ///Bit 27 - POR/PDR reset flag
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    ///Bit 26 - PIN reset flag
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bit 25 - OBLRSTF
    #[inline(always)]
    pub fn oblrstf(&self) -> OBLRSTF_R {
        OBLRSTF_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bit 24 - Firewall reset flag
    #[inline(always)]
    pub fn fwrstf(&self) -> FWRSTF_R {
        FWRSTF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 19 - RTC software reset bit
    #[inline(always)]
    pub fn rtcrst(&self) -> RTCRST_R {
        RTCRST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 18 - RTC clock enable bit
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bits 16:17 - RTC and LCD clock source selection bits
    #[inline(always)]
    pub fn rtcsel(&self) -> RTCSEL_R {
        RTCSEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    ///Bit 14 - CSS on LSE failure detection flag
    #[inline(always)]
    pub fn csslsed(&self) -> CSSLSED_R {
        CSSLSED_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 13 - CSSLSEON
    #[inline(always)]
    pub fn csslseon(&self) -> CSSLSEON_R {
        CSSLSEON_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bits 11:12 - LSEDRV
    #[inline(always)]
    pub fn lsedrv(&self) -> LSEDRV_R {
        LSEDRV_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    ///Bit 10 - External low-speed oscillator bypass bit
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 9 - External low-speed oscillator ready bit
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - External low-speed oscillator enable bit
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 1 - Internal low-speed oscillator ready bit
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Internal low-speed oscillator enable
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 23 - Remove reset flag
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    ///Bit 31 - Low-power reset flag
    #[inline(always)]
    pub fn lpwrrstf(&mut self) -> LPWRRSTF_W {
        LPWRRSTF_W { w: self }
    }
    ///Bit 30 - Window watchdog reset flag
    #[inline(always)]
    pub fn wwdgrstf(&mut self) -> WWDGRSTF_W {
        WWDGRSTF_W { w: self }
    }
    ///Bit 29 - Independent watchdog reset flag
    #[inline(always)]
    pub fn iwdgrstf(&mut self) -> IWDGRSTF_W {
        IWDGRSTF_W { w: self }
    }
    ///Bit 28 - Software reset flag
    #[inline(always)]
    pub fn sftrstf(&mut self) -> SFTRSTF_W {
        SFTRSTF_W { w: self }
    }
    ///Bit 27 - POR/PDR reset flag
    #[inline(always)]
    pub fn porrstf(&mut self) -> PORRSTF_W {
        PORRSTF_W { w: self }
    }
    ///Bit 26 - PIN reset flag
    #[inline(always)]
    pub fn pinrstf(&mut self) -> PINRSTF_W {
        PINRSTF_W { w: self }
    }
    ///Bit 25 - OBLRSTF
    #[inline(always)]
    pub fn oblrstf(&mut self) -> OBLRSTF_W {
        OBLRSTF_W { w: self }
    }
    ///Bit 24 - Firewall reset flag
    #[inline(always)]
    pub fn fwrstf(&mut self) -> FWRSTF_W {
        FWRSTF_W { w: self }
    }
    ///Bit 19 - RTC software reset bit
    #[inline(always)]
    pub fn rtcrst(&mut self) -> RTCRST_W {
        RTCRST_W { w: self }
    }
    ///Bit 18 - RTC clock enable bit
    #[inline(always)]
    pub fn rtcen(&mut self) -> RTCEN_W {
        RTCEN_W { w: self }
    }
    ///Bits 16:17 - RTC and LCD clock source selection bits
    #[inline(always)]
    pub fn rtcsel(&mut self) -> RTCSEL_W {
        RTCSEL_W { w: self }
    }
    ///Bit 14 - CSS on LSE failure detection flag
    #[inline(always)]
    pub fn csslsed(&mut self) -> CSSLSED_W {
        CSSLSED_W { w: self }
    }
    ///Bit 13 - CSSLSEON
    #[inline(always)]
    pub fn csslseon(&mut self) -> CSSLSEON_W {
        CSSLSEON_W { w: self }
    }
    ///Bits 11:12 - LSEDRV
    #[inline(always)]
    pub fn lsedrv(&mut self) -> LSEDRV_W {
        LSEDRV_W { w: self }
    }
    ///Bit 10 - External low-speed oscillator bypass bit
    #[inline(always)]
    pub fn lsebyp(&mut self) -> LSEBYP_W {
        LSEBYP_W { w: self }
    }
    ///Bit 8 - External low-speed oscillator enable bit
    #[inline(always)]
    pub fn lseon(&mut self) -> LSEON_W {
        LSEON_W { w: self }
    }
    ///Bit 0 - Internal low-speed oscillator enable
    #[inline(always)]
    pub fn lsion(&mut self) -> LSION_W {
        LSION_W { w: self }
    }
    ///Bit 23 - Remove reset flag
    #[inline(always)]
    pub fn rmvf(&mut self) -> RMVF_W {
        RMVF_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Control and status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [csr](index.html) module
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [csr::R](R) reader structure
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [csr::W](W) writer structure
impl crate::Writable for CSR_SPEC {
    type Writer = W;
}
///`reset()` method sets CSR to value 0x0c00_0000
impl crate::Resettable for CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0c00_0000
    }
}
