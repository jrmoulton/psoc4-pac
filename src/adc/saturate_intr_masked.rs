#[doc = "Register `SATURATE_INTR_MASKED` reader"]
pub struct R(crate::R<SATURATE_INTR_MASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SATURATE_INTR_MASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SATURATE_INTR_MASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SATURATE_INTR_MASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SATURATE_INTR_MASKED` writer"]
pub struct W(crate::W<SATURATE_INTR_MASKED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SATURATE_INTR_MASKED_SPEC>;
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
impl From<crate::W<SATURATE_INTR_MASKED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SATURATE_INTR_MASKED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SATURATE_INTR` reader - Saturate Interrupt"]
pub type SATURATE_INTR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SATURATE_INTR` writer - Saturate Interrupt"]
pub type SATURATE_INTR_W<'a> =
    crate::FieldWriter<'a, u32, SATURATE_INTR_MASKED_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Saturate Interrupt"]
    #[inline(always)]
    pub fn saturate_intr(&self) -> SATURATE_INTR_R {
        SATURATE_INTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Saturate Interrupt"]
    #[inline(always)]
    pub fn saturate_intr(&mut self) -> SATURATE_INTR_W {
        SATURATE_INTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Saturate interrupt masked request register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [saturate_intr_masked](index.html) module"]
pub struct SATURATE_INTR_MASKED_SPEC;
impl crate::RegisterSpec for SATURATE_INTR_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [saturate_intr_masked::R](R) reader structure"]
impl crate::Readable for SATURATE_INTR_MASKED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [saturate_intr_masked::W](W) writer structure"]
impl crate::Writable for SATURATE_INTR_MASKED_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SATURATE_INTR_MASKED to value 0"]
impl crate::Resettable for SATURATE_INTR_MASKED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
