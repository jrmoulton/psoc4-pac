#[doc = "Register `SAMPLE_CTRL` reader"]
pub struct R(crate::R<SAMPLE_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAMPLE_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAMPLE_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAMPLE_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAMPLE_CTRL` writer"]
pub struct W(crate::W<SAMPLE_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAMPLE_CTRL_SPEC>;
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
impl From<crate::W<SAMPLE_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAMPLE_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Conversion resolution for channels that have sub-resolution enabled (RESOLUTION=1) (otherwise resolution is 12-bit).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUB_RESOLUTION_A {
    #[doc = "0: 8-bit."]
    _8B = 0,
    #[doc = "1: 10-bit."]
    _10B = 1,
}
impl From<SUB_RESOLUTION_A> for bool {
    #[inline(always)]
    fn from(variant: SUB_RESOLUTION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUB_RESOLUTION` reader - Conversion resolution for channels that have sub-resolution enabled (RESOLUTION=1) (otherwise resolution is 12-bit)."]
pub type SUB_RESOLUTION_R = crate::BitReader<SUB_RESOLUTION_A>;
impl SUB_RESOLUTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUB_RESOLUTION_A {
        match self.bits {
            false => SUB_RESOLUTION_A::_8B,
            true => SUB_RESOLUTION_A::_10B,
        }
    }
    #[doc = "Checks if the value of the field is `_8B`"]
    #[inline(always)]
    pub fn is_8b(&self) -> bool {
        *self == SUB_RESOLUTION_A::_8B
    }
    #[doc = "Checks if the value of the field is `_10B`"]
    #[inline(always)]
    pub fn is_10b(&self) -> bool {
        *self == SUB_RESOLUTION_A::_10B
    }
}
#[doc = "Field `SUB_RESOLUTION` writer - Conversion resolution for channels that have sub-resolution enabled (RESOLUTION=1) (otherwise resolution is 12-bit)."]
pub type SUB_RESOLUTION_W<'a> = crate::BitWriter<'a, u32, SAMPLE_CTRL_SPEC, SUB_RESOLUTION_A, 0>;
impl<'a> SUB_RESOLUTION_W<'a> {
    #[doc = "8-bit."]
    #[inline(always)]
    pub fn _8b(self) -> &'a mut W {
        self.variant(SUB_RESOLUTION_A::_8B)
    }
    #[doc = "10-bit."]
    #[inline(always)]
    pub fn _10b(self) -> &'a mut W {
        self.variant(SUB_RESOLUTION_A::_10B)
    }
}
#[doc = "Field `LEFT_ALIGN` reader - Left align data in data\\[15:0\\], default data is right aligned in data\\[11:0\\], with sign extension to 16 bits if the channel is differential."]
pub type LEFT_ALIGN_R = crate::BitReader<bool>;
#[doc = "Field `LEFT_ALIGN` writer - Left align data in data\\[15:0\\], default data is right aligned in data\\[11:0\\], with sign extension to 16 bits if the channel is differential."]
pub type LEFT_ALIGN_W<'a> = crate::BitWriter<'a, u32, SAMPLE_CTRL_SPEC, bool, 1>;
#[doc = "Output data from a single ended conversion as a signed value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SINGLE_ENDED_SIGNED_A {
    #[doc = "0: Default: result data is unsigned (zero extended if needed)"]
    UNSIGNED = 0,
    #[doc = "1: result data is signed (sign extended if needed)"]
    SIGNED = 1,
}
impl From<SINGLE_ENDED_SIGNED_A> for bool {
    #[inline(always)]
    fn from(variant: SINGLE_ENDED_SIGNED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SINGLE_ENDED_SIGNED` reader - Output data from a single ended conversion as a signed value"]
pub type SINGLE_ENDED_SIGNED_R = crate::BitReader<SINGLE_ENDED_SIGNED_A>;
impl SINGLE_ENDED_SIGNED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SINGLE_ENDED_SIGNED_A {
        match self.bits {
            false => SINGLE_ENDED_SIGNED_A::UNSIGNED,
            true => SINGLE_ENDED_SIGNED_A::SIGNED,
        }
    }
    #[doc = "Checks if the value of the field is `UNSIGNED`"]
    #[inline(always)]
    pub fn is_unsigned(&self) -> bool {
        *self == SINGLE_ENDED_SIGNED_A::UNSIGNED
    }
    #[doc = "Checks if the value of the field is `SIGNED`"]
    #[inline(always)]
    pub fn is_signed(&self) -> bool {
        *self == SINGLE_ENDED_SIGNED_A::SIGNED
    }
}
#[doc = "Field `SINGLE_ENDED_SIGNED` writer - Output data from a single ended conversion as a signed value"]
pub type SINGLE_ENDED_SIGNED_W<'a> =
    crate::BitWriter<'a, u32, SAMPLE_CTRL_SPEC, SINGLE_ENDED_SIGNED_A, 2>;
impl<'a> SINGLE_ENDED_SIGNED_W<'a> {
    #[doc = "Default: result data is unsigned (zero extended if needed)"]
    #[inline(always)]
    pub fn unsigned(self) -> &'a mut W {
        self.variant(SINGLE_ENDED_SIGNED_A::UNSIGNED)
    }
    #[doc = "result data is signed (sign extended if needed)"]
    #[inline(always)]
    pub fn signed(self) -> &'a mut W {
        self.variant(SINGLE_ENDED_SIGNED_A::SIGNED)
    }
}
#[doc = "Output data from a differential conversion as a signed value\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIFFERENTIAL_SIGNED_A {
    #[doc = "0: result data is unsigned (zero extended if needed)"]
    UNSIGNED = 0,
    #[doc = "1: Default: result data is signed (sign extended if needed)"]
    SIGNED = 1,
}
impl From<DIFFERENTIAL_SIGNED_A> for bool {
    #[inline(always)]
    fn from(variant: DIFFERENTIAL_SIGNED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIFFERENTIAL_SIGNED` reader - Output data from a differential conversion as a signed value"]
pub type DIFFERENTIAL_SIGNED_R = crate::BitReader<DIFFERENTIAL_SIGNED_A>;
impl DIFFERENTIAL_SIGNED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIFFERENTIAL_SIGNED_A {
        match self.bits {
            false => DIFFERENTIAL_SIGNED_A::UNSIGNED,
            true => DIFFERENTIAL_SIGNED_A::SIGNED,
        }
    }
    #[doc = "Checks if the value of the field is `UNSIGNED`"]
    #[inline(always)]
    pub fn is_unsigned(&self) -> bool {
        *self == DIFFERENTIAL_SIGNED_A::UNSIGNED
    }
    #[doc = "Checks if the value of the field is `SIGNED`"]
    #[inline(always)]
    pub fn is_signed(&self) -> bool {
        *self == DIFFERENTIAL_SIGNED_A::SIGNED
    }
}
#[doc = "Field `DIFFERENTIAL_SIGNED` writer - Output data from a differential conversion as a signed value"]
pub type DIFFERENTIAL_SIGNED_W<'a> =
    crate::BitWriter<'a, u32, SAMPLE_CTRL_SPEC, DIFFERENTIAL_SIGNED_A, 3>;
impl<'a> DIFFERENTIAL_SIGNED_W<'a> {
    #[doc = "result data is unsigned (zero extended if needed)"]
    #[inline(always)]
    pub fn unsigned(self) -> &'a mut W {
        self.variant(DIFFERENTIAL_SIGNED_A::UNSIGNED)
    }
    #[doc = "Default: result data is signed (sign extended if needed)"]
    #[inline(always)]
    pub fn signed(self) -> &'a mut W {
        self.variant(DIFFERENTIAL_SIGNED_A::SIGNED)
    }
}
#[doc = "Field `AVG_CNT` reader - Averaging Count for channels that have over sampling enabled (AVG_EN). A channel will be sampled back to back (1<<(AVG_CNT+1)) = \\[2..256\\]
times before the result is stored and the next enabled channel is sampled (1st order accumulate and dump filter). If shifting is not enabled (AVG_SHIFT=0) then the result is forced to shift right so that is fits in 16 bits, so right shift is done by max(0,AVG_CNT-3)."]
pub type AVG_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AVG_CNT` writer - Averaging Count for channels that have over sampling enabled (AVG_EN). A channel will be sampled back to back (1<<(AVG_CNT+1)) = \\[2..256\\]
times before the result is stored and the next enabled channel is sampled (1st order accumulate and dump filter). If shifting is not enabled (AVG_SHIFT=0) then the result is forced to shift right so that is fits in 16 bits, so right shift is done by max(0,AVG_CNT-3)."]
pub type AVG_CNT_W<'a> = crate::FieldWriter<'a, u32, SAMPLE_CTRL_SPEC, u8, u8, 3, 4>;
#[doc = "Field `AVG_SHIFT` reader - Averaging shifting: after averaging the result is shifted right to fit in the sample resolution. For averaging the sample resolution is the highest resolution allowed by wounding."]
pub type AVG_SHIFT_R = crate::BitReader<bool>;
#[doc = "Field `AVG_SHIFT` writer - Averaging shifting: after averaging the result is shifted right to fit in the sample resolution. For averaging the sample resolution is the highest resolution allowed by wounding."]
pub type AVG_SHIFT_W<'a> = crate::BitWriter<'a, u32, SAMPLE_CTRL_SPEC, bool, 7>;
#[doc = "Field `CONTINUOUS` reader - - 0: Wait for next FW_TRIGGER (one shot) or hardware (DSI) trigger (e.g. from TPWM for periodic triggering) before scanning enabled channels. - 1: Continuously scan enabled channels, ignore triggers."]
pub type CONTINUOUS_R = crate::BitReader<bool>;
#[doc = "Field `CONTINUOUS` writer - - 0: Wait for next FW_TRIGGER (one shot) or hardware (DSI) trigger (e.g. from TPWM for periodic triggering) before scanning enabled channels. - 1: Continuously scan enabled channels, ignore triggers."]
pub type CONTINUOUS_W<'a> = crate::BitWriter<'a, u32, SAMPLE_CTRL_SPEC, bool, 16>;
#[doc = "Field `DSI_TRIGGER_EN` reader - - 0: firmware trigger only: disable hardware (DSI) trigger. - 1: enable hardware (DSI) trigger (e.g. from TCPWM, GPIO or UDB)."]
pub type DSI_TRIGGER_EN_R = crate::BitReader<bool>;
#[doc = "Field `DSI_TRIGGER_EN` writer - - 0: firmware trigger only: disable hardware (DSI) trigger. - 1: enable hardware (DSI) trigger (e.g. from TCPWM, GPIO or UDB)."]
pub type DSI_TRIGGER_EN_W<'a> = crate::BitWriter<'a, u32, SAMPLE_CTRL_SPEC, bool, 17>;
#[doc = "Field `DSI_TRIGGER_LEVEL` reader - - 0: DSI trigger signal is a pulse input, a positive edge detected on the DSI trigger signal triggers a new scan. - 1: DSI trigger signal is a level input, as long as the DSI trigger signal remains high the SAR will do continuous scans."]
pub type DSI_TRIGGER_LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `DSI_TRIGGER_LEVEL` writer - - 0: DSI trigger signal is a pulse input, a positive edge detected on the DSI trigger signal triggers a new scan. - 1: DSI trigger signal is a level input, as long as the DSI trigger signal remains high the SAR will do continuous scans."]
pub type DSI_TRIGGER_LEVEL_W<'a> = crate::BitWriter<'a, u32, SAMPLE_CTRL_SPEC, bool, 18>;
#[doc = "Field `DSI_SYNC_TRIGGER` reader - - 0: bypass clock domain synchronisation of the DSI trigger signal. - 1: synchronize the DSI trigger signal to the SAR clock domain, if needed an edge detect is done in the peripheral clock domain."]
pub type DSI_SYNC_TRIGGER_R = crate::BitReader<bool>;
#[doc = "Field `DSI_SYNC_TRIGGER` writer - - 0: bypass clock domain synchronisation of the DSI trigger signal. - 1: synchronize the DSI trigger signal to the SAR clock domain, if needed an edge detect is done in the peripheral clock domain."]
pub type DSI_SYNC_TRIGGER_W<'a> = crate::BitWriter<'a, u32, SAMPLE_CTRL_SPEC, bool, 19>;
#[doc = "Field `EOS_DSI_OUT_EN` reader - Enable to output EOS_INTR to DSI. When enabled each time EOS_INTR is set by the hardware also a pulse is send on the dsi_eos signal."]
pub type EOS_DSI_OUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `EOS_DSI_OUT_EN` writer - Enable to output EOS_INTR to DSI. When enabled each time EOS_INTR is set by the hardware also a pulse is send on the dsi_eos signal."]
pub type EOS_DSI_OUT_EN_W<'a> = crate::BitWriter<'a, u32, SAMPLE_CTRL_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - Conversion resolution for channels that have sub-resolution enabled (RESOLUTION=1) (otherwise resolution is 12-bit)."]
    #[inline(always)]
    pub fn sub_resolution(&self) -> SUB_RESOLUTION_R {
        SUB_RESOLUTION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Left align data in data\\[15:0\\], default data is right aligned in data\\[11:0\\], with sign extension to 16 bits if the channel is differential."]
    #[inline(always)]
    pub fn left_align(&self) -> LEFT_ALIGN_R {
        LEFT_ALIGN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output data from a single ended conversion as a signed value"]
    #[inline(always)]
    pub fn single_ended_signed(&self) -> SINGLE_ENDED_SIGNED_R {
        SINGLE_ENDED_SIGNED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output data from a differential conversion as a signed value"]
    #[inline(always)]
    pub fn differential_signed(&self) -> DIFFERENTIAL_SIGNED_R {
        DIFFERENTIAL_SIGNED_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Averaging Count for channels that have over sampling enabled (AVG_EN). A channel will be sampled back to back (1<<(AVG_CNT+1)) = \\[2..256\\]
times before the result is stored and the next enabled channel is sampled (1st order accumulate and dump filter). If shifting is not enabled (AVG_SHIFT=0) then the result is forced to shift right so that is fits in 16 bits, so right shift is done by max(0,AVG_CNT-3)."]
    #[inline(always)]
    pub fn avg_cnt(&self) -> AVG_CNT_R {
        AVG_CNT_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Averaging shifting: after averaging the result is shifted right to fit in the sample resolution. For averaging the sample resolution is the highest resolution allowed by wounding."]
    #[inline(always)]
    pub fn avg_shift(&self) -> AVG_SHIFT_R {
        AVG_SHIFT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - - 0: Wait for next FW_TRIGGER (one shot) or hardware (DSI) trigger (e.g. from TPWM for periodic triggering) before scanning enabled channels. - 1: Continuously scan enabled channels, ignore triggers."]
    #[inline(always)]
    pub fn continuous(&self) -> CONTINUOUS_R {
        CONTINUOUS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - - 0: firmware trigger only: disable hardware (DSI) trigger. - 1: enable hardware (DSI) trigger (e.g. from TCPWM, GPIO or UDB)."]
    #[inline(always)]
    pub fn dsi_trigger_en(&self) -> DSI_TRIGGER_EN_R {
        DSI_TRIGGER_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - - 0: DSI trigger signal is a pulse input, a positive edge detected on the DSI trigger signal triggers a new scan. - 1: DSI trigger signal is a level input, as long as the DSI trigger signal remains high the SAR will do continuous scans."]
    #[inline(always)]
    pub fn dsi_trigger_level(&self) -> DSI_TRIGGER_LEVEL_R {
        DSI_TRIGGER_LEVEL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - - 0: bypass clock domain synchronisation of the DSI trigger signal. - 1: synchronize the DSI trigger signal to the SAR clock domain, if needed an edge detect is done in the peripheral clock domain."]
    #[inline(always)]
    pub fn dsi_sync_trigger(&self) -> DSI_SYNC_TRIGGER_R {
        DSI_SYNC_TRIGGER_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable to output EOS_INTR to DSI. When enabled each time EOS_INTR is set by the hardware also a pulse is send on the dsi_eos signal."]
    #[inline(always)]
    pub fn eos_dsi_out_en(&self) -> EOS_DSI_OUT_EN_R {
        EOS_DSI_OUT_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Conversion resolution for channels that have sub-resolution enabled (RESOLUTION=1) (otherwise resolution is 12-bit)."]
    #[inline(always)]
    pub fn sub_resolution(&mut self) -> SUB_RESOLUTION_W {
        SUB_RESOLUTION_W::new(self)
    }
    #[doc = "Bit 1 - Left align data in data\\[15:0\\], default data is right aligned in data\\[11:0\\], with sign extension to 16 bits if the channel is differential."]
    #[inline(always)]
    pub fn left_align(&mut self) -> LEFT_ALIGN_W {
        LEFT_ALIGN_W::new(self)
    }
    #[doc = "Bit 2 - Output data from a single ended conversion as a signed value"]
    #[inline(always)]
    pub fn single_ended_signed(&mut self) -> SINGLE_ENDED_SIGNED_W {
        SINGLE_ENDED_SIGNED_W::new(self)
    }
    #[doc = "Bit 3 - Output data from a differential conversion as a signed value"]
    #[inline(always)]
    pub fn differential_signed(&mut self) -> DIFFERENTIAL_SIGNED_W {
        DIFFERENTIAL_SIGNED_W::new(self)
    }
    #[doc = "Bits 4:6 - Averaging Count for channels that have over sampling enabled (AVG_EN). A channel will be sampled back to back (1<<(AVG_CNT+1)) = \\[2..256\\]
times before the result is stored and the next enabled channel is sampled (1st order accumulate and dump filter). If shifting is not enabled (AVG_SHIFT=0) then the result is forced to shift right so that is fits in 16 bits, so right shift is done by max(0,AVG_CNT-3)."]
    #[inline(always)]
    pub fn avg_cnt(&mut self) -> AVG_CNT_W {
        AVG_CNT_W::new(self)
    }
    #[doc = "Bit 7 - Averaging shifting: after averaging the result is shifted right to fit in the sample resolution. For averaging the sample resolution is the highest resolution allowed by wounding."]
    #[inline(always)]
    pub fn avg_shift(&mut self) -> AVG_SHIFT_W {
        AVG_SHIFT_W::new(self)
    }
    #[doc = "Bit 16 - - 0: Wait for next FW_TRIGGER (one shot) or hardware (DSI) trigger (e.g. from TPWM for periodic triggering) before scanning enabled channels. - 1: Continuously scan enabled channels, ignore triggers."]
    #[inline(always)]
    pub fn continuous(&mut self) -> CONTINUOUS_W {
        CONTINUOUS_W::new(self)
    }
    #[doc = "Bit 17 - - 0: firmware trigger only: disable hardware (DSI) trigger. - 1: enable hardware (DSI) trigger (e.g. from TCPWM, GPIO or UDB)."]
    #[inline(always)]
    pub fn dsi_trigger_en(&mut self) -> DSI_TRIGGER_EN_W {
        DSI_TRIGGER_EN_W::new(self)
    }
    #[doc = "Bit 18 - - 0: DSI trigger signal is a pulse input, a positive edge detected on the DSI trigger signal triggers a new scan. - 1: DSI trigger signal is a level input, as long as the DSI trigger signal remains high the SAR will do continuous scans."]
    #[inline(always)]
    pub fn dsi_trigger_level(&mut self) -> DSI_TRIGGER_LEVEL_W {
        DSI_TRIGGER_LEVEL_W::new(self)
    }
    #[doc = "Bit 19 - - 0: bypass clock domain synchronisation of the DSI trigger signal. - 1: synchronize the DSI trigger signal to the SAR clock domain, if needed an edge detect is done in the peripheral clock domain."]
    #[inline(always)]
    pub fn dsi_sync_trigger(&mut self) -> DSI_SYNC_TRIGGER_W {
        DSI_SYNC_TRIGGER_W::new(self)
    }
    #[doc = "Bit 31 - Enable to output EOS_INTR to DSI. When enabled each time EOS_INTR is set by the hardware also a pulse is send on the dsi_eos signal."]
    #[inline(always)]
    pub fn eos_dsi_out_en(&mut self) -> EOS_DSI_OUT_EN_W {
        EOS_DSI_OUT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sample control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sample_ctrl](index.html) module"]
pub struct SAMPLE_CTRL_SPEC;
impl crate::RegisterSpec for SAMPLE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sample_ctrl::R](R) reader structure"]
impl crate::Readable for SAMPLE_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sample_ctrl::W](W) writer structure"]
impl crate::Writable for SAMPLE_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAMPLE_CTRL to value 0x0008_0008"]
impl crate::Resettable for SAMPLE_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0008_0008
    }
}
