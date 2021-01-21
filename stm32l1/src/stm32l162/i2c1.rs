#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CR1"]
    pub cr1: CR1,
    #[doc = "0x04 - CR2"]
    pub cr2: CR2,
    #[doc = "0x08 - OAR1"]
    pub oar1: OAR1,
    #[doc = "0x0c - OAR2"]
    pub oar2: OAR2,
    #[doc = "0x10 - DR"]
    pub dr: DR,
    #[doc = "0x14 - SR1"]
    pub sr1: SR1,
    #[doc = "0x18 - SR2"]
    pub sr2: SR2,
    #[doc = "0x1c - CCR"]
    pub ccr: CCR,
    #[doc = "0x20 - TRISE"]
    pub trise: TRISE,
}
#[doc = "CR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](cr1) module"]
pub type CR1 = crate::Reg<u32, _CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR1;
#[doc = "`read()` method returns [cr1::R](cr1::R) reader structure"]
impl crate::Readable for CR1 {}
#[doc = "`write(|w| ..)` method takes [cr1::W](cr1::W) writer structure"]
impl crate::Writable for CR1 {}
#[doc = "CR1"]
pub mod cr1;
#[doc = "CR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](cr2) module"]
pub type CR2 = crate::Reg<u32, _CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR2;
#[doc = "`read()` method returns [cr2::R](cr2::R) reader structure"]
impl crate::Readable for CR2 {}
#[doc = "`write(|w| ..)` method takes [cr2::W](cr2::W) writer structure"]
impl crate::Writable for CR2 {}
#[doc = "CR2"]
pub mod cr2;
#[doc = "OAR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oar1](oar1) module"]
pub type OAR1 = crate::Reg<u32, _OAR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OAR1;
#[doc = "`read()` method returns [oar1::R](oar1::R) reader structure"]
impl crate::Readable for OAR1 {}
#[doc = "`write(|w| ..)` method takes [oar1::W](oar1::W) writer structure"]
impl crate::Writable for OAR1 {}
#[doc = "OAR1"]
pub mod oar1;
#[doc = "OAR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oar2](oar2) module"]
pub type OAR2 = crate::Reg<u32, _OAR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OAR2;
#[doc = "`read()` method returns [oar2::R](oar2::R) reader structure"]
impl crate::Readable for OAR2 {}
#[doc = "`write(|w| ..)` method takes [oar2::W](oar2::W) writer structure"]
impl crate::Writable for OAR2 {}
#[doc = "OAR2"]
pub mod oar2;
#[doc = "DR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr](dr) module"]
pub type DR = crate::Reg<u32, _DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR;
#[doc = "`read()` method returns [dr::R](dr::R) reader structure"]
impl crate::Readable for DR {}
#[doc = "`write(|w| ..)` method takes [dr::W](dr::W) writer structure"]
impl crate::Writable for DR {}
#[doc = "DR"]
pub mod dr;
#[doc = "SR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr1](sr1) module"]
pub type SR1 = crate::Reg<u32, _SR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR1;
#[doc = "`read()` method returns [sr1::R](sr1::R) reader structure"]
impl crate::Readable for SR1 {}
#[doc = "`write(|w| ..)` method takes [sr1::W](sr1::W) writer structure"]
impl crate::Writable for SR1 {}
#[doc = "SR1"]
pub mod sr1;
#[doc = "SR2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr2](sr2) module"]
pub type SR2 = crate::Reg<u32, _SR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR2;
#[doc = "`read()` method returns [sr2::R](sr2::R) reader structure"]
impl crate::Readable for SR2 {}
#[doc = "SR2"]
pub mod sr2;
#[doc = "CCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](ccr) module"]
pub type CCR = crate::Reg<u32, _CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR;
#[doc = "`read()` method returns [ccr::R](ccr::R) reader structure"]
impl crate::Readable for CCR {}
#[doc = "`write(|w| ..)` method takes [ccr::W](ccr::W) writer structure"]
impl crate::Writable for CCR {}
#[doc = "CCR"]
pub mod ccr;
#[doc = "TRISE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trise](trise) module"]
pub type TRISE = crate::Reg<u32, _TRISE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRISE;
#[doc = "`read()` method returns [trise::R](trise::R) reader structure"]
impl crate::Readable for TRISE {}
#[doc = "`write(|w| ..)` method takes [trise::W](trise::W) writer structure"]
impl crate::Writable for TRISE {}
#[doc = "TRISE"]
pub mod trise;