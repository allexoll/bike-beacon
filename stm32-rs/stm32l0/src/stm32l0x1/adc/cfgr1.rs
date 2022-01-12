///Register `CFGR1` reader
pub struct R(crate::R<CFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CFGR1` writer
pub struct W(crate::W<CFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR1_SPEC>;
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
impl From<crate::W<CFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `AWDCH` reader - Analog watchdog channel selection
pub struct AWDCH_R(crate::FieldReader<u8, u8>);
impl AWDCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        AWDCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AWDCH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `AWDCH` writer - Analog watchdog channel selection
pub struct AWDCH_W<'a> {
    w: &'a mut W,
}
impl<'a> AWDCH_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 26)) | ((value as u32 & 0x1f) << 26);
        self.w
    }
}
///Analog watchdog enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWDEN_A {
    ///0: Analog watchdog disabled
    DISABLED = 0,
    ///1: Analog watchdog enabled
    ENABLED = 1,
}
impl From<AWDEN_A> for bool {
    #[inline(always)]
    fn from(variant: AWDEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AWDEN` reader - Analog watchdog enable
pub struct AWDEN_R(crate::FieldReader<bool, AWDEN_A>);
impl AWDEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AWDEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AWDEN_A {
        match self.bits {
            false => AWDEN_A::DISABLED,
            true => AWDEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == AWDEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == AWDEN_A::ENABLED
    }
}
impl core::ops::Deref for AWDEN_R {
    type Target = crate::FieldReader<bool, AWDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `AWDEN` writer - Analog watchdog enable
pub struct AWDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AWDEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AWDEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Analog watchdog disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AWDEN_A::DISABLED)
    }
    ///Analog watchdog enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AWDEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
///Enable the watchdog on a single channel or on all channels
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWDSGL_A {
    ///0: Analog watchdog enabled on all channels
    ALLCHANNELS = 0,
    ///1: Analog watchdog enabled on a single channel
    SINGLECHANNEL = 1,
}
impl From<AWDSGL_A> for bool {
    #[inline(always)]
    fn from(variant: AWDSGL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AWDSGL` reader - Enable the watchdog on a single channel or on all channels
pub struct AWDSGL_R(crate::FieldReader<bool, AWDSGL_A>);
impl AWDSGL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AWDSGL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AWDSGL_A {
        match self.bits {
            false => AWDSGL_A::ALLCHANNELS,
            true => AWDSGL_A::SINGLECHANNEL,
        }
    }
    ///Checks if the value of the field is `ALLCHANNELS`
    #[inline(always)]
    pub fn is_all_channels(&self) -> bool {
        **self == AWDSGL_A::ALLCHANNELS
    }
    ///Checks if the value of the field is `SINGLECHANNEL`
    #[inline(always)]
    pub fn is_single_channel(&self) -> bool {
        **self == AWDSGL_A::SINGLECHANNEL
    }
}
impl core::ops::Deref for AWDSGL_R {
    type Target = crate::FieldReader<bool, AWDSGL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `AWDSGL` writer - Enable the watchdog on a single channel or on all channels
pub struct AWDSGL_W<'a> {
    w: &'a mut W,
}
impl<'a> AWDSGL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AWDSGL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Analog watchdog enabled on all channels
    #[inline(always)]
    pub fn all_channels(self) -> &'a mut W {
        self.variant(AWDSGL_A::ALLCHANNELS)
    }
    ///Analog watchdog enabled on a single channel
    #[inline(always)]
    pub fn single_channel(self) -> &'a mut W {
        self.variant(AWDSGL_A::SINGLECHANNEL)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
///Discontinuous mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISCEN_A {
    ///0: Discontinuous mode disabled
    DISABLED = 0,
    ///1: Discontinuous mode enabled
    ENABLED = 1,
}
impl From<DISCEN_A> for bool {
    #[inline(always)]
    fn from(variant: DISCEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DISCEN` reader - Discontinuous mode
pub struct DISCEN_R(crate::FieldReader<bool, DISCEN_A>);
impl DISCEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DISCEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DISCEN_A {
        match self.bits {
            false => DISCEN_A::DISABLED,
            true => DISCEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DISCEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DISCEN_A::ENABLED
    }
}
impl core::ops::Deref for DISCEN_R {
    type Target = crate::FieldReader<bool, DISCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DISCEN` writer - Discontinuous mode
pub struct DISCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DISCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Discontinuous mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DISCEN_A::DISABLED)
    }
    ///Discontinuous mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DISCEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
///Auto-off mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOFF_A {
    ///0: Auto-off mode disabled
    DISABLED = 0,
    ///1: Auto-off mode enabled
    ENABLED = 1,
}
impl From<AUTOFF_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOFF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `AUTOFF` reader - Auto-off mode
pub struct AUTOFF_R(crate::FieldReader<bool, AUTOFF_A>);
impl AUTOFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUTOFF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AUTOFF_A {
        match self.bits {
            false => AUTOFF_A::DISABLED,
            true => AUTOFF_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == AUTOFF_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == AUTOFF_A::ENABLED
    }
}
impl core::ops::Deref for AUTOFF_R {
    type Target = crate::FieldReader<bool, AUTOFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `AUTOFF` writer - Auto-off mode
pub struct AUTOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOFF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AUTOFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Auto-off mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AUTOFF_A::DISABLED)
    }
    ///Auto-off mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AUTOFF_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
///Auto-delayed conversion mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAIT_A {
    ///0: Wait conversion mode off
    DISABLED = 0,
    ///1: Wait conversion mode on
    ENABLED = 1,
}
impl From<WAIT_A> for bool {
    #[inline(always)]
    fn from(variant: WAIT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WAIT` reader - Auto-delayed conversion mode
pub struct WAIT_R(crate::FieldReader<bool, WAIT_A>);
impl WAIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAIT_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WAIT_A {
        match self.bits {
            false => WAIT_A::DISABLED,
            true => WAIT_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WAIT_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WAIT_A::ENABLED
    }
}
impl core::ops::Deref for WAIT_R {
    type Target = crate::FieldReader<bool, WAIT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WAIT` writer - Auto-delayed conversion mode
pub struct WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> WAIT_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WAIT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Wait conversion mode off
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAIT_A::DISABLED)
    }
    ///Wait conversion mode on
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WAIT_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
///Single / continuous conversion mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONT_A {
    ///0: Single conversion mode
    SINGLE = 0,
    ///1: Continuous conversion mode
    CONTINUOUS = 1,
}
impl From<CONT_A> for bool {
    #[inline(always)]
    fn from(variant: CONT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CONT` reader - Single / continuous conversion mode
pub struct CONT_R(crate::FieldReader<bool, CONT_A>);
impl CONT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CONT_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CONT_A {
        match self.bits {
            false => CONT_A::SINGLE,
            true => CONT_A::CONTINUOUS,
        }
    }
    ///Checks if the value of the field is `SINGLE`
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        **self == CONT_A::SINGLE
    }
    ///Checks if the value of the field is `CONTINUOUS`
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        **self == CONT_A::CONTINUOUS
    }
}
impl core::ops::Deref for CONT_R {
    type Target = crate::FieldReader<bool, CONT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CONT` writer - Single / continuous conversion mode
pub struct CONT_W<'a> {
    w: &'a mut W,
}
impl<'a> CONT_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CONT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Single conversion mode
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(CONT_A::SINGLE)
    }
    ///Continuous conversion mode
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CONT_A::CONTINUOUS)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
///Overrun management mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRMOD_A {
    ///0: ADC_DR register is preserved with the old data when an overrun is detected
    PRESERVE = 0,
    ///1: ADC_DR register is overwritten with the last conversion result when an overrun is detected
    OVERWRITE = 1,
}
impl From<OVRMOD_A> for bool {
    #[inline(always)]
    fn from(variant: OVRMOD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OVRMOD` reader - Overrun management mode
pub struct OVRMOD_R(crate::FieldReader<bool, OVRMOD_A>);
impl OVRMOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVRMOD_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OVRMOD_A {
        match self.bits {
            false => OVRMOD_A::PRESERVE,
            true => OVRMOD_A::OVERWRITE,
        }
    }
    ///Checks if the value of the field is `PRESERVE`
    #[inline(always)]
    pub fn is_preserve(&self) -> bool {
        **self == OVRMOD_A::PRESERVE
    }
    ///Checks if the value of the field is `OVERWRITE`
    #[inline(always)]
    pub fn is_overwrite(&self) -> bool {
        **self == OVRMOD_A::OVERWRITE
    }
}
impl core::ops::Deref for OVRMOD_R {
    type Target = crate::FieldReader<bool, OVRMOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `OVRMOD` writer - Overrun management mode
pub struct OVRMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRMOD_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OVRMOD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///ADC_DR register is preserved with the old data when an overrun is detected
    #[inline(always)]
    pub fn preserve(self) -> &'a mut W {
        self.variant(OVRMOD_A::PRESERVE)
    }
    ///ADC_DR register is overwritten with the last conversion result when an overrun is detected
    #[inline(always)]
    pub fn overwrite(self) -> &'a mut W {
        self.variant(OVRMOD_A::OVERWRITE)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
///External trigger enable and polarity selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTEN_A {
    ///0: Hardware trigger detection disabled
    DISABLED = 0,
    ///1: Hardware trigger detection on the rising edge
    RISINGEDGE = 1,
    ///2: Hardware trigger detection on the falling edge
    FALLINGEDGE = 2,
    ///3: Hardware trigger detection on both the rising and falling edges
    BOTHEDGES = 3,
}
impl From<EXTEN_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTEN_A) -> Self {
        variant as _
    }
}
///Field `EXTEN` reader - External trigger enable and polarity selection
pub struct EXTEN_R(crate::FieldReader<u8, EXTEN_A>);
impl EXTEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EXTEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EXTEN_A {
        match self.bits {
            0 => EXTEN_A::DISABLED,
            1 => EXTEN_A::RISINGEDGE,
            2 => EXTEN_A::FALLINGEDGE,
            3 => EXTEN_A::BOTHEDGES,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EXTEN_A::DISABLED
    }
    ///Checks if the value of the field is `RISINGEDGE`
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        **self == EXTEN_A::RISINGEDGE
    }
    ///Checks if the value of the field is `FALLINGEDGE`
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        **self == EXTEN_A::FALLINGEDGE
    }
    ///Checks if the value of the field is `BOTHEDGES`
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        **self == EXTEN_A::BOTHEDGES
    }
}
impl core::ops::Deref for EXTEN_R {
    type Target = crate::FieldReader<u8, EXTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EXTEN` writer - External trigger enable and polarity selection
pub struct EXTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EXTEN_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Hardware trigger detection disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EXTEN_A::DISABLED)
    }
    ///Hardware trigger detection on the rising edge
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(EXTEN_A::RISINGEDGE)
    }
    ///Hardware trigger detection on the falling edge
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(EXTEN_A::FALLINGEDGE)
    }
    ///Hardware trigger detection on both the rising and falling edges
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(EXTEN_A::BOTHEDGES)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
///External trigger selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTSEL_A {
    ///0: Timer 6 TRGO event
    TIM6_TRGO = 0,
    ///1: Timer 21 CH2 event
    TIM21_CH2 = 1,
    ///2: Timer 2 TRGO event
    TIM2_TRGO = 2,
    ///3: Timer 2 CH4 event
    TIM2_CH4 = 3,
    ///4: Timer 22 TRGO, Timer 21 TRGO event
    TIM22_TRGO = 4,
    ///5: Timer 2 CH3 event
    TIM2_CH3 = 5,
    ///6: Timer 3 TRGO event
    TIM3_TRGO = 6,
    ///7: EXTI line 11 event
    EXTI_LINE11 = 7,
}
impl From<EXTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTSEL_A) -> Self {
        variant as _
    }
}
///Field `EXTSEL` reader - External trigger selection
pub struct EXTSEL_R(crate::FieldReader<u8, EXTSEL_A>);
impl EXTSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EXTSEL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EXTSEL_A {
        match self.bits {
            0 => EXTSEL_A::TIM6_TRGO,
            1 => EXTSEL_A::TIM21_CH2,
            2 => EXTSEL_A::TIM2_TRGO,
            3 => EXTSEL_A::TIM2_CH4,
            4 => EXTSEL_A::TIM22_TRGO,
            5 => EXTSEL_A::TIM2_CH3,
            6 => EXTSEL_A::TIM3_TRGO,
            7 => EXTSEL_A::EXTI_LINE11,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `TIM6_TRGO`
    #[inline(always)]
    pub fn is_tim6_trgo(&self) -> bool {
        **self == EXTSEL_A::TIM6_TRGO
    }
    ///Checks if the value of the field is `TIM21_CH2`
    #[inline(always)]
    pub fn is_tim21_ch2(&self) -> bool {
        **self == EXTSEL_A::TIM21_CH2
    }
    ///Checks if the value of the field is `TIM2_TRGO`
    #[inline(always)]
    pub fn is_tim2_trgo(&self) -> bool {
        **self == EXTSEL_A::TIM2_TRGO
    }
    ///Checks if the value of the field is `TIM2_CH4`
    #[inline(always)]
    pub fn is_tim2_ch4(&self) -> bool {
        **self == EXTSEL_A::TIM2_CH4
    }
    ///Checks if the value of the field is `TIM22_TRGO`
    #[inline(always)]
    pub fn is_tim22_trgo(&self) -> bool {
        **self == EXTSEL_A::TIM22_TRGO
    }
    ///Checks if the value of the field is `TIM2_CH3`
    #[inline(always)]
    pub fn is_tim2_ch3(&self) -> bool {
        **self == EXTSEL_A::TIM2_CH3
    }
    ///Checks if the value of the field is `TIM3_TRGO`
    #[inline(always)]
    pub fn is_tim3_trgo(&self) -> bool {
        **self == EXTSEL_A::TIM3_TRGO
    }
    ///Checks if the value of the field is `EXTI_LINE11`
    #[inline(always)]
    pub fn is_exti_line11(&self) -> bool {
        **self == EXTSEL_A::EXTI_LINE11
    }
}
impl core::ops::Deref for EXTSEL_R {
    type Target = crate::FieldReader<u8, EXTSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EXTSEL` writer - External trigger selection
pub struct EXTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTSEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EXTSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Timer 6 TRGO event
    #[inline(always)]
    pub fn tim6_trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM6_TRGO)
    }
    ///Timer 21 CH2 event
    #[inline(always)]
    pub fn tim21_ch2(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM21_CH2)
    }
    ///Timer 2 TRGO event
    #[inline(always)]
    pub fn tim2_trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM2_TRGO)
    }
    ///Timer 2 CH4 event
    #[inline(always)]
    pub fn tim2_ch4(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM2_CH4)
    }
    ///Timer 22 TRGO, Timer 21 TRGO event
    #[inline(always)]
    pub fn tim22_trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM22_TRGO)
    }
    ///Timer 2 CH3 event
    #[inline(always)]
    pub fn tim2_ch3(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM2_CH3)
    }
    ///Timer 3 TRGO event
    #[inline(always)]
    pub fn tim3_trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM3_TRGO)
    }
    ///EXTI line 11 event
    #[inline(always)]
    pub fn exti_line11(self) -> &'a mut W {
        self.variant(EXTSEL_A::EXTI_LINE11)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | ((value as u32 & 0x07) << 6);
        self.w
    }
}
///Data alignment
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALIGN_A {
    ///0: Right alignment
    RIGHT = 0,
    ///1: Left alignment
    LEFT = 1,
}
impl From<ALIGN_A> for bool {
    #[inline(always)]
    fn from(variant: ALIGN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ALIGN` reader - Data alignment
pub struct ALIGN_R(crate::FieldReader<bool, ALIGN_A>);
impl ALIGN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALIGN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ALIGN_A {
        match self.bits {
            false => ALIGN_A::RIGHT,
            true => ALIGN_A::LEFT,
        }
    }
    ///Checks if the value of the field is `RIGHT`
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        **self == ALIGN_A::RIGHT
    }
    ///Checks if the value of the field is `LEFT`
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        **self == ALIGN_A::LEFT
    }
}
impl core::ops::Deref for ALIGN_R {
    type Target = crate::FieldReader<bool, ALIGN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ALIGN` writer - Data alignment
pub struct ALIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALIGN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ALIGN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Right alignment
    #[inline(always)]
    pub fn right(self) -> &'a mut W {
        self.variant(ALIGN_A::RIGHT)
    }
    ///Left alignment
    #[inline(always)]
    pub fn left(self) -> &'a mut W {
        self.variant(ALIGN_A::LEFT)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
///Data resolution
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RES_A {
    ///0: 12 bits
    TWELVEBIT = 0,
    ///1: 10 bits
    TENBIT = 1,
    ///2: 8 bits
    EIGHTBIT = 2,
    ///3: 6 bits
    SIXBIT = 3,
}
impl From<RES_A> for u8 {
    #[inline(always)]
    fn from(variant: RES_A) -> Self {
        variant as _
    }
}
///Field `RES` reader - Data resolution
pub struct RES_R(crate::FieldReader<u8, RES_A>);
impl RES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RES_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RES_A {
        match self.bits {
            0 => RES_A::TWELVEBIT,
            1 => RES_A::TENBIT,
            2 => RES_A::EIGHTBIT,
            3 => RES_A::SIXBIT,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `TWELVEBIT`
    #[inline(always)]
    pub fn is_twelve_bit(&self) -> bool {
        **self == RES_A::TWELVEBIT
    }
    ///Checks if the value of the field is `TENBIT`
    #[inline(always)]
    pub fn is_ten_bit(&self) -> bool {
        **self == RES_A::TENBIT
    }
    ///Checks if the value of the field is `EIGHTBIT`
    #[inline(always)]
    pub fn is_eight_bit(&self) -> bool {
        **self == RES_A::EIGHTBIT
    }
    ///Checks if the value of the field is `SIXBIT`
    #[inline(always)]
    pub fn is_six_bit(&self) -> bool {
        **self == RES_A::SIXBIT
    }
}
impl core::ops::Deref for RES_R {
    type Target = crate::FieldReader<u8, RES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RES` writer - Data resolution
pub struct RES_W<'a> {
    w: &'a mut W,
}
impl<'a> RES_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RES_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///12 bits
    #[inline(always)]
    pub fn twelve_bit(self) -> &'a mut W {
        self.variant(RES_A::TWELVEBIT)
    }
    ///10 bits
    #[inline(always)]
    pub fn ten_bit(self) -> &'a mut W {
        self.variant(RES_A::TENBIT)
    }
    ///8 bits
    #[inline(always)]
    pub fn eight_bit(self) -> &'a mut W {
        self.variant(RES_A::EIGHTBIT)
    }
    ///6 bits
    #[inline(always)]
    pub fn six_bit(self) -> &'a mut W {
        self.variant(RES_A::SIXBIT)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
///Scan sequence direction
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCANDIR_A {
    ///0: Upward scan (from CHSEL0 to CHSEL18)
    UPWARD = 0,
    ///1: Backward scan (from CHSEL18 to CHSEL0)
    BACKWARD = 1,
}
impl From<SCANDIR_A> for bool {
    #[inline(always)]
    fn from(variant: SCANDIR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SCANDIR` reader - Scan sequence direction
pub struct SCANDIR_R(crate::FieldReader<bool, SCANDIR_A>);
impl SCANDIR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCANDIR_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SCANDIR_A {
        match self.bits {
            false => SCANDIR_A::UPWARD,
            true => SCANDIR_A::BACKWARD,
        }
    }
    ///Checks if the value of the field is `UPWARD`
    #[inline(always)]
    pub fn is_upward(&self) -> bool {
        **self == SCANDIR_A::UPWARD
    }
    ///Checks if the value of the field is `BACKWARD`
    #[inline(always)]
    pub fn is_backward(&self) -> bool {
        **self == SCANDIR_A::BACKWARD
    }
}
impl core::ops::Deref for SCANDIR_R {
    type Target = crate::FieldReader<bool, SCANDIR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SCANDIR` writer - Scan sequence direction
pub struct SCANDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> SCANDIR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SCANDIR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Upward scan (from CHSEL0 to CHSEL18)
    #[inline(always)]
    pub fn upward(self) -> &'a mut W {
        self.variant(SCANDIR_A::UPWARD)
    }
    ///Backward scan (from CHSEL18 to CHSEL0)
    #[inline(always)]
    pub fn backward(self) -> &'a mut W {
        self.variant(SCANDIR_A::BACKWARD)
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
///Direct memery access configuration
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMACFG_A {
    ///0: DMA one shot mode selected
    ONESHOT = 0,
    ///1: DMA circular mode selected
    CIRCULAR = 1,
}
impl From<DMACFG_A> for bool {
    #[inline(always)]
    fn from(variant: DMACFG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DMACFG` reader - Direct memery access configuration
pub struct DMACFG_R(crate::FieldReader<bool, DMACFG_A>);
impl DMACFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMACFG_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMACFG_A {
        match self.bits {
            false => DMACFG_A::ONESHOT,
            true => DMACFG_A::CIRCULAR,
        }
    }
    ///Checks if the value of the field is `ONESHOT`
    #[inline(always)]
    pub fn is_one_shot(&self) -> bool {
        **self == DMACFG_A::ONESHOT
    }
    ///Checks if the value of the field is `CIRCULAR`
    #[inline(always)]
    pub fn is_circular(&self) -> bool {
        **self == DMACFG_A::CIRCULAR
    }
}
impl core::ops::Deref for DMACFG_R {
    type Target = crate::FieldReader<bool, DMACFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DMACFG` writer - Direct memery access configuration
pub struct DMACFG_W<'a> {
    w: &'a mut W,
}
impl<'a> DMACFG_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DMACFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///DMA one shot mode selected
    #[inline(always)]
    pub fn one_shot(self) -> &'a mut W {
        self.variant(DMACFG_A::ONESHOT)
    }
    ///DMA circular mode selected
    #[inline(always)]
    pub fn circular(self) -> &'a mut W {
        self.variant(DMACFG_A::CIRCULAR)
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
///Direct memory access enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN_A {
    ///0: DMA disabled
    DISABLED = 0,
    ///1: DMA enabled
    ENABLED = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAEN` reader - Direct memory access enable
pub struct DMAEN_R(crate::FieldReader<bool, DMAEN_A>);
impl DMAEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMAEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::DISABLED,
            true => DMAEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DMAEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DMAEN_A::ENABLED
    }
}
impl core::ops::Deref for DMAEN_R {
    type Target = crate::FieldReader<bool, DMAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DMAEN` writer - Direct memory access enable
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DMAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///DMA disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAEN_A::DISABLED)
    }
    ///DMA enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAEN_A::ENABLED)
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
impl R {
    ///Bits 26:30 - Analog watchdog channel selection
    #[inline(always)]
    pub fn awdch(&self) -> AWDCH_R {
        AWDCH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    ///Bit 23 - Analog watchdog enable
    #[inline(always)]
    pub fn awden(&self) -> AWDEN_R {
        AWDEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 22 - Enable the watchdog on a single channel or on all channels
    #[inline(always)]
    pub fn awdsgl(&self) -> AWDSGL_R {
        AWDSGL_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 16 - Discontinuous mode
    #[inline(always)]
    pub fn discen(&self) -> DISCEN_R {
        DISCEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 15 - Auto-off mode
    #[inline(always)]
    pub fn autoff(&self) -> AUTOFF_R {
        AUTOFF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 14 - Auto-delayed conversion mode
    #[inline(always)]
    pub fn wait(&self) -> WAIT_R {
        WAIT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 13 - Single / continuous conversion mode
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 12 - Overrun management mode
    #[inline(always)]
    pub fn ovrmod(&self) -> OVRMOD_R {
        OVRMOD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bits 10:11 - External trigger enable and polarity selection
    #[inline(always)]
    pub fn exten(&self) -> EXTEN_R {
        EXTEN_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    ///Bits 6:8 - External trigger selection
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    ///Bit 5 - Data alignment
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bits 3:4 - Data resolution
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    ///Bit 2 - Scan sequence direction
    #[inline(always)]
    pub fn scandir(&self) -> SCANDIR_R {
        SCANDIR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Direct memery access configuration
    #[inline(always)]
    pub fn dmacfg(&self) -> DMACFG_R {
        DMACFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Direct memory access enable
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bits 26:30 - Analog watchdog channel selection
    #[inline(always)]
    pub fn awdch(&mut self) -> AWDCH_W {
        AWDCH_W { w: self }
    }
    ///Bit 23 - Analog watchdog enable
    #[inline(always)]
    pub fn awden(&mut self) -> AWDEN_W {
        AWDEN_W { w: self }
    }
    ///Bit 22 - Enable the watchdog on a single channel or on all channels
    #[inline(always)]
    pub fn awdsgl(&mut self) -> AWDSGL_W {
        AWDSGL_W { w: self }
    }
    ///Bit 16 - Discontinuous mode
    #[inline(always)]
    pub fn discen(&mut self) -> DISCEN_W {
        DISCEN_W { w: self }
    }
    ///Bit 15 - Auto-off mode
    #[inline(always)]
    pub fn autoff(&mut self) -> AUTOFF_W {
        AUTOFF_W { w: self }
    }
    ///Bit 14 - Auto-delayed conversion mode
    #[inline(always)]
    pub fn wait(&mut self) -> WAIT_W {
        WAIT_W { w: self }
    }
    ///Bit 13 - Single / continuous conversion mode
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W {
        CONT_W { w: self }
    }
    ///Bit 12 - Overrun management mode
    #[inline(always)]
    pub fn ovrmod(&mut self) -> OVRMOD_W {
        OVRMOD_W { w: self }
    }
    ///Bits 10:11 - External trigger enable and polarity selection
    #[inline(always)]
    pub fn exten(&mut self) -> EXTEN_W {
        EXTEN_W { w: self }
    }
    ///Bits 6:8 - External trigger selection
    #[inline(always)]
    pub fn extsel(&mut self) -> EXTSEL_W {
        EXTSEL_W { w: self }
    }
    ///Bit 5 - Data alignment
    #[inline(always)]
    pub fn align(&mut self) -> ALIGN_W {
        ALIGN_W { w: self }
    }
    ///Bits 3:4 - Data resolution
    #[inline(always)]
    pub fn res(&mut self) -> RES_W {
        RES_W { w: self }
    }
    ///Bit 2 - Scan sequence direction
    #[inline(always)]
    pub fn scandir(&mut self) -> SCANDIR_W {
        SCANDIR_W { w: self }
    }
    ///Bit 1 - Direct memery access configuration
    #[inline(always)]
    pub fn dmacfg(&mut self) -> DMACFG_W {
        DMACFG_W { w: self }
    }
    ///Bit 0 - Direct memory access enable
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///configuration register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfgr1](index.html) module
pub struct CFGR1_SPEC;
impl crate::RegisterSpec for CFGR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [cfgr1::R](R) reader structure
impl crate::Readable for CFGR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cfgr1::W](W) writer structure
impl crate::Writable for CFGR1_SPEC {
    type Writer = W;
}
///`reset()` method sets CFGR1 to value 0
impl crate::Resettable for CFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
