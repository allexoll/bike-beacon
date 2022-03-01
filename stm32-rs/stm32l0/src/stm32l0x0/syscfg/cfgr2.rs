///Register `CFGR2` reader
pub struct R(crate::R<CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CFGR2` writer
pub struct W(crate::W<CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR2_SPEC>;
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
impl From<crate::W<CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `I2C2_FMP` reader - I2C2 Fm+ drive capability enable bit
pub struct I2C2_FMP_R(crate::FieldReader<bool, bool>);
impl I2C2_FMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C2_FMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C2_FMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `I2C2_FMP` writer - I2C2 Fm+ drive capability enable bit
pub struct I2C2_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2_FMP_W<'a> {
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
///Field `I2C1_FMP` reader - I2C1 Fm+ drive capability enable bit
pub struct I2C1_FMP_R(crate::FieldReader<bool, bool>);
impl I2C1_FMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C1_FMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C1_FMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `I2C1_FMP` writer - I2C1 Fm+ drive capability enable bit
pub struct I2C1_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_FMP_W<'a> {
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
///Field `I2C_PB9_FMP` reader - Fm+ drive capability on PB9 enable bit
pub struct I2C_PB9_FMP_R(crate::FieldReader<bool, bool>);
impl I2C_PB9_FMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_PB9_FMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_PB9_FMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `I2C_PB9_FMP` writer - Fm+ drive capability on PB9 enable bit
pub struct I2C_PB9_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_PB9_FMP_W<'a> {
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
///Field `I2C_PB8_FMP` reader - Fm+ drive capability on PB8 enable bit
pub struct I2C_PB8_FMP_R(crate::FieldReader<bool, bool>);
impl I2C_PB8_FMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_PB8_FMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_PB8_FMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `I2C_PB8_FMP` writer - Fm+ drive capability on PB8 enable bit
pub struct I2C_PB8_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_PB8_FMP_W<'a> {
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
///Field `I2C_PB7_FMP` reader - Fm+ drive capability on PB7 enable bit
pub struct I2C_PB7_FMP_R(crate::FieldReader<bool, bool>);
impl I2C_PB7_FMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_PB7_FMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_PB7_FMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `I2C_PB7_FMP` writer - Fm+ drive capability on PB7 enable bit
pub struct I2C_PB7_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_PB7_FMP_W<'a> {
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
///Field `I2C_PB6_FMP` reader - Fm+ drive capability on PB6 enable bit
pub struct I2C_PB6_FMP_R(crate::FieldReader<bool, bool>);
impl I2C_PB6_FMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_PB6_FMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_PB6_FMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `I2C_PB6_FMP` writer - Fm+ drive capability on PB6 enable bit
pub struct I2C_PB6_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_PB6_FMP_W<'a> {
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
///Field `CAPA` reader - Configuration of internal VLCD rail connection to optional external capacitor
pub struct CAPA_R(crate::FieldReader<u8, u8>);
impl CAPA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CAPA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CAPA` writer - Configuration of internal VLCD rail connection to optional external capacitor
pub struct CAPA_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | ((value as u32 & 0x07) << 1);
        self.w
    }
}
///Field `FWDIS` reader - Firewall disable bit
pub struct FWDIS_R(crate::FieldReader<bool, bool>);
impl FWDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FWDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FWDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `FWDIS` writer - Firewall disable bit
pub struct FWDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> FWDIS_W<'a> {
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
    ///Bit 13 - I2C2 Fm+ drive capability enable bit
    #[inline(always)]
    pub fn i2c2_fmp(&self) -> I2C2_FMP_R {
        I2C2_FMP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 12 - I2C1 Fm+ drive capability enable bit
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2C1_FMP_R {
        I2C1_FMP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 11 - Fm+ drive capability on PB9 enable bit
    #[inline(always)]
    pub fn i2c_pb9_fmp(&self) -> I2C_PB9_FMP_R {
        I2C_PB9_FMP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - Fm+ drive capability on PB8 enable bit
    #[inline(always)]
    pub fn i2c_pb8_fmp(&self) -> I2C_PB8_FMP_R {
        I2C_PB8_FMP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 9 - Fm+ drive capability on PB7 enable bit
    #[inline(always)]
    pub fn i2c_pb7_fmp(&self) -> I2C_PB7_FMP_R {
        I2C_PB7_FMP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - Fm+ drive capability on PB6 enable bit
    #[inline(always)]
    pub fn i2c_pb6_fmp(&self) -> I2C_PB6_FMP_R {
        I2C_PB6_FMP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bits 1:3 - Configuration of internal VLCD rail connection to optional external capacitor
    #[inline(always)]
    pub fn capa(&self) -> CAPA_R {
        CAPA_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    ///Bit 0 - Firewall disable bit
    #[inline(always)]
    pub fn fwdis(&self) -> FWDIS_R {
        FWDIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 13 - I2C2 Fm+ drive capability enable bit
    #[inline(always)]
    pub fn i2c2_fmp(&mut self) -> I2C2_FMP_W {
        I2C2_FMP_W { w: self }
    }
    ///Bit 12 - I2C1 Fm+ drive capability enable bit
    #[inline(always)]
    pub fn i2c1_fmp(&mut self) -> I2C1_FMP_W {
        I2C1_FMP_W { w: self }
    }
    ///Bit 11 - Fm+ drive capability on PB9 enable bit
    #[inline(always)]
    pub fn i2c_pb9_fmp(&mut self) -> I2C_PB9_FMP_W {
        I2C_PB9_FMP_W { w: self }
    }
    ///Bit 10 - Fm+ drive capability on PB8 enable bit
    #[inline(always)]
    pub fn i2c_pb8_fmp(&mut self) -> I2C_PB8_FMP_W {
        I2C_PB8_FMP_W { w: self }
    }
    ///Bit 9 - Fm+ drive capability on PB7 enable bit
    #[inline(always)]
    pub fn i2c_pb7_fmp(&mut self) -> I2C_PB7_FMP_W {
        I2C_PB7_FMP_W { w: self }
    }
    ///Bit 8 - Fm+ drive capability on PB6 enable bit
    #[inline(always)]
    pub fn i2c_pb6_fmp(&mut self) -> I2C_PB6_FMP_W {
        I2C_PB6_FMP_W { w: self }
    }
    ///Bits 1:3 - Configuration of internal VLCD rail connection to optional external capacitor
    #[inline(always)]
    pub fn capa(&mut self) -> CAPA_W {
        CAPA_W { w: self }
    }
    ///Bit 0 - Firewall disable bit
    #[inline(always)]
    pub fn fwdis(&mut self) -> FWDIS_W {
        FWDIS_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SYSCFG configuration register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfgr2](index.html) module
pub struct CFGR2_SPEC;
impl crate::RegisterSpec for CFGR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [cfgr2::R](R) reader structure
impl crate::Readable for CFGR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cfgr2::W](W) writer structure
impl crate::Writable for CFGR2_SPEC {
    type Writer = W;
}
///`reset()` method sets CFGR2 to value 0
impl crate::Resettable for CFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
