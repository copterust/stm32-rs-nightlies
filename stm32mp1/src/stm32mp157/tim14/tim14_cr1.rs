#[doc = "Reader of register TIM14_CR1"]
pub type R = crate::R<u16, super::TIM14_CR1>;
#[doc = "Writer for register TIM14_CR1"]
pub type W = crate::W<u16, super::TIM14_CR1>;
#[doc = "Register TIM14_CR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::TIM14_CR1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CEN`"]
pub type CEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CEN`"]
pub struct CEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CEN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `UDIS`"]
pub type UDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UDIS`"]
pub struct UDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> UDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `URS`"]
pub type URS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `URS`"]
pub struct URS_W<'a> {
    w: &'a mut W,
}
impl<'a> URS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `OPM`"]
pub type OPM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPM`"]
pub struct OPM_W<'a> {
    w: &'a mut W,
}
impl<'a> OPM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `ARPE`"]
pub type ARPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARPE`"]
pub struct ARPE_W<'a> {
    w: &'a mut W,
}
impl<'a> ARPE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `CKD`"]
pub type CKD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CKD`"]
pub struct CKD_W<'a> {
    w: &'a mut W,
}
impl<'a> CKD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u16) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `UIFREMAP`"]
pub type UIFREMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UIFREMAP`"]
pub struct UIFREMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> UIFREMAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - CEN"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - UDIS"]
    #[inline(always)]
    pub fn udis(&self) -> UDIS_R {
        UDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - URS"]
    #[inline(always)]
    pub fn urs(&self) -> URS_R {
        URS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - OPM"]
    #[inline(always)]
    pub fn opm(&self) -> OPM_R {
        OPM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ARPE"]
    #[inline(always)]
    pub fn arpe(&self) -> ARPE_R {
        ARPE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - CKD"]
    #[inline(always)]
    pub fn ckd(&self) -> CKD_R {
        CKD_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 11 - UIFREMAP"]
    #[inline(always)]
    pub fn uifremap(&self) -> UIFREMAP_R {
        UIFREMAP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CEN"]
    #[inline(always)]
    pub fn cen(&mut self) -> CEN_W {
        CEN_W { w: self }
    }
    #[doc = "Bit 1 - UDIS"]
    #[inline(always)]
    pub fn udis(&mut self) -> UDIS_W {
        UDIS_W { w: self }
    }
    #[doc = "Bit 2 - URS"]
    #[inline(always)]
    pub fn urs(&mut self) -> URS_W {
        URS_W { w: self }
    }
    #[doc = "Bit 3 - OPM"]
    #[inline(always)]
    pub fn opm(&mut self) -> OPM_W {
        OPM_W { w: self }
    }
    #[doc = "Bit 7 - ARPE"]
    #[inline(always)]
    pub fn arpe(&mut self) -> ARPE_W {
        ARPE_W { w: self }
    }
    #[doc = "Bits 8:9 - CKD"]
    #[inline(always)]
    pub fn ckd(&mut self) -> CKD_W {
        CKD_W { w: self }
    }
    #[doc = "Bit 11 - UIFREMAP"]
    #[inline(always)]
    pub fn uifremap(&mut self) -> UIFREMAP_W {
        UIFREMAP_W { w: self }
    }
}
