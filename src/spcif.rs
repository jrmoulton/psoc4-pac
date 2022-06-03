#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash/NVL geometry information"]
    pub geometry: crate::Reg<geometry::GEOMETRY_SPEC>,
    _reserved1: [u8; 0x18],
    #[doc = "0x1c - NVL write data register"]
    pub nvl_wr_data: crate::Reg<nvl_wr_data::NVL_WR_DATA_SPEC>,
    _reserved2: [u8; 0x07d0],
    #[doc = "0x7f0 - SPCIF interrupt request register"]
    pub intr: crate::Reg<intr::INTR_SPEC>,
    #[doc = "0x7f4 - SPCIF interrupt set request register"]
    pub intr_set: crate::Reg<intr_set::INTR_SET_SPEC>,
    #[doc = "0x7f8 - SPCIF interrupt mask register"]
    pub intr_mask: crate::Reg<intr_mask::INTR_MASK_SPEC>,
    #[doc = "0x7fc - SPCIF interrupt masked request register"]
    pub intr_masked: crate::Reg<intr_masked::INTR_MASKED_SPEC>,
}
#[doc = "GEOMETRY register accessor: an alias for `Reg<GEOMETRY_SPEC>`"]
pub type GEOMETRY = crate::Reg<geometry::GEOMETRY_SPEC>;
#[doc = "Flash/NVL geometry information"]
pub mod geometry;
#[doc = "NVL_WR_DATA register accessor: an alias for `Reg<NVL_WR_DATA_SPEC>`"]
pub type NVL_WR_DATA = crate::Reg<nvl_wr_data::NVL_WR_DATA_SPEC>;
#[doc = "NVL write data register"]
pub mod nvl_wr_data;
#[doc = "INTR register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "SPCIF interrupt request register"]
pub mod intr;
#[doc = "INTR_SET register accessor: an alias for `Reg<INTR_SET_SPEC>`"]
pub type INTR_SET = crate::Reg<intr_set::INTR_SET_SPEC>;
#[doc = "SPCIF interrupt set request register"]
pub mod intr_set;
#[doc = "INTR_MASK register accessor: an alias for `Reg<INTR_MASK_SPEC>`"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "SPCIF interrupt mask register"]
pub mod intr_mask;
#[doc = "INTR_MASKED register accessor: an alias for `Reg<INTR_MASKED_SPEC>`"]
pub type INTR_MASKED = crate::Reg<intr_masked::INTR_MASKED_SPEC>;
#[doc = "SPCIF interrupt masked request register"]
pub mod intr_masked;
