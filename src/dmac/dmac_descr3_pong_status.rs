#[doc = "Register `DMAC_DESCR3_PONG_STATUS` reader"]
pub struct R(crate::R<DMAC_DESCR3_PONG_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_DESCR3_PONG_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_DESCR3_PONG_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_DESCR3_PONG_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAC_DESCR3_PONG_STATUS` writer"]
pub struct W(crate::W<DMAC_DESCR3_PONG_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAC_DESCR3_PONG_STATUS_SPEC>;
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
impl From<crate::W<DMAC_DESCR3_PONG_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAC_DESCR3_PONG_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Descriptor validity status. Hardware set this field to '0' when a descriptor is done, but only if CONTROL.INV_DESCR is '1'. Software sets this field to '1' when a descriptor is initialized.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VALID_A {
    #[doc = "0: Invalid, cannot be used for a data transfer. An attempt to use this descriptor for a data transfer will result in an INVALID_DESCR response code and the interrupt cause bit is set to '1'."]
    INVALID = 0,
    #[doc = "1: Valid."]
    VALID = 1,
}
impl From<VALID_A> for bool {
    #[inline(always)]
    fn from(variant: VALID_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VALID` reader - Descriptor validity status. Hardware set this field to '0' when a descriptor is done, but only if CONTROL.INV_DESCR is '1'. Software sets this field to '1' when a descriptor is initialized."]
pub type VALID_R = crate::BitReader<VALID_A>;
impl VALID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VALID_A {
        match self.bits {
            false => VALID_A::INVALID,
            true => VALID_A::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `INVALID`"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        *self == VALID_A::INVALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == VALID_A::VALID
    }
}
#[doc = "Field `VALID` writer - Descriptor validity status. Hardware set this field to '0' when a descriptor is done, but only if CONTROL.INV_DESCR is '1'. Software sets this field to '1' when a descriptor is initialized."]
pub type VALID_W<'a> = crate::BitWriter<'a, u32, DMAC_DESCR3_PONG_STATUS_SPEC, VALID_A, 31>;
impl<'a> VALID_W<'a> {
    #[doc = "Invalid, cannot be used for a data transfer. An attempt to use this descriptor for a data transfer will result in an INVALID_DESCR response code and the interrupt cause bit is set to '1'."]
    #[inline(always)]
    pub fn invalid(self) -> &'a mut W {
        self.variant(VALID_A::INVALID)
    }
    #[doc = "Valid."]
    #[inline(always)]
    pub fn valid(self) -> &'a mut W {
        self.variant(VALID_A::VALID)
    }
}
#[doc = "Response code (the first two codes NO_ERROR and DONE are the result of normal behavior, the other codes are the result of erroneous behavior).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RESONSE_A {
    #[doc = "0: No error. Setting this response does NOT set the interrupt bit to '1'. STATUS.VALID is NOT affected. CHx_CTL.ENABLED is NOT affected. CHx_CTL.PING_PONG is not updated. This response is used for an unused or not completed descriptor. Software should set the RESPONSE field to '0'/NO_ERROR during descriptor initialization."]
    NO_ERROR = 0,
    #[doc = "1: Descriptor is done (without errors). Setting this response sets the interrupt cause bit to '1' if CONTROL.SET_CAUSE is '1'. STATUS.VALID is set to '0' if CONTROL.INV_DESCR is '1'. CHx_CTL.ENABLED is NOT affected. CHx_CTL.PING_PONG is updated if CONTROL.FLIPPING is '1'."]
    DONE = 1,
    #[doc = "2: Bus error while loading data from the source location. Setting this response sets the interrupt cause bit to '1'. STATUS.VALID is set '0'. CHx_CTL.ENABLED is set to '0'. CHx_CTL.PING_PONG is not updated (it identifies the descriptor that caused the error)."]
    SRC_BUS_ERROR = 2,
    #[doc = "3: Bus error while storing data to the destination location. Setting this response sets the interrupt cause bit to '1'. STATUS.VALID is set '1'. CHx_CTL.ENABLED is set to '0'. CHx_CTL.PING_PONG is not updated (it identifies the descriptor that caused the error)."]
    DST_BUS_ERROR = 3,
    #[doc = "4: Misalignment of source address. This occurs on a 16-bit bus transfer that is not 2-byte aligned or on a 32-bit bus transfer that is not 4-byte aligned. Setting this response sets the interrupt cause bit to '1'. STATUS.VALID is set '0'. CHx_CTL.ENABLED is set to '0'. CHx_CTL.PING_PONG is not updated (it identifies the descriptor that caused the error)."]
    SRC_MISAL = 4,
    #[doc = "5: Misalignment of destination address. This occurs on a 16-bit bus transfer that is not 2-byte aligned or on a 32-bit bus transfer that is not 4-byte aligned. Setting this response sets the interrupt cause bit to '1'. STATUS.VALID is set '0'. CHx_CTL.ENABLED is set to '0'. CHx_CTL.PING_PONG is not updated (it identifies the descriptor that caused the error)."]
    DST_MISAL = 5,
    #[doc = "6: Invalid descriptor (STATUS.VALID is '0'). This occurs when an activated channel has an invalid descriptor. CHx_CTL.ENABLED is set to '0'. CHx_CTL.PING_PONG is not updated (it identifies the descriptor that caused the error)."]
    INVALID = 6,
}
impl From<RESONSE_A> for u8 {
    #[inline(always)]
    fn from(variant: RESONSE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RESONSE` reader - Response code (the first two codes NO_ERROR and DONE are the result of normal behavior, the other codes are the result of erroneous behavior)."]
pub type RESONSE_R = crate::FieldReader<u8, RESONSE_A>;
impl RESONSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RESONSE_A> {
        match self.bits {
            0 => Some(RESONSE_A::NO_ERROR),
            1 => Some(RESONSE_A::DONE),
            2 => Some(RESONSE_A::SRC_BUS_ERROR),
            3 => Some(RESONSE_A::DST_BUS_ERROR),
            4 => Some(RESONSE_A::SRC_MISAL),
            5 => Some(RESONSE_A::DST_MISAL),
            6 => Some(RESONSE_A::INVALID),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == RESONSE_A::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `DONE`"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == RESONSE_A::DONE
    }
    #[doc = "Checks if the value of the field is `SRC_BUS_ERROR`"]
    #[inline(always)]
    pub fn is_src_bus_error(&self) -> bool {
        *self == RESONSE_A::SRC_BUS_ERROR
    }
    #[doc = "Checks if the value of the field is `DST_BUS_ERROR`"]
    #[inline(always)]
    pub fn is_dst_bus_error(&self) -> bool {
        *self == RESONSE_A::DST_BUS_ERROR
    }
    #[doc = "Checks if the value of the field is `SRC_MISAL`"]
    #[inline(always)]
    pub fn is_src_misal(&self) -> bool {
        *self == RESONSE_A::SRC_MISAL
    }
    #[doc = "Checks if the value of the field is `DST_MISAL`"]
    #[inline(always)]
    pub fn is_dst_misal(&self) -> bool {
        *self == RESONSE_A::DST_MISAL
    }
    #[doc = "Checks if the value of the field is `INVALID`"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        *self == RESONSE_A::INVALID
    }
}
#[doc = "Field `RESONSE` writer - Response code (the first two codes NO_ERROR and DONE are the result of normal behavior, the other codes are the result of erroneous behavior)."]
pub type RESONSE_W<'a> =
    crate::FieldWriter<'a, u32, DMAC_DESCR3_PONG_STATUS_SPEC, u8, RESONSE_A, 3, 16>;
impl<'a> RESONSE_W<'a> {
    #[doc = "No error. Setting this response does NOT set the interrupt bit to '1'. STATUS.VALID is NOT affected. CHx_CTL.ENABLED is NOT affected. CHx_CTL.PING_PONG is not updated. This response is used for an unused or not completed descriptor. Software should set the RESPONSE field to '0'/NO_ERROR during descriptor initialization."]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(RESONSE_A::NO_ERROR)
    }
    #[doc = "Descriptor is done (without errors). Setting this response sets the interrupt cause bit to '1' if CONTROL.SET_CAUSE is '1'. STATUS.VALID is set to '0' if CONTROL.INV_DESCR is '1'. CHx_CTL.ENABLED is NOT affected. CHx_CTL.PING_PONG is updated if CONTROL.FLIPPING is '1'."]
    #[inline(always)]
    pub fn done(self) -> &'a mut W {
        self.variant(RESONSE_A::DONE)
    }
    #[doc = "Bus error while loading data from the source location. Setting this response sets the interrupt cause bit to '1'. STATUS.VALID is set '0'. CHx_CTL.ENABLED is set to '0'. CHx_CTL.PING_PONG is not updated (it identifies the descriptor that caused the error)."]
    #[inline(always)]
    pub fn src_bus_error(self) -> &'a mut W {
        self.variant(RESONSE_A::SRC_BUS_ERROR)
    }
    #[doc = "Bus error while storing data to the destination location. Setting this response sets the interrupt cause bit to '1'. STATUS.VALID is set '1'. CHx_CTL.ENABLED is set to '0'. CHx_CTL.PING_PONG is not updated (it identifies the descriptor that caused the error)."]
    #[inline(always)]
    pub fn dst_bus_error(self) -> &'a mut W {
        self.variant(RESONSE_A::DST_BUS_ERROR)
    }
    #[doc = "Misalignment of source address. This occurs on a 16-bit bus transfer that is not 2-byte aligned or on a 32-bit bus transfer that is not 4-byte aligned. Setting this response sets the interrupt cause bit to '1'. STATUS.VALID is set '0'. CHx_CTL.ENABLED is set to '0'. CHx_CTL.PING_PONG is not updated (it identifies the descriptor that caused the error)."]
    #[inline(always)]
    pub fn src_misal(self) -> &'a mut W {
        self.variant(RESONSE_A::SRC_MISAL)
    }
    #[doc = "Misalignment of destination address. This occurs on a 16-bit bus transfer that is not 2-byte aligned or on a 32-bit bus transfer that is not 4-byte aligned. Setting this response sets the interrupt cause bit to '1'. STATUS.VALID is set '0'. CHx_CTL.ENABLED is set to '0'. CHx_CTL.PING_PONG is not updated (it identifies the descriptor that caused the error)."]
    #[inline(always)]
    pub fn dst_misal(self) -> &'a mut W {
        self.variant(RESONSE_A::DST_MISAL)
    }
    #[doc = "Invalid descriptor (STATUS.VALID is '0'). This occurs when an activated channel has an invalid descriptor. CHx_CTL.ENABLED is set to '0'. CHx_CTL.PING_PONG is not updated (it identifies the descriptor that caused the error)."]
    #[inline(always)]
    pub fn invalid(self) -> &'a mut W {
        self.variant(RESONSE_A::INVALID)
    }
}
#[doc = "Field `CURR_DATA_NR` reader - Specifies the index of the current data transfer. This value increases from 0 to CONTROL.DATA_NR. HW sets this field: ? - When a descriptor is done (RESPONSE is DONE), the field is set to '0' when PING_CTL.INV_DESCR is '0' and the field is set to PING_CTL.DATA_NR when PING_CTL.INV_DESCR is '1'. ? - When a descriptor is not done (RESPONSE is NO_ERROR), the field reflects the progress of a data transfer. ? - In case of erroneous behavior (RESPONSE is neither DONE or NO_ERROR), the field is not updated, but keeps its value to ease debugging. ? HW only modifies this field for an active descriptor (STATUS.VALID to be '1'). At descriptor initialization, SW should set this field to '0'. ? This field allows software to read the progress of the data transfer. Note that SRC.ADDR and DST.ADDR represent base addresses and are not modified during data transfer. However, STATUS.CURR_DATA_NR is modified during data transfer and provides an offset wrt. the base addresses."]
pub type CURR_DATA_NR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CURR_DATA_NR` writer - Specifies the index of the current data transfer. This value increases from 0 to CONTROL.DATA_NR. HW sets this field: ? - When a descriptor is done (RESPONSE is DONE), the field is set to '0' when PING_CTL.INV_DESCR is '0' and the field is set to PING_CTL.DATA_NR when PING_CTL.INV_DESCR is '1'. ? - When a descriptor is not done (RESPONSE is NO_ERROR), the field reflects the progress of a data transfer. ? - In case of erroneous behavior (RESPONSE is neither DONE or NO_ERROR), the field is not updated, but keeps its value to ease debugging. ? HW only modifies this field for an active descriptor (STATUS.VALID to be '1'). At descriptor initialization, SW should set this field to '0'. ? This field allows software to read the progress of the data transfer. Note that SRC.ADDR and DST.ADDR represent base addresses and are not modified during data transfer. However, STATUS.CURR_DATA_NR is modified during data transfer and provides an offset wrt. the base addresses."]
pub type CURR_DATA_NR_W<'a> =
    crate::FieldWriter<'a, u32, DMAC_DESCR3_PONG_STATUS_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bit 31 - Descriptor validity status. Hardware set this field to '0' when a descriptor is done, but only if CONTROL.INV_DESCR is '1'. Software sets this field to '1' when a descriptor is initialized."]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Response code (the first two codes NO_ERROR and DONE are the result of normal behavior, the other codes are the result of erroneous behavior)."]
    #[inline(always)]
    pub fn resonse(&self) -> RESONSE_R {
        RESONSE_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 0:15 - Specifies the index of the current data transfer. This value increases from 0 to CONTROL.DATA_NR. HW sets this field: ? - When a descriptor is done (RESPONSE is DONE), the field is set to '0' when PING_CTL.INV_DESCR is '0' and the field is set to PING_CTL.DATA_NR when PING_CTL.INV_DESCR is '1'. ? - When a descriptor is not done (RESPONSE is NO_ERROR), the field reflects the progress of a data transfer. ? - In case of erroneous behavior (RESPONSE is neither DONE or NO_ERROR), the field is not updated, but keeps its value to ease debugging. ? HW only modifies this field for an active descriptor (STATUS.VALID to be '1'). At descriptor initialization, SW should set this field to '0'. ? This field allows software to read the progress of the data transfer. Note that SRC.ADDR and DST.ADDR represent base addresses and are not modified during data transfer. However, STATUS.CURR_DATA_NR is modified during data transfer and provides an offset wrt. the base addresses."]
    #[inline(always)]
    pub fn curr_data_nr(&self) -> CURR_DATA_NR_R {
        CURR_DATA_NR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - Descriptor validity status. Hardware set this field to '0' when a descriptor is done, but only if CONTROL.INV_DESCR is '1'. Software sets this field to '1' when a descriptor is initialized."]
    #[inline(always)]
    pub fn valid(&mut self) -> VALID_W {
        VALID_W::new(self)
    }
    #[doc = "Bits 16:18 - Response code (the first two codes NO_ERROR and DONE are the result of normal behavior, the other codes are the result of erroneous behavior)."]
    #[inline(always)]
    pub fn resonse(&mut self) -> RESONSE_W {
        RESONSE_W::new(self)
    }
    #[doc = "Bits 0:15 - Specifies the index of the current data transfer. This value increases from 0 to CONTROL.DATA_NR. HW sets this field: ? - When a descriptor is done (RESPONSE is DONE), the field is set to '0' when PING_CTL.INV_DESCR is '0' and the field is set to PING_CTL.DATA_NR when PING_CTL.INV_DESCR is '1'. ? - When a descriptor is not done (RESPONSE is NO_ERROR), the field reflects the progress of a data transfer. ? - In case of erroneous behavior (RESPONSE is neither DONE or NO_ERROR), the field is not updated, but keeps its value to ease debugging. ? HW only modifies this field for an active descriptor (STATUS.VALID to be '1'). At descriptor initialization, SW should set this field to '0'. ? This field allows software to read the progress of the data transfer. Note that SRC.ADDR and DST.ADDR represent base addresses and are not modified during data transfer. However, STATUS.CURR_DATA_NR is modified during data transfer and provides an offset wrt. the base addresses."]
    #[inline(always)]
    pub fn curr_data_nr(&mut self) -> CURR_DATA_NR_W {
        CURR_DATA_NR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Descriptor 1 status register for channel 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_descr3_pong_status](index.html) module"]
pub struct DMAC_DESCR3_PONG_STATUS_SPEC;
impl crate::RegisterSpec for DMAC_DESCR3_PONG_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_descr3_pong_status::R](R) reader structure"]
impl crate::Readable for DMAC_DESCR3_PONG_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmac_descr3_pong_status::W](W) writer structure"]
impl crate::Writable for DMAC_DESCR3_PONG_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAC_DESCR3_PONG_STATUS to value 0"]
impl crate::Resettable for DMAC_DESCR3_PONG_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
