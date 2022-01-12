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
///Calibration output enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COE_A {
    ///0: Calibration output disabled
    DISABLED = 0,
    ///1: Calibration output enabled
    ENABLED = 1,
}
impl From<COE_A> for bool {
    #[inline(always)]
    fn from(variant: COE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `COE` reader - Calibration output enable
pub struct COE_R(crate::FieldReader<bool, COE_A>);
impl COE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> COE_A {
        match self.bits {
            false => COE_A::DISABLED,
            true => COE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == COE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == COE_A::ENABLED
    }
}
impl core::ops::Deref for COE_R {
    type Target = crate::FieldReader<bool, COE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `COE` writer - Calibration output enable
pub struct COE_W<'a> {
    w: &'a mut W,
}
impl<'a> COE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: COE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Calibration output disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COE_A::DISABLED)
    }
    ///Calibration output enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COE_A::ENABLED)
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
///Output selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OSEL_A {
    ///0: Output disabled
    DISABLED = 0,
    ///1: Alarm A output enabled
    ALARMA = 1,
    ///2: Alarm B output enabled
    ALARMB = 2,
    ///3: Wakeup output enabled
    WAKEUP = 3,
}
impl From<OSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: OSEL_A) -> Self {
        variant as _
    }
}
///Field `OSEL` reader - Output selection
pub struct OSEL_R(crate::FieldReader<u8, OSEL_A>);
impl OSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OSEL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OSEL_A {
        match self.bits {
            0 => OSEL_A::DISABLED,
            1 => OSEL_A::ALARMA,
            2 => OSEL_A::ALARMB,
            3 => OSEL_A::WAKEUP,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == OSEL_A::DISABLED
    }
    ///Checks if the value of the field is `ALARMA`
    #[inline(always)]
    pub fn is_alarm_a(&self) -> bool {
        **self == OSEL_A::ALARMA
    }
    ///Checks if the value of the field is `ALARMB`
    #[inline(always)]
    pub fn is_alarm_b(&self) -> bool {
        **self == OSEL_A::ALARMB
    }
    ///Checks if the value of the field is `WAKEUP`
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        **self == OSEL_A::WAKEUP
    }
}
impl core::ops::Deref for OSEL_R {
    type Target = crate::FieldReader<u8, OSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `OSEL` writer - Output selection
pub struct OSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OSEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Output disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OSEL_A::DISABLED)
    }
    ///Alarm A output enabled
    #[inline(always)]
    pub fn alarm_a(self) -> &'a mut W {
        self.variant(OSEL_A::ALARMA)
    }
    ///Alarm B output enabled
    #[inline(always)]
    pub fn alarm_b(self) -> &'a mut W {
        self.variant(OSEL_A::ALARMB)
    }
    ///Wakeup output enabled
    #[inline(always)]
    pub fn wakeup(self) -> &'a mut W {
        self.variant(OSEL_A::WAKEUP)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | ((value as u32 & 0x03) << 21);
        self.w
    }
}
///Output polarity
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL_A {
    ///0: The pin is high when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\[1:0\])
    HIGH = 0,
    ///1: The pin is low when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\[1:0\])
    LOW = 1,
}
impl From<POL_A> for bool {
    #[inline(always)]
    fn from(variant: POL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `POL` reader - Output polarity
pub struct POL_R(crate::FieldReader<bool, POL_A>);
impl POL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> POL_A {
        match self.bits {
            false => POL_A::HIGH,
            true => POL_A::LOW,
        }
    }
    ///Checks if the value of the field is `HIGH`
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == POL_A::HIGH
    }
    ///Checks if the value of the field is `LOW`
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == POL_A::LOW
    }
}
impl core::ops::Deref for POL_R {
    type Target = crate::FieldReader<bool, POL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `POL` writer - Output polarity
pub struct POL_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///The pin is high when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\[1:0\])
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POL_A::HIGH)
    }
    ///The pin is low when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\[1:0\])
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POL_A::LOW)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
///Calibration output selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COSEL_A {
    ///0: Calibration output is 512 Hz (with default prescaler setting)
    CALFREQ_512HZ = 0,
    ///1: Calibration output is 1 Hz (with default prescaler setting)
    CALFREQ_1HZ = 1,
}
impl From<COSEL_A> for bool {
    #[inline(always)]
    fn from(variant: COSEL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `COSEL` reader - Calibration output selection
pub struct COSEL_R(crate::FieldReader<bool, COSEL_A>);
impl COSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COSEL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> COSEL_A {
        match self.bits {
            false => COSEL_A::CALFREQ_512HZ,
            true => COSEL_A::CALFREQ_1HZ,
        }
    }
    ///Checks if the value of the field is `CALFREQ_512HZ`
    #[inline(always)]
    pub fn is_cal_freq_512hz(&self) -> bool {
        **self == COSEL_A::CALFREQ_512HZ
    }
    ///Checks if the value of the field is `CALFREQ_1HZ`
    #[inline(always)]
    pub fn is_cal_freq_1hz(&self) -> bool {
        **self == COSEL_A::CALFREQ_1HZ
    }
}
impl core::ops::Deref for COSEL_R {
    type Target = crate::FieldReader<bool, COSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `COSEL` writer - Calibration output selection
pub struct COSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COSEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: COSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Calibration output is 512 Hz (with default prescaler setting)
    #[inline(always)]
    pub fn cal_freq_512hz(self) -> &'a mut W {
        self.variant(COSEL_A::CALFREQ_512HZ)
    }
    ///Calibration output is 1 Hz (with default prescaler setting)
    #[inline(always)]
    pub fn cal_freq_1hz(self) -> &'a mut W {
        self.variant(COSEL_A::CALFREQ_1HZ)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
///Backup
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BKP_A {
    ///0: Daylight Saving Time change has not been performed
    DST_NOT_CHANGED = 0,
    ///1: Daylight Saving Time change has been performed
    DST_CHANGED = 1,
}
impl From<BKP_A> for bool {
    #[inline(always)]
    fn from(variant: BKP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BKP` reader - Backup
pub struct BKP_R(crate::FieldReader<bool, BKP_A>);
impl BKP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BKP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BKP_A {
        match self.bits {
            false => BKP_A::DST_NOT_CHANGED,
            true => BKP_A::DST_CHANGED,
        }
    }
    ///Checks if the value of the field is `DST_NOT_CHANGED`
    #[inline(always)]
    pub fn is_dst_not_changed(&self) -> bool {
        **self == BKP_A::DST_NOT_CHANGED
    }
    ///Checks if the value of the field is `DST_CHANGED`
    #[inline(always)]
    pub fn is_dst_changed(&self) -> bool {
        **self == BKP_A::DST_CHANGED
    }
}
impl core::ops::Deref for BKP_R {
    type Target = crate::FieldReader<bool, BKP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BKP` writer - Backup
pub struct BKP_W<'a> {
    w: &'a mut W,
}
impl<'a> BKP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BKP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Daylight Saving Time change has not been performed
    #[inline(always)]
    pub fn dst_not_changed(self) -> &'a mut W {
        self.variant(BKP_A::DST_NOT_CHANGED)
    }
    ///Daylight Saving Time change has been performed
    #[inline(always)]
    pub fn dst_changed(self) -> &'a mut W {
        self.variant(BKP_A::DST_CHANGED)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
///Subtract 1 hour (winter time change)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUB1H_AW {
    ///1: Subtracts 1 hour to the current time. This can be used for winter time change outside initialization mode
    SUB1 = 1,
}
impl From<SUB1H_AW> for bool {
    #[inline(always)]
    fn from(variant: SUB1H_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `SUB1H` writer - Subtract 1 hour (winter time change)
pub struct SUB1H_W<'a> {
    w: &'a mut W,
}
impl<'a> SUB1H_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SUB1H_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Subtracts 1 hour to the current time. This can be used for winter time change outside initialization mode
    #[inline(always)]
    pub fn sub1(self) -> &'a mut W {
        self.variant(SUB1H_AW::SUB1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
///Add 1 hour (summer time change)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADD1H_AW {
    ///1: Adds 1 hour to the current time. This can be used for summer time change outside initialization mode
    ADD1 = 1,
}
impl From<ADD1H_AW> for bool {
    #[inline(always)]
    fn from(variant: ADD1H_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `ADD1H` writer - Add 1 hour (summer time change)
pub struct ADD1H_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD1H_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ADD1H_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Adds 1 hour to the current time. This can be used for summer time change outside initialization mode
    #[inline(always)]
    pub fn add1(self) -> &'a mut W {
        self.variant(ADD1H_AW::ADD1)
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
///Time-stamp interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSIE_A {
    ///0: Time-stamp Interrupt disabled
    DISABLED = 0,
    ///1: Time-stamp Interrupt enabled
    ENABLED = 1,
}
impl From<TSIE_A> for bool {
    #[inline(always)]
    fn from(variant: TSIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TSIE` reader - Time-stamp interrupt enable
pub struct TSIE_R(crate::FieldReader<bool, TSIE_A>);
impl TSIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TSIE_A {
        match self.bits {
            false => TSIE_A::DISABLED,
            true => TSIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TSIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TSIE_A::ENABLED
    }
}
impl core::ops::Deref for TSIE_R {
    type Target = crate::FieldReader<bool, TSIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TSIE` writer - Time-stamp interrupt enable
pub struct TSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TSIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Time-stamp Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TSIE_A::DISABLED)
    }
    ///Time-stamp Interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TSIE_A::ENABLED)
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
///Wakeup timer interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUTIE_A {
    ///0: Wakeup timer interrupt disabled
    DISABLED = 0,
    ///1: Wakeup timer interrupt enabled
    ENABLED = 1,
}
impl From<WUTIE_A> for bool {
    #[inline(always)]
    fn from(variant: WUTIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WUTIE` reader - Wakeup timer interrupt enable
pub struct WUTIE_R(crate::FieldReader<bool, WUTIE_A>);
impl WUTIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WUTIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WUTIE_A {
        match self.bits {
            false => WUTIE_A::DISABLED,
            true => WUTIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WUTIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WUTIE_A::ENABLED
    }
}
impl core::ops::Deref for WUTIE_R {
    type Target = crate::FieldReader<bool, WUTIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WUTIE` writer - Wakeup timer interrupt enable
pub struct WUTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WUTIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WUTIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Wakeup timer interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WUTIE_A::DISABLED)
    }
    ///Wakeup timer interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WUTIE_A::ENABLED)
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
///Alarm B interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALRBIE_A {
    ///0: Alarm B Interrupt disabled
    DISABLED = 0,
    ///1: Alarm B Interrupt enabled
    ENABLED = 1,
}
impl From<ALRBIE_A> for bool {
    #[inline(always)]
    fn from(variant: ALRBIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ALRBIE` reader - Alarm B interrupt enable
pub struct ALRBIE_R(crate::FieldReader<bool, ALRBIE_A>);
impl ALRBIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALRBIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ALRBIE_A {
        match self.bits {
            false => ALRBIE_A::DISABLED,
            true => ALRBIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ALRBIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ALRBIE_A::ENABLED
    }
}
impl core::ops::Deref for ALRBIE_R {
    type Target = crate::FieldReader<bool, ALRBIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ALRBIE` writer - Alarm B interrupt enable
pub struct ALRBIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRBIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ALRBIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Alarm B Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ALRBIE_A::DISABLED)
    }
    ///Alarm B Interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ALRBIE_A::ENABLED)
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
///Alarm A interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALRAIE_A {
    ///0: Alarm A interrupt disabled
    DISABLED = 0,
    ///1: Alarm A interrupt enabled
    ENABLED = 1,
}
impl From<ALRAIE_A> for bool {
    #[inline(always)]
    fn from(variant: ALRAIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ALRAIE` reader - Alarm A interrupt enable
pub struct ALRAIE_R(crate::FieldReader<bool, ALRAIE_A>);
impl ALRAIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALRAIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ALRAIE_A {
        match self.bits {
            false => ALRAIE_A::DISABLED,
            true => ALRAIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ALRAIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ALRAIE_A::ENABLED
    }
}
impl core::ops::Deref for ALRAIE_R {
    type Target = crate::FieldReader<bool, ALRAIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ALRAIE` writer - Alarm A interrupt enable
pub struct ALRAIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRAIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ALRAIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Alarm A interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ALRAIE_A::DISABLED)
    }
    ///Alarm A interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ALRAIE_A::ENABLED)
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
///timestamp enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSE_A {
    ///0: Timestamp disabled
    DISABLED = 0,
    ///1: Timestamp enabled
    ENABLED = 1,
}
impl From<TSE_A> for bool {
    #[inline(always)]
    fn from(variant: TSE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TSE` reader - timestamp enable
pub struct TSE_R(crate::FieldReader<bool, TSE_A>);
impl TSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TSE_A {
        match self.bits {
            false => TSE_A::DISABLED,
            true => TSE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TSE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TSE_A::ENABLED
    }
}
impl core::ops::Deref for TSE_R {
    type Target = crate::FieldReader<bool, TSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TSE` writer - timestamp enable
pub struct TSE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TSE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Timestamp disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TSE_A::DISABLED)
    }
    ///Timestamp enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TSE_A::ENABLED)
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
///Wakeup timer enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUTE_A {
    ///0: Wakeup timer disabled
    DISABLED = 0,
    ///1: Wakeup timer enabled
    ENABLED = 1,
}
impl From<WUTE_A> for bool {
    #[inline(always)]
    fn from(variant: WUTE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WUTE` reader - Wakeup timer enable
pub struct WUTE_R(crate::FieldReader<bool, WUTE_A>);
impl WUTE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WUTE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WUTE_A {
        match self.bits {
            false => WUTE_A::DISABLED,
            true => WUTE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WUTE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WUTE_A::ENABLED
    }
}
impl core::ops::Deref for WUTE_R {
    type Target = crate::FieldReader<bool, WUTE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WUTE` writer - Wakeup timer enable
pub struct WUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> WUTE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WUTE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Wakeup timer disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WUTE_A::DISABLED)
    }
    ///Wakeup timer enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WUTE_A::ENABLED)
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
///Alarm B enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALRBE_A {
    ///0: Alarm B disabled
    DISABLED = 0,
    ///1: Alarm B enabled
    ENABLED = 1,
}
impl From<ALRBE_A> for bool {
    #[inline(always)]
    fn from(variant: ALRBE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ALRBE` reader - Alarm B enable
pub struct ALRBE_R(crate::FieldReader<bool, ALRBE_A>);
impl ALRBE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALRBE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ALRBE_A {
        match self.bits {
            false => ALRBE_A::DISABLED,
            true => ALRBE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ALRBE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ALRBE_A::ENABLED
    }
}
impl core::ops::Deref for ALRBE_R {
    type Target = crate::FieldReader<bool, ALRBE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ALRBE` writer - Alarm B enable
pub struct ALRBE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRBE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ALRBE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Alarm B disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ALRBE_A::DISABLED)
    }
    ///Alarm B enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ALRBE_A::ENABLED)
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
///Alarm A enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALRAE_A {
    ///0: Alarm A disabled
    DISABLED = 0,
    ///1: Alarm A enabled
    ENABLED = 1,
}
impl From<ALRAE_A> for bool {
    #[inline(always)]
    fn from(variant: ALRAE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ALRAE` reader - Alarm A enable
pub struct ALRAE_R(crate::FieldReader<bool, ALRAE_A>);
impl ALRAE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALRAE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ALRAE_A {
        match self.bits {
            false => ALRAE_A::DISABLED,
            true => ALRAE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ALRAE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ALRAE_A::ENABLED
    }
}
impl core::ops::Deref for ALRAE_R {
    type Target = crate::FieldReader<bool, ALRAE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ALRAE` writer - Alarm A enable
pub struct ALRAE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRAE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ALRAE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Alarm A disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ALRAE_A::DISABLED)
    }
    ///Alarm A enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ALRAE_A::ENABLED)
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
///Hour format
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FMT_A {
    ///0: 24 hour/day format
    TWENTY_FOUR_HOUR = 0,
    ///1: AM/PM hour format
    AM_PM = 1,
}
impl From<FMT_A> for bool {
    #[inline(always)]
    fn from(variant: FMT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FMT` reader - Hour format
pub struct FMT_R(crate::FieldReader<bool, FMT_A>);
impl FMT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FMT_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FMT_A {
        match self.bits {
            false => FMT_A::TWENTY_FOUR_HOUR,
            true => FMT_A::AM_PM,
        }
    }
    ///Checks if the value of the field is `TWENTY_FOUR_HOUR`
    #[inline(always)]
    pub fn is_twenty_four_hour(&self) -> bool {
        **self == FMT_A::TWENTY_FOUR_HOUR
    }
    ///Checks if the value of the field is `AM_PM`
    #[inline(always)]
    pub fn is_am_pm(&self) -> bool {
        **self == FMT_A::AM_PM
    }
}
impl core::ops::Deref for FMT_R {
    type Target = crate::FieldReader<bool, FMT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `FMT` writer - Hour format
pub struct FMT_W<'a> {
    w: &'a mut W,
}
impl<'a> FMT_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FMT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///24 hour/day format
    #[inline(always)]
    pub fn twenty_four_hour(self) -> &'a mut W {
        self.variant(FMT_A::TWENTY_FOUR_HOUR)
    }
    ///AM/PM hour format
    #[inline(always)]
    pub fn am_pm(self) -> &'a mut W {
        self.variant(FMT_A::AM_PM)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
///Bypass the shadow registers
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPSHAD_A {
    ///0: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken from the shadow registers, which are updated once every two RTCCLK cycles
    SHADOWREG = 0,
    ///1: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken directly from the calendar counters
    BYPASSSHADOWREG = 1,
}
impl From<BYPSHAD_A> for bool {
    #[inline(always)]
    fn from(variant: BYPSHAD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BYPSHAD` reader - Bypass the shadow registers
pub struct BYPSHAD_R(crate::FieldReader<bool, BYPSHAD_A>);
impl BYPSHAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYPSHAD_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BYPSHAD_A {
        match self.bits {
            false => BYPSHAD_A::SHADOWREG,
            true => BYPSHAD_A::BYPASSSHADOWREG,
        }
    }
    ///Checks if the value of the field is `SHADOWREG`
    #[inline(always)]
    pub fn is_shadow_reg(&self) -> bool {
        **self == BYPSHAD_A::SHADOWREG
    }
    ///Checks if the value of the field is `BYPASSSHADOWREG`
    #[inline(always)]
    pub fn is_bypass_shadow_reg(&self) -> bool {
        **self == BYPSHAD_A::BYPASSSHADOWREG
    }
}
impl core::ops::Deref for BYPSHAD_R {
    type Target = crate::FieldReader<bool, BYPSHAD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BYPSHAD` writer - Bypass the shadow registers
pub struct BYPSHAD_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPSHAD_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BYPSHAD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken from the shadow registers, which are updated once every two RTCCLK cycles
    #[inline(always)]
    pub fn shadow_reg(self) -> &'a mut W {
        self.variant(BYPSHAD_A::SHADOWREG)
    }
    ///Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken directly from the calendar counters
    #[inline(always)]
    pub fn bypass_shadow_reg(self) -> &'a mut W {
        self.variant(BYPSHAD_A::BYPASSSHADOWREG)
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
///RTC_REFIN reference clock detection enable (50 or 60 Hz)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFCKON_A {
    ///0: RTC_REFIN detection disabled
    DISABLED = 0,
    ///1: RTC_REFIN detection enabled
    ENABLED = 1,
}
impl From<REFCKON_A> for bool {
    #[inline(always)]
    fn from(variant: REFCKON_A) -> Self {
        variant as u8 != 0
    }
}
///Field `REFCKON` reader - RTC_REFIN reference clock detection enable (50 or 60 Hz)
pub struct REFCKON_R(crate::FieldReader<bool, REFCKON_A>);
impl REFCKON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REFCKON_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> REFCKON_A {
        match self.bits {
            false => REFCKON_A::DISABLED,
            true => REFCKON_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REFCKON_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REFCKON_A::ENABLED
    }
}
impl core::ops::Deref for REFCKON_R {
    type Target = crate::FieldReader<bool, REFCKON_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `REFCKON` writer - RTC_REFIN reference clock detection enable (50 or 60 Hz)
pub struct REFCKON_W<'a> {
    w: &'a mut W,
}
impl<'a> REFCKON_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: REFCKON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///RTC_REFIN detection disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REFCKON_A::DISABLED)
    }
    ///RTC_REFIN detection enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REFCKON_A::ENABLED)
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
///Time-stamp event active edge
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSEDGE_A {
    ///0: RTC_TS input rising edge generates a time-stamp event
    RISINGEDGE = 0,
    ///1: RTC_TS input falling edge generates a time-stamp event
    FALLINGEDGE = 1,
}
impl From<TSEDGE_A> for bool {
    #[inline(always)]
    fn from(variant: TSEDGE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TSEDGE` reader - Time-stamp event active edge
pub struct TSEDGE_R(crate::FieldReader<bool, TSEDGE_A>);
impl TSEDGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSEDGE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TSEDGE_A {
        match self.bits {
            false => TSEDGE_A::RISINGEDGE,
            true => TSEDGE_A::FALLINGEDGE,
        }
    }
    ///Checks if the value of the field is `RISINGEDGE`
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        **self == TSEDGE_A::RISINGEDGE
    }
    ///Checks if the value of the field is `FALLINGEDGE`
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        **self == TSEDGE_A::FALLINGEDGE
    }
}
impl core::ops::Deref for TSEDGE_R {
    type Target = crate::FieldReader<bool, TSEDGE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TSEDGE` writer - Time-stamp event active edge
pub struct TSEDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEDGE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TSEDGE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///RTC_TS input rising edge generates a time-stamp event
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(TSEDGE_A::RISINGEDGE)
    }
    ///RTC_TS input falling edge generates a time-stamp event
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(TSEDGE_A::FALLINGEDGE)
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
///Wakeup clock selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WUCKSEL_A {
    ///0: RTC/16 clock is selected
    DIV16 = 0,
    ///1: RTC/8 clock is selected
    DIV8 = 1,
    ///2: RTC/4 clock is selected
    DIV4 = 2,
    ///3: RTC/2 clock is selected
    DIV2 = 3,
    ///4: ck_spre (usually 1 Hz) clock is selected
    CLOCKSPARE = 4,
    ///6: ck_spre (usually 1 Hz) clock is selected and 2^16 is added to the WUT counter value
    CLOCKSPAREWITHOFFSET = 6,
}
impl From<WUCKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: WUCKSEL_A) -> Self {
        variant as _
    }
}
///Field `WUCKSEL` reader - Wakeup clock selection
pub struct WUCKSEL_R(crate::FieldReader<u8, WUCKSEL_A>);
impl WUCKSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WUCKSEL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<WUCKSEL_A> {
        match self.bits {
            0 => Some(WUCKSEL_A::DIV16),
            1 => Some(WUCKSEL_A::DIV8),
            2 => Some(WUCKSEL_A::DIV4),
            3 => Some(WUCKSEL_A::DIV2),
            4 => Some(WUCKSEL_A::CLOCKSPARE),
            6 => Some(WUCKSEL_A::CLOCKSPAREWITHOFFSET),
            _ => None,
        }
    }
    ///Checks if the value of the field is `DIV16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        **self == WUCKSEL_A::DIV16
    }
    ///Checks if the value of the field is `DIV8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == WUCKSEL_A::DIV8
    }
    ///Checks if the value of the field is `DIV4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == WUCKSEL_A::DIV4
    }
    ///Checks if the value of the field is `DIV2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == WUCKSEL_A::DIV2
    }
    ///Checks if the value of the field is `CLOCKSPARE`
    #[inline(always)]
    pub fn is_clock_spare(&self) -> bool {
        **self == WUCKSEL_A::CLOCKSPARE
    }
    ///Checks if the value of the field is `CLOCKSPAREWITHOFFSET`
    #[inline(always)]
    pub fn is_clock_spare_with_offset(&self) -> bool {
        **self == WUCKSEL_A::CLOCKSPAREWITHOFFSET
    }
}
impl core::ops::Deref for WUCKSEL_R {
    type Target = crate::FieldReader<u8, WUCKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WUCKSEL` writer - Wakeup clock selection
pub struct WUCKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WUCKSEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WUCKSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///RTC/16 clock is selected
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(WUCKSEL_A::DIV16)
    }
    ///RTC/8 clock is selected
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(WUCKSEL_A::DIV8)
    }
    ///RTC/4 clock is selected
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(WUCKSEL_A::DIV4)
    }
    ///RTC/2 clock is selected
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(WUCKSEL_A::DIV2)
    }
    ///ck_spre (usually 1 Hz) clock is selected
    #[inline(always)]
    pub fn clock_spare(self) -> &'a mut W {
        self.variant(WUCKSEL_A::CLOCKSPARE)
    }
    ///ck_spre (usually 1 Hz) clock is selected and 2^16 is added to the WUT counter value
    #[inline(always)]
    pub fn clock_spare_with_offset(self) -> &'a mut W {
        self.variant(WUCKSEL_A::CLOCKSPAREWITHOFFSET)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    ///Bit 23 - Calibration output enable
    #[inline(always)]
    pub fn coe(&self) -> COE_R {
        COE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bits 21:22 - Output selection
    #[inline(always)]
    pub fn osel(&self) -> OSEL_R {
        OSEL_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    ///Bit 20 - Output polarity
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 19 - Calibration output selection
    #[inline(always)]
    pub fn cosel(&self) -> COSEL_R {
        COSEL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 18 - Backup
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 15 - Time-stamp interrupt enable
    #[inline(always)]
    pub fn tsie(&self) -> TSIE_R {
        TSIE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 14 - Wakeup timer interrupt enable
    #[inline(always)]
    pub fn wutie(&self) -> WUTIE_R {
        WUTIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 13 - Alarm B interrupt enable
    #[inline(always)]
    pub fn alrbie(&self) -> ALRBIE_R {
        ALRBIE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 12 - Alarm A interrupt enable
    #[inline(always)]
    pub fn alraie(&self) -> ALRAIE_R {
        ALRAIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 11 - timestamp enable
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - Wakeup timer enable
    #[inline(always)]
    pub fn wute(&self) -> WUTE_R {
        WUTE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 9 - Alarm B enable
    #[inline(always)]
    pub fn alrbe(&self) -> ALRBE_R {
        ALRBE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - Alarm A enable
    #[inline(always)]
    pub fn alrae(&self) -> ALRAE_R {
        ALRAE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 6 - Hour format
    #[inline(always)]
    pub fn fmt(&self) -> FMT_R {
        FMT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - Bypass the shadow registers
    #[inline(always)]
    pub fn bypshad(&self) -> BYPSHAD_R {
        BYPSHAD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - RTC_REFIN reference clock detection enable (50 or 60 Hz)
    #[inline(always)]
    pub fn refckon(&self) -> REFCKON_R {
        REFCKON_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - Time-stamp event active edge
    #[inline(always)]
    pub fn tsedge(&self) -> TSEDGE_R {
        TSEDGE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bits 0:2 - Wakeup clock selection
    #[inline(always)]
    pub fn wucksel(&self) -> WUCKSEL_R {
        WUCKSEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    ///Bit 23 - Calibration output enable
    #[inline(always)]
    pub fn coe(&mut self) -> COE_W {
        COE_W { w: self }
    }
    ///Bits 21:22 - Output selection
    #[inline(always)]
    pub fn osel(&mut self) -> OSEL_W {
        OSEL_W { w: self }
    }
    ///Bit 20 - Output polarity
    #[inline(always)]
    pub fn pol(&mut self) -> POL_W {
        POL_W { w: self }
    }
    ///Bit 19 - Calibration output selection
    #[inline(always)]
    pub fn cosel(&mut self) -> COSEL_W {
        COSEL_W { w: self }
    }
    ///Bit 18 - Backup
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W {
        BKP_W { w: self }
    }
    ///Bit 17 - Subtract 1 hour (winter time change)
    #[inline(always)]
    pub fn sub1h(&mut self) -> SUB1H_W {
        SUB1H_W { w: self }
    }
    ///Bit 16 - Add 1 hour (summer time change)
    #[inline(always)]
    pub fn add1h(&mut self) -> ADD1H_W {
        ADD1H_W { w: self }
    }
    ///Bit 15 - Time-stamp interrupt enable
    #[inline(always)]
    pub fn tsie(&mut self) -> TSIE_W {
        TSIE_W { w: self }
    }
    ///Bit 14 - Wakeup timer interrupt enable
    #[inline(always)]
    pub fn wutie(&mut self) -> WUTIE_W {
        WUTIE_W { w: self }
    }
    ///Bit 13 - Alarm B interrupt enable
    #[inline(always)]
    pub fn alrbie(&mut self) -> ALRBIE_W {
        ALRBIE_W { w: self }
    }
    ///Bit 12 - Alarm A interrupt enable
    #[inline(always)]
    pub fn alraie(&mut self) -> ALRAIE_W {
        ALRAIE_W { w: self }
    }
    ///Bit 11 - timestamp enable
    #[inline(always)]
    pub fn tse(&mut self) -> TSE_W {
        TSE_W { w: self }
    }
    ///Bit 10 - Wakeup timer enable
    #[inline(always)]
    pub fn wute(&mut self) -> WUTE_W {
        WUTE_W { w: self }
    }
    ///Bit 9 - Alarm B enable
    #[inline(always)]
    pub fn alrbe(&mut self) -> ALRBE_W {
        ALRBE_W { w: self }
    }
    ///Bit 8 - Alarm A enable
    #[inline(always)]
    pub fn alrae(&mut self) -> ALRAE_W {
        ALRAE_W { w: self }
    }
    ///Bit 6 - Hour format
    #[inline(always)]
    pub fn fmt(&mut self) -> FMT_W {
        FMT_W { w: self }
    }
    ///Bit 5 - Bypass the shadow registers
    #[inline(always)]
    pub fn bypshad(&mut self) -> BYPSHAD_W {
        BYPSHAD_W { w: self }
    }
    ///Bit 4 - RTC_REFIN reference clock detection enable (50 or 60 Hz)
    #[inline(always)]
    pub fn refckon(&mut self) -> REFCKON_W {
        REFCKON_W { w: self }
    }
    ///Bit 3 - Time-stamp event active edge
    #[inline(always)]
    pub fn tsedge(&mut self) -> TSEDGE_W {
        TSEDGE_W { w: self }
    }
    ///Bits 0:2 - Wakeup clock selection
    #[inline(always)]
    pub fn wucksel(&mut self) -> WUCKSEL_W {
        WUCKSEL_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RTC control register
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
