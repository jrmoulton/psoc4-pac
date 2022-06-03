#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt cause register"]
    pub intr_cause: crate::Reg<intr_cause::INTR_CAUSE_SPEC>,
    _reserved1: [u8; 0x2c],
    #[doc = "0x30 - DFT control register"]
    pub dft_ctrl: crate::Reg<dft_ctrl::DFT_CTRL_SPEC>,
    _reserved2: [u8; 0xd4],
    #[doc = "0x108 - PASS Control"]
    pub pass_ctrl: crate::Reg<pass_ctrl::PASS_CTRL_SPEC>,
    _reserved3: [u8; 0x0cf4],
    #[doc = "0xe00..0xe08 - DSAB configuration"]
    pub dsab: DSAB,
    _reserved4: [u8; 0xf8],
    #[doc = "0xf00 - DSAB Trim bits"]
    pub dsab_trim: crate::Reg<dsab_trim::DSAB_TRIM_SPEC>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct DSAB {
    #[doc = "0x00 - global DSAB control"]
    pub dsab_ctrl: crate::Reg<self::dsab::dsab_ctrl::DSAB_CTRL_SPEC>,
    #[doc = "0x04 - DFT bits"]
    pub dsab_dft: crate::Reg<self::dsab::dsab_dft::DSAB_DFT_SPEC>,
}
#[doc = r"Register block"]
#[doc = "DSAB configuration"]
pub mod dsab;
#[doc = "INTR_CAUSE register accessor: an alias for `Reg<INTR_CAUSE_SPEC>`"]
pub type INTR_CAUSE = crate::Reg<intr_cause::INTR_CAUSE_SPEC>;
#[doc = "Interrupt cause register"]
pub mod intr_cause;
#[doc = "DFT_CTRL register accessor: an alias for `Reg<DFT_CTRL_SPEC>`"]
pub type DFT_CTRL = crate::Reg<dft_ctrl::DFT_CTRL_SPEC>;
#[doc = "DFT control register"]
pub mod dft_ctrl;
#[doc = "PASS_CTRL register accessor: an alias for `Reg<PASS_CTRL_SPEC>`"]
pub type PASS_CTRL = crate::Reg<pass_ctrl::PASS_CTRL_SPEC>;
#[doc = "PASS Control"]
pub mod pass_ctrl;
#[doc = "DSAB_TRIM register accessor: an alias for `Reg<DSAB_TRIM_SPEC>`"]
pub type DSAB_TRIM = crate::Reg<dsab_trim::DSAB_TRIM_SPEC>;
#[doc = "DSAB Trim bits"]
pub mod dsab_trim;
