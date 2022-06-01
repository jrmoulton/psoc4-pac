#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x4020_01c8],
    #[doc = "0x402001c8 - Current counter value"]
    pub counter: crate::Reg<counter::COUNTER_SPEC>,
    #[doc = "0x402001cc - Compare / capture value"]
    pub cc: crate::Reg<cc::CC_SPEC>,
    #[doc = "0x402001d0 - Compare / capture buffer value"]
    pub cc_buf: crate::Reg<cc_buf::CC_BUF_SPEC>,
    #[doc = "0x402001d4 - Period value"]
    pub period: crate::Reg<period::PERIOD_SPEC>,
    #[doc = "0x402001d8 - Period buffer value"]
    pub period_buf: crate::Reg<period_buf::PERIOD_BUF_SPEC>,
}
#[doc = "COUNTER register accessor: an alias for `Reg<COUNTER_SPEC>`"]
pub type COUNTER = crate::Reg<counter::COUNTER_SPEC>;
#[doc = "Current counter value"]
pub mod counter;
#[doc = "CC register accessor: an alias for `Reg<CC_SPEC>`"]
pub type CC = crate::Reg<cc::CC_SPEC>;
#[doc = "Compare / capture value"]
pub mod cc;
#[doc = "CC_BUF register accessor: an alias for `Reg<CC_BUF_SPEC>`"]
pub type CC_BUF = crate::Reg<cc_buf::CC_BUF_SPEC>;
#[doc = "Compare / capture buffer value"]
pub mod cc_buf;
#[doc = "PERIOD register accessor: an alias for `Reg<PERIOD_SPEC>`"]
pub type PERIOD = crate::Reg<period::PERIOD_SPEC>;
#[doc = "Period value"]
pub mod period;
#[doc = "PERIOD_BUF register accessor: an alias for `Reg<PERIOD_BUF_SPEC>`"]
pub type PERIOD_BUF = crate::Reg<period_buf::PERIOD_BUF_SPEC>;
#[doc = "Period buffer value"]
pub mod period_buf;
