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
///Field `FECAP` reader - Frequency error capture
pub struct FECAP_R(crate::FieldReader<u16, u16>);
impl FECAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        FECAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FECAP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `FEDIR` reader - Frequency error direction
pub struct FEDIR_R(crate::FieldReader<bool, bool>);
impl FEDIR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FEDIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FEDIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TRIMOVF` reader - Trimming overflow or underflow
pub struct TRIMOVF_R(crate::FieldReader<bool, bool>);
impl TRIMOVF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIMOVF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIMOVF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SYNCMISS` reader - SYNC missed
pub struct SYNCMISS_R(crate::FieldReader<bool, bool>);
impl SYNCMISS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYNCMISS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNCMISS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SYNCERR` reader - SYNC error
pub struct SYNCERR_R(crate::FieldReader<bool, bool>);
impl SYNCERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYNCERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNCERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ESYNCF` reader - Expected SYNC flag
pub struct ESYNCF_R(crate::FieldReader<bool, bool>);
impl ESYNCF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ESYNCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ESYNCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ERRF` reader - Error flag
pub struct ERRF_R(crate::FieldReader<bool, bool>);
impl ERRF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERRF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERRF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SYNCWARNF` reader - SYNC warning flag
pub struct SYNCWARNF_R(crate::FieldReader<bool, bool>);
impl SYNCWARNF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYNCWARNF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNCWARNF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SYNCOKF` reader - SYNC event OK flag
pub struct SYNCOKF_R(crate::FieldReader<bool, bool>);
impl SYNCOKF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYNCOKF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNCOKF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bits 16:31 - Frequency error capture
    #[inline(always)]
    pub fn fecap(&self) -> FECAP_R {
        FECAP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    ///Bit 15 - Frequency error direction
    #[inline(always)]
    pub fn fedir(&self) -> FEDIR_R {
        FEDIR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 10 - Trimming overflow or underflow
    #[inline(always)]
    pub fn trimovf(&self) -> TRIMOVF_R {
        TRIMOVF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 9 - SYNC missed
    #[inline(always)]
    pub fn syncmiss(&self) -> SYNCMISS_R {
        SYNCMISS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - SYNC error
    #[inline(always)]
    pub fn syncerr(&self) -> SYNCERR_R {
        SYNCERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 3 - Expected SYNC flag
    #[inline(always)]
    pub fn esyncf(&self) -> ESYNCF_R {
        ESYNCF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Error flag
    #[inline(always)]
    pub fn errf(&self) -> ERRF_R {
        ERRF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - SYNC warning flag
    #[inline(always)]
    pub fn syncwarnf(&self) -> SYNCWARNF_R {
        SYNCWARNF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - SYNC event OK flag
    #[inline(always)]
    pub fn syncokf(&self) -> SYNCOKF_R {
        SYNCOKF_R::new((self.bits & 0x01) != 0)
    }
}
///interrupt and status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
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
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
