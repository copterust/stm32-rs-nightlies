#[doc = "Writer for register TIMCICR"]
pub type W = crate::W<u32, super::TIMCICR>;
#[doc = "Register TIMCICR `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMCICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Delayed Protection Flag Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLYPRTC_AW {
    #[doc = "1: Clears associated flag in ISR register"]
    CLEAR = 1,
}
impl From<DLYPRTC_AW> for bool {
    #[inline(always)]
    fn from(variant: DLYPRTC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DLYPRTC`"]
pub struct DLYPRTC_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYPRTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DLYPRTC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears associated flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DLYPRTC_AW::CLEAR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reset Interrupt flag Clear"]
pub type RSTC_AW = DLYPRTC_AW;
#[doc = "Write proxy for field `RSTC`"]
pub struct RSTC_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears associated flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DLYPRTC_AW::CLEAR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Output 2 Reset flag Clear"]
pub type RSTX2C_AW = DLYPRTC_AW;
#[doc = "Write proxy for field `RSTx2C`"]
pub struct RSTX2C_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTX2C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTX2C_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears associated flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DLYPRTC_AW::CLEAR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Output 2 Set flag Clear"]
pub type SET2XC_AW = DLYPRTC_AW;
#[doc = "Write proxy for field `SET2xC`"]
pub struct SET2XC_W<'a> {
    w: &'a mut W,
}
impl<'a> SET2XC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SET2XC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears associated flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DLYPRTC_AW::CLEAR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Output 1 Reset flag Clear"]
pub type RSTX1C_AW = DLYPRTC_AW;
#[doc = "Write proxy for field `RSTx1C`"]
pub struct RSTX1C_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTX1C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTX1C_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears associated flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DLYPRTC_AW::CLEAR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Output 1 Set flag Clear"]
pub type SET1XC_AW = DLYPRTC_AW;
#[doc = "Write proxy for field `SET1xC`"]
pub struct SET1XC_W<'a> {
    w: &'a mut W,
}
impl<'a> SET1XC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SET1XC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears associated flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DLYPRTC_AW::CLEAR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Capture2 Interrupt flag Clear"]
pub type CPT2C_AW = DLYPRTC_AW;
#[doc = "Write proxy for field `CPT2C`"]
pub struct CPT2C_W<'a> {
    w: &'a mut W,
}
impl<'a> CPT2C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPT2C_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears associated flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DLYPRTC_AW::CLEAR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Capture1 Interrupt flag Clear"]
pub type CPT1C_AW = DLYPRTC_AW;
#[doc = "Write proxy for field `CPT1C`"]
pub struct CPT1C_W<'a> {
    w: &'a mut W,
}
impl<'a> CPT1C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPT1C_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears associated flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DLYPRTC_AW::CLEAR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Update Interrupt flag Clear"]
pub type UPDC_AW = DLYPRTC_AW;
#[doc = "Write proxy for field `UPDC`"]
pub struct UPDC_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UPDC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears associated flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DLYPRTC_AW::CLEAR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Repetition Interrupt flag Clear"]
pub type REPC_AW = DLYPRTC_AW;
#[doc = "Write proxy for field `REPC`"]
pub struct REPC_W<'a> {
    w: &'a mut W,
}
impl<'a> REPC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REPC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears associated flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DLYPRTC_AW::CLEAR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Compare 4 Interrupt flag Clear"]
pub type CMP4C_AW = DLYPRTC_AW;
#[doc = "Write proxy for field `CMP4C`"]
pub struct CMP4C_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP4C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP4C_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears associated flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DLYPRTC_AW::CLEAR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Compare 3 Interrupt flag Clear"]
pub type CMP3C_AW = DLYPRTC_AW;
#[doc = "Write proxy for field `CMP3C`"]
pub struct CMP3C_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP3C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP3C_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears associated flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DLYPRTC_AW::CLEAR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Compare 2 Interrupt flag Clear"]
pub type CMP2C_AW = DLYPRTC_AW;
#[doc = "Write proxy for field `CMP2C`"]
pub struct CMP2C_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP2C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP2C_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears associated flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DLYPRTC_AW::CLEAR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Compare 1 Interrupt flag Clear"]
pub type CMP1C_AW = DLYPRTC_AW;
#[doc = "Write proxy for field `CMP1C`"]
pub struct CMP1C_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP1C_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears associated flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DLYPRTC_AW::CLEAR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl W {
    #[doc = "Bit 14 - Delayed Protection Flag Clear"]
    #[inline(always)]
    pub fn dlyprtc(&mut self) -> DLYPRTC_W {
        DLYPRTC_W { w: self }
    }
    #[doc = "Bit 13 - Reset Interrupt flag Clear"]
    #[inline(always)]
    pub fn rstc(&mut self) -> RSTC_W {
        RSTC_W { w: self }
    }
    #[doc = "Bit 12 - Output 2 Reset flag Clear"]
    #[inline(always)]
    pub fn rstx2c(&mut self) -> RSTX2C_W {
        RSTX2C_W { w: self }
    }
    #[doc = "Bit 11 - Output 2 Set flag Clear"]
    #[inline(always)]
    pub fn set2x_c(&mut self) -> SET2XC_W {
        SET2XC_W { w: self }
    }
    #[doc = "Bit 10 - Output 1 Reset flag Clear"]
    #[inline(always)]
    pub fn rstx1c(&mut self) -> RSTX1C_W {
        RSTX1C_W { w: self }
    }
    #[doc = "Bit 9 - Output 1 Set flag Clear"]
    #[inline(always)]
    pub fn set1x_c(&mut self) -> SET1XC_W {
        SET1XC_W { w: self }
    }
    #[doc = "Bit 8 - Capture2 Interrupt flag Clear"]
    #[inline(always)]
    pub fn cpt2c(&mut self) -> CPT2C_W {
        CPT2C_W { w: self }
    }
    #[doc = "Bit 7 - Capture1 Interrupt flag Clear"]
    #[inline(always)]
    pub fn cpt1c(&mut self) -> CPT1C_W {
        CPT1C_W { w: self }
    }
    #[doc = "Bit 6 - Update Interrupt flag Clear"]
    #[inline(always)]
    pub fn updc(&mut self) -> UPDC_W {
        UPDC_W { w: self }
    }
    #[doc = "Bit 4 - Repetition Interrupt flag Clear"]
    #[inline(always)]
    pub fn repc(&mut self) -> REPC_W {
        REPC_W { w: self }
    }
    #[doc = "Bit 3 - Compare 4 Interrupt flag Clear"]
    #[inline(always)]
    pub fn cmp4c(&mut self) -> CMP4C_W {
        CMP4C_W { w: self }
    }
    #[doc = "Bit 2 - Compare 3 Interrupt flag Clear"]
    #[inline(always)]
    pub fn cmp3c(&mut self) -> CMP3C_W {
        CMP3C_W { w: self }
    }
    #[doc = "Bit 1 - Compare 2 Interrupt flag Clear"]
    #[inline(always)]
    pub fn cmp2c(&mut self) -> CMP2C_W {
        CMP2C_W { w: self }
    }
    #[doc = "Bit 0 - Compare 1 Interrupt flag Clear"]
    #[inline(always)]
    pub fn cmp1c(&mut self) -> CMP1C_W {
        CMP1C_W { w: self }
    }
}
