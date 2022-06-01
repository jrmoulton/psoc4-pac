#[doc = "Register `Cy_INTR_MASKED` reader"]
pub struct R(crate::R<CY_INTR_MASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CY_INTR_MASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CY_INTR_MASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CY_INTR_MASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Cy_INTR_MASKED` writer"]
pub struct W(crate::W<CY_INTR_MASKED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CY_INTR_MASKED_SPEC>;
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
impl From<crate::W<CY_INTR_MASKED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CY_INTR_MASKED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP1_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type COMP1_MASKED_R = crate::BitReader<bool>;
#[doc = "Field `COMP2_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type COMP2_MASKED_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn comp1_masked(&self) -> COMP1_MASKED_R {
        COMP1_MASKED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn comp2_masked(&self) -> COMP2_MASKED_R {
        COMP2_MASKED_R::new(((self.bits >> 1) & 1) != 0)
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
#[doc = "LPCOMP interrupt masked\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cy_intr_masked](index.html) module"]
pub struct CY_INTR_MASKED_SPEC;
impl crate::RegisterSpec for CY_INTR_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cy_intr_masked::R](R) reader structure"]
impl crate::Readable for CY_INTR_MASKED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cy_intr_masked::W](W) writer structure"]
impl crate::Writable for CY_INTR_MASKED_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Cy_INTR_MASKED to value 0"]
impl crate::Resettable for CY_INTR_MASKED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
