///Register `SWIER` reader
pub struct R(crate::R<SWIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWIER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SWIER` writer
pub struct W(crate::W<SWIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWIER_SPEC>;
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
impl From<crate::W<SWIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWIER_SPEC>) -> Self {
        W(writer)
    }
}
///Software Interrupt on line 0
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWI0_A {
    ///1: Generates an interrupt request
    PEND = 1,
}
impl From<SWI0_A> for bool {
    #[inline(always)]
    fn from(variant: SWI0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SWI0` reader - Software Interrupt on line 0
pub struct SWI0_R(crate::FieldReader<bool, SWI0_A>);
impl SWI0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWI0_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<SWI0_A> {
        match self.bits {
            true => Some(SWI0_A::PEND),
            _ => None,
        }
    }
    ///Checks if the value of the field is `PEND`
    #[inline(always)]
    pub fn is_pend(&self) -> bool {
        **self == SWI0_A::PEND
    }
}
impl core::ops::Deref for SWI0_R {
    type Target = crate::FieldReader<bool, SWI0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SWI0` writer - Software Interrupt on line 0
pub struct SWI0_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI0_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWI0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWI0_A::PEND)
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
///Software Interrupt on line 1
pub type SWI1_A = SWI0_A;
///Field `SWI1` reader - Software Interrupt on line 1
pub type SWI1_R = SWI0_R;
///Field `SWI1` writer - Software Interrupt on line 1
pub struct SWI1_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWI1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWI1_A::PEND)
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
///Software Interrupt on line 2
pub type SWI2_A = SWI0_A;
///Field `SWI2` reader - Software Interrupt on line 2
pub type SWI2_R = SWI0_R;
///Field `SWI2` writer - Software Interrupt on line 2
pub struct SWI2_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWI2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWI2_A::PEND)
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
///Software Interrupt on line 3
pub type SWI3_A = SWI0_A;
///Field `SWI3` reader - Software Interrupt on line 3
pub type SWI3_R = SWI0_R;
///Field `SWI3` writer - Software Interrupt on line 3
pub struct SWI3_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI3_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWI3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWI3_A::PEND)
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
///Software Interrupt on line 4
pub type SWI4_A = SWI0_A;
///Field `SWI4` reader - Software Interrupt on line 4
pub type SWI4_R = SWI0_R;
///Field `SWI4` writer - Software Interrupt on line 4
pub struct SWI4_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI4_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWI4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWI4_A::PEND)
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
///Software Interrupt on line 5
pub type SWI5_A = SWI0_A;
///Field `SWI5` reader - Software Interrupt on line 5
pub type SWI5_R = SWI0_R;
///Field `SWI5` writer - Software Interrupt on line 5
pub struct SWI5_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI5_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWI5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWI5_A::PEND)
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
///Software Interrupt on line 6
pub type SWI6_A = SWI0_A;
///Field `SWI6` reader - Software Interrupt on line 6
pub type SWI6_R = SWI0_R;
///Field `SWI6` writer - Software Interrupt on line 6
pub struct SWI6_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI6_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWI6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWI6_A::PEND)
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
///Software Interrupt on line 7
pub type SWI7_A = SWI0_A;
///Field `SWI7` reader - Software Interrupt on line 7
pub type SWI7_R = SWI0_R;
///Field `SWI7` writer - Software Interrupt on line 7
pub struct SWI7_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI7_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWI7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWI7_A::PEND)
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
///Software Interrupt on line 8
pub type SWI8_A = SWI0_A;
///Field `SWI8` reader - Software Interrupt on line 8
pub type SWI8_R = SWI0_R;
///Field `SWI8` writer - Software Interrupt on line 8
pub struct SWI8_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI8_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWI8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWI8_A::PEND)
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
///Software Interrupt on line 9
pub type SWI9_A = SWI0_A;
///Field `SWI9` reader - Software Interrupt on line 9
pub type SWI9_R = SWI0_R;
///Field `SWI9` writer - Software Interrupt on line 9
pub struct SWI9_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI9_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWI9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWI9_A::PEND)
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
///Software Interrupt on line 10
pub type SWI10_A = SWI0_A;
///Field `SWI10` reader - Software Interrupt on line 10
pub type SWI10_R = SWI0_R;
///Field `SWI10` writer - Software Interrupt on line 10
pub struct SWI10_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI10_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWI10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWI10_A::PEND)
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
///Software Interrupt on line 11
pub type SWI11_A = SWI0_A;
///Field `SWI11` reader - Software Interrupt on line 11
pub type SWI11_R = SWI0_R;
///Field `SWI11` writer - Software Interrupt on line 11
pub struct SWI11_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI11_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWI11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWI11_A::PEND)
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
///Software Interrupt on line 12
pub type SWI12_A = SWI0_A;
///Field `SWI12` reader - Software Interrupt on line 12
pub type SWI12_R = SWI0_R;
///Field `SWI12` writer - Software Interrupt on line 12
pub struct SWI12_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI12_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWI12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWI12_A::PEND)
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
///Software Interrupt on line 13
pub type SWI13_A = SWI0_A;
///Field `SWI13` reader - Software Interrupt on line 13
pub type SWI13_R = SWI0_R;
///Field `SWI13` writer - Software Interrupt on line 13
pub struct SWI13_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI13_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWI13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWI13_A::PEND)
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
///Software Interrupt on line 14
pub type SWI14_A = SWI0_A;
///Field `SWI14` reader - Software Interrupt on line 14
pub type SWI14_R = SWI0_R;
///Field `SWI14` writer - Software Interrupt on line 14
pub struct SWI14_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI14_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWI14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWI14_A::PEND)
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
///Software Interrupt on line 15
pub type SWI15_A = SWI0_A;
///Field `SWI15` reader - Software Interrupt on line 15
pub type SWI15_R = SWI0_R;
///Field `SWI15` writer - Software Interrupt on line 15
pub struct SWI15_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI15_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWI15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWI15_A::PEND)
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
///Software Interrupt on line 16
pub type SWI16_A = SWI0_A;
///Field `SWI16` reader - Software Interrupt on line 16
pub type SWI16_R = SWI0_R;
///Field `SWI16` writer - Software Interrupt on line 16
pub struct SWI16_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI16_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWI16_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWI16_A::PEND)
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
///Software Interrupt on line 17
pub type SWI17_A = SWI0_A;
///Field `SWI17` reader - Software Interrupt on line 17
pub type SWI17_R = SWI0_R;
///Field `SWI17` writer - Software Interrupt on line 17
pub struct SWI17_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI17_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWI17_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWI17_A::PEND)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
///Software Interrupt on line 19
pub type SWI19_A = SWI0_A;
///Field `SWI19` reader - Software Interrupt on line 19
pub type SWI19_R = SWI0_R;
///Field `SWI19` writer - Software Interrupt on line 19
pub struct SWI19_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI19_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWI19_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWI19_A::PEND)
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
///Software Interrupt on line 20
pub type SWI20_A = SWI0_A;
///Field `SWI20` reader - Software Interrupt on line 20
pub type SWI20_R = SWI0_R;
///Field `SWI20` writer - Software Interrupt on line 20
pub struct SWI20_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI20_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWI20_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWI20_A::PEND)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
///Software Interrupt on line 21
pub type SWI21_A = SWI0_A;
///Field `SWI21` reader - Software Interrupt on line 21
pub type SWI21_R = SWI0_R;
///Field `SWI21` writer - Software Interrupt on line 21
pub struct SWI21_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI21_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWI21_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWI21_A::PEND)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
///Software Interrupt on line 22
pub type SWI22_A = SWI0_A;
///Field `SWI22` reader - Software Interrupt on line 22
pub type SWI22_R = SWI0_R;
///Field `SWI22` writer - Software Interrupt on line 22
pub struct SWI22_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI22_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWI22_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWI22_A::PEND)
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
impl R {
    ///Bit 0 - Software Interrupt on line 0
    #[inline(always)]
    pub fn swi0(&self) -> SWI0_R {
        SWI0_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Software Interrupt on line 1
    #[inline(always)]
    pub fn swi1(&self) -> SWI1_R {
        SWI1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Software Interrupt on line 2
    #[inline(always)]
    pub fn swi2(&self) -> SWI2_R {
        SWI2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Software Interrupt on line 3
    #[inline(always)]
    pub fn swi3(&self) -> SWI3_R {
        SWI3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Software Interrupt on line 4
    #[inline(always)]
    pub fn swi4(&self) -> SWI4_R {
        SWI4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - Software Interrupt on line 5
    #[inline(always)]
    pub fn swi5(&self) -> SWI5_R {
        SWI5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - Software Interrupt on line 6
    #[inline(always)]
    pub fn swi6(&self) -> SWI6_R {
        SWI6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Software Interrupt on line 7
    #[inline(always)]
    pub fn swi7(&self) -> SWI7_R {
        SWI7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - Software Interrupt on line 8
    #[inline(always)]
    pub fn swi8(&self) -> SWI8_R {
        SWI8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - Software Interrupt on line 9
    #[inline(always)]
    pub fn swi9(&self) -> SWI9_R {
        SWI9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - Software Interrupt on line 10
    #[inline(always)]
    pub fn swi10(&self) -> SWI10_R {
        SWI10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - Software Interrupt on line 11
    #[inline(always)]
    pub fn swi11(&self) -> SWI11_R {
        SWI11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - Software Interrupt on line 12
    #[inline(always)]
    pub fn swi12(&self) -> SWI12_R {
        SWI12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - Software Interrupt on line 13
    #[inline(always)]
    pub fn swi13(&self) -> SWI13_R {
        SWI13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - Software Interrupt on line 14
    #[inline(always)]
    pub fn swi14(&self) -> SWI14_R {
        SWI14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - Software Interrupt on line 15
    #[inline(always)]
    pub fn swi15(&self) -> SWI15_R {
        SWI15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 16 - Software Interrupt on line 16
    #[inline(always)]
    pub fn swi16(&self) -> SWI16_R {
        SWI16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - Software Interrupt on line 17
    #[inline(always)]
    pub fn swi17(&self) -> SWI17_R {
        SWI17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 19 - Software Interrupt on line 19
    #[inline(always)]
    pub fn swi19(&self) -> SWI19_R {
        SWI19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 20 - Software Interrupt on line 20
    #[inline(always)]
    pub fn swi20(&self) -> SWI20_R {
        SWI20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 21 - Software Interrupt on line 21
    #[inline(always)]
    pub fn swi21(&self) -> SWI21_R {
        SWI21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 22 - Software Interrupt on line 22
    #[inline(always)]
    pub fn swi22(&self) -> SWI22_R {
        SWI22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Software Interrupt on line 0
    #[inline(always)]
    pub fn swi0(&mut self) -> SWI0_W {
        SWI0_W { w: self }
    }
    ///Bit 1 - Software Interrupt on line 1
    #[inline(always)]
    pub fn swi1(&mut self) -> SWI1_W {
        SWI1_W { w: self }
    }
    ///Bit 2 - Software Interrupt on line 2
    #[inline(always)]
    pub fn swi2(&mut self) -> SWI2_W {
        SWI2_W { w: self }
    }
    ///Bit 3 - Software Interrupt on line 3
    #[inline(always)]
    pub fn swi3(&mut self) -> SWI3_W {
        SWI3_W { w: self }
    }
    ///Bit 4 - Software Interrupt on line 4
    #[inline(always)]
    pub fn swi4(&mut self) -> SWI4_W {
        SWI4_W { w: self }
    }
    ///Bit 5 - Software Interrupt on line 5
    #[inline(always)]
    pub fn swi5(&mut self) -> SWI5_W {
        SWI5_W { w: self }
    }
    ///Bit 6 - Software Interrupt on line 6
    #[inline(always)]
    pub fn swi6(&mut self) -> SWI6_W {
        SWI6_W { w: self }
    }
    ///Bit 7 - Software Interrupt on line 7
    #[inline(always)]
    pub fn swi7(&mut self) -> SWI7_W {
        SWI7_W { w: self }
    }
    ///Bit 8 - Software Interrupt on line 8
    #[inline(always)]
    pub fn swi8(&mut self) -> SWI8_W {
        SWI8_W { w: self }
    }
    ///Bit 9 - Software Interrupt on line 9
    #[inline(always)]
    pub fn swi9(&mut self) -> SWI9_W {
        SWI9_W { w: self }
    }
    ///Bit 10 - Software Interrupt on line 10
    #[inline(always)]
    pub fn swi10(&mut self) -> SWI10_W {
        SWI10_W { w: self }
    }
    ///Bit 11 - Software Interrupt on line 11
    #[inline(always)]
    pub fn swi11(&mut self) -> SWI11_W {
        SWI11_W { w: self }
    }
    ///Bit 12 - Software Interrupt on line 12
    #[inline(always)]
    pub fn swi12(&mut self) -> SWI12_W {
        SWI12_W { w: self }
    }
    ///Bit 13 - Software Interrupt on line 13
    #[inline(always)]
    pub fn swi13(&mut self) -> SWI13_W {
        SWI13_W { w: self }
    }
    ///Bit 14 - Software Interrupt on line 14
    #[inline(always)]
    pub fn swi14(&mut self) -> SWI14_W {
        SWI14_W { w: self }
    }
    ///Bit 15 - Software Interrupt on line 15
    #[inline(always)]
    pub fn swi15(&mut self) -> SWI15_W {
        SWI15_W { w: self }
    }
    ///Bit 16 - Software Interrupt on line 16
    #[inline(always)]
    pub fn swi16(&mut self) -> SWI16_W {
        SWI16_W { w: self }
    }
    ///Bit 17 - Software Interrupt on line 17
    #[inline(always)]
    pub fn swi17(&mut self) -> SWI17_W {
        SWI17_W { w: self }
    }
    ///Bit 19 - Software Interrupt on line 19
    #[inline(always)]
    pub fn swi19(&mut self) -> SWI19_W {
        SWI19_W { w: self }
    }
    ///Bit 20 - Software Interrupt on line 20
    #[inline(always)]
    pub fn swi20(&mut self) -> SWI20_W {
        SWI20_W { w: self }
    }
    ///Bit 21 - Software Interrupt on line 21
    #[inline(always)]
    pub fn swi21(&mut self) -> SWI21_W {
        SWI21_W { w: self }
    }
    ///Bit 22 - Software Interrupt on line 22
    #[inline(always)]
    pub fn swi22(&mut self) -> SWI22_W {
        SWI22_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Software interrupt event register (EXTI_SWIER)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [swier](index.html) module
pub struct SWIER_SPEC;
impl crate::RegisterSpec for SWIER_SPEC {
    type Ux = u32;
}
///`read()` method returns [swier::R](R) reader structure
impl crate::Readable for SWIER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [swier::W](W) writer structure
impl crate::Writable for SWIER_SPEC {
    type Writer = W;
}
///`reset()` method sets SWIER to value 0
impl crate::Resettable for SWIER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}