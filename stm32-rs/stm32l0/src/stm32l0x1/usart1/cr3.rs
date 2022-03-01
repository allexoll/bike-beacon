///Register `CR3` reader
pub struct R(crate::R<CR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR3` writer
pub struct W(crate::W<CR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR3_SPEC>;
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
impl From<crate::W<CR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR3_SPEC>) -> Self {
        W(writer)
    }
}
///Wakeup from Stop mode interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUFIE_A {
    ///0: Interrupt is inhibited
    DISABLED = 0,
    ///1: An USART interrupt is generated whenever WUF=1 in the ISR register
    ENABLED = 1,
}
impl From<WUFIE_A> for bool {
    #[inline(always)]
    fn from(variant: WUFIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WUFIE` reader - Wakeup from Stop mode interrupt enable
pub struct WUFIE_R(crate::FieldReader<bool, WUFIE_A>);
impl WUFIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WUFIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WUFIE_A {
        match self.bits {
            false => WUFIE_A::DISABLED,
            true => WUFIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WUFIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WUFIE_A::ENABLED
    }
}
impl core::ops::Deref for WUFIE_R {
    type Target = crate::FieldReader<bool, WUFIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WUFIE` writer - Wakeup from Stop mode interrupt enable
pub struct WUFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WUFIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WUFIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt is inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WUFIE_A::DISABLED)
    }
    ///An USART interrupt is generated whenever WUF=1 in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WUFIE_A::ENABLED)
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
///Wakeup from Stop mode interrupt flag selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WUS_A {
    ///0: WUF active on address match
    ADDRESS = 0,
    ///2: WuF active on Start bit detection
    START = 2,
    ///3: WUF active on RXNE
    RXNE = 3,
}
impl From<WUS_A> for u8 {
    #[inline(always)]
    fn from(variant: WUS_A) -> Self {
        variant as _
    }
}
///Field `WUS` reader - Wakeup from Stop mode interrupt flag selection
pub struct WUS_R(crate::FieldReader<u8, WUS_A>);
impl WUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WUS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<WUS_A> {
        match self.bits {
            0 => Some(WUS_A::ADDRESS),
            2 => Some(WUS_A::START),
            3 => Some(WUS_A::RXNE),
            _ => None,
        }
    }
    ///Checks if the value of the field is `ADDRESS`
    #[inline(always)]
    pub fn is_address(&self) -> bool {
        **self == WUS_A::ADDRESS
    }
    ///Checks if the value of the field is `START`
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        **self == WUS_A::START
    }
    ///Checks if the value of the field is `RXNE`
    #[inline(always)]
    pub fn is_rxne(&self) -> bool {
        **self == WUS_A::RXNE
    }
}
impl core::ops::Deref for WUS_R {
    type Target = crate::FieldReader<u8, WUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WUS` writer - Wakeup from Stop mode interrupt flag selection
pub struct WUS_W<'a> {
    w: &'a mut W,
}
impl<'a> WUS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WUS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///WUF active on address match
    #[inline(always)]
    pub fn address(self) -> &'a mut W {
        self.variant(WUS_A::ADDRESS)
    }
    ///WuF active on Start bit detection
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(WUS_A::START)
    }
    ///WUF active on RXNE
    #[inline(always)]
    pub fn rxne(self) -> &'a mut W {
        self.variant(WUS_A::RXNE)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
///Field `SCARCNT` reader - Smartcard auto-retry count
pub struct SCARCNT_R(crate::FieldReader<u8, u8>);
impl SCARCNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCARCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCARCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SCARCNT` writer - Smartcard auto-retry count
pub struct SCARCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SCARCNT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | ((value as u32 & 0x07) << 17);
        self.w
    }
}
///Driver enable polarity selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEP_A {
    ///0: DE signal is active high
    HIGH = 0,
    ///1: DE signal is active low
    LOW = 1,
}
impl From<DEP_A> for bool {
    #[inline(always)]
    fn from(variant: DEP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DEP` reader - Driver enable polarity selection
pub struct DEP_R(crate::FieldReader<bool, DEP_A>);
impl DEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DEP_A {
        match self.bits {
            false => DEP_A::HIGH,
            true => DEP_A::LOW,
        }
    }
    ///Checks if the value of the field is `HIGH`
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == DEP_A::HIGH
    }
    ///Checks if the value of the field is `LOW`
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == DEP_A::LOW
    }
}
impl core::ops::Deref for DEP_R {
    type Target = crate::FieldReader<bool, DEP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DEP` writer - Driver enable polarity selection
pub struct DEP_W<'a> {
    w: &'a mut W,
}
impl<'a> DEP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DEP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///DE signal is active high
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(DEP_A::HIGH)
    }
    ///DE signal is active low
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(DEP_A::LOW)
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
///Driver enable mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEM_A {
    ///0: DE function is disabled
    DISABLED = 0,
    ///1: The DE signal is output on the RTS pin
    ENABLED = 1,
}
impl From<DEM_A> for bool {
    #[inline(always)]
    fn from(variant: DEM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DEM` reader - Driver enable mode
pub struct DEM_R(crate::FieldReader<bool, DEM_A>);
impl DEM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEM_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DEM_A {
        match self.bits {
            false => DEM_A::DISABLED,
            true => DEM_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DEM_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DEM_A::ENABLED
    }
}
impl core::ops::Deref for DEM_R {
    type Target = crate::FieldReader<bool, DEM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DEM` writer - Driver enable mode
pub struct DEM_W<'a> {
    w: &'a mut W,
}
impl<'a> DEM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DEM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///DE function is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DEM_A::DISABLED)
    }
    ///The DE signal is output on the RTS pin
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DEM_A::ENABLED)
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
///DMA Disable on Reception Error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDRE_A {
    ///0: DMA is not disabled in case of reception error
    NOTDISABLED = 0,
    ///1: DMA is disabled following a reception error
    DISABLED = 1,
}
impl From<DDRE_A> for bool {
    #[inline(always)]
    fn from(variant: DDRE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DDRE` reader - DMA Disable on Reception Error
pub struct DDRE_R(crate::FieldReader<bool, DDRE_A>);
impl DDRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DDRE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DDRE_A {
        match self.bits {
            false => DDRE_A::NOTDISABLED,
            true => DDRE_A::DISABLED,
        }
    }
    ///Checks if the value of the field is `NOTDISABLED`
    #[inline(always)]
    pub fn is_not_disabled(&self) -> bool {
        **self == DDRE_A::NOTDISABLED
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DDRE_A::DISABLED
    }
}
impl core::ops::Deref for DDRE_R {
    type Target = crate::FieldReader<bool, DDRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DDRE` writer - DMA Disable on Reception Error
pub struct DDRE_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DDRE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///DMA is not disabled in case of reception error
    #[inline(always)]
    pub fn not_disabled(self) -> &'a mut W {
        self.variant(DDRE_A::NOTDISABLED)
    }
    ///DMA is disabled following a reception error
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DDRE_A::DISABLED)
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
///Overrun Disable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRDIS_A {
    ///0: Overrun Error Flag, ORE, is set when received data is not read before receiving new data
    ENABLED = 0,
    ///1: Overrun functionality is disabled. If new data is received while the RXNE flag is still set the ORE flag is not set and the new received data overwrites the previous content of the RDR register
    DISABLED = 1,
}
impl From<OVRDIS_A> for bool {
    #[inline(always)]
    fn from(variant: OVRDIS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OVRDIS` reader - Overrun Disable
pub struct OVRDIS_R(crate::FieldReader<bool, OVRDIS_A>);
impl OVRDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVRDIS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OVRDIS_A {
        match self.bits {
            false => OVRDIS_A::ENABLED,
            true => OVRDIS_A::DISABLED,
        }
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == OVRDIS_A::ENABLED
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == OVRDIS_A::DISABLED
    }
}
impl core::ops::Deref for OVRDIS_R {
    type Target = crate::FieldReader<bool, OVRDIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `OVRDIS` writer - Overrun Disable
pub struct OVRDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRDIS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OVRDIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Overrun Error Flag, ORE, is set when received data is not read before receiving new data
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OVRDIS_A::ENABLED)
    }
    ///Overrun functionality is disabled. If new data is received while the RXNE flag is still set the ORE flag is not set and the new received data overwrites the previous content of the RDR register
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OVRDIS_A::DISABLED)
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
///One sample bit method enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONEBIT_A {
    ///0: Three sample bit method
    SAMPLE3 = 0,
    ///1: One sample bit method
    SAMPLE1 = 1,
}
impl From<ONEBIT_A> for bool {
    #[inline(always)]
    fn from(variant: ONEBIT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ONEBIT` reader - One sample bit method enable
pub struct ONEBIT_R(crate::FieldReader<bool, ONEBIT_A>);
impl ONEBIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ONEBIT_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ONEBIT_A {
        match self.bits {
            false => ONEBIT_A::SAMPLE3,
            true => ONEBIT_A::SAMPLE1,
        }
    }
    ///Checks if the value of the field is `SAMPLE3`
    #[inline(always)]
    pub fn is_sample3(&self) -> bool {
        **self == ONEBIT_A::SAMPLE3
    }
    ///Checks if the value of the field is `SAMPLE1`
    #[inline(always)]
    pub fn is_sample1(&self) -> bool {
        **self == ONEBIT_A::SAMPLE1
    }
}
impl core::ops::Deref for ONEBIT_R {
    type Target = crate::FieldReader<bool, ONEBIT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ONEBIT` writer - One sample bit method enable
pub struct ONEBIT_W<'a> {
    w: &'a mut W,
}
impl<'a> ONEBIT_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ONEBIT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Three sample bit method
    #[inline(always)]
    pub fn sample3(self) -> &'a mut W {
        self.variant(ONEBIT_A::SAMPLE3)
    }
    ///One sample bit method
    #[inline(always)]
    pub fn sample1(self) -> &'a mut W {
        self.variant(ONEBIT_A::SAMPLE1)
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
///CTS interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSIE_A {
    ///0: Interrupt is inhibited
    DISABLED = 0,
    ///1: An interrupt is generated whenever CTSIF=1 in the ISR register
    ENABLED = 1,
}
impl From<CTSIE_A> for bool {
    #[inline(always)]
    fn from(variant: CTSIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSIE` reader - CTS interrupt enable
pub struct CTSIE_R(crate::FieldReader<bool, CTSIE_A>);
impl CTSIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTSIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CTSIE_A {
        match self.bits {
            false => CTSIE_A::DISABLED,
            true => CTSIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CTSIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CTSIE_A::ENABLED
    }
}
impl core::ops::Deref for CTSIE_R {
    type Target = crate::FieldReader<bool, CTSIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CTSIE` writer - CTS interrupt enable
pub struct CTSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CTSIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt is inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTSIE_A::DISABLED)
    }
    ///An interrupt is generated whenever CTSIF=1 in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTSIE_A::ENABLED)
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
///CTS enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSE_A {
    ///0: CTS hardware flow control disabled
    DISABLED = 0,
    ///1: CTS mode enabled, data is only transmitted when the CTS input is asserted
    ENABLED = 1,
}
impl From<CTSE_A> for bool {
    #[inline(always)]
    fn from(variant: CTSE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSE` reader - CTS enable
pub struct CTSE_R(crate::FieldReader<bool, CTSE_A>);
impl CTSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTSE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CTSE_A {
        match self.bits {
            false => CTSE_A::DISABLED,
            true => CTSE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CTSE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CTSE_A::ENABLED
    }
}
impl core::ops::Deref for CTSE_R {
    type Target = crate::FieldReader<bool, CTSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CTSE` writer - CTS enable
pub struct CTSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CTSE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///CTS hardware flow control disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTSE_A::DISABLED)
    }
    ///CTS mode enabled, data is only transmitted when the CTS input is asserted
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTSE_A::ENABLED)
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
///RTS enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTSE_A {
    ///0: RTS hardware flow control disabled
    DISABLED = 0,
    ///1: RTS output enabled, data is only requested when there is space in the receive buffer
    ENABLED = 1,
}
impl From<RTSE_A> for bool {
    #[inline(always)]
    fn from(variant: RTSE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RTSE` reader - RTS enable
pub struct RTSE_R(crate::FieldReader<bool, RTSE_A>);
impl RTSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTSE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RTSE_A {
        match self.bits {
            false => RTSE_A::DISABLED,
            true => RTSE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RTSE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RTSE_A::ENABLED
    }
}
impl core::ops::Deref for RTSE_R {
    type Target = crate::FieldReader<bool, RTSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RTSE` writer - RTS enable
pub struct RTSE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTSE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RTSE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///RTS hardware flow control disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTSE_A::DISABLED)
    }
    ///RTS output enabled, data is only requested when there is space in the receive buffer
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RTSE_A::ENABLED)
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
///DMA enable transmitter
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAT_A {
    ///0: DMA mode is disabled for transmission
    DISABLED = 0,
    ///1: DMA mode is enabled for transmission
    ENABLED = 1,
}
impl From<DMAT_A> for bool {
    #[inline(always)]
    fn from(variant: DMAT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAT` reader - DMA enable transmitter
pub struct DMAT_R(crate::FieldReader<bool, DMAT_A>);
impl DMAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMAT_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMAT_A {
        match self.bits {
            false => DMAT_A::DISABLED,
            true => DMAT_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DMAT_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DMAT_A::ENABLED
    }
}
impl core::ops::Deref for DMAT_R {
    type Target = crate::FieldReader<bool, DMAT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DMAT` writer - DMA enable transmitter
pub struct DMAT_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAT_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DMAT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///DMA mode is disabled for transmission
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAT_A::DISABLED)
    }
    ///DMA mode is enabled for transmission
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAT_A::ENABLED)
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
///DMA enable receiver
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAR_A {
    ///0: DMA mode is disabled for reception
    DISABLED = 0,
    ///1: DMA mode is enabled for reception
    ENABLED = 1,
}
impl From<DMAR_A> for bool {
    #[inline(always)]
    fn from(variant: DMAR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAR` reader - DMA enable receiver
pub struct DMAR_R(crate::FieldReader<bool, DMAR_A>);
impl DMAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMAR_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMAR_A {
        match self.bits {
            false => DMAR_A::DISABLED,
            true => DMAR_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DMAR_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DMAR_A::ENABLED
    }
}
impl core::ops::Deref for DMAR_R {
    type Target = crate::FieldReader<bool, DMAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DMAR` writer - DMA enable receiver
pub struct DMAR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DMAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///DMA mode is disabled for reception
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAR_A::DISABLED)
    }
    ///DMA mode is enabled for reception
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAR_A::ENABLED)
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
///Smartcard mode enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCEN_A {
    ///0: Smartcard Mode disabled
    DISABLED = 0,
    ///1: Smartcard Mode enabled
    ENABLED = 1,
}
impl From<SCEN_A> for bool {
    #[inline(always)]
    fn from(variant: SCEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SCEN` reader - Smartcard mode enable
pub struct SCEN_R(crate::FieldReader<bool, SCEN_A>);
impl SCEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SCEN_A {
        match self.bits {
            false => SCEN_A::DISABLED,
            true => SCEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SCEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SCEN_A::ENABLED
    }
}
impl core::ops::Deref for SCEN_R {
    type Target = crate::FieldReader<bool, SCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SCEN` writer - Smartcard mode enable
pub struct SCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Smartcard Mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SCEN_A::DISABLED)
    }
    ///Smartcard Mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SCEN_A::ENABLED)
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
///Smartcard NACK enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACK_A {
    ///0: NACK transmission in case of parity error is disabled
    DISABLED = 0,
    ///1: NACK transmission during parity error is enabled
    ENABLED = 1,
}
impl From<NACK_A> for bool {
    #[inline(always)]
    fn from(variant: NACK_A) -> Self {
        variant as u8 != 0
    }
}
///Field `NACK` reader - Smartcard NACK enable
pub struct NACK_R(crate::FieldReader<bool, NACK_A>);
impl NACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NACK_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> NACK_A {
        match self.bits {
            false => NACK_A::DISABLED,
            true => NACK_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == NACK_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == NACK_A::ENABLED
    }
}
impl core::ops::Deref for NACK_R {
    type Target = crate::FieldReader<bool, NACK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `NACK` writer - Smartcard NACK enable
pub struct NACK_W<'a> {
    w: &'a mut W,
}
impl<'a> NACK_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: NACK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///NACK transmission in case of parity error is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NACK_A::DISABLED)
    }
    ///NACK transmission during parity error is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NACK_A::ENABLED)
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
///Half-duplex selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDSEL_A {
    ///0: Half duplex mode is not selected
    NOTSELECTED = 0,
    ///1: Half duplex mode is selected
    SELECTED = 1,
}
impl From<HDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: HDSEL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `HDSEL` reader - Half-duplex selection
pub struct HDSEL_R(crate::FieldReader<bool, HDSEL_A>);
impl HDSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HDSEL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HDSEL_A {
        match self.bits {
            false => HDSEL_A::NOTSELECTED,
            true => HDSEL_A::SELECTED,
        }
    }
    ///Checks if the value of the field is `NOTSELECTED`
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        **self == HDSEL_A::NOTSELECTED
    }
    ///Checks if the value of the field is `SELECTED`
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        **self == HDSEL_A::SELECTED
    }
}
impl core::ops::Deref for HDSEL_R {
    type Target = crate::FieldReader<bool, HDSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `HDSEL` writer - Half-duplex selection
pub struct HDSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HDSEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: HDSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Half duplex mode is not selected
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut W {
        self.variant(HDSEL_A::NOTSELECTED)
    }
    ///Half duplex mode is selected
    #[inline(always)]
    pub fn selected(self) -> &'a mut W {
        self.variant(HDSEL_A::SELECTED)
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
///Ir low-power
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRLP_A {
    ///0: Normal mode
    NORMAL = 0,
    ///1: Low-power mode
    LOWPOWER = 1,
}
impl From<IRLP_A> for bool {
    #[inline(always)]
    fn from(variant: IRLP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IRLP` reader - Ir low-power
pub struct IRLP_R(crate::FieldReader<bool, IRLP_A>);
impl IRLP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IRLP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IRLP_A {
        match self.bits {
            false => IRLP_A::NORMAL,
            true => IRLP_A::LOWPOWER,
        }
    }
    ///Checks if the value of the field is `NORMAL`
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == IRLP_A::NORMAL
    }
    ///Checks if the value of the field is `LOWPOWER`
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        **self == IRLP_A::LOWPOWER
    }
}
impl core::ops::Deref for IRLP_R {
    type Target = crate::FieldReader<bool, IRLP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `IRLP` writer - Ir low-power
pub struct IRLP_W<'a> {
    w: &'a mut W,
}
impl<'a> IRLP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IRLP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Normal mode
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(IRLP_A::NORMAL)
    }
    ///Low-power mode
    #[inline(always)]
    pub fn low_power(self) -> &'a mut W {
        self.variant(IRLP_A::LOWPOWER)
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
///Ir mode enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IREN_A {
    ///0: IrDA disabled
    DISABLED = 0,
    ///1: IrDA enabled
    ENABLED = 1,
}
impl From<IREN_A> for bool {
    #[inline(always)]
    fn from(variant: IREN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `IREN` reader - Ir mode enable
pub struct IREN_R(crate::FieldReader<bool, IREN_A>);
impl IREN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IREN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IREN_A {
        match self.bits {
            false => IREN_A::DISABLED,
            true => IREN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == IREN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == IREN_A::ENABLED
    }
}
impl core::ops::Deref for IREN_R {
    type Target = crate::FieldReader<bool, IREN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `IREN` writer - Ir mode enable
pub struct IREN_W<'a> {
    w: &'a mut W,
}
impl<'a> IREN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IREN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///IrDA disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IREN_A::DISABLED)
    }
    ///IrDA enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IREN_A::ENABLED)
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
///Error interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EIE_A {
    ///0: Interrupt is inhibited
    DISABLED = 0,
    ///1: An interrupt is generated when FE=1 or ORE=1 or NF=1 in the ISR register
    ENABLED = 1,
}
impl From<EIE_A> for bool {
    #[inline(always)]
    fn from(variant: EIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EIE` reader - Error interrupt enable
pub struct EIE_R(crate::FieldReader<bool, EIE_A>);
impl EIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EIE_A {
        match self.bits {
            false => EIE_A::DISABLED,
            true => EIE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == EIE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == EIE_A::ENABLED
    }
}
impl core::ops::Deref for EIE_R {
    type Target = crate::FieldReader<bool, EIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EIE` writer - Error interrupt enable
pub struct EIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt is inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EIE_A::DISABLED)
    }
    ///An interrupt is generated when FE=1 or ORE=1 or NF=1 in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EIE_A::ENABLED)
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
    ///Bit 22 - Wakeup from Stop mode interrupt enable
    #[inline(always)]
    pub fn wufie(&self) -> WUFIE_R {
        WUFIE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bits 20:21 - Wakeup from Stop mode interrupt flag selection
    #[inline(always)]
    pub fn wus(&self) -> WUS_R {
        WUS_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    ///Bits 17:19 - Smartcard auto-retry count
    #[inline(always)]
    pub fn scarcnt(&self) -> SCARCNT_R {
        SCARCNT_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    ///Bit 15 - Driver enable polarity selection
    #[inline(always)]
    pub fn dep(&self) -> DEP_R {
        DEP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 14 - Driver enable mode
    #[inline(always)]
    pub fn dem(&self) -> DEM_R {
        DEM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 13 - DMA Disable on Reception Error
    #[inline(always)]
    pub fn ddre(&self) -> DDRE_R {
        DDRE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 12 - Overrun Disable
    #[inline(always)]
    pub fn ovrdis(&self) -> OVRDIS_R {
        OVRDIS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 11 - One sample bit method enable
    #[inline(always)]
    pub fn onebit(&self) -> ONEBIT_R {
        ONEBIT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - CTS interrupt enable
    #[inline(always)]
    pub fn ctsie(&self) -> CTSIE_R {
        CTSIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 9 - CTS enable
    #[inline(always)]
    pub fn ctse(&self) -> CTSE_R {
        CTSE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - RTS enable
    #[inline(always)]
    pub fn rtse(&self) -> RTSE_R {
        RTSE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 7 - DMA enable transmitter
    #[inline(always)]
    pub fn dmat(&self) -> DMAT_R {
        DMAT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 6 - DMA enable receiver
    #[inline(always)]
    pub fn dmar(&self) -> DMAR_R {
        DMAR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - Smartcard mode enable
    #[inline(always)]
    pub fn scen(&self) -> SCEN_R {
        SCEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - Smartcard NACK enable
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - Half-duplex selection
    #[inline(always)]
    pub fn hdsel(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Ir low-power
    #[inline(always)]
    pub fn irlp(&self) -> IRLP_R {
        IRLP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Ir mode enable
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Error interrupt enable
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 22 - Wakeup from Stop mode interrupt enable
    #[inline(always)]
    pub fn wufie(&mut self) -> WUFIE_W {
        WUFIE_W { w: self }
    }
    ///Bits 20:21 - Wakeup from Stop mode interrupt flag selection
    #[inline(always)]
    pub fn wus(&mut self) -> WUS_W {
        WUS_W { w: self }
    }
    ///Bits 17:19 - Smartcard auto-retry count
    #[inline(always)]
    pub fn scarcnt(&mut self) -> SCARCNT_W {
        SCARCNT_W { w: self }
    }
    ///Bit 15 - Driver enable polarity selection
    #[inline(always)]
    pub fn dep(&mut self) -> DEP_W {
        DEP_W { w: self }
    }
    ///Bit 14 - Driver enable mode
    #[inline(always)]
    pub fn dem(&mut self) -> DEM_W {
        DEM_W { w: self }
    }
    ///Bit 13 - DMA Disable on Reception Error
    #[inline(always)]
    pub fn ddre(&mut self) -> DDRE_W {
        DDRE_W { w: self }
    }
    ///Bit 12 - Overrun Disable
    #[inline(always)]
    pub fn ovrdis(&mut self) -> OVRDIS_W {
        OVRDIS_W { w: self }
    }
    ///Bit 11 - One sample bit method enable
    #[inline(always)]
    pub fn onebit(&mut self) -> ONEBIT_W {
        ONEBIT_W { w: self }
    }
    ///Bit 10 - CTS interrupt enable
    #[inline(always)]
    pub fn ctsie(&mut self) -> CTSIE_W {
        CTSIE_W { w: self }
    }
    ///Bit 9 - CTS enable
    #[inline(always)]
    pub fn ctse(&mut self) -> CTSE_W {
        CTSE_W { w: self }
    }
    ///Bit 8 - RTS enable
    #[inline(always)]
    pub fn rtse(&mut self) -> RTSE_W {
        RTSE_W { w: self }
    }
    ///Bit 7 - DMA enable transmitter
    #[inline(always)]
    pub fn dmat(&mut self) -> DMAT_W {
        DMAT_W { w: self }
    }
    ///Bit 6 - DMA enable receiver
    #[inline(always)]
    pub fn dmar(&mut self) -> DMAR_W {
        DMAR_W { w: self }
    }
    ///Bit 5 - Smartcard mode enable
    #[inline(always)]
    pub fn scen(&mut self) -> SCEN_W {
        SCEN_W { w: self }
    }
    ///Bit 4 - Smartcard NACK enable
    #[inline(always)]
    pub fn nack(&mut self) -> NACK_W {
        NACK_W { w: self }
    }
    ///Bit 3 - Half-duplex selection
    #[inline(always)]
    pub fn hdsel(&mut self) -> HDSEL_W {
        HDSEL_W { w: self }
    }
    ///Bit 2 - Ir low-power
    #[inline(always)]
    pub fn irlp(&mut self) -> IRLP_W {
        IRLP_W { w: self }
    }
    ///Bit 1 - Ir mode enable
    #[inline(always)]
    pub fn iren(&mut self) -> IREN_W {
        IREN_W { w: self }
    }
    ///Bit 0 - Error interrupt enable
    #[inline(always)]
    pub fn eie(&mut self) -> EIE_W {
        EIE_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Control register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr3](index.html) module
pub struct CR3_SPEC;
impl crate::RegisterSpec for CR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr3::R](R) reader structure
impl crate::Readable for CR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr3::W](W) writer structure
impl crate::Writable for CR3_SPEC {
    type Writer = W;
}
///`reset()` method sets CR3 to value 0
impl crate::Resettable for CR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
