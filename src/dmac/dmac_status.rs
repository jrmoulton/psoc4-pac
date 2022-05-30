#[doc = "Register `DMAC_STATUS` reader"]
pub struct R(crate::R<DMAC_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAC_STATUS` writer"]
pub struct W(crate::W<DMAC_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAC_STATUS_SPEC>;
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
impl From<crate::W<DMAC_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAC_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_NR` reader - Specifies the index of the currently active data transfer. This value increases from '0' to the DATA_NR field specified of the currently active descriptor control word."]
pub type DATA_NR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CH_ADDR` reader - Specifies the channel number of the currently active channel. For example, if channel 7 is active, DMAC_STATUS.ACTIVE is '1' and STATUS.CH_ADDR is '7'."]
pub type CH_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "State of the data transfer engine.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STATE_A {
    #[doc = "0: Idle state when the DMA is not active."]
    IDLE = 0,
    #[doc = "1: The DMA is loading the descriptor to the DMA transfer engine."]
    LOAD_DESCR = 1,
    #[doc = "2: The DMA is getting the value from the source location."]
    LOAD_SRC = 2,
    #[doc = "3: The DMA is storing the value at the destination location."]
    STORE_DST = 3,
    #[doc = "4: The DMA is updating the descriptors after completion of transfer."]
    STORE_DESCR = 4,
    #[doc = "5: The DMA is waiting for the level sensitive trigger to deactivate."]
    WAIT_TRIG_DEACT = 5,
    #[doc = "6: There was an error during the transaction and the DMA is writing the error code to the channel status register."]
    STORE_ERROR = 6,
}
impl From<STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STATE` reader - State of the data transfer engine."]
pub type STATE_R = crate::FieldReader<u8, STATE_A>;
impl STATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STATE_A> {
        match self.bits {
            0 => Some(STATE_A::IDLE),
            1 => Some(STATE_A::LOAD_DESCR),
            2 => Some(STATE_A::LOAD_SRC),
            3 => Some(STATE_A::STORE_DST),
            4 => Some(STATE_A::STORE_DESCR),
            5 => Some(STATE_A::WAIT_TRIG_DEACT),
            6 => Some(STATE_A::STORE_ERROR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == STATE_A::IDLE
    }
    #[doc = "Checks if the value of the field is `LOAD_DESCR`"]
    #[inline(always)]
    pub fn is_load_descr(&self) -> bool {
        *self == STATE_A::LOAD_DESCR
    }
    #[doc = "Checks if the value of the field is `LOAD_SRC`"]
    #[inline(always)]
    pub fn is_load_src(&self) -> bool {
        *self == STATE_A::LOAD_SRC
    }
    #[doc = "Checks if the value of the field is `STORE_DST`"]
    #[inline(always)]
    pub fn is_store_dst(&self) -> bool {
        *self == STATE_A::STORE_DST
    }
    #[doc = "Checks if the value of the field is `STORE_DESCR`"]
    #[inline(always)]
    pub fn is_store_descr(&self) -> bool {
        *self == STATE_A::STORE_DESCR
    }
    #[doc = "Checks if the value of the field is `WAIT_TRIG_DEACT`"]
    #[inline(always)]
    pub fn is_wait_trig_deact(&self) -> bool {
        *self == STATE_A::WAIT_TRIG_DEACT
    }
    #[doc = "Checks if the value of the field is `STORE_ERROR`"]
    #[inline(always)]
    pub fn is_store_error(&self) -> bool {
        *self == STATE_A::STORE_ERROR
    }
}
#[doc = "Field `PRIO` reader - Specifies the priority of the currently active channel."]
pub type PRIO_R = crate::FieldReader<u8, u8>;
#[doc = "Specifies whether the PING descriptor ('0') or PONG descriptor ('1') of the channel is currently in use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PING_PONG_A {
    #[doc = "0: Descriptor 0 (PING) is currently in use."]
    DESCR0 = 0,
    #[doc = "1: Descriptor 1 (PONG) is currently in use."]
    DESCR1 = 1,
}
impl From<PING_PONG_A> for bool {
    #[inline(always)]
    fn from(variant: PING_PONG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PING_PONG` reader - Specifies whether the PING descriptor ('0') or PONG descriptor ('1') of the channel is currently in use."]
pub type PING_PONG_R = crate::BitReader<PING_PONG_A>;
impl PING_PONG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PING_PONG_A {
        match self.bits {
            false => PING_PONG_A::DESCR0,
            true => PING_PONG_A::DESCR1,
        }
    }
    #[doc = "Checks if the value of the field is `DESCR0`"]
    #[inline(always)]
    pub fn is_descr0(&self) -> bool {
        *self == PING_PONG_A::DESCR0
    }
    #[doc = "Checks if the value of the field is `DESCR1`"]
    #[inline(always)]
    pub fn is_descr1(&self) -> bool {
        *self == PING_PONG_A::DESCR1
    }
}
#[doc = "Specifies if there is a currently active (pending) channel in the data transfer engine.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTIVE_A {
    #[doc = "0: No currently active channel."]
    IDLE = 0,
    #[doc = "1: Currently active channel."]
    ACTIVE = 1,
}
impl From<ACTIVE_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACTIVE` reader - Specifies if there is a currently active (pending) channel in the data transfer engine."]
pub type ACTIVE_R = crate::BitReader<ACTIVE_A>;
impl ACTIVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE_A {
        match self.bits {
            false => ACTIVE_A::IDLE,
            true => ACTIVE_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == ACTIVE_A::IDLE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ACTIVE_A::ACTIVE
    }
}
impl R {
    #[doc = "Bits 0:15 - Specifies the index of the currently active data transfer. This value increases from '0' to the DATA_NR field specified of the currently active descriptor control word."]
    #[inline(always)]
    pub fn data_nr(&self) -> DATA_NR_R {
        DATA_NR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:20 - Specifies the channel number of the currently active channel. For example, if channel 7 is active, DMAC_STATUS.ACTIVE is '1' and STATUS.CH_ADDR is '7'."]
    #[inline(always)]
    pub fn ch_addr(&self) -> CH_ADDR_R {
        CH_ADDR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26 - State of the data transfer engine."]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:29 - Specifies the priority of the currently active channel."]
    #[inline(always)]
    pub fn prio(&self) -> PRIO_R {
        PRIO_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - Specifies whether the PING descriptor ('0') or PONG descriptor ('1') of the channel is currently in use."]
    #[inline(always)]
    pub fn ping_pong(&self) -> PING_PONG_R {
        PING_PONG_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 31 - Specifies if there is a currently active (pending) channel in the data transfer engine."]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA controller status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_status](index.html) module"]
pub struct DMAC_STATUS_SPEC;
impl crate::RegisterSpec for DMAC_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_status::R](R) reader structure"]
impl crate::Readable for DMAC_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmac_status::W](W) writer structure"]
impl crate::Writable for DMAC_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAC_STATUS to value 0"]
impl crate::Resettable for DMAC_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
