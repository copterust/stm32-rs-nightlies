#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - configuration register 1"]
    pub cfgr1: CFGR1,
    #[doc = "0x04 - CCM SRAM protection register"]
    pub rcr: RCR,
    #[doc = "0x08 - external interrupt configuration register 1"]
    pub exticr1: EXTICR1,
    #[doc = "0x0c - external interrupt configuration register 2"]
    pub exticr2: EXTICR2,
    #[doc = "0x10 - external interrupt configuration register 3"]
    pub exticr3: EXTICR3,
    #[doc = "0x14 - external interrupt configuration register 4"]
    pub exticr4: EXTICR4,
    #[doc = "0x18 - configuration register 2"]
    pub cfgr2: CFGR2,
    _reserved7: [u8; 52usize],
    #[doc = "0x50 - configuration register 3"]
    pub cfgr3: CFGR3,
}
#[doc = "configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr1](cfgr1) module"]
pub type CFGR1 = crate::Reg<u32, _CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR1;
#[doc = "`read()` method returns [cfgr1::R](cfgr1::R) reader structure"]
impl crate::Readable for CFGR1 {}
#[doc = "`write(|w| ..)` method takes [cfgr1::W](cfgr1::W) writer structure"]
impl crate::Writable for CFGR1 {}
#[doc = "configuration register 1"]
pub mod cfgr1;
#[doc = "external interrupt configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr1](exticr1) module"]
pub type EXTICR1 = crate::Reg<u32, _EXTICR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR1;
#[doc = "`read()` method returns [exticr1::R](exticr1::R) reader structure"]
impl crate::Readable for EXTICR1 {}
#[doc = "`write(|w| ..)` method takes [exticr1::W](exticr1::W) writer structure"]
impl crate::Writable for EXTICR1 {}
#[doc = "external interrupt configuration register 1"]
pub mod exticr1;
#[doc = "external interrupt configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr2](exticr2) module"]
pub type EXTICR2 = crate::Reg<u32, _EXTICR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR2;
#[doc = "`read()` method returns [exticr2::R](exticr2::R) reader structure"]
impl crate::Readable for EXTICR2 {}
#[doc = "`write(|w| ..)` method takes [exticr2::W](exticr2::W) writer structure"]
impl crate::Writable for EXTICR2 {}
#[doc = "external interrupt configuration register 2"]
pub mod exticr2;
#[doc = "external interrupt configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr3](exticr3) module"]
pub type EXTICR3 = crate::Reg<u32, _EXTICR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR3;
#[doc = "`read()` method returns [exticr3::R](exticr3::R) reader structure"]
impl crate::Readable for EXTICR3 {}
#[doc = "`write(|w| ..)` method takes [exticr3::W](exticr3::W) writer structure"]
impl crate::Writable for EXTICR3 {}
#[doc = "external interrupt configuration register 3"]
pub mod exticr3;
#[doc = "external interrupt configuration register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr4](exticr4) module"]
pub type EXTICR4 = crate::Reg<u32, _EXTICR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR4;
#[doc = "`read()` method returns [exticr4::R](exticr4::R) reader structure"]
impl crate::Readable for EXTICR4 {}
#[doc = "`write(|w| ..)` method takes [exticr4::W](exticr4::W) writer structure"]
impl crate::Writable for EXTICR4 {}
#[doc = "external interrupt configuration register 4"]
pub mod exticr4;
#[doc = "configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr2](cfgr2) module"]
pub type CFGR2 = crate::Reg<u32, _CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR2;
#[doc = "`read()` method returns [cfgr2::R](cfgr2::R) reader structure"]
impl crate::Readable for CFGR2 {}
#[doc = "`write(|w| ..)` method takes [cfgr2::W](cfgr2::W) writer structure"]
impl crate::Writable for CFGR2 {}
#[doc = "configuration register 2"]
pub mod cfgr2;
#[doc = "CCM SRAM protection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcr](rcr) module"]
pub type RCR = crate::Reg<u32, _RCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCR;
#[doc = "`read()` method returns [rcr::R](rcr::R) reader structure"]
impl crate::Readable for RCR {}
#[doc = "`write(|w| ..)` method takes [rcr::W](rcr::W) writer structure"]
impl crate::Writable for RCR {}
#[doc = "CCM SRAM protection register"]
pub mod rcr;
#[doc = "configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr3](cfgr3) module"]
pub type CFGR3 = crate::Reg<u32, _CFGR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR3;
#[doc = "`read()` method returns [cfgr3::R](cfgr3::R) reader structure"]
impl crate::Readable for CFGR3 {}
#[doc = "`write(|w| ..)` method takes [cfgr3::W](cfgr3::W) writer structure"]
impl crate::Writable for CFGR3 {}
#[doc = "configuration register 3"]
pub mod cfgr3;
