///Register `CR1` reader
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR1` writer
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
///Auto-reload preload enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARPE_A {
    ///0: TIMx_APRR register is not buffered
    DISABLED = 0,
    ///1: TIMx_APRR register is buffered
    ENABLED = 1,
}
impl From<ARPE_A> for bool {
    #[inline(always)]
    fn from(variant: ARPE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ARPE` reader - Auto-reload preload enable
pub struct ARPE_R(crate::FieldReader<bool, ARPE_A>);
impl ARPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ARPE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ARPE_A {
        match self.bits {
            false => ARPE_A::DISABLED,
            true => ARPE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ARPE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ARPE_A::ENABLED
    }
}
impl core::ops::Deref for ARPE_R {
    type Target = crate::FieldReader<bool, ARPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ARPE` writer - Auto-reload preload enable
pub struct ARPE_W<'a> {
    w: &'a mut W,
}
impl<'a> ARPE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ARPE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///TIMx_APRR register is not buffered
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ARPE_A::DISABLED)
    }
    ///TIMx_APRR register is buffered
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ARPE_A::ENABLED)
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
///One-pulse mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPM_A {
    ///0: Counter is not stopped at update event
    DISABLED = 0,
    ///1: Counter stops counting at the next update event (clearing the CEN bit)
    ENABLED = 1,
}
impl From<OPM_A> for bool {
    #[inline(always)]
    fn from(variant: OPM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OPM` reader - One-pulse mode
pub struct OPM_R(crate::FieldReader<bool, OPM_A>);
impl OPM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OPM_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OPM_A {
        match self.bits {
            false => OPM_A::DISABLED,
            true => OPM_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == OPM_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == OPM_A::ENABLED
    }
}
impl core::ops::Deref for OPM_R {
    type Target = crate::FieldReader<bool, OPM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `OPM` writer - One-pulse mode
pub struct OPM_W<'a> {
    w: &'a mut W,
}
impl<'a> OPM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OPM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Counter is not stopped at update event
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OPM_A::DISABLED)
    }
    ///Counter stops counting at the next update event (clearing the CEN bit)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OPM_A::ENABLED)
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
///Update request source
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum URS_A {
    ///0: Any of counter overflow/underflow, setting UG, or update through slave mode, generates an update interrupt or DMA request
    ANYEVENT = 0,
    ///1: Only counter overflow/underflow generates an update interrupt or DMA request
    COUNTERONLY = 1,
}
impl From<URS_A> for bool {
    #[inline(always)]
    fn from(variant: URS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `URS` reader - Update request source
pub struct URS_R(crate::FieldReader<bool, URS_A>);
impl URS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        URS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> URS_A {
        match self.bits {
            false => URS_A::ANYEVENT,
            true => URS_A::COUNTERONLY,
        }
    }
    ///Checks if the value of the field is `ANYEVENT`
    #[inline(always)]
    pub fn is_any_event(&self) -> bool {
        **self == URS_A::ANYEVENT
    }
    ///Checks if the value of the field is `COUNTERONLY`
    #[inline(always)]
    pub fn is_counter_only(&self) -> bool {
        **self == URS_A::COUNTERONLY
    }
}
impl core::ops::Deref for URS_R {
    type Target = crate::FieldReader<bool, URS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `URS` writer - Update request source
pub struct URS_W<'a> {
    w: &'a mut W,
}
impl<'a> URS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: URS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Any of counter overflow/underflow, setting UG, or update through slave mode, generates an update interrupt or DMA request
    #[inline(always)]
    pub fn any_event(self) -> &'a mut W {
        self.variant(URS_A::ANYEVENT)
    }
    ///Only counter overflow/underflow generates an update interrupt or DMA request
    #[inline(always)]
    pub fn counter_only(self) -> &'a mut W {
        self.variant(URS_A::COUNTERONLY)
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
///Update disable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UDIS_A {
    ///0: Update event enabled
    ENABLED = 0,
    ///1: Update event disabled
    DISABLED = 1,
}
impl From<UDIS_A> for bool {
    #[inline(always)]
    fn from(variant: UDIS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `UDIS` reader - Update disable
pub struct UDIS_R(crate::FieldReader<bool, UDIS_A>);
impl UDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UDIS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> UDIS_A {
        match self.bits {
            false => UDIS_A::ENABLED,
            true => UDIS_A::DISABLED,
        }
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == UDIS_A::ENABLED
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == UDIS_A::DISABLED
    }
}
impl core::ops::Deref for UDIS_R {
    type Target = crate::FieldReader<bool, UDIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `UDIS` writer - Update disable
pub struct UDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> UDIS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: UDIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Update event enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UDIS_A::ENABLED)
    }
    ///Update event disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UDIS_A::DISABLED)
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
///Counter enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEN_A {
    ///0: Counter disabled
    DISABLED = 0,
    ///1: Counter enabled
    ENABLED = 1,
}
impl From<CEN_A> for bool {
    #[inline(always)]
    fn from(variant: CEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CEN` reader - Counter enable
pub struct CEN_R(crate::FieldReader<bool, CEN_A>);
impl CEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CEN_A {
        match self.bits {
            false => CEN_A::DISABLED,
            true => CEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CEN_A::ENABLED
    }
}
impl core::ops::Deref for CEN_R {
    type Target = crate::FieldReader<bool, CEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CEN` writer - Counter enable
pub struct CEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Counter disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CEN_A::DISABLED)
    }
    ///Counter enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CEN_A::ENABLED)
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
    ///Bit 7 - Auto-reload preload enable
    #[inline(always)]
    pub fn arpe(&self) -> ARPE_R {
        ARPE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 3 - One-pulse mode
    #[inline(always)]
    pub fn opm(&self) -> OPM_R {
        OPM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Update request source
    #[inline(always)]
    pub fn urs(&self) -> URS_R {
        URS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Update disable
    #[inline(always)]
    pub fn udis(&self) -> UDIS_R {
        UDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Counter enable
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 7 - Auto-reload preload enable
    #[inline(always)]
    pub fn arpe(&mut self) -> ARPE_W {
        ARPE_W { w: self }
    }
    ///Bit 3 - One-pulse mode
    #[inline(always)]
    pub fn opm(&mut self) -> OPM_W {
        OPM_W { w: self }
    }
    ///Bit 2 - Update request source
    #[inline(always)]
    pub fn urs(&mut self) -> URS_W {
        URS_W { w: self }
    }
    ///Bit 1 - Update disable
    #[inline(always)]
    pub fn udis(&mut self) -> UDIS_W {
        UDIS_W { w: self }
    }
    ///Bit 0 - Counter enable
    #[inline(always)]
    pub fn cen(&mut self) -> CEN_W {
        CEN_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///control register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr1](index.html) module
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr1::R](R) reader structure
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr1::W](W) writer structure
impl crate::Writable for CR1_SPEC {
    type Writer = W;
}
///`reset()` method sets CR1 to value 0
impl crate::Resettable for CR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}