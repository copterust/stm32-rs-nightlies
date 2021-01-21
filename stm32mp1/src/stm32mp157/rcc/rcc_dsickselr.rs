#[doc = "Reader of register RCC_DSICKSELR"]
pub type R = crate::R<u32, super::RCC_DSICKSELR>;
#[doc = "Writer for register RCC_DSICKSELR"]
pub type W = crate::W<u32, super::RCC_DSICKSELR>;
#[doc = "Register RCC_DSICKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_DSICKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DSISRC`"]
pub type DSISRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSISRC`"]
pub struct DSISRC_W<'a> {
    w: &'a mut W,
}
impl<'a> DSISRC_W<'a> {
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
impl R {
    #[doc = "Bit 0 - DSISRC"]
    #[inline(always)]
    pub fn dsisrc(&self) -> DSISRC_R {
        DSISRC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DSISRC"]
    #[inline(always)]
    pub fn dsisrc(&mut self) -> DSISRC_W {
        DSISRC_W { w: self }
    }
}
