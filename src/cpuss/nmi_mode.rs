#[doc = "Register `NMI_MODE` reader"]
pub struct R(crate::R<NMI_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NMI_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NMI_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NMI_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NMI_MODE` writer"]
pub struct W(crate::W<NMI_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NMI_MODE_SPEC>;
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
impl From<crate::W<NMI_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NMI_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSI_NMI_PULSE` reader - Specifies DSI NMI format: '0': level sensitive; i.e. no pulse generator. '1': pulse generator on rising edge."]
pub type DSI_NMI_PULSE_R = crate::BitReader<bool>;
#[doc = "Field `DSI_NMI_PULSE` writer - Specifies DSI NMI format: '0': level sensitive; i.e. no pulse generator. '1': pulse generator on rising edge."]
pub type DSI_NMI_PULSE_W<'a> = crate::BitWriter<'a, u32, NMI_MODE_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Specifies DSI NMI format: '0': level sensitive; i.e. no pulse generator. '1': pulse generator on rising edge."]
    #[inline(always)]
    pub fn dsi_nmi_pulse(&self) -> DSI_NMI_PULSE_R {
        DSI_NMI_PULSE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Specifies DSI NMI format: '0': level sensitive; i.e. no pulse generator. '1': pulse generator on rising edge."]
    #[inline(always)]
    pub fn dsi_nmi_pulse(&mut self) -> DSI_NMI_PULSE_W {
        DSI_NMI_PULSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI NMI pulse mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nmi_mode](index.html) module"]
pub struct NMI_MODE_SPEC;
impl crate::RegisterSpec for NMI_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nmi_mode::R](R) reader structure"]
impl crate::Readable for NMI_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nmi_mode::W](W) writer structure"]
impl crate::Writable for NMI_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NMI_MODE to value 0"]
impl crate::Resettable for NMI_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
