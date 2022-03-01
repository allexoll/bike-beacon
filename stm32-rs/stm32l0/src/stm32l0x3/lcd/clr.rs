///Register `CLR` writer
pub struct W(crate::W<CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLR_SPEC>;
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
impl From<crate::W<CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `UDDC` writer - Update display done clear
pub struct UDDC_W<'a> {
    w: &'a mut W,
}
impl<'a> UDDC_W<'a> {
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
///Field `SOFC` writer - Start of frame flag clear
pub struct SOFC_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFC_W<'a> {
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
impl W {
    ///Bit 3 - Update display done clear
    #[inline(always)]
    pub fn uddc(&mut self) -> UDDC_W {
        UDDC_W { w: self }
    }
    ///Bit 1 - Start of frame flag clear
    #[inline(always)]
    pub fn sofc(&mut self) -> SOFC_W {
        SOFC_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///clear register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [clr](index.html) module
pub struct CLR_SPEC;
impl crate::RegisterSpec for CLR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [clr::W](W) writer structure
impl crate::Writable for CLR_SPEC {
    type Writer = W;
}
///`reset()` method sets CLR to value 0
impl crate::Resettable for CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
