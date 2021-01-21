#[doc = "Reader of register R2ENDADDR"]
pub type R = crate::R<u32, super::R2ENDADDR>;
#[doc = "Writer for register R2ENDADDR"]
pub type W = crate::W<u32, super::R2ENDADDR>;
#[doc = "Register R2ENDADDR `reset()`'s with value 0x0fff"]
impl crate::ResetValue for super::R2ENDADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0fff
    }
}
#[doc = "Reader of field `REGx_END_ADDR`"]
pub type REGX_END_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `REGx_END_ADDR`"]
pub struct REGX_END_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> REGX_END_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Region AXI end address"]
    #[inline(always)]
    pub fn regx_end_addr(&self) -> REGX_END_ADDR_R {
        REGX_END_ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region AXI end address"]
    #[inline(always)]
    pub fn regx_end_addr(&mut self) -> REGX_END_ADDR_W {
        REGX_END_ADDR_W { w: self }
    }
}
