#[doc = "Reader of register DMARPDR"]
pub type R = crate::R<u32, super::DMARPDR>;
#[doc = "Writer for register DMARPDR"]
pub type W = crate::W<u32, super::DMARPDR>;
#[doc = "Register DMARPDR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMARPDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RPD`"]
pub type RPD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RPD`"]
pub struct RPD_W<'a> {
    w: &'a mut W,
}
impl<'a> RPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Receive poll demand"]
    #[inline(always)]
    pub fn rpd(&self) -> RPD_R {
        RPD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive poll demand"]
    #[inline(always)]
    pub fn rpd(&mut self) -> RPD_W {
        RPD_W { w: self }
    }
}
