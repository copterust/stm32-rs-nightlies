#[doc = "Writer for register QUADSPI_FCR"]
pub type W = crate::W<u32, super::QUADSPI_FCR>;
#[doc = "Register QUADSPI_FCR `reset()`'s with value 0"]
impl crate::ResetValue for super::QUADSPI_FCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CTEF`"]
pub struct CTEF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEF_W<'a> {
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
#[doc = "Write proxy for field `CTCF`"]
pub struct CTCF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCF_W<'a> {
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
#[doc = "Write proxy for field `CSMF`"]
pub struct CSMF_W<'a> {
    w: &'a mut W,
}
impl<'a> CSMF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Write proxy for field `CTOF`"]
pub struct CTOF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTOF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - CTEF"]
    #[inline(always)]
    pub fn ctef(&mut self) -> CTEF_W {
        CTEF_W { w: self }
    }
    #[doc = "Bit 1 - CTCF"]
    #[inline(always)]
    pub fn ctcf(&mut self) -> CTCF_W {
        CTCF_W { w: self }
    }
    #[doc = "Bit 3 - CSMF"]
    #[inline(always)]
    pub fn csmf(&mut self) -> CSMF_W {
        CSMF_W { w: self }
    }
    #[doc = "Bit 4 - CTOF"]
    #[inline(always)]
    pub fn ctof(&mut self) -> CTOF_W {
        CTOF_W { w: self }
    }
}
