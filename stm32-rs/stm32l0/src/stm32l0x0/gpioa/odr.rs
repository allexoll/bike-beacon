///Register `ODR` reader
pub struct R(crate::R<ODR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ODR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ODR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ODR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ODR` writer
pub struct W(crate::W<ODR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ODR_SPEC>;
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
impl From<crate::W<ODR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ODR_SPEC>) -> Self {
        W(writer)
    }
}
///Port output data bit (y = 0..15)
pub type OD15_A = OD0_A;
///Field `OD15` reader - Port output data bit (y = 0..15)
pub type OD15_R = OD0_R;
///Field `OD15` writer - Port output data bit (y = 0..15)
pub struct OD15_W<'a> {
    w: &'a mut W,
}
impl<'a> OD15_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OD15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Set output to logic high
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OD15_A::HIGH)
    }
    ///Set output to logic low
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OD15_A::LOW)
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
///Port output data bit (y = 0..15)
pub type OD14_A = OD0_A;
///Field `OD14` reader - Port output data bit (y = 0..15)
pub type OD14_R = OD0_R;
///Field `OD14` writer - Port output data bit (y = 0..15)
pub struct OD14_W<'a> {
    w: &'a mut W,
}
impl<'a> OD14_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OD14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Set output to logic high
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OD14_A::HIGH)
    }
    ///Set output to logic low
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OD14_A::LOW)
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
///Port output data bit (y = 0..15)
pub type OD13_A = OD0_A;
///Field `OD13` reader - Port output data bit (y = 0..15)
pub type OD13_R = OD0_R;
///Field `OD13` writer - Port output data bit (y = 0..15)
pub struct OD13_W<'a> {
    w: &'a mut W,
}
impl<'a> OD13_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OD13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Set output to logic high
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OD13_A::HIGH)
    }
    ///Set output to logic low
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OD13_A::LOW)
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
///Port output data bit (y = 0..15)
pub type OD12_A = OD0_A;
///Field `OD12` reader - Port output data bit (y = 0..15)
pub type OD12_R = OD0_R;
///Field `OD12` writer - Port output data bit (y = 0..15)
pub struct OD12_W<'a> {
    w: &'a mut W,
}
impl<'a> OD12_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OD12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Set output to logic high
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OD12_A::HIGH)
    }
    ///Set output to logic low
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OD12_A::LOW)
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
///Port output data bit (y = 0..15)
pub type OD11_A = OD0_A;
///Field `OD11` reader - Port output data bit (y = 0..15)
pub type OD11_R = OD0_R;
///Field `OD11` writer - Port output data bit (y = 0..15)
pub struct OD11_W<'a> {
    w: &'a mut W,
}
impl<'a> OD11_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OD11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Set output to logic high
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OD11_A::HIGH)
    }
    ///Set output to logic low
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OD11_A::LOW)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
///Port output data bit (y = 0..15)
pub type OD10_A = OD0_A;
///Field `OD10` reader - Port output data bit (y = 0..15)
pub type OD10_R = OD0_R;
///Field `OD10` writer - Port output data bit (y = 0..15)
pub struct OD10_W<'a> {
    w: &'a mut W,
}
impl<'a> OD10_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OD10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Set output to logic high
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OD10_A::HIGH)
    }
    ///Set output to logic low
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OD10_A::LOW)
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
///Port output data bit (y = 0..15)
pub type OD9_A = OD0_A;
///Field `OD9` reader - Port output data bit (y = 0..15)
pub type OD9_R = OD0_R;
///Field `OD9` writer - Port output data bit (y = 0..15)
pub struct OD9_W<'a> {
    w: &'a mut W,
}
impl<'a> OD9_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OD9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Set output to logic high
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OD9_A::HIGH)
    }
    ///Set output to logic low
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OD9_A::LOW)
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
///Port output data bit (y = 0..15)
pub type OD8_A = OD0_A;
///Field `OD8` reader - Port output data bit (y = 0..15)
pub type OD8_R = OD0_R;
///Field `OD8` writer - Port output data bit (y = 0..15)
pub struct OD8_W<'a> {
    w: &'a mut W,
}
impl<'a> OD8_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OD8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Set output to logic high
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OD8_A::HIGH)
    }
    ///Set output to logic low
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OD8_A::LOW)
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
///Port output data bit (y = 0..15)
pub type OD7_A = OD0_A;
///Field `OD7` reader - Port output data bit (y = 0..15)
pub type OD7_R = OD0_R;
///Field `OD7` writer - Port output data bit (y = 0..15)
pub struct OD7_W<'a> {
    w: &'a mut W,
}
impl<'a> OD7_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OD7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Set output to logic high
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OD7_A::HIGH)
    }
    ///Set output to logic low
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OD7_A::LOW)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
///Port output data bit (y = 0..15)
pub type OD6_A = OD0_A;
///Field `OD6` reader - Port output data bit (y = 0..15)
pub type OD6_R = OD0_R;
///Field `OD6` writer - Port output data bit (y = 0..15)
pub struct OD6_W<'a> {
    w: &'a mut W,
}
impl<'a> OD6_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OD6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Set output to logic high
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OD6_A::HIGH)
    }
    ///Set output to logic low
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OD6_A::LOW)
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
///Port output data bit (y = 0..15)
pub type OD5_A = OD0_A;
///Field `OD5` reader - Port output data bit (y = 0..15)
pub type OD5_R = OD0_R;
///Field `OD5` writer - Port output data bit (y = 0..15)
pub struct OD5_W<'a> {
    w: &'a mut W,
}
impl<'a> OD5_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OD5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Set output to logic high
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OD5_A::HIGH)
    }
    ///Set output to logic low
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OD5_A::LOW)
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
///Port output data bit (y = 0..15)
pub type OD4_A = OD0_A;
///Field `OD4` reader - Port output data bit (y = 0..15)
pub type OD4_R = OD0_R;
///Field `OD4` writer - Port output data bit (y = 0..15)
pub struct OD4_W<'a> {
    w: &'a mut W,
}
impl<'a> OD4_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OD4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Set output to logic high
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OD4_A::HIGH)
    }
    ///Set output to logic low
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OD4_A::LOW)
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
///Port output data bit (y = 0..15)
pub type OD3_A = OD0_A;
///Field `OD3` reader - Port output data bit (y = 0..15)
pub type OD3_R = OD0_R;
///Field `OD3` writer - Port output data bit (y = 0..15)
pub struct OD3_W<'a> {
    w: &'a mut W,
}
impl<'a> OD3_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OD3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Set output to logic high
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OD3_A::HIGH)
    }
    ///Set output to logic low
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OD3_A::LOW)
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
///Port output data bit (y = 0..15)
pub type OD2_A = OD0_A;
///Field `OD2` reader - Port output data bit (y = 0..15)
pub type OD2_R = OD0_R;
///Field `OD2` writer - Port output data bit (y = 0..15)
pub struct OD2_W<'a> {
    w: &'a mut W,
}
impl<'a> OD2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OD2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Set output to logic high
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OD2_A::HIGH)
    }
    ///Set output to logic low
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OD2_A::LOW)
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
///Port output data bit (y = 0..15)
pub type OD1_A = OD0_A;
///Field `OD1` reader - Port output data bit (y = 0..15)
pub type OD1_R = OD0_R;
///Field `OD1` writer - Port output data bit (y = 0..15)
pub struct OD1_W<'a> {
    w: &'a mut W,
}
impl<'a> OD1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OD1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Set output to logic high
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OD1_A::HIGH)
    }
    ///Set output to logic low
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OD1_A::LOW)
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
///Port output data bit (y = 0..15)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OD0_A {
    ///1: Set output to logic high
    HIGH = 1,
    ///0: Set output to logic low
    LOW = 0,
}
impl From<OD0_A> for bool {
    #[inline(always)]
    fn from(variant: OD0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OD0` reader - Port output data bit (y = 0..15)
pub struct OD0_R(crate::FieldReader<bool, OD0_A>);
impl OD0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OD0_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OD0_A {
        match self.bits {
            true => OD0_A::HIGH,
            false => OD0_A::LOW,
        }
    }
    ///Checks if the value of the field is `HIGH`
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == OD0_A::HIGH
    }
    ///Checks if the value of the field is `LOW`
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == OD0_A::LOW
    }
}
impl core::ops::Deref for OD0_R {
    type Target = crate::FieldReader<bool, OD0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `OD0` writer - Port output data bit (y = 0..15)
pub struct OD0_W<'a> {
    w: &'a mut W,
}
impl<'a> OD0_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OD0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Set output to logic high
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OD0_A::HIGH)
    }
    ///Set output to logic low
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OD0_A::LOW)
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
    ///Bit 15 - Port output data bit (y = 0..15)
    #[inline(always)]
    pub fn od15(&self) -> OD15_R {
        OD15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 14 - Port output data bit (y = 0..15)
    #[inline(always)]
    pub fn od14(&self) -> OD14_R {
        OD14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 13 - Port output data bit (y = 0..15)
    #[inline(always)]
    pub fn od13(&self) -> OD13_R {
        OD13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 12 - Port output data bit (y = 0..15)
    #[inline(always)]
    pub fn od12(&self) -> OD12_R {
        OD12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 11 - Port output data bit (y = 0..15)
    #[inline(always)]
    pub fn od11(&self) -> OD11_R {
        OD11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - Port output data bit (y = 0..15)
    #[inline(always)]
    pub fn od10(&self) -> OD10_R {
        OD10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 9 - Port output data bit (y = 0..15)
    #[inline(always)]
    pub fn od9(&self) -> OD9_R {
        OD9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - Port output data bit (y = 0..15)
    #[inline(always)]
    pub fn od8(&self) -> OD8_R {
        OD8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 7 - Port output data bit (y = 0..15)
    #[inline(always)]
    pub fn od7(&self) -> OD7_R {
        OD7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 6 - Port output data bit (y = 0..15)
    #[inline(always)]
    pub fn od6(&self) -> OD6_R {
        OD6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - Port output data bit (y = 0..15)
    #[inline(always)]
    pub fn od5(&self) -> OD5_R {
        OD5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - Port output data bit (y = 0..15)
    #[inline(always)]
    pub fn od4(&self) -> OD4_R {
        OD4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - Port output data bit (y = 0..15)
    #[inline(always)]
    pub fn od3(&self) -> OD3_R {
        OD3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Port output data bit (y = 0..15)
    #[inline(always)]
    pub fn od2(&self) -> OD2_R {
        OD2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Port output data bit (y = 0..15)
    #[inline(always)]
    pub fn od1(&self) -> OD1_R {
        OD1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Port output data bit (y = 0..15)
    #[inline(always)]
    pub fn od0(&self) -> OD0_R {
        OD0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 15 - Port output data bit (y = 0..15)
    #[inline(always)]
    pub fn od15(&mut self) -> OD15_W {
        OD15_W { w: self }
    }
    ///Bit 14 - Port output data bit (y = 0..15)
    #[inline(always)]
    pub fn od14(&mut self) -> OD14_W {
        OD14_W { w: self }
    }
    ///Bit 13 - Port output data bit (y = 0..15)
    #[inline(always)]
    pub fn od13(&mut self) -> OD13_W {
        OD13_W { w: self }
    }
    ///Bit 12 - Port output data bit (y = 0..15)
    #[inline(always)]
    pub fn od12(&mut self) -> OD12_W {
        OD12_W { w: self }
    }
    ///Bit 11 - Port output data bit (y = 0..15)
    #[inline(always)]
    pub fn od11(&mut self) -> OD11_W {
        OD11_W { w: self }
    }
    ///Bit 10 - Port output data bit (y = 0..15)
    #[inline(always)]
    pub fn od10(&mut self) -> OD10_W {
        OD10_W { w: self }
    }
    ///Bit 9 - Port output data bit (y = 0..15)
    #[inline(always)]
    pub fn od9(&mut self) -> OD9_W {
        OD9_W { w: self }
    }
    ///Bit 8 - Port output data bit (y = 0..15)
    #[inline(always)]
    pub fn od8(&mut self) -> OD8_W {
        OD8_W { w: self }
    }
    ///Bit 7 - Port output data bit (y = 0..15)
    #[inline(always)]
    pub fn od7(&mut self) -> OD7_W {
        OD7_W { w: self }
    }
    ///Bit 6 - Port output data bit (y = 0..15)
    #[inline(always)]
    pub fn od6(&mut self) -> OD6_W {
        OD6_W { w: self }
    }
    ///Bit 5 - Port output data bit (y = 0..15)
    #[inline(always)]
    pub fn od5(&mut self) -> OD5_W {
        OD5_W { w: self }
    }
    ///Bit 4 - Port output data bit (y = 0..15)
    #[inline(always)]
    pub fn od4(&mut self) -> OD4_W {
        OD4_W { w: self }
    }
    ///Bit 3 - Port output data bit (y = 0..15)
    #[inline(always)]
    pub fn od3(&mut self) -> OD3_W {
        OD3_W { w: self }
    }
    ///Bit 2 - Port output data bit (y = 0..15)
    #[inline(always)]
    pub fn od2(&mut self) -> OD2_W {
        OD2_W { w: self }
    }
    ///Bit 1 - Port output data bit (y = 0..15)
    #[inline(always)]
    pub fn od1(&mut self) -> OD1_W {
        OD1_W { w: self }
    }
    ///Bit 0 - Port output data bit (y = 0..15)
    #[inline(always)]
    pub fn od0(&mut self) -> OD0_W {
        OD0_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GPIO port output data register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [odr](index.html) module
pub struct ODR_SPEC;
impl crate::RegisterSpec for ODR_SPEC {
    type Ux = u32;
}
///`read()` method returns [odr::R](R) reader structure
impl crate::Readable for ODR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [odr::W](W) writer structure
impl crate::Writable for ODR_SPEC {
    type Writer = W;
}
///`reset()` method sets ODR to value 0
impl crate::Resettable for ODR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
