#[doc = "Reader of register C2CR1"]
pub type R = crate::R<u32, super::C2CR1>;
#[doc = "Writer for register C2CR1"]
pub type W = crate::W<u32, super::C2CR1>;
#[doc = "Register C2CR1 `reset()`'s with value 0x07"]
impl crate::ResetValue for super::C2CR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x07
    }
}
#[doc = "Reader of field `FPDS`"]
pub type FPDS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPDS`"]
pub struct FPDS_W<'a> {
    w: &'a mut W,
}
impl<'a> FPDS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `FPDR`"]
pub type FPDR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPDR`"]
pub struct FPDR_W<'a> {
    w: &'a mut W,
}
impl<'a> FPDR_W<'a> {
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
#[doc = "Reader of field `LPMS`"]
pub type LPMS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPMS`"]
pub struct LPMS_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 5 - Flash memory power down mode during LPSleep for CPU2"]
    #[inline(always)]
    pub fn fpds(&self) -> FPDS_R {
        FPDS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Flash memory power down mode during LPRun for CPU2"]
    #[inline(always)]
    pub fn fpdr(&self) -> FPDR_R {
        FPDR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - Low-power mode selection for CPU2"]
    #[inline(always)]
    pub fn lpms(&self) -> LPMS_R {
        LPMS_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 5 - Flash memory power down mode during LPSleep for CPU2"]
    #[inline(always)]
    pub fn fpds(&mut self) -> FPDS_W {
        FPDS_W { w: self }
    }
    #[doc = "Bit 4 - Flash memory power down mode during LPRun for CPU2"]
    #[inline(always)]
    pub fn fpdr(&mut self) -> FPDR_W {
        FPDR_W { w: self }
    }
    #[doc = "Bits 0:2 - Low-power mode selection for CPU2"]
    #[inline(always)]
    pub fn lpms(&mut self) -> LPMS_W {
        LPMS_W { w: self }
    }
}
