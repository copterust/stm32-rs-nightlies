#[doc = "Reader of register DCMI_RIS"]
pub type R = crate::R<u32, super::DCMI_RIS>;
#[doc = "Reader of field `FRAME_RIS`"]
pub type FRAME_RIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVR_RIS`"]
pub type OVR_RIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERR_RIS`"]
pub type ERR_RIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `VSYNC_RIS`"]
pub type VSYNC_RIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `LINE_RIS`"]
pub type LINE_RIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - FRAME_RIS"]
    #[inline(always)]
    pub fn frame_ris(&self) -> FRAME_RIS_R {
        FRAME_RIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - OVR_RIS"]
    #[inline(always)]
    pub fn ovr_ris(&self) -> OVR_RIS_R {
        OVR_RIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ERR_RIS"]
    #[inline(always)]
    pub fn err_ris(&self) -> ERR_RIS_R {
        ERR_RIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - VSYNC_RIS"]
    #[inline(always)]
    pub fn vsync_ris(&self) -> VSYNC_RIS_R {
        VSYNC_RIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LINE_RIS"]
    #[inline(always)]
    pub fn line_ris(&self) -> LINE_RIS_R {
        LINE_RIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
