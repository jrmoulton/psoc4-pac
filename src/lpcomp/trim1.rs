#[doc = "Register `TRIM1` reader"]
pub struct R(crate::R<TRIM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIM1` writer"]
pub struct W(crate::W<TRIM1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIM1_SPEC>;
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
impl From<crate::W<TRIM1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIM1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP1_TRIMA` reader - Trim A for Comparator #1"]
pub type COMP1_TRIMA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP1_TRIMA` writer - Trim A for Comparator #1"]
pub type COMP1_TRIMA_W<'a> = crate::FieldWriter<'a, u32, TRIM1_SPEC, u8, u8, 5, 0>;
impl R {
    #[doc = "Bits 0:4 - Trim A for Comparator #1"]
    #[inline(always)]
    pub fn comp1_trima(&self) -> COMP1_TRIMA_R {
        COMP1_TRIMA_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Trim A for Comparator #1"]
    #[inline(always)]
    pub fn comp1_trima(&mut self) -> COMP1_TRIMA_W {
        COMP1_TRIMA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPCOMP Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trim1](index.html) module"]
pub struct TRIM1_SPEC;
impl crate::RegisterSpec for TRIM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trim1::R](R) reader structure"]
impl crate::Readable for TRIM1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trim1::W](W) writer structure"]
impl crate::Writable for TRIM1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRIM1 to value 0"]
impl crate::Resettable for TRIM1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
