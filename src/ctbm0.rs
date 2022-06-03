#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - global CTB and power control"]
    pub ctb_ctrl: crate::Reg<ctb_ctrl::CTB_CTRL_SPEC>,
    #[doc = "0x04 - Opamp0 and resistor0 control"]
    pub oa_res0_ctrl: crate::Reg<oa_res0_ctrl::OA_RES0_CTRL_SPEC>,
    #[doc = "0x08 - Opamp1 and resistor1 control"]
    pub oa_res1_ctrl: crate::Reg<oa_res1_ctrl::OA_RES1_CTRL_SPEC>,
    #[doc = "0x0c - Comparator status"]
    pub comp_stat: crate::Reg<comp_stat::COMP_STAT_SPEC>,
    _reserved4: [u8; 0x10],
    #[doc = "0x20 - Interrupt request register"]
    pub intr: crate::Reg<intr::INTR_SPEC>,
    #[doc = "0x24 - Interrupt request set register"]
    pub intr_set: crate::Reg<intr_set::INTR_SET_SPEC>,
    #[doc = "0x28 - Interrupt request mask"]
    pub intr_mask: crate::Reg<intr_mask::INTR_MASK_SPEC>,
    #[doc = "0x2c - Interrupt request masked"]
    pub intr_masked: crate::Reg<intr_masked::INTR_MASKED_SPEC>,
    #[doc = "0x30 - Was 'Analog DfT controls', now used as Risk Mitigation bits (RMP)"]
    pub dft_ctrl: crate::Reg<dft_ctrl::DFT_CTRL_SPEC>,
    _reserved9: [u8; 0x4c],
    #[doc = "0x80 - Opamp0 switch control"]
    pub oa0_sw: crate::Reg<oa0_sw::OA0_SW_SPEC>,
    #[doc = "0x84 - Opamp0 switch control clear"]
    pub oa0_sw_clear: crate::Reg<oa0_sw_clear::OA0_SW_CLEAR_SPEC>,
    #[doc = "0x88 - Opamp1 switch control"]
    pub oa1_sw: crate::Reg<oa1_sw::OA1_SW_SPEC>,
    #[doc = "0x8c - Opamp1 switch control clear"]
    pub oa1_sw_clear: crate::Reg<oa1_sw_clear::OA1_SW_CLEAR_SPEC>,
    _reserved13: [u8; 0x30],
    #[doc = "0xc0 - CTB bus switch control"]
    pub ctb_sw_hw_ctrl: crate::Reg<ctb_sw_hw_ctrl::CTB_SW_HW_CTRL_SPEC>,
    #[doc = "0xc4 - CTB bus switch control status"]
    pub ctb_sw_status: crate::Reg<ctb_sw_status::CTB_SW_STATUS_SPEC>,
    _reserved15: [u8; 0x0e38],
    #[doc = "0xf00 - Opamp0 trim control"]
    pub oa0_offset_trim: crate::Reg<oa0_offset_trim::OA0_OFFSET_TRIM_SPEC>,
    #[doc = "0xf04 - Opamp0 trim control"]
    pub oa0_slope_offset_trim: crate::Reg<oa0_slope_offset_trim::OA0_SLOPE_OFFSET_TRIM_SPEC>,
    #[doc = "0xf08 - Opamp0 trim control"]
    pub oa0_comp_trim: crate::Reg<oa0_comp_trim::OA0_COMP_TRIM_SPEC>,
    #[doc = "0xf0c - Opamp1 trim control"]
    pub oa1_offset_trim: crate::Reg<oa1_offset_trim::OA1_OFFSET_TRIM_SPEC>,
    #[doc = "0xf10 - Opamp1 trim control"]
    pub oa1_slope_offset_trim: crate::Reg<oa1_slope_offset_trim::OA1_SLOPE_OFFSET_TRIM_SPEC>,
    #[doc = "0xf14 - Opamp1 trim control"]
    pub oa1_comp_trim: crate::Reg<oa1_comp_trim::OA1_COMP_TRIM_SPEC>,
}
#[doc = "CTB_CTRL register accessor: an alias for `Reg<CTB_CTRL_SPEC>`"]
pub type CTB_CTRL = crate::Reg<ctb_ctrl::CTB_CTRL_SPEC>;
#[doc = "global CTB and power control"]
pub mod ctb_ctrl;
#[doc = "OA_RES0_CTRL register accessor: an alias for `Reg<OA_RES0_CTRL_SPEC>`"]
pub type OA_RES0_CTRL = crate::Reg<oa_res0_ctrl::OA_RES0_CTRL_SPEC>;
#[doc = "Opamp0 and resistor0 control"]
pub mod oa_res0_ctrl;
#[doc = "OA_RES1_CTRL register accessor: an alias for `Reg<OA_RES1_CTRL_SPEC>`"]
pub type OA_RES1_CTRL = crate::Reg<oa_res1_ctrl::OA_RES1_CTRL_SPEC>;
#[doc = "Opamp1 and resistor1 control"]
pub mod oa_res1_ctrl;
#[doc = "COMP_STAT register accessor: an alias for `Reg<COMP_STAT_SPEC>`"]
pub type COMP_STAT = crate::Reg<comp_stat::COMP_STAT_SPEC>;
#[doc = "Comparator status"]
pub mod comp_stat;
#[doc = "INTR register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Interrupt request register"]
pub mod intr;
#[doc = "INTR_SET register accessor: an alias for `Reg<INTR_SET_SPEC>`"]
pub type INTR_SET = crate::Reg<intr_set::INTR_SET_SPEC>;
#[doc = "Interrupt request set register"]
pub mod intr_set;
#[doc = "INTR_MASK register accessor: an alias for `Reg<INTR_MASK_SPEC>`"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "Interrupt request mask"]
pub mod intr_mask;
#[doc = "INTR_MASKED register accessor: an alias for `Reg<INTR_MASKED_SPEC>`"]
pub type INTR_MASKED = crate::Reg<intr_masked::INTR_MASKED_SPEC>;
#[doc = "Interrupt request masked"]
pub mod intr_masked;
#[doc = "DFT_CTRL register accessor: an alias for `Reg<DFT_CTRL_SPEC>`"]
pub type DFT_CTRL = crate::Reg<dft_ctrl::DFT_CTRL_SPEC>;
#[doc = "Was 'Analog DfT controls', now used as Risk Mitigation bits (RMP)"]
pub mod dft_ctrl;
#[doc = "OA0_SW register accessor: an alias for `Reg<OA0_SW_SPEC>`"]
pub type OA0_SW = crate::Reg<oa0_sw::OA0_SW_SPEC>;
#[doc = "Opamp0 switch control"]
pub mod oa0_sw;
#[doc = "OA0_SW_CLEAR register accessor: an alias for `Reg<OA0_SW_CLEAR_SPEC>`"]
pub type OA0_SW_CLEAR = crate::Reg<oa0_sw_clear::OA0_SW_CLEAR_SPEC>;
#[doc = "Opamp0 switch control clear"]
pub mod oa0_sw_clear;
#[doc = "OA1_SW register accessor: an alias for `Reg<OA1_SW_SPEC>`"]
pub type OA1_SW = crate::Reg<oa1_sw::OA1_SW_SPEC>;
#[doc = "Opamp1 switch control"]
pub mod oa1_sw;
#[doc = "OA1_SW_CLEAR register accessor: an alias for `Reg<OA1_SW_CLEAR_SPEC>`"]
pub type OA1_SW_CLEAR = crate::Reg<oa1_sw_clear::OA1_SW_CLEAR_SPEC>;
#[doc = "Opamp1 switch control clear"]
pub mod oa1_sw_clear;
#[doc = "CTB_SW_HW_CTRL register accessor: an alias for `Reg<CTB_SW_HW_CTRL_SPEC>`"]
pub type CTB_SW_HW_CTRL = crate::Reg<ctb_sw_hw_ctrl::CTB_SW_HW_CTRL_SPEC>;
#[doc = "CTB bus switch control"]
pub mod ctb_sw_hw_ctrl;
#[doc = "CTB_SW_STATUS register accessor: an alias for `Reg<CTB_SW_STATUS_SPEC>`"]
pub type CTB_SW_STATUS = crate::Reg<ctb_sw_status::CTB_SW_STATUS_SPEC>;
#[doc = "CTB bus switch control status"]
pub mod ctb_sw_status;
#[doc = "OA0_OFFSET_TRIM register accessor: an alias for `Reg<OA0_OFFSET_TRIM_SPEC>`"]
pub type OA0_OFFSET_TRIM = crate::Reg<oa0_offset_trim::OA0_OFFSET_TRIM_SPEC>;
#[doc = "Opamp0 trim control"]
pub mod oa0_offset_trim;
#[doc = "OA0_SLOPE_OFFSET_TRIM register accessor: an alias for `Reg<OA0_SLOPE_OFFSET_TRIM_SPEC>`"]
pub type OA0_SLOPE_OFFSET_TRIM = crate::Reg<oa0_slope_offset_trim::OA0_SLOPE_OFFSET_TRIM_SPEC>;
#[doc = "Opamp0 trim control"]
pub mod oa0_slope_offset_trim;
#[doc = "OA0_COMP_TRIM register accessor: an alias for `Reg<OA0_COMP_TRIM_SPEC>`"]
pub type OA0_COMP_TRIM = crate::Reg<oa0_comp_trim::OA0_COMP_TRIM_SPEC>;
#[doc = "Opamp0 trim control"]
pub mod oa0_comp_trim;
#[doc = "OA1_OFFSET_TRIM register accessor: an alias for `Reg<OA1_OFFSET_TRIM_SPEC>`"]
pub type OA1_OFFSET_TRIM = crate::Reg<oa1_offset_trim::OA1_OFFSET_TRIM_SPEC>;
#[doc = "Opamp1 trim control"]
pub mod oa1_offset_trim;
#[doc = "OA1_SLOPE_OFFSET_TRIM register accessor: an alias for `Reg<OA1_SLOPE_OFFSET_TRIM_SPEC>`"]
pub type OA1_SLOPE_OFFSET_TRIM = crate::Reg<oa1_slope_offset_trim::OA1_SLOPE_OFFSET_TRIM_SPEC>;
#[doc = "Opamp1 trim control"]
pub mod oa1_slope_offset_trim;
#[doc = "OA1_COMP_TRIM register accessor: an alias for `Reg<OA1_COMP_TRIM_SPEC>`"]
pub type OA1_COMP_TRIM = crate::Reg<oa1_comp_trim::OA1_COMP_TRIM_SPEC>;
#[doc = "Opamp1 trim control"]
pub mod oa1_comp_trim;
