#[doc = "Register `INJ_CHAN_CONFIG` reader"]
pub struct R(crate::R<INJ_CHAN_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INJ_CHAN_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INJ_CHAN_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INJ_CHAN_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INJ_CHAN_CONFIG` writer"]
pub struct W(crate::W<INJ_CHAN_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INJ_CHAN_CONFIG_SPEC>;
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
impl From<crate::W<INJ_CHAN_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INJ_CHAN_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INJ_PIN_ADDR` reader - Address of the pin to be sampled by this channel"]
pub type INJ_PIN_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INJ_PIN_ADDR` writer - Address of the pin to be sampled by this channel"]
pub type INJ_PIN_ADDR_W<'a> = crate::FieldWriter<'a, u32, INJ_CHAN_CONFIG_SPEC, u8, u8, 3, 0>;
#[doc = "Field `INJ_PORT_ADDR` reader - Address of the port that contains the pin to be sampled by this channel"]
pub type INJ_PORT_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INJ_PORT_ADDR` writer - Address of the port that contains the pin to be sampled by this channel"]
pub type INJ_PORT_ADDR_W<'a> = crate::FieldWriter<'a, u32, INJ_CHAN_CONFIG_SPEC, u8, u8, 3, 4>;
#[doc = "Field `INJ_DIFFERENTIAL_EN` reader - Differential enable for this channel"]
pub type INJ_DIFFERENTIAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `INJ_DIFFERENTIAL_EN` writer - Differential enable for this channel"]
pub type INJ_DIFFERENTIAL_EN_W<'a> = crate::BitWriter<'a, u32, INJ_CHAN_CONFIG_SPEC, bool, 8>;
#[doc = "Field `INJ_RESOLUTION` reader - Resolution for this channel"]
pub type INJ_RESOLUTION_R = crate::BitReader<bool>;
#[doc = "Field `INJ_RESOLUTION` writer - Resolution for this channel"]
pub type INJ_RESOLUTION_W<'a> = crate::BitWriter<'a, u32, INJ_CHAN_CONFIG_SPEC, bool, 9>;
#[doc = "Field `INJ_AVG_EN` reader - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)."]
pub type INJ_AVG_EN_R = crate::BitReader<bool>;
#[doc = "Field `INJ_AVG_EN` writer - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)."]
pub type INJ_AVG_EN_W<'a> = crate::BitWriter<'a, u32, INJ_CHAN_CONFIG_SPEC, bool, 10>;
#[doc = "Field `INJ_SAMPLE_TIME_SEL` reader - Sample time select: select which of the 4 global sample times to use for this channel."]
pub type INJ_SAMPLE_TIME_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INJ_SAMPLE_TIME_SEL` writer - Sample time select: select which of the 4 global sample times to use for this channel."]
pub type INJ_SAMPLE_TIME_SEL_W<'a> =
    crate::FieldWriter<'a, u32, INJ_CHAN_CONFIG_SPEC, u8, u8, 2, 12>;
#[doc = "Field `INJ_TAILGATING` reader - Injection channel tailgating"]
pub type INJ_TAILGATING_R = crate::BitReader<bool>;
#[doc = "Field `INJ_TAILGATING` writer - Injection channel tailgating"]
pub type INJ_TAILGATING_W<'a> = crate::BitWriter<'a, u32, INJ_CHAN_CONFIG_SPEC, bool, 30>;
#[doc = "Field `INJ_START_EN` reader - Set by firmware to enable the injection channel"]
pub type INJ_START_EN_R = crate::BitReader<bool>;
#[doc = "Field `INJ_START_EN` writer - Set by firmware to enable the injection channel"]
pub type INJ_START_EN_W<'a> = crate::BitWriter<'a, u32, INJ_CHAN_CONFIG_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:2 - Address of the pin to be sampled by this channel"]
    #[inline(always)]
    pub fn inj_pin_addr(&self) -> INJ_PIN_ADDR_R {
        INJ_PIN_ADDR_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Address of the port that contains the pin to be sampled by this channel"]
    #[inline(always)]
    pub fn inj_port_addr(&self) -> INJ_PORT_ADDR_R {
        INJ_PORT_ADDR_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Differential enable for this channel"]
    #[inline(always)]
    pub fn inj_differential_en(&self) -> INJ_DIFFERENTIAL_EN_R {
        INJ_DIFFERENTIAL_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Resolution for this channel"]
    #[inline(always)]
    pub fn inj_resolution(&self) -> INJ_RESOLUTION_R {
        INJ_RESOLUTION_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)."]
    #[inline(always)]
    pub fn inj_avg_en(&self) -> INJ_AVG_EN_R {
        INJ_AVG_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Sample time select: select which of the 4 global sample times to use for this channel."]
    #[inline(always)]
    pub fn inj_sample_time_sel(&self) -> INJ_SAMPLE_TIME_SEL_R {
        INJ_SAMPLE_TIME_SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 30 - Injection channel tailgating"]
    #[inline(always)]
    pub fn inj_tailgating(&self) -> INJ_TAILGATING_R {
        INJ_TAILGATING_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Set by firmware to enable the injection channel"]
    #[inline(always)]
    pub fn inj_start_en(&self) -> INJ_START_EN_R {
        INJ_START_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Address of the pin to be sampled by this channel"]
    #[inline(always)]
    pub fn inj_pin_addr(&mut self) -> INJ_PIN_ADDR_W {
        INJ_PIN_ADDR_W::new(self)
    }
    #[doc = "Bits 4:6 - Address of the port that contains the pin to be sampled by this channel"]
    #[inline(always)]
    pub fn inj_port_addr(&mut self) -> INJ_PORT_ADDR_W {
        INJ_PORT_ADDR_W::new(self)
    }
    #[doc = "Bit 8 - Differential enable for this channel"]
    #[inline(always)]
    pub fn inj_differential_en(&mut self) -> INJ_DIFFERENTIAL_EN_W {
        INJ_DIFFERENTIAL_EN_W::new(self)
    }
    #[doc = "Bit 9 - Resolution for this channel"]
    #[inline(always)]
    pub fn inj_resolution(&mut self) -> INJ_RESOLUTION_W {
        INJ_RESOLUTION_W::new(self)
    }
    #[doc = "Bit 10 - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)."]
    #[inline(always)]
    pub fn inj_avg_en(&mut self) -> INJ_AVG_EN_W {
        INJ_AVG_EN_W::new(self)
    }
    #[doc = "Bits 12:13 - Sample time select: select which of the 4 global sample times to use for this channel."]
    #[inline(always)]
    pub fn inj_sample_time_sel(&mut self) -> INJ_SAMPLE_TIME_SEL_W {
        INJ_SAMPLE_TIME_SEL_W::new(self)
    }
    #[doc = "Bit 30 - Injection channel tailgating"]
    #[inline(always)]
    pub fn inj_tailgating(&mut self) -> INJ_TAILGATING_W {
        INJ_TAILGATING_W::new(self)
    }
    #[doc = "Bit 31 - Set by firmware to enable the injection channel"]
    #[inline(always)]
    pub fn inj_start_en(&mut self) -> INJ_START_EN_W {
        INJ_START_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Injection channel configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inj_chan_config](index.html) module"]
pub struct INJ_CHAN_CONFIG_SPEC;
impl crate::RegisterSpec for INJ_CHAN_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inj_chan_config::R](R) reader structure"]
impl crate::Readable for INJ_CHAN_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inj_chan_config::W](W) writer structure"]
impl crate::Writable for INJ_CHAN_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INJ_CHAN_CONFIG to value 0"]
impl crate::Resettable for INJ_CHAN_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
