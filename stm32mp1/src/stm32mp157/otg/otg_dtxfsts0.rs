#[doc = "Reader of register OTG_DTXFSTS0"]
pub type R = crate::R<u32, super::OTG_DTXFSTS0>;
#[doc = "Reader of field `INEPTFSAV`"]
pub type INEPTFSAV_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - INEPTFSAV"]
    #[inline(always)]
    pub fn ineptfsav(&self) -> INEPTFSAV_R {
        INEPTFSAV_R::new((self.bits & 0xffff) as u16)
    }
}
