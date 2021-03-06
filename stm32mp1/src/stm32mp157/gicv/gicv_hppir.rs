#[doc = "Reader of register GICV_HPPIR"]
pub type R = crate::R<u32, super::GICV_HPPIR>;
#[doc = "Reader of field `PENDINTID`"]
pub type PENDINTID_R = crate::R<u16, u16>;
#[doc = "Reader of field `CPUID`"]
pub type CPUID_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:9 - PENDINTID"]
    #[inline(always)]
    pub fn pendintid(&self) -> PENDINTID_R {
        PENDINTID_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - CPUID"]
    #[inline(always)]
    pub fn cpuid(&self) -> CPUID_R {
        CPUID_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
