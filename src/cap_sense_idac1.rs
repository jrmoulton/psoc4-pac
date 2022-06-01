#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x4028_0004],
    #[doc = "0x40280004 - CSD IDAC Polarity"]
    pub csd_idac_polarity: crate::Reg<csd_idac_polarity::CSD_IDAC_POLARITY_SPEC>,
    #[doc = "0x40280008 - CSD IDAC register"]
    pub csd_idac: crate::Reg<csd_idac::CSD_IDAC_SPEC>,
    _reserved2: [u8; 0xfef4],
    #[doc = "0x4028ff00 - CSD IDAC TRIM1"]
    pub csd_trim1: crate::Reg<csd_trim1::CSD_TRIM1_SPEC>,
    #[doc = "0x4028ff04 - CSD IDAC TRIM2"]
    pub csd_trim2: crate::Reg<csd_trim2::CSD_TRIM2_SPEC>,
}
#[doc = "CSD_IDAC_POLARITY register accessor: an alias for `Reg<CSD_IDAC_POLARITY_SPEC>`"]
pub type CSD_IDAC_POLARITY = crate::Reg<csd_idac_polarity::CSD_IDAC_POLARITY_SPEC>;
#[doc = "CSD IDAC Polarity"]
pub mod csd_idac_polarity;
#[doc = "CSD_IDAC register accessor: an alias for `Reg<CSD_IDAC_SPEC>`"]
pub type CSD_IDAC = crate::Reg<csd_idac::CSD_IDAC_SPEC>;
#[doc = "CSD IDAC register"]
pub mod csd_idac;
#[doc = "CSD_TRIM1 register accessor: an alias for `Reg<CSD_TRIM1_SPEC>`"]
pub type CSD_TRIM1 = crate::Reg<csd_trim1::CSD_TRIM1_SPEC>;
#[doc = "CSD IDAC TRIM1"]
pub mod csd_trim1;
#[doc = "CSD_TRIM2 register accessor: an alias for `Reg<CSD_TRIM2_SPEC>`"]
pub type CSD_TRIM2 = crate::Reg<csd_trim2::CSD_TRIM2_SPEC>;
#[doc = "CSD IDAC TRIM2"]
pub mod csd_trim2;
