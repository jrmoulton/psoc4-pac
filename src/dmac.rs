#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x4010_1000],
    #[doc = "0x40101000 - DMA controller control register"]
    pub dmac_ctl: crate::Reg<dmac_ctl::DMAC_CTL_SPEC>,
    _reserved1: [u8; 0x0c],
    #[doc = "0x40101010 - DMA controller status register"]
    pub dmac_status: crate::Reg<dmac_status::DMAC_STATUS_SPEC>,
    #[doc = "0x40101014 - Source address currently being used by the DMA controller"]
    pub dmac_status_src_addr: crate::Reg<dmac_status_src_addr::DMAC_STATUS_SRC_ADDR_SPEC>,
    #[doc = "0x40101018 - Destination address currently being used by the DMA controller"]
    pub dmac_status_dst_addr: crate::Reg<dmac_status_dst_addr::DMAC_STATUS_DST_ADDR_SPEC>,
    #[doc = "0x4010101c - Channel activation status"]
    pub dmac_status_ch_act: crate::Reg<dmac_status_ch_act::DMAC_STATUS_CH_ACT_SPEC>,
    _reserved5: [u8; 0x60],
    #[doc = "0x40101080 - DMA channel 0 control register"]
    pub dmac_ch_ctl0: crate::Reg<dmac_ch_ctl0::DMAC_CH_CTL0_SPEC>,
    #[doc = "0x40101084 - DMA channel 1 control register"]
    pub dmac_ch_ctl1: crate::Reg<dmac_ch_ctl1::DMAC_CH_CTL1_SPEC>,
    #[doc = "0x40101088 - DMA channel 2 control register"]
    pub dmac_ch_ctl2: crate::Reg<dmac_ch_ctl2::DMAC_CH_CTL2_SPEC>,
    #[doc = "0x4010108c - DMA channel 3 control register"]
    pub dmac_ch_ctl3: crate::Reg<dmac_ch_ctl3::DMAC_CH_CTL3_SPEC>,
    #[doc = "0x40101090 - DMA channel 4 control register"]
    pub dmac_ch_ctl4: crate::Reg<dmac_ch_ctl4::DMAC_CH_CTL4_SPEC>,
    #[doc = "0x40101094 - DMA channel 5 control register"]
    pub dmac_ch_ctl5: crate::Reg<dmac_ch_ctl5::DMAC_CH_CTL5_SPEC>,
    #[doc = "0x40101098 - DMA channel 6 control register"]
    pub dmac_ch_ctl6: crate::Reg<dmac_ch_ctl6::DMAC_CH_CTL6_SPEC>,
    #[doc = "0x4010109c - DMA channel 7 control register"]
    pub dmac_ch_ctl7: crate::Reg<dmac_ch_ctl7::DMAC_CH_CTL7_SPEC>,
    _reserved13: [u8; 0x0750],
    #[doc = "0x401017f0 - Interrupt register"]
    pub dmac_intr: crate::Reg<dmac_intr::DMAC_INTR_SPEC>,
    #[doc = "0x401017f4 - Interrupt set register"]
    pub dmac_intr_set: crate::Reg<dmac_intr_set::DMAC_INTR_SET_SPEC>,
    #[doc = "0x401017f8 - Interrupt mask register"]
    pub dmac_intr_mask: crate::Reg<dmac_intr_mask::DMAC_INTR_MASK_SPEC>,
    #[doc = "0x401017fc - Interrupt masked register"]
    pub dmac_intr_masked: crate::Reg<dmac_intr_masked::DMAC_INTR_MASKED_SPEC>,
    #[doc = "0x40101800 - Descriptor 0 source address location for channel 0"]
    pub dmac_descr0_ping_src: crate::Reg<dmac_descr0_ping_src::DMAC_DESCR0_PING_SRC_SPEC>,
    #[doc = "0x40101804 - Descriptor 0 destination address location for channel 0"]
    pub dmac_descr0_ping_dst: crate::Reg<dmac_descr0_ping_dst::DMAC_DESCR0_PING_DST_SPEC>,
    #[doc = "0x40101808 - Descriptor 0 control register for channel 0"]
    pub dmac_descr0_ping_ctl: crate::Reg<dmac_descr0_ping_ctl::DMAC_DESCR0_PING_CTL_SPEC>,
    #[doc = "0x4010180c - Descriptor 0 status register for channel 0"]
    pub dmac_descr0_ping_status: crate::Reg<dmac_descr0_ping_status::DMAC_DESCR0_PING_STATUS_SPEC>,
    #[doc = "0x40101810 - Descriptor 1 source address location for channel 0"]
    pub dmac_descr0_pong_src: crate::Reg<dmac_descr0_pong_src::DMAC_DESCR0_PONG_SRC_SPEC>,
    #[doc = "0x40101814 - Descriptor 1 destination address location for channel 0"]
    pub dmac_descr0_pong_dst: crate::Reg<dmac_descr0_pong_dst::DMAC_DESCR0_PONG_DST_SPEC>,
    #[doc = "0x40101818 - Descriptor 1 control register for channel 0"]
    pub dmac_descr0_pong_ctl: crate::Reg<dmac_descr0_pong_ctl::DMAC_DESCR0_PONG_CTL_SPEC>,
    #[doc = "0x4010181c - Descriptor 1 status register for channel 0"]
    pub dmac_descr0_pong_status: crate::Reg<dmac_descr0_pong_status::DMAC_DESCR0_PONG_STATUS_SPEC>,
    #[doc = "0x40101820 - Descriptor 0 source address location for channel 1"]
    pub dmac_descr1_ping_src: crate::Reg<dmac_descr1_ping_src::DMAC_DESCR1_PING_SRC_SPEC>,
    #[doc = "0x40101824 - Descriptor 0 destination address location for channel 1"]
    pub dmac_descr1_ping_dst: crate::Reg<dmac_descr1_ping_dst::DMAC_DESCR1_PING_DST_SPEC>,
    #[doc = "0x40101828 - Descriptor 0 control register for channel 1"]
    pub dmac_descr1_ping_ctl: crate::Reg<dmac_descr1_ping_ctl::DMAC_DESCR1_PING_CTL_SPEC>,
    #[doc = "0x4010182c - Descriptor 0 status register for channel 1"]
    pub dmac_descr1_ping_status: crate::Reg<dmac_descr1_ping_status::DMAC_DESCR1_PING_STATUS_SPEC>,
    #[doc = "0x40101830 - Descriptor 1 source address location for channel 1"]
    pub dmac_descr1_pong_src: crate::Reg<dmac_descr1_pong_src::DMAC_DESCR1_PONG_SRC_SPEC>,
    #[doc = "0x40101834 - Descriptor 1 destination address location for channel 1"]
    pub dmac_descr1_pong_dst: crate::Reg<dmac_descr1_pong_dst::DMAC_DESCR1_PONG_DST_SPEC>,
    #[doc = "0x40101838 - Descriptor 1 control register for channel 1"]
    pub dmac_descr1_pong_ctl: crate::Reg<dmac_descr1_pong_ctl::DMAC_DESCR1_PONG_CTL_SPEC>,
    #[doc = "0x4010183c - Descriptor 1 status register for channel 1"]
    pub dmac_descr1_pong_status: crate::Reg<dmac_descr1_pong_status::DMAC_DESCR1_PONG_STATUS_SPEC>,
    #[doc = "0x40101840 - Descriptor 0 source address location for channel 2"]
    pub dmac_descr2_ping_src: crate::Reg<dmac_descr2_ping_src::DMAC_DESCR2_PING_SRC_SPEC>,
    #[doc = "0x40101844 - Descriptor 0 destination address location for channel 2"]
    pub dmac_descr2_ping_dst: crate::Reg<dmac_descr2_ping_dst::DMAC_DESCR2_PING_DST_SPEC>,
    #[doc = "0x40101848 - Descriptor 0 control register for channel 2"]
    pub dmac_descr2_ping_ctl: crate::Reg<dmac_descr2_ping_ctl::DMAC_DESCR2_PING_CTL_SPEC>,
    #[doc = "0x4010184c - Descriptor 0 status register for channel 2"]
    pub dmac_descr2_ping_status: crate::Reg<dmac_descr2_ping_status::DMAC_DESCR2_PING_STATUS_SPEC>,
    #[doc = "0x40101850 - Descriptor 1 source address location for channel 2"]
    pub dmac_descr2_pong_src: crate::Reg<dmac_descr2_pong_src::DMAC_DESCR2_PONG_SRC_SPEC>,
    #[doc = "0x40101854 - Descriptor 1 destination address location for channel 2"]
    pub dmac_descr2_pong_dst: crate::Reg<dmac_descr2_pong_dst::DMAC_DESCR2_PONG_DST_SPEC>,
    #[doc = "0x40101858 - Descriptor 1 control register for channel 2"]
    pub dmac_descr2_pong_ctl: crate::Reg<dmac_descr2_pong_ctl::DMAC_DESCR2_PONG_CTL_SPEC>,
    #[doc = "0x4010185c - Descriptor 1 status register for channel 2"]
    pub dmac_descr2_pong_status: crate::Reg<dmac_descr2_pong_status::DMAC_DESCR2_PONG_STATUS_SPEC>,
    #[doc = "0x40101860 - Descriptor 0 source address location for channel 3"]
    pub dmac_descr3_ping_src: crate::Reg<dmac_descr3_ping_src::DMAC_DESCR3_PING_SRC_SPEC>,
    #[doc = "0x40101864 - Descriptor 0 destination address location for channel 3"]
    pub dmac_descr3_ping_dst: crate::Reg<dmac_descr3_ping_dst::DMAC_DESCR3_PING_DST_SPEC>,
    #[doc = "0x40101868 - Descriptor 0 control register for channel 3"]
    pub dmac_descr3_ping_ctl: crate::Reg<dmac_descr3_ping_ctl::DMAC_DESCR3_PING_CTL_SPEC>,
    #[doc = "0x4010186c - Descriptor 0 status register for channel 3"]
    pub dmac_descr3_ping_status: crate::Reg<dmac_descr3_ping_status::DMAC_DESCR3_PING_STATUS_SPEC>,
    #[doc = "0x40101870 - Descriptor 1 source address location for channel 3"]
    pub dmac_descr3_pong_src: crate::Reg<dmac_descr3_pong_src::DMAC_DESCR3_PONG_SRC_SPEC>,
    #[doc = "0x40101874 - Descriptor 1 destination address location for channel 3"]
    pub dmac_descr3_pong_dst: crate::Reg<dmac_descr3_pong_dst::DMAC_DESCR3_PONG_DST_SPEC>,
    #[doc = "0x40101878 - Descriptor 1 control register for channel 3"]
    pub dmac_descr3_pong_ctl: crate::Reg<dmac_descr3_pong_ctl::DMAC_DESCR3_PONG_CTL_SPEC>,
    #[doc = "0x4010187c - Descriptor 1 status register for channel 3"]
    pub dmac_descr3_pong_status: crate::Reg<dmac_descr3_pong_status::DMAC_DESCR3_PONG_STATUS_SPEC>,
    #[doc = "0x40101880 - Descriptor 0 source address location for channel 4"]
    pub dmac_descr4_ping_src: crate::Reg<dmac_descr4_ping_src::DMAC_DESCR4_PING_SRC_SPEC>,
    #[doc = "0x40101884 - Descriptor 0 destination address location for channel 4"]
    pub dmac_descr4_ping_dst: crate::Reg<dmac_descr4_ping_dst::DMAC_DESCR4_PING_DST_SPEC>,
    #[doc = "0x40101888 - Descriptor 0 control register for channel 4"]
    pub dmac_descr4_ping_ctl: crate::Reg<dmac_descr4_ping_ctl::DMAC_DESCR4_PING_CTL_SPEC>,
    #[doc = "0x4010188c - Descriptor 0 status register for channel 4"]
    pub dmac_descr4_ping_status: crate::Reg<dmac_descr4_ping_status::DMAC_DESCR4_PING_STATUS_SPEC>,
    #[doc = "0x40101890 - Descriptor 1 source address location for channel 4"]
    pub dmac_descr4_pong_src: crate::Reg<dmac_descr4_pong_src::DMAC_DESCR4_PONG_SRC_SPEC>,
    #[doc = "0x40101894 - Descriptor 1 destination address location for channel 4"]
    pub dmac_descr4_pong_dst: crate::Reg<dmac_descr4_pong_dst::DMAC_DESCR4_PONG_DST_SPEC>,
    #[doc = "0x40101898 - Descriptor 1 control register for channel 4"]
    pub dmac_descr4_pong_ctl: crate::Reg<dmac_descr4_pong_ctl::DMAC_DESCR4_PONG_CTL_SPEC>,
    #[doc = "0x4010189c - Descriptor 1 status register for channel 4"]
    pub dmac_descr4_pong_status: crate::Reg<dmac_descr4_pong_status::DMAC_DESCR4_PONG_STATUS_SPEC>,
    #[doc = "0x401018a0 - Descriptor 0 source address location for channel 5"]
    pub dmac_descr5_ping_src: crate::Reg<dmac_descr5_ping_src::DMAC_DESCR5_PING_SRC_SPEC>,
    #[doc = "0x401018a4 - Descriptor 0 destination address location for channel 5"]
    pub dmac_descr5_ping_dst: crate::Reg<dmac_descr5_ping_dst::DMAC_DESCR5_PING_DST_SPEC>,
    #[doc = "0x401018a8 - Descriptor 0 control register for channel 5"]
    pub dmac_descr5_ping_ctl: crate::Reg<dmac_descr5_ping_ctl::DMAC_DESCR5_PING_CTL_SPEC>,
    #[doc = "0x401018ac - Descriptor 0 status register for channel 5"]
    pub dmac_descr5_ping_status: crate::Reg<dmac_descr5_ping_status::DMAC_DESCR5_PING_STATUS_SPEC>,
    #[doc = "0x401018b0 - Descriptor 1 source address location for channel 5"]
    pub dmac_descr5_pong_src: crate::Reg<dmac_descr5_pong_src::DMAC_DESCR5_PONG_SRC_SPEC>,
    #[doc = "0x401018b4 - Descriptor 1 destination address location for channel 5"]
    pub dmac_descr5_pong_dst: crate::Reg<dmac_descr5_pong_dst::DMAC_DESCR5_PONG_DST_SPEC>,
    #[doc = "0x401018b8 - Descriptor 1 control register for channel 5"]
    pub dmac_descr5_pong_ctl: crate::Reg<dmac_descr5_pong_ctl::DMAC_DESCR5_PONG_CTL_SPEC>,
    #[doc = "0x401018bc - Descriptor 1 status register for channel 5"]
    pub dmac_descr5_pong_status: crate::Reg<dmac_descr5_pong_status::DMAC_DESCR5_PONG_STATUS_SPEC>,
    #[doc = "0x401018c0 - Descriptor 0 source address location for channel 6"]
    pub dmac_descr6_ping_src: crate::Reg<dmac_descr6_ping_src::DMAC_DESCR6_PING_SRC_SPEC>,
    #[doc = "0x401018c4 - Descriptor 0 destination address location for channel 6"]
    pub dmac_descr6_ping_dst: crate::Reg<dmac_descr6_ping_dst::DMAC_DESCR6_PING_DST_SPEC>,
    #[doc = "0x401018c8 - Descriptor 0 control register for channel 6"]
    pub dmac_descr6_ping_ctl: crate::Reg<dmac_descr6_ping_ctl::DMAC_DESCR6_PING_CTL_SPEC>,
    #[doc = "0x401018cc - Descriptor 0 status register for channel 6"]
    pub dmac_descr6_ping_status: crate::Reg<dmac_descr6_ping_status::DMAC_DESCR6_PING_STATUS_SPEC>,
    #[doc = "0x401018d0 - Descriptor 1 source address location for channel 6"]
    pub dmac_descr6_pong_src: crate::Reg<dmac_descr6_pong_src::DMAC_DESCR6_PONG_SRC_SPEC>,
    #[doc = "0x401018d4 - Descriptor 1 destination address location for channel 6"]
    pub dmac_descr6_pong_dst: crate::Reg<dmac_descr6_pong_dst::DMAC_DESCR6_PONG_DST_SPEC>,
    #[doc = "0x401018d8 - Descriptor 1 control register for channel 6"]
    pub dmac_descr6_pong_ctl: crate::Reg<dmac_descr6_pong_ctl::DMAC_DESCR6_PONG_CTL_SPEC>,
    #[doc = "0x401018dc - Descriptor 1 status register for channel 6"]
    pub dmac_descr6_pong_status: crate::Reg<dmac_descr6_pong_status::DMAC_DESCR6_PONG_STATUS_SPEC>,
    #[doc = "0x401018e0 - Descriptor 0 source address location for channel 7"]
    pub dmac_descr7_ping_src: crate::Reg<dmac_descr7_ping_src::DMAC_DESCR7_PING_SRC_SPEC>,
    #[doc = "0x401018e4 - Descriptor 0 destination address location for channel 7"]
    pub dmac_descr7_ping_dst: crate::Reg<dmac_descr7_ping_dst::DMAC_DESCR7_PING_DST_SPEC>,
    #[doc = "0x401018e8 - Descriptor 0 control register for channel 7"]
    pub dmac_descr7_ping_ctl: crate::Reg<dmac_descr7_ping_ctl::DMAC_DESCR7_PING_CTL_SPEC>,
    #[doc = "0x401018ec - Descriptor 0 status register for channel 7"]
    pub dmac_descr7_ping_status: crate::Reg<dmac_descr7_ping_status::DMAC_DESCR7_PING_STATUS_SPEC>,
    #[doc = "0x401018f0 - Descriptor 1 source address location for channel 7"]
    pub dmac_descr7_pong_src: crate::Reg<dmac_descr7_pong_src::DMAC_DESCR7_PONG_SRC_SPEC>,
    #[doc = "0x401018f4 - Descriptor 1 destination address location for channel 7"]
    pub dmac_descr7_pong_dst: crate::Reg<dmac_descr7_pong_dst::DMAC_DESCR7_PONG_DST_SPEC>,
    #[doc = "0x401018f8 - Descriptor 1 control register for channel 7"]
    pub dmac_descr7_pong_ctl: crate::Reg<dmac_descr7_pong_ctl::DMAC_DESCR7_PONG_CTL_SPEC>,
    #[doc = "0x401018fc - Descriptor 1 status register for channel 7"]
    pub dmac_descr7_pong_status: crate::Reg<dmac_descr7_pong_status::DMAC_DESCR7_PONG_STATUS_SPEC>,
}
#[doc = "DMAC_CTL register accessor: an alias for `Reg<DMAC_CTL_SPEC>`"]
pub type DMAC_CTL = crate::Reg<dmac_ctl::DMAC_CTL_SPEC>;
#[doc = "DMA controller control register"]
pub mod dmac_ctl;
#[doc = "DMAC_STATUS register accessor: an alias for `Reg<DMAC_STATUS_SPEC>`"]
pub type DMAC_STATUS = crate::Reg<dmac_status::DMAC_STATUS_SPEC>;
#[doc = "DMA controller status register"]
pub mod dmac_status;
#[doc = "DMAC_STATUS_SRC_ADDR register accessor: an alias for `Reg<DMAC_STATUS_SRC_ADDR_SPEC>`"]
pub type DMAC_STATUS_SRC_ADDR = crate::Reg<dmac_status_src_addr::DMAC_STATUS_SRC_ADDR_SPEC>;
#[doc = "Source address currently being used by the DMA controller"]
pub mod dmac_status_src_addr;
#[doc = "DMAC_STATUS_DST_ADDR register accessor: an alias for `Reg<DMAC_STATUS_DST_ADDR_SPEC>`"]
pub type DMAC_STATUS_DST_ADDR = crate::Reg<dmac_status_dst_addr::DMAC_STATUS_DST_ADDR_SPEC>;
#[doc = "Destination address currently being used by the DMA controller"]
pub mod dmac_status_dst_addr;
#[doc = "DMAC_STATUS_CH_ACT register accessor: an alias for `Reg<DMAC_STATUS_CH_ACT_SPEC>`"]
pub type DMAC_STATUS_CH_ACT = crate::Reg<dmac_status_ch_act::DMAC_STATUS_CH_ACT_SPEC>;
#[doc = "Channel activation status"]
pub mod dmac_status_ch_act;
#[doc = "DMAC_CH_CTL0 register accessor: an alias for `Reg<DMAC_CH_CTL0_SPEC>`"]
pub type DMAC_CH_CTL0 = crate::Reg<dmac_ch_ctl0::DMAC_CH_CTL0_SPEC>;
#[doc = "DMA channel 0 control register"]
pub mod dmac_ch_ctl0;
#[doc = "DMAC_CH_CTL1 register accessor: an alias for `Reg<DMAC_CH_CTL1_SPEC>`"]
pub type DMAC_CH_CTL1 = crate::Reg<dmac_ch_ctl1::DMAC_CH_CTL1_SPEC>;
#[doc = "DMA channel 1 control register"]
pub mod dmac_ch_ctl1;
#[doc = "DMAC_CH_CTL2 register accessor: an alias for `Reg<DMAC_CH_CTL2_SPEC>`"]
pub type DMAC_CH_CTL2 = crate::Reg<dmac_ch_ctl2::DMAC_CH_CTL2_SPEC>;
#[doc = "DMA channel 2 control register"]
pub mod dmac_ch_ctl2;
#[doc = "DMAC_CH_CTL3 register accessor: an alias for `Reg<DMAC_CH_CTL3_SPEC>`"]
pub type DMAC_CH_CTL3 = crate::Reg<dmac_ch_ctl3::DMAC_CH_CTL3_SPEC>;
#[doc = "DMA channel 3 control register"]
pub mod dmac_ch_ctl3;
#[doc = "DMAC_CH_CTL4 register accessor: an alias for `Reg<DMAC_CH_CTL4_SPEC>`"]
pub type DMAC_CH_CTL4 = crate::Reg<dmac_ch_ctl4::DMAC_CH_CTL4_SPEC>;
#[doc = "DMA channel 4 control register"]
pub mod dmac_ch_ctl4;
#[doc = "DMAC_CH_CTL5 register accessor: an alias for `Reg<DMAC_CH_CTL5_SPEC>`"]
pub type DMAC_CH_CTL5 = crate::Reg<dmac_ch_ctl5::DMAC_CH_CTL5_SPEC>;
#[doc = "DMA channel 5 control register"]
pub mod dmac_ch_ctl5;
#[doc = "DMAC_CH_CTL6 register accessor: an alias for `Reg<DMAC_CH_CTL6_SPEC>`"]
pub type DMAC_CH_CTL6 = crate::Reg<dmac_ch_ctl6::DMAC_CH_CTL6_SPEC>;
#[doc = "DMA channel 6 control register"]
pub mod dmac_ch_ctl6;
#[doc = "DMAC_CH_CTL7 register accessor: an alias for `Reg<DMAC_CH_CTL7_SPEC>`"]
pub type DMAC_CH_CTL7 = crate::Reg<dmac_ch_ctl7::DMAC_CH_CTL7_SPEC>;
#[doc = "DMA channel 7 control register"]
pub mod dmac_ch_ctl7;
#[doc = "DMAC_INTR register accessor: an alias for `Reg<DMAC_INTR_SPEC>`"]
pub type DMAC_INTR = crate::Reg<dmac_intr::DMAC_INTR_SPEC>;
#[doc = "Interrupt register"]
pub mod dmac_intr;
#[doc = "DMAC_INTR_SET register accessor: an alias for `Reg<DMAC_INTR_SET_SPEC>`"]
pub type DMAC_INTR_SET = crate::Reg<dmac_intr_set::DMAC_INTR_SET_SPEC>;
#[doc = "Interrupt set register"]
pub mod dmac_intr_set;
#[doc = "DMAC_INTR_MASK register accessor: an alias for `Reg<DMAC_INTR_MASK_SPEC>`"]
pub type DMAC_INTR_MASK = crate::Reg<dmac_intr_mask::DMAC_INTR_MASK_SPEC>;
#[doc = "Interrupt mask register"]
pub mod dmac_intr_mask;
#[doc = "DMAC_INTR_MASKED register accessor: an alias for `Reg<DMAC_INTR_MASKED_SPEC>`"]
pub type DMAC_INTR_MASKED = crate::Reg<dmac_intr_masked::DMAC_INTR_MASKED_SPEC>;
#[doc = "Interrupt masked register"]
pub mod dmac_intr_masked;
#[doc = "DMAC_DESCR0_PING_SRC register accessor: an alias for `Reg<DMAC_DESCR0_PING_SRC_SPEC>`"]
pub type DMAC_DESCR0_PING_SRC = crate::Reg<dmac_descr0_ping_src::DMAC_DESCR0_PING_SRC_SPEC>;
#[doc = "Descriptor 0 source address location for channel 0"]
pub mod dmac_descr0_ping_src;
#[doc = "DMAC_DESCR0_PING_DST register accessor: an alias for `Reg<DMAC_DESCR0_PING_DST_SPEC>`"]
pub type DMAC_DESCR0_PING_DST = crate::Reg<dmac_descr0_ping_dst::DMAC_DESCR0_PING_DST_SPEC>;
#[doc = "Descriptor 0 destination address location for channel 0"]
pub mod dmac_descr0_ping_dst;
#[doc = "DMAC_DESCR0_PING_CTL register accessor: an alias for `Reg<DMAC_DESCR0_PING_CTL_SPEC>`"]
pub type DMAC_DESCR0_PING_CTL = crate::Reg<dmac_descr0_ping_ctl::DMAC_DESCR0_PING_CTL_SPEC>;
#[doc = "Descriptor 0 control register for channel 0"]
pub mod dmac_descr0_ping_ctl;
#[doc = "DMAC_DESCR0_PING_STATUS register accessor: an alias for `Reg<DMAC_DESCR0_PING_STATUS_SPEC>`"]
pub type DMAC_DESCR0_PING_STATUS =
    crate::Reg<dmac_descr0_ping_status::DMAC_DESCR0_PING_STATUS_SPEC>;
#[doc = "Descriptor 0 status register for channel 0"]
pub mod dmac_descr0_ping_status;
#[doc = "DMAC_DESCR0_PONG_SRC register accessor: an alias for `Reg<DMAC_DESCR0_PONG_SRC_SPEC>`"]
pub type DMAC_DESCR0_PONG_SRC = crate::Reg<dmac_descr0_pong_src::DMAC_DESCR0_PONG_SRC_SPEC>;
#[doc = "Descriptor 1 source address location for channel 0"]
pub mod dmac_descr0_pong_src;
#[doc = "DMAC_DESCR0_PONG_DST register accessor: an alias for `Reg<DMAC_DESCR0_PONG_DST_SPEC>`"]
pub type DMAC_DESCR0_PONG_DST = crate::Reg<dmac_descr0_pong_dst::DMAC_DESCR0_PONG_DST_SPEC>;
#[doc = "Descriptor 1 destination address location for channel 0"]
pub mod dmac_descr0_pong_dst;
#[doc = "DMAC_DESCR0_PONG_CTL register accessor: an alias for `Reg<DMAC_DESCR0_PONG_CTL_SPEC>`"]
pub type DMAC_DESCR0_PONG_CTL = crate::Reg<dmac_descr0_pong_ctl::DMAC_DESCR0_PONG_CTL_SPEC>;
#[doc = "Descriptor 1 control register for channel 0"]
pub mod dmac_descr0_pong_ctl;
#[doc = "DMAC_DESCR0_PONG_STATUS register accessor: an alias for `Reg<DMAC_DESCR0_PONG_STATUS_SPEC>`"]
pub type DMAC_DESCR0_PONG_STATUS =
    crate::Reg<dmac_descr0_pong_status::DMAC_DESCR0_PONG_STATUS_SPEC>;
#[doc = "Descriptor 1 status register for channel 0"]
pub mod dmac_descr0_pong_status;
#[doc = "DMAC_DESCR1_PING_SRC register accessor: an alias for `Reg<DMAC_DESCR1_PING_SRC_SPEC>`"]
pub type DMAC_DESCR1_PING_SRC = crate::Reg<dmac_descr1_ping_src::DMAC_DESCR1_PING_SRC_SPEC>;
#[doc = "Descriptor 0 source address location for channel 1"]
pub mod dmac_descr1_ping_src;
#[doc = "DMAC_DESCR1_PING_DST register accessor: an alias for `Reg<DMAC_DESCR1_PING_DST_SPEC>`"]
pub type DMAC_DESCR1_PING_DST = crate::Reg<dmac_descr1_ping_dst::DMAC_DESCR1_PING_DST_SPEC>;
#[doc = "Descriptor 0 destination address location for channel 1"]
pub mod dmac_descr1_ping_dst;
#[doc = "DMAC_DESCR1_PING_CTL register accessor: an alias for `Reg<DMAC_DESCR1_PING_CTL_SPEC>`"]
pub type DMAC_DESCR1_PING_CTL = crate::Reg<dmac_descr1_ping_ctl::DMAC_DESCR1_PING_CTL_SPEC>;
#[doc = "Descriptor 0 control register for channel 1"]
pub mod dmac_descr1_ping_ctl;
#[doc = "DMAC_DESCR1_PING_STATUS register accessor: an alias for `Reg<DMAC_DESCR1_PING_STATUS_SPEC>`"]
pub type DMAC_DESCR1_PING_STATUS =
    crate::Reg<dmac_descr1_ping_status::DMAC_DESCR1_PING_STATUS_SPEC>;
#[doc = "Descriptor 0 status register for channel 1"]
pub mod dmac_descr1_ping_status;
#[doc = "DMAC_DESCR1_PONG_SRC register accessor: an alias for `Reg<DMAC_DESCR1_PONG_SRC_SPEC>`"]
pub type DMAC_DESCR1_PONG_SRC = crate::Reg<dmac_descr1_pong_src::DMAC_DESCR1_PONG_SRC_SPEC>;
#[doc = "Descriptor 1 source address location for channel 1"]
pub mod dmac_descr1_pong_src;
#[doc = "DMAC_DESCR1_PONG_DST register accessor: an alias for `Reg<DMAC_DESCR1_PONG_DST_SPEC>`"]
pub type DMAC_DESCR1_PONG_DST = crate::Reg<dmac_descr1_pong_dst::DMAC_DESCR1_PONG_DST_SPEC>;
#[doc = "Descriptor 1 destination address location for channel 1"]
pub mod dmac_descr1_pong_dst;
#[doc = "DMAC_DESCR1_PONG_CTL register accessor: an alias for `Reg<DMAC_DESCR1_PONG_CTL_SPEC>`"]
pub type DMAC_DESCR1_PONG_CTL = crate::Reg<dmac_descr1_pong_ctl::DMAC_DESCR1_PONG_CTL_SPEC>;
#[doc = "Descriptor 1 control register for channel 1"]
pub mod dmac_descr1_pong_ctl;
#[doc = "DMAC_DESCR1_PONG_STATUS register accessor: an alias for `Reg<DMAC_DESCR1_PONG_STATUS_SPEC>`"]
pub type DMAC_DESCR1_PONG_STATUS =
    crate::Reg<dmac_descr1_pong_status::DMAC_DESCR1_PONG_STATUS_SPEC>;
#[doc = "Descriptor 1 status register for channel 1"]
pub mod dmac_descr1_pong_status;
#[doc = "DMAC_DESCR2_PING_SRC register accessor: an alias for `Reg<DMAC_DESCR2_PING_SRC_SPEC>`"]
pub type DMAC_DESCR2_PING_SRC = crate::Reg<dmac_descr2_ping_src::DMAC_DESCR2_PING_SRC_SPEC>;
#[doc = "Descriptor 0 source address location for channel 2"]
pub mod dmac_descr2_ping_src;
#[doc = "DMAC_DESCR2_PING_DST register accessor: an alias for `Reg<DMAC_DESCR2_PING_DST_SPEC>`"]
pub type DMAC_DESCR2_PING_DST = crate::Reg<dmac_descr2_ping_dst::DMAC_DESCR2_PING_DST_SPEC>;
#[doc = "Descriptor 0 destination address location for channel 2"]
pub mod dmac_descr2_ping_dst;
#[doc = "DMAC_DESCR2_PING_CTL register accessor: an alias for `Reg<DMAC_DESCR2_PING_CTL_SPEC>`"]
pub type DMAC_DESCR2_PING_CTL = crate::Reg<dmac_descr2_ping_ctl::DMAC_DESCR2_PING_CTL_SPEC>;
#[doc = "Descriptor 0 control register for channel 2"]
pub mod dmac_descr2_ping_ctl;
#[doc = "DMAC_DESCR2_PING_STATUS register accessor: an alias for `Reg<DMAC_DESCR2_PING_STATUS_SPEC>`"]
pub type DMAC_DESCR2_PING_STATUS =
    crate::Reg<dmac_descr2_ping_status::DMAC_DESCR2_PING_STATUS_SPEC>;
#[doc = "Descriptor 0 status register for channel 2"]
pub mod dmac_descr2_ping_status;
#[doc = "DMAC_DESCR2_PONG_SRC register accessor: an alias for `Reg<DMAC_DESCR2_PONG_SRC_SPEC>`"]
pub type DMAC_DESCR2_PONG_SRC = crate::Reg<dmac_descr2_pong_src::DMAC_DESCR2_PONG_SRC_SPEC>;
#[doc = "Descriptor 1 source address location for channel 2"]
pub mod dmac_descr2_pong_src;
#[doc = "DMAC_DESCR2_PONG_DST register accessor: an alias for `Reg<DMAC_DESCR2_PONG_DST_SPEC>`"]
pub type DMAC_DESCR2_PONG_DST = crate::Reg<dmac_descr2_pong_dst::DMAC_DESCR2_PONG_DST_SPEC>;
#[doc = "Descriptor 1 destination address location for channel 2"]
pub mod dmac_descr2_pong_dst;
#[doc = "DMAC_DESCR2_PONG_CTL register accessor: an alias for `Reg<DMAC_DESCR2_PONG_CTL_SPEC>`"]
pub type DMAC_DESCR2_PONG_CTL = crate::Reg<dmac_descr2_pong_ctl::DMAC_DESCR2_PONG_CTL_SPEC>;
#[doc = "Descriptor 1 control register for channel 2"]
pub mod dmac_descr2_pong_ctl;
#[doc = "DMAC_DESCR2_PONG_STATUS register accessor: an alias for `Reg<DMAC_DESCR2_PONG_STATUS_SPEC>`"]
pub type DMAC_DESCR2_PONG_STATUS =
    crate::Reg<dmac_descr2_pong_status::DMAC_DESCR2_PONG_STATUS_SPEC>;
#[doc = "Descriptor 1 status register for channel 2"]
pub mod dmac_descr2_pong_status;
#[doc = "DMAC_DESCR3_PING_SRC register accessor: an alias for `Reg<DMAC_DESCR3_PING_SRC_SPEC>`"]
pub type DMAC_DESCR3_PING_SRC = crate::Reg<dmac_descr3_ping_src::DMAC_DESCR3_PING_SRC_SPEC>;
#[doc = "Descriptor 0 source address location for channel 3"]
pub mod dmac_descr3_ping_src;
#[doc = "DMAC_DESCR3_PING_DST register accessor: an alias for `Reg<DMAC_DESCR3_PING_DST_SPEC>`"]
pub type DMAC_DESCR3_PING_DST = crate::Reg<dmac_descr3_ping_dst::DMAC_DESCR3_PING_DST_SPEC>;
#[doc = "Descriptor 0 destination address location for channel 3"]
pub mod dmac_descr3_ping_dst;
#[doc = "DMAC_DESCR3_PING_CTL register accessor: an alias for `Reg<DMAC_DESCR3_PING_CTL_SPEC>`"]
pub type DMAC_DESCR3_PING_CTL = crate::Reg<dmac_descr3_ping_ctl::DMAC_DESCR3_PING_CTL_SPEC>;
#[doc = "Descriptor 0 control register for channel 3"]
pub mod dmac_descr3_ping_ctl;
#[doc = "DMAC_DESCR3_PING_STATUS register accessor: an alias for `Reg<DMAC_DESCR3_PING_STATUS_SPEC>`"]
pub type DMAC_DESCR3_PING_STATUS =
    crate::Reg<dmac_descr3_ping_status::DMAC_DESCR3_PING_STATUS_SPEC>;
#[doc = "Descriptor 0 status register for channel 3"]
pub mod dmac_descr3_ping_status;
#[doc = "DMAC_DESCR3_PONG_SRC register accessor: an alias for `Reg<DMAC_DESCR3_PONG_SRC_SPEC>`"]
pub type DMAC_DESCR3_PONG_SRC = crate::Reg<dmac_descr3_pong_src::DMAC_DESCR3_PONG_SRC_SPEC>;
#[doc = "Descriptor 1 source address location for channel 3"]
pub mod dmac_descr3_pong_src;
#[doc = "DMAC_DESCR3_PONG_DST register accessor: an alias for `Reg<DMAC_DESCR3_PONG_DST_SPEC>`"]
pub type DMAC_DESCR3_PONG_DST = crate::Reg<dmac_descr3_pong_dst::DMAC_DESCR3_PONG_DST_SPEC>;
#[doc = "Descriptor 1 destination address location for channel 3"]
pub mod dmac_descr3_pong_dst;
#[doc = "DMAC_DESCR3_PONG_CTL register accessor: an alias for `Reg<DMAC_DESCR3_PONG_CTL_SPEC>`"]
pub type DMAC_DESCR3_PONG_CTL = crate::Reg<dmac_descr3_pong_ctl::DMAC_DESCR3_PONG_CTL_SPEC>;
#[doc = "Descriptor 1 control register for channel 3"]
pub mod dmac_descr3_pong_ctl;
#[doc = "DMAC_DESCR3_PONG_STATUS register accessor: an alias for `Reg<DMAC_DESCR3_PONG_STATUS_SPEC>`"]
pub type DMAC_DESCR3_PONG_STATUS =
    crate::Reg<dmac_descr3_pong_status::DMAC_DESCR3_PONG_STATUS_SPEC>;
#[doc = "Descriptor 1 status register for channel 3"]
pub mod dmac_descr3_pong_status;
#[doc = "DMAC_DESCR4_PING_SRC register accessor: an alias for `Reg<DMAC_DESCR4_PING_SRC_SPEC>`"]
pub type DMAC_DESCR4_PING_SRC = crate::Reg<dmac_descr4_ping_src::DMAC_DESCR4_PING_SRC_SPEC>;
#[doc = "Descriptor 0 source address location for channel 4"]
pub mod dmac_descr4_ping_src;
#[doc = "DMAC_DESCR4_PING_DST register accessor: an alias for `Reg<DMAC_DESCR4_PING_DST_SPEC>`"]
pub type DMAC_DESCR4_PING_DST = crate::Reg<dmac_descr4_ping_dst::DMAC_DESCR4_PING_DST_SPEC>;
#[doc = "Descriptor 0 destination address location for channel 4"]
pub mod dmac_descr4_ping_dst;
#[doc = "DMAC_DESCR4_PING_CTL register accessor: an alias for `Reg<DMAC_DESCR4_PING_CTL_SPEC>`"]
pub type DMAC_DESCR4_PING_CTL = crate::Reg<dmac_descr4_ping_ctl::DMAC_DESCR4_PING_CTL_SPEC>;
#[doc = "Descriptor 0 control register for channel 4"]
pub mod dmac_descr4_ping_ctl;
#[doc = "DMAC_DESCR4_PING_STATUS register accessor: an alias for `Reg<DMAC_DESCR4_PING_STATUS_SPEC>`"]
pub type DMAC_DESCR4_PING_STATUS =
    crate::Reg<dmac_descr4_ping_status::DMAC_DESCR4_PING_STATUS_SPEC>;
#[doc = "Descriptor 0 status register for channel 4"]
pub mod dmac_descr4_ping_status;
#[doc = "DMAC_DESCR4_PONG_SRC register accessor: an alias for `Reg<DMAC_DESCR4_PONG_SRC_SPEC>`"]
pub type DMAC_DESCR4_PONG_SRC = crate::Reg<dmac_descr4_pong_src::DMAC_DESCR4_PONG_SRC_SPEC>;
#[doc = "Descriptor 1 source address location for channel 4"]
pub mod dmac_descr4_pong_src;
#[doc = "DMAC_DESCR4_PONG_DST register accessor: an alias for `Reg<DMAC_DESCR4_PONG_DST_SPEC>`"]
pub type DMAC_DESCR4_PONG_DST = crate::Reg<dmac_descr4_pong_dst::DMAC_DESCR4_PONG_DST_SPEC>;
#[doc = "Descriptor 1 destination address location for channel 4"]
pub mod dmac_descr4_pong_dst;
#[doc = "DMAC_DESCR4_PONG_CTL register accessor: an alias for `Reg<DMAC_DESCR4_PONG_CTL_SPEC>`"]
pub type DMAC_DESCR4_PONG_CTL = crate::Reg<dmac_descr4_pong_ctl::DMAC_DESCR4_PONG_CTL_SPEC>;
#[doc = "Descriptor 1 control register for channel 4"]
pub mod dmac_descr4_pong_ctl;
#[doc = "DMAC_DESCR4_PONG_STATUS register accessor: an alias for `Reg<DMAC_DESCR4_PONG_STATUS_SPEC>`"]
pub type DMAC_DESCR4_PONG_STATUS =
    crate::Reg<dmac_descr4_pong_status::DMAC_DESCR4_PONG_STATUS_SPEC>;
#[doc = "Descriptor 1 status register for channel 4"]
pub mod dmac_descr4_pong_status;
#[doc = "DMAC_DESCR5_PING_SRC register accessor: an alias for `Reg<DMAC_DESCR5_PING_SRC_SPEC>`"]
pub type DMAC_DESCR5_PING_SRC = crate::Reg<dmac_descr5_ping_src::DMAC_DESCR5_PING_SRC_SPEC>;
#[doc = "Descriptor 0 source address location for channel 5"]
pub mod dmac_descr5_ping_src;
#[doc = "DMAC_DESCR5_PING_DST register accessor: an alias for `Reg<DMAC_DESCR5_PING_DST_SPEC>`"]
pub type DMAC_DESCR5_PING_DST = crate::Reg<dmac_descr5_ping_dst::DMAC_DESCR5_PING_DST_SPEC>;
#[doc = "Descriptor 0 destination address location for channel 5"]
pub mod dmac_descr5_ping_dst;
#[doc = "DMAC_DESCR5_PING_CTL register accessor: an alias for `Reg<DMAC_DESCR5_PING_CTL_SPEC>`"]
pub type DMAC_DESCR5_PING_CTL = crate::Reg<dmac_descr5_ping_ctl::DMAC_DESCR5_PING_CTL_SPEC>;
#[doc = "Descriptor 0 control register for channel 5"]
pub mod dmac_descr5_ping_ctl;
#[doc = "DMAC_DESCR5_PING_STATUS register accessor: an alias for `Reg<DMAC_DESCR5_PING_STATUS_SPEC>`"]
pub type DMAC_DESCR5_PING_STATUS =
    crate::Reg<dmac_descr5_ping_status::DMAC_DESCR5_PING_STATUS_SPEC>;
#[doc = "Descriptor 0 status register for channel 5"]
pub mod dmac_descr5_ping_status;
#[doc = "DMAC_DESCR5_PONG_SRC register accessor: an alias for `Reg<DMAC_DESCR5_PONG_SRC_SPEC>`"]
pub type DMAC_DESCR5_PONG_SRC = crate::Reg<dmac_descr5_pong_src::DMAC_DESCR5_PONG_SRC_SPEC>;
#[doc = "Descriptor 1 source address location for channel 5"]
pub mod dmac_descr5_pong_src;
#[doc = "DMAC_DESCR5_PONG_DST register accessor: an alias for `Reg<DMAC_DESCR5_PONG_DST_SPEC>`"]
pub type DMAC_DESCR5_PONG_DST = crate::Reg<dmac_descr5_pong_dst::DMAC_DESCR5_PONG_DST_SPEC>;
#[doc = "Descriptor 1 destination address location for channel 5"]
pub mod dmac_descr5_pong_dst;
#[doc = "DMAC_DESCR5_PONG_CTL register accessor: an alias for `Reg<DMAC_DESCR5_PONG_CTL_SPEC>`"]
pub type DMAC_DESCR5_PONG_CTL = crate::Reg<dmac_descr5_pong_ctl::DMAC_DESCR5_PONG_CTL_SPEC>;
#[doc = "Descriptor 1 control register for channel 5"]
pub mod dmac_descr5_pong_ctl;
#[doc = "DMAC_DESCR5_PONG_STATUS register accessor: an alias for `Reg<DMAC_DESCR5_PONG_STATUS_SPEC>`"]
pub type DMAC_DESCR5_PONG_STATUS =
    crate::Reg<dmac_descr5_pong_status::DMAC_DESCR5_PONG_STATUS_SPEC>;
#[doc = "Descriptor 1 status register for channel 5"]
pub mod dmac_descr5_pong_status;
#[doc = "DMAC_DESCR6_PING_SRC register accessor: an alias for `Reg<DMAC_DESCR6_PING_SRC_SPEC>`"]
pub type DMAC_DESCR6_PING_SRC = crate::Reg<dmac_descr6_ping_src::DMAC_DESCR6_PING_SRC_SPEC>;
#[doc = "Descriptor 0 source address location for channel 6"]
pub mod dmac_descr6_ping_src;
#[doc = "DMAC_DESCR6_PING_DST register accessor: an alias for `Reg<DMAC_DESCR6_PING_DST_SPEC>`"]
pub type DMAC_DESCR6_PING_DST = crate::Reg<dmac_descr6_ping_dst::DMAC_DESCR6_PING_DST_SPEC>;
#[doc = "Descriptor 0 destination address location for channel 6"]
pub mod dmac_descr6_ping_dst;
#[doc = "DMAC_DESCR6_PING_CTL register accessor: an alias for `Reg<DMAC_DESCR6_PING_CTL_SPEC>`"]
pub type DMAC_DESCR6_PING_CTL = crate::Reg<dmac_descr6_ping_ctl::DMAC_DESCR6_PING_CTL_SPEC>;
#[doc = "Descriptor 0 control register for channel 6"]
pub mod dmac_descr6_ping_ctl;
#[doc = "DMAC_DESCR6_PING_STATUS register accessor: an alias for `Reg<DMAC_DESCR6_PING_STATUS_SPEC>`"]
pub type DMAC_DESCR6_PING_STATUS =
    crate::Reg<dmac_descr6_ping_status::DMAC_DESCR6_PING_STATUS_SPEC>;
#[doc = "Descriptor 0 status register for channel 6"]
pub mod dmac_descr6_ping_status;
#[doc = "DMAC_DESCR6_PONG_SRC register accessor: an alias for `Reg<DMAC_DESCR6_PONG_SRC_SPEC>`"]
pub type DMAC_DESCR6_PONG_SRC = crate::Reg<dmac_descr6_pong_src::DMAC_DESCR6_PONG_SRC_SPEC>;
#[doc = "Descriptor 1 source address location for channel 6"]
pub mod dmac_descr6_pong_src;
#[doc = "DMAC_DESCR6_PONG_DST register accessor: an alias for `Reg<DMAC_DESCR6_PONG_DST_SPEC>`"]
pub type DMAC_DESCR6_PONG_DST = crate::Reg<dmac_descr6_pong_dst::DMAC_DESCR6_PONG_DST_SPEC>;
#[doc = "Descriptor 1 destination address location for channel 6"]
pub mod dmac_descr6_pong_dst;
#[doc = "DMAC_DESCR6_PONG_CTL register accessor: an alias for `Reg<DMAC_DESCR6_PONG_CTL_SPEC>`"]
pub type DMAC_DESCR6_PONG_CTL = crate::Reg<dmac_descr6_pong_ctl::DMAC_DESCR6_PONG_CTL_SPEC>;
#[doc = "Descriptor 1 control register for channel 6"]
pub mod dmac_descr6_pong_ctl;
#[doc = "DMAC_DESCR6_PONG_STATUS register accessor: an alias for `Reg<DMAC_DESCR6_PONG_STATUS_SPEC>`"]
pub type DMAC_DESCR6_PONG_STATUS =
    crate::Reg<dmac_descr6_pong_status::DMAC_DESCR6_PONG_STATUS_SPEC>;
#[doc = "Descriptor 1 status register for channel 6"]
pub mod dmac_descr6_pong_status;
#[doc = "DMAC_DESCR7_PING_SRC register accessor: an alias for `Reg<DMAC_DESCR7_PING_SRC_SPEC>`"]
pub type DMAC_DESCR7_PING_SRC = crate::Reg<dmac_descr7_ping_src::DMAC_DESCR7_PING_SRC_SPEC>;
#[doc = "Descriptor 0 source address location for channel 7"]
pub mod dmac_descr7_ping_src;
#[doc = "DMAC_DESCR7_PING_DST register accessor: an alias for `Reg<DMAC_DESCR7_PING_DST_SPEC>`"]
pub type DMAC_DESCR7_PING_DST = crate::Reg<dmac_descr7_ping_dst::DMAC_DESCR7_PING_DST_SPEC>;
#[doc = "Descriptor 0 destination address location for channel 7"]
pub mod dmac_descr7_ping_dst;
#[doc = "DMAC_DESCR7_PING_CTL register accessor: an alias for `Reg<DMAC_DESCR7_PING_CTL_SPEC>`"]
pub type DMAC_DESCR7_PING_CTL = crate::Reg<dmac_descr7_ping_ctl::DMAC_DESCR7_PING_CTL_SPEC>;
#[doc = "Descriptor 0 control register for channel 7"]
pub mod dmac_descr7_ping_ctl;
#[doc = "DMAC_DESCR7_PING_STATUS register accessor: an alias for `Reg<DMAC_DESCR7_PING_STATUS_SPEC>`"]
pub type DMAC_DESCR7_PING_STATUS =
    crate::Reg<dmac_descr7_ping_status::DMAC_DESCR7_PING_STATUS_SPEC>;
#[doc = "Descriptor 0 status register for channel 7"]
pub mod dmac_descr7_ping_status;
#[doc = "DMAC_DESCR7_PONG_SRC register accessor: an alias for `Reg<DMAC_DESCR7_PONG_SRC_SPEC>`"]
pub type DMAC_DESCR7_PONG_SRC = crate::Reg<dmac_descr7_pong_src::DMAC_DESCR7_PONG_SRC_SPEC>;
#[doc = "Descriptor 1 source address location for channel 7"]
pub mod dmac_descr7_pong_src;
#[doc = "DMAC_DESCR7_PONG_DST register accessor: an alias for `Reg<DMAC_DESCR7_PONG_DST_SPEC>`"]
pub type DMAC_DESCR7_PONG_DST = crate::Reg<dmac_descr7_pong_dst::DMAC_DESCR7_PONG_DST_SPEC>;
#[doc = "Descriptor 1 destination address location for channel 7"]
pub mod dmac_descr7_pong_dst;
#[doc = "DMAC_DESCR7_PONG_CTL register accessor: an alias for `Reg<DMAC_DESCR7_PONG_CTL_SPEC>`"]
pub type DMAC_DESCR7_PONG_CTL = crate::Reg<dmac_descr7_pong_ctl::DMAC_DESCR7_PONG_CTL_SPEC>;
#[doc = "Descriptor 1 control register for channel 7"]
pub mod dmac_descr7_pong_ctl;
#[doc = "DMAC_DESCR7_PONG_STATUS register accessor: an alias for `Reg<DMAC_DESCR7_PONG_STATUS_SPEC>`"]
pub type DMAC_DESCR7_PONG_STATUS =
    crate::Reg<dmac_descr7_pong_status::DMAC_DESCR7_PONG_STATUS_SPEC>;
#[doc = "Descriptor 1 status register for channel 7"]
pub mod dmac_descr7_pong_status;
