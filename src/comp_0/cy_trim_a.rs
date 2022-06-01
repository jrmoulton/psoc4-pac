#[doc = "Register `Cy_TRIM_A` reader"]
pub struct R(crate::R<CY_TRIM_A_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CY_TRIM_A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CY_TRIM_A_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CY_TRIM_A_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Cy_TRIM_A` writer"]
pub struct W(crate::W<CY_TRIM_A_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CY_TRIM_A_SPEC>;
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
impl From<crate::W<CY_TRIM_A_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CY_TRIM_A_SPEC>) -> Self {
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
#[doc = "LPCOMP trim A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cy_trim_a](index.html) module"]
pub struct CY_TRIM_A_SPEC;
impl crate::RegisterSpec for CY_TRIM_A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cy_trim_a::R](R) reader structure"]
impl crate::Readable for CY_TRIM_A_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cy_trim_a::W](W) writer structure"]
impl crate::Writable for CY_TRIM_A_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Cy_TRIM_A to value 0"]
impl crate::Resettable for CY_TRIM_A_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
