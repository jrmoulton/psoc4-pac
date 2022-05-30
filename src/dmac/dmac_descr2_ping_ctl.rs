#[doc = "Register `DMAC_DESCR2_PING_CTL` reader"]
pub struct R(crate::R<DMAC_DESCR2_PING_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_DESCR2_PING_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_DESCR2_PING_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_DESCR2_PING_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAC_DESCR2_PING_CTL` writer"]
pub struct W(crate::W<DMAC_DESCR2_PING_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAC_DESCR2_PING_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DMAC_DESCR2_PING_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAC_DESCR2_PING_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_NR` reader - Number of data elements this descriptor transfers. A value of N results in a transfer of N+1 data elements."]
pub type DATA_NR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DATA_NR` writer - Number of data elements this descriptor transfers. A value of N results in a transfer of N+1 data elements."]
pub type DATA_NR_W<'a> = crate::FieldWriter<'a, u32, DMAC_DESCR2_PING_CTL_SPEC, u16, u16, 16, 0>;
#[doc = "Specifies the data element size.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATA_SIZE_A {
    #[doc = "0: 1 byte."]
    BYTE = 0,
    #[doc = "1: Halfword (2 bytes)."]
    HALFWORD = 1,
    #[doc = "2: Word (4 bytes)."]
    WORD = 2,
}
impl From<DATA_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: DATA_SIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DATA_SIZE` reader - Specifies the data element size."]
pub type DATA_SIZE_R = crate::FieldReader<u8, DATA_SIZE_A>;
impl DATA_SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DATA_SIZE_A> {
        match self.bits {
            0 => Some(DATA_SIZE_A::BYTE),
            1 => Some(DATA_SIZE_A::HALFWORD),
            2 => Some(DATA_SIZE_A::WORD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == DATA_SIZE_A::BYTE
    }
    #[doc = "Checks if the value of the field is `HALFWORD`"]
    #[inline(always)]
    pub fn is_halfword(&self) -> bool {
        *self == DATA_SIZE_A::HALFWORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == DATA_SIZE_A::WORD
    }
}
#[doc = "Field `DATA_SIZE` writer - Specifies the data element size."]
pub type DATA_SIZE_W<'a> =
    crate::FieldWriter<'a, u32, DMAC_DESCR2_PING_CTL_SPEC, u8, DATA_SIZE_A, 2, 16>;
impl<'a> DATA_SIZE_W<'a> {
    #[doc = "1 byte."]
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(DATA_SIZE_A::BYTE)
    }
    #[doc = "Halfword (2 bytes)."]
    #[inline(always)]
    pub fn halfword(self) -> &'a mut W {
        self.variant(DATA_SIZE_A::HALFWORD)
    }
    #[doc = "Word (4 bytes)."]
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(DATA_SIZE_A::WORD)
    }
}
#[doc = "Specifies the bus transfer size to the destination location.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DST_TRANSFER_SIZE_A {
    #[doc = "0: As specified by DATA_SIZE field."]
    DATA_SIZE = 0,
    #[doc = "1: Word (4 bytes)."]
    WORD = 1,
}
impl From<DST_TRANSFER_SIZE_A> for bool {
    #[inline(always)]
    fn from(variant: DST_TRANSFER_SIZE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DST_TRANSFER_SIZE` reader - Specifies the bus transfer size to the destination location."]
pub type DST_TRANSFER_SIZE_R = crate::BitReader<DST_TRANSFER_SIZE_A>;
impl DST_TRANSFER_SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DST_TRANSFER_SIZE_A {
        match self.bits {
            false => DST_TRANSFER_SIZE_A::DATA_SIZE,
            true => DST_TRANSFER_SIZE_A::WORD,
        }
    }
    #[doc = "Checks if the value of the field is `DATA_SIZE`"]
    #[inline(always)]
    pub fn is_data_size(&self) -> bool {
        *self == DST_TRANSFER_SIZE_A::DATA_SIZE
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == DST_TRANSFER_SIZE_A::WORD
    }
}
#[doc = "Field `DST_TRANSFER_SIZE` writer - Specifies the bus transfer size to the destination location."]
pub type DST_TRANSFER_SIZE_W<'a> =
    crate::BitWriter<'a, u32, DMAC_DESCR2_PING_CTL_SPEC, DST_TRANSFER_SIZE_A, 20>;
impl<'a> DST_TRANSFER_SIZE_W<'a> {
    #[doc = "As specified by DATA_SIZE field."]
    #[inline(always)]
    pub fn data_size(self) -> &'a mut W {
        self.variant(DST_TRANSFER_SIZE_A::DATA_SIZE)
    }
    #[doc = "Word (4 bytes)."]
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(DST_TRANSFER_SIZE_A::WORD)
    }
}
#[doc = "Specifies whether the destination address is incremented by the DST_TRANSFER_SIZE after the transfer of each data element.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DST_ADDR_INCR_A {
    #[doc = "0: No address increment."]
    INC_NONE = 0,
    #[doc = "1: Increment the destination address."]
    INC_DST_ADDR = 1,
}
impl From<DST_ADDR_INCR_A> for bool {
    #[inline(always)]
    fn from(variant: DST_ADDR_INCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DST_ADDR_INCR` reader - Specifies whether the destination address is incremented by the DST_TRANSFER_SIZE after the transfer of each data element."]
pub type DST_ADDR_INCR_R = crate::BitReader<DST_ADDR_INCR_A>;
impl DST_ADDR_INCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DST_ADDR_INCR_A {
        match self.bits {
            false => DST_ADDR_INCR_A::INC_NONE,
            true => DST_ADDR_INCR_A::INC_DST_ADDR,
        }
    }
    #[doc = "Checks if the value of the field is `INC_NONE`"]
    #[inline(always)]
    pub fn is_inc_none(&self) -> bool {
        *self == DST_ADDR_INCR_A::INC_NONE
    }
    #[doc = "Checks if the value of the field is `INC_DST_ADDR`"]
    #[inline(always)]
    pub fn is_inc_dst_addr(&self) -> bool {
        *self == DST_ADDR_INCR_A::INC_DST_ADDR
    }
}
#[doc = "Field `DST_ADDR_INCR` writer - Specifies whether the destination address is incremented by the DST_TRANSFER_SIZE after the transfer of each data element."]
pub type DST_ADDR_INCR_W<'a> =
    crate::BitWriter<'a, u32, DMAC_DESCR2_PING_CTL_SPEC, DST_ADDR_INCR_A, 21>;
impl<'a> DST_ADDR_INCR_W<'a> {
    #[doc = "No address increment."]
    #[inline(always)]
    pub fn inc_none(self) -> &'a mut W {
        self.variant(DST_ADDR_INCR_A::INC_NONE)
    }
    #[doc = "Increment the destination address."]
    #[inline(always)]
    pub fn inc_dst_addr(self) -> &'a mut W {
        self.variant(DST_ADDR_INCR_A::INC_DST_ADDR)
    }
}
#[doc = "Specifies the bus transfer size to the source location.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_TRANSFER_SIZE_A {
    #[doc = "0: As specified by DATA_SIZE field."]
    DATA_SIZE = 0,
    #[doc = "1: Word (4 bytes)."]
    WORD = 1,
}
impl From<SRC_TRANSFER_SIZE_A> for bool {
    #[inline(always)]
    fn from(variant: SRC_TRANSFER_SIZE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRC_TRANSFER_SIZE` reader - Specifies the bus transfer size to the source location."]
pub type SRC_TRANSFER_SIZE_R = crate::BitReader<SRC_TRANSFER_SIZE_A>;
impl SRC_TRANSFER_SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC_TRANSFER_SIZE_A {
        match self.bits {
            false => SRC_TRANSFER_SIZE_A::DATA_SIZE,
            true => SRC_TRANSFER_SIZE_A::WORD,
        }
    }
    #[doc = "Checks if the value of the field is `DATA_SIZE`"]
    #[inline(always)]
    pub fn is_data_size(&self) -> bool {
        *self == SRC_TRANSFER_SIZE_A::DATA_SIZE
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == SRC_TRANSFER_SIZE_A::WORD
    }
}
#[doc = "Field `SRC_TRANSFER_SIZE` writer - Specifies the bus transfer size to the source location."]
pub type SRC_TRANSFER_SIZE_W<'a> =
    crate::BitWriter<'a, u32, DMAC_DESCR2_PING_CTL_SPEC, SRC_TRANSFER_SIZE_A, 22>;
impl<'a> SRC_TRANSFER_SIZE_W<'a> {
    #[doc = "As specified by DATA_SIZE field."]
    #[inline(always)]
    pub fn data_size(self) -> &'a mut W {
        self.variant(SRC_TRANSFER_SIZE_A::DATA_SIZE)
    }
    #[doc = "Word (4 bytes)."]
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(SRC_TRANSFER_SIZE_A::WORD)
    }
}
#[doc = "Specifies whether the source address is incremented by the SRC_TRANSFER_SIZE after the transfer of each data element.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_ADDR_INCR_A {
    #[doc = "0: No address increment."]
    INC_NONE = 0,
    #[doc = "1: Increment the source address."]
    INC_SRC_ADDR = 1,
}
impl From<SRC_ADDR_INCR_A> for bool {
    #[inline(always)]
    fn from(variant: SRC_ADDR_INCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRC_ADDR_INCR` reader - Specifies whether the source address is incremented by the SRC_TRANSFER_SIZE after the transfer of each data element."]
pub type SRC_ADDR_INCR_R = crate::BitReader<SRC_ADDR_INCR_A>;
impl SRC_ADDR_INCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC_ADDR_INCR_A {
        match self.bits {
            false => SRC_ADDR_INCR_A::INC_NONE,
            true => SRC_ADDR_INCR_A::INC_SRC_ADDR,
        }
    }
    #[doc = "Checks if the value of the field is `INC_NONE`"]
    #[inline(always)]
    pub fn is_inc_none(&self) -> bool {
        *self == SRC_ADDR_INCR_A::INC_NONE
    }
    #[doc = "Checks if the value of the field is `INC_SRC_ADDR`"]
    #[inline(always)]
    pub fn is_inc_src_addr(&self) -> bool {
        *self == SRC_ADDR_INCR_A::INC_SRC_ADDR
    }
}
#[doc = "Field `SRC_ADDR_INCR` writer - Specifies whether the source address is incremented by the SRC_TRANSFER_SIZE after the transfer of each data element."]
pub type SRC_ADDR_INCR_W<'a> =
    crate::BitWriter<'a, u32, DMAC_DESCR2_PING_CTL_SPEC, SRC_ADDR_INCR_A, 23>;
impl<'a> SRC_ADDR_INCR_W<'a> {
    #[doc = "No address increment."]
    #[inline(always)]
    pub fn inc_none(self) -> &'a mut W {
        self.variant(SRC_ADDR_INCR_A::INC_NONE)
    }
    #[doc = "Increment the source address."]
    #[inline(always)]
    pub fn inc_src_addr(self) -> &'a mut W {
        self.variant(SRC_ADDR_INCR_A::INC_SRC_ADDR)
    }
}
#[doc = "Specifies whether the data transfer engine should wait for the channel to be deactivated; i.e. the selected system trigger is not active. This field is used to synchronize the controller's data transfer(s) with the agent that generated the trigger. This field is ONLY used at the completion of an opcode. E.g., a FIFO indicates that it is empty and it needs a new data sample. The agent removes the trigger ONLY when the data sample has been written by the transfer engine AND received by the agent. Furthermore, the agent's trigger may be delayed by a few cycles before it reaches the DW/DMA controller. This field is used for level sensitive trigger, which reflect state (pulse sensitive triggers should have this field set to '0'). The wait cycles incurred by this field reduce DW/DMA controller performance.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WAIT_FOR_DEACT_A {
    #[doc = "0: Do not wait for de-activation (for pulse sensitive triggers)."]
    PULSE = 0,
    #[doc = "1: Wait for up to 4 cycles."]
    LEVEL_FOUR = 1,
    #[doc = "2: Wait for up to 8 cycles."]
    LEVEL_EIGHT = 2,
    #[doc = "3: Wait indefinitely. This option may result in DMA controller lockup if the system trigger is not de-activated by the source agent."]
    LEVEL_UNKNOWN = 3,
}
impl From<WAIT_FOR_DEACT_A> for u8 {
    #[inline(always)]
    fn from(variant: WAIT_FOR_DEACT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WAIT_FOR_DEACT` reader - Specifies whether the data transfer engine should wait for the channel to be deactivated; i.e. the selected system trigger is not active. This field is used to synchronize the controller's data transfer(s) with the agent that generated the trigger. This field is ONLY used at the completion of an opcode. E.g., a FIFO indicates that it is empty and it needs a new data sample. The agent removes the trigger ONLY when the data sample has been written by the transfer engine AND received by the agent. Furthermore, the agent's trigger may be delayed by a few cycles before it reaches the DW/DMA controller. This field is used for level sensitive trigger, which reflect state (pulse sensitive triggers should have this field set to '0'). The wait cycles incurred by this field reduce DW/DMA controller performance."]
pub type WAIT_FOR_DEACT_R = crate::FieldReader<u8, WAIT_FOR_DEACT_A>;
impl WAIT_FOR_DEACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAIT_FOR_DEACT_A {
        match self.bits {
            0 => WAIT_FOR_DEACT_A::PULSE,
            1 => WAIT_FOR_DEACT_A::LEVEL_FOUR,
            2 => WAIT_FOR_DEACT_A::LEVEL_EIGHT,
            3 => WAIT_FOR_DEACT_A::LEVEL_UNKNOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULSE`"]
    #[inline(always)]
    pub fn is_pulse(&self) -> bool {
        *self == WAIT_FOR_DEACT_A::PULSE
    }
    #[doc = "Checks if the value of the field is `LEVEL_FOUR`"]
    #[inline(always)]
    pub fn is_level_four(&self) -> bool {
        *self == WAIT_FOR_DEACT_A::LEVEL_FOUR
    }
    #[doc = "Checks if the value of the field is `LEVEL_EIGHT`"]
    #[inline(always)]
    pub fn is_level_eight(&self) -> bool {
        *self == WAIT_FOR_DEACT_A::LEVEL_EIGHT
    }
    #[doc = "Checks if the value of the field is `LEVEL_UNKNOWN`"]
    #[inline(always)]
    pub fn is_level_unknown(&self) -> bool {
        *self == WAIT_FOR_DEACT_A::LEVEL_UNKNOWN
    }
}
#[doc = "Field `WAIT_FOR_DEACT` writer - Specifies whether the data transfer engine should wait for the channel to be deactivated; i.e. the selected system trigger is not active. This field is used to synchronize the controller's data transfer(s) with the agent that generated the trigger. This field is ONLY used at the completion of an opcode. E.g., a FIFO indicates that it is empty and it needs a new data sample. The agent removes the trigger ONLY when the data sample has been written by the transfer engine AND received by the agent. Furthermore, the agent's trigger may be delayed by a few cycles before it reaches the DW/DMA controller. This field is used for level sensitive trigger, which reflect state (pulse sensitive triggers should have this field set to '0'). The wait cycles incurred by this field reduce DW/DMA controller performance."]
pub type WAIT_FOR_DEACT_W<'a> =
    crate::FieldWriterSafe<'a, u32, DMAC_DESCR2_PING_CTL_SPEC, u8, WAIT_FOR_DEACT_A, 2, 24>;
impl<'a> WAIT_FOR_DEACT_W<'a> {
    #[doc = "Do not wait for de-activation (for pulse sensitive triggers)."]
    #[inline(always)]
    pub fn pulse(self) -> &'a mut W {
        self.variant(WAIT_FOR_DEACT_A::PULSE)
    }
    #[doc = "Wait for up to 4 cycles."]
    #[inline(always)]
    pub fn level_four(self) -> &'a mut W {
        self.variant(WAIT_FOR_DEACT_A::LEVEL_FOUR)
    }
    #[doc = "Wait for up to 8 cycles."]
    #[inline(always)]
    pub fn level_eight(self) -> &'a mut W {
        self.variant(WAIT_FOR_DEACT_A::LEVEL_EIGHT)
    }
    #[doc = "Wait indefinitely. This option may result in DMA controller lockup if the system trigger is not de-activated by the source agent."]
    #[inline(always)]
    pub fn level_unknown(self) -> &'a mut W {
        self.variant(WAIT_FOR_DEACT_A::LEVEL_UNKNOWN)
    }
}
#[doc = "Field `INV_DESCR` reader - If set, the VALID bit of the descriptor's STATUS word is set to '0' on completion of the current descriptor structure."]
pub type INV_DESCR_R = crate::BitReader<bool>;
#[doc = "Field `INV_DESCR` writer - If set, the VALID bit of the descriptor's STATUS word is set to '0' on completion of the current descriptor structure."]
pub type INV_DESCR_W<'a> = crate::BitWriter<'a, u32, DMAC_DESCR2_PING_CTL_SPEC, bool, 26>;
#[doc = "Field `SET_CAUSE` reader - If set, the interrupt bit of the channel is set to '1' on completion of the current descriptor structure."]
pub type SET_CAUSE_R = crate::BitReader<bool>;
#[doc = "Field `SET_CAUSE` writer - If set, the interrupt bit of the channel is set to '1' on completion of the current descriptor structure."]
pub type SET_CAUSE_W<'a> = crate::BitWriter<'a, u32, DMAC_DESCR2_PING_CTL_SPEC, bool, 27>;
#[doc = "Field `PREEMPTABLE` reader - If set, the transfer is preemptable. Multi data element transfers are constructed out of multiple single data element load (from the source location) and store (to the destination location) sequences. This field allows higher priority activated channels to preempt the current transfer in between these atomic (load, store) sequences. Preemption will NOT deactivate the current channel. As a result, after completion of a higher priority activated channel, the current channel is rescheduled."]
pub type PREEMPTABLE_R = crate::BitReader<bool>;
#[doc = "Field `PREEMPTABLE` writer - If set, the transfer is preemptable. Multi data element transfers are constructed out of multiple single data element load (from the source location) and store (to the destination location) sequences. This field allows higher priority activated channels to preempt the current transfer in between these atomic (load, store) sequences. Preemption will NOT deactivate the current channel. As a result, after completion of a higher priority activated channel, the current channel is rescheduled."]
pub type PREEMPTABLE_W<'a> = crate::BitWriter<'a, u32, DMAC_DESCR2_PING_CTL_SPEC, bool, 28>;
#[doc = "Field `FLIPPING` reader - If set, on completion of the current descriptor structure, the current descriptor identifier is flipped/inverted. In DMA mode, descriptor list transfer, flipping of the current descriptor identifier can be used to construct a linked list of descriptor structures."]
pub type FLIPPING_R = crate::BitReader<bool>;
#[doc = "Field `FLIPPING` writer - If set, on completion of the current descriptor structure, the current descriptor identifier is flipped/inverted. In DMA mode, descriptor list transfer, flipping of the current descriptor identifier can be used to construct a linked list of descriptor structures."]
pub type FLIPPING_W<'a> = crate::BitWriter<'a, u32, DMAC_DESCR2_PING_CTL_SPEC, bool, 29>;
#[doc = "Specifies how the DMA reacts to a trigger event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPCODE_A {
    #[doc = "0: A single trigger initiates a single data element transfer. This opcode specifies a transfer of a single data element. The current descriptor is completed when the amount of transferred single data elements equals the programmed buffer size (DATA_NR+1)."]
    SINGLE_DATA_ELEMENT = 0,
    #[doc = "1: A single trigger initiates an entire descriptor transfer. This opcode specifies a transfer of DATA_NR+1 data elements as specified by the current descriptor structure. The current descriptor is completed when its data transfer is completed."]
    ENTIRE_DESCRIPTOR = 1,
    #[doc = "2: A single trigger initiates a descriptor list transfer. This opcode specifies a transfer of DATA_NR+1 data elements as specified by the current descriptor structure and by successive valid descriptors. The current descriptor is completed when its data transfer is completed. This OPCODE relies on FLIPPING to be set to '1', such that the CHi_CTL.PING_PONG field is flipped/inverted and the successive descriptor is used. This continues for as long as the successive descriptor is valid. Note that as the HW is using the PING/PONG descriptor, the SW can prepare the alternate PONG/PING descriptor. The interrupt mechanism is used by HW to convey to the SW that the current descriptor is completed (and can be prepared for a successive transfer)."]
    ENTIRE_DESCRIPTOR_CHAIN = 2,
}
impl From<OPCODE_A> for u8 {
    #[inline(always)]
    fn from(variant: OPCODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OPCODE` reader - Specifies how the DMA reacts to a trigger event."]
pub type OPCODE_R = crate::FieldReader<u8, OPCODE_A>;
impl OPCODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPCODE_A> {
        match self.bits {
            0 => Some(OPCODE_A::SINGLE_DATA_ELEMENT),
            1 => Some(OPCODE_A::ENTIRE_DESCRIPTOR),
            2 => Some(OPCODE_A::ENTIRE_DESCRIPTOR_CHAIN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE_DATA_ELEMENT`"]
    #[inline(always)]
    pub fn is_single_data_element(&self) -> bool {
        *self == OPCODE_A::SINGLE_DATA_ELEMENT
    }
    #[doc = "Checks if the value of the field is `ENTIRE_DESCRIPTOR`"]
    #[inline(always)]
    pub fn is_entire_descriptor(&self) -> bool {
        *self == OPCODE_A::ENTIRE_DESCRIPTOR
    }
    #[doc = "Checks if the value of the field is `ENTIRE_DESCRIPTOR_CHAIN`"]
    #[inline(always)]
    pub fn is_entire_descriptor_chain(&self) -> bool {
        *self == OPCODE_A::ENTIRE_DESCRIPTOR_CHAIN
    }
}
#[doc = "Field `OPCODE` writer - Specifies how the DMA reacts to a trigger event."]
pub type OPCODE_W<'a> = crate::FieldWriter<'a, u32, DMAC_DESCR2_PING_CTL_SPEC, u8, OPCODE_A, 2, 30>;
impl<'a> OPCODE_W<'a> {
    #[doc = "A single trigger initiates a single data element transfer. This opcode specifies a transfer of a single data element. The current descriptor is completed when the amount of transferred single data elements equals the programmed buffer size (DATA_NR+1)."]
    #[inline(always)]
    pub fn single_data_element(self) -> &'a mut W {
        self.variant(OPCODE_A::SINGLE_DATA_ELEMENT)
    }
    #[doc = "A single trigger initiates an entire descriptor transfer. This opcode specifies a transfer of DATA_NR+1 data elements as specified by the current descriptor structure. The current descriptor is completed when its data transfer is completed."]
    #[inline(always)]
    pub fn entire_descriptor(self) -> &'a mut W {
        self.variant(OPCODE_A::ENTIRE_DESCRIPTOR)
    }
    #[doc = "A single trigger initiates a descriptor list transfer. This opcode specifies a transfer of DATA_NR+1 data elements as specified by the current descriptor structure and by successive valid descriptors. The current descriptor is completed when its data transfer is completed. This OPCODE relies on FLIPPING to be set to '1', such that the CHi_CTL.PING_PONG field is flipped/inverted and the successive descriptor is used. This continues for as long as the successive descriptor is valid. Note that as the HW is using the PING/PONG descriptor, the SW can prepare the alternate PONG/PING descriptor. The interrupt mechanism is used by HW to convey to the SW that the current descriptor is completed (and can be prepared for a successive transfer)."]
    #[inline(always)]
    pub fn entire_descriptor_chain(self) -> &'a mut W {
        self.variant(OPCODE_A::ENTIRE_DESCRIPTOR_CHAIN)
    }
}
impl R {
    #[doc = "Bits 0:15 - Number of data elements this descriptor transfers. A value of N results in a transfer of N+1 data elements."]
    #[inline(always)]
    pub fn data_nr(&self) -> DATA_NR_R {
        DATA_NR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - Specifies the data element size."]
    #[inline(always)]
    pub fn data_size(&self) -> DATA_SIZE_R {
        DATA_SIZE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - Specifies the bus transfer size to the destination location."]
    #[inline(always)]
    pub fn dst_transfer_size(&self) -> DST_TRANSFER_SIZE_R {
        DST_TRANSFER_SIZE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Specifies whether the destination address is incremented by the DST_TRANSFER_SIZE after the transfer of each data element."]
    #[inline(always)]
    pub fn dst_addr_incr(&self) -> DST_ADDR_INCR_R {
        DST_ADDR_INCR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Specifies the bus transfer size to the source location."]
    #[inline(always)]
    pub fn src_transfer_size(&self) -> SRC_TRANSFER_SIZE_R {
        SRC_TRANSFER_SIZE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Specifies whether the source address is incremented by the SRC_TRANSFER_SIZE after the transfer of each data element."]
    #[inline(always)]
    pub fn src_addr_incr(&self) -> SRC_ADDR_INCR_R {
        SRC_ADDR_INCR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Specifies whether the data transfer engine should wait for the channel to be deactivated; i.e. the selected system trigger is not active. This field is used to synchronize the controller's data transfer(s) with the agent that generated the trigger. This field is ONLY used at the completion of an opcode. E.g., a FIFO indicates that it is empty and it needs a new data sample. The agent removes the trigger ONLY when the data sample has been written by the transfer engine AND received by the agent. Furthermore, the agent's trigger may be delayed by a few cycles before it reaches the DW/DMA controller. This field is used for level sensitive trigger, which reflect state (pulse sensitive triggers should have this field set to '0'). The wait cycles incurred by this field reduce DW/DMA controller performance."]
    #[inline(always)]
    pub fn wait_for_deact(&self) -> WAIT_FOR_DEACT_R {
        WAIT_FOR_DEACT_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - If set, the VALID bit of the descriptor's STATUS word is set to '0' on completion of the current descriptor structure."]
    #[inline(always)]
    pub fn inv_descr(&self) -> INV_DESCR_R {
        INV_DESCR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - If set, the interrupt bit of the channel is set to '1' on completion of the current descriptor structure."]
    #[inline(always)]
    pub fn set_cause(&self) -> SET_CAUSE_R {
        SET_CAUSE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - If set, the transfer is preemptable. Multi data element transfers are constructed out of multiple single data element load (from the source location) and store (to the destination location) sequences. This field allows higher priority activated channels to preempt the current transfer in between these atomic (load, store) sequences. Preemption will NOT deactivate the current channel. As a result, after completion of a higher priority activated channel, the current channel is rescheduled."]
    #[inline(always)]
    pub fn preemptable(&self) -> PREEMPTABLE_R {
        PREEMPTABLE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - If set, on completion of the current descriptor structure, the current descriptor identifier is flipped/inverted. In DMA mode, descriptor list transfer, flipping of the current descriptor identifier can be used to construct a linked list of descriptor structures."]
    #[inline(always)]
    pub fn flipping(&self) -> FLIPPING_R {
        FLIPPING_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Specifies how the DMA reacts to a trigger event."]
    #[inline(always)]
    pub fn opcode(&self) -> OPCODE_R {
        OPCODE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of data elements this descriptor transfers. A value of N results in a transfer of N+1 data elements."]
    #[inline(always)]
    pub fn data_nr(&mut self) -> DATA_NR_W {
        DATA_NR_W::new(self)
    }
    #[doc = "Bits 16:17 - Specifies the data element size."]
    #[inline(always)]
    pub fn data_size(&mut self) -> DATA_SIZE_W {
        DATA_SIZE_W::new(self)
    }
    #[doc = "Bit 20 - Specifies the bus transfer size to the destination location."]
    #[inline(always)]
    pub fn dst_transfer_size(&mut self) -> DST_TRANSFER_SIZE_W {
        DST_TRANSFER_SIZE_W::new(self)
    }
    #[doc = "Bit 21 - Specifies whether the destination address is incremented by the DST_TRANSFER_SIZE after the transfer of each data element."]
    #[inline(always)]
    pub fn dst_addr_incr(&mut self) -> DST_ADDR_INCR_W {
        DST_ADDR_INCR_W::new(self)
    }
    #[doc = "Bit 22 - Specifies the bus transfer size to the source location."]
    #[inline(always)]
    pub fn src_transfer_size(&mut self) -> SRC_TRANSFER_SIZE_W {
        SRC_TRANSFER_SIZE_W::new(self)
    }
    #[doc = "Bit 23 - Specifies whether the source address is incremented by the SRC_TRANSFER_SIZE after the transfer of each data element."]
    #[inline(always)]
    pub fn src_addr_incr(&mut self) -> SRC_ADDR_INCR_W {
        SRC_ADDR_INCR_W::new(self)
    }
    #[doc = "Bits 24:25 - Specifies whether the data transfer engine should wait for the channel to be deactivated; i.e. the selected system trigger is not active. This field is used to synchronize the controller's data transfer(s) with the agent that generated the trigger. This field is ONLY used at the completion of an opcode. E.g., a FIFO indicates that it is empty and it needs a new data sample. The agent removes the trigger ONLY when the data sample has been written by the transfer engine AND received by the agent. Furthermore, the agent's trigger may be delayed by a few cycles before it reaches the DW/DMA controller. This field is used for level sensitive trigger, which reflect state (pulse sensitive triggers should have this field set to '0'). The wait cycles incurred by this field reduce DW/DMA controller performance."]
    #[inline(always)]
    pub fn wait_for_deact(&mut self) -> WAIT_FOR_DEACT_W {
        WAIT_FOR_DEACT_W::new(self)
    }
    #[doc = "Bit 26 - If set, the VALID bit of the descriptor's STATUS word is set to '0' on completion of the current descriptor structure."]
    #[inline(always)]
    pub fn inv_descr(&mut self) -> INV_DESCR_W {
        INV_DESCR_W::new(self)
    }
    #[doc = "Bit 27 - If set, the interrupt bit of the channel is set to '1' on completion of the current descriptor structure."]
    #[inline(always)]
    pub fn set_cause(&mut self) -> SET_CAUSE_W {
        SET_CAUSE_W::new(self)
    }
    #[doc = "Bit 28 - If set, the transfer is preemptable. Multi data element transfers are constructed out of multiple single data element load (from the source location) and store (to the destination location) sequences. This field allows higher priority activated channels to preempt the current transfer in between these atomic (load, store) sequences. Preemption will NOT deactivate the current channel. As a result, after completion of a higher priority activated channel, the current channel is rescheduled."]
    #[inline(always)]
    pub fn preemptable(&mut self) -> PREEMPTABLE_W {
        PREEMPTABLE_W::new(self)
    }
    #[doc = "Bit 29 - If set, on completion of the current descriptor structure, the current descriptor identifier is flipped/inverted. In DMA mode, descriptor list transfer, flipping of the current descriptor identifier can be used to construct a linked list of descriptor structures."]
    #[inline(always)]
    pub fn flipping(&mut self) -> FLIPPING_W {
        FLIPPING_W::new(self)
    }
    #[doc = "Bits 30:31 - Specifies how the DMA reacts to a trigger event."]
    #[inline(always)]
    pub fn opcode(&mut self) -> OPCODE_W {
        OPCODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Descriptor 0 control register for channel 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_descr2_ping_ctl](index.html) module"]
pub struct DMAC_DESCR2_PING_CTL_SPEC;
impl crate::RegisterSpec for DMAC_DESCR2_PING_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_descr2_ping_ctl::R](R) reader structure"]
impl crate::Readable for DMAC_DESCR2_PING_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmac_descr2_ping_ctl::W](W) writer structure"]
impl crate::Writable for DMAC_DESCR2_PING_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAC_DESCR2_PING_CTL to value 0"]
impl crate::Resettable for DMAC_DESCR2_PING_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
