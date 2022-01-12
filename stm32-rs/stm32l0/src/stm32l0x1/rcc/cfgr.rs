///Register `CFGR` reader
pub struct R(crate::R<CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CFGR` writer
pub struct W(crate::W<CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR_SPEC>;
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
impl From<crate::W<CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Microcontroller clock output prescaler
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MCOPRE_A {
    ///0: No division
    DIV1 = 0,
    ///1: Division by 2
    DIV2 = 1,
    ///2: Division by 4
    DIV4 = 2,
    ///3: Division by 8
    DIV8 = 3,
    ///4: Division by 16
    DIV16 = 4,
}
impl From<MCOPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: MCOPRE_A) -> Self {
        variant as _
    }
}
///Field `MCOPRE` reader - Microcontroller clock output prescaler
pub struct MCOPRE_R(crate::FieldReader<u8, MCOPRE_A>);
impl MCOPRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MCOPRE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<MCOPRE_A> {
        match self.bits {
            0 => Some(MCOPRE_A::DIV1),
            1 => Some(MCOPRE_A::DIV2),
            2 => Some(MCOPRE_A::DIV4),
            3 => Some(MCOPRE_A::DIV8),
            4 => Some(MCOPRE_A::DIV16),
            _ => None,
        }
    }
    ///Checks if the value of the field is `DIV1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        **self == MCOPRE_A::DIV1
    }
    ///Checks if the value of the field is `DIV2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == MCOPRE_A::DIV2
    }
    ///Checks if the value of the field is `DIV4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == MCOPRE_A::DIV4
    }
    ///Checks if the value of the field is `DIV8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == MCOPRE_A::DIV8
    }
    ///Checks if the value of the field is `DIV16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        **self == MCOPRE_A::DIV16
    }
}
impl core::ops::Deref for MCOPRE_R {
    type Target = crate::FieldReader<u8, MCOPRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MCOPRE` writer - Microcontroller clock output prescaler
pub struct MCOPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCOPRE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MCOPRE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///No division
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(MCOPRE_A::DIV1)
    }
    ///Division by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(MCOPRE_A::DIV2)
    }
    ///Division by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(MCOPRE_A::DIV4)
    }
    ///Division by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(MCOPRE_A::DIV8)
    }
    ///Division by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(MCOPRE_A::DIV16)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
///Microcontroller clock output selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MCOSEL_A {
    ///0: No clock
    NOCLOCK = 0,
    ///1: SYSCLK clock selected
    SYSCLK = 1,
    ///2: HSI oscillator clock selected
    HSI16 = 2,
    ///3: MSI oscillator clock selected
    MSI = 3,
    ///4: HSE oscillator clock selected
    HSE = 4,
    ///5: PLL clock selected
    PLL = 5,
    ///6: LSI oscillator clock selected
    LSI = 6,
    ///7: LSE oscillator clock selected
    LSE = 7,
}
impl From<MCOSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: MCOSEL_A) -> Self {
        variant as _
    }
}
///Field `MCOSEL` reader - Microcontroller clock output selection
pub struct MCOSEL_R(crate::FieldReader<u8, MCOSEL_A>);
impl MCOSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MCOSEL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MCOSEL_A {
        match self.bits {
            0 => MCOSEL_A::NOCLOCK,
            1 => MCOSEL_A::SYSCLK,
            2 => MCOSEL_A::HSI16,
            3 => MCOSEL_A::MSI,
            4 => MCOSEL_A::HSE,
            5 => MCOSEL_A::PLL,
            6 => MCOSEL_A::LSI,
            7 => MCOSEL_A::LSE,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `NOCLOCK`
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        **self == MCOSEL_A::NOCLOCK
    }
    ///Checks if the value of the field is `SYSCLK`
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        **self == MCOSEL_A::SYSCLK
    }
    ///Checks if the value of the field is `HSI16`
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        **self == MCOSEL_A::HSI16
    }
    ///Checks if the value of the field is `MSI`
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        **self == MCOSEL_A::MSI
    }
    ///Checks if the value of the field is `HSE`
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        **self == MCOSEL_A::HSE
    }
    ///Checks if the value of the field is `PLL`
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        **self == MCOSEL_A::PLL
    }
    ///Checks if the value of the field is `LSI`
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        **self == MCOSEL_A::LSI
    }
    ///Checks if the value of the field is `LSE`
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        **self == MCOSEL_A::LSE
    }
}
impl core::ops::Deref for MCOSEL_R {
    type Target = crate::FieldReader<u8, MCOSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MCOSEL` writer - Microcontroller clock output selection
pub struct MCOSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MCOSEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MCOSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///No clock
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut W {
        self.variant(MCOSEL_A::NOCLOCK)
    }
    ///SYSCLK clock selected
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(MCOSEL_A::SYSCLK)
    }
    ///HSI oscillator clock selected
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(MCOSEL_A::HSI16)
    }
    ///MSI oscillator clock selected
    #[inline(always)]
    pub fn msi(self) -> &'a mut W {
        self.variant(MCOSEL_A::MSI)
    }
    ///HSE oscillator clock selected
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(MCOSEL_A::HSE)
    }
    ///PLL clock selected
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(MCOSEL_A::PLL)
    }
    ///LSI oscillator clock selected
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(MCOSEL_A::LSI)
    }
    ///LSE oscillator clock selected
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(MCOSEL_A::LSE)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
///PLL output division
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLDIV_A {
    ///1: PLLVCO / 2
    DIV2 = 1,
    ///2: PLLVCO / 3
    DIV3 = 2,
    ///3: PLLVCO / 4
    DIV4 = 3,
}
impl From<PLLDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLDIV_A) -> Self {
        variant as _
    }
}
///Field `PLLDIV` reader - PLL output division
pub struct PLLDIV_R(crate::FieldReader<u8, PLLDIV_A>);
impl PLLDIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PLLDIV_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<PLLDIV_A> {
        match self.bits {
            1 => Some(PLLDIV_A::DIV2),
            2 => Some(PLLDIV_A::DIV3),
            3 => Some(PLLDIV_A::DIV4),
            _ => None,
        }
    }
    ///Checks if the value of the field is `DIV2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == PLLDIV_A::DIV2
    }
    ///Checks if the value of the field is `DIV3`
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        **self == PLLDIV_A::DIV3
    }
    ///Checks if the value of the field is `DIV4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == PLLDIV_A::DIV4
    }
}
impl core::ops::Deref for PLLDIV_R {
    type Target = crate::FieldReader<u8, PLLDIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PLLDIV` writer - PLL output division
pub struct PLLDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLDIV_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PLLDIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///PLLVCO / 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLDIV_A::DIV2)
    }
    ///PLLVCO / 3
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(PLLDIV_A::DIV3)
    }
    ///PLLVCO / 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLDIV_A::DIV4)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
///PLL multiplication factor
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLMUL_A {
    ///0: PLL clock entry x 3
    MUL3 = 0,
    ///1: PLL clock entry x 4
    MUL4 = 1,
    ///2: PLL clock entry x 6
    MUL6 = 2,
    ///3: PLL clock entry x 8
    MUL8 = 3,
    ///4: PLL clock entry x 12
    MUL12 = 4,
    ///5: PLL clock entry x 16
    MUL16 = 5,
    ///6: PLL clock entry x 24
    MUL24 = 6,
    ///7: PLL clock entry x 32
    MUL32 = 7,
    ///8: PLL clock entry x 48
    MUL48 = 8,
}
impl From<PLLMUL_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLMUL_A) -> Self {
        variant as _
    }
}
///Field `PLLMUL` reader - PLL multiplication factor
pub struct PLLMUL_R(crate::FieldReader<u8, PLLMUL_A>);
impl PLLMUL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PLLMUL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<PLLMUL_A> {
        match self.bits {
            0 => Some(PLLMUL_A::MUL3),
            1 => Some(PLLMUL_A::MUL4),
            2 => Some(PLLMUL_A::MUL6),
            3 => Some(PLLMUL_A::MUL8),
            4 => Some(PLLMUL_A::MUL12),
            5 => Some(PLLMUL_A::MUL16),
            6 => Some(PLLMUL_A::MUL24),
            7 => Some(PLLMUL_A::MUL32),
            8 => Some(PLLMUL_A::MUL48),
            _ => None,
        }
    }
    ///Checks if the value of the field is `MUL3`
    #[inline(always)]
    pub fn is_mul3(&self) -> bool {
        **self == PLLMUL_A::MUL3
    }
    ///Checks if the value of the field is `MUL4`
    #[inline(always)]
    pub fn is_mul4(&self) -> bool {
        **self == PLLMUL_A::MUL4
    }
    ///Checks if the value of the field is `MUL6`
    #[inline(always)]
    pub fn is_mul6(&self) -> bool {
        **self == PLLMUL_A::MUL6
    }
    ///Checks if the value of the field is `MUL8`
    #[inline(always)]
    pub fn is_mul8(&self) -> bool {
        **self == PLLMUL_A::MUL8
    }
    ///Checks if the value of the field is `MUL12`
    #[inline(always)]
    pub fn is_mul12(&self) -> bool {
        **self == PLLMUL_A::MUL12
    }
    ///Checks if the value of the field is `MUL16`
    #[inline(always)]
    pub fn is_mul16(&self) -> bool {
        **self == PLLMUL_A::MUL16
    }
    ///Checks if the value of the field is `MUL24`
    #[inline(always)]
    pub fn is_mul24(&self) -> bool {
        **self == PLLMUL_A::MUL24
    }
    ///Checks if the value of the field is `MUL32`
    #[inline(always)]
    pub fn is_mul32(&self) -> bool {
        **self == PLLMUL_A::MUL32
    }
    ///Checks if the value of the field is `MUL48`
    #[inline(always)]
    pub fn is_mul48(&self) -> bool {
        **self == PLLMUL_A::MUL48
    }
}
impl core::ops::Deref for PLLMUL_R {
    type Target = crate::FieldReader<u8, PLLMUL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PLLMUL` writer - PLL multiplication factor
pub struct PLLMUL_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLMUL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PLLMUL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///PLL clock entry x 3
    #[inline(always)]
    pub fn mul3(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL3)
    }
    ///PLL clock entry x 4
    #[inline(always)]
    pub fn mul4(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL4)
    }
    ///PLL clock entry x 6
    #[inline(always)]
    pub fn mul6(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL6)
    }
    ///PLL clock entry x 8
    #[inline(always)]
    pub fn mul8(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL8)
    }
    ///PLL clock entry x 12
    #[inline(always)]
    pub fn mul12(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL12)
    }
    ///PLL clock entry x 16
    #[inline(always)]
    pub fn mul16(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL16)
    }
    ///PLL clock entry x 24
    #[inline(always)]
    pub fn mul24(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL24)
    }
    ///PLL clock entry x 32
    #[inline(always)]
    pub fn mul32(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL32)
    }
    ///PLL clock entry x 48
    #[inline(always)]
    pub fn mul48(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL48)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | ((value as u32 & 0x0f) << 18);
        self.w
    }
}
///PLL entry clock source
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLSRC_A {
    ///0: HSI selected as PLL input clock
    HSI16 = 0,
    ///1: HSE selected as PLL input clock
    HSE = 1,
}
impl From<PLLSRC_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSRC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLSRC` reader - PLL entry clock source
pub struct PLLSRC_R(crate::FieldReader<bool, PLLSRC_A>);
impl PLLSRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLLSRC_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLSRC_A {
        match self.bits {
            false => PLLSRC_A::HSI16,
            true => PLLSRC_A::HSE,
        }
    }
    ///Checks if the value of the field is `HSI16`
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        **self == PLLSRC_A::HSI16
    }
    ///Checks if the value of the field is `HSE`
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        **self == PLLSRC_A::HSE
    }
}
impl core::ops::Deref for PLLSRC_R {
    type Target = crate::FieldReader<bool, PLLSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PLLSRC` writer - PLL entry clock source
pub struct PLLSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSRC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PLLSRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///HSI selected as PLL input clock
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(PLLSRC_A::HSI16)
    }
    ///HSE selected as PLL input clock
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(PLLSRC_A::HSE)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
///Wake-up from stop clock selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPWUCK_A {
    ///0: Internal 64 KHz to 4 MHz (MSI) oscillator selected as wake-up from Stop clock
    MSI = 0,
    ///1: Internal 16 MHz (HSI) oscillator selected as wake-up from Stop clock (or HSI16/4 if HSI16DIVEN=1)
    HSI16 = 1,
}
impl From<STOPWUCK_A> for bool {
    #[inline(always)]
    fn from(variant: STOPWUCK_A) -> Self {
        variant as u8 != 0
    }
}
///Field `STOPWUCK` reader - Wake-up from stop clock selection
pub struct STOPWUCK_R(crate::FieldReader<bool, STOPWUCK_A>);
impl STOPWUCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STOPWUCK_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> STOPWUCK_A {
        match self.bits {
            false => STOPWUCK_A::MSI,
            true => STOPWUCK_A::HSI16,
        }
    }
    ///Checks if the value of the field is `MSI`
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        **self == STOPWUCK_A::MSI
    }
    ///Checks if the value of the field is `HSI16`
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        **self == STOPWUCK_A::HSI16
    }
}
impl core::ops::Deref for STOPWUCK_R {
    type Target = crate::FieldReader<bool, STOPWUCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `STOPWUCK` writer - Wake-up from stop clock selection
pub struct STOPWUCK_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPWUCK_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: STOPWUCK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Internal 64 KHz to 4 MHz (MSI) oscillator selected as wake-up from Stop clock
    #[inline(always)]
    pub fn msi(self) -> &'a mut W {
        self.variant(STOPWUCK_A::MSI)
    }
    ///Internal 16 MHz (HSI) oscillator selected as wake-up from Stop clock (or HSI16/4 if HSI16DIVEN=1)
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(STOPWUCK_A::HSI16)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
///APB high-speed prescaler (APB2)
pub type PPRE2_A = PPRE1_A;
///Field `PPRE2` reader - APB high-speed prescaler (APB2)
pub type PPRE2_R = PPRE1_R;
///Field `PPRE2` writer - APB high-speed prescaler (APB2)
pub struct PPRE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PPRE2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PPRE2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///HCLK not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PPRE2_A::DIV1)
    }
    ///HCLK divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PPRE2_A::DIV2)
    }
    ///HCLK divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PPRE2_A::DIV4)
    }
    ///HCLK divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PPRE2_A::DIV8)
    }
    ///HCLK divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PPRE2_A::DIV16)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | ((value as u32 & 0x07) << 11);
        self.w
    }
}
///APB low-speed prescaler (APB1)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PPRE1_A {
    ///0: HCLK not divided
    DIV1 = 0,
    ///4: HCLK divided by 2
    DIV2 = 4,
    ///5: HCLK divided by 4
    DIV4 = 5,
    ///6: HCLK divided by 8
    DIV8 = 6,
    ///7: HCLK divided by 16
    DIV16 = 7,
}
impl From<PPRE1_A> for u8 {
    #[inline(always)]
    fn from(variant: PPRE1_A) -> Self {
        variant as _
    }
}
///Field `PPRE1` reader - APB low-speed prescaler (APB1)
pub struct PPRE1_R(crate::FieldReader<u8, PPRE1_A>);
impl PPRE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PPRE1_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<PPRE1_A> {
        match self.bits {
            0 => Some(PPRE1_A::DIV1),
            4 => Some(PPRE1_A::DIV2),
            5 => Some(PPRE1_A::DIV4),
            6 => Some(PPRE1_A::DIV8),
            7 => Some(PPRE1_A::DIV16),
            _ => None,
        }
    }
    ///Checks if the value of the field is `DIV1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        **self == PPRE1_A::DIV1
    }
    ///Checks if the value of the field is `DIV2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == PPRE1_A::DIV2
    }
    ///Checks if the value of the field is `DIV4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == PPRE1_A::DIV4
    }
    ///Checks if the value of the field is `DIV8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == PPRE1_A::DIV8
    }
    ///Checks if the value of the field is `DIV16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        **self == PPRE1_A::DIV16
    }
}
impl core::ops::Deref for PPRE1_R {
    type Target = crate::FieldReader<u8, PPRE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PPRE1` writer - APB low-speed prescaler (APB1)
pub struct PPRE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PPRE1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PPRE1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///HCLK not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PPRE1_A::DIV1)
    }
    ///HCLK divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PPRE1_A::DIV2)
    }
    ///HCLK divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PPRE1_A::DIV4)
    }
    ///HCLK divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PPRE1_A::DIV8)
    }
    ///HCLK divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PPRE1_A::DIV16)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
///AHB prescaler
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HPRE_A {
    ///0: system clock not divided
    DIV1 = 0,
    ///8: system clock divided by 2
    DIV2 = 8,
    ///9: system clock divided by 4
    DIV4 = 9,
    ///10: system clock divided by 8
    DIV8 = 10,
    ///11: system clock divided by 16
    DIV16 = 11,
    ///12: system clock divided by 64
    DIV64 = 12,
    ///13: system clock divided by 128
    DIV128 = 13,
    ///14: system clock divided by 256
    DIV256 = 14,
    ///15: system clock divided by 512
    DIV512 = 15,
}
impl From<HPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: HPRE_A) -> Self {
        variant as _
    }
}
///Field `HPRE` reader - AHB prescaler
pub struct HPRE_R(crate::FieldReader<u8, HPRE_A>);
impl HPRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HPRE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<HPRE_A> {
        match self.bits {
            0 => Some(HPRE_A::DIV1),
            8 => Some(HPRE_A::DIV2),
            9 => Some(HPRE_A::DIV4),
            10 => Some(HPRE_A::DIV8),
            11 => Some(HPRE_A::DIV16),
            12 => Some(HPRE_A::DIV64),
            13 => Some(HPRE_A::DIV128),
            14 => Some(HPRE_A::DIV256),
            15 => Some(HPRE_A::DIV512),
            _ => None,
        }
    }
    ///Checks if the value of the field is `DIV1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        **self == HPRE_A::DIV1
    }
    ///Checks if the value of the field is `DIV2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == HPRE_A::DIV2
    }
    ///Checks if the value of the field is `DIV4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == HPRE_A::DIV4
    }
    ///Checks if the value of the field is `DIV8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == HPRE_A::DIV8
    }
    ///Checks if the value of the field is `DIV16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        **self == HPRE_A::DIV16
    }
    ///Checks if the value of the field is `DIV64`
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        **self == HPRE_A::DIV64
    }
    ///Checks if the value of the field is `DIV128`
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        **self == HPRE_A::DIV128
    }
    ///Checks if the value of the field is `DIV256`
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        **self == HPRE_A::DIV256
    }
    ///Checks if the value of the field is `DIV512`
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        **self == HPRE_A::DIV512
    }
}
impl core::ops::Deref for HPRE_R {
    type Target = crate::FieldReader<u8, HPRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `HPRE` writer - AHB prescaler
pub struct HPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> HPRE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: HPRE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///system clock not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(HPRE_A::DIV1)
    }
    ///system clock divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(HPRE_A::DIV2)
    }
    ///system clock divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(HPRE_A::DIV4)
    }
    ///system clock divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(HPRE_A::DIV8)
    }
    ///system clock divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(HPRE_A::DIV16)
    }
    ///system clock divided by 64
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(HPRE_A::DIV64)
    }
    ///system clock divided by 128
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(HPRE_A::DIV128)
    }
    ///system clock divided by 256
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(HPRE_A::DIV256)
    }
    ///system clock divided by 512
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(HPRE_A::DIV512)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
///System clock switch status
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SWS_A {
    ///0: MSI oscillator used as system clock
    MSI = 0,
    ///1: HSI oscillator used as system clock
    HSI16 = 1,
    ///2: HSE oscillator used as system clock
    HSE = 2,
    ///3: PLL used as system clock
    PLL = 3,
}
impl From<SWS_A> for u8 {
    #[inline(always)]
    fn from(variant: SWS_A) -> Self {
        variant as _
    }
}
///Field `SWS` reader - System clock switch status
pub struct SWS_R(crate::FieldReader<u8, SWS_A>);
impl SWS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SWS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SWS_A {
        match self.bits {
            0 => SWS_A::MSI,
            1 => SWS_A::HSI16,
            2 => SWS_A::HSE,
            3 => SWS_A::PLL,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `MSI`
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        **self == SWS_A::MSI
    }
    ///Checks if the value of the field is `HSI16`
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        **self == SWS_A::HSI16
    }
    ///Checks if the value of the field is `HSE`
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        **self == SWS_A::HSE
    }
    ///Checks if the value of the field is `PLL`
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        **self == SWS_A::PLL
    }
}
impl core::ops::Deref for SWS_R {
    type Target = crate::FieldReader<u8, SWS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///System clock switch
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SW_A {
    ///0: MSI oscillator used as system clock
    MSI = 0,
    ///1: HSI oscillator used as system clock
    HSI16 = 1,
    ///2: HSE oscillator used as system clock
    HSE = 2,
    ///3: PLL used as system clock
    PLL = 3,
}
impl From<SW_A> for u8 {
    #[inline(always)]
    fn from(variant: SW_A) -> Self {
        variant as _
    }
}
///Field `SW` reader - System clock switch
pub struct SW_R(crate::FieldReader<u8, SW_A>);
impl SW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SW_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SW_A {
        match self.bits {
            0 => SW_A::MSI,
            1 => SW_A::HSI16,
            2 => SW_A::HSE,
            3 => SW_A::PLL,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `MSI`
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        **self == SW_A::MSI
    }
    ///Checks if the value of the field is `HSI16`
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        **self == SW_A::HSI16
    }
    ///Checks if the value of the field is `HSE`
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        **self == SW_A::HSE
    }
    ///Checks if the value of the field is `PLL`
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        **self == SW_A::PLL
    }
}
impl core::ops::Deref for SW_R {
    type Target = crate::FieldReader<u8, SW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SW` writer - System clock switch
pub struct SW_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SW_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///MSI oscillator used as system clock
    #[inline(always)]
    pub fn msi(self) -> &'a mut W {
        self.variant(SW_A::MSI)
    }
    ///HSI oscillator used as system clock
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(SW_A::HSI16)
    }
    ///HSE oscillator used as system clock
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(SW_A::HSE)
    }
    ///PLL used as system clock
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(SW_A::PLL)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    ///Bits 28:30 - Microcontroller clock output prescaler
    #[inline(always)]
    pub fn mcopre(&self) -> MCOPRE_R {
        MCOPRE_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    ///Bits 24:26 - Microcontroller clock output selection
    #[inline(always)]
    pub fn mcosel(&self) -> MCOSEL_R {
        MCOSEL_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    ///Bits 22:23 - PLL output division
    #[inline(always)]
    pub fn plldiv(&self) -> PLLDIV_R {
        PLLDIV_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    ///Bits 18:21 - PLL multiplication factor
    #[inline(always)]
    pub fn pllmul(&self) -> PLLMUL_R {
        PLLMUL_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    ///Bit 16 - PLL entry clock source
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 15 - Wake-up from stop clock selection
    #[inline(always)]
    pub fn stopwuck(&self) -> STOPWUCK_R {
        STOPWUCK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bits 11:13 - APB high-speed prescaler (APB2)
    #[inline(always)]
    pub fn ppre2(&self) -> PPRE2_R {
        PPRE2_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    ///Bits 8:10 - APB low-speed prescaler (APB1)
    #[inline(always)]
    pub fn ppre1(&self) -> PPRE1_R {
        PPRE1_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    ///Bits 4:7 - AHB prescaler
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 2:3 - System clock switch status
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    ///Bits 0:1 - System clock switch
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    ///Bits 28:30 - Microcontroller clock output prescaler
    #[inline(always)]
    pub fn mcopre(&mut self) -> MCOPRE_W {
        MCOPRE_W { w: self }
    }
    ///Bits 24:26 - Microcontroller clock output selection
    #[inline(always)]
    pub fn mcosel(&mut self) -> MCOSEL_W {
        MCOSEL_W { w: self }
    }
    ///Bits 22:23 - PLL output division
    #[inline(always)]
    pub fn plldiv(&mut self) -> PLLDIV_W {
        PLLDIV_W { w: self }
    }
    ///Bits 18:21 - PLL multiplication factor
    #[inline(always)]
    pub fn pllmul(&mut self) -> PLLMUL_W {
        PLLMUL_W { w: self }
    }
    ///Bit 16 - PLL entry clock source
    #[inline(always)]
    pub fn pllsrc(&mut self) -> PLLSRC_W {
        PLLSRC_W { w: self }
    }
    ///Bit 15 - Wake-up from stop clock selection
    #[inline(always)]
    pub fn stopwuck(&mut self) -> STOPWUCK_W {
        STOPWUCK_W { w: self }
    }
    ///Bits 11:13 - APB high-speed prescaler (APB2)
    #[inline(always)]
    pub fn ppre2(&mut self) -> PPRE2_W {
        PPRE2_W { w: self }
    }
    ///Bits 8:10 - APB low-speed prescaler (APB1)
    #[inline(always)]
    pub fn ppre1(&mut self) -> PPRE1_W {
        PPRE1_W { w: self }
    }
    ///Bits 4:7 - AHB prescaler
    #[inline(always)]
    pub fn hpre(&mut self) -> HPRE_W {
        HPRE_W { w: self }
    }
    ///Bits 0:1 - System clock switch
    #[inline(always)]
    pub fn sw(&mut self) -> SW_W {
        SW_W { w: self }
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
///For information about available fields see [cfgr](index.html) module
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cfgr::R](R) reader structure
impl crate::Readable for CFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cfgr::W](W) writer structure
impl crate::Writable for CFGR_SPEC {
    type Writer = W;
}
///`reset()` method sets CFGR to value 0
impl crate::Resettable for CFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
