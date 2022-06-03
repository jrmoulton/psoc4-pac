#[doc = "DR register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "Port output data register"]
pub mod dr;
#[doc = "PS register accessor: an alias for `Reg<PS_SPEC>`"]
pub type PS = crate::Reg<ps::PS_SPEC>;
#[doc = "Port IO pad state register"]
pub mod ps;
#[doc = "PC register accessor: an alias for `Reg<PC_SPEC>`"]
pub type PC = crate::Reg<pc::PC_SPEC>;
#[doc = "Port configuration register"]
pub mod pc;
#[doc = "INTR_CFG register accessor: an alias for `Reg<INTR_CFG_SPEC>`"]
pub type INTR_CFG = crate::Reg<intr_cfg::INTR_CFG_SPEC>;
#[doc = "Port interrupt configuration register"]
pub mod intr_cfg;
#[doc = "INTR register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Port interrupt status register"]
pub mod intr;
#[doc = "SIO register accessor: an alias for `Reg<SIO_SPEC>`"]
pub type SIO = crate::Reg<sio::SIO_SPEC>;
#[doc = "Port SIO configuration register"]
pub mod sio;
#[doc = "PC2 register accessor: an alias for `Reg<PC2_SPEC>`"]
pub type PC2 = crate::Reg<pc2::PC2_SPEC>;
#[doc = "Port configuration register 2"]
pub mod pc2;
#[doc = "DR_SET register accessor: an alias for `Reg<DR_SET_SPEC>`"]
pub type DR_SET = crate::Reg<dr_set::DR_SET_SPEC>;
#[doc = "Port output data set register"]
pub mod dr_set;
#[doc = "DR_CLR register accessor: an alias for `Reg<DR_CLR_SPEC>`"]
pub type DR_CLR = crate::Reg<dr_clr::DR_CLR_SPEC>;
#[doc = "Port output data clear register"]
pub mod dr_clr;
#[doc = "DR_INV register accessor: an alias for `Reg<DR_INV_SPEC>`"]
pub type DR_INV = crate::Reg<dr_inv::DR_INV_SPEC>;
#[doc = "Port output data invert register"]
pub mod dr_inv;
#[doc = "VREFGEN register accessor: an alias for `Reg<VREFGEN_SPEC>`"]
pub type VREFGEN = crate::Reg<vrefgen::VREFGEN_SPEC>;
#[doc = "Reference generator configuration register"]
pub mod vrefgen;
