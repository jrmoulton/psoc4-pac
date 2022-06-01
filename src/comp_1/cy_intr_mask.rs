#[doc = "Register `Cy_INTR_MASK` reader"]
pub struct R(crate::R<CY_INTR_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CY_INTR_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CY_INTR_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CY_INTR_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Cy_INTR_MASK` writer"]
pub struct W(crate::W<CY_INTR_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CY_INTR_MASK_SPEC>;
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
impl From<crate::W<CY_INTR_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CY_INTR_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP1_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type COMP1_MASK_R = crate::BitReader<bool>;
#[doc = "Field `COMP1_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type COMP1_MASK_W<'a> = crate::BitWriter<'a, u32, CY_INTR_MASK_SPEC, bool, 0>;
#[doc = "Field `COMP2_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type COMP2_MASK_R = crate::BitReader<bool>;
#[doc = "Field `COMP2_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type COMP2_MASK_W<'a> = crate::BitWriter<'a, u32, CY_INTR_MASK_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn comp1_mask(&self) -> COMP1_MASK_R {
        COMP1_MASK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn comp2_mask(&self) -> COMP2_MASK_R {
        COMP2_MASK_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn comp1_mask(&mut self) -> COMP1_MASK_W {
        COMP1_MASK_W::new(self)
    }
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn comp2_mask(&mut self) -> COMP2_MASK_W {
        COMP2_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPCOMP interrupt mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cy_intr_mask](index.html) module"]
pub struct CY_INTR_MASK_SPEC;
impl crate::RegisterSpec for CY_INTR_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cy_intr_mask::R](R) reader structure"]
impl crate::Readable for CY_INTR_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cy_intr_mask::W](W) writer structure"]
impl crate::Writable for CY_INTR_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Cy_INTR_MASK to value 0"]
impl crate::Resettable for CY_INTR_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
