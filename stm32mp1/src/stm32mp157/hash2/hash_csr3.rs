#[doc = "Reader of register HASH_CSR3"]
pub type R = crate::R<u32, super::HASH_CSR3>;
#[doc = "Writer for register HASH_CSR3"]
pub type W = crate::W<u32, super::HASH_CSR3>;
#[doc = "Register HASH_CSR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS3`"]
pub type CS3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS3`"]
pub struct CS3_W<'a> {
    w: &'a mut W,
}
impl<'a> CS3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS3"]
    #[inline(always)]
    pub fn cs3(&self) -> CS3_R {
        CS3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS3"]
    #[inline(always)]
    pub fn cs3(&mut self) -> CS3_W {
        CS3_W { w: self }
    }
}
