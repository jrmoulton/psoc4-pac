#[doc = "Register `MUX_SWITCH_CLEAR0` reader"]
pub struct R(crate::R<MUX_SWITCH_CLEAR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MUX_SWITCH_CLEAR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MUX_SWITCH_CLEAR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MUX_SWITCH_CLEAR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MUX_SWITCH_CLEAR0` writer"]
pub struct W(crate::W<MUX_SWITCH_CLEAR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MUX_SWITCH_CLEAR0_SPEC>;
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
impl From<crate::W<MUX_SWITCH_CLEAR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MUX_SWITCH_CLEAR0_SPEC>) -> Self {
        W(writer)
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
#[doc = "SARMUX Firmware switch control clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mux_switch_clear0](index.html) module"]
pub struct MUX_SWITCH_CLEAR0_SPEC;
impl crate::RegisterSpec for MUX_SWITCH_CLEAR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mux_switch_clear0::R](R) reader structure"]
impl crate::Readable for MUX_SWITCH_CLEAR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mux_switch_clear0::W](W) writer structure"]
impl crate::Writable for MUX_SWITCH_CLEAR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MUX_SWITCH_CLEAR0 to value 0"]
impl crate::Resettable for MUX_SWITCH_CLEAR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
