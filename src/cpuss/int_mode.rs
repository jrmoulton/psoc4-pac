#[doc = "Register `INT_MODE` reader"]
pub struct R(crate::R<INT_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_MODE` writer"]
pub struct W(crate::W<INT_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_MODE_SPEC>;
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
impl From<crate::W<INT_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSI_INT_PULSE` reader - Specifies DSI interrupt format: '0': level sensitive; i.e. no pulse generator. '1': pulse generator on rising edge."]
pub type DSI_INT_PULSE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DSI_INT_PULSE` writer - Specifies DSI interrupt format: '0': level sensitive; i.e. no pulse generator. '1': pulse generator on rising edge."]
pub type DSI_INT_PULSE_W<'a> = crate::FieldWriter<'a, u32, INT_MODE_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Specifies DSI interrupt format: '0': level sensitive; i.e. no pulse generator. '1': pulse generator on rising edge."]
    #[inline(always)]
    pub fn dsi_int_pulse(&self) -> DSI_INT_PULSE_R {
        DSI_INT_PULSE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies DSI interrupt format: '0': level sensitive; i.e. no pulse generator. '1': pulse generator on rising edge."]
    #[inline(always)]
    pub fn dsi_int_pulse(&mut self) -> DSI_INT_PULSE_W {
        DSI_INT_PULSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI interrupt pulse mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_mode](index.html) module"]
pub struct INT_MODE_SPEC;
impl crate::RegisterSpec for INT_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_mode::R](R) reader structure"]
impl crate::Readable for INT_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_mode::W](W) writer structure"]
impl crate::Writable for INT_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_MODE to value 0"]
impl crate::Resettable for INT_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
