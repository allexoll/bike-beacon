///Register `IOCCR` reader
pub struct R(crate::R<IOCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOCCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IOCCR` writer
pub struct W(crate::W<IOCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOCCR_SPEC>;
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
impl From<crate::W<IOCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOCCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `G8_IO4` reader - G8_IO4
pub struct G8_IO4_R(crate::FieldReader<bool, bool>);
impl G8_IO4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        G8_IO4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G8_IO4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `G8_IO4` writer - G8_IO4
pub struct G8_IO4_W<'a> {
    w: &'a mut W,
}
impl<'a> G8_IO4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
///Field `G8_IO3` reader - G8_IO3
pub struct G8_IO3_R(crate::FieldReader<bool, bool>);
impl G8_IO3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        G8_IO3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G8_IO3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `G8_IO3` writer - G8_IO3
pub struct G8_IO3_W<'a> {
    w: &'a mut W,
}
impl<'a> G8_IO3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
///Field `G8_IO2` reader - G8_IO2
pub struct G8_IO2_R(crate::FieldReader<bool, bool>);
impl G8_IO2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        G8_IO2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G8_IO2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `G8_IO2` writer - G8_IO2
pub struct G8_IO2_W<'a> {
    w: &'a mut W,
}
impl<'a> G8_IO2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
///Field `G8_IO1` reader - G8_IO1
pub struct G8_IO1_R(crate::FieldReader<bool, bool>);
impl G8_IO1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        G8_IO1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G8_IO1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `G8_IO1` writer - G8_IO1
pub struct G8_IO1_W<'a> {
    w: &'a mut W,
}
impl<'a> G8_IO1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
///Field `G7_IO4` reader - G7_IO4
pub struct G7_IO4_R(crate::FieldReader<bool, bool>);
impl G7_IO4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        G7_IO4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G7_IO4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `G7_IO4` writer - G7_IO4
pub struct G7_IO4_W<'a> {
    w: &'a mut W,
}
impl<'a> G7_IO4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
///Field `G7_IO3` reader - G7_IO3
pub struct G7_IO3_R(crate::FieldReader<bool, bool>);
impl G7_IO3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        G7_IO3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G7_IO3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `G7_IO3` writer - G7_IO3
pub struct G7_IO3_W<'a> {
    w: &'a mut W,
}
impl<'a> G7_IO3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
///Field `G7_IO2` reader - G7_IO2
pub struct G7_IO2_R(crate::FieldReader<bool, bool>);
impl G7_IO2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        G7_IO2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G7_IO2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `G7_IO2` writer - G7_IO2
pub struct G7_IO2_W<'a> {
    w: &'a mut W,
}
impl<'a> G7_IO2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
///Field `G7_IO1` reader - G7_IO1
pub struct G7_IO1_R(crate::FieldReader<bool, bool>);
impl G7_IO1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        G7_IO1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G7_IO1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `G7_IO1` writer - G7_IO1
pub struct G7_IO1_W<'a> {
    w: &'a mut W,
}
impl<'a> G7_IO1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
///Field `G6_IO4` reader - G6_IO4
pub struct G6_IO4_R(crate::FieldReader<bool, bool>);
impl G6_IO4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        G6_IO4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G6_IO4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `G6_IO4` writer - G6_IO4
pub struct G6_IO4_W<'a> {
    w: &'a mut W,
}
impl<'a> G6_IO4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
///Field `G6_IO3` reader - G6_IO3
pub struct G6_IO3_R(crate::FieldReader<bool, bool>);
impl G6_IO3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        G6_IO3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G6_IO3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `G6_IO3` writer - G6_IO3
pub struct G6_IO3_W<'a> {
    w: &'a mut W,
}
impl<'a> G6_IO3_W<'a> {
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
///Field `G6_IO2` reader - G6_IO2
pub struct G6_IO2_R(crate::FieldReader<bool, bool>);
impl G6_IO2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        G6_IO2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G6_IO2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `G6_IO2` writer - G6_IO2
pub struct G6_IO2_W<'a> {
    w: &'a mut W,
}
impl<'a> G6_IO2_W<'a> {
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
///Field `G6_IO1` reader - G6_IO1
pub struct G6_IO1_R(crate::FieldReader<bool, bool>);
impl G6_IO1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        G6_IO1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G6_IO1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `G6_IO1` writer - G6_IO1
pub struct G6_IO1_W<'a> {
    w: &'a mut W,
}
impl<'a> G6_IO1_W<'a> {
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
///Field `G5_IO4` reader - G5_IO4
pub struct G5_IO4_R(crate::FieldReader<bool, bool>);
impl G5_IO4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        G5_IO4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G5_IO4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `G5_IO4` writer - G5_IO4
pub struct G5_IO4_W<'a> {
    w: &'a mut W,
}
impl<'a> G5_IO4_W<'a> {
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
///Field `G5_IO3` reader - G5_IO3
pub struct G5_IO3_R(crate::FieldReader<bool, bool>);
impl G5_IO3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        G5_IO3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G5_IO3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `G5_IO3` writer - G5_IO3
pub struct G5_IO3_W<'a> {
    w: &'a mut W,
}
impl<'a> G5_IO3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
///Field `G5_IO2` reader - G5_IO2
pub struct G5_IO2_R(crate::FieldReader<bool, bool>);
impl G5_IO2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        G5_IO2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G5_IO2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `G5_IO2` writer - G5_IO2
pub struct G5_IO2_W<'a> {
    w: &'a mut W,
}
impl<'a> G5_IO2_W<'a> {
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
///Field `G5_IO1` reader - G5_IO1
pub struct G5_IO1_R(crate::FieldReader<bool, bool>);
impl G5_IO1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        G5_IO1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G5_IO1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `G5_IO1` writer - G5_IO1
pub struct G5_IO1_W<'a> {
    w: &'a mut W,
}
impl<'a> G5_IO1_W<'a> {
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
///Field `G4_IO4` reader - G4_IO4
pub struct G4_IO4_R(crate::FieldReader<bool, bool>);
impl G4_IO4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        G4_IO4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G4_IO4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `G4_IO4` writer - G4_IO4
pub struct G4_IO4_W<'a> {
    w: &'a mut W,
}
impl<'a> G4_IO4_W<'a> {
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
///Field `G4_IO3` reader - G4_IO3
pub struct G4_IO3_R(crate::FieldReader<bool, bool>);
impl G4_IO3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        G4_IO3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G4_IO3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `G4_IO3` writer - G4_IO3
pub struct G4_IO3_W<'a> {
    w: &'a mut W,
}
impl<'a> G4_IO3_W<'a> {
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
///Field `G4_IO2` reader - G4_IO2
pub struct G4_IO2_R(crate::FieldReader<bool, bool>);
impl G4_IO2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        G4_IO2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G4_IO2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `G4_IO2` writer - G4_IO2
pub struct G4_IO2_W<'a> {
    w: &'a mut W,
}
impl<'a> G4_IO2_W<'a> {
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
///Field `G4_IO1` reader - G4_IO1
pub struct G4_IO1_R(crate::FieldReader<bool, bool>);
impl G4_IO1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        G4_IO1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G4_IO1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `G4_IO1` writer - G4_IO1
pub struct G4_IO1_W<'a> {
    w: &'a mut W,
}
impl<'a> G4_IO1_W<'a> {
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
///Field `G3_IO4` reader - G3_IO4
pub struct G3_IO4_R(crate::FieldReader<bool, bool>);
impl G3_IO4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        G3_IO4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G3_IO4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `G3_IO4` writer - G3_IO4
pub struct G3_IO4_W<'a> {
    w: &'a mut W,
}
impl<'a> G3_IO4_W<'a> {
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
///Field `G3_IO3` reader - G3_IO3
pub struct G3_IO3_R(crate::FieldReader<bool, bool>);
impl G3_IO3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        G3_IO3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G3_IO3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `G3_IO3` writer - G3_IO3
pub struct G3_IO3_W<'a> {
    w: &'a mut W,
}
impl<'a> G3_IO3_W<'a> {
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
///Field `G3_IO2` reader - G3_IO2
pub struct G3_IO2_R(crate::FieldReader<bool, bool>);
impl G3_IO2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        G3_IO2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G3_IO2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `G3_IO2` writer - G3_IO2
pub struct G3_IO2_W<'a> {
    w: &'a mut W,
}
impl<'a> G3_IO2_W<'a> {
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
///Field `G3_IO1` reader - G3_IO1
pub struct G3_IO1_R(crate::FieldReader<bool, bool>);
impl G3_IO1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        G3_IO1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G3_IO1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `G3_IO1` writer - G3_IO1
pub struct G3_IO1_W<'a> {
    w: &'a mut W,
}
impl<'a> G3_IO1_W<'a> {
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
///Field `G2_IO4` reader - G2_IO4
pub struct G2_IO4_R(crate::FieldReader<bool, bool>);
impl G2_IO4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        G2_IO4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G2_IO4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `G2_IO4` writer - G2_IO4
pub struct G2_IO4_W<'a> {
    w: &'a mut W,
}
impl<'a> G2_IO4_W<'a> {
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
///Field `G2_IO3` reader - G2_IO3
pub struct G2_IO3_R(crate::FieldReader<bool, bool>);
impl G2_IO3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        G2_IO3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G2_IO3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `G2_IO3` writer - G2_IO3
pub struct G2_IO3_W<'a> {
    w: &'a mut W,
}
impl<'a> G2_IO3_W<'a> {
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
///Field `G2_IO2` reader - G2_IO2
pub struct G2_IO2_R(crate::FieldReader<bool, bool>);
impl G2_IO2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        G2_IO2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G2_IO2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `G2_IO2` writer - G2_IO2
pub struct G2_IO2_W<'a> {
    w: &'a mut W,
}
impl<'a> G2_IO2_W<'a> {
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
///Field `G2_IO1` reader - G2_IO1
pub struct G2_IO1_R(crate::FieldReader<bool, bool>);
impl G2_IO1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        G2_IO1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G2_IO1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `G2_IO1` writer - G2_IO1
pub struct G2_IO1_W<'a> {
    w: &'a mut W,
}
impl<'a> G2_IO1_W<'a> {
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
///Field `G1_IO4` reader - G1_IO4
pub struct G1_IO4_R(crate::FieldReader<bool, bool>);
impl G1_IO4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        G1_IO4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G1_IO4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `G1_IO4` writer - G1_IO4
pub struct G1_IO4_W<'a> {
    w: &'a mut W,
}
impl<'a> G1_IO4_W<'a> {
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
///Field `G1_IO3` reader - G1_IO3
pub struct G1_IO3_R(crate::FieldReader<bool, bool>);
impl G1_IO3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        G1_IO3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G1_IO3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `G1_IO3` writer - G1_IO3
pub struct G1_IO3_W<'a> {
    w: &'a mut W,
}
impl<'a> G1_IO3_W<'a> {
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
///Field `G1_IO2` reader - G1_IO2
pub struct G1_IO2_R(crate::FieldReader<bool, bool>);
impl G1_IO2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        G1_IO2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G1_IO2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `G1_IO2` writer - G1_IO2
pub struct G1_IO2_W<'a> {
    w: &'a mut W,
}
impl<'a> G1_IO2_W<'a> {
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
///Field `G1_IO1` reader - G1_IO1
pub struct G1_IO1_R(crate::FieldReader<bool, bool>);
impl G1_IO1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        G1_IO1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G1_IO1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `G1_IO1` writer - G1_IO1
pub struct G1_IO1_W<'a> {
    w: &'a mut W,
}
impl<'a> G1_IO1_W<'a> {
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
    ///Bit 31 - G8_IO4
    #[inline(always)]
    pub fn g8_io4(&self) -> G8_IO4_R {
        G8_IO4_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    ///Bit 30 - G8_IO3
    #[inline(always)]
    pub fn g8_io3(&self) -> G8_IO3_R {
        G8_IO3_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 29 - G8_IO2
    #[inline(always)]
    pub fn g8_io2(&self) -> G8_IO2_R {
        G8_IO2_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    ///Bit 28 - G8_IO1
    #[inline(always)]
    pub fn g8_io1(&self) -> G8_IO1_R {
        G8_IO1_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    ///Bit 27 - G7_IO4
    #[inline(always)]
    pub fn g7_io4(&self) -> G7_IO4_R {
        G7_IO4_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    ///Bit 26 - G7_IO3
    #[inline(always)]
    pub fn g7_io3(&self) -> G7_IO3_R {
        G7_IO3_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bit 25 - G7_IO2
    #[inline(always)]
    pub fn g7_io2(&self) -> G7_IO2_R {
        G7_IO2_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bit 24 - G7_IO1
    #[inline(always)]
    pub fn g7_io1(&self) -> G7_IO1_R {
        G7_IO1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 23 - G6_IO4
    #[inline(always)]
    pub fn g6_io4(&self) -> G6_IO4_R {
        G6_IO4_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 22 - G6_IO3
    #[inline(always)]
    pub fn g6_io3(&self) -> G6_IO3_R {
        G6_IO3_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 21 - G6_IO2
    #[inline(always)]
    pub fn g6_io2(&self) -> G6_IO2_R {
        G6_IO2_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 20 - G6_IO1
    #[inline(always)]
    pub fn g6_io1(&self) -> G6_IO1_R {
        G6_IO1_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 19 - G5_IO4
    #[inline(always)]
    pub fn g5_io4(&self) -> G5_IO4_R {
        G5_IO4_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 18 - G5_IO3
    #[inline(always)]
    pub fn g5_io3(&self) -> G5_IO3_R {
        G5_IO3_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 17 - G5_IO2
    #[inline(always)]
    pub fn g5_io2(&self) -> G5_IO2_R {
        G5_IO2_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 16 - G5_IO1
    #[inline(always)]
    pub fn g5_io1(&self) -> G5_IO1_R {
        G5_IO1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 15 - G4_IO4
    #[inline(always)]
    pub fn g4_io4(&self) -> G4_IO4_R {
        G4_IO4_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 14 - G4_IO3
    #[inline(always)]
    pub fn g4_io3(&self) -> G4_IO3_R {
        G4_IO3_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 13 - G4_IO2
    #[inline(always)]
    pub fn g4_io2(&self) -> G4_IO2_R {
        G4_IO2_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 12 - G4_IO1
    #[inline(always)]
    pub fn g4_io1(&self) -> G4_IO1_R {
        G4_IO1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 11 - G3_IO4
    #[inline(always)]
    pub fn g3_io4(&self) -> G3_IO4_R {
        G3_IO4_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - G3_IO3
    #[inline(always)]
    pub fn g3_io3(&self) -> G3_IO3_R {
        G3_IO3_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 9 - G3_IO2
    #[inline(always)]
    pub fn g3_io2(&self) -> G3_IO2_R {
        G3_IO2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - G3_IO1
    #[inline(always)]
    pub fn g3_io1(&self) -> G3_IO1_R {
        G3_IO1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 7 - G2_IO4
    #[inline(always)]
    pub fn g2_io4(&self) -> G2_IO4_R {
        G2_IO4_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 6 - G2_IO3
    #[inline(always)]
    pub fn g2_io3(&self) -> G2_IO3_R {
        G2_IO3_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - G2_IO2
    #[inline(always)]
    pub fn g2_io2(&self) -> G2_IO2_R {
        G2_IO2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - G2_IO1
    #[inline(always)]
    pub fn g2_io1(&self) -> G2_IO1_R {
        G2_IO1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - G1_IO4
    #[inline(always)]
    pub fn g1_io4(&self) -> G1_IO4_R {
        G1_IO4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - G1_IO3
    #[inline(always)]
    pub fn g1_io3(&self) -> G1_IO3_R {
        G1_IO3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - G1_IO2
    #[inline(always)]
    pub fn g1_io2(&self) -> G1_IO2_R {
        G1_IO2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - G1_IO1
    #[inline(always)]
    pub fn g1_io1(&self) -> G1_IO1_R {
        G1_IO1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 31 - G8_IO4
    #[inline(always)]
    pub fn g8_io4(&mut self) -> G8_IO4_W {
        G8_IO4_W { w: self }
    }
    ///Bit 30 - G8_IO3
    #[inline(always)]
    pub fn g8_io3(&mut self) -> G8_IO3_W {
        G8_IO3_W { w: self }
    }
    ///Bit 29 - G8_IO2
    #[inline(always)]
    pub fn g8_io2(&mut self) -> G8_IO2_W {
        G8_IO2_W { w: self }
    }
    ///Bit 28 - G8_IO1
    #[inline(always)]
    pub fn g8_io1(&mut self) -> G8_IO1_W {
        G8_IO1_W { w: self }
    }
    ///Bit 27 - G7_IO4
    #[inline(always)]
    pub fn g7_io4(&mut self) -> G7_IO4_W {
        G7_IO4_W { w: self }
    }
    ///Bit 26 - G7_IO3
    #[inline(always)]
    pub fn g7_io3(&mut self) -> G7_IO3_W {
        G7_IO3_W { w: self }
    }
    ///Bit 25 - G7_IO2
    #[inline(always)]
    pub fn g7_io2(&mut self) -> G7_IO2_W {
        G7_IO2_W { w: self }
    }
    ///Bit 24 - G7_IO1
    #[inline(always)]
    pub fn g7_io1(&mut self) -> G7_IO1_W {
        G7_IO1_W { w: self }
    }
    ///Bit 23 - G6_IO4
    #[inline(always)]
    pub fn g6_io4(&mut self) -> G6_IO4_W {
        G6_IO4_W { w: self }
    }
    ///Bit 22 - G6_IO3
    #[inline(always)]
    pub fn g6_io3(&mut self) -> G6_IO3_W {
        G6_IO3_W { w: self }
    }
    ///Bit 21 - G6_IO2
    #[inline(always)]
    pub fn g6_io2(&mut self) -> G6_IO2_W {
        G6_IO2_W { w: self }
    }
    ///Bit 20 - G6_IO1
    #[inline(always)]
    pub fn g6_io1(&mut self) -> G6_IO1_W {
        G6_IO1_W { w: self }
    }
    ///Bit 19 - G5_IO4
    #[inline(always)]
    pub fn g5_io4(&mut self) -> G5_IO4_W {
        G5_IO4_W { w: self }
    }
    ///Bit 18 - G5_IO3
    #[inline(always)]
    pub fn g5_io3(&mut self) -> G5_IO3_W {
        G5_IO3_W { w: self }
    }
    ///Bit 17 - G5_IO2
    #[inline(always)]
    pub fn g5_io2(&mut self) -> G5_IO2_W {
        G5_IO2_W { w: self }
    }
    ///Bit 16 - G5_IO1
    #[inline(always)]
    pub fn g5_io1(&mut self) -> G5_IO1_W {
        G5_IO1_W { w: self }
    }
    ///Bit 15 - G4_IO4
    #[inline(always)]
    pub fn g4_io4(&mut self) -> G4_IO4_W {
        G4_IO4_W { w: self }
    }
    ///Bit 14 - G4_IO3
    #[inline(always)]
    pub fn g4_io3(&mut self) -> G4_IO3_W {
        G4_IO3_W { w: self }
    }
    ///Bit 13 - G4_IO2
    #[inline(always)]
    pub fn g4_io2(&mut self) -> G4_IO2_W {
        G4_IO2_W { w: self }
    }
    ///Bit 12 - G4_IO1
    #[inline(always)]
    pub fn g4_io1(&mut self) -> G4_IO1_W {
        G4_IO1_W { w: self }
    }
    ///Bit 11 - G3_IO4
    #[inline(always)]
    pub fn g3_io4(&mut self) -> G3_IO4_W {
        G3_IO4_W { w: self }
    }
    ///Bit 10 - G3_IO3
    #[inline(always)]
    pub fn g3_io3(&mut self) -> G3_IO3_W {
        G3_IO3_W { w: self }
    }
    ///Bit 9 - G3_IO2
    #[inline(always)]
    pub fn g3_io2(&mut self) -> G3_IO2_W {
        G3_IO2_W { w: self }
    }
    ///Bit 8 - G3_IO1
    #[inline(always)]
    pub fn g3_io1(&mut self) -> G3_IO1_W {
        G3_IO1_W { w: self }
    }
    ///Bit 7 - G2_IO4
    #[inline(always)]
    pub fn g2_io4(&mut self) -> G2_IO4_W {
        G2_IO4_W { w: self }
    }
    ///Bit 6 - G2_IO3
    #[inline(always)]
    pub fn g2_io3(&mut self) -> G2_IO3_W {
        G2_IO3_W { w: self }
    }
    ///Bit 5 - G2_IO2
    #[inline(always)]
    pub fn g2_io2(&mut self) -> G2_IO2_W {
        G2_IO2_W { w: self }
    }
    ///Bit 4 - G2_IO1
    #[inline(always)]
    pub fn g2_io1(&mut self) -> G2_IO1_W {
        G2_IO1_W { w: self }
    }
    ///Bit 3 - G1_IO4
    #[inline(always)]
    pub fn g1_io4(&mut self) -> G1_IO4_W {
        G1_IO4_W { w: self }
    }
    ///Bit 2 - G1_IO3
    #[inline(always)]
    pub fn g1_io3(&mut self) -> G1_IO3_W {
        G1_IO3_W { w: self }
    }
    ///Bit 1 - G1_IO2
    #[inline(always)]
    pub fn g1_io2(&mut self) -> G1_IO2_W {
        G1_IO2_W { w: self }
    }
    ///Bit 0 - G1_IO1
    #[inline(always)]
    pub fn g1_io1(&mut self) -> G1_IO1_W {
        G1_IO1_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///I/O channel control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ioccr](index.html) module
pub struct IOCCR_SPEC;
impl crate::RegisterSpec for IOCCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ioccr::R](R) reader structure
impl crate::Readable for IOCCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ioccr::W](W) writer structure
impl crate::Writable for IOCCR_SPEC {
    type Writer = W;
}
///`reset()` method sets IOCCR to value 0
impl crate::Resettable for IOCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
