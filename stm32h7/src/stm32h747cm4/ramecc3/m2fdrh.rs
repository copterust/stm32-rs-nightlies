#[doc = "Reader of register M2FDRH"]
pub type R = crate::R<u32, super::M2FDRH>;
#[doc = "Writer for register M2FDRH"]
pub type W = crate::W<u32, super::M2FDRH>;
#[doc = "Register M2FDRH `reset()`'s with value 0"]
impl crate::ResetValue for super::M2FDRH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FDATAH`"]
pub type FDATAH_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ECC failing data high"]
    #[inline(always)]
    pub fn fdatah(&self) -> FDATAH_R {
        FDATAH_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {}
