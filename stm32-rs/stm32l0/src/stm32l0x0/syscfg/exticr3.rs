///Register `EXTICR3` reader
pub struct R(crate::R<EXTICR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTICR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTICR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTICR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `EXTICR3` writer
pub struct W(crate::W<EXTICR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTICR3_SPEC>;
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
impl From<crate::W<EXTICR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTICR3_SPEC>) -> Self {
        W(writer)
    }
}
///EXTI x configuration (x = 8 to 11)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI11_A {
    ///0: Select PA11 as the source input for the EXTI11 external interrupt
    PA11 = 0,
    ///1: Select PB11 as the source input for the EXTI11 external interrupt
    PB11 = 1,
    ///2: Select PC11 as the source input for the EXTI11 external interrupt
    PC11 = 2,
    ///3: Select PD11 as the source input for the EXTI11 external interrupt
    PD11 = 3,
    ///4: Select PE11 as the source input for the EXTI11 external interrupt
    PE11 = 4,
    ///5: Select PH11 as the source input for the EXTI11 external interrupt
    PH11 = 5,
}
impl From<EXTI11_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI11_A) -> Self {
        variant as _
    }
}
///Field `EXTI11` reader - EXTI x configuration (x = 8 to 11)
pub struct EXTI11_R(crate::FieldReader<u8, EXTI11_A>);
impl EXTI11_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EXTI11_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI11_A> {
        match self.bits {
            0 => Some(EXTI11_A::PA11),
            1 => Some(EXTI11_A::PB11),
            2 => Some(EXTI11_A::PC11),
            3 => Some(EXTI11_A::PD11),
            4 => Some(EXTI11_A::PE11),
            5 => Some(EXTI11_A::PH11),
            _ => None,
        }
    }
    ///Checks if the value of the field is `PA11`
    #[inline(always)]
    pub fn is_pa11(&self) -> bool {
        **self == EXTI11_A::PA11
    }
    ///Checks if the value of the field is `PB11`
    #[inline(always)]
    pub fn is_pb11(&self) -> bool {
        **self == EXTI11_A::PB11
    }
    ///Checks if the value of the field is `PC11`
    #[inline(always)]
    pub fn is_pc11(&self) -> bool {
        **self == EXTI11_A::PC11
    }
    ///Checks if the value of the field is `PD11`
    #[inline(always)]
    pub fn is_pd11(&self) -> bool {
        **self == EXTI11_A::PD11
    }
    ///Checks if the value of the field is `PE11`
    #[inline(always)]
    pub fn is_pe11(&self) -> bool {
        **self == EXTI11_A::PE11
    }
    ///Checks if the value of the field is `PH11`
    #[inline(always)]
    pub fn is_ph11(&self) -> bool {
        **self == EXTI11_A::PH11
    }
}
impl core::ops::Deref for EXTI11_R {
    type Target = crate::FieldReader<u8, EXTI11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EXTI11` writer - EXTI x configuration (x = 8 to 11)
pub struct EXTI11_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI11_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EXTI11_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Select PA11 as the source input for the EXTI11 external interrupt
    #[inline(always)]
    pub fn pa11(self) -> &'a mut W {
        self.variant(EXTI11_A::PA11)
    }
    ///Select PB11 as the source input for the EXTI11 external interrupt
    #[inline(always)]
    pub fn pb11(self) -> &'a mut W {
        self.variant(EXTI11_A::PB11)
    }
    ///Select PC11 as the source input for the EXTI11 external interrupt
    #[inline(always)]
    pub fn pc11(self) -> &'a mut W {
        self.variant(EXTI11_A::PC11)
    }
    ///Select PD11 as the source input for the EXTI11 external interrupt
    #[inline(always)]
    pub fn pd11(self) -> &'a mut W {
        self.variant(EXTI11_A::PD11)
    }
    ///Select PE11 as the source input for the EXTI11 external interrupt
    #[inline(always)]
    pub fn pe11(self) -> &'a mut W {
        self.variant(EXTI11_A::PE11)
    }
    ///Select PH11 as the source input for the EXTI11 external interrupt
    #[inline(always)]
    pub fn ph11(self) -> &'a mut W {
        self.variant(EXTI11_A::PH11)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
///EXTI10
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI10_A {
    ///0: Select PA10 as the source input for the EXTI10 external interrupt
    PA10 = 0,
    ///1: Select PB10 as the source input for the EXTI10 external interrupt
    PB10 = 1,
    ///2: Select PC10 as the source input for the EXTI10 external interrupt
    PC10 = 2,
    ///3: Select PD10 as the source input for the EXTI10 external interrupt
    PD10 = 3,
    ///4: Select PE10 as the source input for the EXTI10 external interrupt
    PE10 = 4,
    ///5: Select PH10 as the source input for the EXTI10 external interrupt
    PH10 = 5,
}
impl From<EXTI10_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI10_A) -> Self {
        variant as _
    }
}
///Field `EXTI10` reader - EXTI10
pub struct EXTI10_R(crate::FieldReader<u8, EXTI10_A>);
impl EXTI10_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EXTI10_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI10_A> {
        match self.bits {
            0 => Some(EXTI10_A::PA10),
            1 => Some(EXTI10_A::PB10),
            2 => Some(EXTI10_A::PC10),
            3 => Some(EXTI10_A::PD10),
            4 => Some(EXTI10_A::PE10),
            5 => Some(EXTI10_A::PH10),
            _ => None,
        }
    }
    ///Checks if the value of the field is `PA10`
    #[inline(always)]
    pub fn is_pa10(&self) -> bool {
        **self == EXTI10_A::PA10
    }
    ///Checks if the value of the field is `PB10`
    #[inline(always)]
    pub fn is_pb10(&self) -> bool {
        **self == EXTI10_A::PB10
    }
    ///Checks if the value of the field is `PC10`
    #[inline(always)]
    pub fn is_pc10(&self) -> bool {
        **self == EXTI10_A::PC10
    }
    ///Checks if the value of the field is `PD10`
    #[inline(always)]
    pub fn is_pd10(&self) -> bool {
        **self == EXTI10_A::PD10
    }
    ///Checks if the value of the field is `PE10`
    #[inline(always)]
    pub fn is_pe10(&self) -> bool {
        **self == EXTI10_A::PE10
    }
    ///Checks if the value of the field is `PH10`
    #[inline(always)]
    pub fn is_ph10(&self) -> bool {
        **self == EXTI10_A::PH10
    }
}
impl core::ops::Deref for EXTI10_R {
    type Target = crate::FieldReader<u8, EXTI10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EXTI10` writer - EXTI10
pub struct EXTI10_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI10_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EXTI10_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Select PA10 as the source input for the EXTI10 external interrupt
    #[inline(always)]
    pub fn pa10(self) -> &'a mut W {
        self.variant(EXTI10_A::PA10)
    }
    ///Select PB10 as the source input for the EXTI10 external interrupt
    #[inline(always)]
    pub fn pb10(self) -> &'a mut W {
        self.variant(EXTI10_A::PB10)
    }
    ///Select PC10 as the source input for the EXTI10 external interrupt
    #[inline(always)]
    pub fn pc10(self) -> &'a mut W {
        self.variant(EXTI10_A::PC10)
    }
    ///Select PD10 as the source input for the EXTI10 external interrupt
    #[inline(always)]
    pub fn pd10(self) -> &'a mut W {
        self.variant(EXTI10_A::PD10)
    }
    ///Select PE10 as the source input for the EXTI10 external interrupt
    #[inline(always)]
    pub fn pe10(self) -> &'a mut W {
        self.variant(EXTI10_A::PE10)
    }
    ///Select PH10 as the source input for the EXTI10 external interrupt
    #[inline(always)]
    pub fn ph10(self) -> &'a mut W {
        self.variant(EXTI10_A::PH10)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
///EXTI x configuration (x = 8 to 11)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI9_A {
    ///0: Select PA9 as the source input for the EXTI9 external interrupt
    PA9 = 0,
    ///1: Select PB9 as the source input for the EXTI9 external interrupt
    PB9 = 1,
    ///2: Select PC9 as the source input for the EXTI9 external interrupt
    PC9 = 2,
    ///3: Select PD9 as the source input for the EXTI9 external interrupt
    PD9 = 3,
    ///4: Select PE9 as the source input for the EXTI9 external interrupt
    PE9 = 4,
    ///5: Select PH9 as the source input for the EXTI9 external interrupt
    PH9 = 5,
}
impl From<EXTI9_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI9_A) -> Self {
        variant as _
    }
}
///Field `EXTI9` reader - EXTI x configuration (x = 8 to 11)
pub struct EXTI9_R(crate::FieldReader<u8, EXTI9_A>);
impl EXTI9_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EXTI9_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI9_A> {
        match self.bits {
            0 => Some(EXTI9_A::PA9),
            1 => Some(EXTI9_A::PB9),
            2 => Some(EXTI9_A::PC9),
            3 => Some(EXTI9_A::PD9),
            4 => Some(EXTI9_A::PE9),
            5 => Some(EXTI9_A::PH9),
            _ => None,
        }
    }
    ///Checks if the value of the field is `PA9`
    #[inline(always)]
    pub fn is_pa9(&self) -> bool {
        **self == EXTI9_A::PA9
    }
    ///Checks if the value of the field is `PB9`
    #[inline(always)]
    pub fn is_pb9(&self) -> bool {
        **self == EXTI9_A::PB9
    }
    ///Checks if the value of the field is `PC9`
    #[inline(always)]
    pub fn is_pc9(&self) -> bool {
        **self == EXTI9_A::PC9
    }
    ///Checks if the value of the field is `PD9`
    #[inline(always)]
    pub fn is_pd9(&self) -> bool {
        **self == EXTI9_A::PD9
    }
    ///Checks if the value of the field is `PE9`
    #[inline(always)]
    pub fn is_pe9(&self) -> bool {
        **self == EXTI9_A::PE9
    }
    ///Checks if the value of the field is `PH9`
    #[inline(always)]
    pub fn is_ph9(&self) -> bool {
        **self == EXTI9_A::PH9
    }
}
impl core::ops::Deref for EXTI9_R {
    type Target = crate::FieldReader<u8, EXTI9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EXTI9` writer - EXTI x configuration (x = 8 to 11)
pub struct EXTI9_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI9_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EXTI9_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Select PA9 as the source input for the EXTI9 external interrupt
    #[inline(always)]
    pub fn pa9(self) -> &'a mut W {
        self.variant(EXTI9_A::PA9)
    }
    ///Select PB9 as the source input for the EXTI9 external interrupt
    #[inline(always)]
    pub fn pb9(self) -> &'a mut W {
        self.variant(EXTI9_A::PB9)
    }
    ///Select PC9 as the source input for the EXTI9 external interrupt
    #[inline(always)]
    pub fn pc9(self) -> &'a mut W {
        self.variant(EXTI9_A::PC9)
    }
    ///Select PD9 as the source input for the EXTI9 external interrupt
    #[inline(always)]
    pub fn pd9(self) -> &'a mut W {
        self.variant(EXTI9_A::PD9)
    }
    ///Select PE9 as the source input for the EXTI9 external interrupt
    #[inline(always)]
    pub fn pe9(self) -> &'a mut W {
        self.variant(EXTI9_A::PE9)
    }
    ///Select PH9 as the source input for the EXTI9 external interrupt
    #[inline(always)]
    pub fn ph9(self) -> &'a mut W {
        self.variant(EXTI9_A::PH9)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
///EXTI x configuration (x = 8 to 11)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI8_A {
    ///0: Select PA8 as the source input for the EXTI8 external interrupt
    PA8 = 0,
    ///1: Select PB8 as the source input for the EXTI8 external interrupt
    PB8 = 1,
    ///2: Select PC8 as the source input for the EXTI8 external interrupt
    PC8 = 2,
    ///3: Select PD8 as the source input for the EXTI8 external interrupt
    PD8 = 3,
    ///4: Select PE8 as the source input for the EXTI8 external interrupt
    PE8 = 4,
}
impl From<EXTI8_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI8_A) -> Self {
        variant as _
    }
}
///Field `EXTI8` reader - EXTI x configuration (x = 8 to 11)
pub struct EXTI8_R(crate::FieldReader<u8, EXTI8_A>);
impl EXTI8_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EXTI8_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI8_A> {
        match self.bits {
            0 => Some(EXTI8_A::PA8),
            1 => Some(EXTI8_A::PB8),
            2 => Some(EXTI8_A::PC8),
            3 => Some(EXTI8_A::PD8),
            4 => Some(EXTI8_A::PE8),
            _ => None,
        }
    }
    ///Checks if the value of the field is `PA8`
    #[inline(always)]
    pub fn is_pa8(&self) -> bool {
        **self == EXTI8_A::PA8
    }
    ///Checks if the value of the field is `PB8`
    #[inline(always)]
    pub fn is_pb8(&self) -> bool {
        **self == EXTI8_A::PB8
    }
    ///Checks if the value of the field is `PC8`
    #[inline(always)]
    pub fn is_pc8(&self) -> bool {
        **self == EXTI8_A::PC8
    }
    ///Checks if the value of the field is `PD8`
    #[inline(always)]
    pub fn is_pd8(&self) -> bool {
        **self == EXTI8_A::PD8
    }
    ///Checks if the value of the field is `PE8`
    #[inline(always)]
    pub fn is_pe8(&self) -> bool {
        **self == EXTI8_A::PE8
    }
}
impl core::ops::Deref for EXTI8_R {
    type Target = crate::FieldReader<u8, EXTI8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EXTI8` writer - EXTI x configuration (x = 8 to 11)
pub struct EXTI8_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI8_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EXTI8_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Select PA8 as the source input for the EXTI8 external interrupt
    #[inline(always)]
    pub fn pa8(self) -> &'a mut W {
        self.variant(EXTI8_A::PA8)
    }
    ///Select PB8 as the source input for the EXTI8 external interrupt
    #[inline(always)]
    pub fn pb8(self) -> &'a mut W {
        self.variant(EXTI8_A::PB8)
    }
    ///Select PC8 as the source input for the EXTI8 external interrupt
    #[inline(always)]
    pub fn pc8(self) -> &'a mut W {
        self.variant(EXTI8_A::PC8)
    }
    ///Select PD8 as the source input for the EXTI8 external interrupt
    #[inline(always)]
    pub fn pd8(self) -> &'a mut W {
        self.variant(EXTI8_A::PD8)
    }
    ///Select PE8 as the source input for the EXTI8 external interrupt
    #[inline(always)]
    pub fn pe8(self) -> &'a mut W {
        self.variant(EXTI8_A::PE8)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    ///Bits 12:15 - EXTI x configuration (x = 8 to 11)
    #[inline(always)]
    pub fn exti11(&self) -> EXTI11_R {
        EXTI11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 8:11 - EXTI10
    #[inline(always)]
    pub fn exti10(&self) -> EXTI10_R {
        EXTI10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 4:7 - EXTI x configuration (x = 8 to 11)
    #[inline(always)]
    pub fn exti9(&self) -> EXTI9_R {
        EXTI9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 0:3 - EXTI x configuration (x = 8 to 11)
    #[inline(always)]
    pub fn exti8(&self) -> EXTI8_R {
        EXTI8_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    ///Bits 12:15 - EXTI x configuration (x = 8 to 11)
    #[inline(always)]
    pub fn exti11(&mut self) -> EXTI11_W {
        EXTI11_W { w: self }
    }
    ///Bits 8:11 - EXTI10
    #[inline(always)]
    pub fn exti10(&mut self) -> EXTI10_W {
        EXTI10_W { w: self }
    }
    ///Bits 4:7 - EXTI x configuration (x = 8 to 11)
    #[inline(always)]
    pub fn exti9(&mut self) -> EXTI9_W {
        EXTI9_W { w: self }
    }
    ///Bits 0:3 - EXTI x configuration (x = 8 to 11)
    #[inline(always)]
    pub fn exti8(&mut self) -> EXTI8_W {
        EXTI8_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///external interrupt configuration register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [exticr3](index.html) module
pub struct EXTICR3_SPEC;
impl crate::RegisterSpec for EXTICR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [exticr3::R](R) reader structure
impl crate::Readable for EXTICR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [exticr3::W](W) writer structure
impl crate::Writable for EXTICR3_SPEC {
    type Writer = W;
}
///`reset()` method sets EXTICR3 to value 0
impl crate::Resettable for EXTICR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
