#[doc = "Reader of register OR"]
pub type R = crate::R<u32, super::OR>;
#[doc = "Writer for register OR"]
pub type W = crate::W<u32, super::OR>;
#[doc = "Register OR `reset()`'s with value 0"]
impl crate::ResetValue for super::OR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_ALARM_TYPE`"]
pub type RTC_ALARM_TYPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_ALARM_TYPE`"]
pub struct RTC_ALARM_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_ALARM_TYPE_W<'a> {
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
#[doc = "Reader of field `TSINSEL`"]
pub type TSINSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSINSEL`"]
pub struct TSINSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TSINSEL_W<'a> {
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
    #[doc = "Bit 3 - RTC_ALARM on PC13 output type"]
    #[inline(always)]
    pub fn rtc_alarm_type(&self) -> RTC_ALARM_TYPE_R {
        RTC_ALARM_TYPE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 1 - TIMESTAMP mapping"]
    #[inline(always)]
    pub fn tsinsel(&self) -> TSINSEL_R {
        TSINSEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - RTC_ALARM on PC13 output type"]
    #[inline(always)]
    pub fn rtc_alarm_type(&mut self) -> RTC_ALARM_TYPE_W {
        RTC_ALARM_TYPE_W { w: self }
    }
    #[doc = "Bit 1 - TIMESTAMP mapping"]
    #[inline(always)]
    pub fn tsinsel(&mut self) -> TSINSEL_W {
        TSINSEL_W { w: self }
    }
}
