#[doc = "Reader of register DOUTR28"]
pub type R = crate::R<u32, super::DOUTR28>;
#[doc = "Writer for register DOUTR28"]
pub type W = crate::W<u32, super::DOUTR28>;
#[doc = "Register DOUTR28 `reset()`'s with value 0"]
impl crate::ResetValue for super::DOUTR28 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DOUT28`"]
pub type DOUT28_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DOUT28`"]
pub struct DOUT28_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT28_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout28(&self) -> DOUT28_R {
        DOUT28_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout28(&mut self) -> DOUT28_W {
        DOUT28_W { w: self }
    }
}
