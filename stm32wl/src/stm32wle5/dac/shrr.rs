#[doc = "Reader of register SHRR"]
pub type R = crate::R<u32, super::SHRR>;
#[doc = "Writer for register SHRR"]
pub type W = crate::W<u32, super::SHRR>;
#[doc = "Register SHRR `reset()`'s with value 0x0001_0001"]
impl crate::ResetValue for super::SHRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_0001
    }
}
#[doc = "Reader of field `TREFRESH1`"]
pub type TREFRESH1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TREFRESH1`"]
pub struct TREFRESH1_W<'a> {
    w: &'a mut W,
}
impl<'a> TREFRESH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - DAC Channel 1 refresh Time (only valid in Sample and Hold mode)"]
    #[inline(always)]
    pub fn trefresh1(&self) -> TREFRESH1_R {
        TREFRESH1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DAC Channel 1 refresh Time (only valid in Sample and Hold mode)"]
    #[inline(always)]
    pub fn trefresh1(&mut self) -> TREFRESH1_W {
        TREFRESH1_W { w: self }
    }
}
