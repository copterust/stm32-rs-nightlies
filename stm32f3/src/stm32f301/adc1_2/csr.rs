#[doc = "Reader of register CSR"]
pub type R = crate::R<u32, super::CSR>;
#[doc = "Injected Context Queue Overflow flag of the slave ADC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JQOVF_SLV_A {
    #[doc = "0: No injected context queue overflow has occurred"]
    NOOVERFLOW = 0,
    #[doc = "1: Injected context queue overflow has occurred"]
    OVERFLOW = 1,
}
impl From<JQOVF_SLV_A> for bool {
    #[inline(always)]
    fn from(variant: JQOVF_SLV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `JQOVF_SLV`"]
pub type JQOVF_SLV_R = crate::R<bool, JQOVF_SLV_A>;
impl JQOVF_SLV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JQOVF_SLV_A {
        match self.bits {
            false => JQOVF_SLV_A::NOOVERFLOW,
            true => JQOVF_SLV_A::OVERFLOW,
        }
    }
    #[doc = "Checks if the value of the field is `NOOVERFLOW`"]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == JQOVF_SLV_A::NOOVERFLOW
    }
    #[doc = "Checks if the value of the field is `OVERFLOW`"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == JQOVF_SLV_A::OVERFLOW
    }
}
#[doc = "Analog watchdog 3 flag of the slave ADC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWD3_SLV_A {
    #[doc = "0: No analog watchdog event occurred"]
    NOEVENT = 0,
    #[doc = "1: Analog watchdog event occurred"]
    EVENT = 1,
}
impl From<AWD3_SLV_A> for bool {
    #[inline(always)]
    fn from(variant: AWD3_SLV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AWD3_SLV`"]
pub type AWD3_SLV_R = crate::R<bool, AWD3_SLV_A>;
impl AWD3_SLV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWD3_SLV_A {
        match self.bits {
            false => AWD3_SLV_A::NOEVENT,
            true => AWD3_SLV_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == AWD3_SLV_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == AWD3_SLV_A::EVENT
    }
}
#[doc = "Analog watchdog 2 flag of the slave ADC"]
pub type AWD2_SLV_A = AWD3_SLV_A;
#[doc = "Reader of field `AWD2_SLV`"]
pub type AWD2_SLV_R = crate::R<bool, AWD3_SLV_A>;
#[doc = "Analog watchdog 1 flag of the slave ADC"]
pub type AWD1_SLV_A = AWD3_SLV_A;
#[doc = "Reader of field `AWD1_SLV`"]
pub type AWD1_SLV_R = crate::R<bool, AWD3_SLV_A>;
#[doc = "End of injected sequence flag of the slave ADC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JEOS_SLV_A {
    #[doc = "0: Injected sequence is not complete"]
    NOTCOMPLETE = 0,
    #[doc = "1: Injected sequence complete"]
    COMPLETE = 1,
}
impl From<JEOS_SLV_A> for bool {
    #[inline(always)]
    fn from(variant: JEOS_SLV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `JEOS_SLV`"]
pub type JEOS_SLV_R = crate::R<bool, JEOS_SLV_A>;
impl JEOS_SLV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JEOS_SLV_A {
        match self.bits {
            false => JEOS_SLV_A::NOTCOMPLETE,
            true => JEOS_SLV_A::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETE`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == JEOS_SLV_A::NOTCOMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == JEOS_SLV_A::COMPLETE
    }
}
#[doc = "End of injected conversion flag of the slave ADC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JEOC_SLV_A {
    #[doc = "0: Injected conversion is not complete"]
    NOTCOMPLETE = 0,
    #[doc = "1: Injected conversion complete"]
    COMPLETE = 1,
}
impl From<JEOC_SLV_A> for bool {
    #[inline(always)]
    fn from(variant: JEOC_SLV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `JEOC_SLV`"]
pub type JEOC_SLV_R = crate::R<bool, JEOC_SLV_A>;
impl JEOC_SLV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JEOC_SLV_A {
        match self.bits {
            false => JEOC_SLV_A::NOTCOMPLETE,
            true => JEOC_SLV_A::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETE`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == JEOC_SLV_A::NOTCOMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == JEOC_SLV_A::COMPLETE
    }
}
#[doc = "Overrun flag of the slave ADC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVR_SLV_A {
    #[doc = "0: No overrun occurred"]
    NOOVERRUN = 0,
    #[doc = "1: Overrun occurred"]
    OVERRUN = 1,
}
impl From<OVR_SLV_A> for bool {
    #[inline(always)]
    fn from(variant: OVR_SLV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OVR_SLV`"]
pub type OVR_SLV_R = crate::R<bool, OVR_SLV_A>;
impl OVR_SLV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVR_SLV_A {
        match self.bits {
            false => OVR_SLV_A::NOOVERRUN,
            true => OVR_SLV_A::OVERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NOOVERRUN`"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVR_SLV_A::NOOVERRUN
    }
    #[doc = "Checks if the value of the field is `OVERRUN`"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVR_SLV_A::OVERRUN
    }
}
#[doc = "End of regular sequence flag of the slave ADC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOS_SLV_A {
    #[doc = "0: Regular sequence is not complete"]
    NOTCOMPLETE = 0,
    #[doc = "1: Regular sequence complete"]
    COMPLETE = 1,
}
impl From<EOS_SLV_A> for bool {
    #[inline(always)]
    fn from(variant: EOS_SLV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EOS_SLV`"]
pub type EOS_SLV_R = crate::R<bool, EOS_SLV_A>;
impl EOS_SLV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOS_SLV_A {
        match self.bits {
            false => EOS_SLV_A::NOTCOMPLETE,
            true => EOS_SLV_A::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETE`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOS_SLV_A::NOTCOMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOS_SLV_A::COMPLETE
    }
}
#[doc = "End of regular conversion of the slave ADC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOC_SLV_A {
    #[doc = "0: Regular conversion is not complete"]
    NOTCOMPLETE = 0,
    #[doc = "1: Regular conversion complete"]
    COMPLETE = 1,
}
impl From<EOC_SLV_A> for bool {
    #[inline(always)]
    fn from(variant: EOC_SLV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EOC_SLV`"]
pub type EOC_SLV_R = crate::R<bool, EOC_SLV_A>;
impl EOC_SLV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOC_SLV_A {
        match self.bits {
            false => EOC_SLV_A::NOTCOMPLETE,
            true => EOC_SLV_A::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETE`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOC_SLV_A::NOTCOMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOC_SLV_A::COMPLETE
    }
}
#[doc = "End of Sampling phase flag of the slave ADC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOSMP_SLV_A {
    #[doc = "0: End of sampling phase no yet reached"]
    NOTENDED = 0,
    #[doc = "1: End of sampling phase reached"]
    ENDED = 1,
}
impl From<EOSMP_SLV_A> for bool {
    #[inline(always)]
    fn from(variant: EOSMP_SLV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EOSMP_SLV`"]
pub type EOSMP_SLV_R = crate::R<bool, EOSMP_SLV_A>;
impl EOSMP_SLV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOSMP_SLV_A {
        match self.bits {
            false => EOSMP_SLV_A::NOTENDED,
            true => EOSMP_SLV_A::ENDED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTENDED`"]
    #[inline(always)]
    pub fn is_not_ended(&self) -> bool {
        *self == EOSMP_SLV_A::NOTENDED
    }
    #[doc = "Checks if the value of the field is `ENDED`"]
    #[inline(always)]
    pub fn is_ended(&self) -> bool {
        *self == EOSMP_SLV_A::ENDED
    }
}
#[doc = "Slave ADC ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADRDY_SLV_A {
    #[doc = "0: ADC is not ready to start conversion"]
    NOTREADY = 0,
    #[doc = "1: ADC is ready to start conversion"]
    READY = 1,
}
impl From<ADRDY_SLV_A> for bool {
    #[inline(always)]
    fn from(variant: ADRDY_SLV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADRDY_SLV`"]
pub type ADRDY_SLV_R = crate::R<bool, ADRDY_SLV_A>;
impl ADRDY_SLV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADRDY_SLV_A {
        match self.bits {
            false => ADRDY_SLV_A::NOTREADY,
            true => ADRDY_SLV_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == ADRDY_SLV_A::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == ADRDY_SLV_A::READY
    }
}
#[doc = "Injected Context Queue Overflow flag of the master ADC"]
pub type JQOVF_MST_A = JQOVF_SLV_A;
#[doc = "Reader of field `JQOVF_MST`"]
pub type JQOVF_MST_R = crate::R<bool, JQOVF_SLV_A>;
#[doc = "Analog watchdog 3 flag of the master ADC"]
pub type AWD3_MST_A = AWD3_SLV_A;
#[doc = "Reader of field `AWD3_MST`"]
pub type AWD3_MST_R = crate::R<bool, AWD3_SLV_A>;
#[doc = "Analog watchdog 2 flag of the master ADC"]
pub type AWD2_MST_A = AWD3_SLV_A;
#[doc = "Reader of field `AWD2_MST`"]
pub type AWD2_MST_R = crate::R<bool, AWD3_SLV_A>;
#[doc = "Analog watchdog 1 flag of the master ADC"]
pub type AWD1_MST_A = AWD3_SLV_A;
#[doc = "Reader of field `AWD1_MST`"]
pub type AWD1_MST_R = crate::R<bool, AWD3_SLV_A>;
#[doc = "End of injected sequence flag of the master ADC"]
pub type JEOS_MST_A = JEOS_SLV_A;
#[doc = "Reader of field `JEOS_MST`"]
pub type JEOS_MST_R = crate::R<bool, JEOS_SLV_A>;
#[doc = "End of injected conversion flag of the master ADC"]
pub type JEOC_MST_A = JEOC_SLV_A;
#[doc = "Reader of field `JEOC_MST`"]
pub type JEOC_MST_R = crate::R<bool, JEOC_SLV_A>;
#[doc = "Overrun flag of the master ADC"]
pub type OVR_MST_A = OVR_SLV_A;
#[doc = "Reader of field `OVR_MST`"]
pub type OVR_MST_R = crate::R<bool, OVR_SLV_A>;
#[doc = "End of regular sequence flag of the master ADC"]
pub type EOS_MST_A = EOS_SLV_A;
#[doc = "Reader of field `EOS_MST`"]
pub type EOS_MST_R = crate::R<bool, EOS_SLV_A>;
#[doc = "End of regular conversion of the master ADC"]
pub type EOC_MST_A = EOC_SLV_A;
#[doc = "Reader of field `EOC_MST`"]
pub type EOC_MST_R = crate::R<bool, EOC_SLV_A>;
#[doc = "End of Sampling phase flag of the master ADC"]
pub type EOSMP_MST_A = EOSMP_SLV_A;
#[doc = "Reader of field `EOSMP_MST`"]
pub type EOSMP_MST_R = crate::R<bool, EOSMP_SLV_A>;
#[doc = "Master ADC ready"]
pub type ADRDY_MST_A = ADRDY_SLV_A;
#[doc = "Reader of field `ADRDY_MST`"]
pub type ADRDY_MST_R = crate::R<bool, ADRDY_SLV_A>;
impl R {
    #[doc = "Bit 26 - Injected Context Queue Overflow flag of the slave ADC"]
    #[inline(always)]
    pub fn jqovf_slv(&self) -> JQOVF_SLV_R {
        JQOVF_SLV_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Analog watchdog 3 flag of the slave ADC"]
    #[inline(always)]
    pub fn awd3_slv(&self) -> AWD3_SLV_R {
        AWD3_SLV_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Analog watchdog 2 flag of the slave ADC"]
    #[inline(always)]
    pub fn awd2_slv(&self) -> AWD2_SLV_R {
        AWD2_SLV_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Analog watchdog 1 flag of the slave ADC"]
    #[inline(always)]
    pub fn awd1_slv(&self) -> AWD1_SLV_R {
        AWD1_SLV_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - End of injected sequence flag of the slave ADC"]
    #[inline(always)]
    pub fn jeos_slv(&self) -> JEOS_SLV_R {
        JEOS_SLV_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - End of injected conversion flag of the slave ADC"]
    #[inline(always)]
    pub fn jeoc_slv(&self) -> JEOC_SLV_R {
        JEOC_SLV_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Overrun flag of the slave ADC"]
    #[inline(always)]
    pub fn ovr_slv(&self) -> OVR_SLV_R {
        OVR_SLV_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - End of regular sequence flag of the slave ADC"]
    #[inline(always)]
    pub fn eos_slv(&self) -> EOS_SLV_R {
        EOS_SLV_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - End of regular conversion of the slave ADC"]
    #[inline(always)]
    pub fn eoc_slv(&self) -> EOC_SLV_R {
        EOC_SLV_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - End of Sampling phase flag of the slave ADC"]
    #[inline(always)]
    pub fn eosmp_slv(&self) -> EOSMP_SLV_R {
        EOSMP_SLV_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Slave ADC ready"]
    #[inline(always)]
    pub fn adrdy_slv(&self) -> ADRDY_SLV_R {
        ADRDY_SLV_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Injected Context Queue Overflow flag of the master ADC"]
    #[inline(always)]
    pub fn jqovf_mst(&self) -> JQOVF_MST_R {
        JQOVF_MST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Analog watchdog 3 flag of the master ADC"]
    #[inline(always)]
    pub fn awd3_mst(&self) -> AWD3_MST_R {
        AWD3_MST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Analog watchdog 2 flag of the master ADC"]
    #[inline(always)]
    pub fn awd2_mst(&self) -> AWD2_MST_R {
        AWD2_MST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Analog watchdog 1 flag of the master ADC"]
    #[inline(always)]
    pub fn awd1_mst(&self) -> AWD1_MST_R {
        AWD1_MST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - End of injected sequence flag of the master ADC"]
    #[inline(always)]
    pub fn jeos_mst(&self) -> JEOS_MST_R {
        JEOS_MST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - End of injected conversion flag of the master ADC"]
    #[inline(always)]
    pub fn jeoc_mst(&self) -> JEOC_MST_R {
        JEOC_MST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Overrun flag of the master ADC"]
    #[inline(always)]
    pub fn ovr_mst(&self) -> OVR_MST_R {
        OVR_MST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - End of regular sequence flag of the master ADC"]
    #[inline(always)]
    pub fn eos_mst(&self) -> EOS_MST_R {
        EOS_MST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - End of regular conversion of the master ADC"]
    #[inline(always)]
    pub fn eoc_mst(&self) -> EOC_MST_R {
        EOC_MST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - End of Sampling phase flag of the master ADC"]
    #[inline(always)]
    pub fn eosmp_mst(&self) -> EOSMP_MST_R {
        EOSMP_MST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Master ADC ready"]
    #[inline(always)]
    pub fn adrdy_mst(&self) -> ADRDY_MST_R {
        ADRDY_MST_R::new((self.bits & 0x01) != 0)
    }
}
