///Register `AFRL` reader
pub struct R(crate::R<AFRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFRL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AFRL` writer
pub struct W(crate::W<AFRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFRL_SPEC>;
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
impl From<crate::W<AFRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFRL_SPEC>) -> Self {
        W(writer)
    }
}
///Alternate function selection for port x pin y (y = 0..7)
pub type AFSEL7_A = AFSEL0_A;
///Field `AFSEL7` reader - Alternate function selection for port x pin y (y = 0..7)
pub type AFSEL7_R = AFSEL0_R;
///Field `AFSEL7` writer - Alternate function selection for port x pin y (y = 0..7)
pub struct AFSEL7_W<'a> {
    w: &'a mut W,
}
impl<'a> AFSEL7_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AFSEL7_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///AF0
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFSEL7_A::AF0)
    }
    ///AF1
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFSEL7_A::AF1)
    }
    ///AF2
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFSEL7_A::AF2)
    }
    ///AF3
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFSEL7_A::AF3)
    }
    ///AF4
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFSEL7_A::AF4)
    }
    ///AF5
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFSEL7_A::AF5)
    }
    ///AF6
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFSEL7_A::AF6)
    }
    ///AF7
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFSEL7_A::AF7)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
///Alternate function selection for port x pin y (y = 0..7)
pub type AFSEL6_A = AFSEL0_A;
///Field `AFSEL6` reader - Alternate function selection for port x pin y (y = 0..7)
pub type AFSEL6_R = AFSEL0_R;
///Field `AFSEL6` writer - Alternate function selection for port x pin y (y = 0..7)
pub struct AFSEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> AFSEL6_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AFSEL6_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///AF0
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFSEL6_A::AF0)
    }
    ///AF1
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFSEL6_A::AF1)
    }
    ///AF2
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFSEL6_A::AF2)
    }
    ///AF3
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFSEL6_A::AF3)
    }
    ///AF4
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFSEL6_A::AF4)
    }
    ///AF5
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFSEL6_A::AF5)
    }
    ///AF6
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFSEL6_A::AF6)
    }
    ///AF7
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFSEL6_A::AF7)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
///Alternate function selection for port x pin y (y = 0..7)
pub type AFSEL5_A = AFSEL0_A;
///Field `AFSEL5` reader - Alternate function selection for port x pin y (y = 0..7)
pub type AFSEL5_R = AFSEL0_R;
///Field `AFSEL5` writer - Alternate function selection for port x pin y (y = 0..7)
pub struct AFSEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> AFSEL5_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AFSEL5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///AF0
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFSEL5_A::AF0)
    }
    ///AF1
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFSEL5_A::AF1)
    }
    ///AF2
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFSEL5_A::AF2)
    }
    ///AF3
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFSEL5_A::AF3)
    }
    ///AF4
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFSEL5_A::AF4)
    }
    ///AF5
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFSEL5_A::AF5)
    }
    ///AF6
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFSEL5_A::AF6)
    }
    ///AF7
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFSEL5_A::AF7)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
///Alternate function selection for port x pin y (y = 0..7)
pub type AFSEL4_A = AFSEL0_A;
///Field `AFSEL4` reader - Alternate function selection for port x pin y (y = 0..7)
pub type AFSEL4_R = AFSEL0_R;
///Field `AFSEL4` writer - Alternate function selection for port x pin y (y = 0..7)
pub struct AFSEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> AFSEL4_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AFSEL4_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///AF0
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFSEL4_A::AF0)
    }
    ///AF1
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFSEL4_A::AF1)
    }
    ///AF2
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFSEL4_A::AF2)
    }
    ///AF3
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFSEL4_A::AF3)
    }
    ///AF4
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFSEL4_A::AF4)
    }
    ///AF5
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFSEL4_A::AF5)
    }
    ///AF6
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFSEL4_A::AF6)
    }
    ///AF7
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFSEL4_A::AF7)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
///Alternate function selection for port x pin y (y = 0..7)
pub type AFSEL3_A = AFSEL0_A;
///Field `AFSEL3` reader - Alternate function selection for port x pin y (y = 0..7)
pub type AFSEL3_R = AFSEL0_R;
///Field `AFSEL3` writer - Alternate function selection for port x pin y (y = 0..7)
pub struct AFSEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> AFSEL3_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AFSEL3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///AF0
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFSEL3_A::AF0)
    }
    ///AF1
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFSEL3_A::AF1)
    }
    ///AF2
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFSEL3_A::AF2)
    }
    ///AF3
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFSEL3_A::AF3)
    }
    ///AF4
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFSEL3_A::AF4)
    }
    ///AF5
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFSEL3_A::AF5)
    }
    ///AF6
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFSEL3_A::AF6)
    }
    ///AF7
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFSEL3_A::AF7)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
///Alternate function selection for port x pin y (y = 0..7)
pub type AFSEL2_A = AFSEL0_A;
///Field `AFSEL2` reader - Alternate function selection for port x pin y (y = 0..7)
pub type AFSEL2_R = AFSEL0_R;
///Field `AFSEL2` writer - Alternate function selection for port x pin y (y = 0..7)
pub struct AFSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> AFSEL2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AFSEL2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///AF0
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFSEL2_A::AF0)
    }
    ///AF1
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFSEL2_A::AF1)
    }
    ///AF2
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFSEL2_A::AF2)
    }
    ///AF3
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFSEL2_A::AF3)
    }
    ///AF4
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFSEL2_A::AF4)
    }
    ///AF5
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFSEL2_A::AF5)
    }
    ///AF6
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFSEL2_A::AF6)
    }
    ///AF7
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFSEL2_A::AF7)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
///Alternate function selection for port x pin y (y = 0..7)
pub type AFSEL1_A = AFSEL0_A;
///Field `AFSEL1` reader - Alternate function selection for port x pin y (y = 0..7)
pub type AFSEL1_R = AFSEL0_R;
///Field `AFSEL1` writer - Alternate function selection for port x pin y (y = 0..7)
pub struct AFSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> AFSEL1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AFSEL1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///AF0
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFSEL1_A::AF0)
    }
    ///AF1
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFSEL1_A::AF1)
    }
    ///AF2
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFSEL1_A::AF2)
    }
    ///AF3
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFSEL1_A::AF3)
    }
    ///AF4
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFSEL1_A::AF4)
    }
    ///AF5
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFSEL1_A::AF5)
    }
    ///AF6
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFSEL1_A::AF6)
    }
    ///AF7
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFSEL1_A::AF7)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
///Alternate function selection for port x pin y (y = 0..7)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AFSEL0_A {
    ///0: AF0
    AF0 = 0,
    ///1: AF1
    AF1 = 1,
    ///2: AF2
    AF2 = 2,
    ///3: AF3
    AF3 = 3,
    ///4: AF4
    AF4 = 4,
    ///5: AF5
    AF5 = 5,
    ///6: AF6
    AF6 = 6,
    ///7: AF7
    AF7 = 7,
}
impl From<AFSEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: AFSEL0_A) -> Self {
        variant as _
    }
}
///Field `AFSEL0` reader - Alternate function selection for port x pin y (y = 0..7)
pub struct AFSEL0_R(crate::FieldReader<u8, AFSEL0_A>);
impl AFSEL0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        AFSEL0_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<AFSEL0_A> {
        match self.bits {
            0 => Some(AFSEL0_A::AF0),
            1 => Some(AFSEL0_A::AF1),
            2 => Some(AFSEL0_A::AF2),
            3 => Some(AFSEL0_A::AF3),
            4 => Some(AFSEL0_A::AF4),
            5 => Some(AFSEL0_A::AF5),
            6 => Some(AFSEL0_A::AF6),
            7 => Some(AFSEL0_A::AF7),
            _ => None,
        }
    }
    ///Checks if the value of the field is `AF0`
    #[inline(always)]
    pub fn is_af0(&self) -> bool {
        **self == AFSEL0_A::AF0
    }
    ///Checks if the value of the field is `AF1`
    #[inline(always)]
    pub fn is_af1(&self) -> bool {
        **self == AFSEL0_A::AF1
    }
    ///Checks if the value of the field is `AF2`
    #[inline(always)]
    pub fn is_af2(&self) -> bool {
        **self == AFSEL0_A::AF2
    }
    ///Checks if the value of the field is `AF3`
    #[inline(always)]
    pub fn is_af3(&self) -> bool {
        **self == AFSEL0_A::AF3
    }
    ///Checks if the value of the field is `AF4`
    #[inline(always)]
    pub fn is_af4(&self) -> bool {
        **self == AFSEL0_A::AF4
    }
    ///Checks if the value of the field is `AF5`
    #[inline(always)]
    pub fn is_af5(&self) -> bool {
        **self == AFSEL0_A::AF5
    }
    ///Checks if the value of the field is `AF6`
    #[inline(always)]
    pub fn is_af6(&self) -> bool {
        **self == AFSEL0_A::AF6
    }
    ///Checks if the value of the field is `AF7`
    #[inline(always)]
    pub fn is_af7(&self) -> bool {
        **self == AFSEL0_A::AF7
    }
}
impl core::ops::Deref for AFSEL0_R {
    type Target = crate::FieldReader<u8, AFSEL0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `AFSEL0` writer - Alternate function selection for port x pin y (y = 0..7)
pub struct AFSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> AFSEL0_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AFSEL0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///AF0
    #[inline(always)]
    pub fn af0(self) -> &'a mut W {
        self.variant(AFSEL0_A::AF0)
    }
    ///AF1
    #[inline(always)]
    pub fn af1(self) -> &'a mut W {
        self.variant(AFSEL0_A::AF1)
    }
    ///AF2
    #[inline(always)]
    pub fn af2(self) -> &'a mut W {
        self.variant(AFSEL0_A::AF2)
    }
    ///AF3
    #[inline(always)]
    pub fn af3(self) -> &'a mut W {
        self.variant(AFSEL0_A::AF3)
    }
    ///AF4
    #[inline(always)]
    pub fn af4(self) -> &'a mut W {
        self.variant(AFSEL0_A::AF4)
    }
    ///AF5
    #[inline(always)]
    pub fn af5(self) -> &'a mut W {
        self.variant(AFSEL0_A::AF5)
    }
    ///AF6
    #[inline(always)]
    pub fn af6(self) -> &'a mut W {
        self.variant(AFSEL0_A::AF6)
    }
    ///AF7
    #[inline(always)]
    pub fn af7(self) -> &'a mut W {
        self.variant(AFSEL0_A::AF7)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    ///Bits 28:31 - Alternate function selection for port x pin y (y = 0..7)
    #[inline(always)]
    pub fn afsel7(&self) -> AFSEL7_R {
        AFSEL7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    ///Bits 24:27 - Alternate function selection for port x pin y (y = 0..7)
    #[inline(always)]
    pub fn afsel6(&self) -> AFSEL6_R {
        AFSEL6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 20:23 - Alternate function selection for port x pin y (y = 0..7)
    #[inline(always)]
    pub fn afsel5(&self) -> AFSEL5_R {
        AFSEL5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 16:19 - Alternate function selection for port x pin y (y = 0..7)
    #[inline(always)]
    pub fn afsel4(&self) -> AFSEL4_R {
        AFSEL4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 12:15 - Alternate function selection for port x pin y (y = 0..7)
    #[inline(always)]
    pub fn afsel3(&self) -> AFSEL3_R {
        AFSEL3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 8:11 - Alternate function selection for port x pin y (y = 0..7)
    #[inline(always)]
    pub fn afsel2(&self) -> AFSEL2_R {
        AFSEL2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 4:7 - Alternate function selection for port x pin y (y = 0..7)
    #[inline(always)]
    pub fn afsel1(&self) -> AFSEL1_R {
        AFSEL1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 0:3 - Alternate function selection for port x pin y (y = 0..7)
    #[inline(always)]
    pub fn afsel0(&self) -> AFSEL0_R {
        AFSEL0_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    ///Bits 28:31 - Alternate function selection for port x pin y (y = 0..7)
    #[inline(always)]
    pub fn afsel7(&mut self) -> AFSEL7_W {
        AFSEL7_W { w: self }
    }
    ///Bits 24:27 - Alternate function selection for port x pin y (y = 0..7)
    #[inline(always)]
    pub fn afsel6(&mut self) -> AFSEL6_W {
        AFSEL6_W { w: self }
    }
    ///Bits 20:23 - Alternate function selection for port x pin y (y = 0..7)
    #[inline(always)]
    pub fn afsel5(&mut self) -> AFSEL5_W {
        AFSEL5_W { w: self }
    }
    ///Bits 16:19 - Alternate function selection for port x pin y (y = 0..7)
    #[inline(always)]
    pub fn afsel4(&mut self) -> AFSEL4_W {
        AFSEL4_W { w: self }
    }
    ///Bits 12:15 - Alternate function selection for port x pin y (y = 0..7)
    #[inline(always)]
    pub fn afsel3(&mut self) -> AFSEL3_W {
        AFSEL3_W { w: self }
    }
    ///Bits 8:11 - Alternate function selection for port x pin y (y = 0..7)
    #[inline(always)]
    pub fn afsel2(&mut self) -> AFSEL2_W {
        AFSEL2_W { w: self }
    }
    ///Bits 4:7 - Alternate function selection for port x pin y (y = 0..7)
    #[inline(always)]
    pub fn afsel1(&mut self) -> AFSEL1_W {
        AFSEL1_W { w: self }
    }
    ///Bits 0:3 - Alternate function selection for port x pin y (y = 0..7)
    #[inline(always)]
    pub fn afsel0(&mut self) -> AFSEL0_W {
        AFSEL0_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GPIO alternate function low register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [afrl](index.html) module
pub struct AFRL_SPEC;
impl crate::RegisterSpec for AFRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [afrl::R](R) reader structure
impl crate::Readable for AFRL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [afrl::W](W) writer structure
impl crate::Writable for AFRL_SPEC {
    type Writer = W;
}
///`reset()` method sets AFRL to value 0
impl crate::Resettable for AFRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
