#[doc = "Reader of register EMR1"]
pub type R = crate::R<u32, super::EMR1>;
#[doc = "Writer for register EMR1"]
pub type W = crate::W<u32, super::EMR1>;
#[doc = "Register EMR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::EMR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EM`"]
pub type EM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `EM`"]
pub struct EM_W<'a> {
    w: &'a mut W,
}
impl<'a> EM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `EM17`"]
pub type EM17_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EM17`"]
pub struct EM17_W<'a> {
    w: &'a mut W,
}
impl<'a> EM17_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 17)) | (((value as u32) & 0x3f) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CPU(m) Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em(&self) -> EM_R {
        EM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 17:22 - CPU(m) Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em17(&self) -> EM17_R {
        EM17_R::new(((self.bits >> 17) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - CPU(m) Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em(&mut self) -> EM_W {
        EM_W { w: self }
    }
    #[doc = "Bits 17:22 - CPU(m) Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em17(&mut self) -> EM17_W {
        EM17_W { w: self }
    }
}
