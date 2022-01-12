///Register `APB1_FZ` reader
pub struct R(crate::R<APB1_FZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1_FZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1_FZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1_FZ_SPEC>) -> Self {
        R(reader)
    }
}
///Register `APB1_FZ` writer
pub struct W(crate::W<APB1_FZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1_FZ_SPEC>;
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
impl From<crate::W<APB1_FZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1_FZ_SPEC>) -> Self {
        W(writer)
    }
}
///Debug Timer 2 stopped when Core is halted
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_TIMER2_STOP_A {
    ///0: The counter clock of TIMx is fed even if the core is halted
    CONTINUE = 0,
    ///1: The counter clock of TIMx is stopped when the core is halted
    STOP = 1,
}
impl From<DBG_TIMER2_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIMER2_STOP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_TIMER2_STOP` reader - Debug Timer 2 stopped when Core is halted
pub struct DBG_TIMER2_STOP_R(crate::FieldReader<bool, DBG_TIMER2_STOP_A>);
impl DBG_TIMER2_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIMER2_STOP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DBG_TIMER2_STOP_A {
        match self.bits {
            false => DBG_TIMER2_STOP_A::CONTINUE,
            true => DBG_TIMER2_STOP_A::STOP,
        }
    }
    ///Checks if the value of the field is `CONTINUE`
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        **self == DBG_TIMER2_STOP_A::CONTINUE
    }
    ///Checks if the value of the field is `STOP`
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == DBG_TIMER2_STOP_A::STOP
    }
}
impl core::ops::Deref for DBG_TIMER2_STOP_R {
    type Target = crate::FieldReader<bool, DBG_TIMER2_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DBG_TIMER2_STOP` writer - Debug Timer 2 stopped when Core is halted
pub struct DBG_TIMER2_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIMER2_STOP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DBG_TIMER2_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///The counter clock of TIMx is fed even if the core is halted
    #[inline(always)]
    pub fn continue_(self) -> &'a mut W {
        self.variant(DBG_TIMER2_STOP_A::CONTINUE)
    }
    ///The counter clock of TIMx is stopped when the core is halted
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(DBG_TIMER2_STOP_A::STOP)
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
///Debug Timer 6 stopped when Core is halted
pub type DBG_TIMER6_STOP_A = DBG_TIMER2_STOP_A;
///Field `DBG_TIMER6_STOP` reader - Debug Timer 6 stopped when Core is halted
pub type DBG_TIMER6_STOP_R = DBG_TIMER2_STOP_R;
///Field `DBG_TIMER6_STOP` writer - Debug Timer 6 stopped when Core is halted
pub struct DBG_TIMER6_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIMER6_STOP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DBG_TIMER6_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///The counter clock of TIMx is fed even if the core is halted
    #[inline(always)]
    pub fn continue_(self) -> &'a mut W {
        self.variant(DBG_TIMER6_STOP_A::CONTINUE)
    }
    ///The counter clock of TIMx is stopped when the core is halted
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(DBG_TIMER6_STOP_A::STOP)
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
///Debug RTC stopped when Core is halted
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_RTC_STOP_A {
    ///0: The clock of the RTC counter is fed even if the core is halted
    CONTINUE = 0,
    ///1: The clock of the RTC counter is stopped when the core is halted
    STOP = 1,
}
impl From<DBG_RTC_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_RTC_STOP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_RTC_STOP` reader - Debug RTC stopped when Core is halted
pub struct DBG_RTC_STOP_R(crate::FieldReader<bool, DBG_RTC_STOP_A>);
impl DBG_RTC_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBG_RTC_STOP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DBG_RTC_STOP_A {
        match self.bits {
            false => DBG_RTC_STOP_A::CONTINUE,
            true => DBG_RTC_STOP_A::STOP,
        }
    }
    ///Checks if the value of the field is `CONTINUE`
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        **self == DBG_RTC_STOP_A::CONTINUE
    }
    ///Checks if the value of the field is `STOP`
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == DBG_RTC_STOP_A::STOP
    }
}
impl core::ops::Deref for DBG_RTC_STOP_R {
    type Target = crate::FieldReader<bool, DBG_RTC_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DBG_RTC_STOP` writer - Debug RTC stopped when Core is halted
pub struct DBG_RTC_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_RTC_STOP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DBG_RTC_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///The clock of the RTC counter is fed even if the core is halted
    #[inline(always)]
    pub fn continue_(self) -> &'a mut W {
        self.variant(DBG_RTC_STOP_A::CONTINUE)
    }
    ///The clock of the RTC counter is stopped when the core is halted
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(DBG_RTC_STOP_A::STOP)
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
///Debug Window Wachdog stopped when Core is halted
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_WWDG_STOP_A {
    ///0: The window watchdog counter clock continues even if the core is halted
    CONTINUE = 0,
    ///1: The window watchdog counter clock is stopped when the core is halted
    STOP = 1,
}
impl From<DBG_WWDG_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_WWDG_STOP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_WWDG_STOP` reader - Debug Window Wachdog stopped when Core is halted
pub struct DBG_WWDG_STOP_R(crate::FieldReader<bool, DBG_WWDG_STOP_A>);
impl DBG_WWDG_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBG_WWDG_STOP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DBG_WWDG_STOP_A {
        match self.bits {
            false => DBG_WWDG_STOP_A::CONTINUE,
            true => DBG_WWDG_STOP_A::STOP,
        }
    }
    ///Checks if the value of the field is `CONTINUE`
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        **self == DBG_WWDG_STOP_A::CONTINUE
    }
    ///Checks if the value of the field is `STOP`
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == DBG_WWDG_STOP_A::STOP
    }
}
impl core::ops::Deref for DBG_WWDG_STOP_R {
    type Target = crate::FieldReader<bool, DBG_WWDG_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DBG_WWDG_STOP` writer - Debug Window Wachdog stopped when Core is halted
pub struct DBG_WWDG_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_WWDG_STOP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DBG_WWDG_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///The window watchdog counter clock continues even if the core is halted
    #[inline(always)]
    pub fn continue_(self) -> &'a mut W {
        self.variant(DBG_WWDG_STOP_A::CONTINUE)
    }
    ///The window watchdog counter clock is stopped when the core is halted
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(DBG_WWDG_STOP_A::STOP)
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
///Debug Independent Wachdog stopped when Core is halted
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_IWDG_STOP_A {
    ///0: The independent watchdog counter clock continues even if the core is halted
    CONTINUE = 0,
    ///1: The independent watchdog counter clock is stopped when the core is halted
    STOP = 1,
}
impl From<DBG_IWDG_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_IWDG_STOP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_IWDG_STOP` reader - Debug Independent Wachdog stopped when Core is halted
pub struct DBG_IWDG_STOP_R(crate::FieldReader<bool, DBG_IWDG_STOP_A>);
impl DBG_IWDG_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBG_IWDG_STOP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DBG_IWDG_STOP_A {
        match self.bits {
            false => DBG_IWDG_STOP_A::CONTINUE,
            true => DBG_IWDG_STOP_A::STOP,
        }
    }
    ///Checks if the value of the field is `CONTINUE`
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        **self == DBG_IWDG_STOP_A::CONTINUE
    }
    ///Checks if the value of the field is `STOP`
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == DBG_IWDG_STOP_A::STOP
    }
}
impl core::ops::Deref for DBG_IWDG_STOP_R {
    type Target = crate::FieldReader<bool, DBG_IWDG_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DBG_IWDG_STOP` writer - Debug Independent Wachdog stopped when Core is halted
pub struct DBG_IWDG_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_IWDG_STOP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DBG_IWDG_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///The independent watchdog counter clock continues even if the core is halted
    #[inline(always)]
    pub fn continue_(self) -> &'a mut W {
        self.variant(DBG_IWDG_STOP_A::CONTINUE)
    }
    ///The independent watchdog counter clock is stopped when the core is halted
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(DBG_IWDG_STOP_A::STOP)
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
///I2C1 SMBUS timeout mode stopped when core is halted
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_I2C1_STOP_A {
    ///0: Same behavior as in normal mode
    NORMALMODE = 0,
    ///1: I2C3 SMBUS timeout is frozen
    SMBUSTIMEOUTFROZEN = 1,
}
impl From<DBG_I2C1_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_I2C1_STOP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_I2C1_STOP` reader - I2C1 SMBUS timeout mode stopped when core is halted
pub struct DBG_I2C1_STOP_R(crate::FieldReader<bool, DBG_I2C1_STOP_A>);
impl DBG_I2C1_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBG_I2C1_STOP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DBG_I2C1_STOP_A {
        match self.bits {
            false => DBG_I2C1_STOP_A::NORMALMODE,
            true => DBG_I2C1_STOP_A::SMBUSTIMEOUTFROZEN,
        }
    }
    ///Checks if the value of the field is `NORMALMODE`
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        **self == DBG_I2C1_STOP_A::NORMALMODE
    }
    ///Checks if the value of the field is `SMBUSTIMEOUTFROZEN`
    #[inline(always)]
    pub fn is_smbus_timeout_frozen(&self) -> bool {
        **self == DBG_I2C1_STOP_A::SMBUSTIMEOUTFROZEN
    }
}
impl core::ops::Deref for DBG_I2C1_STOP_R {
    type Target = crate::FieldReader<bool, DBG_I2C1_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DBG_I2C1_STOP` writer - I2C1 SMBUS timeout mode stopped when core is halted
pub struct DBG_I2C1_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_I2C1_STOP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DBG_I2C1_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Same behavior as in normal mode
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut W {
        self.variant(DBG_I2C1_STOP_A::NORMALMODE)
    }
    ///I2C3 SMBUS timeout is frozen
    #[inline(always)]
    pub fn smbus_timeout_frozen(self) -> &'a mut W {
        self.variant(DBG_I2C1_STOP_A::SMBUSTIMEOUTFROZEN)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
///I2C2 SMBUS timeout mode stopped when core is halted
pub type DBG_I2C2_STOP_A = DBG_I2C1_STOP_A;
///Field `DBG_I2C2_STOP` reader - I2C2 SMBUS timeout mode stopped when core is halted
pub type DBG_I2C2_STOP_R = DBG_I2C1_STOP_R;
///Field `DBG_I2C2_STOP` writer - I2C2 SMBUS timeout mode stopped when core is halted
pub struct DBG_I2C2_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_I2C2_STOP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DBG_I2C2_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Same behavior as in normal mode
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut W {
        self.variant(DBG_I2C2_STOP_A::NORMALMODE)
    }
    ///I2C3 SMBUS timeout is frozen
    #[inline(always)]
    pub fn smbus_timeout_frozen(self) -> &'a mut W {
        self.variant(DBG_I2C2_STOP_A::SMBUSTIMEOUTFROZEN)
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
///LPTIM1 counter stopped when core is halted
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_LPTIMER_STOP_A {
    ///0: LPTIM1 counter clock is fed even if the core is halted
    CONTINUE = 0,
    ///1: LPTIM1 counter clock is stopped when the core is halted
    STOP = 1,
}
impl From<DBG_LPTIMER_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_LPTIMER_STOP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_LPTIMER_STOP` reader - LPTIM1 counter stopped when core is halted
pub struct DBG_LPTIMER_STOP_R(crate::FieldReader<bool, DBG_LPTIMER_STOP_A>);
impl DBG_LPTIMER_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBG_LPTIMER_STOP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DBG_LPTIMER_STOP_A {
        match self.bits {
            false => DBG_LPTIMER_STOP_A::CONTINUE,
            true => DBG_LPTIMER_STOP_A::STOP,
        }
    }
    ///Checks if the value of the field is `CONTINUE`
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        **self == DBG_LPTIMER_STOP_A::CONTINUE
    }
    ///Checks if the value of the field is `STOP`
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == DBG_LPTIMER_STOP_A::STOP
    }
}
impl core::ops::Deref for DBG_LPTIMER_STOP_R {
    type Target = crate::FieldReader<bool, DBG_LPTIMER_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DBG_LPTIMER_STOP` writer - LPTIM1 counter stopped when core is halted
pub struct DBG_LPTIMER_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_LPTIMER_STOP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DBG_LPTIMER_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///LPTIM1 counter clock is fed even if the core is halted
    #[inline(always)]
    pub fn continue_(self) -> &'a mut W {
        self.variant(DBG_LPTIMER_STOP_A::CONTINUE)
    }
    ///LPTIM1 counter clock is stopped when the core is halted
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(DBG_LPTIMER_STOP_A::STOP)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    ///Bit 0 - Debug Timer 2 stopped when Core is halted
    #[inline(always)]
    pub fn dbg_timer2_stop(&self) -> DBG_TIMER2_STOP_R {
        DBG_TIMER2_STOP_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 4 - Debug Timer 6 stopped when Core is halted
    #[inline(always)]
    pub fn dbg_timer6_stop(&self) -> DBG_TIMER6_STOP_R {
        DBG_TIMER6_STOP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 10 - Debug RTC stopped when Core is halted
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DBG_RTC_STOP_R {
        DBG_RTC_STOP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - Debug Window Wachdog stopped when Core is halted
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DBG_WWDG_STOP_R {
        DBG_WWDG_STOP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - Debug Independent Wachdog stopped when Core is halted
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DBG_IWDG_STOP_R {
        DBG_IWDG_STOP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 21 - I2C1 SMBUS timeout mode stopped when core is halted
    #[inline(always)]
    pub fn dbg_i2c1_stop(&self) -> DBG_I2C1_STOP_R {
        DBG_I2C1_STOP_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 22 - I2C2 SMBUS timeout mode stopped when core is halted
    #[inline(always)]
    pub fn dbg_i2c2_stop(&self) -> DBG_I2C2_STOP_R {
        DBG_I2C2_STOP_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 31 - LPTIM1 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_lptimer_stop(&self) -> DBG_LPTIMER_STOP_R {
        DBG_LPTIMER_STOP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Debug Timer 2 stopped when Core is halted
    #[inline(always)]
    pub fn dbg_timer2_stop(&mut self) -> DBG_TIMER2_STOP_W {
        DBG_TIMER2_STOP_W { w: self }
    }
    ///Bit 4 - Debug Timer 6 stopped when Core is halted
    #[inline(always)]
    pub fn dbg_timer6_stop(&mut self) -> DBG_TIMER6_STOP_W {
        DBG_TIMER6_STOP_W { w: self }
    }
    ///Bit 10 - Debug RTC stopped when Core is halted
    #[inline(always)]
    pub fn dbg_rtc_stop(&mut self) -> DBG_RTC_STOP_W {
        DBG_RTC_STOP_W { w: self }
    }
    ///Bit 11 - Debug Window Wachdog stopped when Core is halted
    #[inline(always)]
    pub fn dbg_wwdg_stop(&mut self) -> DBG_WWDG_STOP_W {
        DBG_WWDG_STOP_W { w: self }
    }
    ///Bit 12 - Debug Independent Wachdog stopped when Core is halted
    #[inline(always)]
    pub fn dbg_iwdg_stop(&mut self) -> DBG_IWDG_STOP_W {
        DBG_IWDG_STOP_W { w: self }
    }
    ///Bit 21 - I2C1 SMBUS timeout mode stopped when core is halted
    #[inline(always)]
    pub fn dbg_i2c1_stop(&mut self) -> DBG_I2C1_STOP_W {
        DBG_I2C1_STOP_W { w: self }
    }
    ///Bit 22 - I2C2 SMBUS timeout mode stopped when core is halted
    #[inline(always)]
    pub fn dbg_i2c2_stop(&mut self) -> DBG_I2C2_STOP_W {
        DBG_I2C2_STOP_W { w: self }
    }
    ///Bit 31 - LPTIM1 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_lptimer_stop(&mut self) -> DBG_LPTIMER_STOP_W {
        DBG_LPTIMER_STOP_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///APB Low Freeze Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb1_fz](index.html) module
pub struct APB1_FZ_SPEC;
impl crate::RegisterSpec for APB1_FZ_SPEC {
    type Ux = u32;
}
///`read()` method returns [apb1_fz::R](R) reader structure
impl crate::Readable for APB1_FZ_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [apb1_fz::W](W) writer structure
impl crate::Writable for APB1_FZ_SPEC {
    type Writer = W;
}
///`reset()` method sets APB1_FZ to value 0
impl crate::Resettable for APB1_FZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
