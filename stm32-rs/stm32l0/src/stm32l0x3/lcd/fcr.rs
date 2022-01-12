///Register `FCR` reader
pub struct R(crate::R<FCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FCR` writer
pub struct W(crate::W<FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCR_SPEC>;
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
impl From<crate::W<FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PS` reader - PS 16-bit prescaler
pub struct PS_R(crate::FieldReader<u8, u8>);
impl PS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PS` writer - PS 16-bit prescaler
pub struct PS_W<'a> {
    w: &'a mut W,
}
impl<'a> PS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 22)) | ((value as u32 & 0x0f) << 22);
        self.w
    }
}
///Field `DIV` reader - DIV clock divider
pub struct DIV_R(crate::FieldReader<u8, u8>);
impl DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DIV` writer - DIV clock divider
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | ((value as u32 & 0x0f) << 18);
        self.w
    }
}
///Field `BLINK` reader - Blink mode selection
pub struct BLINK_R(crate::FieldReader<u8, u8>);
impl BLINK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BLINK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLINK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BLINK` writer - Blink mode selection
pub struct BLINK_W<'a> {
    w: &'a mut W,
}
impl<'a> BLINK_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
///Field `BLINKF` reader - Blink frequency selection
pub struct BLINKF_R(crate::FieldReader<u8, u8>);
impl BLINKF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BLINKF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLINKF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BLINKF` writer - Blink frequency selection
pub struct BLINKF_W<'a> {
    w: &'a mut W,
}
impl<'a> BLINKF_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | ((value as u32 & 0x07) << 13);
        self.w
    }
}
///Field `CC` reader - Contrast control
pub struct CC_R(crate::FieldReader<u8, u8>);
impl CC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CC` writer - Contrast control
pub struct CC_W<'a> {
    w: &'a mut W,
}
impl<'a> CC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | ((value as u32 & 0x07) << 10);
        self.w
    }
}
///Field `DEAD` reader - Dead time duration
pub struct DEAD_R(crate::FieldReader<u8, u8>);
impl DEAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DEAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEAD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DEAD` writer - Dead time duration
pub struct DEAD_W<'a> {
    w: &'a mut W,
}
impl<'a> DEAD_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 7)) | ((value as u32 & 0x07) << 7);
        self.w
    }
}
///Field `PON` reader - Pulse ON duration
pub struct PON_R(crate::FieldReader<u8, u8>);
impl PON_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PON_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PON` writer - Pulse ON duration
pub struct PON_W<'a> {
    w: &'a mut W,
}
impl<'a> PON_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
///Field `UDDIE` reader - Update display done interrupt enable
pub struct UDDIE_R(crate::FieldReader<bool, bool>);
impl UDDIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UDDIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UDDIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `UDDIE` writer - Update display done interrupt enable
pub struct UDDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UDDIE_W<'a> {
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
///Field `SOFIE` reader - Start of frame interrupt enable
pub struct SOFIE_R(crate::FieldReader<bool, bool>);
impl SOFIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SOFIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SOFIE` writer - Start of frame interrupt enable
pub struct SOFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFIE_W<'a> {
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
///Field `HD` reader - High drive enable
pub struct HD_R(crate::FieldReader<bool, bool>);
impl HD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `HD` writer - High drive enable
pub struct HD_W<'a> {
    w: &'a mut W,
}
impl<'a> HD_W<'a> {
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
    ///Bits 22:25 - PS 16-bit prescaler
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    ///Bits 18:21 - DIV clock divider
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    ///Bits 16:17 - Blink mode selection
    #[inline(always)]
    pub fn blink(&self) -> BLINK_R {
        BLINK_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    ///Bits 13:15 - Blink frequency selection
    #[inline(always)]
    pub fn blinkf(&self) -> BLINKF_R {
        BLINKF_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    ///Bits 10:12 - Contrast control
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    ///Bits 7:9 - Dead time duration
    #[inline(always)]
    pub fn dead(&self) -> DEAD_R {
        DEAD_R::new(((self.bits >> 7) & 0x07) as u8)
    }
    ///Bits 4:6 - Pulse ON duration
    #[inline(always)]
    pub fn pon(&self) -> PON_R {
        PON_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    ///Bit 3 - Update display done interrupt enable
    #[inline(always)]
    pub fn uddie(&self) -> UDDIE_R {
        UDDIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 1 - Start of frame interrupt enable
    #[inline(always)]
    pub fn sofie(&self) -> SOFIE_R {
        SOFIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - High drive enable
    #[inline(always)]
    pub fn hd(&self) -> HD_R {
        HD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bits 22:25 - PS 16-bit prescaler
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W {
        PS_W { w: self }
    }
    ///Bits 18:21 - DIV clock divider
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
    ///Bits 16:17 - Blink mode selection
    #[inline(always)]
    pub fn blink(&mut self) -> BLINK_W {
        BLINK_W { w: self }
    }
    ///Bits 13:15 - Blink frequency selection
    #[inline(always)]
    pub fn blinkf(&mut self) -> BLINKF_W {
        BLINKF_W { w: self }
    }
    ///Bits 10:12 - Contrast control
    #[inline(always)]
    pub fn cc(&mut self) -> CC_W {
        CC_W { w: self }
    }
    ///Bits 7:9 - Dead time duration
    #[inline(always)]
    pub fn dead(&mut self) -> DEAD_W {
        DEAD_W { w: self }
    }
    ///Bits 4:6 - Pulse ON duration
    #[inline(always)]
    pub fn pon(&mut self) -> PON_W {
        PON_W { w: self }
    }
    ///Bit 3 - Update display done interrupt enable
    #[inline(always)]
    pub fn uddie(&mut self) -> UDDIE_W {
        UDDIE_W { w: self }
    }
    ///Bit 1 - Start of frame interrupt enable
    #[inline(always)]
    pub fn sofie(&mut self) -> SOFIE_W {
        SOFIE_W { w: self }
    }
    ///Bit 0 - High drive enable
    #[inline(always)]
    pub fn hd(&mut self) -> HD_W {
        HD_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///frame control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fcr](index.html) module
pub struct FCR_SPEC;
impl crate::RegisterSpec for FCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [fcr::R](R) reader structure
impl crate::Readable for FCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fcr::W](W) writer structure
impl crate::Writable for FCR_SPEC {
    type Writer = W;
}
///`reset()` method sets FCR to value 0
impl crate::Resettable for FCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
