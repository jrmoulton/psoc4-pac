#[doc = "Register `OA0_COMP_TRIM` reader"]
pub struct R(crate::R<OA0_COMP_TRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OA0_COMP_TRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OA0_COMP_TRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OA0_COMP_TRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OA0_COMP_TRIM` writer"]
pub struct W(crate::W<OA0_COMP_TRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OA0_COMP_TRIM_SPEC>;
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
impl From<crate::W<OA0_COMP_TRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OA0_COMP_TRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OA0_COMP_TRIM` reader - Opamp 0 Compensation Capacitor Trim"]
pub type OA0_COMP_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OA0_COMP_TRIM` writer - Opamp 0 Compensation Capacitor Trim"]
pub type OA0_COMP_TRIM_W<'a> = crate::FieldWriter<'a, u32, OA0_COMP_TRIM_SPEC, u8, u8, 2, 0>;
impl R {
    #[doc = "Bits 0:1 - Opamp 0 Compensation Capacitor Trim"]
    #[inline(always)]
    pub fn oa0_comp_trim(&self) -> OA0_COMP_TRIM_R {
        OA0_COMP_TRIM_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Opamp 0 Compensation Capacitor Trim"]
    #[inline(always)]
    pub fn oa0_comp_trim(&mut self) -> OA0_COMP_TRIM_W {
        OA0_COMP_TRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Opamp0 trim control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oa0_comp_trim](index.html) module"]
pub struct OA0_COMP_TRIM_SPEC;
impl crate::RegisterSpec for OA0_COMP_TRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oa0_comp_trim::R](R) reader structure"]
impl crate::Readable for OA0_COMP_TRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oa0_comp_trim::W](W) writer structure"]
impl crate::Writable for OA0_COMP_TRIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OA0_COMP_TRIM to value 0"]
impl crate::Resettable for OA0_COMP_TRIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
