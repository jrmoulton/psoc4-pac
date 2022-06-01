#[doc = "Register `WOUNDING` reader"]
pub struct R(crate::R<WOUNDING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WOUNDING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WOUNDING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WOUNDING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WOUNDING` writer"]
pub struct W(crate::W<WOUNDING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WOUNDING_SPEC>;
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
impl From<crate::W<WOUNDING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WOUNDING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WOUND_RESOLUTION` reader - Maximum SAR resolution allowed"]
pub type WOUND_RESOLUTION_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - Maximum SAR resolution allowed"]
    #[inline(always)]
    pub fn wound_resolution(&self) -> WOUND_RESOLUTION_R {
        WOUND_RESOLUTION_R::new((self.bits & 3) as u8)
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
#[doc = "SAR wounding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wounding](index.html) module"]
pub struct WOUNDING_SPEC;
impl crate::RegisterSpec for WOUNDING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wounding::R](R) reader structure"]
impl crate::Readable for WOUNDING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wounding::W](W) writer structure"]
impl crate::Writable for WOUNDING_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WOUNDING to value 0"]
impl crate::Resettable for WOUNDING_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
