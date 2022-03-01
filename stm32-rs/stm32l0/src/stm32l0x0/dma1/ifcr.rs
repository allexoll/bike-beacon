///Register `IFCR` writer
pub struct W(crate::W<IFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFCR_SPEC>;
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
impl From<crate::W<IFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFCR_SPEC>) -> Self {
        W(writer)
    }
}
///Channel x transfer error clear (x = 1 ..7)
pub type CTEIF7_AW = CTEIF1_AW;
///Field `CTEIF7` writer - Channel x transfer error clear (x = 1 ..7)
pub struct CTEIF7_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF7_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CTEIF7_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the TEIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF7_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
///Channel x half transfer clear (x = 1 ..7)
pub type CHTIF7_AW = CHTIF1_AW;
///Field `CHTIF7` writer - Channel x half transfer clear (x = 1 ..7)
pub struct CHTIF7_W<'a> {
    w: &'a mut W,
}
impl<'a> CHTIF7_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CHTIF7_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the HTIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF7_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
///Channel x transfer complete clear (x = 1 ..7)
pub type CTCIF7_AW = CTCIF1_AW;
///Field `CTCIF7` writer - Channel x transfer complete clear (x = 1 ..7)
pub struct CTCIF7_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIF7_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CTCIF7_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the TCIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF7_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
///Channel x global interrupt clear (x = 1 ..7)
pub type CGIF7_AW = CGIF1_AW;
///Field `CGIF7` writer - Channel x global interrupt clear (x = 1 ..7)
pub struct CGIF7_W<'a> {
    w: &'a mut W,
}
impl<'a> CGIF7_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CGIF7_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the GIF, TEIF, HTIF, TCIF flags in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CGIF7_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
///Channel x transfer error clear (x = 1 ..7)
pub type CTEIF6_AW = CTEIF1_AW;
///Field `CTEIF6` writer - Channel x transfer error clear (x = 1 ..7)
pub struct CTEIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF6_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CTEIF6_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the TEIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF6_AW::CLEAR)
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
///Channel x half transfer clear (x = 1 ..7)
pub type CHTIF6_AW = CHTIF1_AW;
///Field `CHTIF6` writer - Channel x half transfer clear (x = 1 ..7)
pub struct CHTIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> CHTIF6_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CHTIF6_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the HTIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF6_AW::CLEAR)
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
///Channel x transfer complete clear (x = 1 ..7)
pub type CTCIF6_AW = CTCIF1_AW;
///Field `CTCIF6` writer - Channel x transfer complete clear (x = 1 ..7)
pub struct CTCIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIF6_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CTCIF6_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the TCIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF6_AW::CLEAR)
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
///Channel x global interrupt clear (x = 1 ..7)
pub type CGIF6_AW = CGIF1_AW;
///Field `CGIF6` writer - Channel x global interrupt clear (x = 1 ..7)
pub struct CGIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> CGIF6_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CGIF6_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the GIF, TEIF, HTIF, TCIF flags in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CGIF6_AW::CLEAR)
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
///Channel x transfer error clear (x = 1 ..7)
pub type CTEIF5_AW = CTEIF1_AW;
///Field `CTEIF5` writer - Channel x transfer error clear (x = 1 ..7)
pub struct CTEIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF5_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CTEIF5_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the TEIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF5_AW::CLEAR)
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
///Channel x half transfer clear (x = 1 ..7)
pub type CHTIF5_AW = CHTIF1_AW;
///Field `CHTIF5` writer - Channel x half transfer clear (x = 1 ..7)
pub struct CHTIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> CHTIF5_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CHTIF5_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the HTIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF5_AW::CLEAR)
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
///Channel x transfer complete clear (x = 1 ..7)
pub type CTCIF5_AW = CTCIF1_AW;
///Field `CTCIF5` writer - Channel x transfer complete clear (x = 1 ..7)
pub struct CTCIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIF5_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CTCIF5_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the TCIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF5_AW::CLEAR)
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
///Channel x global interrupt clear (x = 1 ..7)
pub type CGIF5_AW = CGIF1_AW;
///Field `CGIF5` writer - Channel x global interrupt clear (x = 1 ..7)
pub struct CGIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> CGIF5_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CGIF5_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the GIF, TEIF, HTIF, TCIF flags in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CGIF5_AW::CLEAR)
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
///Channel x transfer error clear (x = 1 ..7)
pub type CTEIF4_AW = CTEIF1_AW;
///Field `CTEIF4` writer - Channel x transfer error clear (x = 1 ..7)
pub struct CTEIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF4_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CTEIF4_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the TEIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF4_AW::CLEAR)
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
///Channel x half transfer clear (x = 1 ..7)
pub type CHTIF4_AW = CHTIF1_AW;
///Field `CHTIF4` writer - Channel x half transfer clear (x = 1 ..7)
pub struct CHTIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CHTIF4_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CHTIF4_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the HTIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF4_AW::CLEAR)
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
///Channel x transfer complete clear (x = 1 ..7)
pub type CTCIF4_AW = CTCIF1_AW;
///Field `CTCIF4` writer - Channel x transfer complete clear (x = 1 ..7)
pub struct CTCIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIF4_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CTCIF4_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the TCIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF4_AW::CLEAR)
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
///Channel x global interrupt clear (x = 1 ..7)
pub type CGIF4_AW = CGIF1_AW;
///Field `CGIF4` writer - Channel x global interrupt clear (x = 1 ..7)
pub struct CGIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CGIF4_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CGIF4_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the GIF, TEIF, HTIF, TCIF flags in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CGIF4_AW::CLEAR)
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
///Channel x transfer error clear (x = 1 ..7)
pub type CTEIF3_AW = CTEIF1_AW;
///Field `CTEIF3` writer - Channel x transfer error clear (x = 1 ..7)
pub struct CTEIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF3_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CTEIF3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the TEIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF3_AW::CLEAR)
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
///Channel x half transfer clear (x = 1 ..7)
pub type CHTIF3_AW = CHTIF1_AW;
///Field `CHTIF3` writer - Channel x half transfer clear (x = 1 ..7)
pub struct CHTIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CHTIF3_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CHTIF3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the HTIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF3_AW::CLEAR)
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
///Channel x transfer complete clear (x = 1 ..7)
pub type CTCIF3_AW = CTCIF1_AW;
///Field `CTCIF3` writer - Channel x transfer complete clear (x = 1 ..7)
pub struct CTCIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIF3_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CTCIF3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the TCIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF3_AW::CLEAR)
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
///Channel x global interrupt clear (x = 1 ..7)
pub type CGIF3_AW = CGIF1_AW;
///Field `CGIF3` writer - Channel x global interrupt clear (x = 1 ..7)
pub struct CGIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CGIF3_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CGIF3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the GIF, TEIF, HTIF, TCIF flags in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CGIF3_AW::CLEAR)
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
///Channel x transfer error clear (x = 1 ..7)
pub type CTEIF2_AW = CTEIF1_AW;
///Field `CTEIF2` writer - Channel x transfer error clear (x = 1 ..7)
pub struct CTEIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CTEIF2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the TEIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF2_AW::CLEAR)
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
///Channel x half transfer clear (x = 1 ..7)
pub type CHTIF2_AW = CHTIF1_AW;
///Field `CHTIF2` writer - Channel x half transfer clear (x = 1 ..7)
pub struct CHTIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CHTIF2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CHTIF2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the HTIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF2_AW::CLEAR)
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
///Channel x transfer complete clear (x = 1 ..7)
pub type CTCIF2_AW = CTCIF1_AW;
///Field `CTCIF2` writer - Channel x transfer complete clear (x = 1 ..7)
pub struct CTCIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIF2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CTCIF2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the TCIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF2_AW::CLEAR)
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
///Channel x global interrupt clear (x = 1 ..7)
pub type CGIF2_AW = CGIF1_AW;
///Field `CGIF2` writer - Channel x global interrupt clear (x = 1 ..7)
pub struct CGIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CGIF2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CGIF2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the GIF, TEIF, HTIF, TCIF flags in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CGIF2_AW::CLEAR)
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
///Channel x transfer error clear (x = 1 ..7)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTEIF1_AW {
    ///1: Clears the TEIF flag in the ISR register
    CLEAR = 1,
}
impl From<CTEIF1_AW> for bool {
    #[inline(always)]
    fn from(variant: CTEIF1_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CTEIF1` writer - Channel x transfer error clear (x = 1 ..7)
pub struct CTEIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CTEIF1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the TEIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF1_AW::CLEAR)
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
///Channel x half transfer clear (x = 1 ..7)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHTIF1_AW {
    ///1: Clears the HTIF flag in the ISR register
    CLEAR = 1,
}
impl From<CHTIF1_AW> for bool {
    #[inline(always)]
    fn from(variant: CHTIF1_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CHTIF1` writer - Channel x half transfer clear (x = 1 ..7)
pub struct CHTIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CHTIF1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CHTIF1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the HTIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF1_AW::CLEAR)
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
///Channel x transfer complete clear (x = 1 ..7)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTCIF1_AW {
    ///1: Clears the TCIF flag in the ISR register
    CLEAR = 1,
}
impl From<CTCIF1_AW> for bool {
    #[inline(always)]
    fn from(variant: CTCIF1_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CTCIF1` writer - Channel x transfer complete clear (x = 1 ..7)
pub struct CTCIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIF1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CTCIF1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the TCIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF1_AW::CLEAR)
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
///Channel x global interrupt clear (x = 1 ..7)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CGIF1_AW {
    ///1: Clears the GIF, TEIF, HTIF, TCIF flags in the ISR register
    CLEAR = 1,
}
impl From<CGIF1_AW> for bool {
    #[inline(always)]
    fn from(variant: CGIF1_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CGIF1` writer - Channel x global interrupt clear (x = 1 ..7)
pub struct CGIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CGIF1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CGIF1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears the GIF, TEIF, HTIF, TCIF flags in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CGIF1_AW::CLEAR)
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
impl W {
    ///Bit 27 - Channel x transfer error clear (x = 1 ..7)
    #[inline(always)]
    pub fn cteif7(&mut self) -> CTEIF7_W {
        CTEIF7_W { w: self }
    }
    ///Bit 26 - Channel x half transfer clear (x = 1 ..7)
    #[inline(always)]
    pub fn chtif7(&mut self) -> CHTIF7_W {
        CHTIF7_W { w: self }
    }
    ///Bit 25 - Channel x transfer complete clear (x = 1 ..7)
    #[inline(always)]
    pub fn ctcif7(&mut self) -> CTCIF7_W {
        CTCIF7_W { w: self }
    }
    ///Bit 24 - Channel x global interrupt clear (x = 1 ..7)
    #[inline(always)]
    pub fn cgif7(&mut self) -> CGIF7_W {
        CGIF7_W { w: self }
    }
    ///Bit 23 - Channel x transfer error clear (x = 1 ..7)
    #[inline(always)]
    pub fn cteif6(&mut self) -> CTEIF6_W {
        CTEIF6_W { w: self }
    }
    ///Bit 22 - Channel x half transfer clear (x = 1 ..7)
    #[inline(always)]
    pub fn chtif6(&mut self) -> CHTIF6_W {
        CHTIF6_W { w: self }
    }
    ///Bit 21 - Channel x transfer complete clear (x = 1 ..7)
    #[inline(always)]
    pub fn ctcif6(&mut self) -> CTCIF6_W {
        CTCIF6_W { w: self }
    }
    ///Bit 20 - Channel x global interrupt clear (x = 1 ..7)
    #[inline(always)]
    pub fn cgif6(&mut self) -> CGIF6_W {
        CGIF6_W { w: self }
    }
    ///Bit 19 - Channel x transfer error clear (x = 1 ..7)
    #[inline(always)]
    pub fn cteif5(&mut self) -> CTEIF5_W {
        CTEIF5_W { w: self }
    }
    ///Bit 18 - Channel x half transfer clear (x = 1 ..7)
    #[inline(always)]
    pub fn chtif5(&mut self) -> CHTIF5_W {
        CHTIF5_W { w: self }
    }
    ///Bit 17 - Channel x transfer complete clear (x = 1 ..7)
    #[inline(always)]
    pub fn ctcif5(&mut self) -> CTCIF5_W {
        CTCIF5_W { w: self }
    }
    ///Bit 16 - Channel x global interrupt clear (x = 1 ..7)
    #[inline(always)]
    pub fn cgif5(&mut self) -> CGIF5_W {
        CGIF5_W { w: self }
    }
    ///Bit 15 - Channel x transfer error clear (x = 1 ..7)
    #[inline(always)]
    pub fn cteif4(&mut self) -> CTEIF4_W {
        CTEIF4_W { w: self }
    }
    ///Bit 14 - Channel x half transfer clear (x = 1 ..7)
    #[inline(always)]
    pub fn chtif4(&mut self) -> CHTIF4_W {
        CHTIF4_W { w: self }
    }
    ///Bit 13 - Channel x transfer complete clear (x = 1 ..7)
    #[inline(always)]
    pub fn ctcif4(&mut self) -> CTCIF4_W {
        CTCIF4_W { w: self }
    }
    ///Bit 12 - Channel x global interrupt clear (x = 1 ..7)
    #[inline(always)]
    pub fn cgif4(&mut self) -> CGIF4_W {
        CGIF4_W { w: self }
    }
    ///Bit 11 - Channel x transfer error clear (x = 1 ..7)
    #[inline(always)]
    pub fn cteif3(&mut self) -> CTEIF3_W {
        CTEIF3_W { w: self }
    }
    ///Bit 10 - Channel x half transfer clear (x = 1 ..7)
    #[inline(always)]
    pub fn chtif3(&mut self) -> CHTIF3_W {
        CHTIF3_W { w: self }
    }
    ///Bit 9 - Channel x transfer complete clear (x = 1 ..7)
    #[inline(always)]
    pub fn ctcif3(&mut self) -> CTCIF3_W {
        CTCIF3_W { w: self }
    }
    ///Bit 8 - Channel x global interrupt clear (x = 1 ..7)
    #[inline(always)]
    pub fn cgif3(&mut self) -> CGIF3_W {
        CGIF3_W { w: self }
    }
    ///Bit 7 - Channel x transfer error clear (x = 1 ..7)
    #[inline(always)]
    pub fn cteif2(&mut self) -> CTEIF2_W {
        CTEIF2_W { w: self }
    }
    ///Bit 6 - Channel x half transfer clear (x = 1 ..7)
    #[inline(always)]
    pub fn chtif2(&mut self) -> CHTIF2_W {
        CHTIF2_W { w: self }
    }
    ///Bit 5 - Channel x transfer complete clear (x = 1 ..7)
    #[inline(always)]
    pub fn ctcif2(&mut self) -> CTCIF2_W {
        CTCIF2_W { w: self }
    }
    ///Bit 4 - Channel x global interrupt clear (x = 1 ..7)
    #[inline(always)]
    pub fn cgif2(&mut self) -> CGIF2_W {
        CGIF2_W { w: self }
    }
    ///Bit 3 - Channel x transfer error clear (x = 1 ..7)
    #[inline(always)]
    pub fn cteif1(&mut self) -> CTEIF1_W {
        CTEIF1_W { w: self }
    }
    ///Bit 2 - Channel x half transfer clear (x = 1 ..7)
    #[inline(always)]
    pub fn chtif1(&mut self) -> CHTIF1_W {
        CHTIF1_W { w: self }
    }
    ///Bit 1 - Channel x transfer complete clear (x = 1 ..7)
    #[inline(always)]
    pub fn ctcif1(&mut self) -> CTCIF1_W {
        CTCIF1_W { w: self }
    }
    ///Bit 0 - Channel x global interrupt clear (x = 1 ..7)
    #[inline(always)]
    pub fn cgif1(&mut self) -> CGIF1_W {
        CGIF1_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///interrupt flag clear register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ifcr](index.html) module
pub struct IFCR_SPEC;
impl crate::RegisterSpec for IFCR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [ifcr::W](W) writer structure
impl crate::Writable for IFCR_SPEC {
    type Writer = W;
}
///`reset()` method sets IFCR to value 0
impl crate::Resettable for IFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
