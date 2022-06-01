#[doc = "Register `AVG_STAT` reader"]
pub struct R(crate::R<AVG_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AVG_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AVG_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AVG_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AVG_STAT` writer"]
pub struct W(crate::W<AVG_STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AVG_STAT_SPEC>;
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
impl From<crate::W<AVG_STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AVG_STAT_SPEC>) -> Self {
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
#[doc = "Current averaging status (for debug)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [avg_stat](index.html) module"]
pub struct AVG_STAT_SPEC;
impl crate::RegisterSpec for AVG_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [avg_stat::R](R) reader structure"]
impl crate::Readable for AVG_STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [avg_stat::W](W) writer structure"]
impl crate::Writable for AVG_STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AVG_STAT to value 0"]
impl crate::Resettable for AVG_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}