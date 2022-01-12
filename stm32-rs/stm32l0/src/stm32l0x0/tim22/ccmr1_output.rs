///Register `CCMR1_Output` reader
pub struct R(crate::R<CCMR1_OUTPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCMR1_OUTPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCMR1_OUTPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCMR1_OUTPUT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCMR1_Output` writer
pub struct W(crate::W<CCMR1_OUTPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCMR1_OUTPUT_SPEC>;
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
impl From<crate::W<CCMR1_OUTPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCMR1_OUTPUT_SPEC>) -> Self {
        W(writer)
    }
}
///Output Compare 2 mode
pub type OC2M_A = OC1M_A;
///Field `OC2M` reader - Output Compare 2 mode
pub type OC2M_R = OC1M_R;
///Field `OC2M` writer - Output Compare 2 mode
pub struct OC2M_W<'a> {
    w: &'a mut W,
}
impl<'a> OC2M_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OC2M_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs
    #[inline(always)]
    pub fn frozen(self) -> &'a mut W {
        self.variant(OC2M_A::FROZEN)
    }
    ///Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register
    #[inline(always)]
    pub fn active_on_match(self) -> &'a mut W {
        self.variant(OC2M_A::ACTIVEONMATCH)
    }
    ///Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register
    #[inline(always)]
    pub fn inactive_on_match(self) -> &'a mut W {
        self.variant(OC2M_A::INACTIVEONMATCH)
    }
    ///OCyREF toggles when TIMx_CNT=TIMx_CCRy
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OC2M_A::TOGGLE)
    }
    ///OCyREF is forced low
    #[inline(always)]
    pub fn force_inactive(self) -> &'a mut W {
        self.variant(OC2M_A::FORCEINACTIVE)
    }
    ///OCyREF is forced high
    #[inline(always)]
    pub fn force_active(self) -> &'a mut W {
        self.variant(OC2M_A::FORCEACTIVE)
    }
    ///In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active
    #[inline(always)]
    pub fn pwm_mode1(self) -> &'a mut W {
        self.variant(OC2M_A::PWMMODE1)
    }
    ///Inversely to PwmMode1
    #[inline(always)]
    pub fn pwm_mode2(self) -> &'a mut W {
        self.variant(OC2M_A::PWMMODE2)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
///Output Compare 2 preload enable
pub type OC2PE_A = OC1PE_A;
///Field `OC2PE` reader - Output Compare 2 preload enable
pub type OC2PE_R = OC1PE_R;
///Field `OC2PE` writer - Output Compare 2 preload enable
pub struct OC2PE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC2PE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OC2PE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Preload register on CCRx disabled. New values written to CCRx are taken into account immediately
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OC2PE_A::DISABLED)
    }
    ///Preload register on CCRx enabled. Preload value is loaded into active register on each update event
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OC2PE_A::ENABLED)
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
///Field `OC2FE` reader - Output Compare 2 fast enable
pub struct OC2FE_R(crate::FieldReader<bool, bool>);
impl OC2FE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OC2FE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC2FE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `OC2FE` writer - Output Compare 2 fast enable
pub struct OC2FE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC2FE_W<'a> {
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
///Capture/Compare 2 selection
pub type CC2S_A = CC1S_A;
///Field `CC2S` reader - Capture/Compare 2 selection
pub type CC2S_R = CC1S_R;
///Field `CC2S` writer - Capture/Compare 2 selection
pub struct CC2S_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2S_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC2S_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///CCx channel is configured as output
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(CC2S_A::OUTPUT)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
///Output Compare 1 mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OC1M_A {
    ///0: The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs
    FROZEN = 0,
    ///1: Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register
    ACTIVEONMATCH = 1,
    ///2: Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register
    INACTIVEONMATCH = 2,
    ///3: OCyREF toggles when TIMx_CNT=TIMx_CCRy
    TOGGLE = 3,
    ///4: OCyREF is forced low
    FORCEINACTIVE = 4,
    ///5: OCyREF is forced high
    FORCEACTIVE = 5,
    ///6: In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active
    PWMMODE1 = 6,
    ///7: Inversely to PwmMode1
    PWMMODE2 = 7,
}
impl From<OC1M_A> for u8 {
    #[inline(always)]
    fn from(variant: OC1M_A) -> Self {
        variant as _
    }
}
///Field `OC1M` reader - Output Compare 1 mode
pub struct OC1M_R(crate::FieldReader<u8, OC1M_A>);
impl OC1M_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OC1M_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OC1M_A {
        match self.bits {
            0 => OC1M_A::FROZEN,
            1 => OC1M_A::ACTIVEONMATCH,
            2 => OC1M_A::INACTIVEONMATCH,
            3 => OC1M_A::TOGGLE,
            4 => OC1M_A::FORCEINACTIVE,
            5 => OC1M_A::FORCEACTIVE,
            6 => OC1M_A::PWMMODE1,
            7 => OC1M_A::PWMMODE2,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `FROZEN`
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        **self == OC1M_A::FROZEN
    }
    ///Checks if the value of the field is `ACTIVEONMATCH`
    #[inline(always)]
    pub fn is_active_on_match(&self) -> bool {
        **self == OC1M_A::ACTIVEONMATCH
    }
    ///Checks if the value of the field is `INACTIVEONMATCH`
    #[inline(always)]
    pub fn is_inactive_on_match(&self) -> bool {
        **self == OC1M_A::INACTIVEONMATCH
    }
    ///Checks if the value of the field is `TOGGLE`
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        **self == OC1M_A::TOGGLE
    }
    ///Checks if the value of the field is `FORCEINACTIVE`
    #[inline(always)]
    pub fn is_force_inactive(&self) -> bool {
        **self == OC1M_A::FORCEINACTIVE
    }
    ///Checks if the value of the field is `FORCEACTIVE`
    #[inline(always)]
    pub fn is_force_active(&self) -> bool {
        **self == OC1M_A::FORCEACTIVE
    }
    ///Checks if the value of the field is `PWMMODE1`
    #[inline(always)]
    pub fn is_pwm_mode1(&self) -> bool {
        **self == OC1M_A::PWMMODE1
    }
    ///Checks if the value of the field is `PWMMODE2`
    #[inline(always)]
    pub fn is_pwm_mode2(&self) -> bool {
        **self == OC1M_A::PWMMODE2
    }
}
impl core::ops::Deref for OC1M_R {
    type Target = crate::FieldReader<u8, OC1M_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `OC1M` writer - Output Compare 1 mode
pub struct OC1M_W<'a> {
    w: &'a mut W,
}
impl<'a> OC1M_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OC1M_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs
    #[inline(always)]
    pub fn frozen(self) -> &'a mut W {
        self.variant(OC1M_A::FROZEN)
    }
    ///Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register
    #[inline(always)]
    pub fn active_on_match(self) -> &'a mut W {
        self.variant(OC1M_A::ACTIVEONMATCH)
    }
    ///Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register
    #[inline(always)]
    pub fn inactive_on_match(self) -> &'a mut W {
        self.variant(OC1M_A::INACTIVEONMATCH)
    }
    ///OCyREF toggles when TIMx_CNT=TIMx_CCRy
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(OC1M_A::TOGGLE)
    }
    ///OCyREF is forced low
    #[inline(always)]
    pub fn force_inactive(self) -> &'a mut W {
        self.variant(OC1M_A::FORCEINACTIVE)
    }
    ///OCyREF is forced high
    #[inline(always)]
    pub fn force_active(self) -> &'a mut W {
        self.variant(OC1M_A::FORCEACTIVE)
    }
    ///In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active
    #[inline(always)]
    pub fn pwm_mode1(self) -> &'a mut W {
        self.variant(OC1M_A::PWMMODE1)
    }
    ///Inversely to PwmMode1
    #[inline(always)]
    pub fn pwm_mode2(self) -> &'a mut W {
        self.variant(OC1M_A::PWMMODE2)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
///Output Compare 1 preload enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OC1PE_A {
    ///0: Preload register on CCRx disabled. New values written to CCRx are taken into account immediately
    DISABLED = 0,
    ///1: Preload register on CCRx enabled. Preload value is loaded into active register on each update event
    ENABLED = 1,
}
impl From<OC1PE_A> for bool {
    #[inline(always)]
    fn from(variant: OC1PE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OC1PE` reader - Output Compare 1 preload enable
pub struct OC1PE_R(crate::FieldReader<bool, OC1PE_A>);
impl OC1PE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OC1PE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OC1PE_A {
        match self.bits {
            false => OC1PE_A::DISABLED,
            true => OC1PE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == OC1PE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == OC1PE_A::ENABLED
    }
}
impl core::ops::Deref for OC1PE_R {
    type Target = crate::FieldReader<bool, OC1PE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `OC1PE` writer - Output Compare 1 preload enable
pub struct OC1PE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC1PE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OC1PE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Preload register on CCRx disabled. New values written to CCRx are taken into account immediately
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OC1PE_A::DISABLED)
    }
    ///Preload register on CCRx enabled. Preload value is loaded into active register on each update event
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OC1PE_A::ENABLED)
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
///Field `OC1FE` reader - Output Compare 1 fast enable
pub struct OC1FE_R(crate::FieldReader<bool, bool>);
impl OC1FE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OC1FE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC1FE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `OC1FE` writer - Output Compare 1 fast enable
pub struct OC1FE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC1FE_W<'a> {
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
///Capture/Compare 1 selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CC1S_A {
    ///0: CCx channel is configured as output
    OUTPUT = 0,
}
impl From<CC1S_A> for u8 {
    #[inline(always)]
    fn from(variant: CC1S_A) -> Self {
        variant as _
    }
}
///Field `CC1S` reader - Capture/Compare 1 selection
pub struct CC1S_R(crate::FieldReader<u8, CC1S_A>);
impl CC1S_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CC1S_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CC1S_A> {
        match self.bits {
            0 => Some(CC1S_A::OUTPUT),
            _ => None,
        }
    }
    ///Checks if the value of the field is `OUTPUT`
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        **self == CC1S_A::OUTPUT
    }
}
impl core::ops::Deref for CC1S_R {
    type Target = crate::FieldReader<u8, CC1S_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CC1S` writer - Capture/Compare 1 selection
pub struct CC1S_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1S_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC1S_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///CCx channel is configured as output
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(CC1S_A::OUTPUT)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    ///Bits 12:14 - Output Compare 2 mode
    #[inline(always)]
    pub fn oc2m(&self) -> OC2M_R {
        OC2M_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    ///Bit 11 - Output Compare 2 preload enable
    #[inline(always)]
    pub fn oc2pe(&self) -> OC2PE_R {
        OC2PE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - Output Compare 2 fast enable
    #[inline(always)]
    pub fn oc2fe(&self) -> OC2FE_R {
        OC2FE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bits 8:9 - Capture/Compare 2 selection
    #[inline(always)]
    pub fn cc2s(&self) -> CC2S_R {
        CC2S_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    ///Bits 4:6 - Output Compare 1 mode
    #[inline(always)]
    pub fn oc1m(&self) -> OC1M_R {
        OC1M_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    ///Bit 3 - Output Compare 1 preload enable
    #[inline(always)]
    pub fn oc1pe(&self) -> OC1PE_R {
        OC1PE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Output Compare 1 fast enable
    #[inline(always)]
    pub fn oc1fe(&self) -> OC1FE_R {
        OC1FE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bits 0:1 - Capture/Compare 1 selection
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    ///Bits 12:14 - Output Compare 2 mode
    #[inline(always)]
    pub fn oc2m(&mut self) -> OC2M_W {
        OC2M_W { w: self }
    }
    ///Bit 11 - Output Compare 2 preload enable
    #[inline(always)]
    pub fn oc2pe(&mut self) -> OC2PE_W {
        OC2PE_W { w: self }
    }
    ///Bit 10 - Output Compare 2 fast enable
    #[inline(always)]
    pub fn oc2fe(&mut self) -> OC2FE_W {
        OC2FE_W { w: self }
    }
    ///Bits 8:9 - Capture/Compare 2 selection
    #[inline(always)]
    pub fn cc2s(&mut self) -> CC2S_W {
        CC2S_W { w: self }
    }
    ///Bits 4:6 - Output Compare 1 mode
    #[inline(always)]
    pub fn oc1m(&mut self) -> OC1M_W {
        OC1M_W { w: self }
    }
    ///Bit 3 - Output Compare 1 preload enable
    #[inline(always)]
    pub fn oc1pe(&mut self) -> OC1PE_W {
        OC1PE_W { w: self }
    }
    ///Bit 2 - Output Compare 1 fast enable
    #[inline(always)]
    pub fn oc1fe(&mut self) -> OC1FE_W {
        OC1FE_W { w: self }
    }
    ///Bits 0:1 - Capture/Compare 1 selection
    #[inline(always)]
    pub fn cc1s(&mut self) -> CC1S_W {
        CC1S_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///capture/compare mode register (output mode)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccmr1_output](index.html) module
pub struct CCMR1_OUTPUT_SPEC;
impl crate::RegisterSpec for CCMR1_OUTPUT_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccmr1_output::R](R) reader structure
impl crate::Readable for CCMR1_OUTPUT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccmr1_output::W](W) writer structure
impl crate::Writable for CCMR1_OUTPUT_SPEC {
    type Writer = W;
}
///`reset()` method sets CCMR1_Output to value 0
impl crate::Resettable for CCMR1_OUTPUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
