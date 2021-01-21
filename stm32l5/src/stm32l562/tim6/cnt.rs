#[doc = "Reader of register CNT"]
pub type R = crate::R<u32, super::CNT>;
#[doc = "Writer for register CNT"]
pub type W = crate::W<u32, super::CNT>;
#[doc = "Register CNT `reset()`'s with value 0"]
impl crate::ResetValue for super::CNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CNT_bit0`"]
pub type CNT_BIT0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CNT_bit0`"]
pub struct CNT_BIT0_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_BIT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `UIFCPY`"]
pub type UIFCPY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UIFCPY`"]
pub struct UIFCPY_W<'a> {
    w: &'a mut W,
}
impl<'a> UIFCPY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CNT"]
    #[inline(always)]
    pub fn cnt_bit0(&self) -> CNT_BIT0_R {
        CNT_BIT0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - UIFCPY or Res"]
    #[inline(always)]
    pub fn uifcpy(&self) -> UIFCPY_R {
        UIFCPY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - CNT"]
    #[inline(always)]
    pub fn cnt_bit0(&mut self) -> CNT_BIT0_W {
        CNT_BIT0_W { w: self }
    }
    #[doc = "Bit 31 - UIFCPY or Res"]
    #[inline(always)]
    pub fn uifcpy(&mut self) -> UIFCPY_W {
        UIFCPY_W { w: self }
    }
}
