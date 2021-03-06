#[doc = "Reader of register CRCSADDR"]
pub type R = crate::R<u32, super::CRCSADDR>;
#[doc = "Writer for register CRCSADDR"]
pub type W = crate::W<u32, super::CRCSADDR>;
#[doc = "Register CRCSADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::CRCSADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CRC_START_ADDR`"]
pub type CRC_START_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CRC_START_ADDR`"]
pub struct CRC_START_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_START_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 2)) | (((value as u32) & 0x0003_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:19 - CRC start address on bank 1"]
    #[inline(always)]
    pub fn crc_start_addr(&self) -> CRC_START_ADDR_R {
        CRC_START_ADDR_R::new(((self.bits >> 2) & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:19 - CRC start address on bank 1"]
    #[inline(always)]
    pub fn crc_start_addr(&mut self) -> CRC_START_ADDR_W {
        CRC_START_ADDR_W { w: self }
    }
}
