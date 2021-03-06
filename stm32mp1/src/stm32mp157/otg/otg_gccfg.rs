#[doc = "Reader of register OTG_GCCFG"]
pub type R = crate::R<u32, super::OTG_GCCFG>;
#[doc = "Writer for register OTG_GCCFG"]
pub type W = crate::W<u32, super::OTG_GCCFG>;
#[doc = "Register OTG_GCCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::OTG_GCCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PDET`"]
pub type PDET_R = crate::R<bool, bool>;
#[doc = "Reader of field `SDET`"]
pub type SDET_R = crate::R<bool, bool>;
#[doc = "Reader of field `PS2DET`"]
pub type PS2DET_R = crate::R<bool, bool>;
#[doc = "Reader of field `PWRDWN`"]
pub type PWRDWN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWRDWN`"]
pub struct PWRDWN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRDWN_W<'a> {
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
#[doc = "Reader of field `BCDEN`"]
pub type BCDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BCDEN`"]
pub struct BCDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BCDEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `PDEN`"]
pub type PDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDEN`"]
pub struct PDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `SDEN`"]
pub type SDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDEN`"]
pub struct SDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `VBDEN`"]
pub type VBDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBDEN`"]
pub struct VBDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VBDEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `IDEN`"]
pub type IDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDEN`"]
pub struct IDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IDEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - PDET"]
    #[inline(always)]
    pub fn pdet(&self) -> PDET_R {
        PDET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SDET"]
    #[inline(always)]
    pub fn sdet(&self) -> SDET_R {
        SDET_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PS2DET"]
    #[inline(always)]
    pub fn ps2det(&self) -> PS2DET_R {
        PS2DET_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 16 - PWRDWN"]
    #[inline(always)]
    pub fn pwrdwn(&self) -> PWRDWN_R {
        PWRDWN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - BCDEN"]
    #[inline(always)]
    pub fn bcden(&self) -> BCDEN_R {
        BCDEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PDEN"]
    #[inline(always)]
    pub fn pden(&self) -> PDEN_R {
        PDEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - SDEN"]
    #[inline(always)]
    pub fn sden(&self) -> SDEN_R {
        SDEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - VBDEN"]
    #[inline(always)]
    pub fn vbden(&self) -> VBDEN_R {
        VBDEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - IDEN"]
    #[inline(always)]
    pub fn iden(&self) -> IDEN_R {
        IDEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - PWRDWN"]
    #[inline(always)]
    pub fn pwrdwn(&mut self) -> PWRDWN_W {
        PWRDWN_W { w: self }
    }
    #[doc = "Bit 17 - BCDEN"]
    #[inline(always)]
    pub fn bcden(&mut self) -> BCDEN_W {
        BCDEN_W { w: self }
    }
    #[doc = "Bit 19 - PDEN"]
    #[inline(always)]
    pub fn pden(&mut self) -> PDEN_W {
        PDEN_W { w: self }
    }
    #[doc = "Bit 20 - SDEN"]
    #[inline(always)]
    pub fn sden(&mut self) -> SDEN_W {
        SDEN_W { w: self }
    }
    #[doc = "Bit 21 - VBDEN"]
    #[inline(always)]
    pub fn vbden(&mut self) -> VBDEN_W {
        VBDEN_W { w: self }
    }
    #[doc = "Bit 22 - IDEN"]
    #[inline(always)]
    pub fn iden(&mut self) -> IDEN_W {
        IDEN_W { w: self }
    }
}
