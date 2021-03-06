#[doc = "Reader of register HLCR"]
pub type R = crate::R<u32, super::HLCR>;
#[doc = "Writer for register HLCR"]
pub type W = crate::W<u32, super::HLCR>;
#[doc = "Register HLCR `reset()`'s with value 0"]
impl crate::ResetValue for super::HLCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ALTERNATE`"]
pub type ALTERNATE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ALTERNATE`"]
pub struct ALTERNATE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALTERNATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Alternate bytes"]
    #[inline(always)]
    pub fn alternate(&self) -> ALTERNATE_R {
        ALTERNATE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Alternate bytes"]
    #[inline(always)]
    pub fn alternate(&mut self) -> ALTERNATE_W {
        ALTERNATE_W { w: self }
    }
}
