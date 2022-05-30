#[doc = "Register `DMAC_INTR_MASK` reader"]
pub struct R(crate::R<DMAC_INTR_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_INTR_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_INTR_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_INTR_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAC_INTR_MASK` writer"]
pub struct W(crate::W<DMAC_INTR_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAC_INTR_MASK_SPEC>;
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
impl From<crate::W<DMAC_INTR_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAC_INTR_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH` reader - Mask for corresponding field in INTR register."]
pub type CH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CH` writer - Mask for corresponding field in INTR register."]
pub type CH_W<'a> = crate::FieldWriter<'a, u32, DMAC_INTR_MASK_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    pub fn ch(&self) -> CH_R {
        CH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask for corresponding field in INTR register."]
    #[inline(always)]
    pub fn ch(&mut self) -> CH_W {
        CH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_intr_mask](index.html) module"]
pub struct DMAC_INTR_MASK_SPEC;
impl crate::RegisterSpec for DMAC_INTR_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_intr_mask::R](R) reader structure"]
impl crate::Readable for DMAC_INTR_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmac_intr_mask::W](W) writer structure"]
impl crate::Writable for DMAC_INTR_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAC_INTR_MASK to value 0"]
impl crate::Resettable for DMAC_INTR_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
