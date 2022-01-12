///Register `MODER` reader
pub struct R(crate::R<MODER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MODER` writer
pub struct W(crate::W<MODER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODER_SPEC>;
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
impl From<crate::W<MODER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODER_SPEC>) -> Self {
        W(writer)
    }
}
///Port x configuration bits (y = 0..15)
///
///Value on reset: 3
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE0_A {
    ///0: Input mode (reset state)
    INPUT = 0,
    ///1: General purpose output mode
    OUTPUT = 1,
    ///2: Alternate function mode
    ALTERNATE = 2,
    ///3: Analog mode
    ANALOG = 3,
}
impl From<MODE0_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE0_A) -> Self {
        variant as _
    }
}
///Field `MODE0` reader - Port x configuration bits (y = 0..15)
pub struct MODE0_R(crate::FieldReader<u8, MODE0_A>);
impl MODE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MODE0_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MODE0_A {
        match self.bits {
            0 => MODE0_A::INPUT,
            1 => MODE0_A::OUTPUT,
            2 => MODE0_A::ALTERNATE,
            3 => MODE0_A::ANALOG,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `INPUT`
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        **self == MODE0_A::INPUT
    }
    ///Checks if the value of the field is `OUTPUT`
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        **self == MODE0_A::OUTPUT
    }
    ///Checks if the value of the field is `ALTERNATE`
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        **self == MODE0_A::ALTERNATE
    }
    ///Checks if the value of the field is `ANALOG`
    #[inline(always)]
    pub fn is_analog(&self) -> bool {
        **self == MODE0_A::ANALOG
    }
}
impl core::ops::Deref for MODE0_R {
    type Target = crate::FieldReader<u8, MODE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MODE0` writer - Port x configuration bits (y = 0..15)
pub struct MODE0_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE0_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MODE0_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Input mode (reset state)
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE0_A::INPUT)
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE0_A::OUTPUT)
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(MODE0_A::ALTERNATE)
    }
    ///Analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(MODE0_A::ANALOG)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
///Port x configuration bits (y = 0..15)
pub type MODE1_A = MODE0_A;
///Field `MODE1` reader - Port x configuration bits (y = 0..15)
pub type MODE1_R = MODE0_R;
///Field `MODE1` writer - Port x configuration bits (y = 0..15)
pub struct MODE1_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MODE1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Input mode (reset state)
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE1_A::INPUT)
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE1_A::OUTPUT)
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(MODE1_A::ALTERNATE)
    }
    ///Analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(MODE1_A::ANALOG)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
///Port x configuration bits (y = 0..15)
pub type MODE2_A = MODE0_A;
///Field `MODE2` reader - Port x configuration bits (y = 0..15)
pub type MODE2_R = MODE0_R;
///Field `MODE2` writer - Port x configuration bits (y = 0..15)
pub struct MODE2_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MODE2_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Input mode (reset state)
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE2_A::INPUT)
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE2_A::OUTPUT)
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(MODE2_A::ALTERNATE)
    }
    ///Analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(MODE2_A::ANALOG)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
///Port x configuration bits (y = 0..15)
pub type MODE3_A = MODE0_A;
///Field `MODE3` reader - Port x configuration bits (y = 0..15)
pub type MODE3_R = MODE0_R;
///Field `MODE3` writer - Port x configuration bits (y = 0..15)
pub struct MODE3_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE3_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MODE3_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Input mode (reset state)
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE3_A::INPUT)
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE3_A::OUTPUT)
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(MODE3_A::ALTERNATE)
    }
    ///Analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(MODE3_A::ANALOG)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
///Port x configuration bits (y = 0..15)
pub type MODE4_A = MODE0_A;
///Field `MODE4` reader - Port x configuration bits (y = 0..15)
pub type MODE4_R = MODE0_R;
///Field `MODE4` writer - Port x configuration bits (y = 0..15)
pub struct MODE4_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE4_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MODE4_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Input mode (reset state)
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE4_A::INPUT)
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE4_A::OUTPUT)
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(MODE4_A::ALTERNATE)
    }
    ///Analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(MODE4_A::ANALOG)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
///Port x configuration bits (y = 0..15)
pub type MODE5_A = MODE0_A;
///Field `MODE5` reader - Port x configuration bits (y = 0..15)
pub type MODE5_R = MODE0_R;
///Field `MODE5` writer - Port x configuration bits (y = 0..15)
pub struct MODE5_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE5_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MODE5_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Input mode (reset state)
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE5_A::INPUT)
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE5_A::OUTPUT)
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(MODE5_A::ALTERNATE)
    }
    ///Analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(MODE5_A::ANALOG)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
///Port x configuration bits (y = 0..15)
pub type MODE6_A = MODE0_A;
///Field `MODE6` reader - Port x configuration bits (y = 0..15)
pub type MODE6_R = MODE0_R;
///Field `MODE6` writer - Port x configuration bits (y = 0..15)
pub struct MODE6_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE6_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MODE6_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Input mode (reset state)
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE6_A::INPUT)
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE6_A::OUTPUT)
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(MODE6_A::ALTERNATE)
    }
    ///Analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(MODE6_A::ANALOG)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
///Port x configuration bits (y = 0..15)
pub type MODE7_A = MODE0_A;
///Field `MODE7` reader - Port x configuration bits (y = 0..15)
pub type MODE7_R = MODE0_R;
///Field `MODE7` writer - Port x configuration bits (y = 0..15)
pub struct MODE7_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE7_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MODE7_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Input mode (reset state)
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE7_A::INPUT)
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE7_A::OUTPUT)
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(MODE7_A::ALTERNATE)
    }
    ///Analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(MODE7_A::ANALOG)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
///Port x configuration bits (y = 0..15)
pub type MODE8_A = MODE0_A;
///Field `MODE8` reader - Port x configuration bits (y = 0..15)
pub type MODE8_R = MODE0_R;
///Field `MODE8` writer - Port x configuration bits (y = 0..15)
pub struct MODE8_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE8_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MODE8_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Input mode (reset state)
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE8_A::INPUT)
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE8_A::OUTPUT)
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(MODE8_A::ALTERNATE)
    }
    ///Analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(MODE8_A::ANALOG)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
///Port x configuration bits (y = 0..15)
pub type MODE9_A = MODE0_A;
///Field `MODE9` reader - Port x configuration bits (y = 0..15)
pub type MODE9_R = MODE0_R;
///Field `MODE9` writer - Port x configuration bits (y = 0..15)
pub struct MODE9_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE9_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MODE9_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Input mode (reset state)
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE9_A::INPUT)
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE9_A::OUTPUT)
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(MODE9_A::ALTERNATE)
    }
    ///Analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(MODE9_A::ANALOG)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
///Port x configuration bits (y = 0..15)
pub type MODE10_A = MODE0_A;
///Field `MODE10` reader - Port x configuration bits (y = 0..15)
pub type MODE10_R = MODE0_R;
///Field `MODE10` writer - Port x configuration bits (y = 0..15)
pub struct MODE10_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE10_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MODE10_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Input mode (reset state)
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE10_A::INPUT)
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE10_A::OUTPUT)
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(MODE10_A::ALTERNATE)
    }
    ///Analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(MODE10_A::ANALOG)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
///Port x configuration bits (y = 0..15)
pub type MODE11_A = MODE0_A;
///Field `MODE11` reader - Port x configuration bits (y = 0..15)
pub type MODE11_R = MODE0_R;
///Field `MODE11` writer - Port x configuration bits (y = 0..15)
pub struct MODE11_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE11_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MODE11_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Input mode (reset state)
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE11_A::INPUT)
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE11_A::OUTPUT)
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(MODE11_A::ALTERNATE)
    }
    ///Analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(MODE11_A::ANALOG)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
///Port x configuration bits (y = 0..15)
pub type MODE12_A = MODE0_A;
///Field `MODE12` reader - Port x configuration bits (y = 0..15)
pub type MODE12_R = MODE0_R;
///Field `MODE12` writer - Port x configuration bits (y = 0..15)
pub struct MODE12_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE12_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MODE12_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Input mode (reset state)
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE12_A::INPUT)
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE12_A::OUTPUT)
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(MODE12_A::ALTERNATE)
    }
    ///Analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(MODE12_A::ANALOG)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
///Port x configuration bits (y = 0..15)
pub type MODE13_A = MODE0_A;
///Field `MODE13` reader - Port x configuration bits (y = 0..15)
pub type MODE13_R = MODE0_R;
///Field `MODE13` writer - Port x configuration bits (y = 0..15)
pub struct MODE13_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE13_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MODE13_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Input mode (reset state)
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE13_A::INPUT)
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE13_A::OUTPUT)
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(MODE13_A::ALTERNATE)
    }
    ///Analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(MODE13_A::ANALOG)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
///Port x configuration bits (y = 0..15)
pub type MODE14_A = MODE0_A;
///Field `MODE14` reader - Port x configuration bits (y = 0..15)
pub type MODE14_R = MODE0_R;
///Field `MODE14` writer - Port x configuration bits (y = 0..15)
pub struct MODE14_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE14_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MODE14_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Input mode (reset state)
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE14_A::INPUT)
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE14_A::OUTPUT)
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(MODE14_A::ALTERNATE)
    }
    ///Analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(MODE14_A::ANALOG)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
///Port x configuration bits (y = 0..15)
pub type MODE15_A = MODE0_A;
///Field `MODE15` reader - Port x configuration bits (y = 0..15)
pub type MODE15_R = MODE0_R;
///Field `MODE15` writer - Port x configuration bits (y = 0..15)
pub struct MODE15_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE15_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MODE15_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Input mode (reset state)
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE15_A::INPUT)
    }
    ///General purpose output mode
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(MODE15_A::OUTPUT)
    }
    ///Alternate function mode
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(MODE15_A::ALTERNATE)
    }
    ///Analog mode
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(MODE15_A::ANALOG)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    ///Bits 0:1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode0(&self) -> MODE0_R {
        MODE0_R::new((self.bits & 0x03) as u8)
    }
    ///Bits 2:3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    ///Bits 4:5 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    ///Bits 6:7 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode3(&self) -> MODE3_R {
        MODE3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    ///Bits 8:9 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode4(&self) -> MODE4_R {
        MODE4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    ///Bits 10:11 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode5(&self) -> MODE5_R {
        MODE5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    ///Bits 12:13 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode6(&self) -> MODE6_R {
        MODE6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    ///Bits 14:15 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode7(&self) -> MODE7_R {
        MODE7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    ///Bits 16:17 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode8(&self) -> MODE8_R {
        MODE8_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    ///Bits 18:19 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode9(&self) -> MODE9_R {
        MODE9_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    ///Bits 20:21 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode10(&self) -> MODE10_R {
        MODE10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    ///Bits 22:23 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode11(&self) -> MODE11_R {
        MODE11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    ///Bits 24:25 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode12(&self) -> MODE12_R {
        MODE12_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    ///Bits 26:27 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode13(&self) -> MODE13_R {
        MODE13_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    ///Bits 28:29 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode14(&self) -> MODE14_R {
        MODE14_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    ///Bits 30:31 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode15(&self) -> MODE15_R {
        MODE15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    ///Bits 0:1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode0(&mut self) -> MODE0_W {
        MODE0_W { w: self }
    }
    ///Bits 2:3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode1(&mut self) -> MODE1_W {
        MODE1_W { w: self }
    }
    ///Bits 4:5 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode2(&mut self) -> MODE2_W {
        MODE2_W { w: self }
    }
    ///Bits 6:7 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode3(&mut self) -> MODE3_W {
        MODE3_W { w: self }
    }
    ///Bits 8:9 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode4(&mut self) -> MODE4_W {
        MODE4_W { w: self }
    }
    ///Bits 10:11 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode5(&mut self) -> MODE5_W {
        MODE5_W { w: self }
    }
    ///Bits 12:13 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode6(&mut self) -> MODE6_W {
        MODE6_W { w: self }
    }
    ///Bits 14:15 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode7(&mut self) -> MODE7_W {
        MODE7_W { w: self }
    }
    ///Bits 16:17 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode8(&mut self) -> MODE8_W {
        MODE8_W { w: self }
    }
    ///Bits 18:19 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode9(&mut self) -> MODE9_W {
        MODE9_W { w: self }
    }
    ///Bits 20:21 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode10(&mut self) -> MODE10_W {
        MODE10_W { w: self }
    }
    ///Bits 22:23 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode11(&mut self) -> MODE11_W {
        MODE11_W { w: self }
    }
    ///Bits 24:25 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode12(&mut self) -> MODE12_W {
        MODE12_W { w: self }
    }
    ///Bits 26:27 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode13(&mut self) -> MODE13_W {
        MODE13_W { w: self }
    }
    ///Bits 28:29 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode14(&mut self) -> MODE14_W {
        MODE14_W { w: self }
    }
    ///Bits 30:31 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn mode15(&mut self) -> MODE15_W {
        MODE15_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GPIO port mode register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [moder](index.html) module
pub struct MODER_SPEC;
impl crate::RegisterSpec for MODER_SPEC {
    type Ux = u32;
}
///`read()` method returns [moder::R](R) reader structure
impl crate::Readable for MODER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [moder::W](W) writer structure
impl crate::Writable for MODER_SPEC {
    type Writer = W;
}
///`reset()` method sets MODER to value 0xebff_fcff
impl crate::Resettable for MODER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xebff_fcff
    }
}
