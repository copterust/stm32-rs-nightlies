#[doc = "Reader of register HASH_CSR13"]
pub type R = crate::R<u32, super::HASH_CSR13>;
#[doc = "Writer for register HASH_CSR13"]
pub type W = crate::W<u32, super::HASH_CSR13>;
#[doc = "Register HASH_CSR13 `reset()`'s with value 0"]
impl crate::ResetValue for super::HASH_CSR13 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CS13`"]
pub type CS13_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CS13`"]
pub struct CS13_W<'a> {
    w: &'a mut W,
}
impl<'a> CS13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS13"]
    #[inline(always)]
    pub fn cs13(&self) -> CS13_R {
        CS13_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS13"]
    #[inline(always)]
    pub fn cs13(&mut self) -> CS13_W {
        CS13_W { w: self }
    }
}
