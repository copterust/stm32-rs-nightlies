#[doc = "Reader of register SDMMC_CMDR"]
pub type R = crate::R<u32, super::SDMMC_CMDR>;
#[doc = "Writer for register SDMMC_CMDR"]
pub type W = crate::W<u32, super::SDMMC_CMDR>;
#[doc = "Register SDMMC_CMDR `reset()`'s with value 0"]
impl crate::ResetValue for super::SDMMC_CMDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMDINDEX`"]
pub type CMDINDEX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMDINDEX`"]
pub struct CMDINDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDINDEX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `CMDTRANS`"]
pub type CMDTRANS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMDTRANS`"]
pub struct CMDTRANS_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDTRANS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `CMDSTOP`"]
pub type CMDSTOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMDSTOP`"]
pub struct CMDSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDSTOP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `WAITRESP`"]
pub type WAITRESP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WAITRESP`"]
pub struct WAITRESP_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITRESP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `WAITINT`"]
pub type WAITINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAITINT`"]
pub struct WAITINT_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITINT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `WAITPEND`"]
pub type WAITPEND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAITPEND`"]
pub struct WAITPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITPEND_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `CPSMEN`"]
pub type CPSMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPSMEN`"]
pub struct CPSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CPSMEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `DTHOLD`"]
pub type DTHOLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTHOLD`"]
pub struct DTHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> DTHOLD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `BOOTMODE`"]
pub type BOOTMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOOTMODE`"]
pub struct BOOTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOTMODE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `BOOTEN`"]
pub type BOOTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOOTEN`"]
pub struct BOOTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOTEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `CMDSUSPEND`"]
pub type CMDSUSPEND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMDSUSPEND`"]
pub struct CMDSUSPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDSUSPEND_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - CMDINDEX"]
    #[inline(always)]
    pub fn cmdindex(&self) -> CMDINDEX_R {
        CMDINDEX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - CMDTRANS"]
    #[inline(always)]
    pub fn cmdtrans(&self) -> CMDTRANS_R {
        CMDTRANS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CMDSTOP"]
    #[inline(always)]
    pub fn cmdstop(&self) -> CMDSTOP_R {
        CMDSTOP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - WAITRESP"]
    #[inline(always)]
    pub fn waitresp(&self) -> WAITRESP_R {
        WAITRESP_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - WAITINT"]
    #[inline(always)]
    pub fn waitint(&self) -> WAITINT_R {
        WAITINT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - WAITPEND"]
    #[inline(always)]
    pub fn waitpend(&self) -> WAITPEND_R {
        WAITPEND_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - CPSMEN"]
    #[inline(always)]
    pub fn cpsmen(&self) -> CPSMEN_R {
        CPSMEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - DTHOLD"]
    #[inline(always)]
    pub fn dthold(&self) -> DTHOLD_R {
        DTHOLD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - BOOTMODE"]
    #[inline(always)]
    pub fn bootmode(&self) -> BOOTMODE_R {
        BOOTMODE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - BOOTEN"]
    #[inline(always)]
    pub fn booten(&self) -> BOOTEN_R {
        BOOTEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CMDSUSPEND"]
    #[inline(always)]
    pub fn cmdsuspend(&self) -> CMDSUSPEND_R {
        CMDSUSPEND_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - CMDINDEX"]
    #[inline(always)]
    pub fn cmdindex(&mut self) -> CMDINDEX_W {
        CMDINDEX_W { w: self }
    }
    #[doc = "Bit 6 - CMDTRANS"]
    #[inline(always)]
    pub fn cmdtrans(&mut self) -> CMDTRANS_W {
        CMDTRANS_W { w: self }
    }
    #[doc = "Bit 7 - CMDSTOP"]
    #[inline(always)]
    pub fn cmdstop(&mut self) -> CMDSTOP_W {
        CMDSTOP_W { w: self }
    }
    #[doc = "Bits 8:9 - WAITRESP"]
    #[inline(always)]
    pub fn waitresp(&mut self) -> WAITRESP_W {
        WAITRESP_W { w: self }
    }
    #[doc = "Bit 10 - WAITINT"]
    #[inline(always)]
    pub fn waitint(&mut self) -> WAITINT_W {
        WAITINT_W { w: self }
    }
    #[doc = "Bit 11 - WAITPEND"]
    #[inline(always)]
    pub fn waitpend(&mut self) -> WAITPEND_W {
        WAITPEND_W { w: self }
    }
    #[doc = "Bit 12 - CPSMEN"]
    #[inline(always)]
    pub fn cpsmen(&mut self) -> CPSMEN_W {
        CPSMEN_W { w: self }
    }
    #[doc = "Bit 13 - DTHOLD"]
    #[inline(always)]
    pub fn dthold(&mut self) -> DTHOLD_W {
        DTHOLD_W { w: self }
    }
    #[doc = "Bit 14 - BOOTMODE"]
    #[inline(always)]
    pub fn bootmode(&mut self) -> BOOTMODE_W {
        BOOTMODE_W { w: self }
    }
    #[doc = "Bit 15 - BOOTEN"]
    #[inline(always)]
    pub fn booten(&mut self) -> BOOTEN_W {
        BOOTEN_W { w: self }
    }
    #[doc = "Bit 16 - CMDSUSPEND"]
    #[inline(always)]
    pub fn cmdsuspend(&mut self) -> CMDSUSPEND_W {
        CMDSUSPEND_W { w: self }
    }
}
