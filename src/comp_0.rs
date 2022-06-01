#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x402b_0004],
    #[doc = "0x402b0004 - LPCOMP configuration"]
    pub cy_config: crate::Reg<cy_config::CY_CONFIG_SPEC>,
    _reserved1: [u8; 0x08],
    #[doc = "0x402b0010 - LPCOMP interrupt"]
    pub cy_intr: crate::Reg<cy_intr::CY_INTR_SPEC>,
    #[doc = "0x402b0014 - LPCOMP interrupt set register"]
    pub cy_intr_set: crate::Reg<cy_intr_set::CY_INTR_SET_SPEC>,
    #[doc = "0x402b0018 - LPCOMP interrupt mask"]
    pub cy_intr_mask: crate::Reg<cy_intr_mask::CY_INTR_MASK_SPEC>,
    #[doc = "0x402b001c - LPCOMP interrupt masked"]
    pub cy_intr_masked: crate::Reg<cy_intr_masked::CY_INTR_MASKED_SPEC>,
    _reserved5: [u8; 0xfee0],
    #[doc = "0x402bff00 - LPCOMP trim A"]
    pub cy_trim_a: crate::Reg<cy_trim_a::CY_TRIM_A_SPEC>,
    #[doc = "0x402bff04 - LPCOMP trim B"]
    pub cy_trim_b: crate::Reg<cy_trim_b::CY_TRIM_B_SPEC>,
}
#[doc = "Cy_CONFIG register accessor: an alias for `Reg<CY_CONFIG_SPEC>`"]
pub type CY_CONFIG = crate::Reg<cy_config::CY_CONFIG_SPEC>;
#[doc = "LPCOMP configuration"]
pub mod cy_config;
#[doc = "Cy_INTR register accessor: an alias for `Reg<CY_INTR_SPEC>`"]
pub type CY_INTR = crate::Reg<cy_intr::CY_INTR_SPEC>;
#[doc = "LPCOMP interrupt"]
pub mod cy_intr;
#[doc = "Cy_INTR_SET register accessor: an alias for `Reg<CY_INTR_SET_SPEC>`"]
pub type CY_INTR_SET = crate::Reg<cy_intr_set::CY_INTR_SET_SPEC>;
#[doc = "LPCOMP interrupt set register"]
pub mod cy_intr_set;
#[doc = "Cy_INTR_MASK register accessor: an alias for `Reg<CY_INTR_MASK_SPEC>`"]
pub type CY_INTR_MASK = crate::Reg<cy_intr_mask::CY_INTR_MASK_SPEC>;
#[doc = "LPCOMP interrupt mask"]
pub mod cy_intr_mask;
#[doc = "Cy_INTR_MASKED register accessor: an alias for `Reg<CY_INTR_MASKED_SPEC>`"]
pub type CY_INTR_MASKED = crate::Reg<cy_intr_masked::CY_INTR_MASKED_SPEC>;
#[doc = "LPCOMP interrupt masked"]
pub mod cy_intr_masked;
#[doc = "Cy_TRIM_A register accessor: an alias for `Reg<CY_TRIM_A_SPEC>`"]
pub type CY_TRIM_A = crate::Reg<cy_trim_a::CY_TRIM_A_SPEC>;
#[doc = "LPCOMP trim A"]
pub mod cy_trim_a;
#[doc = "Cy_TRIM_B register accessor: an alias for `Reg<CY_TRIM_B_SPEC>`"]
pub type CY_TRIM_B = crate::Reg<cy_trim_b::CY_TRIM_B_SPEC>;
#[doc = "LPCOMP trim B"]
pub mod cy_trim_b;
