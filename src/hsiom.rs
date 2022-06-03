#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - HSIOM port registers"]
    pub prt0: PRT,
    _reserved1: [u8; 0xfc],
    #[doc = "0x100 - HSIOM port registers"]
    pub prt1: PRT,
    _reserved2: [u8; 0xfc],
    #[doc = "0x200 - HSIOM port registers"]
    pub prt2: PRT,
    _reserved3: [u8; 0xfc],
    #[doc = "0x300 - HSIOM port registers"]
    pub prt3: PRT,
    _reserved4: [u8; 0xfc],
    #[doc = "0x400 - HSIOM port registers"]
    pub prt4: PRT,
    _reserved5: [u8; 0x1bfc],
    #[doc = "0x2000 - Pump control"]
    pub pump_ctl: crate::Reg<pump_ctl::PUMP_CTL_SPEC>,
    _reserved6: [u8; 0xfc],
    #[doc = "0x2100..0x2120 - AMUX splitter cell control"]
    pub amux_split_ctl: [crate::Reg<amux_split_ctl::AMUX_SPLIT_CTL_SPEC>; 8],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct PRT {
    #[doc = "0x00 - Port selection register"]
    pub port_sel: crate::Reg<self::prt::port_sel::PORT_SEL_SPEC>,
}
#[doc = r"Register block"]
#[doc = "HSIOM port registers"]
pub mod prt;
#[doc = "PUMP_CTL register accessor: an alias for `Reg<PUMP_CTL_SPEC>`"]
pub type PUMP_CTL = crate::Reg<pump_ctl::PUMP_CTL_SPEC>;
#[doc = "Pump control"]
pub mod pump_ctl;
#[doc = "AMUX_SPLIT_CTL register accessor: an alias for `Reg<AMUX_SPLIT_CTL_SPEC>`"]
pub type AMUX_SPLIT_CTL = crate::Reg<amux_split_ctl::AMUX_SPLIT_CTL_SPEC>;
#[doc = "AMUX splitter cell control"]
pub mod amux_split_ctl;
