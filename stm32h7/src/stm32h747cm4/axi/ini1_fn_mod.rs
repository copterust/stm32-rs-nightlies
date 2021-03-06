#[doc = "Reader of register INI1_FN_MOD"]
pub type R = crate::R<u32, super::INI1_FN_MOD>;
#[doc = "Writer for register INI1_FN_MOD"]
pub type W = crate::W<u32, super::INI1_FN_MOD>;
#[doc = "Register INI1_FN_MOD `reset()`'s with value 0x04"]
impl crate::ResetValue for super::INI1_FN_MOD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04
    }
}
#[doc = "Reader of field `READ_ISS_OVERRIDE`"]
pub type READ_ISS_OVERRIDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `READ_ISS_OVERRIDE`"]
pub struct READ_ISS_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_ISS_OVERRIDE_W<'a> {
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
#[doc = "Reader of field `WRITE_ISS_OVERRIDE`"]
pub type WRITE_ISS_OVERRIDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRITE_ISS_OVERRIDE`"]
pub struct WRITE_ISS_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE_ISS_OVERRIDE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Override ASIB read issuing capability"]
    #[inline(always)]
    pub fn read_iss_override(&self) -> READ_ISS_OVERRIDE_R {
        READ_ISS_OVERRIDE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Override ASIB write issuing capability"]
    #[inline(always)]
    pub fn write_iss_override(&self) -> WRITE_ISS_OVERRIDE_R {
        WRITE_ISS_OVERRIDE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Override ASIB read issuing capability"]
    #[inline(always)]
    pub fn read_iss_override(&mut self) -> READ_ISS_OVERRIDE_W {
        READ_ISS_OVERRIDE_W { w: self }
    }
    #[doc = "Bit 1 - Override ASIB write issuing capability"]
    #[inline(always)]
    pub fn write_iss_override(&mut self) -> WRITE_ISS_OVERRIDE_W {
        WRITE_ISS_OVERRIDE_W { w: self }
    }
}
