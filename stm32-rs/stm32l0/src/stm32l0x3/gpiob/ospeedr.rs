///Register `OSPEEDR` reader
pub struct R(crate::R<OSPEEDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSPEEDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSPEEDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSPEEDR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OSPEEDR` writer
pub struct W(crate::W<OSPEEDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSPEEDR_SPEC>;
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
impl From<crate::W<OSPEEDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSPEEDR_SPEC>) -> Self {
        W(writer)
    }
}
///Port x configuration bits (y = 0..15)
pub type OSPEED15_A = OSPEED0_A;
///Field `OSPEED15` reader - Port x configuration bits (y = 0..15)
pub type OSPEED15_R = OSPEED0_R;
///Field `OSPEED15` writer - Port x configuration bits (y = 0..15)
pub struct OSPEED15_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED15_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OSPEED15_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Low speed
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::LOWSPEED)
    }
    ///Medium speed
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::MEDIUMSPEED)
    }
    ///High speed
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::HIGHSPEED)
    }
    ///Very high speed
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEED15_A::VERYHIGHSPEED)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
///Port x configuration bits (y = 0..15)
pub type OSPEED14_A = OSPEED0_A;
///Field `OSPEED14` reader - Port x configuration bits (y = 0..15)
pub type OSPEED14_R = OSPEED0_R;
///Field `OSPEED14` writer - Port x configuration bits (y = 0..15)
pub struct OSPEED14_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED14_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OSPEED14_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Low speed
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEED14_A::LOWSPEED)
    }
    ///Medium speed
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEED14_A::MEDIUMSPEED)
    }
    ///High speed
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEED14_A::HIGHSPEED)
    }
    ///Very high speed
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEED14_A::VERYHIGHSPEED)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
///Port x configuration bits (y = 0..15)
pub type OSPEED13_A = OSPEED0_A;
///Field `OSPEED13` reader - Port x configuration bits (y = 0..15)
pub type OSPEED13_R = OSPEED0_R;
///Field `OSPEED13` writer - Port x configuration bits (y = 0..15)
pub struct OSPEED13_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED13_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OSPEED13_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Low speed
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEED13_A::LOWSPEED)
    }
    ///Medium speed
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEED13_A::MEDIUMSPEED)
    }
    ///High speed
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEED13_A::HIGHSPEED)
    }
    ///Very high speed
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEED13_A::VERYHIGHSPEED)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
///Port x configuration bits (y = 0..15)
pub type OSPEED12_A = OSPEED0_A;
///Field `OSPEED12` reader - Port x configuration bits (y = 0..15)
pub type OSPEED12_R = OSPEED0_R;
///Field `OSPEED12` writer - Port x configuration bits (y = 0..15)
pub struct OSPEED12_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED12_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OSPEED12_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Low speed
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEED12_A::LOWSPEED)
    }
    ///Medium speed
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEED12_A::MEDIUMSPEED)
    }
    ///High speed
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEED12_A::HIGHSPEED)
    }
    ///Very high speed
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEED12_A::VERYHIGHSPEED)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
///Port x configuration bits (y = 0..15)
pub type OSPEED11_A = OSPEED0_A;
///Field `OSPEED11` reader - Port x configuration bits (y = 0..15)
pub type OSPEED11_R = OSPEED0_R;
///Field `OSPEED11` writer - Port x configuration bits (y = 0..15)
pub struct OSPEED11_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED11_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OSPEED11_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Low speed
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEED11_A::LOWSPEED)
    }
    ///Medium speed
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEED11_A::MEDIUMSPEED)
    }
    ///High speed
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEED11_A::HIGHSPEED)
    }
    ///Very high speed
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEED11_A::VERYHIGHSPEED)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
///Port x configuration bits (y = 0..15)
pub type OSPEED10_A = OSPEED0_A;
///Field `OSPEED10` reader - Port x configuration bits (y = 0..15)
pub type OSPEED10_R = OSPEED0_R;
///Field `OSPEED10` writer - Port x configuration bits (y = 0..15)
pub struct OSPEED10_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED10_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OSPEED10_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Low speed
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEED10_A::LOWSPEED)
    }
    ///Medium speed
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEED10_A::MEDIUMSPEED)
    }
    ///High speed
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEED10_A::HIGHSPEED)
    }
    ///Very high speed
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEED10_A::VERYHIGHSPEED)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
///Port x configuration bits (y = 0..15)
pub type OSPEED9_A = OSPEED0_A;
///Field `OSPEED9` reader - Port x configuration bits (y = 0..15)
pub type OSPEED9_R = OSPEED0_R;
///Field `OSPEED9` writer - Port x configuration bits (y = 0..15)
pub struct OSPEED9_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED9_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OSPEED9_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Low speed
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEED9_A::LOWSPEED)
    }
    ///Medium speed
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEED9_A::MEDIUMSPEED)
    }
    ///High speed
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEED9_A::HIGHSPEED)
    }
    ///Very high speed
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEED9_A::VERYHIGHSPEED)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
///Port x configuration bits (y = 0..15)
pub type OSPEED8_A = OSPEED0_A;
///Field `OSPEED8` reader - Port x configuration bits (y = 0..15)
pub type OSPEED8_R = OSPEED0_R;
///Field `OSPEED8` writer - Port x configuration bits (y = 0..15)
pub struct OSPEED8_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED8_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OSPEED8_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Low speed
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEED8_A::LOWSPEED)
    }
    ///Medium speed
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEED8_A::MEDIUMSPEED)
    }
    ///High speed
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEED8_A::HIGHSPEED)
    }
    ///Very high speed
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEED8_A::VERYHIGHSPEED)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
///Port x configuration bits (y = 0..15)
pub type OSPEED7_A = OSPEED0_A;
///Field `OSPEED7` reader - Port x configuration bits (y = 0..15)
pub type OSPEED7_R = OSPEED0_R;
///Field `OSPEED7` writer - Port x configuration bits (y = 0..15)
pub struct OSPEED7_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED7_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OSPEED7_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Low speed
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEED7_A::LOWSPEED)
    }
    ///Medium speed
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEED7_A::MEDIUMSPEED)
    }
    ///High speed
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEED7_A::HIGHSPEED)
    }
    ///Very high speed
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEED7_A::VERYHIGHSPEED)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
///Port x configuration bits (y = 0..15)
pub type OSPEED6_A = OSPEED0_A;
///Field `OSPEED6` reader - Port x configuration bits (y = 0..15)
pub type OSPEED6_R = OSPEED0_R;
///Field `OSPEED6` writer - Port x configuration bits (y = 0..15)
pub struct OSPEED6_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED6_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OSPEED6_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Low speed
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEED6_A::LOWSPEED)
    }
    ///Medium speed
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEED6_A::MEDIUMSPEED)
    }
    ///High speed
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEED6_A::HIGHSPEED)
    }
    ///Very high speed
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEED6_A::VERYHIGHSPEED)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
///Port x configuration bits (y = 0..15)
pub type OSPEED5_A = OSPEED0_A;
///Field `OSPEED5` reader - Port x configuration bits (y = 0..15)
pub type OSPEED5_R = OSPEED0_R;
///Field `OSPEED5` writer - Port x configuration bits (y = 0..15)
pub struct OSPEED5_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED5_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OSPEED5_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Low speed
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEED5_A::LOWSPEED)
    }
    ///Medium speed
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEED5_A::MEDIUMSPEED)
    }
    ///High speed
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEED5_A::HIGHSPEED)
    }
    ///Very high speed
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEED5_A::VERYHIGHSPEED)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
///Port x configuration bits (y = 0..15)
pub type OSPEED4_A = OSPEED0_A;
///Field `OSPEED4` reader - Port x configuration bits (y = 0..15)
pub type OSPEED4_R = OSPEED0_R;
///Field `OSPEED4` writer - Port x configuration bits (y = 0..15)
pub struct OSPEED4_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED4_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OSPEED4_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Low speed
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEED4_A::LOWSPEED)
    }
    ///Medium speed
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEED4_A::MEDIUMSPEED)
    }
    ///High speed
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEED4_A::HIGHSPEED)
    }
    ///Very high speed
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEED4_A::VERYHIGHSPEED)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
///Port x configuration bits (y = 0..15)
pub type OSPEED3_A = OSPEED0_A;
///Field `OSPEED3` reader - Port x configuration bits (y = 0..15)
pub type OSPEED3_R = OSPEED0_R;
///Field `OSPEED3` writer - Port x configuration bits (y = 0..15)
pub struct OSPEED3_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED3_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OSPEED3_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Low speed
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEED3_A::LOWSPEED)
    }
    ///Medium speed
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEED3_A::MEDIUMSPEED)
    }
    ///High speed
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEED3_A::HIGHSPEED)
    }
    ///Very high speed
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEED3_A::VERYHIGHSPEED)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
///Port x configuration bits (y = 0..15)
pub type OSPEED2_A = OSPEED0_A;
///Field `OSPEED2` reader - Port x configuration bits (y = 0..15)
pub type OSPEED2_R = OSPEED0_R;
///Field `OSPEED2` writer - Port x configuration bits (y = 0..15)
pub struct OSPEED2_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OSPEED2_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Low speed
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEED2_A::LOWSPEED)
    }
    ///Medium speed
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEED2_A::MEDIUMSPEED)
    }
    ///High speed
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEED2_A::HIGHSPEED)
    }
    ///Very high speed
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEED2_A::VERYHIGHSPEED)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
///Port x configuration bits (y = 0..15)
pub type OSPEED1_A = OSPEED0_A;
///Field `OSPEED1` reader - Port x configuration bits (y = 0..15)
pub type OSPEED1_R = OSPEED0_R;
///Field `OSPEED1` writer - Port x configuration bits (y = 0..15)
pub struct OSPEED1_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OSPEED1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Low speed
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEED1_A::LOWSPEED)
    }
    ///Medium speed
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEED1_A::MEDIUMSPEED)
    }
    ///High speed
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEED1_A::HIGHSPEED)
    }
    ///Very high speed
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEED1_A::VERYHIGHSPEED)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
///Port x configuration bits (y = 0..15)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OSPEED0_A {
    ///0: Low speed
    LOWSPEED = 0,
    ///1: Medium speed
    MEDIUMSPEED = 1,
    ///2: High speed
    HIGHSPEED = 2,
    ///3: Very high speed
    VERYHIGHSPEED = 3,
}
impl From<OSPEED0_A> for u8 {
    #[inline(always)]
    fn from(variant: OSPEED0_A) -> Self {
        variant as _
    }
}
///Field `OSPEED0` reader - Port x configuration bits (y = 0..15)
pub struct OSPEED0_R(crate::FieldReader<u8, OSPEED0_A>);
impl OSPEED0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OSPEED0_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OSPEED0_A {
        match self.bits {
            0 => OSPEED0_A::LOWSPEED,
            1 => OSPEED0_A::MEDIUMSPEED,
            2 => OSPEED0_A::HIGHSPEED,
            3 => OSPEED0_A::VERYHIGHSPEED,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `LOWSPEED`
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        **self == OSPEED0_A::LOWSPEED
    }
    ///Checks if the value of the field is `MEDIUMSPEED`
    #[inline(always)]
    pub fn is_medium_speed(&self) -> bool {
        **self == OSPEED0_A::MEDIUMSPEED
    }
    ///Checks if the value of the field is `HIGHSPEED`
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        **self == OSPEED0_A::HIGHSPEED
    }
    ///Checks if the value of the field is `VERYHIGHSPEED`
    #[inline(always)]
    pub fn is_very_high_speed(&self) -> bool {
        **self == OSPEED0_A::VERYHIGHSPEED
    }
}
impl core::ops::Deref for OSPEED0_R {
    type Target = crate::FieldReader<u8, OSPEED0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `OSPEED0` writer - Port x configuration bits (y = 0..15)
pub struct OSPEED0_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEED0_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OSPEED0_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Low speed
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(OSPEED0_A::LOWSPEED)
    }
    ///Medium speed
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(OSPEED0_A::MEDIUMSPEED)
    }
    ///High speed
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OSPEED0_A::HIGHSPEED)
    }
    ///Very high speed
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut W {
        self.variant(OSPEED0_A::VERYHIGHSPEED)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    ///Bits 30:31 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeed15(&self) -> OSPEED15_R {
        OSPEED15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    ///Bits 28:29 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeed14(&self) -> OSPEED14_R {
        OSPEED14_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    ///Bits 26:27 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeed13(&self) -> OSPEED13_R {
        OSPEED13_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    ///Bits 24:25 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeed12(&self) -> OSPEED12_R {
        OSPEED12_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    ///Bits 22:23 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeed11(&self) -> OSPEED11_R {
        OSPEED11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    ///Bits 20:21 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeed10(&self) -> OSPEED10_R {
        OSPEED10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    ///Bits 18:19 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeed9(&self) -> OSPEED9_R {
        OSPEED9_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    ///Bits 16:17 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeed8(&self) -> OSPEED8_R {
        OSPEED8_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    ///Bits 14:15 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeed7(&self) -> OSPEED7_R {
        OSPEED7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    ///Bits 12:13 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeed6(&self) -> OSPEED6_R {
        OSPEED6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    ///Bits 10:11 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeed5(&self) -> OSPEED5_R {
        OSPEED5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    ///Bits 8:9 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeed4(&self) -> OSPEED4_R {
        OSPEED4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    ///Bits 6:7 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeed3(&self) -> OSPEED3_R {
        OSPEED3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    ///Bits 4:5 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeed2(&self) -> OSPEED2_R {
        OSPEED2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    ///Bits 2:3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeed1(&self) -> OSPEED1_R {
        OSPEED1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    ///Bits 0:1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeed0(&self) -> OSPEED0_R {
        OSPEED0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    ///Bits 30:31 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeed15(&mut self) -> OSPEED15_W {
        OSPEED15_W { w: self }
    }
    ///Bits 28:29 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeed14(&mut self) -> OSPEED14_W {
        OSPEED14_W { w: self }
    }
    ///Bits 26:27 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeed13(&mut self) -> OSPEED13_W {
        OSPEED13_W { w: self }
    }
    ///Bits 24:25 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeed12(&mut self) -> OSPEED12_W {
        OSPEED12_W { w: self }
    }
    ///Bits 22:23 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeed11(&mut self) -> OSPEED11_W {
        OSPEED11_W { w: self }
    }
    ///Bits 20:21 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeed10(&mut self) -> OSPEED10_W {
        OSPEED10_W { w: self }
    }
    ///Bits 18:19 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeed9(&mut self) -> OSPEED9_W {
        OSPEED9_W { w: self }
    }
    ///Bits 16:17 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeed8(&mut self) -> OSPEED8_W {
        OSPEED8_W { w: self }
    }
    ///Bits 14:15 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeed7(&mut self) -> OSPEED7_W {
        OSPEED7_W { w: self }
    }
    ///Bits 12:13 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeed6(&mut self) -> OSPEED6_W {
        OSPEED6_W { w: self }
    }
    ///Bits 10:11 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeed5(&mut self) -> OSPEED5_W {
        OSPEED5_W { w: self }
    }
    ///Bits 8:9 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeed4(&mut self) -> OSPEED4_W {
        OSPEED4_W { w: self }
    }
    ///Bits 6:7 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeed3(&mut self) -> OSPEED3_W {
        OSPEED3_W { w: self }
    }
    ///Bits 4:5 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeed2(&mut self) -> OSPEED2_W {
        OSPEED2_W { w: self }
    }
    ///Bits 2:3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeed1(&mut self) -> OSPEED1_W {
        OSPEED1_W { w: self }
    }
    ///Bits 0:1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeed0(&mut self) -> OSPEED0_W {
        OSPEED0_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GPIO port output speed register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ospeedr](index.html) module
pub struct OSPEEDR_SPEC;
impl crate::RegisterSpec for OSPEEDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ospeedr::R](R) reader structure
impl crate::Readable for OSPEEDR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ospeedr::W](W) writer structure
impl crate::Writable for OSPEEDR_SPEC {
    type Writer = W;
}
///`reset()` method sets OSPEEDR to value 0
impl crate::Resettable for OSPEEDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
