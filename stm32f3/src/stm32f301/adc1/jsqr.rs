#[doc = "Reader of register JSQR"]
pub type R = crate::R<u32, super::JSQR>;
#[doc = "Writer for register JSQR"]
pub type W = crate::W<u32, super::JSQR>;
#[doc = "Register JSQR `reset()`'s with value 0"]
impl crate::ResetValue for super::JSQR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `JSQ4`"]
pub type JSQ4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `JSQ4`"]
pub struct JSQ4_W<'a> {
    w: &'a mut W,
}
impl<'a> JSQ4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 26)) | (((value as u32) & 0x1f) << 26);
        self.w
    }
}
#[doc = "Reader of field `JSQ3`"]
pub type JSQ3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `JSQ3`"]
pub struct JSQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> JSQ3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | (((value as u32) & 0x1f) << 20);
        self.w
    }
}
#[doc = "Reader of field `JSQ2`"]
pub type JSQ2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `JSQ2`"]
pub struct JSQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> JSQ2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 14)) | (((value as u32) & 0x1f) << 14);
        self.w
    }
}
#[doc = "Reader of field `JSQ1`"]
pub type JSQ1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `JSQ1`"]
pub struct JSQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> JSQ1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "External Trigger Enable and Polarity Selection for injected channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum JEXTEN_A {
    #[doc = "0: Trigger detection disabled"]
    DISABLED = 0,
    #[doc = "1: Trigger detection on the rising edge"]
    RISINGEDGE = 1,
    #[doc = "2: Trigger detection on the falling edge"]
    FALLINGEDGE = 2,
    #[doc = "3: Trigger detection on both the rising and falling edges"]
    BOTHEDGES = 3,
}
impl From<JEXTEN_A> for u8 {
    #[inline(always)]
    fn from(variant: JEXTEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `JEXTEN`"]
pub type JEXTEN_R = crate::R<u8, JEXTEN_A>;
impl JEXTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JEXTEN_A {
        match self.bits {
            0 => JEXTEN_A::DISABLED,
            1 => JEXTEN_A::RISINGEDGE,
            2 => JEXTEN_A::FALLINGEDGE,
            3 => JEXTEN_A::BOTHEDGES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JEXTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `RISINGEDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == JEXTEN_A::RISINGEDGE
    }
    #[doc = "Checks if the value of the field is `FALLINGEDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == JEXTEN_A::FALLINGEDGE
    }
    #[doc = "Checks if the value of the field is `BOTHEDGES`"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == JEXTEN_A::BOTHEDGES
    }
}
#[doc = "Write proxy for field `JEXTEN`"]
pub struct JEXTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> JEXTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JEXTEN_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Trigger detection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JEXTEN_A::DISABLED)
    }
    #[doc = "Trigger detection on the rising edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(JEXTEN_A::RISINGEDGE)
    }
    #[doc = "Trigger detection on the falling edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(JEXTEN_A::FALLINGEDGE)
    }
    #[doc = "Trigger detection on both the rising and falling edges"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(JEXTEN_A::BOTHEDGES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "External Trigger Selection for injected group\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum JEXTSEL_A {
    #[doc = "9: HRTIM_ADCTRG2 event"]
    HRTIM_ADCTRG2 = 9,
    #[doc = "10: HRTIM_ADCTRG4 event"]
    HRTIM_ADCTRG4 = 10,
}
impl From<JEXTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: JEXTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `JEXTSEL`"]
pub type JEXTSEL_R = crate::R<u8, JEXTSEL_A>;
impl JEXTSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, JEXTSEL_A> {
        use crate::Variant::*;
        match self.bits {
            9 => Val(JEXTSEL_A::HRTIM_ADCTRG2),
            10 => Val(JEXTSEL_A::HRTIM_ADCTRG4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `HRTIM_ADCTRG2`"]
    #[inline(always)]
    pub fn is_hrtim_adctrg2(&self) -> bool {
        *self == JEXTSEL_A::HRTIM_ADCTRG2
    }
    #[doc = "Checks if the value of the field is `HRTIM_ADCTRG4`"]
    #[inline(always)]
    pub fn is_hrtim_adctrg4(&self) -> bool {
        *self == JEXTSEL_A::HRTIM_ADCTRG4
    }
}
#[doc = "Write proxy for field `JEXTSEL`"]
pub struct JEXTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> JEXTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JEXTSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "HRTIM_ADCTRG2 event"]
    #[inline(always)]
    pub fn hrtim_adctrg2(self) -> &'a mut W {
        self.variant(JEXTSEL_A::HRTIM_ADCTRG2)
    }
    #[doc = "HRTIM_ADCTRG4 event"]
    #[inline(always)]
    pub fn hrtim_adctrg4(self) -> &'a mut W {
        self.variant(JEXTSEL_A::HRTIM_ADCTRG4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
        self.w
    }
}
#[doc = "Reader of field `JL`"]
pub type JL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `JL`"]
pub struct JL_W<'a> {
    w: &'a mut W,
}
impl<'a> JL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 26:30 - 4th conversion in the injected sequence"]
    #[inline(always)]
    pub fn jsq4(&self) -> JSQ4_R {
        JSQ4_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - 3rd conversion in the injected sequence"]
    #[inline(always)]
    pub fn jsq3(&self) -> JSQ3_R {
        JSQ3_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 14:18 - 2nd conversion in the injected sequence"]
    #[inline(always)]
    pub fn jsq2(&self) -> JSQ2_R {
        JSQ2_R::new(((self.bits >> 14) & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 1st conversion in the injected sequence"]
    #[inline(always)]
    pub fn jsq1(&self) -> JSQ1_R {
        JSQ1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - External Trigger Enable and Polarity Selection for injected channels"]
    #[inline(always)]
    pub fn jexten(&self) -> JEXTEN_R {
        JEXTEN_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 2:5 - External Trigger Selection for injected group"]
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 0:1 - Injected channel sequence length"]
    #[inline(always)]
    pub fn jl(&self) -> JL_R {
        JL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 26:30 - 4th conversion in the injected sequence"]
    #[inline(always)]
    pub fn jsq4(&mut self) -> JSQ4_W {
        JSQ4_W { w: self }
    }
    #[doc = "Bits 20:24 - 3rd conversion in the injected sequence"]
    #[inline(always)]
    pub fn jsq3(&mut self) -> JSQ3_W {
        JSQ3_W { w: self }
    }
    #[doc = "Bits 14:18 - 2nd conversion in the injected sequence"]
    #[inline(always)]
    pub fn jsq2(&mut self) -> JSQ2_W {
        JSQ2_W { w: self }
    }
    #[doc = "Bits 8:12 - 1st conversion in the injected sequence"]
    #[inline(always)]
    pub fn jsq1(&mut self) -> JSQ1_W {
        JSQ1_W { w: self }
    }
    #[doc = "Bits 6:7 - External Trigger Enable and Polarity Selection for injected channels"]
    #[inline(always)]
    pub fn jexten(&mut self) -> JEXTEN_W {
        JEXTEN_W { w: self }
    }
    #[doc = "Bits 2:5 - External Trigger Selection for injected group"]
    #[inline(always)]
    pub fn jextsel(&mut self) -> JEXTSEL_W {
        JEXTSEL_W { w: self }
    }
    #[doc = "Bits 0:1 - Injected channel sequence length"]
    #[inline(always)]
    pub fn jl(&mut self) -> JL_W {
        JL_W { w: self }
    }
}