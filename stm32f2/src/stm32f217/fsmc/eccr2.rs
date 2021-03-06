#[doc = "Reader of register ECCR2"]
pub type R = crate::R<u32, super::ECCR2>;
#[doc = "Reader of field `ECC`"]
pub type ECC_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ECC result"]
    #[inline(always)]
    pub fn ecc(&self) -> ECC_R {
        ECC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
