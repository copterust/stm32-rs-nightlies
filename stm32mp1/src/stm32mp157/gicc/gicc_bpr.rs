#[doc = "Reader of register GICC_BPR"]
pub type R = crate::R<u32, super::GICC_BPR>;
#[doc = "Writer for register GICC_BPR"]
pub type W = crate::W<u32, super::GICC_BPR>;
#[doc = "Register GICC_BPR `reset()`'s with value 0x02"]
impl crate::ResetValue for super::GICC_BPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Reader of field `BINARY_POINT`"]
pub type BINARY_POINT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BINARY_POINT`"]
pub struct BINARY_POINT_W<'a> {
    w: &'a mut W,
}
impl<'a> BINARY_POINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - BINARY_POINT"]
    #[inline(always)]
    pub fn binary_point(&self) -> BINARY_POINT_R {
        BINARY_POINT_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - BINARY_POINT"]
    #[inline(always)]
    pub fn binary_point(&mut self) -> BINARY_POINT_W {
        BINARY_POINT_W { w: self }
    }
}
