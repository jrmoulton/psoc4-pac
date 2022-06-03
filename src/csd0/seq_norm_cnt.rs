#[doc = "Register `SEQ_NORM_CNT` reader"]
pub struct R(crate::R<SEQ_NORM_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEQ_NORM_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEQ_NORM_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEQ_NORM_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEQ_NORM_CNT` writer"]
pub struct W(crate::W<SEQ_NORM_CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEQ_NORM_CNT_SPEC>;
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
impl From<crate::W<SEQ_NORM_CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEQ_NORM_CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONV_CNT` reader - Number of conversion per sample, if set to 0 the Sample_norm state will be skipped. Sample window size = SEQ_NORM_CNT.CONV_CNT * (SENSE_PERIOD.SENSE_DIV+1). Note for CSDv1 Sample window size = PERIOD"]
pub type CONV_CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CONV_CNT` writer - Number of conversion per sample, if set to 0 the Sample_norm state will be skipped. Sample window size = SEQ_NORM_CNT.CONV_CNT * (SENSE_PERIOD.SENSE_DIV+1). Note for CSDv1 Sample window size = PERIOD"]
pub type CONV_CNT_W<'a> = crate::FieldWriter<'a, u32, SEQ_NORM_CNT_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Number of conversion per sample, if set to 0 the Sample_norm state will be skipped. Sample window size = SEQ_NORM_CNT.CONV_CNT * (SENSE_PERIOD.SENSE_DIV+1). Note for CSDv1 Sample window size = PERIOD"]
    #[inline(always)]
    pub fn conv_cnt(&self) -> CONV_CNT_R {
        CONV_CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of conversion per sample, if set to 0 the Sample_norm state will be skipped. Sample window size = SEQ_NORM_CNT.CONV_CNT * (SENSE_PERIOD.SENSE_DIV+1). Note for CSDv1 Sample window size = PERIOD"]
    #[inline(always)]
    pub fn conv_cnt(&mut self) -> CONV_CNT_W {
        CONV_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sequencer Normal conversion and sample counts\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq_norm_cnt](index.html) module"]
pub struct SEQ_NORM_CNT_SPEC;
impl crate::RegisterSpec for SEQ_NORM_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seq_norm_cnt::R](R) reader structure"]
impl crate::Readable for SEQ_NORM_CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seq_norm_cnt::W](W) writer structure"]
impl crate::Writable for SEQ_NORM_CNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEQ_NORM_CNT to value 0"]
impl crate::Resettable for SEQ_NORM_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
