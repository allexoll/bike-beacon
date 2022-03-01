///Register `DR` reader
pub struct R(crate::R<DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DR` writer
pub struct W(crate::W<DR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DR_SPEC>;
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
impl From<crate::W<DR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `YT` reader - Year tens in BCD format
pub struct YT_R(crate::FieldReader<u8, u8>);
impl YT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        YT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for YT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `YT` writer - Year tens in BCD format
pub struct YT_W<'a> {
    w: &'a mut W,
}
impl<'a> YT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
///Field `YU` reader - Year units in BCD format
pub struct YU_R(crate::FieldReader<u8, u8>);
impl YU_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        YU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for YU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `YU` writer - Year units in BCD format
pub struct YU_W<'a> {
    w: &'a mut W,
}
impl<'a> YU_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
///Field `WDU` reader - Week day units
pub struct WDU_R(crate::FieldReader<u8, u8>);
impl WDU_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WDU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WDU` writer - Week day units
pub struct WDU_W<'a> {
    w: &'a mut W,
}
impl<'a> WDU_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | ((value as u32 & 0x07) << 13);
        self.w
    }
}
///Field `MT` reader - Month tens in BCD format
pub struct MT_R(crate::FieldReader<bool, bool>);
impl MT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MT` writer - Month tens in BCD format
pub struct MT_W<'a> {
    w: &'a mut W,
}
impl<'a> MT_W<'a> {
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
///Field `MU` reader - Month units in BCD format
pub struct MU_R(crate::FieldReader<u8, u8>);
impl MU_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MU` writer - Month units in BCD format
pub struct MU_W<'a> {
    w: &'a mut W,
}
impl<'a> MU_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
///Field `DT` reader - Date tens in BCD format
pub struct DT_R(crate::FieldReader<u8, u8>);
impl DT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DT` writer - Date tens in BCD format
pub struct DT_W<'a> {
    w: &'a mut W,
}
impl<'a> DT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
///Field `DU` reader - Date units in BCD format
pub struct DU_R(crate::FieldReader<u8, u8>);
impl DU_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DU` writer - Date units in BCD format
pub struct DU_W<'a> {
    w: &'a mut W,
}
impl<'a> DU_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    ///Bits 20:23 - Year tens in BCD format
    #[inline(always)]
    pub fn yt(&self) -> YT_R {
        YT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 16:19 - Year units in BCD format
    #[inline(always)]
    pub fn yu(&self) -> YU_R {
        YU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 13:15 - Week day units
    #[inline(always)]
    pub fn wdu(&self) -> WDU_R {
        WDU_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    ///Bit 12 - Month tens in BCD format
    #[inline(always)]
    pub fn mt(&self) -> MT_R {
        MT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bits 8:11 - Month units in BCD format
    #[inline(always)]
    pub fn mu(&self) -> MU_R {
        MU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 4:5 - Date tens in BCD format
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    ///Bits 0:3 - Date units in BCD format
    #[inline(always)]
    pub fn du(&self) -> DU_R {
        DU_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    ///Bits 20:23 - Year tens in BCD format
    #[inline(always)]
    pub fn yt(&mut self) -> YT_W {
        YT_W { w: self }
    }
    ///Bits 16:19 - Year units in BCD format
    #[inline(always)]
    pub fn yu(&mut self) -> YU_W {
        YU_W { w: self }
    }
    ///Bits 13:15 - Week day units
    #[inline(always)]
    pub fn wdu(&mut self) -> WDU_W {
        WDU_W { w: self }
    }
    ///Bit 12 - Month tens in BCD format
    #[inline(always)]
    pub fn mt(&mut self) -> MT_W {
        MT_W { w: self }
    }
    ///Bits 8:11 - Month units in BCD format
    #[inline(always)]
    pub fn mu(&mut self) -> MU_W {
        MU_W { w: self }
    }
    ///Bits 4:5 - Date tens in BCD format
    #[inline(always)]
    pub fn dt(&mut self) -> DT_W {
        DT_W { w: self }
    }
    ///Bits 0:3 - Date units in BCD format
    #[inline(always)]
    pub fn du(&mut self) -> DU_W {
        DU_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RTC date register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dr](index.html) module
pub struct DR_SPEC;
impl crate::RegisterSpec for DR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dr::R](R) reader structure
impl crate::Readable for DR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dr::W](W) writer structure
impl crate::Writable for DR_SPEC {
    type Writer = W;
}
///`reset()` method sets DR to value 0
impl crate::Resettable for DR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
