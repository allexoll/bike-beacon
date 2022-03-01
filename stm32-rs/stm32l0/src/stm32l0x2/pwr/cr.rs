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
///Field `LPDS` reader - Low-power deep sleep
pub struct LPDS_R(crate::FieldReader<bool, bool>);
impl LPDS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPDS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPDS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LPDS` writer - Low-power deep sleep
pub struct LPDS_W<'a> {
    w: &'a mut W,
}
impl<'a> LPDS_W<'a> {
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
///Power down deepsleep
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDDS_A {
    ///0: Enter Stop mode when the CPU enters deepsleep
    STOP_MODE = 0,
    ///1: Enter Standby mode when the CPU enters deepsleep
    STANDBY_MODE = 1,
}
impl From<PDDS_A> for bool {
    #[inline(always)]
    fn from(variant: PDDS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PDDS` reader - Power down deepsleep
pub struct PDDS_R(crate::FieldReader<bool, PDDS_A>);
impl PDDS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDDS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PDDS_A {
        match self.bits {
            false => PDDS_A::STOP_MODE,
            true => PDDS_A::STANDBY_MODE,
        }
    }
    ///Checks if the value of the field is `STOP_MODE`
    #[inline(always)]
    pub fn is_stop_mode(&self) -> bool {
        **self == PDDS_A::STOP_MODE
    }
    ///Checks if the value of the field is `STANDBY_MODE`
    #[inline(always)]
    pub fn is_standby_mode(&self) -> bool {
        **self == PDDS_A::STANDBY_MODE
    }
}
impl core::ops::Deref for PDDS_R {
    type Target = crate::FieldReader<bool, PDDS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PDDS` writer - Power down deepsleep
pub struct PDDS_W<'a> {
    w: &'a mut W,
}
impl<'a> PDDS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PDDS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Enter Stop mode when the CPU enters deepsleep
    #[inline(always)]
    pub fn stop_mode(self) -> &'a mut W {
        self.variant(PDDS_A::STOP_MODE)
    }
    ///Enter Standby mode when the CPU enters deepsleep
    #[inline(always)]
    pub fn standby_mode(self) -> &'a mut W {
        self.variant(PDDS_A::STANDBY_MODE)
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
///Clear wakeup flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CWUF_A {
    ///1: Clear the WUF Wakeup flag after 2 system clock cycles
    CLEAR = 1,
}
impl From<CWUF_A> for bool {
    #[inline(always)]
    fn from(variant: CWUF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CWUF` reader - Clear wakeup flag
pub struct CWUF_R(crate::FieldReader<bool, CWUF_A>);
impl CWUF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CWUF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CWUF_A> {
        match self.bits {
            true => Some(CWUF_A::CLEAR),
            _ => None,
        }
    }
    ///Checks if the value of the field is `CLEAR`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == CWUF_A::CLEAR
    }
}
impl core::ops::Deref for CWUF_R {
    type Target = crate::FieldReader<bool, CWUF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CWUF` writer - Clear wakeup flag
pub struct CWUF_W<'a> {
    w: &'a mut W,
}
impl<'a> CWUF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CWUF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear the WUF Wakeup flag after 2 system clock cycles
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CWUF_A::CLEAR)
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
///Clear standby flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSBF_A {
    ///1: Clear the SBF Standby flag
    CLEAR = 1,
}
impl From<CSBF_A> for bool {
    #[inline(always)]
    fn from(variant: CSBF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSBF` reader - Clear standby flag
pub struct CSBF_R(crate::FieldReader<bool, CSBF_A>);
impl CSBF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSBF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CSBF_A> {
        match self.bits {
            true => Some(CSBF_A::CLEAR),
            _ => None,
        }
    }
    ///Checks if the value of the field is `CLEAR`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == CSBF_A::CLEAR
    }
}
impl core::ops::Deref for CSBF_R {
    type Target = crate::FieldReader<bool, CSBF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CSBF` writer - Clear standby flag
pub struct CSBF_W<'a> {
    w: &'a mut W,
}
impl<'a> CSBF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CSBF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clear the SBF Standby flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CSBF_A::CLEAR)
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
///Power voltage detector enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PVDE_A {
    ///0: PVD Disabled
    DISABLED = 0,
    ///1: PVD Enabled
    ENABLED = 1,
}
impl From<PVDE_A> for bool {
    #[inline(always)]
    fn from(variant: PVDE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PVDE` reader - Power voltage detector enable
pub struct PVDE_R(crate::FieldReader<bool, PVDE_A>);
impl PVDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PVDE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PVDE_A {
        match self.bits {
            false => PVDE_A::DISABLED,
            true => PVDE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PVDE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PVDE_A::ENABLED
    }
}
impl core::ops::Deref for PVDE_R {
    type Target = crate::FieldReader<bool, PVDE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PVDE` writer - Power voltage detector enable
pub struct PVDE_W<'a> {
    w: &'a mut W,
}
impl<'a> PVDE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PVDE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///PVD Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PVDE_A::DISABLED)
    }
    ///PVD Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PVDE_A::ENABLED)
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
///PVD level selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLS_A {
    ///0: 1.9 V
    V1_9 = 0,
    ///1: 2.1 V
    V2_1 = 1,
    ///2: 2.3 V
    V2_3 = 2,
    ///3: 2.5 V
    V2_5 = 3,
    ///4: 2.7 V
    V2_7 = 4,
    ///5: 2.9 V
    V2_9 = 5,
    ///6: 3.1 V
    V3_1 = 6,
    ///7: External input analog voltage (Compare internally to VREFINT)
    EXTERNAL = 7,
}
impl From<PLS_A> for u8 {
    #[inline(always)]
    fn from(variant: PLS_A) -> Self {
        variant as _
    }
}
///Field `PLS` reader - PVD level selection
pub struct PLS_R(crate::FieldReader<u8, PLS_A>);
impl PLS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PLS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLS_A {
        match self.bits {
            0 => PLS_A::V1_9,
            1 => PLS_A::V2_1,
            2 => PLS_A::V2_3,
            3 => PLS_A::V2_5,
            4 => PLS_A::V2_7,
            5 => PLS_A::V2_9,
            6 => PLS_A::V3_1,
            7 => PLS_A::EXTERNAL,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `V1_9`
    #[inline(always)]
    pub fn is_v1_9(&self) -> bool {
        **self == PLS_A::V1_9
    }
    ///Checks if the value of the field is `V2_1`
    #[inline(always)]
    pub fn is_v2_1(&self) -> bool {
        **self == PLS_A::V2_1
    }
    ///Checks if the value of the field is `V2_3`
    #[inline(always)]
    pub fn is_v2_3(&self) -> bool {
        **self == PLS_A::V2_3
    }
    ///Checks if the value of the field is `V2_5`
    #[inline(always)]
    pub fn is_v2_5(&self) -> bool {
        **self == PLS_A::V2_5
    }
    ///Checks if the value of the field is `V2_7`
    #[inline(always)]
    pub fn is_v2_7(&self) -> bool {
        **self == PLS_A::V2_7
    }
    ///Checks if the value of the field is `V2_9`
    #[inline(always)]
    pub fn is_v2_9(&self) -> bool {
        **self == PLS_A::V2_9
    }
    ///Checks if the value of the field is `V3_1`
    #[inline(always)]
    pub fn is_v3_1(&self) -> bool {
        **self == PLS_A::V3_1
    }
    ///Checks if the value of the field is `EXTERNAL`
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        **self == PLS_A::EXTERNAL
    }
}
impl core::ops::Deref for PLS_R {
    type Target = crate::FieldReader<u8, PLS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PLS` writer - PVD level selection
pub struct PLS_W<'a> {
    w: &'a mut W,
}
impl<'a> PLS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PLS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///1.9 V
    #[inline(always)]
    pub fn v1_9(self) -> &'a mut W {
        self.variant(PLS_A::V1_9)
    }
    ///2.1 V
    #[inline(always)]
    pub fn v2_1(self) -> &'a mut W {
        self.variant(PLS_A::V2_1)
    }
    ///2.3 V
    #[inline(always)]
    pub fn v2_3(self) -> &'a mut W {
        self.variant(PLS_A::V2_3)
    }
    ///2.5 V
    #[inline(always)]
    pub fn v2_5(self) -> &'a mut W {
        self.variant(PLS_A::V2_5)
    }
    ///2.7 V
    #[inline(always)]
    pub fn v2_7(self) -> &'a mut W {
        self.variant(PLS_A::V2_7)
    }
    ///2.9 V
    #[inline(always)]
    pub fn v2_9(self) -> &'a mut W {
        self.variant(PLS_A::V2_9)
    }
    ///3.1 V
    #[inline(always)]
    pub fn v3_1(self) -> &'a mut W {
        self.variant(PLS_A::V3_1)
    }
    ///External input analog voltage (Compare internally to VREFINT)
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(PLS_A::EXTERNAL)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
///Disable backup domain write protection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBP_A {
    ///0: Access to RTC, RTC Backup and RCC CSR registers disabled
    DISABLED = 0,
    ///1: Access to RTC, RTC Backup and RCC CSR registers enabled
    ENABLED = 1,
}
impl From<DBP_A> for bool {
    #[inline(always)]
    fn from(variant: DBP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DBP` reader - Disable backup domain write protection
pub struct DBP_R(crate::FieldReader<bool, DBP_A>);
impl DBP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DBP_A {
        match self.bits {
            false => DBP_A::DISABLED,
            true => DBP_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DBP_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DBP_A::ENABLED
    }
}
impl core::ops::Deref for DBP_R {
    type Target = crate::FieldReader<bool, DBP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DBP` writer - Disable backup domain write protection
pub struct DBP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DBP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Access to RTC, RTC Backup and RCC CSR registers disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DBP_A::DISABLED)
    }
    ///Access to RTC, RTC Backup and RCC CSR registers enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DBP_A::ENABLED)
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
///Ultra-low-power mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ULP_A {
    ///0: VREFINT is on in low-power mode
    ENABLED = 0,
    ///1: VREFINT is off in low-power mode
    DISABLED = 1,
}
impl From<ULP_A> for bool {
    #[inline(always)]
    fn from(variant: ULP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ULP` reader - Ultra-low-power mode
pub struct ULP_R(crate::FieldReader<bool, ULP_A>);
impl ULP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ULP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ULP_A {
        match self.bits {
            false => ULP_A::ENABLED,
            true => ULP_A::DISABLED,
        }
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ULP_A::ENABLED
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ULP_A::DISABLED
    }
}
impl core::ops::Deref for ULP_R {
    type Target = crate::FieldReader<bool, ULP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ULP` writer - Ultra-low-power mode
pub struct ULP_W<'a> {
    w: &'a mut W,
}
impl<'a> ULP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ULP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///VREFINT is on in low-power mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ULP_A::ENABLED)
    }
    ///VREFINT is off in low-power mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ULP_A::DISABLED)
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
///Fast wakeup
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWU_A {
    ///0: Low-power modes exit occurs only when VREFINT is ready
    DISABLED = 0,
    ///1: VREFINT start up time is ignored when exiting low-power modes
    ENABLED = 1,
}
impl From<FWU_A> for bool {
    #[inline(always)]
    fn from(variant: FWU_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FWU` reader - Fast wakeup
pub struct FWU_R(crate::FieldReader<bool, FWU_A>);
impl FWU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FWU_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FWU_A {
        match self.bits {
            false => FWU_A::DISABLED,
            true => FWU_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FWU_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FWU_A::ENABLED
    }
}
impl core::ops::Deref for FWU_R {
    type Target = crate::FieldReader<bool, FWU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `FWU` writer - Fast wakeup
pub struct FWU_W<'a> {
    w: &'a mut W,
}
impl<'a> FWU_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FWU_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Low-power modes exit occurs only when VREFINT is ready
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FWU_A::DISABLED)
    }
    ///VREFINT start up time is ignored when exiting low-power modes
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FWU_A::ENABLED)
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
///Voltage scaling range selection
///
///Value on reset: 2
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VOS_A {
    ///1: 1.8 V (range 1)
    V1_8 = 1,
    ///2: 1.5 V (range 2)
    V1_5 = 2,
    ///3: 1.2 V (range 3)
    V1_2 = 3,
}
impl From<VOS_A> for u8 {
    #[inline(always)]
    fn from(variant: VOS_A) -> Self {
        variant as _
    }
}
///Field `VOS` reader - Voltage scaling range selection
pub struct VOS_R(crate::FieldReader<u8, VOS_A>);
impl VOS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VOS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<VOS_A> {
        match self.bits {
            1 => Some(VOS_A::V1_8),
            2 => Some(VOS_A::V1_5),
            3 => Some(VOS_A::V1_2),
            _ => None,
        }
    }
    ///Checks if the value of the field is `V1_8`
    #[inline(always)]
    pub fn is_v1_8(&self) -> bool {
        **self == VOS_A::V1_8
    }
    ///Checks if the value of the field is `V1_5`
    #[inline(always)]
    pub fn is_v1_5(&self) -> bool {
        **self == VOS_A::V1_5
    }
    ///Checks if the value of the field is `V1_2`
    #[inline(always)]
    pub fn is_v1_2(&self) -> bool {
        **self == VOS_A::V1_2
    }
}
impl core::ops::Deref for VOS_R {
    type Target = crate::FieldReader<u8, VOS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `VOS` writer - Voltage scaling range selection
pub struct VOS_W<'a> {
    w: &'a mut W,
}
impl<'a> VOS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: VOS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///1.8 V (range 1)
    #[inline(always)]
    pub fn v1_8(self) -> &'a mut W {
        self.variant(VOS_A::V1_8)
    }
    ///1.5 V (range 2)
    #[inline(always)]
    pub fn v1_5(self) -> &'a mut W {
        self.variant(VOS_A::V1_5)
    }
    ///1.2 V (range 3)
    #[inline(always)]
    pub fn v1_2(self) -> &'a mut W {
        self.variant(VOS_A::V1_2)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | ((value as u32 & 0x03) << 11);
        self.w
    }
}
///Deep sleep mode with Flash memory kept off
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DS_EE_KOFF_A {
    ///0: NVM woken up when exiting from Deepsleep mode even if the bit RUN_PD is set
    NVMWAKEUP = 0,
    ///1: NVM not woken up when exiting from low-power mode (if the bit RUN_PD is set)
    NVMSLEEP = 1,
}
impl From<DS_EE_KOFF_A> for bool {
    #[inline(always)]
    fn from(variant: DS_EE_KOFF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DS_EE_KOFF` reader - Deep sleep mode with Flash memory kept off
pub struct DS_EE_KOFF_R(crate::FieldReader<bool, DS_EE_KOFF_A>);
impl DS_EE_KOFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DS_EE_KOFF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DS_EE_KOFF_A {
        match self.bits {
            false => DS_EE_KOFF_A::NVMWAKEUP,
            true => DS_EE_KOFF_A::NVMSLEEP,
        }
    }
    ///Checks if the value of the field is `NVMWAKEUP`
    #[inline(always)]
    pub fn is_nvmwake_up(&self) -> bool {
        **self == DS_EE_KOFF_A::NVMWAKEUP
    }
    ///Checks if the value of the field is `NVMSLEEP`
    #[inline(always)]
    pub fn is_nvmsleep(&self) -> bool {
        **self == DS_EE_KOFF_A::NVMSLEEP
    }
}
impl core::ops::Deref for DS_EE_KOFF_R {
    type Target = crate::FieldReader<bool, DS_EE_KOFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DS_EE_KOFF` writer - Deep sleep mode with Flash memory kept off
pub struct DS_EE_KOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> DS_EE_KOFF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DS_EE_KOFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///NVM woken up when exiting from Deepsleep mode even if the bit RUN_PD is set
    #[inline(always)]
    pub fn nvmwake_up(self) -> &'a mut W {
        self.variant(DS_EE_KOFF_A::NVMWAKEUP)
    }
    ///NVM not woken up when exiting from low-power mode (if the bit RUN_PD is set)
    #[inline(always)]
    pub fn nvmsleep(self) -> &'a mut W {
        self.variant(DS_EE_KOFF_A::NVMSLEEP)
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
///Low power run mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPRUN_A {
    ///0: Voltage regulator in Main mode in Low-power run mode
    MAIN_MODE = 0,
    ///1: Voltage regulator in low-power mode in Low-power run mode
    LOW_POWER_MODE = 1,
}
impl From<LPRUN_A> for bool {
    #[inline(always)]
    fn from(variant: LPRUN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LPRUN` reader - Low power run mode
pub struct LPRUN_R(crate::FieldReader<bool, LPRUN_A>);
impl LPRUN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPRUN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LPRUN_A {
        match self.bits {
            false => LPRUN_A::MAIN_MODE,
            true => LPRUN_A::LOW_POWER_MODE,
        }
    }
    ///Checks if the value of the field is `MAIN_MODE`
    #[inline(always)]
    pub fn is_main_mode(&self) -> bool {
        **self == LPRUN_A::MAIN_MODE
    }
    ///Checks if the value of the field is `LOW_POWER_MODE`
    #[inline(always)]
    pub fn is_low_power_mode(&self) -> bool {
        **self == LPRUN_A::LOW_POWER_MODE
    }
}
impl core::ops::Deref for LPRUN_R {
    type Target = crate::FieldReader<bool, LPRUN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LPRUN` writer - Low power run mode
pub struct LPRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPRUN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LPRUN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Voltage regulator in Main mode in Low-power run mode
    #[inline(always)]
    pub fn main_mode(self) -> &'a mut W {
        self.variant(LPRUN_A::MAIN_MODE)
    }
    ///Voltage regulator in low-power mode in Low-power run mode
    #[inline(always)]
    pub fn low_power_mode(self) -> &'a mut W {
        self.variant(LPRUN_A::LOW_POWER_MODE)
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
///Low-power deepsleep/Sleep/Low-power run
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPSDSR_A {
    ///0: Voltage regulator on during Deepsleep/Sleep/Low-power run mode
    MAIN_MODE = 0,
    ///1: Voltage regulator in low-power mode during Deepsleep/Sleep/Low-power run mode
    LOW_POWER_MODE = 1,
}
impl From<LPSDSR_A> for bool {
    #[inline(always)]
    fn from(variant: LPSDSR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LPSDSR` reader - Low-power deepsleep/Sleep/Low-power run
pub struct LPSDSR_R(crate::FieldReader<bool, LPSDSR_A>);
impl LPSDSR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPSDSR_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LPSDSR_A {
        match self.bits {
            false => LPSDSR_A::MAIN_MODE,
            true => LPSDSR_A::LOW_POWER_MODE,
        }
    }
    ///Checks if the value of the field is `MAIN_MODE`
    #[inline(always)]
    pub fn is_main_mode(&self) -> bool {
        **self == LPSDSR_A::MAIN_MODE
    }
    ///Checks if the value of the field is `LOW_POWER_MODE`
    #[inline(always)]
    pub fn is_low_power_mode(&self) -> bool {
        **self == LPSDSR_A::LOW_POWER_MODE
    }
}
impl core::ops::Deref for LPSDSR_R {
    type Target = crate::FieldReader<bool, LPSDSR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LPSDSR` writer - Low-power deepsleep/Sleep/Low-power run
pub struct LPSDSR_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSDSR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LPSDSR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Voltage regulator on during Deepsleep/Sleep/Low-power run mode
    #[inline(always)]
    pub fn main_mode(self) -> &'a mut W {
        self.variant(LPSDSR_A::MAIN_MODE)
    }
    ///Voltage regulator in low-power mode during Deepsleep/Sleep/Low-power run mode
    #[inline(always)]
    pub fn low_power_mode(self) -> &'a mut W {
        self.variant(LPSDSR_A::LOW_POWER_MODE)
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
    ///Bit 0 - Low-power deep sleep
    #[inline(always)]
    pub fn lpds(&self) -> LPDS_R {
        LPDS_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Power down deepsleep
    #[inline(always)]
    pub fn pdds(&self) -> PDDS_R {
        PDDS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Clear wakeup flag
    #[inline(always)]
    pub fn cwuf(&self) -> CWUF_R {
        CWUF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Clear standby flag
    #[inline(always)]
    pub fn csbf(&self) -> CSBF_R {
        CSBF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Power voltage detector enable
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bits 5:7 - PVD level selection
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    ///Bit 8 - Disable backup domain write protection
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - Ultra-low-power mode
    #[inline(always)]
    pub fn ulp(&self) -> ULP_R {
        ULP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - Fast wakeup
    #[inline(always)]
    pub fn fwu(&self) -> FWU_R {
        FWU_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bits 11:12 - Voltage scaling range selection
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    ///Bit 13 - Deep sleep mode with Flash memory kept off
    #[inline(always)]
    pub fn ds_ee_koff(&self) -> DS_EE_KOFF_R {
        DS_EE_KOFF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - Low power run mode
    #[inline(always)]
    pub fn lprun(&self) -> LPRUN_R {
        LPRUN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 0 - Low-power deepsleep/Sleep/Low-power run
    #[inline(always)]
    pub fn lpsdsr(&self) -> LPSDSR_R {
        LPSDSR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Low-power deep sleep
    #[inline(always)]
    pub fn lpds(&mut self) -> LPDS_W {
        LPDS_W { w: self }
    }
    ///Bit 1 - Power down deepsleep
    #[inline(always)]
    pub fn pdds(&mut self) -> PDDS_W {
        PDDS_W { w: self }
    }
    ///Bit 2 - Clear wakeup flag
    #[inline(always)]
    pub fn cwuf(&mut self) -> CWUF_W {
        CWUF_W { w: self }
    }
    ///Bit 3 - Clear standby flag
    #[inline(always)]
    pub fn csbf(&mut self) -> CSBF_W {
        CSBF_W { w: self }
    }
    ///Bit 4 - Power voltage detector enable
    #[inline(always)]
    pub fn pvde(&mut self) -> PVDE_W {
        PVDE_W { w: self }
    }
    ///Bits 5:7 - PVD level selection
    #[inline(always)]
    pub fn pls(&mut self) -> PLS_W {
        PLS_W { w: self }
    }
    ///Bit 8 - Disable backup domain write protection
    #[inline(always)]
    pub fn dbp(&mut self) -> DBP_W {
        DBP_W { w: self }
    }
    ///Bit 9 - Ultra-low-power mode
    #[inline(always)]
    pub fn ulp(&mut self) -> ULP_W {
        ULP_W { w: self }
    }
    ///Bit 10 - Fast wakeup
    #[inline(always)]
    pub fn fwu(&mut self) -> FWU_W {
        FWU_W { w: self }
    }
    ///Bits 11:12 - Voltage scaling range selection
    #[inline(always)]
    pub fn vos(&mut self) -> VOS_W {
        VOS_W { w: self }
    }
    ///Bit 13 - Deep sleep mode with Flash memory kept off
    #[inline(always)]
    pub fn ds_ee_koff(&mut self) -> DS_EE_KOFF_W {
        DS_EE_KOFF_W { w: self }
    }
    ///Bit 14 - Low power run mode
    #[inline(always)]
    pub fn lprun(&mut self) -> LPRUN_W {
        LPRUN_W { w: self }
    }
    ///Bit 0 - Low-power deepsleep/Sleep/Low-power run
    #[inline(always)]
    pub fn lpsdsr(&mut self) -> LPSDSR_W {
        LPSDSR_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///power control register
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
///`reset()` method sets CR to value 0x1000
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1000
    }
}
