#[doc = "Reader of register CR3"]
pub type R = crate::R<u32, super::CR3>;
#[doc = "Writer for register CR3"]
pub type W = crate::W<u32, super::CR3>;
#[doc = "Register CR3 `reset()`'s with value 0x8000"]
impl crate::ResetValue for super::CR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000
    }
}
#[doc = "Reader of field `EIWUL`"]
pub type EIWUL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EIWUL`"]
pub struct EIWUL_W<'a> {
    w: &'a mut W,
}
impl<'a> EIWUL_W<'a> {
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
#[doc = "Reader of field `EC2H`"]
pub type EC2H_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EC2H`"]
pub struct EC2H_W<'a> {
    w: &'a mut W,
}
impl<'a> EC2H_W<'a> {
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
#[doc = "Reader of field `EWRFIRQ`"]
pub type EWRFIRQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EWRFIRQ`"]
pub struct EWRFIRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> EWRFIRQ_W<'a> {
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
#[doc = "Reader of field `EWRFBUSY`"]
pub type EWRFBUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EWRFBUSY`"]
pub struct EWRFBUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> EWRFBUSY_W<'a> {
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
pub type RRS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RRS`"]
pub struct RRS_W<'a> {
    w: &'a mut W,
}
impl<'a> RRS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `EWPVD`"]
pub type EWPVD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EWPVD`"]
pub struct EWPVD_W<'a> {
    w: &'a mut W,
}
impl<'a> EWPVD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `EULPEN`"]
pub type EULPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EULPEN`"]
pub struct EULPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EULPEN_W<'a> {
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
    #[doc = "Bit 15 - Enable internal wakeup line for CPU1"]
    #[inline(always)]
    pub fn eiwul(&self) -> EIWUL_R {
        EIWUL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - nable CPU2 Hold interrupt for CPU1"]
    #[inline(always)]
    pub fn ec2h(&self) -> EC2H_R {
        EC2H_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - akeup for CPU1"]
    #[inline(always)]
    pub fn ewrfirq(&self) -> EWRFIRQ_R {
        EWRFIRQ_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable Radio BUSY Wakeup from Standby for CPU1"]
    #[inline(always)]
    pub fn ewrfbusy(&self) -> EWRFBUSY_R {
        EWRFBUSY_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Apply pull-up and pull-down configuration from CPU1"]
    #[inline(always)]
    pub fn apc(&self) -> APC_R {
        APC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SRAM2 retention in Standby mode"]
    #[inline(always)]
    pub fn rrs(&self) -> RRS_R {
        RRS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable wakeup PVD for CPU1"]
    #[inline(always)]
    pub fn ewpvd(&self) -> EWPVD_R {
        EWPVD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Ultra-low-power enable"]
    #[inline(always)]
    pub fn eulpen(&self) -> EULPEN_R {
        EULPEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable Wakeup pin WKUP3 for CPU1"]
    #[inline(always)]
    pub fn ewup3(&self) -> EWUP3_R {
        EWUP3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable Wakeup pin WKUP2 for CPU1"]
    #[inline(always)]
    pub fn ewup2(&self) -> EWUP2_R {
        EWUP2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enable Wakeup pin WKUP1 for CPU1"]
    #[inline(always)]
    pub fn ewup1(&self) -> EWUP1_R {
        EWUP1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Enable internal wakeup line for CPU1"]
    #[inline(always)]
    pub fn eiwul(&mut self) -> EIWUL_W {
        EIWUL_W { w: self }
    }
    #[doc = "Bit 14 - nable CPU2 Hold interrupt for CPU1"]
    #[inline(always)]
    pub fn ec2h(&mut self) -> EC2H_W {
        EC2H_W { w: self }
    }
    #[doc = "Bit 13 - akeup for CPU1"]
    #[inline(always)]
    pub fn ewrfirq(&mut self) -> EWRFIRQ_W {
        EWRFIRQ_W { w: self }
    }
    #[doc = "Bit 11 - Enable Radio BUSY Wakeup from Standby for CPU1"]
    #[inline(always)]
    pub fn ewrfbusy(&mut self) -> EWRFBUSY_W {
        EWRFBUSY_W { w: self }
    }
    #[doc = "Bit 10 - Apply pull-up and pull-down configuration from CPU1"]
    #[inline(always)]
    pub fn apc(&mut self) -> APC_W {
        APC_W { w: self }
    }
    #[doc = "Bit 9 - SRAM2 retention in Standby mode"]
    #[inline(always)]
    pub fn rrs(&mut self) -> RRS_W {
        RRS_W { w: self }
    }
    #[doc = "Bit 8 - Enable wakeup PVD for CPU1"]
    #[inline(always)]
    pub fn ewpvd(&mut self) -> EWPVD_W {
        EWPVD_W { w: self }
    }
    #[doc = "Bit 7 - Ultra-low-power enable"]
    #[inline(always)]
    pub fn eulpen(&mut self) -> EULPEN_W {
        EULPEN_W { w: self }
    }
    #[doc = "Bit 2 - Enable Wakeup pin WKUP3 for CPU1"]
    #[inline(always)]
    pub fn ewup3(&mut self) -> EWUP3_W {
        EWUP3_W { w: self }
    }
    #[doc = "Bit 1 - Enable Wakeup pin WKUP2 for CPU1"]
    #[inline(always)]
    pub fn ewup2(&mut self) -> EWUP2_W {
        EWUP2_W { w: self }
    }
    #[doc = "Bit 0 - Enable Wakeup pin WKUP1 for CPU1"]
    #[inline(always)]
    pub fn ewup1(&mut self) -> EWUP1_W {
        EWUP1_W { w: self }
    }
}
