#[doc = "Reader of register GPIOF_OSPEEDR"]
pub type R = crate::R<u32, super::GPIOF_OSPEEDR>;
#[doc = "Writer for register GPIOF_OSPEEDR"]
pub type W = crate::W<u32, super::GPIOF_OSPEEDR>;
#[doc = "Register GPIOF_OSPEEDR `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIOF_OSPEEDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OSPEEDR0`"]
pub type OSPEEDR0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSPEEDR0`"]
pub struct OSPEEDR0_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `OSPEEDR1`"]
pub type OSPEEDR1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSPEEDR1`"]
pub struct OSPEEDR1_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `OSPEEDR2`"]
pub type OSPEEDR2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSPEEDR2`"]
pub struct OSPEEDR2_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `OSPEEDR3`"]
pub type OSPEEDR3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSPEEDR3`"]
pub struct OSPEEDR3_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `OSPEEDR4`"]
pub type OSPEEDR4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSPEEDR4`"]
pub struct OSPEEDR4_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `OSPEEDR5`"]
pub type OSPEEDR5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSPEEDR5`"]
pub struct OSPEEDR5_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `OSPEEDR6`"]
pub type OSPEEDR6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSPEEDR6`"]
pub struct OSPEEDR6_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `OSPEEDR7`"]
pub type OSPEEDR7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSPEEDR7`"]
pub struct OSPEEDR7_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `OSPEEDR8`"]
pub type OSPEEDR8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSPEEDR8`"]
pub struct OSPEEDR8_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `OSPEEDR9`"]
pub type OSPEEDR9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSPEEDR9`"]
pub struct OSPEEDR9_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `OSPEEDR10`"]
pub type OSPEEDR10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSPEEDR10`"]
pub struct OSPEEDR10_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `OSPEEDR11`"]
pub type OSPEEDR11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSPEEDR11`"]
pub struct OSPEEDR11_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `OSPEEDR12`"]
pub type OSPEEDR12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSPEEDR12`"]
pub struct OSPEEDR12_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `OSPEEDR13`"]
pub type OSPEEDR13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSPEEDR13`"]
pub struct OSPEEDR13_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `OSPEEDR14`"]
pub type OSPEEDR14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSPEEDR14`"]
pub struct OSPEEDR14_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `OSPEEDR15`"]
pub type OSPEEDR15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSPEEDR15`"]
pub struct OSPEEDR15_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - OSPEEDR0"]
    #[inline(always)]
    pub fn ospeedr0(&self) -> OSPEEDR0_R {
        OSPEEDR0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - OSPEEDR1"]
    #[inline(always)]
    pub fn ospeedr1(&self) -> OSPEEDR1_R {
        OSPEEDR1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - OSPEEDR2"]
    #[inline(always)]
    pub fn ospeedr2(&self) -> OSPEEDR2_R {
        OSPEEDR2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - OSPEEDR3"]
    #[inline(always)]
    pub fn ospeedr3(&self) -> OSPEEDR3_R {
        OSPEEDR3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - OSPEEDR4"]
    #[inline(always)]
    pub fn ospeedr4(&self) -> OSPEEDR4_R {
        OSPEEDR4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - OSPEEDR5"]
    #[inline(always)]
    pub fn ospeedr5(&self) -> OSPEEDR5_R {
        OSPEEDR5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - OSPEEDR6"]
    #[inline(always)]
    pub fn ospeedr6(&self) -> OSPEEDR6_R {
        OSPEEDR6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - OSPEEDR7"]
    #[inline(always)]
    pub fn ospeedr7(&self) -> OSPEEDR7_R {
        OSPEEDR7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - OSPEEDR8"]
    #[inline(always)]
    pub fn ospeedr8(&self) -> OSPEEDR8_R {
        OSPEEDR8_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - OSPEEDR9"]
    #[inline(always)]
    pub fn ospeedr9(&self) -> OSPEEDR9_R {
        OSPEEDR9_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - OSPEEDR10"]
    #[inline(always)]
    pub fn ospeedr10(&self) -> OSPEEDR10_R {
        OSPEEDR10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - OSPEEDR11"]
    #[inline(always)]
    pub fn ospeedr11(&self) -> OSPEEDR11_R {
        OSPEEDR11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - OSPEEDR12"]
    #[inline(always)]
    pub fn ospeedr12(&self) -> OSPEEDR12_R {
        OSPEEDR12_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - OSPEEDR13"]
    #[inline(always)]
    pub fn ospeedr13(&self) -> OSPEEDR13_R {
        OSPEEDR13_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - OSPEEDR14"]
    #[inline(always)]
    pub fn ospeedr14(&self) -> OSPEEDR14_R {
        OSPEEDR14_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - OSPEEDR15"]
    #[inline(always)]
    pub fn ospeedr15(&self) -> OSPEEDR15_R {
        OSPEEDR15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - OSPEEDR0"]
    #[inline(always)]
    pub fn ospeedr0(&mut self) -> OSPEEDR0_W {
        OSPEEDR0_W { w: self }
    }
    #[doc = "Bits 2:3 - OSPEEDR1"]
    #[inline(always)]
    pub fn ospeedr1(&mut self) -> OSPEEDR1_W {
        OSPEEDR1_W { w: self }
    }
    #[doc = "Bits 4:5 - OSPEEDR2"]
    #[inline(always)]
    pub fn ospeedr2(&mut self) -> OSPEEDR2_W {
        OSPEEDR2_W { w: self }
    }
    #[doc = "Bits 6:7 - OSPEEDR3"]
    #[inline(always)]
    pub fn ospeedr3(&mut self) -> OSPEEDR3_W {
        OSPEEDR3_W { w: self }
    }
    #[doc = "Bits 8:9 - OSPEEDR4"]
    #[inline(always)]
    pub fn ospeedr4(&mut self) -> OSPEEDR4_W {
        OSPEEDR4_W { w: self }
    }
    #[doc = "Bits 10:11 - OSPEEDR5"]
    #[inline(always)]
    pub fn ospeedr5(&mut self) -> OSPEEDR5_W {
        OSPEEDR5_W { w: self }
    }
    #[doc = "Bits 12:13 - OSPEEDR6"]
    #[inline(always)]
    pub fn ospeedr6(&mut self) -> OSPEEDR6_W {
        OSPEEDR6_W { w: self }
    }
    #[doc = "Bits 14:15 - OSPEEDR7"]
    #[inline(always)]
    pub fn ospeedr7(&mut self) -> OSPEEDR7_W {
        OSPEEDR7_W { w: self }
    }
    #[doc = "Bits 16:17 - OSPEEDR8"]
    #[inline(always)]
    pub fn ospeedr8(&mut self) -> OSPEEDR8_W {
        OSPEEDR8_W { w: self }
    }
    #[doc = "Bits 18:19 - OSPEEDR9"]
    #[inline(always)]
    pub fn ospeedr9(&mut self) -> OSPEEDR9_W {
        OSPEEDR9_W { w: self }
    }
    #[doc = "Bits 20:21 - OSPEEDR10"]
    #[inline(always)]
    pub fn ospeedr10(&mut self) -> OSPEEDR10_W {
        OSPEEDR10_W { w: self }
    }
    #[doc = "Bits 22:23 - OSPEEDR11"]
    #[inline(always)]
    pub fn ospeedr11(&mut self) -> OSPEEDR11_W {
        OSPEEDR11_W { w: self }
    }
    #[doc = "Bits 24:25 - OSPEEDR12"]
    #[inline(always)]
    pub fn ospeedr12(&mut self) -> OSPEEDR12_W {
        OSPEEDR12_W { w: self }
    }
    #[doc = "Bits 26:27 - OSPEEDR13"]
    #[inline(always)]
    pub fn ospeedr13(&mut self) -> OSPEEDR13_W {
        OSPEEDR13_W { w: self }
    }
    #[doc = "Bits 28:29 - OSPEEDR14"]
    #[inline(always)]
    pub fn ospeedr14(&mut self) -> OSPEEDR14_W {
        OSPEEDR14_W { w: self }
    }
    #[doc = "Bits 30:31 - OSPEEDR15"]
    #[inline(always)]
    pub fn ospeedr15(&mut self) -> OSPEEDR15_W {
        OSPEEDR15_W { w: self }
    }
}
