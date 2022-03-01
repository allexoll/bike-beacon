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
///Field `ADDCODE` reader - Address match code (Slave mode)
pub struct ADDCODE_R(crate::FieldReader<u8, u8>);
impl ADDCODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADDCODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDCODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Transfer direction (Slave mode)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR_A {
    ///0: Write transfer, slave enters receiver mode
    WRITE = 0,
    ///1: Read transfer, slave enters transmitter mode
    READ = 1,
}
impl From<DIR_A> for bool {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIR` reader - Transfer direction (Slave mode)
pub struct DIR_R(crate::FieldReader<bool, DIR_A>);
impl DIR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIR_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DIR_A {
        match self.bits {
            false => DIR_A::WRITE,
            true => DIR_A::READ,
        }
    }
    ///Checks if the value of the field is `WRITE`
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        **self == DIR_A::WRITE
    }
    ///Checks if the value of the field is `READ`
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == DIR_A::READ
    }
}
impl core::ops::Deref for DIR_R {
    type Target = crate::FieldReader<bool, DIR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Bus busy
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    ///0: No communication is in progress on the bus
    NOTBUSY = 0,
    ///1: A communication is in progress on the bus
    BUSY = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BUSY` reader - Bus busy
pub struct BUSY_R(crate::FieldReader<bool, BUSY_A>);
impl BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::NOTBUSY,
            true => BUSY_A::BUSY,
        }
    }
    ///Checks if the value of the field is `NOTBUSY`
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        **self == BUSY_A::NOTBUSY
    }
    ///Checks if the value of the field is `BUSY`
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == BUSY_A::BUSY
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, BUSY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///SMBus alert
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALERT_A {
    ///0: SMBA alert is not detected
    NOALERT = 0,
    ///1: SMBA alert event is detected on SMBA pin
    ALERT = 1,
}
impl From<ALERT_A> for bool {
    #[inline(always)]
    fn from(variant: ALERT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ALERT` reader - SMBus alert
pub struct ALERT_R(crate::FieldReader<bool, ALERT_A>);
impl ALERT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALERT_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ALERT_A {
        match self.bits {
            false => ALERT_A::NOALERT,
            true => ALERT_A::ALERT,
        }
    }
    ///Checks if the value of the field is `NOALERT`
    #[inline(always)]
    pub fn is_no_alert(&self) -> bool {
        **self == ALERT_A::NOALERT
    }
    ///Checks if the value of the field is `ALERT`
    #[inline(always)]
    pub fn is_alert(&self) -> bool {
        **self == ALERT_A::ALERT
    }
}
impl core::ops::Deref for ALERT_R {
    type Target = crate::FieldReader<bool, ALERT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Timeout or t_low detection flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEOUT_A {
    ///0: No timeout occured
    NOTIMEOUT = 0,
    ///1: Timeout occured
    TIMEOUT = 1,
}
impl From<TIMEOUT_A> for bool {
    #[inline(always)]
    fn from(variant: TIMEOUT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TIMEOUT` reader - Timeout or t_low detection flag
pub struct TIMEOUT_R(crate::FieldReader<bool, TIMEOUT_A>);
impl TIMEOUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMEOUT_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TIMEOUT_A {
        match self.bits {
            false => TIMEOUT_A::NOTIMEOUT,
            true => TIMEOUT_A::TIMEOUT,
        }
    }
    ///Checks if the value of the field is `NOTIMEOUT`
    #[inline(always)]
    pub fn is_no_timeout(&self) -> bool {
        **self == TIMEOUT_A::NOTIMEOUT
    }
    ///Checks if the value of the field is `TIMEOUT`
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        **self == TIMEOUT_A::TIMEOUT
    }
}
impl core::ops::Deref for TIMEOUT_R {
    type Target = crate::FieldReader<bool, TIMEOUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///PEC Error in reception
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PECERR_A {
    ///0: Received PEC does match with PEC register
    MATCH = 0,
    ///1: Received PEC does not match with PEC register
    NOMATCH = 1,
}
impl From<PECERR_A> for bool {
    #[inline(always)]
    fn from(variant: PECERR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PECERR` reader - PEC Error in reception
pub struct PECERR_R(crate::FieldReader<bool, PECERR_A>);
impl PECERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PECERR_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PECERR_A {
        match self.bits {
            false => PECERR_A::MATCH,
            true => PECERR_A::NOMATCH,
        }
    }
    ///Checks if the value of the field is `MATCH`
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        **self == PECERR_A::MATCH
    }
    ///Checks if the value of the field is `NOMATCH`
    #[inline(always)]
    pub fn is_no_match(&self) -> bool {
        **self == PECERR_A::NOMATCH
    }
}
impl core::ops::Deref for PECERR_R {
    type Target = crate::FieldReader<bool, PECERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Overrun/Underrun (slave mode)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVR_A {
    ///0: No overrun/underrun error occurs
    NOOVERRUN = 0,
    ///1: slave mode with NOSTRETCH=1, when an overrun/underrun error occurs
    OVERRUN = 1,
}
impl From<OVR_A> for bool {
    #[inline(always)]
    fn from(variant: OVR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OVR` reader - Overrun/Underrun (slave mode)
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
///Arbitration lost
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARLO_A {
    ///0: No arbitration lost
    NOTLOST = 0,
    ///1: Arbitration lost
    LOST = 1,
}
impl From<ARLO_A> for bool {
    #[inline(always)]
    fn from(variant: ARLO_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ARLO` reader - Arbitration lost
pub struct ARLO_R(crate::FieldReader<bool, ARLO_A>);
impl ARLO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ARLO_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ARLO_A {
        match self.bits {
            false => ARLO_A::NOTLOST,
            true => ARLO_A::LOST,
        }
    }
    ///Checks if the value of the field is `NOTLOST`
    #[inline(always)]
    pub fn is_not_lost(&self) -> bool {
        **self == ARLO_A::NOTLOST
    }
    ///Checks if the value of the field is `LOST`
    #[inline(always)]
    pub fn is_lost(&self) -> bool {
        **self == ARLO_A::LOST
    }
}
impl core::ops::Deref for ARLO_R {
    type Target = crate::FieldReader<bool, ARLO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Bus error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BERR_A {
    ///0: No bus error
    NOERROR = 0,
    ///1: Misplaced Start and Stop condition is detected
    ERROR = 1,
}
impl From<BERR_A> for bool {
    #[inline(always)]
    fn from(variant: BERR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BERR` reader - Bus error
pub struct BERR_R(crate::FieldReader<bool, BERR_A>);
impl BERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BERR_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BERR_A {
        match self.bits {
            false => BERR_A::NOERROR,
            true => BERR_A::ERROR,
        }
    }
    ///Checks if the value of the field is `NOERROR`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == BERR_A::NOERROR
    }
    ///Checks if the value of the field is `ERROR`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == BERR_A::ERROR
    }
}
impl core::ops::Deref for BERR_R {
    type Target = crate::FieldReader<bool, BERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Transfer Complete Reload
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCR_A {
    ///0: Transfer is not complete
    NOTCOMPLETE = 0,
    ///1: NBYTES has been transfered
    COMPLETE = 1,
}
impl From<TCR_A> for bool {
    #[inline(always)]
    fn from(variant: TCR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TCR` reader - Transfer Complete Reload
pub struct TCR_R(crate::FieldReader<bool, TCR_A>);
impl TCR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCR_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TCR_A {
        match self.bits {
            false => TCR_A::NOTCOMPLETE,
            true => TCR_A::COMPLETE,
        }
    }
    ///Checks if the value of the field is `NOTCOMPLETE`
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        **self == TCR_A::NOTCOMPLETE
    }
    ///Checks if the value of the field is `COMPLETE`
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        **self == TCR_A::COMPLETE
    }
}
impl core::ops::Deref for TCR_R {
    type Target = crate::FieldReader<bool, TCR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Transfer Complete (master mode)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC_A {
    ///0: Transfer is not complete
    NOTCOMPLETE = 0,
    ///1: NBYTES has been transfered
    COMPLETE = 1,
}
impl From<TC_A> for bool {
    #[inline(always)]
    fn from(variant: TC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TC` reader - Transfer Complete (master mode)
pub struct TC_R(crate::FieldReader<bool, TC_A>);
impl TC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TC_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TC_A {
        match self.bits {
            false => TC_A::NOTCOMPLETE,
            true => TC_A::COMPLETE,
        }
    }
    ///Checks if the value of the field is `NOTCOMPLETE`
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        **self == TC_A::NOTCOMPLETE
    }
    ///Checks if the value of the field is `COMPLETE`
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        **self == TC_A::COMPLETE
    }
}
impl core::ops::Deref for TC_R {
    type Target = crate::FieldReader<bool, TC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Stop detection flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPF_A {
    ///0: No Stop condition detected
    NOSTOP = 0,
    ///1: Stop condition detected
    STOP = 1,
}
impl From<STOPF_A> for bool {
    #[inline(always)]
    fn from(variant: STOPF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `STOPF` reader - Stop detection flag
pub struct STOPF_R(crate::FieldReader<bool, STOPF_A>);
impl STOPF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STOPF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> STOPF_A {
        match self.bits {
            false => STOPF_A::NOSTOP,
            true => STOPF_A::STOP,
        }
    }
    ///Checks if the value of the field is `NOSTOP`
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        **self == STOPF_A::NOSTOP
    }
    ///Checks if the value of the field is `STOP`
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == STOPF_A::STOP
    }
}
impl core::ops::Deref for STOPF_R {
    type Target = crate::FieldReader<bool, STOPF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Not acknowledge received flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACKF_A {
    ///0: No NACK has been received
    NONACK = 0,
    ///1: NACK has been received
    NACK = 1,
}
impl From<NACKF_A> for bool {
    #[inline(always)]
    fn from(variant: NACKF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `NACKF` reader - Not acknowledge received flag
pub struct NACKF_R(crate::FieldReader<bool, NACKF_A>);
impl NACKF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NACKF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> NACKF_A {
        match self.bits {
            false => NACKF_A::NONACK,
            true => NACKF_A::NACK,
        }
    }
    ///Checks if the value of the field is `NONACK`
    #[inline(always)]
    pub fn is_no_nack(&self) -> bool {
        **self == NACKF_A::NONACK
    }
    ///Checks if the value of the field is `NACK`
    #[inline(always)]
    pub fn is_nack(&self) -> bool {
        **self == NACKF_A::NACK
    }
}
impl core::ops::Deref for NACKF_R {
    type Target = crate::FieldReader<bool, NACKF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Address matched (slave mode)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDR_A {
    ///0: Adress mismatched or not received
    NOTMATCH = 0,
    ///1: Received slave address matched with one of the enabled slave addresses
    MATCH = 1,
}
impl From<ADDR_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADDR` reader - Address matched (slave mode)
pub struct ADDR_R(crate::FieldReader<bool, ADDR_A>);
impl ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADDR_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADDR_A {
        match self.bits {
            false => ADDR_A::NOTMATCH,
            true => ADDR_A::MATCH,
        }
    }
    ///Checks if the value of the field is `NOTMATCH`
    #[inline(always)]
    pub fn is_not_match(&self) -> bool {
        **self == ADDR_A::NOTMATCH
    }
    ///Checks if the value of the field is `MATCH`
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        **self == ADDR_A::MATCH
    }
}
impl core::ops::Deref for ADDR_R {
    type Target = crate::FieldReader<bool, ADDR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Receive data register not empty (receivers)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXNE_A {
    ///0: The RXDR register is empty
    EMPTY = 0,
    ///1: Received data is copied into the RXDR register, and is ready to be read
    NOTEMPTY = 1,
}
impl From<RXNE_A> for bool {
    #[inline(always)]
    fn from(variant: RXNE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RXNE` reader - Receive data register not empty (receivers)
pub struct RXNE_R(crate::FieldReader<bool, RXNE_A>);
impl RXNE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXNE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXNE_A {
        match self.bits {
            false => RXNE_A::EMPTY,
            true => RXNE_A::NOTEMPTY,
        }
    }
    ///Checks if the value of the field is `EMPTY`
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        **self == RXNE_A::EMPTY
    }
    ///Checks if the value of the field is `NOTEMPTY`
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        **self == RXNE_A::NOTEMPTY
    }
}
impl core::ops::Deref for RXNE_R {
    type Target = crate::FieldReader<bool, RXNE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Transmit interrupt status (transmitters)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXIS_A {
    ///0: The TXDR register is not empty
    NOTEMPTY = 0,
    ///1: The TXDR register is empty and the data to be transmitted must be written in the TXDR register
    EMPTY = 1,
}
impl From<TXIS_A> for bool {
    #[inline(always)]
    fn from(variant: TXIS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TXIS` reader - Transmit interrupt status (transmitters)
pub struct TXIS_R(crate::FieldReader<bool, TXIS_A>);
impl TXIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXIS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXIS_A {
        match self.bits {
            false => TXIS_A::NOTEMPTY,
            true => TXIS_A::EMPTY,
        }
    }
    ///Checks if the value of the field is `NOTEMPTY`
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        **self == TXIS_A::NOTEMPTY
    }
    ///Checks if the value of the field is `EMPTY`
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        **self == TXIS_A::EMPTY
    }
}
impl core::ops::Deref for TXIS_R {
    type Target = crate::FieldReader<bool, TXIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TXIS` writer - Transmit interrupt status (transmitters)
pub struct TXIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///The TXDR register is not empty
    #[inline(always)]
    pub fn not_empty(self) -> &'a mut W {
        self.variant(TXIS_A::NOTEMPTY)
    }
    ///The TXDR register is empty and the data to be transmitted must be written in the TXDR register
    #[inline(always)]
    pub fn empty(self) -> &'a mut W {
        self.variant(TXIS_A::EMPTY)
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
///Transmit data register empty (transmitters)
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXE_A {
    ///0: TXDR register not empty
    NOTEMPTY = 0,
    ///1: TXDR register empty
    EMPTY = 1,
}
impl From<TXE_A> for bool {
    #[inline(always)]
    fn from(variant: TXE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TXE` reader - Transmit data register empty (transmitters)
pub struct TXE_R(crate::FieldReader<bool, TXE_A>);
impl TXE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXE_A {
        match self.bits {
            false => TXE_A::NOTEMPTY,
            true => TXE_A::EMPTY,
        }
    }
    ///Checks if the value of the field is `NOTEMPTY`
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        **self == TXE_A::NOTEMPTY
    }
    ///Checks if the value of the field is `EMPTY`
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        **self == TXE_A::EMPTY
    }
}
impl core::ops::Deref for TXE_R {
    type Target = crate::FieldReader<bool, TXE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TXE` writer - Transmit data register empty (transmitters)
pub struct TXE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///TXDR register not empty
    #[inline(always)]
    pub fn not_empty(self) -> &'a mut W {
        self.variant(TXE_A::NOTEMPTY)
    }
    ///TXDR register empty
    #[inline(always)]
    pub fn empty(self) -> &'a mut W {
        self.variant(TXE_A::EMPTY)
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
    ///Bits 17:23 - Address match code (Slave mode)
    #[inline(always)]
    pub fn addcode(&self) -> ADDCODE_R {
        ADDCODE_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    ///Bit 16 - Transfer direction (Slave mode)
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 15 - Bus busy
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 13 - SMBus alert
    #[inline(always)]
    pub fn alert(&self) -> ALERT_R {
        ALERT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 12 - Timeout or t_low detection flag
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 11 - PEC Error in reception
    #[inline(always)]
    pub fn pecerr(&self) -> PECERR_R {
        PECERR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - Overrun/Underrun (slave mode)
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 9 - Arbitration lost
    #[inline(always)]
    pub fn arlo(&self) -> ARLO_R {
        ARLO_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - Bus error
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 7 - Transfer Complete Reload
    #[inline(always)]
    pub fn tcr(&self) -> TCR_R {
        TCR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 6 - Transfer Complete (master mode)
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - Stop detection flag
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - Not acknowledge received flag
    #[inline(always)]
    pub fn nackf(&self) -> NACKF_R {
        NACKF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - Address matched (slave mode)
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Receive data register not empty (receivers)
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Transmit interrupt status (transmitters)
    #[inline(always)]
    pub fn txis(&self) -> TXIS_R {
        TXIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Transmit data register empty (transmitters)
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 1 - Transmit interrupt status (transmitters)
    #[inline(always)]
    pub fn txis(&mut self) -> TXIS_W {
        TXIS_W { w: self }
    }
    ///Bit 0 - Transmit data register empty (transmitters)
    #[inline(always)]
    pub fn txe(&mut self) -> TXE_W {
        TXE_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Interrupt and Status register
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
///`reset()` method sets ISR to value 0x01
impl crate::Resettable for ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
