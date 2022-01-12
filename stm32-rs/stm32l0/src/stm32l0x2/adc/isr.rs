///Register `ISR` reader
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ISR` writer
pub struct W(crate::W<ISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISR_SPEC>;
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
impl From<crate::W<ISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISR_SPEC>) -> Self {
        W(writer)
    }
}
///ADC ready
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADRDY_A {
    ///0: ADC not yet ready to start conversion
    NOTREADY = 0,
    ///1: ADC ready to start conversion
    READY = 1,
}
impl From<ADRDY_A> for bool {
    #[inline(always)]
    fn from(variant: ADRDY_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADRDY` reader - ADC ready
pub struct ADRDY_R(crate::FieldReader<bool, ADRDY_A>);
impl ADRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADRDY_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADRDY_A {
        match self.bits {
            false => ADRDY_A::NOTREADY,
            true => ADRDY_A::READY,
        }
    }
    ///Checks if the value of the field is `NOTREADY`
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        **self == ADRDY_A::NOTREADY
    }
    ///Checks if the value of the field is `READY`
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == ADRDY_A::READY
    }
}
impl core::ops::Deref for ADRDY_R {
    type Target = crate::FieldReader<bool, ADRDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///ADC ready
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADRDY_AW {
    ///1: Clear the ADC ready flag
    CLEAR = 1,
}
impl From<ADRDY_AW> for bool {
    #[inline(always)]
    fn from(variant: ADRDY_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `ADRDY` writer - ADC ready
pub struct ADRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> ADRDY_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ADRDY_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear the ADC ready flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ADRDY_AW::CLEAR)
    }
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
///End of sampling flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOSMP_A {
    ///0: Not at the end of the samplings phase
    NOTATEND = 0,
    ///1: End of sampling phase reached
    ATEND = 1,
}
impl From<EOSMP_A> for bool {
    #[inline(always)]
    fn from(variant: EOSMP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSMP` reader - End of sampling flag
pub struct EOSMP_R(crate::FieldReader<bool, EOSMP_A>);
impl EOSMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EOSMP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EOSMP_A {
        match self.bits {
            false => EOSMP_A::NOTATEND,
            true => EOSMP_A::ATEND,
        }
    }
    ///Checks if the value of the field is `NOTATEND`
    #[inline(always)]
    pub fn is_not_at_end(&self) -> bool {
        **self == EOSMP_A::NOTATEND
    }
    ///Checks if the value of the field is `ATEND`
    #[inline(always)]
    pub fn is_at_end(&self) -> bool {
        **self == EOSMP_A::ATEND
    }
}
impl core::ops::Deref for EOSMP_R {
    type Target = crate::FieldReader<bool, EOSMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///End of sampling flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOSMP_AW {
    ///1: Clear the sampling phase flag
    CLEAR = 1,
}
impl From<EOSMP_AW> for bool {
    #[inline(always)]
    fn from(variant: EOSMP_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSMP` writer - End of sampling flag
pub struct EOSMP_W<'a> {
    w: &'a mut W,
}
impl<'a> EOSMP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EOSMP_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear the sampling phase flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EOSMP_AW::CLEAR)
    }
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
///End of conversion flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOC_A {
    ///0: Channel conversion is not complete
    NOTCOMPLETE = 0,
    ///1: Channel conversion complete
    COMPLETE = 1,
}
impl From<EOC_A> for bool {
    #[inline(always)]
    fn from(variant: EOC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EOC` reader - End of conversion flag
pub struct EOC_R(crate::FieldReader<bool, EOC_A>);
impl EOC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EOC_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EOC_A {
        match self.bits {
            false => EOC_A::NOTCOMPLETE,
            true => EOC_A::COMPLETE,
        }
    }
    ///Checks if the value of the field is `NOTCOMPLETE`
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        **self == EOC_A::NOTCOMPLETE
    }
    ///Checks if the value of the field is `COMPLETE`
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        **self == EOC_A::COMPLETE
    }
}
impl core::ops::Deref for EOC_R {
    type Target = crate::FieldReader<bool, EOC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///End of conversion flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOC_AW {
    ///1: Clear the channel conversion flag
    CLEAR = 1,
}
impl From<EOC_AW> for bool {
    #[inline(always)]
    fn from(variant: EOC_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `EOC` writer - End of conversion flag
pub struct EOC_W<'a> {
    w: &'a mut W,
}
impl<'a> EOC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EOC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear the channel conversion flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EOC_AW::CLEAR)
    }
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
///End of sequence flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOS_A {
    ///0: Conversion sequence is not complete
    NOTCOMPLETE = 0,
    ///1: Conversion sequence complete
    COMPLETE = 1,
}
impl From<EOS_A> for bool {
    #[inline(always)]
    fn from(variant: EOS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EOS` reader - End of sequence flag
pub struct EOS_R(crate::FieldReader<bool, EOS_A>);
impl EOS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EOS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EOS_A {
        match self.bits {
            false => EOS_A::NOTCOMPLETE,
            true => EOS_A::COMPLETE,
        }
    }
    ///Checks if the value of the field is `NOTCOMPLETE`
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        **self == EOS_A::NOTCOMPLETE
    }
    ///Checks if the value of the field is `COMPLETE`
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        **self == EOS_A::COMPLETE
    }
}
impl core::ops::Deref for EOS_R {
    type Target = crate::FieldReader<bool, EOS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///End of sequence flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOS_AW {
    ///1: Clear the conversion sequence flag
    CLEAR = 1,
}
impl From<EOS_AW> for bool {
    #[inline(always)]
    fn from(variant: EOS_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `EOS` writer - End of sequence flag
pub struct EOS_W<'a> {
    w: &'a mut W,
}
impl<'a> EOS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EOS_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear the conversion sequence flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EOS_AW::CLEAR)
    }
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
///ADC overrun
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVR_A {
    ///0: No overrun occurred
    NOOVERRUN = 0,
    ///1: Overrun occurred
    OVERRUN = 1,
}
impl From<OVR_A> for bool {
    #[inline(always)]
    fn from(variant: OVR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OVR` reader - ADC overrun
pub struct OVR_R(crate::FieldReader<bool, OVR_A>);
impl OVR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OVR_A {
        match self.bits {
            false => OVR_A::NOOVERRUN,
            true => OVR_A::OVERRUN,
        }
    }
    ///Checks if the value of the field is `NOOVERRUN`
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        **self == OVR_A::NOOVERRUN
    }
    ///Checks if the value of the field is `OVERRUN`
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        **self == OVR_A::OVERRUN
    }
}
impl core::ops::Deref for OVR_R {
    type Target = crate::FieldReader<bool, OVR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///ADC overrun
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVR_AW {
    ///1: Clear the overrun flag
    CLEAR = 1,
}
impl From<OVR_AW> for bool {
    #[inline(always)]
    fn from(variant: OVR_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `OVR` writer - ADC overrun
pub struct OVR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OVR_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear the overrun flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OVR_AW::CLEAR)
    }
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
///Analog watchdog flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWD_A {
    ///0: No analog watchdog event occurred
    NOEVENT = 0,
    ///1: Analog watchdog event occurred
    EVENT = 1,
}
impl From<AWD_A> for bool {
    #[inline(always)]
    fn from(variant: AWD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD` reader - Analog watchdog flag
pub struct AWD_R(crate::FieldReader<bool, AWD_A>);
impl AWD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AWD_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AWD_A {
        match self.bits {
            false => AWD_A::NOEVENT,
            true => AWD_A::EVENT,
        }
    }
    ///Checks if the value of the field is `NOEVENT`
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == AWD_A::NOEVENT
    }
    ///Checks if the value of the field is `EVENT`
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        **self == AWD_A::EVENT
    }
}
impl core::ops::Deref for AWD_R {
    type Target = crate::FieldReader<bool, AWD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Analog watchdog flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWD_AW {
    ///1: Clear the analog watchdog event flag
    CLEAR = 1,
}
impl From<AWD_AW> for bool {
    #[inline(always)]
    fn from(variant: AWD_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD` writer - Analog watchdog flag
pub struct AWD_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AWD_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear the analog watchdog event flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(AWD_AW::CLEAR)
    }
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
///End Of Calibration flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOCAL_A {
    ///0: Calibration is not complete
    NOTCOMPLETE = 0,
    ///1: Calibration complete
    COMPLETE = 1,
}
impl From<EOCAL_A> for bool {
    #[inline(always)]
    fn from(variant: EOCAL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EOCAL` reader - End Of Calibration flag
pub struct EOCAL_R(crate::FieldReader<bool, EOCAL_A>);
impl EOCAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EOCAL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EOCAL_A {
        match self.bits {
            false => EOCAL_A::NOTCOMPLETE,
            true => EOCAL_A::COMPLETE,
        }
    }
    ///Checks if the value of the field is `NOTCOMPLETE`
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        **self == EOCAL_A::NOTCOMPLETE
    }
    ///Checks if the value of the field is `COMPLETE`
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        **self == EOCAL_A::COMPLETE
    }
}
impl core::ops::Deref for EOCAL_R {
    type Target = crate::FieldReader<bool, EOCAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///End Of Calibration flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOCAL_AW {
    ///1: Clear the calibration flag
    CLEAR = 1,
}
impl From<EOCAL_AW> for bool {
    #[inline(always)]
    fn from(variant: EOCAL_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `EOCAL` writer - End Of Calibration flag
pub struct EOCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> EOCAL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EOCAL_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear the calibration flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EOCAL_AW::CLEAR)
    }
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
impl R {
    ///Bit 0 - ADC ready
    #[inline(always)]
    pub fn adrdy(&self) -> ADRDY_R {
        ADRDY_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - End of sampling flag
    #[inline(always)]
    pub fn eosmp(&self) -> EOSMP_R {
        EOSMP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - End of conversion flag
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - End of sequence flag
    #[inline(always)]
    pub fn eos(&self) -> EOS_R {
        EOS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - ADC overrun
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 7 - Analog watchdog flag
    #[inline(always)]
    pub fn awd(&self) -> AWD_R {
        AWD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 11 - End Of Calibration flag
    #[inline(always)]
    pub fn eocal(&self) -> EOCAL_R {
        EOCAL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - ADC ready
    #[inline(always)]
    pub fn adrdy(&mut self) -> ADRDY_W {
        ADRDY_W { w: self }
    }
    ///Bit 1 - End of sampling flag
    #[inline(always)]
    pub fn eosmp(&mut self) -> EOSMP_W {
        EOSMP_W { w: self }
    }
    ///Bit 2 - End of conversion flag
    #[inline(always)]
    pub fn eoc(&mut self) -> EOC_W {
        EOC_W { w: self }
    }
    ///Bit 3 - End of sequence flag
    #[inline(always)]
    pub fn eos(&mut self) -> EOS_W {
        EOS_W { w: self }
    }
    ///Bit 4 - ADC overrun
    #[inline(always)]
    pub fn ovr(&mut self) -> OVR_W {
        OVR_W { w: self }
    }
    ///Bit 7 - Analog watchdog flag
    #[inline(always)]
    pub fn awd(&mut self) -> AWD_W {
        AWD_W { w: self }
    }
    ///Bit 11 - End Of Calibration flag
    #[inline(always)]
    pub fn eocal(&mut self) -> EOCAL_W {
        EOCAL_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///interrupt and status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [isr](index.html) module
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
///`read()` method returns [isr::R](R) reader structure
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [isr::W](W) writer structure
impl crate::Writable for ISR_SPEC {
    type Writer = W;
}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
