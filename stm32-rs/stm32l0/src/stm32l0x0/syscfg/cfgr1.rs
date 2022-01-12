///Register `CFGR1` reader
pub struct R(crate::R<CFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CFGR1` writer
pub struct W(crate::W<CFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR1_SPEC>;
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
impl From<crate::W<CFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR1_SPEC>) -> Self {
        W(writer)
    }
}
///Boot mode selected by the boot pins status bits
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BOOT_MODE_A {
    ///0: Main Flash memory boot mode
    MAINFLASH = 0,
    ///1: System Flash memory boot mode
    SYSTEMFLASH = 1,
    ///3: Embedded SRAM boot mode
    SRAM = 3,
}
impl From<BOOT_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: BOOT_MODE_A) -> Self {
        variant as _
    }
}
///Field `BOOT_MODE` reader - Boot mode selected by the boot pins status bits
pub struct BOOT_MODE_R(crate::FieldReader<u8, BOOT_MODE_A>);
impl BOOT_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BOOT_MODE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<BOOT_MODE_A> {
        match self.bits {
            0 => Some(BOOT_MODE_A::MAINFLASH),
            1 => Some(BOOT_MODE_A::SYSTEMFLASH),
            3 => Some(BOOT_MODE_A::SRAM),
            _ => None,
        }
    }
    ///Checks if the value of the field is `MAINFLASH`
    #[inline(always)]
    pub fn is_main_flash(&self) -> bool {
        **self == BOOT_MODE_A::MAINFLASH
    }
    ///Checks if the value of the field is `SYSTEMFLASH`
    #[inline(always)]
    pub fn is_system_flash(&self) -> bool {
        **self == BOOT_MODE_A::SYSTEMFLASH
    }
    ///Checks if the value of the field is `SRAM`
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        **self == BOOT_MODE_A::SRAM
    }
}
impl core::ops::Deref for BOOT_MODE_R {
    type Target = crate::FieldReader<u8, BOOT_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Memory mapping selection bits
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MEM_MODE_A {
    ///0: Main Flash memory mapped at 0x0000_0000
    MAINFLASH = 0,
    ///1: System Flash memory mapped at 0x0000_0000
    SYSTEMFLASH = 1,
    ///3: Embedded SRAM mapped at 0x0000_0000
    SRAM = 3,
}
impl From<MEM_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MEM_MODE_A) -> Self {
        variant as _
    }
}
///Field `MEM_MODE` reader - Memory mapping selection bits
pub struct MEM_MODE_R(crate::FieldReader<u8, MEM_MODE_A>);
impl MEM_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MEM_MODE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<MEM_MODE_A> {
        match self.bits {
            0 => Some(MEM_MODE_A::MAINFLASH),
            1 => Some(MEM_MODE_A::SYSTEMFLASH),
            3 => Some(MEM_MODE_A::SRAM),
            _ => None,
        }
    }
    ///Checks if the value of the field is `MAINFLASH`
    #[inline(always)]
    pub fn is_main_flash(&self) -> bool {
        **self == MEM_MODE_A::MAINFLASH
    }
    ///Checks if the value of the field is `SYSTEMFLASH`
    #[inline(always)]
    pub fn is_system_flash(&self) -> bool {
        **self == MEM_MODE_A::SYSTEMFLASH
    }
    ///Checks if the value of the field is `SRAM`
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        **self == MEM_MODE_A::SRAM
    }
}
impl core::ops::Deref for MEM_MODE_R {
    type Target = crate::FieldReader<u8, MEM_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MEM_MODE` writer - Memory mapping selection bits
pub struct MEM_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_MODE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MEM_MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Main Flash memory mapped at 0x0000_0000
    #[inline(always)]
    pub fn main_flash(self) -> &'a mut W {
        self.variant(MEM_MODE_A::MAINFLASH)
    }
    ///System Flash memory mapped at 0x0000_0000
    #[inline(always)]
    pub fn system_flash(self) -> &'a mut W {
        self.variant(MEM_MODE_A::SYSTEMFLASH)
    }
    ///Embedded SRAM mapped at 0x0000_0000
    #[inline(always)]
    pub fn sram(self) -> &'a mut W {
        self.variant(MEM_MODE_A::SRAM)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    ///Bits 8:9 - Boot mode selected by the boot pins status bits
    #[inline(always)]
    pub fn boot_mode(&self) -> BOOT_MODE_R {
        BOOT_MODE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    ///Bits 0:1 - Memory mapping selection bits
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    ///Bits 0:1 - Memory mapping selection bits
    #[inline(always)]
    pub fn mem_mode(&mut self) -> MEM_MODE_W {
        MEM_MODE_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SYSCFG configuration register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfgr1](index.html) module
pub struct CFGR1_SPEC;
impl crate::RegisterSpec for CFGR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [cfgr1::R](R) reader structure
impl crate::Readable for CFGR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cfgr1::W](W) writer structure
impl crate::Writable for CFGR1_SPEC {
    type Writer = W;
}
///`reset()` method sets CFGR1 to value 0
impl crate::Resettable for CFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
