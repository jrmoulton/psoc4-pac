#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x84 - GPIO port registers"]
    pub prt0: PRT,
    _reserved1: [u8; 0x7c],
    #[doc = "0x100..0x184 - GPIO port registers"]
    pub prt1: PRT,
    _reserved2: [u8; 0x7c],
    #[doc = "0x200..0x284 - GPIO port registers"]
    pub prt2: PRT,
    _reserved3: [u8; 0x7c],
    #[doc = "0x300..0x384 - GPIO port registers"]
    pub prt3: PRT,
    _reserved4: [u8; 0x7c],
    #[doc = "0x400..0x484 - GPIO port registers"]
    pub prt4: PRT,
    _reserved5: [u8; 0x0b7c],
    #[doc = "0x1000 - Interrupt port cause register"]
    pub intr_cause: crate::Reg<intr_cause::INTR_CAUSE_SPEC>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct PRT {
    #[doc = "0x00 - Port output data register"]
    pub dr: crate::Reg<self::prt::dr::DR_SPEC>,
    #[doc = "0x04 - Port IO pad state register"]
    pub ps: crate::Reg<self::prt::ps::PS_SPEC>,
    #[doc = "0x08 - Port configuration register"]
    pub pc: crate::Reg<self::prt::pc::PC_SPEC>,
    #[doc = "0x0c - Port interrupt configuration register"]
    pub intr_cfg: crate::Reg<self::prt::intr_cfg::INTR_CFG_SPEC>,
    #[doc = "0x10 - Port interrupt status register"]
    pub intr: crate::Reg<self::prt::intr::INTR_SPEC>,
    #[doc = "0x14 - Port SIO configuration register"]
    pub sio: crate::Reg<self::prt::sio::SIO_SPEC>,
    #[doc = "0x18 - Port configuration register 2"]
    pub pc2: crate::Reg<self::prt::pc2::PC2_SPEC>,
    _reserved7: [u8; 0x24],
    #[doc = "0x40 - Port output data set register"]
    pub dr_set: crate::Reg<self::prt::dr_set::DR_SET_SPEC>,
    #[doc = "0x44 - Port output data clear register"]
    pub dr_clr: crate::Reg<self::prt::dr_clr::DR_CLR_SPEC>,
    #[doc = "0x48 - Port output data invert register"]
    pub dr_inv: crate::Reg<self::prt::dr_inv::DR_INV_SPEC>,
    _reserved10: [u8; 0x34],
    #[doc = "0x80 - Reference generator configuration register"]
    pub vrefgen: crate::Reg<self::prt::vrefgen::VREFGEN_SPEC>,
}
#[doc = r"Register block"]
#[doc = "GPIO port registers"]
pub mod prt;
#[doc = "INTR_CAUSE register accessor: an alias for `Reg<INTR_CAUSE_SPEC>`"]
pub type INTR_CAUSE = crate::Reg<intr_cause::INTR_CAUSE_SPEC>;
#[doc = "Interrupt port cause register"]
pub mod intr_cause;
