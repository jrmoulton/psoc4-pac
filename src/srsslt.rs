#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power Mode Control"]
    pub pwr_control: crate::Reg<pwr_control::PWR_CONTROL_SPEC>,
    #[doc = "0x04 - Power System Key&Delay Register"]
    pub pwr_key_delay: crate::Reg<pwr_key_delay::PWR_KEY_DELAY_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - Power DDFT Mode Selection Register"]
    pub pwr_ddft_select: crate::Reg<pwr_ddft_select::PWR_DDFT_SELECT_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x14 - Test Mode Control Register"]
    pub tst_mode: crate::Reg<tst_mode::TST_MODE_SPEC>,
    _reserved4: [u8; 0x10],
    #[doc = "0x28 - Clock Select Register"]
    pub clk_select: crate::Reg<clk_select::CLK_SELECT_SPEC>,
    #[doc = "0x2c - ILO Configuration"]
    pub clk_ilo_config: crate::Reg<clk_ilo_config::CLK_ILO_CONFIG_SPEC>,
    #[doc = "0x30 - IMO Configuration"]
    pub clk_imo_config: crate::Reg<clk_imo_config::CLK_IMO_CONFIG_SPEC>,
    #[doc = "0x34 - Clock DFT Mode Selection Register"]
    pub clk_dft_select: crate::Reg<clk_dft_select::CLK_DFT_SELECT_SPEC>,
    #[doc = "0x38 - Watchdog Disable Key Register"]
    pub wdt_disable_key: crate::Reg<wdt_disable_key::WDT_DISABLE_KEY_SPEC>,
    #[doc = "0x3c - Watchdog Counter Register"]
    pub wdt_counter: crate::Reg<wdt_counter::WDT_COUNTER_SPEC>,
    #[doc = "0x40 - Watchdog Match Register"]
    pub wdt_match: crate::Reg<wdt_match::WDT_MATCH_SPEC>,
    #[doc = "0x44 - SRSS Interrupt Register"]
    pub srss_intr: crate::Reg<srss_intr::SRSS_INTR_SPEC>,
    #[doc = "0x48 - SRSS Interrupt Set Register"]
    pub srss_intr_set: crate::Reg<srss_intr_set::SRSS_INTR_SET_SPEC>,
    #[doc = "0x4c - SRSS Interrupt Mask Register"]
    pub srss_intr_mask: crate::Reg<srss_intr_mask::SRSS_INTR_MASK_SPEC>,
    _reserved14: [u8; 0x04],
    #[doc = "0x54 - Reset Cause Observation Register"]
    pub res_cause: crate::Reg<res_cause::RES_CAUSE_SPEC>,
    _reserved15: [u8; 0x0ea8],
    #[doc = "0xf00 - Bandgap Trim Register"]
    pub pwr_bg_trim1: crate::Reg<pwr_bg_trim1::PWR_BG_TRIM1_SPEC>,
    #[doc = "0xf04 - Bandgap Trim Register"]
    pub pwr_bg_trim2: crate::Reg<pwr_bg_trim2::PWR_BG_TRIM2_SPEC>,
    #[doc = "0xf08 - IMO Frequency Select Register"]
    pub clk_imo_select: crate::Reg<clk_imo_select::CLK_IMO_SELECT_SPEC>,
    #[doc = "0xf0c - IMO Trim Register"]
    pub clk_imo_trim1: crate::Reg<clk_imo_trim1::CLK_IMO_TRIM1_SPEC>,
    #[doc = "0xf10 - IMO Trim Register"]
    pub clk_imo_trim2: crate::Reg<clk_imo_trim2::CLK_IMO_TRIM2_SPEC>,
    #[doc = "0xf14 - Power System Trim Register"]
    pub pwr_pwrsys_trim1: crate::Reg<pwr_pwrsys_trim1::PWR_PWRSYS_TRIM1_SPEC>,
    #[doc = "0xf18 - IMO Trim Register"]
    pub clk_imo_trim3: crate::Reg<clk_imo_trim3::CLK_IMO_TRIM3_SPEC>,
}
#[doc = "PWR_CONTROL register accessor: an alias for `Reg<PWR_CONTROL_SPEC>`"]
pub type PWR_CONTROL = crate::Reg<pwr_control::PWR_CONTROL_SPEC>;
#[doc = "Power Mode Control"]
pub mod pwr_control;
#[doc = "PWR_KEY_DELAY register accessor: an alias for `Reg<PWR_KEY_DELAY_SPEC>`"]
pub type PWR_KEY_DELAY = crate::Reg<pwr_key_delay::PWR_KEY_DELAY_SPEC>;
#[doc = "Power System Key&Delay Register"]
pub mod pwr_key_delay;
#[doc = "PWR_DDFT_SELECT register accessor: an alias for `Reg<PWR_DDFT_SELECT_SPEC>`"]
pub type PWR_DDFT_SELECT = crate::Reg<pwr_ddft_select::PWR_DDFT_SELECT_SPEC>;
#[doc = "Power DDFT Mode Selection Register"]
pub mod pwr_ddft_select;
#[doc = "TST_MODE register accessor: an alias for `Reg<TST_MODE_SPEC>`"]
pub type TST_MODE = crate::Reg<tst_mode::TST_MODE_SPEC>;
#[doc = "Test Mode Control Register"]
pub mod tst_mode;
#[doc = "CLK_SELECT register accessor: an alias for `Reg<CLK_SELECT_SPEC>`"]
pub type CLK_SELECT = crate::Reg<clk_select::CLK_SELECT_SPEC>;
#[doc = "Clock Select Register"]
pub mod clk_select;
#[doc = "CLK_ILO_CONFIG register accessor: an alias for `Reg<CLK_ILO_CONFIG_SPEC>`"]
pub type CLK_ILO_CONFIG = crate::Reg<clk_ilo_config::CLK_ILO_CONFIG_SPEC>;
#[doc = "ILO Configuration"]
pub mod clk_ilo_config;
#[doc = "CLK_IMO_CONFIG register accessor: an alias for `Reg<CLK_IMO_CONFIG_SPEC>`"]
pub type CLK_IMO_CONFIG = crate::Reg<clk_imo_config::CLK_IMO_CONFIG_SPEC>;
#[doc = "IMO Configuration"]
pub mod clk_imo_config;
#[doc = "CLK_DFT_SELECT register accessor: an alias for `Reg<CLK_DFT_SELECT_SPEC>`"]
pub type CLK_DFT_SELECT = crate::Reg<clk_dft_select::CLK_DFT_SELECT_SPEC>;
#[doc = "Clock DFT Mode Selection Register"]
pub mod clk_dft_select;
#[doc = "WDT_DISABLE_KEY register accessor: an alias for `Reg<WDT_DISABLE_KEY_SPEC>`"]
pub type WDT_DISABLE_KEY = crate::Reg<wdt_disable_key::WDT_DISABLE_KEY_SPEC>;
#[doc = "Watchdog Disable Key Register"]
pub mod wdt_disable_key;
#[doc = "WDT_COUNTER register accessor: an alias for `Reg<WDT_COUNTER_SPEC>`"]
pub type WDT_COUNTER = crate::Reg<wdt_counter::WDT_COUNTER_SPEC>;
#[doc = "Watchdog Counter Register"]
pub mod wdt_counter;
#[doc = "WDT_MATCH register accessor: an alias for `Reg<WDT_MATCH_SPEC>`"]
pub type WDT_MATCH = crate::Reg<wdt_match::WDT_MATCH_SPEC>;
#[doc = "Watchdog Match Register"]
pub mod wdt_match;
#[doc = "SRSS_INTR register accessor: an alias for `Reg<SRSS_INTR_SPEC>`"]
pub type SRSS_INTR = crate::Reg<srss_intr::SRSS_INTR_SPEC>;
#[doc = "SRSS Interrupt Register"]
pub mod srss_intr;
#[doc = "SRSS_INTR_SET register accessor: an alias for `Reg<SRSS_INTR_SET_SPEC>`"]
pub type SRSS_INTR_SET = crate::Reg<srss_intr_set::SRSS_INTR_SET_SPEC>;
#[doc = "SRSS Interrupt Set Register"]
pub mod srss_intr_set;
#[doc = "SRSS_INTR_MASK register accessor: an alias for `Reg<SRSS_INTR_MASK_SPEC>`"]
pub type SRSS_INTR_MASK = crate::Reg<srss_intr_mask::SRSS_INTR_MASK_SPEC>;
#[doc = "SRSS Interrupt Mask Register"]
pub mod srss_intr_mask;
#[doc = "RES_CAUSE register accessor: an alias for `Reg<RES_CAUSE_SPEC>`"]
pub type RES_CAUSE = crate::Reg<res_cause::RES_CAUSE_SPEC>;
#[doc = "Reset Cause Observation Register"]
pub mod res_cause;
#[doc = "PWR_BG_TRIM1 register accessor: an alias for `Reg<PWR_BG_TRIM1_SPEC>`"]
pub type PWR_BG_TRIM1 = crate::Reg<pwr_bg_trim1::PWR_BG_TRIM1_SPEC>;
#[doc = "Bandgap Trim Register"]
pub mod pwr_bg_trim1;
#[doc = "PWR_BG_TRIM2 register accessor: an alias for `Reg<PWR_BG_TRIM2_SPEC>`"]
pub type PWR_BG_TRIM2 = crate::Reg<pwr_bg_trim2::PWR_BG_TRIM2_SPEC>;
#[doc = "Bandgap Trim Register"]
pub mod pwr_bg_trim2;
#[doc = "CLK_IMO_SELECT register accessor: an alias for `Reg<CLK_IMO_SELECT_SPEC>`"]
pub type CLK_IMO_SELECT = crate::Reg<clk_imo_select::CLK_IMO_SELECT_SPEC>;
#[doc = "IMO Frequency Select Register"]
pub mod clk_imo_select;
#[doc = "CLK_IMO_TRIM1 register accessor: an alias for `Reg<CLK_IMO_TRIM1_SPEC>`"]
pub type CLK_IMO_TRIM1 = crate::Reg<clk_imo_trim1::CLK_IMO_TRIM1_SPEC>;
#[doc = "IMO Trim Register"]
pub mod clk_imo_trim1;
#[doc = "CLK_IMO_TRIM2 register accessor: an alias for `Reg<CLK_IMO_TRIM2_SPEC>`"]
pub type CLK_IMO_TRIM2 = crate::Reg<clk_imo_trim2::CLK_IMO_TRIM2_SPEC>;
#[doc = "IMO Trim Register"]
pub mod clk_imo_trim2;
#[doc = "PWR_PWRSYS_TRIM1 register accessor: an alias for `Reg<PWR_PWRSYS_TRIM1_SPEC>`"]
pub type PWR_PWRSYS_TRIM1 = crate::Reg<pwr_pwrsys_trim1::PWR_PWRSYS_TRIM1_SPEC>;
#[doc = "Power System Trim Register"]
pub mod pwr_pwrsys_trim1;
#[doc = "CLK_IMO_TRIM3 register accessor: an alias for `Reg<CLK_IMO_TRIM3_SPEC>`"]
pub type CLK_IMO_TRIM3 = crate::Reg<clk_imo_trim3::CLK_IMO_TRIM3_SPEC>;
#[doc = "IMO Trim Register"]
pub mod clk_imo_trim3;
