#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration register"]
    pub config: crate::Reg<config::CONFIG_SPEC>,
    #[doc = "0x04 - SYSCALL control register"]
    pub sysreq: crate::Reg<sysreq::SYSREQ_SPEC>,
    #[doc = "0x08 - SYSARG control register"]
    pub sysarg: crate::Reg<sysarg::SYSARG_SPEC>,
    #[doc = "0x0c - Protection control register"]
    pub protection: crate::Reg<protection::PROTECTION_SPEC>,
    #[doc = "0x10 - ROM privilege register"]
    pub priv_rom: crate::Reg<priv_rom::PRIV_ROM_SPEC>,
    #[doc = "0x14 - RAM privilege register"]
    pub priv_ram: crate::Reg<priv_ram::PRIV_RAM_SPEC>,
    #[doc = "0x18 - Flash privilege register"]
    pub priv_flash: crate::Reg<priv_flash::PRIV_FLASH_SPEC>,
    #[doc = "0x1c - Wounding register"]
    pub wounding: crate::Reg<wounding::WOUNDING_SPEC>,
    #[doc = "0x20 - Interrupt multiplexer select register"]
    pub int_sel: crate::Reg<int_sel::INT_SEL_SPEC>,
    #[doc = "0x24 - DSI interrupt pulse mode register"]
    pub int_mode: crate::Reg<int_mode::INT_MODE_SPEC>,
    #[doc = "0x28 - DSI NMI pulse mode register"]
    pub nmi_mode: crate::Reg<nmi_mode::NMI_MODE_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0x30 - FLASH control register"]
    pub flash_ctl: crate::Reg<flash_ctl::FLASH_CTL_SPEC>,
    #[doc = "0x34 - ROM control register"]
    pub rom_ctl: crate::Reg<rom_ctl::ROM_CTL_SPEC>,
    #[doc = "0x38 - RAM control register"]
    pub ram_ctl: crate::Reg<ram_ctl::RAM_CTL_SPEC>,
    #[doc = "0x3c - DMA controller register"]
    pub dmac_ctl: crate::Reg<dmac_ctl::DMAC_CTL_SPEC>,
    _reserved15: [u8; 0x60],
    #[doc = "0xa0 - RAM 1 privilege register"]
    pub priv_ram1: crate::Reg<priv_ram1::PRIV_RAM1_SPEC>,
    #[doc = "0xa4 - RAM 1 control register"]
    pub ram1_ctl: crate::Reg<ram1_ctl::RAM1_CTL_SPEC>,
    _reserved17: [u8; 0x08],
    #[doc = "0xb0 - MTB control register"]
    pub mtb_ctl: crate::Reg<mtb_ctl::MTB_CTL_SPEC>,
    _reserved18: [u8; 0x4c],
    #[doc = "0x100..0x160 - Slave control register"]
    pub sl_ctl: [crate::Reg<sl_ctl::SL_CTL_SPEC>; 24],
}
#[doc = "CONFIG register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration register"]
pub mod config;
#[doc = "SYSREQ register accessor: an alias for `Reg<SYSREQ_SPEC>`"]
pub type SYSREQ = crate::Reg<sysreq::SYSREQ_SPEC>;
#[doc = "SYSCALL control register"]
pub mod sysreq;
#[doc = "SYSARG register accessor: an alias for `Reg<SYSARG_SPEC>`"]
pub type SYSARG = crate::Reg<sysarg::SYSARG_SPEC>;
#[doc = "SYSARG control register"]
pub mod sysarg;
#[doc = "PROTECTION register accessor: an alias for `Reg<PROTECTION_SPEC>`"]
pub type PROTECTION = crate::Reg<protection::PROTECTION_SPEC>;
#[doc = "Protection control register"]
pub mod protection;
#[doc = "PRIV_ROM register accessor: an alias for `Reg<PRIV_ROM_SPEC>`"]
pub type PRIV_ROM = crate::Reg<priv_rom::PRIV_ROM_SPEC>;
#[doc = "ROM privilege register"]
pub mod priv_rom;
#[doc = "PRIV_RAM register accessor: an alias for `Reg<PRIV_RAM_SPEC>`"]
pub type PRIV_RAM = crate::Reg<priv_ram::PRIV_RAM_SPEC>;
#[doc = "RAM privilege register"]
pub mod priv_ram;
#[doc = "PRIV_FLASH register accessor: an alias for `Reg<PRIV_FLASH_SPEC>`"]
pub type PRIV_FLASH = crate::Reg<priv_flash::PRIV_FLASH_SPEC>;
#[doc = "Flash privilege register"]
pub mod priv_flash;
#[doc = "WOUNDING register accessor: an alias for `Reg<WOUNDING_SPEC>`"]
pub type WOUNDING = crate::Reg<wounding::WOUNDING_SPEC>;
#[doc = "Wounding register"]
pub mod wounding;
#[doc = "INT_SEL register accessor: an alias for `Reg<INT_SEL_SPEC>`"]
pub type INT_SEL = crate::Reg<int_sel::INT_SEL_SPEC>;
#[doc = "Interrupt multiplexer select register"]
pub mod int_sel;
#[doc = "INT_MODE register accessor: an alias for `Reg<INT_MODE_SPEC>`"]
pub type INT_MODE = crate::Reg<int_mode::INT_MODE_SPEC>;
#[doc = "DSI interrupt pulse mode register"]
pub mod int_mode;
#[doc = "NMI_MODE register accessor: an alias for `Reg<NMI_MODE_SPEC>`"]
pub type NMI_MODE = crate::Reg<nmi_mode::NMI_MODE_SPEC>;
#[doc = "DSI NMI pulse mode register"]
pub mod nmi_mode;
#[doc = "FLASH_CTL register accessor: an alias for `Reg<FLASH_CTL_SPEC>`"]
pub type FLASH_CTL = crate::Reg<flash_ctl::FLASH_CTL_SPEC>;
#[doc = "FLASH control register"]
pub mod flash_ctl;
#[doc = "ROM_CTL register accessor: an alias for `Reg<ROM_CTL_SPEC>`"]
pub type ROM_CTL = crate::Reg<rom_ctl::ROM_CTL_SPEC>;
#[doc = "ROM control register"]
pub mod rom_ctl;
#[doc = "RAM_CTL register accessor: an alias for `Reg<RAM_CTL_SPEC>`"]
pub type RAM_CTL = crate::Reg<ram_ctl::RAM_CTL_SPEC>;
#[doc = "RAM control register"]
pub mod ram_ctl;
#[doc = "DMAC_CTL register accessor: an alias for `Reg<DMAC_CTL_SPEC>`"]
pub type DMAC_CTL = crate::Reg<dmac_ctl::DMAC_CTL_SPEC>;
#[doc = "DMA controller register"]
pub mod dmac_ctl;
#[doc = "PRIV_RAM1 register accessor: an alias for `Reg<PRIV_RAM1_SPEC>`"]
pub type PRIV_RAM1 = crate::Reg<priv_ram1::PRIV_RAM1_SPEC>;
#[doc = "RAM 1 privilege register"]
pub mod priv_ram1;
#[doc = "RAM1_CTL register accessor: an alias for `Reg<RAM1_CTL_SPEC>`"]
pub type RAM1_CTL = crate::Reg<ram1_ctl::RAM1_CTL_SPEC>;
#[doc = "RAM 1 control register"]
pub mod ram1_ctl;
#[doc = "MTB_CTL register accessor: an alias for `Reg<MTB_CTL_SPEC>`"]
pub type MTB_CTL = crate::Reg<mtb_ctl::MTB_CTL_SPEC>;
#[doc = "MTB control register"]
pub mod mtb_ctl;
#[doc = "SL_CTL register accessor: an alias for `Reg<SL_CTL_SPEC>`"]
pub type SL_CTL = crate::Reg<sl_ctl::SL_CTL_SPEC>;
#[doc = "Slave control register"]
pub mod sl_ctl;
