#[doc = "Reader of register DFSDM2_JDATAR"]
pub type R = crate::R<u32, super::DFSDM2_JDATAR>;
#[doc = "Reader of field `JDATACH`"]
pub type JDATACH_R = crate::R<u8, u8>;
#[doc = "Reader of field `JDATA`"]
pub type JDATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:2 - Injected channel most recently converted"]
    #[inline(always)]
    pub fn jdatach(&self) -> JDATACH_R {
        JDATACH_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:31 - Injected group conversion data"]
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
