#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - WCO Configuration Register"]
    pub config: crate::Reg<config::CONFIG_SPEC>,
    #[doc = "0x04 - WCO Status Register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x08 - WCO DPLL Register"]
    pub dpll: crate::Reg<dpll::DPLL_SPEC>,
    _reserved3: [u8; 0x01f4],
    #[doc = "0x200 - Watchdog Counters 0/1"]
    pub wdt_ctrlow: crate::Reg<wdt_ctrlow::WDT_CTRLOW_SPEC>,
    #[doc = "0x204 - Watchdog Counter 2"]
    pub wdt_ctrhigh: crate::Reg<wdt_ctrhigh::WDT_CTRHIGH_SPEC>,
    #[doc = "0x208 - Watchdog counter match values"]
    pub wdt_match: crate::Reg<wdt_match::WDT_MATCH_SPEC>,
    #[doc = "0x20c - Watchdog Counters Configuration"]
    pub wdt_config: crate::Reg<wdt_config::WDT_CONFIG_SPEC>,
    #[doc = "0x210 - Watchdog Counters Control"]
    pub wdt_control: crate::Reg<wdt_control::WDT_CONTROL_SPEC>,
    #[doc = "0x214 - Watchdog Counters Clock Enable"]
    pub wdt_clken: crate::Reg<wdt_clken::WDT_CLKEN_SPEC>,
    _reserved9: [u8; 0x0ce8],
    #[doc = "0xf00 - WCO Trim Register"]
    pub trim: crate::Reg<trim::TRIM_SPEC>,
}
#[doc = "CONFIG register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "WCO Configuration Register"]
pub mod config;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "WCO Status Register"]
pub mod status;
#[doc = "DPLL register accessor: an alias for `Reg<DPLL_SPEC>`"]
pub type DPLL = crate::Reg<dpll::DPLL_SPEC>;
#[doc = "WCO DPLL Register"]
pub mod dpll;
#[doc = "WDT_CTRLOW register accessor: an alias for `Reg<WDT_CTRLOW_SPEC>`"]
pub type WDT_CTRLOW = crate::Reg<wdt_ctrlow::WDT_CTRLOW_SPEC>;
#[doc = "Watchdog Counters 0/1"]
pub mod wdt_ctrlow;
#[doc = "WDT_CTRHIGH register accessor: an alias for `Reg<WDT_CTRHIGH_SPEC>`"]
pub type WDT_CTRHIGH = crate::Reg<wdt_ctrhigh::WDT_CTRHIGH_SPEC>;
#[doc = "Watchdog Counter 2"]
pub mod wdt_ctrhigh;
#[doc = "WDT_MATCH register accessor: an alias for `Reg<WDT_MATCH_SPEC>`"]
pub type WDT_MATCH = crate::Reg<wdt_match::WDT_MATCH_SPEC>;
#[doc = "Watchdog counter match values"]
pub mod wdt_match;
#[doc = "WDT_CONFIG register accessor: an alias for `Reg<WDT_CONFIG_SPEC>`"]
pub type WDT_CONFIG = crate::Reg<wdt_config::WDT_CONFIG_SPEC>;
#[doc = "Watchdog Counters Configuration"]
pub mod wdt_config;
#[doc = "WDT_CONTROL register accessor: an alias for `Reg<WDT_CONTROL_SPEC>`"]
pub type WDT_CONTROL = crate::Reg<wdt_control::WDT_CONTROL_SPEC>;
#[doc = "Watchdog Counters Control"]
pub mod wdt_control;
#[doc = "WDT_CLKEN register accessor: an alias for `Reg<WDT_CLKEN_SPEC>`"]
pub type WDT_CLKEN = crate::Reg<wdt_clken::WDT_CLKEN_SPEC>;
#[doc = "Watchdog Counters Clock Enable"]
pub mod wdt_clken;
#[doc = "TRIM register accessor: an alias for `Reg<TRIM_SPEC>`"]
pub type TRIM = crate::Reg<trim::TRIM_SPEC>;
#[doc = "WCO Trim Register"]
pub mod trim;
