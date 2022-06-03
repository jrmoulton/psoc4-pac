#[doc = "Register `TRIM2` reader"]
pub struct R(crate::R<TRIM2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIM2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIM2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIM2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIM2` writer"]
pub struct W(crate::W<TRIM2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIM2_SPEC>;
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
impl From<crate::W<TRIM2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIM2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP1_TRIMB` reader - Trim B for Comparator #1"]
pub type COMP1_TRIMB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP1_TRIMB` writer - Trim B for Comparator #1"]
pub type COMP1_TRIMB_W<'a> = crate::FieldWriter<'a, u32, TRIM2_SPEC, u8, u8, 5, 0>;
impl R {
    #[doc = "Bits 0:4 - Trim B for Comparator #1"]
    #[inline(always)]
    pub fn comp1_trimb(&self) -> COMP1_TRIMB_R {
        COMP1_TRIMB_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Trim B for Comparator #1"]
    #[inline(always)]
    pub fn comp1_trimb(&mut self) -> COMP1_TRIMB_W {
        COMP1_TRIMB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPCOMP Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trim2](index.html) module"]
pub struct TRIM2_SPEC;
impl crate::RegisterSpec for TRIM2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trim2::R](R) reader structure"]
impl crate::Readable for TRIM2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trim2::W](W) writer structure"]
impl crate::Writable for TRIM2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRIM2 to value 0"]
impl crate::Resettable for TRIM2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
