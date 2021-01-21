#[doc = "Reader of register EXTI_HWCFGR2"]
pub type R = crate::R<u32, super::EXTI_HWCFGR2>;
#[doc = "Reader of field `EVENT_TRG`"]
pub type EVENT_TRG_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - EVENT_TRG"]
    #[inline(always)]
    pub fn event_trg(&self) -> EVENT_TRG_R {
        EVENT_TRG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}