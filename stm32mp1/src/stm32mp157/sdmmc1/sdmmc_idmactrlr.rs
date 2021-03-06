#[doc = "Reader of register SDMMC_IDMACTRLR"]
pub type R = crate::R<u32, super::SDMMC_IDMACTRLR>;
#[doc = "Writer for register SDMMC_IDMACTRLR"]
pub type W = crate::W<u32, super::SDMMC_IDMACTRLR>;
#[doc = "Register SDMMC_IDMACTRLR `reset()`'s with value 0"]
impl crate::ResetValue for super::SDMMC_IDMACTRLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IDMAEN`"]
pub type IDMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDMAEN`"]
pub struct IDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IDMAEN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `IDMABMODE`"]
pub type IDMABMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDMABMODE`"]
pub struct IDMABMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> IDMABMODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - IDMAEN"]
    #[inline(always)]
    pub fn idmaen(&self) -> IDMAEN_R {
        IDMAEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IDMABMODE"]
    #[inline(always)]
    pub fn idmabmode(&self) -> IDMABMODE_R {
        IDMABMODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IDMAEN"]
    #[inline(always)]
    pub fn idmaen(&mut self) -> IDMAEN_W {
        IDMAEN_W { w: self }
    }
    #[doc = "Bit 1 - IDMABMODE"]
    #[inline(always)]
    pub fn idmabmode(&mut self) -> IDMABMODE_W {
        IDMABMODE_W { w: self }
    }
}
