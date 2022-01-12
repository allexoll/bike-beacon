///Register `CR2` reader
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR2` writer
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
///Master mode selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MMS_A {
    ///0: Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO)
    RESET = 0,
    ///1: Enable - the Counter enable signal, CNT_EN, is used as trigger output (TRGO)
    ENABLE = 1,
    ///2: Update - The update event is selected as trigger output (TRGO)
    UPDATE = 2,
    ///3: Compare Pulse - The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred
    COMPAREPULSE = 3,
    ///4: OC1REF signal is used as trigger output (TRGO)
    OC1REF = 4,
    ///5: OC2REF signal is used as trigger output (TRGO)
    OC2REF = 5,
}
impl From<MMS_A> for u8 {
    #[inline(always)]
    fn from(variant: MMS_A) -> Self {
        variant as _
    }
}
///Field `MMS` reader - Master mode selection
pub struct MMS_R(crate::FieldReader<u8, MMS_A>);
impl MMS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MMS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<MMS_A> {
        match self.bits {
            0 => Some(MMS_A::RESET),
            1 => Some(MMS_A::ENABLE),
            2 => Some(MMS_A::UPDATE),
            3 => Some(MMS_A::COMPAREPULSE),
            4 => Some(MMS_A::OC1REF),
            5 => Some(MMS_A::OC2REF),
            _ => None,
        }
    }
    ///Checks if the value of the field is `RESET`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == MMS_A::RESET
    }
    ///Checks if the value of the field is `ENABLE`
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == MMS_A::ENABLE
    }
    ///Checks if the value of the field is `UPDATE`
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        **self == MMS_A::UPDATE
    }
    ///Checks if the value of the field is `COMPAREPULSE`
    #[inline(always)]
    pub fn is_compare_pulse(&self) -> bool {
        **self == MMS_A::COMPAREPULSE
    }
    ///Checks if the value of the field is `OC1REF`
    #[inline(always)]
    pub fn is_oc1ref(&self) -> bool {
        **self == MMS_A::OC1REF
    }
    ///Checks if the value of the field is `OC2REF`
    #[inline(always)]
    pub fn is_oc2ref(&self) -> bool {
        **self == MMS_A::OC2REF
    }
}
impl core::ops::Deref for MMS_R {
    type Target = crate::FieldReader<u8, MMS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MMS` writer - Master mode selection
pub struct MMS_W<'a> {
    w: &'a mut W,
}
impl<'a> MMS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MMS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO)
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(MMS_A::RESET)
    }
    ///Enable - the Counter enable signal, CNT_EN, is used as trigger output (TRGO)
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MMS_A::ENABLE)
    }
    ///Update - The update event is selected as trigger output (TRGO)
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(MMS_A::UPDATE)
    }
    ///Compare Pulse - The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred
    #[inline(always)]
    pub fn compare_pulse(self) -> &'a mut W {
        self.variant(MMS_A::COMPAREPULSE)
    }
    ///OC1REF signal is used as trigger output (TRGO)
    #[inline(always)]
    pub fn oc1ref(self) -> &'a mut W {
        self.variant(MMS_A::OC1REF)
    }
    ///OC2REF signal is used as trigger output (TRGO)
    #[inline(always)]
    pub fn oc2ref(self) -> &'a mut W {
        self.variant(MMS_A::OC2REF)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
impl R {
    ///Bits 4:6 - Master mode selection
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    ///Bits 4:6 - Master mode selection
    #[inline(always)]
    pub fn mms(&mut self) -> MMS_W {
        MMS_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///control register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr2](index.html) module
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr2::R](R) reader structure
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr2::W](W) writer structure
impl crate::Writable for CR2_SPEC {
    type Writer = W;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
