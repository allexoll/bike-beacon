///Register `EXTICR4` reader
pub struct R(crate::R<EXTICR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTICR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTICR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTICR4_SPEC>) -> Self {
        R(reader)
    }
}
///Register `EXTICR4` writer
pub struct W(crate::W<EXTICR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTICR4_SPEC>;
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
impl From<crate::W<EXTICR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTICR4_SPEC>) -> Self {
        W(writer)
    }
}
///EXTI x configuration (x = 12 to 15)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI15_A {
    ///0: Select PA15 as the source input for the EXTI15 external interrupt
    PA15 = 0,
    ///1: Select PB15 as the source input for the EXTI15 external interrupt
    PB15 = 1,
    ///2: Select PC15 as the source input for the EXTI15 external interrupt
    PC15 = 2,
    ///3: Select PD15 as the source input for the EXTI15 external interrupt
    PD15 = 3,
    ///4: Select PE15 as the source input for the EXTI15 external interrupt
    PE15 = 4,
}
impl From<EXTI15_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI15_A) -> Self {
        variant as _
    }
}
///Field `EXTI15` reader - EXTI x configuration (x = 12 to 15)
pub struct EXTI15_R(crate::FieldReader<u8, EXTI15_A>);
impl EXTI15_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EXTI15_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI15_A> {
        match self.bits {
            0 => Some(EXTI15_A::PA15),
            1 => Some(EXTI15_A::PB15),
            2 => Some(EXTI15_A::PC15),
            3 => Some(EXTI15_A::PD15),
            4 => Some(EXTI15_A::PE15),
            _ => None,
        }
    }
    ///Checks if the value of the field is `PA15`
    #[inline(always)]
    pub fn is_pa15(&self) -> bool {
        **self == EXTI15_A::PA15
    }
    ///Checks if the value of the field is `PB15`
    #[inline(always)]
    pub fn is_pb15(&self) -> bool {
        **self == EXTI15_A::PB15
    }
    ///Checks if the value of the field is `PC15`
    #[inline(always)]
    pub fn is_pc15(&self) -> bool {
        **self == EXTI15_A::PC15
    }
    ///Checks if the value of the field is `PD15`
    #[inline(always)]
    pub fn is_pd15(&self) -> bool {
        **self == EXTI15_A::PD15
    }
    ///Checks if the value of the field is `PE15`
    #[inline(always)]
    pub fn is_pe15(&self) -> bool {
        **self == EXTI15_A::PE15
    }
}
impl core::ops::Deref for EXTI15_R {
    type Target = crate::FieldReader<u8, EXTI15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EXTI15` writer - EXTI x configuration (x = 12 to 15)
pub struct EXTI15_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI15_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EXTI15_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Select PA15 as the source input for the EXTI15 external interrupt
    #[inline(always)]
    pub fn pa15(self) -> &'a mut W {
        self.variant(EXTI15_A::PA15)
    }
    ///Select PB15 as the source input for the EXTI15 external interrupt
    #[inline(always)]
    pub fn pb15(self) -> &'a mut W {
        self.variant(EXTI15_A::PB15)
    }
    ///Select PC15 as the source input for the EXTI15 external interrupt
    #[inline(always)]
    pub fn pc15(self) -> &'a mut W {
        self.variant(EXTI15_A::PC15)
    }
    ///Select PD15 as the source input for the EXTI15 external interrupt
    #[inline(always)]
    pub fn pd15(self) -> &'a mut W {
        self.variant(EXTI15_A::PD15)
    }
    ///Select PE15 as the source input for the EXTI15 external interrupt
    #[inline(always)]
    pub fn pe15(self) -> &'a mut W {
        self.variant(EXTI15_A::PE15)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
///EXTI14
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI14_A {
    ///0: Select PA14 as the source input for the EXTI14 external interrupt
    PA14 = 0,
    ///1: Select PB14 as the source input for the EXTI14 external interrupt
    PB14 = 1,
    ///2: Select PC14 as the source input for the EXTI14 external interrupt
    PC14 = 2,
    ///3: Select PD14 as the source input for the EXTI14 external interrupt
    PD14 = 3,
    ///4: Select PE14 as the source input for the EXTI14 external interrupt
    PE14 = 4,
}
impl From<EXTI14_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI14_A) -> Self {
        variant as _
    }
}
///Field `EXTI14` reader - EXTI14
pub struct EXTI14_R(crate::FieldReader<u8, EXTI14_A>);
impl EXTI14_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EXTI14_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI14_A> {
        match self.bits {
            0 => Some(EXTI14_A::PA14),
            1 => Some(EXTI14_A::PB14),
            2 => Some(EXTI14_A::PC14),
            3 => Some(EXTI14_A::PD14),
            4 => Some(EXTI14_A::PE14),
            _ => None,
        }
    }
    ///Checks if the value of the field is `PA14`
    #[inline(always)]
    pub fn is_pa14(&self) -> bool {
        **self == EXTI14_A::PA14
    }
    ///Checks if the value of the field is `PB14`
    #[inline(always)]
    pub fn is_pb14(&self) -> bool {
        **self == EXTI14_A::PB14
    }
    ///Checks if the value of the field is `PC14`
    #[inline(always)]
    pub fn is_pc14(&self) -> bool {
        **self == EXTI14_A::PC14
    }
    ///Checks if the value of the field is `PD14`
    #[inline(always)]
    pub fn is_pd14(&self) -> bool {
        **self == EXTI14_A::PD14
    }
    ///Checks if the value of the field is `PE14`
    #[inline(always)]
    pub fn is_pe14(&self) -> bool {
        **self == EXTI14_A::PE14
    }
}
impl core::ops::Deref for EXTI14_R {
    type Target = crate::FieldReader<u8, EXTI14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EXTI14` writer - EXTI14
pub struct EXTI14_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI14_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EXTI14_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Select PA14 as the source input for the EXTI14 external interrupt
    #[inline(always)]
    pub fn pa14(self) -> &'a mut W {
        self.variant(EXTI14_A::PA14)
    }
    ///Select PB14 as the source input for the EXTI14 external interrupt
    #[inline(always)]
    pub fn pb14(self) -> &'a mut W {
        self.variant(EXTI14_A::PB14)
    }
    ///Select PC14 as the source input for the EXTI14 external interrupt
    #[inline(always)]
    pub fn pc14(self) -> &'a mut W {
        self.variant(EXTI14_A::PC14)
    }
    ///Select PD14 as the source input for the EXTI14 external interrupt
    #[inline(always)]
    pub fn pd14(self) -> &'a mut W {
        self.variant(EXTI14_A::PD14)
    }
    ///Select PE14 as the source input for the EXTI14 external interrupt
    #[inline(always)]
    pub fn pe14(self) -> &'a mut W {
        self.variant(EXTI14_A::PE14)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
///EXTI13
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI13_A {
    ///0: Select PA13 as the source input for the EXTI13 external interrupt
    PA13 = 0,
    ///1: Select PB13 as the source input for the EXTI13 external interrupt
    PB13 = 1,
    ///2: Select PC13 as the source input for the EXTI13 external interrupt
    PC13 = 2,
    ///3: Select PD13 as the source input for the EXTI13 external interrupt
    PD13 = 3,
    ///4: Select PE13 as the source input for the EXTI13 external interrupt
    PE13 = 4,
}
impl From<EXTI13_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI13_A) -> Self {
        variant as _
    }
}
///Field `EXTI13` reader - EXTI13
pub struct EXTI13_R(crate::FieldReader<u8, EXTI13_A>);
impl EXTI13_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EXTI13_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI13_A> {
        match self.bits {
            0 => Some(EXTI13_A::PA13),
            1 => Some(EXTI13_A::PB13),
            2 => Some(EXTI13_A::PC13),
            3 => Some(EXTI13_A::PD13),
            4 => Some(EXTI13_A::PE13),
            _ => None,
        }
    }
    ///Checks if the value of the field is `PA13`
    #[inline(always)]
    pub fn is_pa13(&self) -> bool {
        **self == EXTI13_A::PA13
    }
    ///Checks if the value of the field is `PB13`
    #[inline(always)]
    pub fn is_pb13(&self) -> bool {
        **self == EXTI13_A::PB13
    }
    ///Checks if the value of the field is `PC13`
    #[inline(always)]
    pub fn is_pc13(&self) -> bool {
        **self == EXTI13_A::PC13
    }
    ///Checks if the value of the field is `PD13`
    #[inline(always)]
    pub fn is_pd13(&self) -> bool {
        **self == EXTI13_A::PD13
    }
    ///Checks if the value of the field is `PE13`
    #[inline(always)]
    pub fn is_pe13(&self) -> bool {
        **self == EXTI13_A::PE13
    }
}
impl core::ops::Deref for EXTI13_R {
    type Target = crate::FieldReader<u8, EXTI13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EXTI13` writer - EXTI13
pub struct EXTI13_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI13_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EXTI13_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Select PA13 as the source input for the EXTI13 external interrupt
    #[inline(always)]
    pub fn pa13(self) -> &'a mut W {
        self.variant(EXTI13_A::PA13)
    }
    ///Select PB13 as the source input for the EXTI13 external interrupt
    #[inline(always)]
    pub fn pb13(self) -> &'a mut W {
        self.variant(EXTI13_A::PB13)
    }
    ///Select PC13 as the source input for the EXTI13 external interrupt
    #[inline(always)]
    pub fn pc13(self) -> &'a mut W {
        self.variant(EXTI13_A::PC13)
    }
    ///Select PD13 as the source input for the EXTI13 external interrupt
    #[inline(always)]
    pub fn pd13(self) -> &'a mut W {
        self.variant(EXTI13_A::PD13)
    }
    ///Select PE13 as the source input for the EXTI13 external interrupt
    #[inline(always)]
    pub fn pe13(self) -> &'a mut W {
        self.variant(EXTI13_A::PE13)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
///EXTI12
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI12_A {
    ///0: Select PA12 as the source input for the EXTI12 external interrupt
    PA12 = 0,
    ///1: Select PB12 as the source input for the EXTI12 external interrupt
    PB12 = 1,
    ///2: Select PC12 as the source input for the EXTI12 external interrupt
    PC12 = 2,
    ///3: Select PD12 as the source input for the EXTI12 external interrupt
    PD12 = 3,
    ///4: Select PE12 as the source input for the EXTI12 external interrupt
    PE12 = 4,
}
impl From<EXTI12_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI12_A) -> Self {
        variant as _
    }
}
///Field `EXTI12` reader - EXTI12
pub struct EXTI12_R(crate::FieldReader<u8, EXTI12_A>);
impl EXTI12_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EXTI12_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI12_A> {
        match self.bits {
            0 => Some(EXTI12_A::PA12),
            1 => Some(EXTI12_A::PB12),
            2 => Some(EXTI12_A::PC12),
            3 => Some(EXTI12_A::PD12),
            4 => Some(EXTI12_A::PE12),
            _ => None,
        }
    }
    ///Checks if the value of the field is `PA12`
    #[inline(always)]
    pub fn is_pa12(&self) -> bool {
        **self == EXTI12_A::PA12
    }
    ///Checks if the value of the field is `PB12`
    #[inline(always)]
    pub fn is_pb12(&self) -> bool {
        **self == EXTI12_A::PB12
    }
    ///Checks if the value of the field is `PC12`
    #[inline(always)]
    pub fn is_pc12(&self) -> bool {
        **self == EXTI12_A::PC12
    }
    ///Checks if the value of the field is `PD12`
    #[inline(always)]
    pub fn is_pd12(&self) -> bool {
        **self == EXTI12_A::PD12
    }
    ///Checks if the value of the field is `PE12`
    #[inline(always)]
    pub fn is_pe12(&self) -> bool {
        **self == EXTI12_A::PE12
    }
}
impl core::ops::Deref for EXTI12_R {
    type Target = crate::FieldReader<u8, EXTI12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EXTI12` writer - EXTI12
pub struct EXTI12_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI12_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EXTI12_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Select PA12 as the source input for the EXTI12 external interrupt
    #[inline(always)]
    pub fn pa12(self) -> &'a mut W {
        self.variant(EXTI12_A::PA12)
    }
    ///Select PB12 as the source input for the EXTI12 external interrupt
    #[inline(always)]
    pub fn pb12(self) -> &'a mut W {
        self.variant(EXTI12_A::PB12)
    }
    ///Select PC12 as the source input for the EXTI12 external interrupt
    #[inline(always)]
    pub fn pc12(self) -> &'a mut W {
        self.variant(EXTI12_A::PC12)
    }
    ///Select PD12 as the source input for the EXTI12 external interrupt
    #[inline(always)]
    pub fn pd12(self) -> &'a mut W {
        self.variant(EXTI12_A::PD12)
    }
    ///Select PE12 as the source input for the EXTI12 external interrupt
    #[inline(always)]
    pub fn pe12(self) -> &'a mut W {
        self.variant(EXTI12_A::PE12)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    ///Bits 12:15 - EXTI x configuration (x = 12 to 15)
    #[inline(always)]
    pub fn exti15(&self) -> EXTI15_R {
        EXTI15_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 8:11 - EXTI14
    #[inline(always)]
    pub fn exti14(&self) -> EXTI14_R {
        EXTI14_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 4:7 - EXTI13
    #[inline(always)]
    pub fn exti13(&self) -> EXTI13_R {
        EXTI13_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 0:3 - EXTI12
    #[inline(always)]
    pub fn exti12(&self) -> EXTI12_R {
        EXTI12_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    ///Bits 12:15 - EXTI x configuration (x = 12 to 15)
    #[inline(always)]
    pub fn exti15(&mut self) -> EXTI15_W {
        EXTI15_W { w: self }
    }
    ///Bits 8:11 - EXTI14
    #[inline(always)]
    pub fn exti14(&mut self) -> EXTI14_W {
        EXTI14_W { w: self }
    }
    ///Bits 4:7 - EXTI13
    #[inline(always)]
    pub fn exti13(&mut self) -> EXTI13_W {
        EXTI13_W { w: self }
    }
    ///Bits 0:3 - EXTI12
    #[inline(always)]
    pub fn exti12(&mut self) -> EXTI12_W {
        EXTI12_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///external interrupt configuration register 4
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [exticr4](index.html) module
pub struct EXTICR4_SPEC;
impl crate::RegisterSpec for EXTICR4_SPEC {
    type Ux = u32;
}
///`read()` method returns [exticr4::R](R) reader structure
impl crate::Readable for EXTICR4_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [exticr4::W](W) writer structure
impl crate::Writable for EXTICR4_SPEC {
    type Writer = W;
}
///`reset()` method sets EXTICR4 to value 0
impl crate::Resettable for EXTICR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
