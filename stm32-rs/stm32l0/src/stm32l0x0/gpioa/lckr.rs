///Register `LCKR` reader
pub struct R(crate::R<LCKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCKR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `LCKR` writer
pub struct W(crate::W<LCKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCKR_SPEC>;
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
impl From<crate::W<LCKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCKR_SPEC>) -> Self {
        W(writer)
    }
}
///Port x lock bit y (y= 0..15)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCKK_A {
    ///0: Port configuration lock key not active
    NOTACTIVE = 0,
    ///1: Port configuration lock key active
    ACTIVE = 1,
}
impl From<LCKK_A> for bool {
    #[inline(always)]
    fn from(variant: LCKK_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LCKK` reader - Port x lock bit y (y= 0..15)
pub struct LCKK_R(crate::FieldReader<bool, LCKK_A>);
impl LCKK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCKK_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LCKK_A {
        match self.bits {
            false => LCKK_A::NOTACTIVE,
            true => LCKK_A::ACTIVE,
        }
    }
    ///Checks if the value of the field is `NOTACTIVE`
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        **self == LCKK_A::NOTACTIVE
    }
    ///Checks if the value of the field is `ACTIVE`
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == LCKK_A::ACTIVE
    }
}
impl core::ops::Deref for LCKK_R {
    type Target = crate::FieldReader<bool, LCKK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LCKK` writer - Port x lock bit y (y= 0..15)
pub struct LCKK_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKK_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LCKK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Port configuration lock key not active
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(LCKK_A::NOTACTIVE)
    }
    ///Port configuration lock key active
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(LCKK_A::ACTIVE)
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
///Port x lock bit y (y= 0..15)
pub type LCK15_A = LCK10_A;
///Field `LCK15` reader - Port x lock bit y (y= 0..15)
pub type LCK15_R = LCK10_R;
///Field `LCK15` writer - Port x lock bit y (y= 0..15)
pub struct LCK15_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK15_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LCK15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Port configuration not locked
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LCK15_A::UNLOCKED)
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LCK15_A::LOCKED)
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
///Port x lock bit y (y= 0..15)
pub type LCK14_A = LCK10_A;
///Field `LCK14` reader - Port x lock bit y (y= 0..15)
pub type LCK14_R = LCK10_R;
///Field `LCK14` writer - Port x lock bit y (y= 0..15)
pub struct LCK14_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK14_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LCK14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Port configuration not locked
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LCK14_A::UNLOCKED)
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LCK14_A::LOCKED)
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
///Port x lock bit y (y= 0..15)
pub type LCK13_A = LCK10_A;
///Field `LCK13` reader - Port x lock bit y (y= 0..15)
pub type LCK13_R = LCK10_R;
///Field `LCK13` writer - Port x lock bit y (y= 0..15)
pub struct LCK13_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK13_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LCK13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Port configuration not locked
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LCK13_A::UNLOCKED)
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LCK13_A::LOCKED)
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
///Port x lock bit y (y= 0..15)
pub type LCK12_A = LCK10_A;
///Field `LCK12` reader - Port x lock bit y (y= 0..15)
pub type LCK12_R = LCK10_R;
///Field `LCK12` writer - Port x lock bit y (y= 0..15)
pub struct LCK12_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK12_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LCK12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Port configuration not locked
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LCK12_A::UNLOCKED)
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LCK12_A::LOCKED)
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
///Port x lock bit y (y= 0..15)
pub type LCK11_A = LCK10_A;
///Field `LCK11` reader - Port x lock bit y (y= 0..15)
pub type LCK11_R = LCK10_R;
///Field `LCK11` writer - Port x lock bit y (y= 0..15)
pub struct LCK11_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK11_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LCK11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Port configuration not locked
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LCK11_A::UNLOCKED)
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LCK11_A::LOCKED)
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
///Port x lock bit y (y= 0..15)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCK10_A {
    ///0: Port configuration not locked
    UNLOCKED = 0,
    ///1: Port configuration locked
    LOCKED = 1,
}
impl From<LCK10_A> for bool {
    #[inline(always)]
    fn from(variant: LCK10_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LCK10` reader - Port x lock bit y (y= 0..15)
pub struct LCK10_R(crate::FieldReader<bool, LCK10_A>);
impl LCK10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCK10_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LCK10_A {
        match self.bits {
            false => LCK10_A::UNLOCKED,
            true => LCK10_A::LOCKED,
        }
    }
    ///Checks if the value of the field is `UNLOCKED`
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        **self == LCK10_A::UNLOCKED
    }
    ///Checks if the value of the field is `LOCKED`
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        **self == LCK10_A::LOCKED
    }
}
impl core::ops::Deref for LCK10_R {
    type Target = crate::FieldReader<bool, LCK10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LCK10` writer - Port x lock bit y (y= 0..15)
pub struct LCK10_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK10_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LCK10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Port configuration not locked
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LCK10_A::UNLOCKED)
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LCK10_A::LOCKED)
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
///Port x lock bit y (y= 0..15)
pub type LCK9_A = LCK0_A;
///Field `LCK9` reader - Port x lock bit y (y= 0..15)
pub type LCK9_R = LCK0_R;
///Field `LCK9` writer - Port x lock bit y (y= 0..15)
pub struct LCK9_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK9_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LCK9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Port configuration not locked
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LCK9_A::UNLOCKED)
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LCK9_A::LOCKED)
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
///Port x lock bit y (y= 0..15)
pub type LCK8_A = LCK0_A;
///Field `LCK8` reader - Port x lock bit y (y= 0..15)
pub type LCK8_R = LCK0_R;
///Field `LCK8` writer - Port x lock bit y (y= 0..15)
pub struct LCK8_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK8_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LCK8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Port configuration not locked
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LCK8_A::UNLOCKED)
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LCK8_A::LOCKED)
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
///Port x lock bit y (y= 0..15)
pub type LCK7_A = LCK0_A;
///Field `LCK7` reader - Port x lock bit y (y= 0..15)
pub type LCK7_R = LCK0_R;
///Field `LCK7` writer - Port x lock bit y (y= 0..15)
pub struct LCK7_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK7_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LCK7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Port configuration not locked
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LCK7_A::UNLOCKED)
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LCK7_A::LOCKED)
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
///Port x lock bit y (y= 0..15)
pub type LCK6_A = LCK0_A;
///Field `LCK6` reader - Port x lock bit y (y= 0..15)
pub type LCK6_R = LCK0_R;
///Field `LCK6` writer - Port x lock bit y (y= 0..15)
pub struct LCK6_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK6_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LCK6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Port configuration not locked
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LCK6_A::UNLOCKED)
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LCK6_A::LOCKED)
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
///Port x lock bit y (y= 0..15)
pub type LCK5_A = LCK0_A;
///Field `LCK5` reader - Port x lock bit y (y= 0..15)
pub type LCK5_R = LCK0_R;
///Field `LCK5` writer - Port x lock bit y (y= 0..15)
pub struct LCK5_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK5_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LCK5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Port configuration not locked
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LCK5_A::UNLOCKED)
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LCK5_A::LOCKED)
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
///Port x lock bit y (y= 0..15)
pub type LCK4_A = LCK0_A;
///Field `LCK4` reader - Port x lock bit y (y= 0..15)
pub type LCK4_R = LCK0_R;
///Field `LCK4` writer - Port x lock bit y (y= 0..15)
pub struct LCK4_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK4_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LCK4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Port configuration not locked
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LCK4_A::UNLOCKED)
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LCK4_A::LOCKED)
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
///Port x lock bit y (y= 0..15)
pub type LCK3_A = LCK0_A;
///Field `LCK3` reader - Port x lock bit y (y= 0..15)
pub type LCK3_R = LCK0_R;
///Field `LCK3` writer - Port x lock bit y (y= 0..15)
pub struct LCK3_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK3_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LCK3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Port configuration not locked
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LCK3_A::UNLOCKED)
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LCK3_A::LOCKED)
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
///Port x lock bit y (y= 0..15)
pub type LCK2_A = LCK0_A;
///Field `LCK2` reader - Port x lock bit y (y= 0..15)
pub type LCK2_R = LCK0_R;
///Field `LCK2` writer - Port x lock bit y (y= 0..15)
pub struct LCK2_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LCK2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Port configuration not locked
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LCK2_A::UNLOCKED)
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LCK2_A::LOCKED)
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
///Port x lock bit y (y= 0..15)
pub type LCK1_A = LCK0_A;
///Field `LCK1` reader - Port x lock bit y (y= 0..15)
pub type LCK1_R = LCK0_R;
///Field `LCK1` writer - Port x lock bit y (y= 0..15)
pub struct LCK1_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LCK1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Port configuration not locked
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LCK1_A::UNLOCKED)
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LCK1_A::LOCKED)
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
///Port x lock bit y (y= 0..15)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCK0_A {
    ///0: Port configuration not locked
    UNLOCKED = 0,
    ///1: Port configuration locked
    LOCKED = 1,
}
impl From<LCK0_A> for bool {
    #[inline(always)]
    fn from(variant: LCK0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LCK0` reader - Port x lock bit y (y= 0..15)
pub struct LCK0_R(crate::FieldReader<bool, LCK0_A>);
impl LCK0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LCK0_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LCK0_A {
        match self.bits {
            false => LCK0_A::UNLOCKED,
            true => LCK0_A::LOCKED,
        }
    }
    ///Checks if the value of the field is `UNLOCKED`
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        **self == LCK0_A::UNLOCKED
    }
    ///Checks if the value of the field is `LOCKED`
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        **self == LCK0_A::LOCKED
    }
}
impl core::ops::Deref for LCK0_R {
    type Target = crate::FieldReader<bool, LCK0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LCK0` writer - Port x lock bit y (y= 0..15)
pub struct LCK0_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK0_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LCK0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Port configuration not locked
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LCK0_A::UNLOCKED)
    }
    ///Port configuration locked
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LCK0_A::LOCKED)
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
    ///Bit 16 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lckk(&self) -> LCKK_R {
        LCKK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 15 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck15(&self) -> LCK15_R {
        LCK15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 14 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck14(&self) -> LCK14_R {
        LCK14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 13 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck13(&self) -> LCK13_R {
        LCK13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 12 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck12(&self) -> LCK12_R {
        LCK12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 11 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck11(&self) -> LCK11_R {
        LCK11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck10(&self) -> LCK10_R {
        LCK10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 9 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck9(&self) -> LCK9_R {
        LCK9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck8(&self) -> LCK8_R {
        LCK8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 7 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck7(&self) -> LCK7_R {
        LCK7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 6 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck6(&self) -> LCK6_R {
        LCK6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck5(&self) -> LCK5_R {
        LCK5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck4(&self) -> LCK4_R {
        LCK4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck3(&self) -> LCK3_R {
        LCK3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck2(&self) -> LCK2_R {
        LCK2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck1(&self) -> LCK1_R {
        LCK1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck0(&self) -> LCK0_R {
        LCK0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 16 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lckk(&mut self) -> LCKK_W {
        LCKK_W { w: self }
    }
    ///Bit 15 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck15(&mut self) -> LCK15_W {
        LCK15_W { w: self }
    }
    ///Bit 14 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck14(&mut self) -> LCK14_W {
        LCK14_W { w: self }
    }
    ///Bit 13 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck13(&mut self) -> LCK13_W {
        LCK13_W { w: self }
    }
    ///Bit 12 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck12(&mut self) -> LCK12_W {
        LCK12_W { w: self }
    }
    ///Bit 11 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck11(&mut self) -> LCK11_W {
        LCK11_W { w: self }
    }
    ///Bit 10 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck10(&mut self) -> LCK10_W {
        LCK10_W { w: self }
    }
    ///Bit 9 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck9(&mut self) -> LCK9_W {
        LCK9_W { w: self }
    }
    ///Bit 8 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck8(&mut self) -> LCK8_W {
        LCK8_W { w: self }
    }
    ///Bit 7 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck7(&mut self) -> LCK7_W {
        LCK7_W { w: self }
    }
    ///Bit 6 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck6(&mut self) -> LCK6_W {
        LCK6_W { w: self }
    }
    ///Bit 5 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck5(&mut self) -> LCK5_W {
        LCK5_W { w: self }
    }
    ///Bit 4 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck4(&mut self) -> LCK4_W {
        LCK4_W { w: self }
    }
    ///Bit 3 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck3(&mut self) -> LCK3_W {
        LCK3_W { w: self }
    }
    ///Bit 2 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck2(&mut self) -> LCK2_W {
        LCK2_W { w: self }
    }
    ///Bit 1 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck1(&mut self) -> LCK1_W {
        LCK1_W { w: self }
    }
    ///Bit 0 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck0(&mut self) -> LCK0_W {
        LCK0_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GPIO port configuration lock register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lckr](index.html) module
pub struct LCKR_SPEC;
impl crate::RegisterSpec for LCKR_SPEC {
    type Ux = u32;
}
///`read()` method returns [lckr::R](R) reader structure
impl crate::Readable for LCKR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [lckr::W](W) writer structure
impl crate::Writable for LCKR_SPEC {
    type Writer = W;
}
///`reset()` method sets LCKR to value 0
impl crate::Resettable for LCKR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
