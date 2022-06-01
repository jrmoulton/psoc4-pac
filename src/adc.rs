#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x403a_0000],
    #[doc = "0x403a0000 - Analog control register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x403a0004 - Sample control register"]
    pub sample_ctrl: crate::Reg<sample_ctrl::SAMPLE_CTRL_SPEC>,
    _reserved2: [u8; 0x08],
    #[doc = "0x403a0010 - Sample time specification ST0 and ST1"]
    pub sample_time01: crate::Reg<sample_time01::SAMPLE_TIME01_SPEC>,
    #[doc = "0x403a0014 - Sample time specification ST2 and ST3"]
    pub sample_time23: crate::Reg<sample_time23::SAMPLE_TIME23_SPEC>,
    #[doc = "0x403a0018 - Global range detect threshold register"]
    pub range_thres: crate::Reg<range_thres::RANGE_THRES_SPEC>,
    #[doc = "0x403a001c - Global range detect mode register"]
    pub range_cond: crate::Reg<range_cond::RANGE_COND_SPEC>,
    #[doc = "0x403a0020 - Enable bits for the channels"]
    pub chan_en: crate::Reg<chan_en::CHAN_EN_SPEC>,
    #[doc = "0x403a0024 - Start control register (firmware trigger)"]
    pub start_ctrl: crate::Reg<start_ctrl::START_CTRL_SPEC>,
    _reserved8: [u8; 0x08],
    #[doc = "0x403a0030 - DFT control register"]
    pub dft_ctrl: crate::Reg<dft_ctrl::DFT_CTRL_SPEC>,
    _reserved9: [u8; 0x4c],
    #[doc = "0x403a0080 - Channel0 configuration register"]
    pub chan_config0: crate::Reg<chan_config0::CHAN_CONFIG0_SPEC>,
    #[doc = "0x403a0084 - Channel1 configuration register"]
    pub chan_config1: crate::Reg<chan_config1::CHAN_CONFIG1_SPEC>,
    #[doc = "0x403a0088 - Channel2 configuration register"]
    pub chan_config2: crate::Reg<chan_config2::CHAN_CONFIG2_SPEC>,
    #[doc = "0x403a008c - Channel3 configuration register"]
    pub chan_config3: crate::Reg<chan_config3::CHAN_CONFIG3_SPEC>,
    #[doc = "0x403a0090 - Channel4 configuration register"]
    pub chan_config4: crate::Reg<chan_config4::CHAN_CONFIG4_SPEC>,
    #[doc = "0x403a0094 - Channel5 configuration register"]
    pub chan_config5: crate::Reg<chan_config5::CHAN_CONFIG5_SPEC>,
    #[doc = "0x403a0098 - Channel6 configuration register"]
    pub chan_config6: crate::Reg<chan_config6::CHAN_CONFIG6_SPEC>,
    #[doc = "0x403a009c - Channel7 configuration register"]
    pub chan_config7: crate::Reg<chan_config7::CHAN_CONFIG7_SPEC>,
    _reserved17: [u8; 0x60],
    #[doc = "0x403a0100 - Channel0 working data register"]
    pub chan_work0: crate::Reg<chan_work0::CHAN_WORK0_SPEC>,
    #[doc = "0x403a0104 - Channel1 working data register"]
    pub chan_work1: crate::Reg<chan_work1::CHAN_WORK1_SPEC>,
    #[doc = "0x403a0108 - Channel2 working data register"]
    pub chan_work2: crate::Reg<chan_work2::CHAN_WORK2_SPEC>,
    #[doc = "0x403a010c - Channel3 working data register"]
    pub chan_work3: crate::Reg<chan_work3::CHAN_WORK3_SPEC>,
    #[doc = "0x403a0110 - Channel4 working data register"]
    pub chan_work4: crate::Reg<chan_work4::CHAN_WORK4_SPEC>,
    #[doc = "0x403a0114 - Channel5 working data register"]
    pub chan_work5: crate::Reg<chan_work5::CHAN_WORK5_SPEC>,
    #[doc = "0x403a0118 - Channel6 working data register"]
    pub chan_work6: crate::Reg<chan_work6::CHAN_WORK6_SPEC>,
    #[doc = "0x403a011c - Channel7 working data register"]
    pub chan_work7: crate::Reg<chan_work7::CHAN_WORK7_SPEC>,
    _reserved25: [u8; 0x60],
    #[doc = "0x403a0180 - Channel0 result data register"]
    pub chan_result0: crate::Reg<chan_result0::CHAN_RESULT0_SPEC>,
    #[doc = "0x403a0184 - Channel1 result data register"]
    pub chan_result1: crate::Reg<chan_result1::CHAN_RESULT1_SPEC>,
    #[doc = "0x403a0188 - Channel2 result data register"]
    pub chan_result2: crate::Reg<chan_result2::CHAN_RESULT2_SPEC>,
    #[doc = "0x403a018c - Channel3 result data register"]
    pub chan_result3: crate::Reg<chan_result3::CHAN_RESULT3_SPEC>,
    #[doc = "0x403a0190 - Channel4 result data register"]
    pub chan_result4: crate::Reg<chan_result4::CHAN_RESULT4_SPEC>,
    #[doc = "0x403a0194 - Channel5 result data register"]
    pub chan_result5: crate::Reg<chan_result5::CHAN_RESULT5_SPEC>,
    #[doc = "0x403a0198 - Channel6 result data register"]
    pub chan_result6: crate::Reg<chan_result6::CHAN_RESULT6_SPEC>,
    #[doc = "0x403a019c - Channel7 result data register"]
    pub chan_result7: crate::Reg<chan_result7::CHAN_RESULT7_SPEC>,
    _reserved33: [u8; 0x60],
    #[doc = "0x403a0200 - Channel working data register valid bits"]
    pub chan_work_valid: crate::Reg<chan_work_valid::CHAN_WORK_VALID_SPEC>,
    #[doc = "0x403a0204 - Channel result data register valid bits"]
    pub chan_result_valid: crate::Reg<chan_result_valid::CHAN_RESULT_VALID_SPEC>,
    #[doc = "0x403a0208 - Current status of internal SAR registers (mostly for debug)"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x403a020c - Current averaging status (for debug)"]
    pub avg_stat: crate::Reg<avg_stat::AVG_STAT_SPEC>,
    #[doc = "0x403a0210 - Interrupt request register"]
    pub intr: crate::Reg<intr::INTR_SPEC>,
    #[doc = "0x403a0214 - Not really a register, intended for verification/debug. When read, this register reflects the interrupt request register."]
    pub intr_set: crate::Reg<intr_set::INTR_SET_SPEC>,
    #[doc = "0x403a0218 - Interrupt mask register"]
    pub intr_mask: crate::Reg<intr_mask::INTR_MASK_SPEC>,
    #[doc = "0x403a021c - Interrupt masked request register"]
    pub intr_masked: crate::Reg<intr_masked::INTR_MASKED_SPEC>,
    #[doc = "0x403a0220 - Saturate interrupt request register"]
    pub saturate_intr: crate::Reg<saturate_intr::SATURATE_INTR_SPEC>,
    #[doc = "0x403a0224 - Saturate interrupt set request register"]
    pub saturate_intr_set: crate::Reg<saturate_intr_set::SATURATE_INTR_SET_SPEC>,
    #[doc = "0x403a0228 - Saturate interrupt mask register"]
    pub saturate_intr_mask: crate::Reg<saturate_intr_mask::SATURATE_INTR_MASK_SPEC>,
    #[doc = "0x403a022c - Saturate interrupt masked request register"]
    pub saturate_intr_masked: crate::Reg<saturate_intr_masked::SATURATE_INTR_MASKED_SPEC>,
    #[doc = "0x403a0230 - Range detect interrupt request register"]
    pub range_intr: crate::Reg<range_intr::RANGE_INTR_SPEC>,
    #[doc = "0x403a0234 - Range detect interrupt set request register"]
    pub range_intr_set: crate::Reg<range_intr_set::RANGE_INTR_SET_SPEC>,
    #[doc = "0x403a0238 - Range detect interrupt mask register"]
    pub range_intr_mask: crate::Reg<range_intr_mask::RANGE_INTR_MASK_SPEC>,
    #[doc = "0x403a023c - Range interrupt masked request register"]
    pub range_intr_masked: crate::Reg<range_intr_masked::RANGE_INTR_MASKED_SPEC>,
    #[doc = "0x403a0240 - Interrupt cause register"]
    pub intr_cause: crate::Reg<intr_cause::INTR_CAUSE_SPEC>,
    _reserved50: [u8; 0x3c],
    #[doc = "0x403a0280 - Injection channel configuration register"]
    pub inj_chan_config: crate::Reg<inj_chan_config::INJ_CHAN_CONFIG_SPEC>,
    _reserved51: [u8; 0x0c],
    #[doc = "0x403a0290 - Injection channel result register"]
    pub inj_result: crate::Reg<inj_result::INJ_RESULT_SPEC>,
    _reserved52: [u8; 0x6c],
    #[doc = "0x403a0300 - SARMUX Firmware switch controls"]
    pub mux_switch0: crate::Reg<mux_switch0::MUX_SWITCH0_SPEC>,
    #[doc = "0x403a0304 - SARMUX Firmware switch control clear"]
    pub mux_switch_clear0: crate::Reg<mux_switch_clear0::MUX_SWITCH_CLEAR0_SPEC>,
    #[doc = "0x403a0308 - SARMUX Firmware switch controls"]
    pub mux_switch1: crate::Reg<mux_switch1::MUX_SWITCH1_SPEC>,
    #[doc = "0x403a030c - SARMUX Firmware switch control clear"]
    pub mux_switch_clear1: crate::Reg<mux_switch_clear1::MUX_SWITCH_CLEAR1_SPEC>,
    _reserved56: [u8; 0x30],
    #[doc = "0x403a0340 - SARMUX switch hardware control"]
    pub mux_switch_hw_ctrl: crate::Reg<mux_switch_hw_ctrl::MUX_SWITCH_HW_CTRL_SPEC>,
    _reserved57: [u8; 0x04],
    #[doc = "0x403a0348 - SARMUX switch status"]
    pub mux_switch_status: crate::Reg<mux_switch_status::MUX_SWITCH_STATUS_SPEC>,
    _reserved58: [u8; 0x34],
    #[doc = "0x403a0380 - Switch pump control"]
    pub pump_ctrl: crate::Reg<pump_ctrl::PUMP_CTRL_SPEC>,
    _reserved59: [u8; 0x0b7c],
    #[doc = "0x403a0f00 - Analog trim register"]
    pub ana_trim: crate::Reg<ana_trim::ANA_TRIM_SPEC>,
    #[doc = "0x403a0f04 - SAR wounding register"]
    pub wounding: crate::Reg<wounding::WOUNDING_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Analog control register"]
pub mod ctrl;
#[doc = "SAMPLE_CTRL register accessor: an alias for `Reg<SAMPLE_CTRL_SPEC>`"]
pub type SAMPLE_CTRL = crate::Reg<sample_ctrl::SAMPLE_CTRL_SPEC>;
#[doc = "Sample control register"]
pub mod sample_ctrl;
#[doc = "SAMPLE_TIME01 register accessor: an alias for `Reg<SAMPLE_TIME01_SPEC>`"]
pub type SAMPLE_TIME01 = crate::Reg<sample_time01::SAMPLE_TIME01_SPEC>;
#[doc = "Sample time specification ST0 and ST1"]
pub mod sample_time01;
#[doc = "SAMPLE_TIME23 register accessor: an alias for `Reg<SAMPLE_TIME23_SPEC>`"]
pub type SAMPLE_TIME23 = crate::Reg<sample_time23::SAMPLE_TIME23_SPEC>;
#[doc = "Sample time specification ST2 and ST3"]
pub mod sample_time23;
#[doc = "RANGE_THRES register accessor: an alias for `Reg<RANGE_THRES_SPEC>`"]
pub type RANGE_THRES = crate::Reg<range_thres::RANGE_THRES_SPEC>;
#[doc = "Global range detect threshold register"]
pub mod range_thres;
#[doc = "RANGE_COND register accessor: an alias for `Reg<RANGE_COND_SPEC>`"]
pub type RANGE_COND = crate::Reg<range_cond::RANGE_COND_SPEC>;
#[doc = "Global range detect mode register"]
pub mod range_cond;
#[doc = "CHAN_EN register accessor: an alias for `Reg<CHAN_EN_SPEC>`"]
pub type CHAN_EN = crate::Reg<chan_en::CHAN_EN_SPEC>;
#[doc = "Enable bits for the channels"]
pub mod chan_en;
#[doc = "START_CTRL register accessor: an alias for `Reg<START_CTRL_SPEC>`"]
pub type START_CTRL = crate::Reg<start_ctrl::START_CTRL_SPEC>;
#[doc = "Start control register (firmware trigger)"]
pub mod start_ctrl;
#[doc = "DFT_CTRL register accessor: an alias for `Reg<DFT_CTRL_SPEC>`"]
pub type DFT_CTRL = crate::Reg<dft_ctrl::DFT_CTRL_SPEC>;
#[doc = "DFT control register"]
pub mod dft_ctrl;
#[doc = "CHAN_CONFIG0 register accessor: an alias for `Reg<CHAN_CONFIG0_SPEC>`"]
pub type CHAN_CONFIG0 = crate::Reg<chan_config0::CHAN_CONFIG0_SPEC>;
#[doc = "Channel0 configuration register"]
pub mod chan_config0;
#[doc = "CHAN_CONFIG1 register accessor: an alias for `Reg<CHAN_CONFIG1_SPEC>`"]
pub type CHAN_CONFIG1 = crate::Reg<chan_config1::CHAN_CONFIG1_SPEC>;
#[doc = "Channel1 configuration register"]
pub mod chan_config1;
#[doc = "CHAN_CONFIG2 register accessor: an alias for `Reg<CHAN_CONFIG2_SPEC>`"]
pub type CHAN_CONFIG2 = crate::Reg<chan_config2::CHAN_CONFIG2_SPEC>;
#[doc = "Channel2 configuration register"]
pub mod chan_config2;
#[doc = "CHAN_CONFIG3 register accessor: an alias for `Reg<CHAN_CONFIG3_SPEC>`"]
pub type CHAN_CONFIG3 = crate::Reg<chan_config3::CHAN_CONFIG3_SPEC>;
#[doc = "Channel3 configuration register"]
pub mod chan_config3;
#[doc = "CHAN_CONFIG4 register accessor: an alias for `Reg<CHAN_CONFIG4_SPEC>`"]
pub type CHAN_CONFIG4 = crate::Reg<chan_config4::CHAN_CONFIG4_SPEC>;
#[doc = "Channel4 configuration register"]
pub mod chan_config4;
#[doc = "CHAN_CONFIG5 register accessor: an alias for `Reg<CHAN_CONFIG5_SPEC>`"]
pub type CHAN_CONFIG5 = crate::Reg<chan_config5::CHAN_CONFIG5_SPEC>;
#[doc = "Channel5 configuration register"]
pub mod chan_config5;
#[doc = "CHAN_CONFIG6 register accessor: an alias for `Reg<CHAN_CONFIG6_SPEC>`"]
pub type CHAN_CONFIG6 = crate::Reg<chan_config6::CHAN_CONFIG6_SPEC>;
#[doc = "Channel6 configuration register"]
pub mod chan_config6;
#[doc = "CHAN_CONFIG7 register accessor: an alias for `Reg<CHAN_CONFIG7_SPEC>`"]
pub type CHAN_CONFIG7 = crate::Reg<chan_config7::CHAN_CONFIG7_SPEC>;
#[doc = "Channel7 configuration register"]
pub mod chan_config7;
#[doc = "CHAN_WORK0 register accessor: an alias for `Reg<CHAN_WORK0_SPEC>`"]
pub type CHAN_WORK0 = crate::Reg<chan_work0::CHAN_WORK0_SPEC>;
#[doc = "Channel0 working data register"]
pub mod chan_work0;
#[doc = "CHAN_WORK1 register accessor: an alias for `Reg<CHAN_WORK1_SPEC>`"]
pub type CHAN_WORK1 = crate::Reg<chan_work1::CHAN_WORK1_SPEC>;
#[doc = "Channel1 working data register"]
pub mod chan_work1;
#[doc = "CHAN_WORK2 register accessor: an alias for `Reg<CHAN_WORK2_SPEC>`"]
pub type CHAN_WORK2 = crate::Reg<chan_work2::CHAN_WORK2_SPEC>;
#[doc = "Channel2 working data register"]
pub mod chan_work2;
#[doc = "CHAN_WORK3 register accessor: an alias for `Reg<CHAN_WORK3_SPEC>`"]
pub type CHAN_WORK3 = crate::Reg<chan_work3::CHAN_WORK3_SPEC>;
#[doc = "Channel3 working data register"]
pub mod chan_work3;
#[doc = "CHAN_WORK4 register accessor: an alias for `Reg<CHAN_WORK4_SPEC>`"]
pub type CHAN_WORK4 = crate::Reg<chan_work4::CHAN_WORK4_SPEC>;
#[doc = "Channel4 working data register"]
pub mod chan_work4;
#[doc = "CHAN_WORK5 register accessor: an alias for `Reg<CHAN_WORK5_SPEC>`"]
pub type CHAN_WORK5 = crate::Reg<chan_work5::CHAN_WORK5_SPEC>;
#[doc = "Channel5 working data register"]
pub mod chan_work5;
#[doc = "CHAN_WORK6 register accessor: an alias for `Reg<CHAN_WORK6_SPEC>`"]
pub type CHAN_WORK6 = crate::Reg<chan_work6::CHAN_WORK6_SPEC>;
#[doc = "Channel6 working data register"]
pub mod chan_work6;
#[doc = "CHAN_WORK7 register accessor: an alias for `Reg<CHAN_WORK7_SPEC>`"]
pub type CHAN_WORK7 = crate::Reg<chan_work7::CHAN_WORK7_SPEC>;
#[doc = "Channel7 working data register"]
pub mod chan_work7;
#[doc = "CHAN_RESULT0 register accessor: an alias for `Reg<CHAN_RESULT0_SPEC>`"]
pub type CHAN_RESULT0 = crate::Reg<chan_result0::CHAN_RESULT0_SPEC>;
#[doc = "Channel0 result data register"]
pub mod chan_result0;
#[doc = "CHAN_RESULT1 register accessor: an alias for `Reg<CHAN_RESULT1_SPEC>`"]
pub type CHAN_RESULT1 = crate::Reg<chan_result1::CHAN_RESULT1_SPEC>;
#[doc = "Channel1 result data register"]
pub mod chan_result1;
#[doc = "CHAN_RESULT2 register accessor: an alias for `Reg<CHAN_RESULT2_SPEC>`"]
pub type CHAN_RESULT2 = crate::Reg<chan_result2::CHAN_RESULT2_SPEC>;
#[doc = "Channel2 result data register"]
pub mod chan_result2;
#[doc = "CHAN_RESULT3 register accessor: an alias for `Reg<CHAN_RESULT3_SPEC>`"]
pub type CHAN_RESULT3 = crate::Reg<chan_result3::CHAN_RESULT3_SPEC>;
#[doc = "Channel3 result data register"]
pub mod chan_result3;
#[doc = "CHAN_RESULT4 register accessor: an alias for `Reg<CHAN_RESULT4_SPEC>`"]
pub type CHAN_RESULT4 = crate::Reg<chan_result4::CHAN_RESULT4_SPEC>;
#[doc = "Channel4 result data register"]
pub mod chan_result4;
#[doc = "CHAN_RESULT5 register accessor: an alias for `Reg<CHAN_RESULT5_SPEC>`"]
pub type CHAN_RESULT5 = crate::Reg<chan_result5::CHAN_RESULT5_SPEC>;
#[doc = "Channel5 result data register"]
pub mod chan_result5;
#[doc = "CHAN_RESULT6 register accessor: an alias for `Reg<CHAN_RESULT6_SPEC>`"]
pub type CHAN_RESULT6 = crate::Reg<chan_result6::CHAN_RESULT6_SPEC>;
#[doc = "Channel6 result data register"]
pub mod chan_result6;
#[doc = "CHAN_RESULT7 register accessor: an alias for `Reg<CHAN_RESULT7_SPEC>`"]
pub type CHAN_RESULT7 = crate::Reg<chan_result7::CHAN_RESULT7_SPEC>;
#[doc = "Channel7 result data register"]
pub mod chan_result7;
#[doc = "CHAN_WORK_VALID register accessor: an alias for `Reg<CHAN_WORK_VALID_SPEC>`"]
pub type CHAN_WORK_VALID = crate::Reg<chan_work_valid::CHAN_WORK_VALID_SPEC>;
#[doc = "Channel working data register valid bits"]
pub mod chan_work_valid;
#[doc = "CHAN_RESULT_VALID register accessor: an alias for `Reg<CHAN_RESULT_VALID_SPEC>`"]
pub type CHAN_RESULT_VALID = crate::Reg<chan_result_valid::CHAN_RESULT_VALID_SPEC>;
#[doc = "Channel result data register valid bits"]
pub mod chan_result_valid;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Current status of internal SAR registers (mostly for debug)"]
pub mod status;
#[doc = "AVG_STAT register accessor: an alias for `Reg<AVG_STAT_SPEC>`"]
pub type AVG_STAT = crate::Reg<avg_stat::AVG_STAT_SPEC>;
#[doc = "Current averaging status (for debug)"]
pub mod avg_stat;
#[doc = "INTR register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Interrupt request register"]
pub mod intr;
#[doc = "INTR_SET register accessor: an alias for `Reg<INTR_SET_SPEC>`"]
pub type INTR_SET = crate::Reg<intr_set::INTR_SET_SPEC>;
#[doc = "Not really a register, intended for verification/debug. When read, this register reflects the interrupt request register."]
pub mod intr_set;
#[doc = "INTR_MASK register accessor: an alias for `Reg<INTR_MASK_SPEC>`"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "Interrupt mask register"]
pub mod intr_mask;
#[doc = "INTR_MASKED register accessor: an alias for `Reg<INTR_MASKED_SPEC>`"]
pub type INTR_MASKED = crate::Reg<intr_masked::INTR_MASKED_SPEC>;
#[doc = "Interrupt masked request register"]
pub mod intr_masked;
#[doc = "SATURATE_INTR register accessor: an alias for `Reg<SATURATE_INTR_SPEC>`"]
pub type SATURATE_INTR = crate::Reg<saturate_intr::SATURATE_INTR_SPEC>;
#[doc = "Saturate interrupt request register"]
pub mod saturate_intr;
#[doc = "SATURATE_INTR_SET register accessor: an alias for `Reg<SATURATE_INTR_SET_SPEC>`"]
pub type SATURATE_INTR_SET = crate::Reg<saturate_intr_set::SATURATE_INTR_SET_SPEC>;
#[doc = "Saturate interrupt set request register"]
pub mod saturate_intr_set;
#[doc = "SATURATE_INTR_MASK register accessor: an alias for `Reg<SATURATE_INTR_MASK_SPEC>`"]
pub type SATURATE_INTR_MASK = crate::Reg<saturate_intr_mask::SATURATE_INTR_MASK_SPEC>;
#[doc = "Saturate interrupt mask register"]
pub mod saturate_intr_mask;
#[doc = "SATURATE_INTR_MASKED register accessor: an alias for `Reg<SATURATE_INTR_MASKED_SPEC>`"]
pub type SATURATE_INTR_MASKED = crate::Reg<saturate_intr_masked::SATURATE_INTR_MASKED_SPEC>;
#[doc = "Saturate interrupt masked request register"]
pub mod saturate_intr_masked;
#[doc = "RANGE_INTR register accessor: an alias for `Reg<RANGE_INTR_SPEC>`"]
pub type RANGE_INTR = crate::Reg<range_intr::RANGE_INTR_SPEC>;
#[doc = "Range detect interrupt request register"]
pub mod range_intr;
#[doc = "RANGE_INTR_SET register accessor: an alias for `Reg<RANGE_INTR_SET_SPEC>`"]
pub type RANGE_INTR_SET = crate::Reg<range_intr_set::RANGE_INTR_SET_SPEC>;
#[doc = "Range detect interrupt set request register"]
pub mod range_intr_set;
#[doc = "RANGE_INTR_MASK register accessor: an alias for `Reg<RANGE_INTR_MASK_SPEC>`"]
pub type RANGE_INTR_MASK = crate::Reg<range_intr_mask::RANGE_INTR_MASK_SPEC>;
#[doc = "Range detect interrupt mask register"]
pub mod range_intr_mask;
#[doc = "RANGE_INTR_MASKED register accessor: an alias for `Reg<RANGE_INTR_MASKED_SPEC>`"]
pub type RANGE_INTR_MASKED = crate::Reg<range_intr_masked::RANGE_INTR_MASKED_SPEC>;
#[doc = "Range interrupt masked request register"]
pub mod range_intr_masked;
#[doc = "INTR_CAUSE register accessor: an alias for `Reg<INTR_CAUSE_SPEC>`"]
pub type INTR_CAUSE = crate::Reg<intr_cause::INTR_CAUSE_SPEC>;
#[doc = "Interrupt cause register"]
pub mod intr_cause;
#[doc = "INJ_CHAN_CONFIG register accessor: an alias for `Reg<INJ_CHAN_CONFIG_SPEC>`"]
pub type INJ_CHAN_CONFIG = crate::Reg<inj_chan_config::INJ_CHAN_CONFIG_SPEC>;
#[doc = "Injection channel configuration register"]
pub mod inj_chan_config;
#[doc = "INJ_RESULT register accessor: an alias for `Reg<INJ_RESULT_SPEC>`"]
pub type INJ_RESULT = crate::Reg<inj_result::INJ_RESULT_SPEC>;
#[doc = "Injection channel result register"]
pub mod inj_result;
#[doc = "MUX_SWITCH0 register accessor: an alias for `Reg<MUX_SWITCH0_SPEC>`"]
pub type MUX_SWITCH0 = crate::Reg<mux_switch0::MUX_SWITCH0_SPEC>;
#[doc = "SARMUX Firmware switch controls"]
pub mod mux_switch0;
#[doc = "MUX_SWITCH_CLEAR0 register accessor: an alias for `Reg<MUX_SWITCH_CLEAR0_SPEC>`"]
pub type MUX_SWITCH_CLEAR0 = crate::Reg<mux_switch_clear0::MUX_SWITCH_CLEAR0_SPEC>;
#[doc = "SARMUX Firmware switch control clear"]
pub mod mux_switch_clear0;
#[doc = "MUX_SWITCH1 register accessor: an alias for `Reg<MUX_SWITCH1_SPEC>`"]
pub type MUX_SWITCH1 = crate::Reg<mux_switch1::MUX_SWITCH1_SPEC>;
#[doc = "SARMUX Firmware switch controls"]
pub mod mux_switch1;
#[doc = "MUX_SWITCH_CLEAR1 register accessor: an alias for `Reg<MUX_SWITCH_CLEAR1_SPEC>`"]
pub type MUX_SWITCH_CLEAR1 = crate::Reg<mux_switch_clear1::MUX_SWITCH_CLEAR1_SPEC>;
#[doc = "SARMUX Firmware switch control clear"]
pub mod mux_switch_clear1;
#[doc = "MUX_SWITCH_HW_CTRL register accessor: an alias for `Reg<MUX_SWITCH_HW_CTRL_SPEC>`"]
pub type MUX_SWITCH_HW_CTRL = crate::Reg<mux_switch_hw_ctrl::MUX_SWITCH_HW_CTRL_SPEC>;
#[doc = "SARMUX switch hardware control"]
pub mod mux_switch_hw_ctrl;
#[doc = "MUX_SWITCH_STATUS register accessor: an alias for `Reg<MUX_SWITCH_STATUS_SPEC>`"]
pub type MUX_SWITCH_STATUS = crate::Reg<mux_switch_status::MUX_SWITCH_STATUS_SPEC>;
#[doc = "SARMUX switch status"]
pub mod mux_switch_status;
#[doc = "PUMP_CTRL register accessor: an alias for `Reg<PUMP_CTRL_SPEC>`"]
pub type PUMP_CTRL = crate::Reg<pump_ctrl::PUMP_CTRL_SPEC>;
#[doc = "Switch pump control"]
pub mod pump_ctrl;
#[doc = "ANA_TRIM register accessor: an alias for `Reg<ANA_TRIM_SPEC>`"]
pub type ANA_TRIM = crate::Reg<ana_trim::ANA_TRIM_SPEC>;
#[doc = "Analog trim register"]
pub mod ana_trim;
#[doc = "WOUNDING register accessor: an alias for `Reg<WOUNDING_SPEC>`"]
pub type WOUNDING = crate::Reg<wounding::WOUNDING_SPEC>;
#[doc = "SAR wounding register"]
pub mod wounding;
