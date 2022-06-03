#[doc = "Register `TRIM4` reader"]
pub struct R(crate::R<TRIM4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIM4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIM4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIM4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIM4` writer"]
pub struct W(crate::W<TRIM4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIM4_SPEC>;
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
impl From<crate::W<TRIM4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIM4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP2_TRIMB` reader - Trim B for Comparator #2"]
pub type COMP2_TRIMB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP2_TRIMB` writer - Trim B for Comparator #2"]
pub type COMP2_TRIMB_W<'a> = crate::FieldWriter<'a, u32, TRIM4_SPEC, u8, u8, 5, 0>;
impl R {
    #[doc = "Bits 0:4 - Trim B for Comparator #2"]
    #[inline(always)]
    pub fn comp2_trimb(&self) -> COMP2_TRIMB_R {
        COMP2_TRIMB_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Trim B for Comparator #2"]
    #[inline(always)]
    pub fn comp2_trimb(&mut self) -> COMP2_TRIMB_W {
        COMP2_TRIMB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPCOMP Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trim4](index.html) module"]
pub struct TRIM4_SPEC;
impl crate::RegisterSpec for TRIM4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trim4::R](R) reader structure"]
impl crate::Readable for TRIM4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trim4::W](W) writer structure"]
impl crate::Writable for TRIM4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRIM4 to value 0"]
impl crate::Resettable for TRIM4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
