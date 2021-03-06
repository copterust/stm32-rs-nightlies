#[doc = "Reader of register CNTAR"]
pub type R = crate::R<u32, super::CNTAR>;
#[doc = "Writer for register CNTAR"]
pub type W = crate::W<u32, super::CNTAR>;
#[doc = "Register CNTAR `reset()`'s with value 0"]
impl crate::ResetValue for super::CNTAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CNTx`"]
pub type CNTX_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CNTx`"]
pub struct CNTX_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Timerx Counter value"]
    #[inline(always)]
    pub fn cntx(&self) -> CNTX_R {
        CNTX_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Counter value"]
    #[inline(always)]
    pub fn cntx(&mut self) -> CNTX_W {
        CNTX_W { w: self }
    }
}
