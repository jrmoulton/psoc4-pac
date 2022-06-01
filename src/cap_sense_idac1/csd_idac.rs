#[doc = "Register `CSD_IDAC` reader"]
pub struct R(crate::R<CSD_IDAC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSD_IDAC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSD_IDAC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSD_IDAC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSD_IDAC` writer"]
pub struct W(crate::W<CSD_IDAC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSD_IDAC_SPEC>;
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
impl From<crate::W<CSD_IDAC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSD_IDAC_SPEC>) -> Self {
        W(writer)
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
#[doc = "CSD IDAC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csd_idac](index.html) module"]
pub struct CSD_IDAC_SPEC;
impl crate::RegisterSpec for CSD_IDAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csd_idac::R](R) reader structure"]
impl crate::Readable for CSD_IDAC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csd_idac::W](W) writer structure"]
impl crate::Writable for CSD_IDAC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSD_IDAC to value 0"]
impl crate::Resettable for CSD_IDAC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
