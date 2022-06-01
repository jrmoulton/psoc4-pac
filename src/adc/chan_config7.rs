#[doc = "Register `CHAN_CONFIG7` reader"]
pub struct R(crate::R<CHAN_CONFIG7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHAN_CONFIG7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHAN_CONFIG7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHAN_CONFIG7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHAN_CONFIG7` writer"]
pub struct W(crate::W<CHAN_CONFIG7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHAN_CONFIG7_SPEC>;
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
impl From<crate::W<CHAN_CONFIG7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHAN_CONFIG7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIN_ADDR` reader - Address of the pin to be sampled by this channel"]
pub type PIN_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN_ADDR` writer - Address of the pin to be sampled by this channel"]
pub type PIN_ADDR_W<'a> = crate::FieldWriter<'a, u32, CHAN_CONFIG7_SPEC, u8, u8, 3, 0>;
#[doc = "Field `PORT_ADDR` reader - Address of the port that contains the pin to be sampled by this channel"]
pub type PORT_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PORT_ADDR` writer - Address of the port that contains the pin to be sampled by this channel"]
pub type PORT_ADDR_W<'a> = crate::FieldWriter<'a, u32, CHAN_CONFIG7_SPEC, u8, u8, 3, 4>;
#[doc = "Field `DIFFERENTIAL_EN` reader - Differential enable for this channel"]
pub type DIFFERENTIAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `DIFFERENTIAL_EN` writer - Differential enable for this channel"]
pub type DIFFERENTIAL_EN_W<'a> = crate::BitWriter<'a, u32, CHAN_CONFIG7_SPEC, bool, 8>;
#[doc = "Field `RESOLUTION` reader - Resolution for this channel"]
pub type RESOLUTION_R = crate::BitReader<bool>;
#[doc = "Field `RESOLUTION` writer - Resolution for this channel"]
pub type RESOLUTION_W<'a> = crate::BitWriter<'a, u32, CHAN_CONFIG7_SPEC, bool, 9>;
#[doc = "Field `AVG_EN` reader - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)."]
pub type AVG_EN_R = crate::BitReader<bool>;
#[doc = "Field `AVG_EN` writer - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)."]
pub type AVG_EN_W<'a> = crate::BitWriter<'a, u32, CHAN_CONFIG7_SPEC, bool, 10>;
#[doc = "Field `SAMPLE_TIME_SEL` reader - Sample time select: select which of the 4 global sample times to use for this channel."]
pub type SAMPLE_TIME_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMPLE_TIME_SEL` writer - Sample time select: select which of the 4 global sample times to use for this channel."]
pub type SAMPLE_TIME_SEL_W<'a> = crate::FieldWriter<'a, u32, CHAN_CONFIG7_SPEC, u8, u8, 2, 12>;
#[doc = "Field `DSI_OUT_EN` reader - DSI data output enable for this channel"]
pub type DSI_OUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `DSI_OUT_EN` writer - DSI data output enable for this channel"]
pub type DSI_OUT_EN_W<'a> = crate::BitWriter<'a, u32, CHAN_CONFIG7_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:2 - Address of the pin to be sampled by this channel"]
    #[inline(always)]
    pub fn pin_addr(&self) -> PIN_ADDR_R {
        PIN_ADDR_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Address of the port that contains the pin to be sampled by this channel"]
    #[inline(always)]
    pub fn port_addr(&self) -> PORT_ADDR_R {
        PORT_ADDR_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Differential enable for this channel"]
    #[inline(always)]
    pub fn differential_en(&self) -> DIFFERENTIAL_EN_R {
        DIFFERENTIAL_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Resolution for this channel"]
    #[inline(always)]
    pub fn resolution(&self) -> RESOLUTION_R {
        RESOLUTION_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)."]
    #[inline(always)]
    pub fn avg_en(&self) -> AVG_EN_R {
        AVG_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Sample time select: select which of the 4 global sample times to use for this channel."]
    #[inline(always)]
    pub fn sample_time_sel(&self) -> SAMPLE_TIME_SEL_R {
        SAMPLE_TIME_SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 31 - DSI data output enable for this channel"]
    #[inline(always)]
    pub fn dsi_out_en(&self) -> DSI_OUT_EN_R {
        DSI_OUT_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Address of the pin to be sampled by this channel"]
    #[inline(always)]
    pub fn pin_addr(&mut self) -> PIN_ADDR_W {
        PIN_ADDR_W::new(self)
    }
    #[doc = "Bits 4:6 - Address of the port that contains the pin to be sampled by this channel"]
    #[inline(always)]
    pub fn port_addr(&mut self) -> PORT_ADDR_W {
        PORT_ADDR_W::new(self)
    }
    #[doc = "Bit 8 - Differential enable for this channel"]
    #[inline(always)]
    pub fn differential_en(&mut self) -> DIFFERENTIAL_EN_W {
        DIFFERENTIAL_EN_W::new(self)
    }
    #[doc = "Bit 9 - Resolution for this channel"]
    #[inline(always)]
    pub fn resolution(&mut self) -> RESOLUTION_W {
        RESOLUTION_W::new(self)
    }
    #[doc = "Bit 10 - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)."]
    #[inline(always)]
    pub fn avg_en(&mut self) -> AVG_EN_W {
        AVG_EN_W::new(self)
    }
    #[doc = "Bits 12:13 - Sample time select: select which of the 4 global sample times to use for this channel."]
    #[inline(always)]
    pub fn sample_time_sel(&mut self) -> SAMPLE_TIME_SEL_W {
        SAMPLE_TIME_SEL_W::new(self)
    }
    #[doc = "Bit 31 - DSI data output enable for this channel"]
    #[inline(always)]
    pub fn dsi_out_en(&mut self) -> DSI_OUT_EN_W {
        DSI_OUT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel7 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chan_config7](index.html) module"]
pub struct CHAN_CONFIG7_SPEC;
impl crate::RegisterSpec for CHAN_CONFIG7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chan_config7::R](R) reader structure"]
impl crate::Readable for CHAN_CONFIG7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chan_config7::W](W) writer structure"]
impl crate::Writable for CHAN_CONFIG7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHAN_CONFIG7 to value 0"]
impl crate::Resettable for CHAN_CONFIG7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
