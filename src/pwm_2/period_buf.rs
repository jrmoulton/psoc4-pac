#[doc = "Register `PERIOD_BUF` reader"]
pub struct R(crate::R<PERIOD_BUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIOD_BUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIOD_BUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIOD_BUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERIOD_BUF` writer"]
pub struct W(crate::W<PERIOD_BUF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERIOD_BUF_SPEC>;
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
impl From<crate::W<PERIOD_BUF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERIOD_BUF_SPEC>) -> Self {
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
#[doc = "Period buffer value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [period_buf](index.html) module"]
pub struct PERIOD_BUF_SPEC;
impl crate::RegisterSpec for PERIOD_BUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [period_buf::R](R) reader structure"]
impl crate::Readable for PERIOD_BUF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [period_buf::W](W) writer structure"]
impl crate::Writable for PERIOD_BUF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERIOD_BUF to value 0"]
impl crate::Resettable for PERIOD_BUF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
