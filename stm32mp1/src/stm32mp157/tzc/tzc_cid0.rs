#[doc = "Reader of register TZC_CID0"]
pub type R = crate::R<u32, super::TZC_CID0>;
#[doc = "Reader of field `COMP_ID_0`"]
pub type COMP_ID_0_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - COMP_ID_0"]
    #[inline(always)]
    pub fn comp_id_0(&self) -> COMP_ID_0_R {
        COMP_ID_0_R::new((self.bits & 0xff) as u8)
    }
}
