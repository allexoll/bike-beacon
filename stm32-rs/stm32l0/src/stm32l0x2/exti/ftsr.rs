///Register `FTSR` reader
pub struct R(crate::R<FTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FTSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FTSR` writer
pub struct W(crate::W<FTSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FTSR_SPEC>;
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
impl From<crate::W<FTSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FTSR_SPEC>) -> Self {
        W(writer)
    }
}
///Falling trigger event configuration of line 0
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FT0_A {
    ///0: Falling edge trigger is disabled
    DISABLED = 0,
    ///1: Falling edge trigger is enabled
    ENABLED = 1,
}
impl From<FT0_A> for bool {
    #[inline(always)]
    fn from(variant: FT0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FT0` reader - Falling trigger event configuration of line 0
pub struct FT0_R(crate::FieldReader<bool, FT0_A>);
impl FT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FT0_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FT0_A {
        match self.bits {
            false => FT0_A::DISABLED,
            true => FT0_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FT0_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FT0_A::ENABLED
    }
}
impl core::ops::Deref for FT0_R {
    type Target = crate::FieldReader<bool, FT0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `FT0` writer - Falling trigger event configuration of line 0
pub struct FT0_W<'a> {
    w: &'a mut W,
}
impl<'a> FT0_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FT0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Falling edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT0_A::DISABLED)
    }
    ///Falling edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT0_A::ENABLED)
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
///Falling trigger event configuration of line 1
pub type FT1_A = FT0_A;
///Field `FT1` reader - Falling trigger event configuration of line 1
pub type FT1_R = FT0_R;
///Field `FT1` writer - Falling trigger event configuration of line 1
pub struct FT1_W<'a> {
    w: &'a mut W,
}
impl<'a> FT1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FT1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Falling edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT1_A::DISABLED)
    }
    ///Falling edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT1_A::ENABLED)
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
///Falling trigger event configuration of line 2
pub type FT2_A = FT0_A;
///Field `FT2` reader - Falling trigger event configuration of line 2
pub type FT2_R = FT0_R;
///Field `FT2` writer - Falling trigger event configuration of line 2
pub struct FT2_W<'a> {
    w: &'a mut W,
}
impl<'a> FT2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FT2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Falling edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT2_A::DISABLED)
    }
    ///Falling edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT2_A::ENABLED)
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
///Falling trigger event configuration of line 3
pub type FT3_A = FT0_A;
///Field `FT3` reader - Falling trigger event configuration of line 3
pub type FT3_R = FT0_R;
///Field `FT3` writer - Falling trigger event configuration of line 3
pub struct FT3_W<'a> {
    w: &'a mut W,
}
impl<'a> FT3_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FT3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Falling edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT3_A::DISABLED)
    }
    ///Falling edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT3_A::ENABLED)
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
///Falling trigger event configuration of line 4
pub type FT4_A = FT0_A;
///Field `FT4` reader - Falling trigger event configuration of line 4
pub type FT4_R = FT0_R;
///Field `FT4` writer - Falling trigger event configuration of line 4
pub struct FT4_W<'a> {
    w: &'a mut W,
}
impl<'a> FT4_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FT4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Falling edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT4_A::DISABLED)
    }
    ///Falling edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT4_A::ENABLED)
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
///Falling trigger event configuration of line 5
pub type FT5_A = FT0_A;
///Field `FT5` reader - Falling trigger event configuration of line 5
pub type FT5_R = FT0_R;
///Field `FT5` writer - Falling trigger event configuration of line 5
pub struct FT5_W<'a> {
    w: &'a mut W,
}
impl<'a> FT5_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FT5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Falling edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT5_A::DISABLED)
    }
    ///Falling edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT5_A::ENABLED)
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
///Falling trigger event configuration of line 6
pub type FT6_A = FT0_A;
///Field `FT6` reader - Falling trigger event configuration of line 6
pub type FT6_R = FT0_R;
///Field `FT6` writer - Falling trigger event configuration of line 6
pub struct FT6_W<'a> {
    w: &'a mut W,
}
impl<'a> FT6_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FT6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Falling edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT6_A::DISABLED)
    }
    ///Falling edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT6_A::ENABLED)
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
///Falling trigger event configuration of line 7
pub type FT7_A = FT0_A;
///Field `FT7` reader - Falling trigger event configuration of line 7
pub type FT7_R = FT0_R;
///Field `FT7` writer - Falling trigger event configuration of line 7
pub struct FT7_W<'a> {
    w: &'a mut W,
}
impl<'a> FT7_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FT7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Falling edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT7_A::DISABLED)
    }
    ///Falling edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT7_A::ENABLED)
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
///Falling trigger event configuration of line 8
pub type FT8_A = FT0_A;
///Field `FT8` reader - Falling trigger event configuration of line 8
pub type FT8_R = FT0_R;
///Field `FT8` writer - Falling trigger event configuration of line 8
pub struct FT8_W<'a> {
    w: &'a mut W,
}
impl<'a> FT8_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FT8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Falling edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT8_A::DISABLED)
    }
    ///Falling edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT8_A::ENABLED)
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
///Falling trigger event configuration of line 9
pub type FT9_A = FT0_A;
///Field `FT9` reader - Falling trigger event configuration of line 9
pub type FT9_R = FT0_R;
///Field `FT9` writer - Falling trigger event configuration of line 9
pub struct FT9_W<'a> {
    w: &'a mut W,
}
impl<'a> FT9_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FT9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Falling edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT9_A::DISABLED)
    }
    ///Falling edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT9_A::ENABLED)
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
///Falling trigger event configuration of line 10
pub type FT10_A = FT0_A;
///Field `FT10` reader - Falling trigger event configuration of line 10
pub type FT10_R = FT0_R;
///Field `FT10` writer - Falling trigger event configuration of line 10
pub struct FT10_W<'a> {
    w: &'a mut W,
}
impl<'a> FT10_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FT10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Falling edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT10_A::DISABLED)
    }
    ///Falling edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT10_A::ENABLED)
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
///Falling trigger event configuration of line 11
pub type FT11_A = FT0_A;
///Field `FT11` reader - Falling trigger event configuration of line 11
pub type FT11_R = FT0_R;
///Field `FT11` writer - Falling trigger event configuration of line 11
pub struct FT11_W<'a> {
    w: &'a mut W,
}
impl<'a> FT11_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FT11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Falling edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT11_A::DISABLED)
    }
    ///Falling edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT11_A::ENABLED)
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
///Falling trigger event configuration of line 12
pub type FT12_A = FT0_A;
///Field `FT12` reader - Falling trigger event configuration of line 12
pub type FT12_R = FT0_R;
///Field `FT12` writer - Falling trigger event configuration of line 12
pub struct FT12_W<'a> {
    w: &'a mut W,
}
impl<'a> FT12_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FT12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Falling edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT12_A::DISABLED)
    }
    ///Falling edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT12_A::ENABLED)
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
///Falling trigger event configuration of line 13
pub type FT13_A = FT0_A;
///Field `FT13` reader - Falling trigger event configuration of line 13
pub type FT13_R = FT0_R;
///Field `FT13` writer - Falling trigger event configuration of line 13
pub struct FT13_W<'a> {
    w: &'a mut W,
}
impl<'a> FT13_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FT13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Falling edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT13_A::DISABLED)
    }
    ///Falling edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT13_A::ENABLED)
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
///Falling trigger event configuration of line 14
pub type FT14_A = FT0_A;
///Field `FT14` reader - Falling trigger event configuration of line 14
pub type FT14_R = FT0_R;
///Field `FT14` writer - Falling trigger event configuration of line 14
pub struct FT14_W<'a> {
    w: &'a mut W,
}
impl<'a> FT14_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FT14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Falling edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT14_A::DISABLED)
    }
    ///Falling edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT14_A::ENABLED)
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
///Falling trigger event configuration of line 15
pub type FT15_A = FT0_A;
///Field `FT15` reader - Falling trigger event configuration of line 15
pub type FT15_R = FT0_R;
///Field `FT15` writer - Falling trigger event configuration of line 15
pub struct FT15_W<'a> {
    w: &'a mut W,
}
impl<'a> FT15_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FT15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Falling edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT15_A::DISABLED)
    }
    ///Falling edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT15_A::ENABLED)
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
///Falling trigger event configuration of line 16
pub type FT16_A = FT0_A;
///Field `FT16` reader - Falling trigger event configuration of line 16
pub type FT16_R = FT0_R;
///Field `FT16` writer - Falling trigger event configuration of line 16
pub struct FT16_W<'a> {
    w: &'a mut W,
}
impl<'a> FT16_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FT16_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Falling edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT16_A::DISABLED)
    }
    ///Falling edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT16_A::ENABLED)
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
///Falling trigger event configuration of line 17
pub type FT17_A = FT0_A;
///Field `FT17` reader - Falling trigger event configuration of line 17
pub type FT17_R = FT0_R;
///Field `FT17` writer - Falling trigger event configuration of line 17
pub struct FT17_W<'a> {
    w: &'a mut W,
}
impl<'a> FT17_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FT17_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Falling edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT17_A::DISABLED)
    }
    ///Falling edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT17_A::ENABLED)
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
///Falling trigger event configuration of line 19
pub type FT19_A = FT0_A;
///Field `FT19` reader - Falling trigger event configuration of line 19
pub type FT19_R = FT0_R;
///Field `FT19` writer - Falling trigger event configuration of line 19
pub struct FT19_W<'a> {
    w: &'a mut W,
}
impl<'a> FT19_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FT19_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Falling edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT19_A::DISABLED)
    }
    ///Falling edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT19_A::ENABLED)
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
///Falling trigger event configuration of line 20
pub type FT20_A = FT0_A;
///Field `FT20` reader - Falling trigger event configuration of line 20
pub type FT20_R = FT0_R;
///Field `FT20` writer - Falling trigger event configuration of line 20
pub struct FT20_W<'a> {
    w: &'a mut W,
}
impl<'a> FT20_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FT20_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Falling edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT20_A::DISABLED)
    }
    ///Falling edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT20_A::ENABLED)
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
///Falling trigger event configuration of line 21
pub type FT21_A = FT0_A;
///Field `FT21` reader - Falling trigger event configuration of line 21
pub type FT21_R = FT0_R;
///Field `FT21` writer - Falling trigger event configuration of line 21
pub struct FT21_W<'a> {
    w: &'a mut W,
}
impl<'a> FT21_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FT21_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Falling edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT21_A::DISABLED)
    }
    ///Falling edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT21_A::ENABLED)
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
///Falling trigger event configuration of line 22
pub type FT22_A = FT0_A;
///Field `FT22` reader - Falling trigger event configuration of line 22
pub type FT22_R = FT0_R;
///Field `FT22` writer - Falling trigger event configuration of line 22
pub struct FT22_W<'a> {
    w: &'a mut W,
}
impl<'a> FT22_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FT22_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Falling edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT22_A::DISABLED)
    }
    ///Falling edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT22_A::ENABLED)
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
    ///Bit 0 - Falling trigger event configuration of line 0
    #[inline(always)]
    pub fn ft0(&self) -> FT0_R {
        FT0_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Falling trigger event configuration of line 1
    #[inline(always)]
    pub fn ft1(&self) -> FT1_R {
        FT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Falling trigger event configuration of line 2
    #[inline(always)]
    pub fn ft2(&self) -> FT2_R {
        FT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Falling trigger event configuration of line 3
    #[inline(always)]
    pub fn ft3(&self) -> FT3_R {
        FT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Falling trigger event configuration of line 4
    #[inline(always)]
    pub fn ft4(&self) -> FT4_R {
        FT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - Falling trigger event configuration of line 5
    #[inline(always)]
    pub fn ft5(&self) -> FT5_R {
        FT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - Falling trigger event configuration of line 6
    #[inline(always)]
    pub fn ft6(&self) -> FT6_R {
        FT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Falling trigger event configuration of line 7
    #[inline(always)]
    pub fn ft7(&self) -> FT7_R {
        FT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - Falling trigger event configuration of line 8
    #[inline(always)]
    pub fn ft8(&self) -> FT8_R {
        FT8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - Falling trigger event configuration of line 9
    #[inline(always)]
    pub fn ft9(&self) -> FT9_R {
        FT9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - Falling trigger event configuration of line 10
    #[inline(always)]
    pub fn ft10(&self) -> FT10_R {
        FT10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - Falling trigger event configuration of line 11
    #[inline(always)]
    pub fn ft11(&self) -> FT11_R {
        FT11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - Falling trigger event configuration of line 12
    #[inline(always)]
    pub fn ft12(&self) -> FT12_R {
        FT12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - Falling trigger event configuration of line 13
    #[inline(always)]
    pub fn ft13(&self) -> FT13_R {
        FT13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - Falling trigger event configuration of line 14
    #[inline(always)]
    pub fn ft14(&self) -> FT14_R {
        FT14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - Falling trigger event configuration of line 15
    #[inline(always)]
    pub fn ft15(&self) -> FT15_R {
        FT15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 16 - Falling trigger event configuration of line 16
    #[inline(always)]
    pub fn ft16(&self) -> FT16_R {
        FT16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - Falling trigger event configuration of line 17
    #[inline(always)]
    pub fn ft17(&self) -> FT17_R {
        FT17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 19 - Falling trigger event configuration of line 19
    #[inline(always)]
    pub fn ft19(&self) -> FT19_R {
        FT19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 20 - Falling trigger event configuration of line 20
    #[inline(always)]
    pub fn ft20(&self) -> FT20_R {
        FT20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 21 - Falling trigger event configuration of line 21
    #[inline(always)]
    pub fn ft21(&self) -> FT21_R {
        FT21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 22 - Falling trigger event configuration of line 22
    #[inline(always)]
    pub fn ft22(&self) -> FT22_R {
        FT22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Falling trigger event configuration of line 0
    #[inline(always)]
    pub fn ft0(&mut self) -> FT0_W {
        FT0_W { w: self }
    }
    ///Bit 1 - Falling trigger event configuration of line 1
    #[inline(always)]
    pub fn ft1(&mut self) -> FT1_W {
        FT1_W { w: self }
    }
    ///Bit 2 - Falling trigger event configuration of line 2
    #[inline(always)]
    pub fn ft2(&mut self) -> FT2_W {
        FT2_W { w: self }
    }
    ///Bit 3 - Falling trigger event configuration of line 3
    #[inline(always)]
    pub fn ft3(&mut self) -> FT3_W {
        FT3_W { w: self }
    }
    ///Bit 4 - Falling trigger event configuration of line 4
    #[inline(always)]
    pub fn ft4(&mut self) -> FT4_W {
        FT4_W { w: self }
    }
    ///Bit 5 - Falling trigger event configuration of line 5
    #[inline(always)]
    pub fn ft5(&mut self) -> FT5_W {
        FT5_W { w: self }
    }
    ///Bit 6 - Falling trigger event configuration of line 6
    #[inline(always)]
    pub fn ft6(&mut self) -> FT6_W {
        FT6_W { w: self }
    }
    ///Bit 7 - Falling trigger event configuration of line 7
    #[inline(always)]
    pub fn ft7(&mut self) -> FT7_W {
        FT7_W { w: self }
    }
    ///Bit 8 - Falling trigger event configuration of line 8
    #[inline(always)]
    pub fn ft8(&mut self) -> FT8_W {
        FT8_W { w: self }
    }
    ///Bit 9 - Falling trigger event configuration of line 9
    #[inline(always)]
    pub fn ft9(&mut self) -> FT9_W {
        FT9_W { w: self }
    }
    ///Bit 10 - Falling trigger event configuration of line 10
    #[inline(always)]
    pub fn ft10(&mut self) -> FT10_W {
        FT10_W { w: self }
    }
    ///Bit 11 - Falling trigger event configuration of line 11
    #[inline(always)]
    pub fn ft11(&mut self) -> FT11_W {
        FT11_W { w: self }
    }
    ///Bit 12 - Falling trigger event configuration of line 12
    #[inline(always)]
    pub fn ft12(&mut self) -> FT12_W {
        FT12_W { w: self }
    }
    ///Bit 13 - Falling trigger event configuration of line 13
    #[inline(always)]
    pub fn ft13(&mut self) -> FT13_W {
        FT13_W { w: self }
    }
    ///Bit 14 - Falling trigger event configuration of line 14
    #[inline(always)]
    pub fn ft14(&mut self) -> FT14_W {
        FT14_W { w: self }
    }
    ///Bit 15 - Falling trigger event configuration of line 15
    #[inline(always)]
    pub fn ft15(&mut self) -> FT15_W {
        FT15_W { w: self }
    }
    ///Bit 16 - Falling trigger event configuration of line 16
    #[inline(always)]
    pub fn ft16(&mut self) -> FT16_W {
        FT16_W { w: self }
    }
    ///Bit 17 - Falling trigger event configuration of line 17
    #[inline(always)]
    pub fn ft17(&mut self) -> FT17_W {
        FT17_W { w: self }
    }
    ///Bit 19 - Falling trigger event configuration of line 19
    #[inline(always)]
    pub fn ft19(&mut self) -> FT19_W {
        FT19_W { w: self }
    }
    ///Bit 20 - Falling trigger event configuration of line 20
    #[inline(always)]
    pub fn ft20(&mut self) -> FT20_W {
        FT20_W { w: self }
    }
    ///Bit 21 - Falling trigger event configuration of line 21
    #[inline(always)]
    pub fn ft21(&mut self) -> FT21_W {
        FT21_W { w: self }
    }
    ///Bit 22 - Falling trigger event configuration of line 22
    #[inline(always)]
    pub fn ft22(&mut self) -> FT22_W {
        FT22_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Falling Trigger selection register (EXTI_FTSR)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ftsr](index.html) module
pub struct FTSR_SPEC;
impl crate::RegisterSpec for FTSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ftsr::R](R) reader structure
impl crate::Readable for FTSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ftsr::W](W) writer structure
impl crate::Writable for FTSR_SPEC {
    type Writer = W;
}
///`reset()` method sets FTSR to value 0
impl crate::Resettable for FTSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
