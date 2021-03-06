#[doc = "Reader of register GICD_ISPENDR0"]
pub type R = crate::R<u32, super::GICD_ISPENDR0>;
#[doc = "Writer for register GICD_ISPENDR0"]
pub type W = crate::W<u32, super::GICD_ISPENDR0>;
#[doc = "Register GICD_ISPENDR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::GICD_ISPENDR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ISPENDR0`"]
pub type ISPENDR0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ISPENDR0`"]
pub struct ISPENDR0_W<'a> {
    w: &'a mut W,
}
impl<'a> ISPENDR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ISPENDR0"]
    #[inline(always)]
    pub fn ispendr0(&self) -> ISPENDR0_R {
        ISPENDR0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISPENDR0"]
    #[inline(always)]
    pub fn ispendr0(&mut self) -> ISPENDR0_W {
        ISPENDR0_W { w: self }
    }
}
