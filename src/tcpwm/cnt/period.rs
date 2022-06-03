#[doc = "Register `PERIOD` reader"]
pub struct R(crate::R<PERIOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERIOD` writer"]
pub struct W(crate::W<PERIOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERIOD_SPEC>;
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
impl From<crate::W<PERIOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERIOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERIOD` reader - Period value: upper value of the counter. When the counter should count for n cycles, this field should be set to n-1."]
pub type PERIOD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PERIOD` writer - Period value: upper value of the counter. When the counter should count for n cycles, this field should be set to n-1."]
pub type PERIOD_W<'a> = crate::FieldWriter<'a, u32, PERIOD_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Period value: upper value of the counter. When the counter should count for n cycles, this field should be set to n-1."]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Period value: upper value of the counter. When the counter should count for n cycles, this field should be set to n-1."]
    #[inline(always)]
    pub fn period(&mut self) -> PERIOD_W {
        PERIOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter period register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [period](index.html) module"]
pub struct PERIOD_SPEC;
impl crate::RegisterSpec for PERIOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [period::R](R) reader structure"]
impl crate::Readable for PERIOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [period::W](W) writer structure"]
impl crate::Writable for PERIOD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERIOD to value 0xffff"]
impl crate::Resettable for PERIOD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
