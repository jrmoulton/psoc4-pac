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
#[doc = "Field `SUB_RESOLUTION` reader - Conversion resolution for channels that have sub-resolution enabled (RESOLUTION=1) (otherwise resolution is 12-bit)."]
pub type SUB_RESOLUTION_R = crate::BitReader<bool>;
#[doc = "Field `SUB_RESOLUTION` writer - Conversion resolution for channels that have sub-resolution enabled (RESOLUTION=1) (otherwise resolution is 12-bit)."]
pub type SUB_RESOLUTION_W<'a> = crate::BitWriter<'a, u32, SAMPLE_CTRL_SPEC, bool, 0>;
#[doc = "Field `LEFT_ALIGN` reader - Left align data in data\\[15:0\\], default data is right aligned in data\\[11:0\\], with sign extension to 16 bits if the channel is differential."]
pub type LEFT_ALIGN_R = crate::BitReader<bool>;
#[doc = "Field `LEFT_ALIGN` writer - Left align data in data\\[15:0\\], default data is right aligned in data\\[11:0\\], with sign extension to 16 bits if the channel is differential."]
pub type LEFT_ALIGN_W<'a> = crate::BitWriter<'a, u32, SAMPLE_CTRL_SPEC, bool, 1>;
#[doc = "Field `SINGLE_ENDED_SIGNED` reader - Output data from a single ended conversion as a signed value"]
pub type SINGLE_ENDED_SIGNED_R = crate::BitReader<bool>;
#[doc = "Field `SINGLE_ENDED_SIGNED` writer - Output data from a single ended conversion as a signed value"]
pub type SINGLE_ENDED_SIGNED_W<'a> = crate::BitWriter<'a, u32, SAMPLE_CTRL_SPEC, bool, 2>;
#[doc = "Field `DIFFERENTIAL_SIGNED` reader - Output data from a differential conversion as a signed value"]
pub type DIFFERENTIAL_SIGNED_R = crate::BitReader<bool>;
#[doc = "Field `DIFFERENTIAL_SIGNED` writer - Output data from a differential conversion as a signed value"]
pub type DIFFERENTIAL_SIGNED_W<'a> = crate::BitWriter<'a, u32, SAMPLE_CTRL_SPEC, bool, 3>;
#[doc = "Field `AVG_CNT` reader - Averaging Count for channels that have over sampling enabled"]
pub type AVG_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AVG_CNT` writer - Averaging Count for channels that have over sampling enabled"]
pub type AVG_CNT_W<'a> = crate::FieldWriter<'a, u32, SAMPLE_CTRL_SPEC, u8, u8, 3, 4>;
#[doc = "Field `AVG_SHIFT` reader - Averaging shifting: after averaging the result is shifted right to fit in the sample resolution, i.e. 12 bits."]
pub type AVG_SHIFT_R = crate::BitReader<bool>;
#[doc = "Field `AVG_SHIFT` writer - Averaging shifting: after averaging the result is shifted right to fit in the sample resolution, i.e. 12 bits."]
pub type AVG_SHIFT_W<'a> = crate::BitWriter<'a, u32, SAMPLE_CTRL_SPEC, bool, 7>;
#[doc = "Field `CONTINUOUS` reader - 0: Wait for next FW_TRIGGER or hardware trigger before scanning enabled channels. 1: Continuously scan enabled channels.."]
pub type CONTINUOUS_R = crate::BitReader<bool>;
#[doc = "Field `CONTINUOUS` writer - 0: Wait for next FW_TRIGGER or hardware trigger before scanning enabled channels. 1: Continuously scan enabled channels.."]
pub type CONTINUOUS_W<'a> = crate::BitWriter<'a, u32, SAMPLE_CTRL_SPEC, bool, 16>;
#[doc = "Field `DSI_TRIGGER_EN` reader - 0: firmware trigger only, 1: enable hardware (DSI) trigger."]
pub type DSI_TRIGGER_EN_R = crate::BitReader<bool>;
#[doc = "Field `DSI_TRIGGER_EN` writer - 0: firmware trigger only, 1: enable hardware (DSI) trigger."]
pub type DSI_TRIGGER_EN_W<'a> = crate::BitWriter<'a, u32, SAMPLE_CTRL_SPEC, bool, 17>;
#[doc = "Field `DSI_TRIGGER_LEVEL` reader - 0: DSI trigger signal is a pulse input, 1: DSI trigger signal is a level inpu.t"]
pub type DSI_TRIGGER_LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `DSI_TRIGGER_LEVEL` writer - 0: DSI trigger signal is a pulse input, 1: DSI trigger signal is a level inpu.t"]
pub type DSI_TRIGGER_LEVEL_W<'a> = crate::BitWriter<'a, u32, SAMPLE_CTRL_SPEC, bool, 18>;
#[doc = "Field `DSI_SYNC_TRIGGER` reader - 0: bypass clock domain synchronisation of the DSI trigger signal, 1: synchronize the DSI trigger signal to the SAR clock domain."]
pub type DSI_SYNC_TRIGGER_R = crate::BitReader<bool>;
#[doc = "Field `DSI_SYNC_TRIGGER` writer - 0: bypass clock domain synchronisation of the DSI trigger signal, 1: synchronize the DSI trigger signal to the SAR clock domain."]
pub type DSI_SYNC_TRIGGER_W<'a> = crate::BitWriter<'a, u32, SAMPLE_CTRL_SPEC, bool, 19>;
#[doc = "Field `EOS_DSI_OUT_EN` reader - Enable to output EOS_INTR to DSI"]
pub type EOS_DSI_OUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `EOS_DSI_OUT_EN` writer - Enable to output EOS_INTR to DSI"]
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
    #[doc = "Bits 4:6 - Averaging Count for channels that have over sampling enabled"]
    #[inline(always)]
    pub fn avg_cnt(&self) -> AVG_CNT_R {
        AVG_CNT_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Averaging shifting: after averaging the result is shifted right to fit in the sample resolution, i.e. 12 bits."]
    #[inline(always)]
    pub fn avg_shift(&self) -> AVG_SHIFT_R {
        AVG_SHIFT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - 0: Wait for next FW_TRIGGER or hardware trigger before scanning enabled channels. 1: Continuously scan enabled channels.."]
    #[inline(always)]
    pub fn continuous(&self) -> CONTINUOUS_R {
        CONTINUOUS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 0: firmware trigger only, 1: enable hardware (DSI) trigger."]
    #[inline(always)]
    pub fn dsi_trigger_en(&self) -> DSI_TRIGGER_EN_R {
        DSI_TRIGGER_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 0: DSI trigger signal is a pulse input, 1: DSI trigger signal is a level inpu.t"]
    #[inline(always)]
    pub fn dsi_trigger_level(&self) -> DSI_TRIGGER_LEVEL_R {
        DSI_TRIGGER_LEVEL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 0: bypass clock domain synchronisation of the DSI trigger signal, 1: synchronize the DSI trigger signal to the SAR clock domain."]
    #[inline(always)]
    pub fn dsi_sync_trigger(&self) -> DSI_SYNC_TRIGGER_R {
        DSI_SYNC_TRIGGER_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable to output EOS_INTR to DSI"]
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
    #[doc = "Bits 4:6 - Averaging Count for channels that have over sampling enabled"]
    #[inline(always)]
    pub fn avg_cnt(&mut self) -> AVG_CNT_W {
        AVG_CNT_W::new(self)
    }
    #[doc = "Bit 7 - Averaging shifting: after averaging the result is shifted right to fit in the sample resolution, i.e. 12 bits."]
    #[inline(always)]
    pub fn avg_shift(&mut self) -> AVG_SHIFT_W {
        AVG_SHIFT_W::new(self)
    }
    #[doc = "Bit 16 - 0: Wait for next FW_TRIGGER or hardware trigger before scanning enabled channels. 1: Continuously scan enabled channels.."]
    #[inline(always)]
    pub fn continuous(&mut self) -> CONTINUOUS_W {
        CONTINUOUS_W::new(self)
    }
    #[doc = "Bit 17 - 0: firmware trigger only, 1: enable hardware (DSI) trigger."]
    #[inline(always)]
    pub fn dsi_trigger_en(&mut self) -> DSI_TRIGGER_EN_W {
        DSI_TRIGGER_EN_W::new(self)
    }
    #[doc = "Bit 18 - 0: DSI trigger signal is a pulse input, 1: DSI trigger signal is a level inpu.t"]
    #[inline(always)]
    pub fn dsi_trigger_level(&mut self) -> DSI_TRIGGER_LEVEL_W {
        DSI_TRIGGER_LEVEL_W::new(self)
    }
    #[doc = "Bit 19 - 0: bypass clock domain synchronisation of the DSI trigger signal, 1: synchronize the DSI trigger signal to the SAR clock domain."]
    #[inline(always)]
    pub fn dsi_sync_trigger(&mut self) -> DSI_SYNC_TRIGGER_W {
        DSI_SYNC_TRIGGER_W::new(self)
    }
    #[doc = "Bit 31 - Enable to output EOS_INTR to DSI"]
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
#[doc = "Sample control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sample_ctrl](index.html) module"]
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
#[doc = "`reset()` method sets SAMPLE_CTRL to value 0"]
impl crate::Resettable for SAMPLE_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
