#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Divider command register"]
    pub div_cmd: crate::Reg<div_cmd::DIV_CMD_SPEC>,
    _reserved1: [u8; 0xfc],
    #[doc = "0x100..0x200 - Programmable clock control register"]
    pub pclk_ctl: [crate::Reg<pclk_ctl::PCLK_CTL_SPEC>; 64],
    #[doc = "0x200..0x300 - Divider control register (for 8.0 divider)"]
    pub div_8_ctl: [crate::Reg<div_8_ctl::DIV_8_CTL_SPEC>; 64],
    #[doc = "0x300..0x400 - Divider control register (for 16.0 divider)"]
    pub div_16_ctl: [crate::Reg<div_16_ctl::DIV_16_CTL_SPEC>; 64],
    #[doc = "0x400..0x500 - Divider control register (for 16.5 divider)"]
    pub div_16_5_ctl: [crate::Reg<div_16_5_ctl::DIV_16_5_CTL_SPEC>; 64],
    #[doc = "0x500..0x5fc - Divider control register (for 24.5 divider)"]
    pub div_24_5_ctl: [crate::Reg<div_24_5_ctl::DIV_24_5_CTL_SPEC>; 63],
    _reserved6: [u8; 0x04],
    #[doc = "0x600 - Trigger control register"]
    pub tr_ctl: crate::Reg<tr_ctl::TR_CTL_SPEC>,
    _reserved7: [u8; 0x19fc],
    #[doc = "0x2000..0x2400 - Peripheral Interconnect trigger group control registers"]
    pub tr_group: [TR_GROUP; 2],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct TR_GROUP {
    #[doc = "0x00..0x200 - Trigger control register"]
    pub tr_out_ctl: [crate::Reg<self::tr_group::tr_out_ctl::TR_OUT_CTL_SPEC>; 128],
}
#[doc = r"Register block"]
#[doc = "Peripheral Interconnect trigger group control registers"]
pub mod tr_group;
#[doc = "DIV_CMD register accessor: an alias for `Reg<DIV_CMD_SPEC>`"]
pub type DIV_CMD = crate::Reg<div_cmd::DIV_CMD_SPEC>;
#[doc = "Divider command register"]
pub mod div_cmd;
#[doc = "PCLK_CTL register accessor: an alias for `Reg<PCLK_CTL_SPEC>`"]
pub type PCLK_CTL = crate::Reg<pclk_ctl::PCLK_CTL_SPEC>;
#[doc = "Programmable clock control register"]
pub mod pclk_ctl;
#[doc = "DIV_8_CTL register accessor: an alias for `Reg<DIV_8_CTL_SPEC>`"]
pub type DIV_8_CTL = crate::Reg<div_8_ctl::DIV_8_CTL_SPEC>;
#[doc = "Divider control register (for 8.0 divider)"]
pub mod div_8_ctl;
#[doc = "DIV_16_CTL register accessor: an alias for `Reg<DIV_16_CTL_SPEC>`"]
pub type DIV_16_CTL = crate::Reg<div_16_ctl::DIV_16_CTL_SPEC>;
#[doc = "Divider control register (for 16.0 divider)"]
pub mod div_16_ctl;
#[doc = "DIV_16_5_CTL register accessor: an alias for `Reg<DIV_16_5_CTL_SPEC>`"]
pub type DIV_16_5_CTL = crate::Reg<div_16_5_ctl::DIV_16_5_CTL_SPEC>;
#[doc = "Divider control register (for 16.5 divider)"]
pub mod div_16_5_ctl;
#[doc = "DIV_24_5_CTL register accessor: an alias for `Reg<DIV_24_5_CTL_SPEC>`"]
pub type DIV_24_5_CTL = crate::Reg<div_24_5_ctl::DIV_24_5_CTL_SPEC>;
#[doc = "Divider control register (for 24.5 divider)"]
pub mod div_24_5_ctl;
#[doc = "TR_CTL register accessor: an alias for `Reg<TR_CTL_SPEC>`"]
pub type TR_CTL = crate::Reg<tr_ctl::TR_CTL_SPEC>;
#[doc = "Trigger control register"]
pub mod tr_ctl;
