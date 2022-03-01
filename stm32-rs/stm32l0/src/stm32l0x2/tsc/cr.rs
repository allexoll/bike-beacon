///Register `CR` reader
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR` writer
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CTPH` reader - Charge transfer pulse high
pub struct CTPH_R(crate::FieldReader<u8, u8>);
impl CTPH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CTPH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTPH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CTPH` writer - Charge transfer pulse high
pub struct CTPH_W<'a> {
    w: &'a mut W,
}
impl<'a> CTPH_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
///Field `CTPL` reader - Charge transfer pulse low
pub struct CTPL_R(crate::FieldReader<u8, u8>);
impl CTPL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CTPL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTPL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CTPL` writer - Charge transfer pulse low
pub struct CTPL_W<'a> {
    w: &'a mut W,
}
impl<'a> CTPL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
///Field `SSD` reader - Spread spectrum deviation
pub struct SSD_R(crate::FieldReader<u8, u8>);
impl SSD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SSD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SSD` writer - Spread spectrum deviation
pub struct SSD_W<'a> {
    w: &'a mut W,
}
impl<'a> SSD_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 17)) | ((value as u32 & 0x7f) << 17);
        self.w
    }
}
///Field `SSE` reader - Spread spectrum enable
pub struct SSE_R(crate::FieldReader<bool, bool>);
impl SSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SSE` writer - Spread spectrum enable
pub struct SSE_W<'a> {
    w: &'a mut W,
}
impl<'a> SSE_W<'a> {
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
///Field `SSPSC` reader - Spread spectrum prescaler
pub struct SSPSC_R(crate::FieldReader<bool, bool>);
impl SSPSC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SSPSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSPSC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SSPSC` writer - Spread spectrum prescaler
pub struct SSPSC_W<'a> {
    w: &'a mut W,
}
impl<'a> SSPSC_W<'a> {
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
///Field `PGPSC` reader - pulse generator prescaler
pub struct PGPSC_R(crate::FieldReader<u8, u8>);
impl PGPSC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PGPSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGPSC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PGPSC` writer - pulse generator prescaler
pub struct PGPSC_W<'a> {
    w: &'a mut W,
}
impl<'a> PGPSC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
///Field `MCV` reader - Max count value
pub struct MCV_R(crate::FieldReader<u8, u8>);
impl MCV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MCV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MCV` writer - Max count value
pub struct MCV_W<'a> {
    w: &'a mut W,
}
impl<'a> MCV_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
///Field `IODEF` reader - I/O Default mode
pub struct IODEF_R(crate::FieldReader<bool, bool>);
impl IODEF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IODEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IODEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `IODEF` writer - I/O Default mode
pub struct IODEF_W<'a> {
    w: &'a mut W,
}
impl<'a> IODEF_W<'a> {
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
///Field `SYNCPOL` reader - Synchronization pin polarity
pub struct SYNCPOL_R(crate::FieldReader<bool, bool>);
impl SYNCPOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYNCPOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNCPOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SYNCPOL` writer - Synchronization pin polarity
pub struct SYNCPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCPOL_W<'a> {
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
///Field `AM` reader - Acquisition mode
pub struct AM_R(crate::FieldReader<bool, bool>);
impl AM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `AM` writer - Acquisition mode
pub struct AM_W<'a> {
    w: &'a mut W,
}
impl<'a> AM_W<'a> {
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
///Field `START` reader - Start a new acquisition
pub struct START_R(crate::FieldReader<bool, bool>);
impl START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `START` writer - Start a new acquisition
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
///Field `TSCE` reader - Touch sensing controller enable
pub struct TSCE_R(crate::FieldReader<bool, bool>);
impl TSCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TSCE` writer - Touch sensing controller enable
pub struct TSCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSCE_W<'a> {
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
    ///Bits 28:31 - Charge transfer pulse high
    #[inline(always)]
    pub fn ctph(&self) -> CTPH_R {
        CTPH_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    ///Bits 24:27 - Charge transfer pulse low
    #[inline(always)]
    pub fn ctpl(&self) -> CTPL_R {
        CTPL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 17:23 - Spread spectrum deviation
    #[inline(always)]
    pub fn ssd(&self) -> SSD_R {
        SSD_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    ///Bit 16 - Spread spectrum enable
    #[inline(always)]
    pub fn sse(&self) -> SSE_R {
        SSE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 15 - Spread spectrum prescaler
    #[inline(always)]
    pub fn sspsc(&self) -> SSPSC_R {
        SSPSC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bits 12:14 - pulse generator prescaler
    #[inline(always)]
    pub fn pgpsc(&self) -> PGPSC_R {
        PGPSC_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    ///Bits 5:7 - Max count value
    #[inline(always)]
    pub fn mcv(&self) -> MCV_R {
        MCV_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    ///Bit 4 - I/O Default mode
    #[inline(always)]
    pub fn iodef(&self) -> IODEF_R {
        IODEF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - Synchronization pin polarity
    #[inline(always)]
    pub fn syncpol(&self) -> SYNCPOL_R {
        SYNCPOL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Acquisition mode
    #[inline(always)]
    pub fn am(&self) -> AM_R {
        AM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Start a new acquisition
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Touch sensing controller enable
    #[inline(always)]
    pub fn tsce(&self) -> TSCE_R {
        TSCE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bits 28:31 - Charge transfer pulse high
    #[inline(always)]
    pub fn ctph(&mut self) -> CTPH_W {
        CTPH_W { w: self }
    }
    ///Bits 24:27 - Charge transfer pulse low
    #[inline(always)]
    pub fn ctpl(&mut self) -> CTPL_W {
        CTPL_W { w: self }
    }
    ///Bits 17:23 - Spread spectrum deviation
    #[inline(always)]
    pub fn ssd(&mut self) -> SSD_W {
        SSD_W { w: self }
    }
    ///Bit 16 - Spread spectrum enable
    #[inline(always)]
    pub fn sse(&mut self) -> SSE_W {
        SSE_W { w: self }
    }
    ///Bit 15 - Spread spectrum prescaler
    #[inline(always)]
    pub fn sspsc(&mut self) -> SSPSC_W {
        SSPSC_W { w: self }
    }
    ///Bits 12:14 - pulse generator prescaler
    #[inline(always)]
    pub fn pgpsc(&mut self) -> PGPSC_W {
        PGPSC_W { w: self }
    }
    ///Bits 5:7 - Max count value
    #[inline(always)]
    pub fn mcv(&mut self) -> MCV_W {
        MCV_W { w: self }
    }
    ///Bit 4 - I/O Default mode
    #[inline(always)]
    pub fn iodef(&mut self) -> IODEF_W {
        IODEF_W { w: self }
    }
    ///Bit 3 - Synchronization pin polarity
    #[inline(always)]
    pub fn syncpol(&mut self) -> SYNCPOL_W {
        SYNCPOL_W { w: self }
    }
    ///Bit 2 - Acquisition mode
    #[inline(always)]
    pub fn am(&mut self) -> AM_W {
        AM_W { w: self }
    }
    ///Bit 1 - Start a new acquisition
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    ///Bit 0 - Touch sensing controller enable
    #[inline(always)]
    pub fn tsce(&mut self) -> TSCE_W {
        TSCE_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](index.html) module
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr::R](R) reader structure
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr::W](W) writer structure
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
