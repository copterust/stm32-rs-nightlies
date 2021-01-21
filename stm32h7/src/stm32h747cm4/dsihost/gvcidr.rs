#[doc = "Reader of register GVCIDR"]
pub type R = crate::R<u32, super::GVCIDR>;
#[doc = "Writer for register GVCIDR"]
pub type W = crate::W<u32, super::GVCIDR>;
#[doc = "Register GVCIDR `reset()`'s with value 0"]
impl crate::ResetValue for super::GVCIDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VCID`"]
pub type VCID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VCID`"]
pub struct VCID_W<'a> {
    w: &'a mut W,
}
impl<'a> VCID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Virtual channel ID"]
    #[inline(always)]
    pub fn vcid(&self) -> VCID_R {
        VCID_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Virtual channel ID"]
    #[inline(always)]
    pub fn vcid(&mut self) -> VCID_W {
        VCID_W { w: self }
    }
}
