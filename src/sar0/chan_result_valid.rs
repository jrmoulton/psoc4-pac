#[doc = "Register `CHAN_RESULT_VALID` reader"]
pub struct R(crate::R<CHAN_RESULT_VALID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHAN_RESULT_VALID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHAN_RESULT_VALID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHAN_RESULT_VALID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CHAN_RESULT_VALID` reader - If set the corresponding RESULT data is valid, i.e. was sampled during the last scan."]
pub type CHAN_RESULT_VALID_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - If set the corresponding RESULT data is valid, i.e. was sampled during the last scan."]
    #[inline(always)]
    pub fn chan_result_valid(&self) -> CHAN_RESULT_VALID_R {
        CHAN_RESULT_VALID_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Channel result data register valid bits\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chan_result_valid](index.html) module"]
pub struct CHAN_RESULT_VALID_SPEC;
impl crate::RegisterSpec for CHAN_RESULT_VALID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chan_result_valid::R](R) reader structure"]
impl crate::Readable for CHAN_RESULT_VALID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CHAN_RESULT_VALID to value 0"]
impl crate::Resettable for CHAN_RESULT_VALID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
