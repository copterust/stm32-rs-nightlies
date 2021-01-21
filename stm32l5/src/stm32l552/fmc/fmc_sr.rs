#[doc = "Reader of register FMC_SR"]
pub type R = crate::R<u32, super::FMC_SR>;
#[doc = "Writer for register FMC_SR"]
pub type W = crate::W<u32, super::FMC_SR>;
#[doc = "Register FMC_SR `reset()`'s with value 0x40"]
impl crate::ResetValue for super::FMC_SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x40
    }
}
#[doc = "Reader of field `IRS`"]
pub type IRS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRS`"]
pub struct IRS_W<'a> {
    w: &'a mut W,
}
impl<'a> IRS_W<'a> {
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
#[doc = "Reader of field `ILS`"]
pub type ILS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ILS`"]
pub struct ILS_W<'a> {
    w: &'a mut W,
}
impl<'a> ILS_W<'a> {
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
#[doc = "Reader of field `IFS`"]
pub type IFS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IFS`"]
pub struct IFS_W<'a> {
    w: &'a mut W,
}
impl<'a> IFS_W<'a> {
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
#[doc = "Reader of field `IREN`"]
pub type IREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IREN`"]
pub struct IREN_W<'a> {
    w: &'a mut W,
}
impl<'a> IREN_W<'a> {
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
#[doc = "Reader of field `ILEN`"]
pub type ILEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ILEN`"]
pub struct ILEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ILEN_W<'a> {
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
#[doc = "Reader of field `IFEN`"]
pub type IFEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IFEN`"]
pub struct IFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IFEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `FEMPT`"]
pub type FEMPT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Interrupt rising edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it will be set."]
    #[inline(always)]
    pub fn irs(&self) -> IRS_R {
        IRS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt high-level status The flag is set by hardware and reset by software."]
    #[inline(always)]
    pub fn ils(&self) -> ILS_R {
        ILS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt falling edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it will be set."]
    #[inline(always)]
    pub fn ifs(&self) -> IFS_R {
        IFS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt rising edge detection enable bit"]
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt high-level detection enable bit"]
    #[inline(always)]
    pub fn ilen(&self) -> ILEN_R {
        ILEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt falling edge detection enable bit"]
    #[inline(always)]
    pub fn ifen(&self) -> IFEN_R {
        IFEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - FIFO empty. Read-only bit that provides the status of the FIFO"]
    #[inline(always)]
    pub fn fempt(&self) -> FEMPT_R {
        FEMPT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt rising edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it will be set."]
    #[inline(always)]
    pub fn irs(&mut self) -> IRS_W {
        IRS_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt high-level status The flag is set by hardware and reset by software."]
    #[inline(always)]
    pub fn ils(&mut self) -> ILS_W {
        ILS_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt falling edge status The flag is set by hardware and reset by software. Note: If this bit is written by software to 1 it will be set."]
    #[inline(always)]
    pub fn ifs(&mut self) -> IFS_W {
        IFS_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt rising edge detection enable bit"]
    #[inline(always)]
    pub fn iren(&mut self) -> IREN_W {
        IREN_W { w: self }
    }
    #[doc = "Bit 4 - Interrupt high-level detection enable bit"]
    #[inline(always)]
    pub fn ilen(&mut self) -> ILEN_W {
        ILEN_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt falling edge detection enable bit"]
    #[inline(always)]
    pub fn ifen(&mut self) -> IFEN_W {
        IFEN_W { w: self }
    }
}
