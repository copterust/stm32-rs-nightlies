#[doc = "Reader of register CR3"]
pub type R = crate::R<u32, super::CR3>;
#[doc = "Writer for register CR3"]
pub type W = crate::W<u32, super::CR3>;
#[doc = "Register CR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::CR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UCPD_DBDIS`"]
pub type UCPD_DBDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCPD_DBDIS`"]
pub struct UCPD_DBDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> UCPD_DBDIS_W<'a> {
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
#[doc = "Reader of field `UCPD_STDBY`"]
pub type UCPD_STDBY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCPD_STDBY`"]
pub struct UCPD_STDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> UCPD_STDBY_W<'a> {
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
#[doc = "Reader of field `ULPMEN`"]
pub type ULPMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ULPMEN`"]
pub struct ULPMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ULPMEN_W<'a> {
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
#[doc = "Reader of field `APC`"]
pub type APC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APC`"]
pub struct APC_W<'a> {
    w: &'a mut W,
}
impl<'a> APC_W<'a> {
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
#[doc = "Reader of field `RRS`"]
pub type RRS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RRS`"]
pub struct RRS_W<'a> {
    w: &'a mut W,
}
impl<'a> RRS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `EWUP5`"]
pub type EWUP5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EWUP5`"]
pub struct EWUP5_W<'a> {
    w: &'a mut W,
}
impl<'a> EWUP5_W<'a> {
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
#[doc = "Reader of field `EWUP4`"]
pub type EWUP4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EWUP4`"]
pub struct EWUP4_W<'a> {
    w: &'a mut W,
}
impl<'a> EWUP4_W<'a> {
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
#[doc = "Reader of field `EWUP3`"]
pub type EWUP3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EWUP3`"]
pub struct EWUP3_W<'a> {
    w: &'a mut W,
}
impl<'a> EWUP3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `EWUP2`"]
pub type EWUP2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EWUP2`"]
pub struct EWUP2_W<'a> {
    w: &'a mut W,
}
impl<'a> EWUP2_W<'a> {
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
#[doc = "Reader of field `EWUP1`"]
pub type EWUP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EWUP1`"]
pub struct EWUP1_W<'a> {
    w: &'a mut W,
}
impl<'a> EWUP1_W<'a> {
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
impl R {
    #[doc = "Bit 14 - UCPD_DBDIS"]
    #[inline(always)]
    pub fn ucpd_dbdis(&self) -> UCPD_DBDIS_R {
        UCPD_DBDIS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - UCPD_STDBY"]
    #[inline(always)]
    pub fn ucpd_stdby(&self) -> UCPD_STDBY_R {
        UCPD_STDBY_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ULPMEN"]
    #[inline(always)]
    pub fn ulpmen(&self) -> ULPMEN_R {
        ULPMEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Apply pull-up and pull-down configuration"]
    #[inline(always)]
    pub fn apc(&self) -> APC_R {
        APC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - SRAM2 retention in Standby mode"]
    #[inline(always)]
    pub fn rrs(&self) -> RRS_R {
        RRS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Enable Wakeup pin WKUP5"]
    #[inline(always)]
    pub fn ewup5(&self) -> EWUP5_R {
        EWUP5_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable Wakeup pin WKUP4"]
    #[inline(always)]
    pub fn ewup4(&self) -> EWUP4_R {
        EWUP4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable Wakeup pin WKUP3"]
    #[inline(always)]
    pub fn ewup3(&self) -> EWUP3_R {
        EWUP3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable Wakeup pin WKUP2"]
    #[inline(always)]
    pub fn ewup2(&self) -> EWUP2_R {
        EWUP2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enable Wakeup pin WKUP1"]
    #[inline(always)]
    pub fn ewup1(&self) -> EWUP1_R {
        EWUP1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - UCPD_DBDIS"]
    #[inline(always)]
    pub fn ucpd_dbdis(&mut self) -> UCPD_DBDIS_W {
        UCPD_DBDIS_W { w: self }
    }
    #[doc = "Bit 13 - UCPD_STDBY"]
    #[inline(always)]
    pub fn ucpd_stdby(&mut self) -> UCPD_STDBY_W {
        UCPD_STDBY_W { w: self }
    }
    #[doc = "Bit 11 - ULPMEN"]
    #[inline(always)]
    pub fn ulpmen(&mut self) -> ULPMEN_W {
        ULPMEN_W { w: self }
    }
    #[doc = "Bit 10 - Apply pull-up and pull-down configuration"]
    #[inline(always)]
    pub fn apc(&mut self) -> APC_W {
        APC_W { w: self }
    }
    #[doc = "Bits 8:9 - SRAM2 retention in Standby mode"]
    #[inline(always)]
    pub fn rrs(&mut self) -> RRS_W {
        RRS_W { w: self }
    }
    #[doc = "Bit 4 - Enable Wakeup pin WKUP5"]
    #[inline(always)]
    pub fn ewup5(&mut self) -> EWUP5_W {
        EWUP5_W { w: self }
    }
    #[doc = "Bit 3 - Enable Wakeup pin WKUP4"]
    #[inline(always)]
    pub fn ewup4(&mut self) -> EWUP4_W {
        EWUP4_W { w: self }
    }
    #[doc = "Bit 2 - Enable Wakeup pin WKUP3"]
    #[inline(always)]
    pub fn ewup3(&mut self) -> EWUP3_W {
        EWUP3_W { w: self }
    }
    #[doc = "Bit 1 - Enable Wakeup pin WKUP2"]
    #[inline(always)]
    pub fn ewup2(&mut self) -> EWUP2_W {
        EWUP2_W { w: self }
    }
    #[doc = "Bit 0 - Enable Wakeup pin WKUP1"]
    #[inline(always)]
    pub fn ewup1(&mut self) -> EWUP1_W {
        EWUP1_W { w: self }
    }
}
