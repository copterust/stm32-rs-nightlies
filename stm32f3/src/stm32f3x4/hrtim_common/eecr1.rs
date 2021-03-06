#[doc = "Reader of register EECR1"]
pub type R = crate::R<u32, super::EECR1>;
#[doc = "Writer for register EECR1"]
pub type W = crate::W<u32, super::EECR1>;
#[doc = "Register EECR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::EECR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "External Event 5 Fast mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EE5FAST_A {
    #[doc = "0: External event is re-synchronised by the HRTIM logic before acting on outputs"]
    RESYNCHRONIZED = 0,
    #[doc = "1: External event is acting asynchronously on outputs (low-latency mode)"]
    ASYNCHRONOUS = 1,
}
impl From<EE5FAST_A> for bool {
    #[inline(always)]
    fn from(variant: EE5FAST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EE5FAST`"]
pub type EE5FAST_R = crate::R<bool, EE5FAST_A>;
impl EE5FAST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EE5FAST_A {
        match self.bits {
            false => EE5FAST_A::RESYNCHRONIZED,
            true => EE5FAST_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `RESYNCHRONIZED`"]
    #[inline(always)]
    pub fn is_resynchronized(&self) -> bool {
        *self == EE5FAST_A::RESYNCHRONIZED
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == EE5FAST_A::ASYNCHRONOUS
    }
}
#[doc = "Write proxy for field `EE5FAST`"]
pub struct EE5FAST_W<'a> {
    w: &'a mut W,
}
impl<'a> EE5FAST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE5FAST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External event is re-synchronised by the HRTIM logic before acting on outputs"]
    #[inline(always)]
    pub fn resynchronized(self) -> &'a mut W {
        self.variant(EE5FAST_A::RESYNCHRONIZED)
    }
    #[doc = "External event is acting asynchronously on outputs (low-latency mode)"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(EE5FAST_A::ASYNCHRONOUS)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "External Event 5 Sensitivity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EE5SNS_A {
    #[doc = "0: On active level defined by EExPOL bit"]
    ACTIVE = 0,
    #[doc = "1: Rising edge"]
    RISING = 1,
    #[doc = "2: Falling edge"]
    FALLING = 2,
    #[doc = "3: Both edges"]
    BOTH = 3,
}
impl From<EE5SNS_A> for u8 {
    #[inline(always)]
    fn from(variant: EE5SNS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EE5SNS`"]
pub type EE5SNS_R = crate::R<u8, EE5SNS_A>;
impl EE5SNS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EE5SNS_A {
        match self.bits {
            0 => EE5SNS_A::ACTIVE,
            1 => EE5SNS_A::RISING,
            2 => EE5SNS_A::FALLING,
            3 => EE5SNS_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == EE5SNS_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == EE5SNS_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == EE5SNS_A::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == EE5SNS_A::BOTH
    }
}
#[doc = "Write proxy for field `EE5SNS`"]
pub struct EE5SNS_W<'a> {
    w: &'a mut W,
}
impl<'a> EE5SNS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE5SNS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "On active level defined by EExPOL bit"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(EE5SNS_A::ACTIVE)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(EE5SNS_A::RISING)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(EE5SNS_A::FALLING)
    }
    #[doc = "Both edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(EE5SNS_A::BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | (((value as u32) & 0x03) << 27);
        self.w
    }
}
#[doc = "External Event 5 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EE5POL_A {
    #[doc = "0: External event is active high"]
    ACTIVEHIGH = 0,
    #[doc = "1: External event is active low"]
    ACTIVELOW = 1,
}
impl From<EE5POL_A> for bool {
    #[inline(always)]
    fn from(variant: EE5POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EE5POL`"]
pub type EE5POL_R = crate::R<bool, EE5POL_A>;
impl EE5POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EE5POL_A {
        match self.bits {
            false => EE5POL_A::ACTIVEHIGH,
            true => EE5POL_A::ACTIVELOW,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVEHIGH`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == EE5POL_A::ACTIVEHIGH
    }
    #[doc = "Checks if the value of the field is `ACTIVELOW`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == EE5POL_A::ACTIVELOW
    }
}
#[doc = "Write proxy for field `EE5POL`"]
pub struct EE5POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EE5POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE5POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External event is active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(EE5POL_A::ACTIVEHIGH)
    }
    #[doc = "External event is active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(EE5POL_A::ACTIVELOW)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "External Event 5 Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EE5SRC_A {
    #[doc = "0: Source 1"]
    SRC1 = 0,
    #[doc = "1: Source 2"]
    SRC2 = 1,
    #[doc = "2: Source 3"]
    SRC3 = 2,
    #[doc = "3: Source 4"]
    SRC4 = 3,
}
impl From<EE5SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: EE5SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EE5SRC`"]
pub type EE5SRC_R = crate::R<u8, EE5SRC_A>;
impl EE5SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EE5SRC_A {
        match self.bits {
            0 => EE5SRC_A::SRC1,
            1 => EE5SRC_A::SRC2,
            2 => EE5SRC_A::SRC3,
            3 => EE5SRC_A::SRC4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SRC1`"]
    #[inline(always)]
    pub fn is_src1(&self) -> bool {
        *self == EE5SRC_A::SRC1
    }
    #[doc = "Checks if the value of the field is `SRC2`"]
    #[inline(always)]
    pub fn is_src2(&self) -> bool {
        *self == EE5SRC_A::SRC2
    }
    #[doc = "Checks if the value of the field is `SRC3`"]
    #[inline(always)]
    pub fn is_src3(&self) -> bool {
        *self == EE5SRC_A::SRC3
    }
    #[doc = "Checks if the value of the field is `SRC4`"]
    #[inline(always)]
    pub fn is_src4(&self) -> bool {
        *self == EE5SRC_A::SRC4
    }
}
#[doc = "Write proxy for field `EE5SRC`"]
pub struct EE5SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> EE5SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE5SRC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Source 1"]
    #[inline(always)]
    pub fn src1(self) -> &'a mut W {
        self.variant(EE5SRC_A::SRC1)
    }
    #[doc = "Source 2"]
    #[inline(always)]
    pub fn src2(self) -> &'a mut W {
        self.variant(EE5SRC_A::SRC2)
    }
    #[doc = "Source 3"]
    #[inline(always)]
    pub fn src3(self) -> &'a mut W {
        self.variant(EE5SRC_A::SRC3)
    }
    #[doc = "Source 4"]
    #[inline(always)]
    pub fn src4(self) -> &'a mut W {
        self.variant(EE5SRC_A::SRC4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "External Event 4 Fast mode"]
pub type EE4FAST_A = EE5FAST_A;
#[doc = "Reader of field `EE4FAST`"]
pub type EE4FAST_R = crate::R<bool, EE5FAST_A>;
#[doc = "Write proxy for field `EE4FAST`"]
pub struct EE4FAST_W<'a> {
    w: &'a mut W,
}
impl<'a> EE4FAST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE4FAST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External event is re-synchronised by the HRTIM logic before acting on outputs"]
    #[inline(always)]
    pub fn resynchronized(self) -> &'a mut W {
        self.variant(EE5FAST_A::RESYNCHRONIZED)
    }
    #[doc = "External event is acting asynchronously on outputs (low-latency mode)"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(EE5FAST_A::ASYNCHRONOUS)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "External Event 4 Sensitivity"]
pub type EE4SNS_A = EE5SNS_A;
#[doc = "Reader of field `EE4SNS`"]
pub type EE4SNS_R = crate::R<u8, EE5SNS_A>;
#[doc = "Write proxy for field `EE4SNS`"]
pub struct EE4SNS_W<'a> {
    w: &'a mut W,
}
impl<'a> EE4SNS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE4SNS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "On active level defined by EExPOL bit"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(EE5SNS_A::ACTIVE)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(EE5SNS_A::RISING)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(EE5SNS_A::FALLING)
    }
    #[doc = "Both edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(EE5SNS_A::BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "External Event 4 Polarity"]
pub type EE4POL_A = EE5POL_A;
#[doc = "Reader of field `EE4POL`"]
pub type EE4POL_R = crate::R<bool, EE5POL_A>;
#[doc = "Write proxy for field `EE4POL`"]
pub struct EE4POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EE4POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE4POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External event is active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(EE5POL_A::ACTIVEHIGH)
    }
    #[doc = "External event is active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(EE5POL_A::ACTIVELOW)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "External Event 4 Source"]
pub type EE4SRC_A = EE5SRC_A;
#[doc = "Reader of field `EE4SRC`"]
pub type EE4SRC_R = crate::R<u8, EE5SRC_A>;
#[doc = "Write proxy for field `EE4SRC`"]
pub struct EE4SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> EE4SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE4SRC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Source 1"]
    #[inline(always)]
    pub fn src1(self) -> &'a mut W {
        self.variant(EE5SRC_A::SRC1)
    }
    #[doc = "Source 2"]
    #[inline(always)]
    pub fn src2(self) -> &'a mut W {
        self.variant(EE5SRC_A::SRC2)
    }
    #[doc = "Source 3"]
    #[inline(always)]
    pub fn src3(self) -> &'a mut W {
        self.variant(EE5SRC_A::SRC3)
    }
    #[doc = "Source 4"]
    #[inline(always)]
    pub fn src4(self) -> &'a mut W {
        self.variant(EE5SRC_A::SRC4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "External Event 3 Fast mode"]
pub type EE3FAST_A = EE5FAST_A;
#[doc = "Reader of field `EE3FAST`"]
pub type EE3FAST_R = crate::R<bool, EE5FAST_A>;
#[doc = "Write proxy for field `EE3FAST`"]
pub struct EE3FAST_W<'a> {
    w: &'a mut W,
}
impl<'a> EE3FAST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE3FAST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External event is re-synchronised by the HRTIM logic before acting on outputs"]
    #[inline(always)]
    pub fn resynchronized(self) -> &'a mut W {
        self.variant(EE5FAST_A::RESYNCHRONIZED)
    }
    #[doc = "External event is acting asynchronously on outputs (low-latency mode)"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(EE5FAST_A::ASYNCHRONOUS)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "External Event 3 Sensitivity"]
pub type EE3SNS_A = EE5SNS_A;
#[doc = "Reader of field `EE3SNS`"]
pub type EE3SNS_R = crate::R<u8, EE5SNS_A>;
#[doc = "Write proxy for field `EE3SNS`"]
pub struct EE3SNS_W<'a> {
    w: &'a mut W,
}
impl<'a> EE3SNS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE3SNS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "On active level defined by EExPOL bit"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(EE5SNS_A::ACTIVE)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(EE5SNS_A::RISING)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(EE5SNS_A::FALLING)
    }
    #[doc = "Both edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(EE5SNS_A::BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 15)) | (((value as u32) & 0x03) << 15);
        self.w
    }
}
#[doc = "External Event 3 Polarity"]
pub type EE3POL_A = EE5POL_A;
#[doc = "Reader of field `EE3POL`"]
pub type EE3POL_R = crate::R<bool, EE5POL_A>;
#[doc = "Write proxy for field `EE3POL`"]
pub struct EE3POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EE3POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE3POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External event is active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(EE5POL_A::ACTIVEHIGH)
    }
    #[doc = "External event is active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(EE5POL_A::ACTIVELOW)
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
#[doc = "External Event 3 Source"]
pub type EE3SRC_A = EE5SRC_A;
#[doc = "Reader of field `EE3SRC`"]
pub type EE3SRC_R = crate::R<u8, EE5SRC_A>;
#[doc = "Write proxy for field `EE3SRC`"]
pub struct EE3SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> EE3SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE3SRC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Source 1"]
    #[inline(always)]
    pub fn src1(self) -> &'a mut W {
        self.variant(EE5SRC_A::SRC1)
    }
    #[doc = "Source 2"]
    #[inline(always)]
    pub fn src2(self) -> &'a mut W {
        self.variant(EE5SRC_A::SRC2)
    }
    #[doc = "Source 3"]
    #[inline(always)]
    pub fn src3(self) -> &'a mut W {
        self.variant(EE5SRC_A::SRC3)
    }
    #[doc = "Source 4"]
    #[inline(always)]
    pub fn src4(self) -> &'a mut W {
        self.variant(EE5SRC_A::SRC4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "External Event 2 Fast mode"]
pub type EE2FAST_A = EE5FAST_A;
#[doc = "Reader of field `EE2FAST`"]
pub type EE2FAST_R = crate::R<bool, EE5FAST_A>;
#[doc = "Write proxy for field `EE2FAST`"]
pub struct EE2FAST_W<'a> {
    w: &'a mut W,
}
impl<'a> EE2FAST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE2FAST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External event is re-synchronised by the HRTIM logic before acting on outputs"]
    #[inline(always)]
    pub fn resynchronized(self) -> &'a mut W {
        self.variant(EE5FAST_A::RESYNCHRONIZED)
    }
    #[doc = "External event is acting asynchronously on outputs (low-latency mode)"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(EE5FAST_A::ASYNCHRONOUS)
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
#[doc = "External Event 2 Sensitivity"]
pub type EE2SNS_A = EE5SNS_A;
#[doc = "Reader of field `EE2SNS`"]
pub type EE2SNS_R = crate::R<u8, EE5SNS_A>;
#[doc = "Write proxy for field `EE2SNS`"]
pub struct EE2SNS_W<'a> {
    w: &'a mut W,
}
impl<'a> EE2SNS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE2SNS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "On active level defined by EExPOL bit"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(EE5SNS_A::ACTIVE)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(EE5SNS_A::RISING)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(EE5SNS_A::FALLING)
    }
    #[doc = "Both edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(EE5SNS_A::BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "External Event 2 Polarity"]
pub type EE2POL_A = EE5POL_A;
#[doc = "Reader of field `EE2POL`"]
pub type EE2POL_R = crate::R<bool, EE5POL_A>;
#[doc = "Write proxy for field `EE2POL`"]
pub struct EE2POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EE2POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE2POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External event is active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(EE5POL_A::ACTIVEHIGH)
    }
    #[doc = "External event is active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(EE5POL_A::ACTIVELOW)
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
#[doc = "External Event 2 Source"]
pub type EE2SRC_A = EE5SRC_A;
#[doc = "Reader of field `EE2SRC`"]
pub type EE2SRC_R = crate::R<u8, EE5SRC_A>;
#[doc = "Write proxy for field `EE2SRC`"]
pub struct EE2SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> EE2SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE2SRC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Source 1"]
    #[inline(always)]
    pub fn src1(self) -> &'a mut W {
        self.variant(EE5SRC_A::SRC1)
    }
    #[doc = "Source 2"]
    #[inline(always)]
    pub fn src2(self) -> &'a mut W {
        self.variant(EE5SRC_A::SRC2)
    }
    #[doc = "Source 3"]
    #[inline(always)]
    pub fn src3(self) -> &'a mut W {
        self.variant(EE5SRC_A::SRC3)
    }
    #[doc = "Source 4"]
    #[inline(always)]
    pub fn src4(self) -> &'a mut W {
        self.variant(EE5SRC_A::SRC4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "External Event 1 Fast mode"]
pub type EE1FAST_A = EE5FAST_A;
#[doc = "Reader of field `EE1FAST`"]
pub type EE1FAST_R = crate::R<bool, EE5FAST_A>;
#[doc = "Write proxy for field `EE1FAST`"]
pub struct EE1FAST_W<'a> {
    w: &'a mut W,
}
impl<'a> EE1FAST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE1FAST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External event is re-synchronised by the HRTIM logic before acting on outputs"]
    #[inline(always)]
    pub fn resynchronized(self) -> &'a mut W {
        self.variant(EE5FAST_A::RESYNCHRONIZED)
    }
    #[doc = "External event is acting asynchronously on outputs (low-latency mode)"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(EE5FAST_A::ASYNCHRONOUS)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "External Event 1 Sensitivity"]
pub type EE1SNS_A = EE5SNS_A;
#[doc = "Reader of field `EE1SNS`"]
pub type EE1SNS_R = crate::R<u8, EE5SNS_A>;
#[doc = "Write proxy for field `EE1SNS`"]
pub struct EE1SNS_W<'a> {
    w: &'a mut W,
}
impl<'a> EE1SNS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE1SNS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "On active level defined by EExPOL bit"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(EE5SNS_A::ACTIVE)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(EE5SNS_A::RISING)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(EE5SNS_A::FALLING)
    }
    #[doc = "Both edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(EE5SNS_A::BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "External Event 1 Polarity"]
pub type EE1POL_A = EE5POL_A;
#[doc = "Reader of field `EE1POL`"]
pub type EE1POL_R = crate::R<bool, EE5POL_A>;
#[doc = "Write proxy for field `EE1POL`"]
pub struct EE1POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EE1POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE1POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External event is active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(EE5POL_A::ACTIVEHIGH)
    }
    #[doc = "External event is active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(EE5POL_A::ACTIVELOW)
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
#[doc = "External Event 1 Source"]
pub type EE1SRC_A = EE5SRC_A;
#[doc = "Reader of field `EE1SRC`"]
pub type EE1SRC_R = crate::R<u8, EE5SRC_A>;
#[doc = "Write proxy for field `EE1SRC`"]
pub struct EE1SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> EE1SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EE1SRC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Source 1"]
    #[inline(always)]
    pub fn src1(self) -> &'a mut W {
        self.variant(EE5SRC_A::SRC1)
    }
    #[doc = "Source 2"]
    #[inline(always)]
    pub fn src2(self) -> &'a mut W {
        self.variant(EE5SRC_A::SRC2)
    }
    #[doc = "Source 3"]
    #[inline(always)]
    pub fn src3(self) -> &'a mut W {
        self.variant(EE5SRC_A::SRC3)
    }
    #[doc = "Source 4"]
    #[inline(always)]
    pub fn src4(self) -> &'a mut W {
        self.variant(EE5SRC_A::SRC4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 29 - External Event 5 Fast mode"]
    #[inline(always)]
    pub fn ee5fast(&self) -> EE5FAST_R {
        EE5FAST_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 27:28 - External Event 5 Sensitivity"]
    #[inline(always)]
    pub fn ee5sns(&self) -> EE5SNS_R {
        EE5SNS_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bit 26 - External Event 5 Polarity"]
    #[inline(always)]
    pub fn ee5pol(&self) -> EE5POL_R {
        EE5POL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - External Event 5 Source"]
    #[inline(always)]
    pub fn ee5src(&self) -> EE5SRC_R {
        EE5SRC_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 23 - External Event 4 Fast mode"]
    #[inline(always)]
    pub fn ee4fast(&self) -> EE4FAST_R {
        EE4FAST_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - External Event 4 Sensitivity"]
    #[inline(always)]
    pub fn ee4sns(&self) -> EE4SNS_R {
        EE4SNS_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 20 - External Event 4 Polarity"]
    #[inline(always)]
    pub fn ee4pol(&self) -> EE4POL_R {
        EE4POL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - External Event 4 Source"]
    #[inline(always)]
    pub fn ee4src(&self) -> EE4SRC_R {
        EE4SRC_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 17 - External Event 3 Fast mode"]
    #[inline(always)]
    pub fn ee3fast(&self) -> EE3FAST_R {
        EE3FAST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 15:16 - External Event 3 Sensitivity"]
    #[inline(always)]
    pub fn ee3sns(&self) -> EE3SNS_R {
        EE3SNS_R::new(((self.bits >> 15) & 0x03) as u8)
    }
    #[doc = "Bit 14 - External Event 3 Polarity"]
    #[inline(always)]
    pub fn ee3pol(&self) -> EE3POL_R {
        EE3POL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - External Event 3 Source"]
    #[inline(always)]
    pub fn ee3src(&self) -> EE3SRC_R {
        EE3SRC_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 11 - External Event 2 Fast mode"]
    #[inline(always)]
    pub fn ee2fast(&self) -> EE2FAST_R {
        EE2FAST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - External Event 2 Sensitivity"]
    #[inline(always)]
    pub fn ee2sns(&self) -> EE2SNS_R {
        EE2SNS_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 8 - External Event 2 Polarity"]
    #[inline(always)]
    pub fn ee2pol(&self) -> EE2POL_R {
        EE2POL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - External Event 2 Source"]
    #[inline(always)]
    pub fn ee2src(&self) -> EE2SRC_R {
        EE2SRC_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 5 - External Event 1 Fast mode"]
    #[inline(always)]
    pub fn ee1fast(&self) -> EE1FAST_R {
        EE1FAST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - External Event 1 Sensitivity"]
    #[inline(always)]
    pub fn ee1sns(&self) -> EE1SNS_R {
        EE1SNS_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 2 - External Event 1 Polarity"]
    #[inline(always)]
    pub fn ee1pol(&self) -> EE1POL_R {
        EE1POL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - External Event 1 Source"]
    #[inline(always)]
    pub fn ee1src(&self) -> EE1SRC_R {
        EE1SRC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 29 - External Event 5 Fast mode"]
    #[inline(always)]
    pub fn ee5fast(&mut self) -> EE5FAST_W {
        EE5FAST_W { w: self }
    }
    #[doc = "Bits 27:28 - External Event 5 Sensitivity"]
    #[inline(always)]
    pub fn ee5sns(&mut self) -> EE5SNS_W {
        EE5SNS_W { w: self }
    }
    #[doc = "Bit 26 - External Event 5 Polarity"]
    #[inline(always)]
    pub fn ee5pol(&mut self) -> EE5POL_W {
        EE5POL_W { w: self }
    }
    #[doc = "Bits 24:25 - External Event 5 Source"]
    #[inline(always)]
    pub fn ee5src(&mut self) -> EE5SRC_W {
        EE5SRC_W { w: self }
    }
    #[doc = "Bit 23 - External Event 4 Fast mode"]
    #[inline(always)]
    pub fn ee4fast(&mut self) -> EE4FAST_W {
        EE4FAST_W { w: self }
    }
    #[doc = "Bits 21:22 - External Event 4 Sensitivity"]
    #[inline(always)]
    pub fn ee4sns(&mut self) -> EE4SNS_W {
        EE4SNS_W { w: self }
    }
    #[doc = "Bit 20 - External Event 4 Polarity"]
    #[inline(always)]
    pub fn ee4pol(&mut self) -> EE4POL_W {
        EE4POL_W { w: self }
    }
    #[doc = "Bits 18:19 - External Event 4 Source"]
    #[inline(always)]
    pub fn ee4src(&mut self) -> EE4SRC_W {
        EE4SRC_W { w: self }
    }
    #[doc = "Bit 17 - External Event 3 Fast mode"]
    #[inline(always)]
    pub fn ee3fast(&mut self) -> EE3FAST_W {
        EE3FAST_W { w: self }
    }
    #[doc = "Bits 15:16 - External Event 3 Sensitivity"]
    #[inline(always)]
    pub fn ee3sns(&mut self) -> EE3SNS_W {
        EE3SNS_W { w: self }
    }
    #[doc = "Bit 14 - External Event 3 Polarity"]
    #[inline(always)]
    pub fn ee3pol(&mut self) -> EE3POL_W {
        EE3POL_W { w: self }
    }
    #[doc = "Bits 12:13 - External Event 3 Source"]
    #[inline(always)]
    pub fn ee3src(&mut self) -> EE3SRC_W {
        EE3SRC_W { w: self }
    }
    #[doc = "Bit 11 - External Event 2 Fast mode"]
    #[inline(always)]
    pub fn ee2fast(&mut self) -> EE2FAST_W {
        EE2FAST_W { w: self }
    }
    #[doc = "Bits 9:10 - External Event 2 Sensitivity"]
    #[inline(always)]
    pub fn ee2sns(&mut self) -> EE2SNS_W {
        EE2SNS_W { w: self }
    }
    #[doc = "Bit 8 - External Event 2 Polarity"]
    #[inline(always)]
    pub fn ee2pol(&mut self) -> EE2POL_W {
        EE2POL_W { w: self }
    }
    #[doc = "Bits 6:7 - External Event 2 Source"]
    #[inline(always)]
    pub fn ee2src(&mut self) -> EE2SRC_W {
        EE2SRC_W { w: self }
    }
    #[doc = "Bit 5 - External Event 1 Fast mode"]
    #[inline(always)]
    pub fn ee1fast(&mut self) -> EE1FAST_W {
        EE1FAST_W { w: self }
    }
    #[doc = "Bits 3:4 - External Event 1 Sensitivity"]
    #[inline(always)]
    pub fn ee1sns(&mut self) -> EE1SNS_W {
        EE1SNS_W { w: self }
    }
    #[doc = "Bit 2 - External Event 1 Polarity"]
    #[inline(always)]
    pub fn ee1pol(&mut self) -> EE1POL_W {
        EE1POL_W { w: self }
    }
    #[doc = "Bits 0:1 - External Event 1 Source"]
    #[inline(always)]
    pub fn ee1src(&mut self) -> EE1SRC_W {
        EE1SRC_W { w: self }
    }
}
