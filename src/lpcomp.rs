#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ID & Revision"]
    pub id: crate::Reg<id::ID_SPEC>,
    #[doc = "0x04 - LPCOMP Configuration Register"]
    pub config: crate::Reg<config::CONFIG_SPEC>,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - LPCOMP Interrupt request register"]
    pub intr: crate::Reg<intr::INTR_SPEC>,
    #[doc = "0x14 - LPCOMP Interrupt set register"]
    pub intr_set: crate::Reg<intr_set::INTR_SET_SPEC>,
    #[doc = "0x18 - LPCOMP Interrupt request mask"]
    pub intr_mask: crate::Reg<intr_mask::INTR_MASK_SPEC>,
    #[doc = "0x1c - LPCOMP Interrupt request masked"]
    pub intr_masked: crate::Reg<intr_masked::INTR_MASKED_SPEC>,
    _reserved6: [u8; 0xfee0],
    #[doc = "0xff00 - LPCOMP Trim Register"]
    pub trim1: crate::Reg<trim1::TRIM1_SPEC>,
    #[doc = "0xff04 - LPCOMP Trim Register"]
    pub trim2: crate::Reg<trim2::TRIM2_SPEC>,
    #[doc = "0xff08 - LPCOMP Trim Register"]
    pub trim3: crate::Reg<trim3::TRIM3_SPEC>,
    #[doc = "0xff0c - LPCOMP Trim Register"]
    pub trim4: crate::Reg<trim4::TRIM4_SPEC>,
}
#[doc = "ID register accessor: an alias for `Reg<ID_SPEC>`"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "ID & Revision"]
pub mod id;
#[doc = "CONFIG register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "LPCOMP Configuration Register"]
pub mod config;
#[doc = "INTR register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "LPCOMP Interrupt request register"]
pub mod intr;
#[doc = "INTR_SET register accessor: an alias for `Reg<INTR_SET_SPEC>`"]
pub type INTR_SET = crate::Reg<intr_set::INTR_SET_SPEC>;
#[doc = "LPCOMP Interrupt set register"]
pub mod intr_set;
#[doc = "INTR_MASK register accessor: an alias for `Reg<INTR_MASK_SPEC>`"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "LPCOMP Interrupt request mask"]
pub mod intr_mask;
#[doc = "INTR_MASKED register accessor: an alias for `Reg<INTR_MASKED_SPEC>`"]
pub type INTR_MASKED = crate::Reg<intr_masked::INTR_MASKED_SPEC>;
#[doc = "LPCOMP Interrupt request masked"]
pub mod intr_masked;
#[doc = "TRIM1 register accessor: an alias for `Reg<TRIM1_SPEC>`"]
pub type TRIM1 = crate::Reg<trim1::TRIM1_SPEC>;
#[doc = "LPCOMP Trim Register"]
pub mod trim1;
#[doc = "TRIM2 register accessor: an alias for `Reg<TRIM2_SPEC>`"]
pub type TRIM2 = crate::Reg<trim2::TRIM2_SPEC>;
#[doc = "LPCOMP Trim Register"]
pub mod trim2;
#[doc = "TRIM3 register accessor: an alias for `Reg<TRIM3_SPEC>`"]
pub type TRIM3 = crate::Reg<trim3::TRIM3_SPEC>;
#[doc = "LPCOMP Trim Register"]
pub mod trim3;
#[doc = "TRIM4 register accessor: an alias for `Reg<TRIM4_SPEC>`"]
pub type TRIM4 = crate::Reg<trim4::TRIM4_SPEC>;
#[doc = "LPCOMP Trim Register"]
pub mod trim4;
