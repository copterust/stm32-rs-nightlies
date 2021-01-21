#[doc = "Reader of register FDCAN_RWD"]
pub type R = crate::R<u32, super::FDCAN_RWD>;
#[doc = "Reader of field `WDV`"]
pub type WDV_R = crate::R<u8, u8>;
#[doc = "Reader of field `WDC`"]
pub type WDC_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 8:15 - Watchdog value"]
    #[inline(always)]
    pub fn wdv(&self) -> WDV_R {
        WDV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Watchdog configuration"]
    #[inline(always)]
    pub fn wdc(&self) -> WDC_R {
        WDC_R::new((self.bits & 0xff) as u8)
    }
}