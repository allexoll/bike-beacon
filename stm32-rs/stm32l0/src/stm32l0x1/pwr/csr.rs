///Register `CSR` reader
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CSR` writer
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
///Enable WKUP pin 2
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWUP2_A {
    ///0: WKUP pin 2 is used for general purpose I/Os. An event on the WKUP pin 2 does not wakeup the device from Standby mode
    DISABLED = 0,
    ///1: WKUP pin 2 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 2 wakes-up the system from Standby mode)
    ENABLED = 1,
}
impl From<EWUP2_A> for bool {
    #[inline(always)]
    fn from(variant: EWUP2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EWUP2` reader - Enable WKUP pin 2
pub struct EWUP2_R(crate::FieldReader<bool, EWUP2_A>);
impl EWUP2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EWUP2_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EWUP2_A {
        match self.bits {
            false => EWUP2_A::DISABLED,
            true => EWUP2_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EWUP2_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EWUP2_A::ENABLED
    }
}
impl core::ops::Deref for EWUP2_R {
    type Target = crate::FieldReader<bool, EWUP2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EWUP2` writer - Enable WKUP pin 2
pub struct EWUP2_W<'a> {
    w: &'a mut W,
}
impl<'a> EWUP2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EWUP2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///WKUP pin 2 is used for general purpose I/Os. An event on the WKUP pin 2 does not wakeup the device from Standby mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EWUP2_A::DISABLED)
    }
    ///WKUP pin 2 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 2 wakes-up the system from Standby mode)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EWUP2_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
///Enable WKUP pin 1
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWUP1_A {
    ///0: WKUP pin 1 is used for general purpose I/Os. An event on the WKUP pin 1 does not wakeup the device from Standby mode
    DISABLED = 0,
    ///1: WKUP pin 1 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 1 wakes-up the system from Standby mode)
    ENABLED = 1,
}
impl From<EWUP1_A> for bool {
    #[inline(always)]
    fn from(variant: EWUP1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EWUP1` reader - Enable WKUP pin 1
pub struct EWUP1_R(crate::FieldReader<bool, EWUP1_A>);
impl EWUP1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EWUP1_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EWUP1_A {
        match self.bits {
            false => EWUP1_A::DISABLED,
            true => EWUP1_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EWUP1_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EWUP1_A::ENABLED
    }
}
impl core::ops::Deref for EWUP1_R {
    type Target = crate::FieldReader<bool, EWUP1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EWUP1` writer - Enable WKUP pin 1
pub struct EWUP1_W<'a> {
    w: &'a mut W,
}
impl<'a> EWUP1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EWUP1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///WKUP pin 1 is used for general purpose I/Os. An event on the WKUP pin 1 does not wakeup the device from Standby mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EWUP1_A::DISABLED)
    }
    ///WKUP pin 1 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 1 wakes-up the system from Standby mode)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EWUP1_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
///Internal voltage reference ready flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREFINTRDYF_A {
    ///0: VREFINT is OFF
    NOTREADY = 0,
    ///1: VREFINT is ready
    READY = 1,
}
impl From<VREFINTRDYF_A> for bool {
    #[inline(always)]
    fn from(variant: VREFINTRDYF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VREFINTRDYF` reader - Internal voltage reference ready flag
pub struct VREFINTRDYF_R(crate::FieldReader<bool, VREFINTRDYF_A>);
impl VREFINTRDYF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VREFINTRDYF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> VREFINTRDYF_A {
        match self.bits {
            false => VREFINTRDYF_A::NOTREADY,
            true => VREFINTRDYF_A::READY,
        }
    }
    ///Checks if the value of the field is `NOTREADY`
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        **self == VREFINTRDYF_A::NOTREADY
    }
    ///Checks if the value of the field is `READY`
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == VREFINTRDYF_A::READY
    }
}
impl core::ops::Deref for VREFINTRDYF_R {
    type Target = crate::FieldReader<bool, VREFINTRDYF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///PVD output
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PVDO_A {
    ///0: VDD is higher than the PVD threshold selected with the PLS\[2:0\]
    ///bits
    ABOVETHRESHOLD = 0,
    ///1: VDD is lower than the PVD threshold selected with the PLS\[2:0\]
    ///bits
    BELOWTHRESHOLD = 1,
}
impl From<PVDO_A> for bool {
    #[inline(always)]
    fn from(variant: PVDO_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PVDO` reader - PVD output
pub struct PVDO_R(crate::FieldReader<bool, PVDO_A>);
impl PVDO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PVDO_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PVDO_A {
        match self.bits {
            false => PVDO_A::ABOVETHRESHOLD,
            true => PVDO_A::BELOWTHRESHOLD,
        }
    }
    ///Checks if the value of the field is `ABOVETHRESHOLD`
    #[inline(always)]
    pub fn is_above_threshold(&self) -> bool {
        **self == PVDO_A::ABOVETHRESHOLD
    }
    ///Checks if the value of the field is `BELOWTHRESHOLD`
    #[inline(always)]
    pub fn is_below_threshold(&self) -> bool {
        **self == PVDO_A::BELOWTHRESHOLD
    }
}
impl core::ops::Deref for PVDO_R {
    type Target = crate::FieldReader<bool, PVDO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Standby flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBF_A {
    ///0: Device has not been in Standby mode
    NOSTANDBYEVENT = 0,
    ///1: Device has been in Standby mode
    STANDBYEVENT = 1,
}
impl From<SBF_A> for bool {
    #[inline(always)]
    fn from(variant: SBF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SBF` reader - Standby flag
pub struct SBF_R(crate::FieldReader<bool, SBF_A>);
impl SBF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SBF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SBF_A {
        match self.bits {
            false => SBF_A::NOSTANDBYEVENT,
            true => SBF_A::STANDBYEVENT,
        }
    }
    ///Checks if the value of the field is `NOSTANDBYEVENT`
    #[inline(always)]
    pub fn is_no_standby_event(&self) -> bool {
        **self == SBF_A::NOSTANDBYEVENT
    }
    ///Checks if the value of the field is `STANDBYEVENT`
    #[inline(always)]
    pub fn is_standby_event(&self) -> bool {
        **self == SBF_A::STANDBYEVENT
    }
}
impl core::ops::Deref for SBF_R {
    type Target = crate::FieldReader<bool, SBF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Wakeup flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF_A {
    ///0: No wakeup event occurred
    NOWAKEUPEVENT = 0,
    ///1: A wakeup event was received from the WKUP pin or from the RTC alarm (Alarm A or Alarm B), RTC Tamper event, RTC TimeStamp event or RTC Wakeup)
    WAKEUPEVENT = 1,
}
impl From<WUF_A> for bool {
    #[inline(always)]
    fn from(variant: WUF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WUF` reader - Wakeup flag
pub struct WUF_R(crate::FieldReader<bool, WUF_A>);
impl WUF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WUF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WUF_A {
        match self.bits {
            false => WUF_A::NOWAKEUPEVENT,
            true => WUF_A::WAKEUPEVENT,
        }
    }
    ///Checks if the value of the field is `NOWAKEUPEVENT`
    #[inline(always)]
    pub fn is_no_wakeup_event(&self) -> bool {
        **self == WUF_A::NOWAKEUPEVENT
    }
    ///Checks if the value of the field is `WAKEUPEVENT`
    #[inline(always)]
    pub fn is_wakeup_event(&self) -> bool {
        **self == WUF_A::WAKEUPEVENT
    }
}
impl core::ops::Deref for WUF_R {
    type Target = crate::FieldReader<bool, WUF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Voltage Scaling select flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VOSF_A {
    ///0: Regulator is ready in the selected voltage range
    READY = 0,
    ///1: Regulator voltage output is changing to the required VOS level
    NOTREADY = 1,
}
impl From<VOSF_A> for bool {
    #[inline(always)]
    fn from(variant: VOSF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VOSF` reader - Voltage Scaling select flag
pub struct VOSF_R(crate::FieldReader<bool, VOSF_A>);
impl VOSF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VOSF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> VOSF_A {
        match self.bits {
            false => VOSF_A::READY,
            true => VOSF_A::NOTREADY,
        }
    }
    ///Checks if the value of the field is `READY`
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == VOSF_A::READY
    }
    ///Checks if the value of the field is `NOTREADY`
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        **self == VOSF_A::NOTREADY
    }
}
impl core::ops::Deref for VOSF_R {
    type Target = crate::FieldReader<bool, VOSF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Regulator LP flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGLPF_A {
    ///0: Regulator is ready in Main mode
    READY = 0,
    ///1: Regulator voltage is in low-power mode
    NOTREADY = 1,
}
impl From<REGLPF_A> for bool {
    #[inline(always)]
    fn from(variant: REGLPF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `REGLPF` reader - Regulator LP flag
pub struct REGLPF_R(crate::FieldReader<bool, REGLPF_A>);
impl REGLPF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REGLPF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> REGLPF_A {
        match self.bits {
            false => REGLPF_A::READY,
            true => REGLPF_A::NOTREADY,
        }
    }
    ///Checks if the value of the field is `READY`
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == REGLPF_A::READY
    }
    ///Checks if the value of the field is `NOTREADY`
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        **self == REGLPF_A::NOTREADY
    }
}
impl core::ops::Deref for REGLPF_R {
    type Target = crate::FieldReader<bool, REGLPF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Enable WKUP pin 3
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWUP3_A {
    ///0: WKUP pin 3 is used for general purpose I/Os. An event on the WKUP pin 3 does not wakeup the device from Standby mode
    DISABLED = 0,
    ///1: WKUP pin 3 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 3wakes-up the system from Standby mode)
    ENABLED = 1,
}
impl From<EWUP3_A> for bool {
    #[inline(always)]
    fn from(variant: EWUP3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EWUP3` reader - Enable WKUP pin 3
pub struct EWUP3_R(crate::FieldReader<bool, EWUP3_A>);
impl EWUP3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EWUP3_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EWUP3_A {
        match self.bits {
            false => EWUP3_A::DISABLED,
            true => EWUP3_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EWUP3_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EWUP3_A::ENABLED
    }
}
impl core::ops::Deref for EWUP3_R {
    type Target = crate::FieldReader<bool, EWUP3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EWUP3` writer - Enable WKUP pin 3
pub struct EWUP3_W<'a> {
    w: &'a mut W,
}
impl<'a> EWUP3_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EWUP3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///WKUP pin 3 is used for general purpose I/Os. An event on the WKUP pin 3 does not wakeup the device from Standby mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EWUP3_A::DISABLED)
    }
    ///WKUP pin 3 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 3wakes-up the system from Standby mode)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EWUP3_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
impl R {
    ///Bit 9 - Enable WKUP pin 2
    #[inline(always)]
    pub fn ewup2(&self) -> EWUP2_R {
        EWUP2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - Enable WKUP pin 1
    #[inline(always)]
    pub fn ewup1(&self) -> EWUP1_R {
        EWUP1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 3 - Internal voltage reference ready flag
    #[inline(always)]
    pub fn vrefintrdyf(&self) -> VREFINTRDYF_R {
        VREFINTRDYF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - PVD output
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Standby flag
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Wakeup flag
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 4 - Voltage Scaling select flag
    #[inline(always)]
    pub fn vosf(&self) -> VOSF_R {
        VOSF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - Regulator LP flag
    #[inline(always)]
    pub fn reglpf(&self) -> REGLPF_R {
        REGLPF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 10 - Enable WKUP pin 3
    #[inline(always)]
    pub fn ewup3(&self) -> EWUP3_R {
        EWUP3_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    ///Bit 9 - Enable WKUP pin 2
    #[inline(always)]
    pub fn ewup2(&mut self) -> EWUP2_W {
        EWUP2_W { w: self }
    }
    ///Bit 8 - Enable WKUP pin 1
    #[inline(always)]
    pub fn ewup1(&mut self) -> EWUP1_W {
        EWUP1_W { w: self }
    }
    ///Bit 10 - Enable WKUP pin 3
    #[inline(always)]
    pub fn ewup3(&mut self) -> EWUP3_W {
        EWUP3_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///power control/status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [csr](index.html) module
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [csr::R](R) reader structure
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [csr::W](W) writer structure
impl crate::Writable for CSR_SPEC {
    type Writer = W;
}
///`reset()` method sets CSR to value 0
impl crate::Resettable for CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
