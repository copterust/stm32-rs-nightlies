#[doc = "Reader of register CSR50"]
pub type R = crate::R<u32, super::CSR50>;
#[doc = "Writer for register CSR50"]
pub type W = crate::W<u32, super::CSR50>;
#[doc = "Register CSR50 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSR50 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSR50`"]
pub type CSR50_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSR50`"]
pub struct CSR50_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR50_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR50"]
    #[inline(always)]
    pub fn csr50(&self) -> CSR50_R {
        CSR50_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR50"]
    #[inline(always)]
    pub fn csr50(&mut self) -> CSR50_W {
        CSR50_W { w: self }
    }
}
