#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x4030_0000],
    #[doc = "0x40300000 - Global CTB IP and power control"]
    pub ctbm_ctrl: crate::Reg<ctbm_ctrl::CTBM_CTRL_SPEC>,
    #[doc = "0x40300004 - Opamp and resistor control"]
    pub oa_res_ctrl: crate::Reg<oa_res_ctrl::OA_RES_CTRL_SPEC>,
    _reserved2: [u8; 0x0f00],
    #[doc = "0x40300f08 - Opamp Compenation Capacitor Trim"]
    pub oa_comp_trim: crate::Reg<oa_comp_trim::OA_COMP_TRIM_SPEC>,
}
#[doc = "CTBM_CTRL register accessor: an alias for `Reg<CTBM_CTRL_SPEC>`"]
pub type CTBM_CTRL = crate::Reg<ctbm_ctrl::CTBM_CTRL_SPEC>;
#[doc = "Global CTB IP and power control"]
pub mod ctbm_ctrl;
#[doc = "OA_RES_CTRL register accessor: an alias for `Reg<OA_RES_CTRL_SPEC>`"]
pub type OA_RES_CTRL = crate::Reg<oa_res_ctrl::OA_RES_CTRL_SPEC>;
#[doc = "Opamp and resistor control"]
pub mod oa_res_ctrl;
#[doc = "OA_COMP_TRIM register accessor: an alias for `Reg<OA_COMP_TRIM_SPEC>`"]
pub type OA_COMP_TRIM = crate::Reg<oa_comp_trim::OA_COMP_TRIM_SPEC>;
#[doc = "Opamp Compenation Capacitor Trim"]
pub mod oa_comp_trim;
