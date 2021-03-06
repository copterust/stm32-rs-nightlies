#[doc = "Reader of register RCC_I2C35CKSELR"]
pub type R = crate::R<u32, super::RCC_I2C35CKSELR>;
#[doc = "Writer for register RCC_I2C35CKSELR"]
pub type W = crate::W<u32, super::RCC_I2C35CKSELR>;
#[doc = "Register RCC_I2C35CKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_I2C35CKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C35SRC`"]
pub type I2C35SRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2C35SRC`"]
pub struct I2C35SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C35SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - I2C35SRC"]
    #[inline(always)]
    pub fn i2c35src(&self) -> I2C35SRC_R {
        I2C35SRC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - I2C35SRC"]
    #[inline(always)]
    pub fn i2c35src(&mut self) -> I2C35SRC_W {
        I2C35SRC_W { w: self }
    }
}
