#[doc = "Reader of register DDRCTRL_PCFGW_0"]
pub type R = crate::R<u32, super::DDRCTRL_PCFGW_0>;
#[doc = "Writer for register DDRCTRL_PCFGW_0"]
pub type W = crate::W<u32, super::DDRCTRL_PCFGW_0>;
#[doc = "Register DDRCTRL_PCFGW_0 `reset()`'s with value 0x4000"]
impl crate::ResetValue for super::DDRCTRL_PCFGW_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4000
    }
}
#[doc = "Reader of field `WR_PORT_PRIORITY`"]
pub type WR_PORT_PRIORITY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WR_PORT_PRIORITY`"]
pub struct WR_PORT_PRIORITY_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_PORT_PRIORITY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `WR_PORT_AGING_EN`"]
pub type WR_PORT_AGING_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WR_PORT_AGING_EN`"]
pub struct WR_PORT_AGING_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_PORT_AGING_EN_W<'a> {
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
#[doc = "Reader of field `WR_PORT_URGENT_EN`"]
pub type WR_PORT_URGENT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WR_PORT_URGENT_EN`"]
pub struct WR_PORT_URGENT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_PORT_URGENT_EN_W<'a> {
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
#[doc = "Reader of field `WR_PORT_PAGEMATCH_EN`"]
pub type WR_PORT_PAGEMATCH_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WR_PORT_PAGEMATCH_EN`"]
pub struct WR_PORT_PAGEMATCH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_PORT_PAGEMATCH_EN_W<'a> {
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
impl R {
    #[doc = "Bits 0:9 - WR_PORT_PRIORITY"]
    #[inline(always)]
    pub fn wr_port_priority(&self) -> WR_PORT_PRIORITY_R {
        WR_PORT_PRIORITY_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 12 - WR_PORT_AGING_EN"]
    #[inline(always)]
    pub fn wr_port_aging_en(&self) -> WR_PORT_AGING_EN_R {
        WR_PORT_AGING_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - WR_PORT_URGENT_EN"]
    #[inline(always)]
    pub fn wr_port_urgent_en(&self) -> WR_PORT_URGENT_EN_R {
        WR_PORT_URGENT_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - WR_PORT_PAGEMATCH_EN"]
    #[inline(always)]
    pub fn wr_port_pagematch_en(&self) -> WR_PORT_PAGEMATCH_EN_R {
        WR_PORT_PAGEMATCH_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - WR_PORT_PRIORITY"]
    #[inline(always)]
    pub fn wr_port_priority(&mut self) -> WR_PORT_PRIORITY_W {
        WR_PORT_PRIORITY_W { w: self }
    }
    #[doc = "Bit 12 - WR_PORT_AGING_EN"]
    #[inline(always)]
    pub fn wr_port_aging_en(&mut self) -> WR_PORT_AGING_EN_W {
        WR_PORT_AGING_EN_W { w: self }
    }
    #[doc = "Bit 13 - WR_PORT_URGENT_EN"]
    #[inline(always)]
    pub fn wr_port_urgent_en(&mut self) -> WR_PORT_URGENT_EN_W {
        WR_PORT_URGENT_EN_W { w: self }
    }
    #[doc = "Bit 14 - WR_PORT_PAGEMATCH_EN"]
    #[inline(always)]
    pub fn wr_port_pagematch_en(&mut self) -> WR_PORT_PAGEMATCH_EN_W {
        WR_PORT_PAGEMATCH_EN_W { w: self }
    }
}
