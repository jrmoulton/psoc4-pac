#[doc = "Register `ANA_TRIM` reader"]
pub struct R(crate::R<ANA_TRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANA_TRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANA_TRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANA_TRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANA_TRIM` writer"]
pub struct W(crate::W<ANA_TRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANA_TRIM_SPEC>;
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
impl From<crate::W<ANA_TRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANA_TRIM_SPEC>) -> Self {
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
#[doc = "Analog trim register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana_trim](index.html) module"]
pub struct ANA_TRIM_SPEC;
impl crate::RegisterSpec for ANA_TRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ana_trim::R](R) reader structure"]
impl crate::Readable for ANA_TRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ana_trim::W](W) writer structure"]
impl crate::Writable for ANA_TRIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ANA_TRIM to value 0"]
impl crate::Resettable for ANA_TRIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
