#[doc = "Register `SW_DSI_SEL` reader"]
pub struct R(crate::R<SW_DSI_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SW_DSI_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SW_DSI_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SW_DSI_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SW_DSI_SEL` writer"]
pub struct W(crate::W<SW_DSI_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SW_DSI_SEL_SPEC>;
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
impl From<crate::W<SW_DSI_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SW_DSI_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSI_CSH_TANK` reader - Select waveform for dsi_csh_tank signal (called dsi_cap_lo_en in CDSv1). For CSX when DUAL_CAP_EN is set this signal will have the special functionality to go low one clk_hf cycle ahead of the end of the corresponding csd_sense phase (just like for CSDv1), in all other use-cases the functionality is the same as for other switch controls."]
pub type DSI_CSH_TANK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DSI_CSH_TANK` writer - Select waveform for dsi_csh_tank signal (called dsi_cap_lo_en in CDSv1). For CSX when DUAL_CAP_EN is set this signal will have the special functionality to go low one clk_hf cycle ahead of the end of the corresponding csd_sense phase (just like for CSDv1), in all other use-cases the functionality is the same as for other switch controls."]
pub type DSI_CSH_TANK_W<'a> = crate::FieldWriter<'a, u32, SW_DSI_SEL_SPEC, u8, u8, 3, 0>;
#[doc = "Field `DSI_CMOD` reader - Select waveform for dsi_cmod signal (called dsi_cap_hi_en in CDSv1). For CSX when DUAL_CAP_EN is set this signal will have the special functionality to go low one clk_hf cycle ahead of the end of the corresponding csd_sense phase (just like for CSDv1), in all other use-cases the functionality is the same as for other switch controls."]
pub type DSI_CMOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DSI_CMOD` writer - Select waveform for dsi_cmod signal (called dsi_cap_hi_en in CDSv1). For CSX when DUAL_CAP_EN is set this signal will have the special functionality to go low one clk_hf cycle ahead of the end of the corresponding csd_sense phase (just like for CSDv1), in all other use-cases the functionality is the same as for other switch controls."]
pub type DSI_CMOD_W<'a> = crate::FieldWriter<'a, u32, SW_DSI_SEL_SPEC, u8, u8, 3, 4>;
impl R {
    #[doc = "Bits 0:2 - Select waveform for dsi_csh_tank signal (called dsi_cap_lo_en in CDSv1). For CSX when DUAL_CAP_EN is set this signal will have the special functionality to go low one clk_hf cycle ahead of the end of the corresponding csd_sense phase (just like for CSDv1), in all other use-cases the functionality is the same as for other switch controls."]
    #[inline(always)]
    pub fn dsi_csh_tank(&self) -> DSI_CSH_TANK_R {
        DSI_CSH_TANK_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Select waveform for dsi_cmod signal (called dsi_cap_hi_en in CDSv1). For CSX when DUAL_CAP_EN is set this signal will have the special functionality to go low one clk_hf cycle ahead of the end of the corresponding csd_sense phase (just like for CSDv1), in all other use-cases the functionality is the same as for other switch controls."]
    #[inline(always)]
    pub fn dsi_cmod(&self) -> DSI_CMOD_R {
        DSI_CMOD_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Select waveform for dsi_csh_tank signal (called dsi_cap_lo_en in CDSv1). For CSX when DUAL_CAP_EN is set this signal will have the special functionality to go low one clk_hf cycle ahead of the end of the corresponding csd_sense phase (just like for CSDv1), in all other use-cases the functionality is the same as for other switch controls."]
    #[inline(always)]
    pub fn dsi_csh_tank(&mut self) -> DSI_CSH_TANK_W {
        DSI_CSH_TANK_W::new(self)
    }
    #[doc = "Bits 4:6 - Select waveform for dsi_cmod signal (called dsi_cap_hi_en in CDSv1). For CSX when DUAL_CAP_EN is set this signal will have the special functionality to go low one clk_hf cycle ahead of the end of the corresponding csd_sense phase (just like for CSDv1), in all other use-cases the functionality is the same as for other switch controls."]
    #[inline(always)]
    pub fn dsi_cmod(&mut self) -> DSI_CMOD_W {
        DSI_CMOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI output switch control Waveform selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_dsi_sel](index.html) module"]
pub struct SW_DSI_SEL_SPEC;
impl crate::RegisterSpec for SW_DSI_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sw_dsi_sel::R](R) reader structure"]
impl crate::Readable for SW_DSI_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sw_dsi_sel::W](W) writer structure"]
impl crate::Writable for SW_DSI_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SW_DSI_SEL to value 0"]
impl crate::Resettable for SW_DSI_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
