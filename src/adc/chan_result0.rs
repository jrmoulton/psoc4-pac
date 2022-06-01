#[doc = "Register `CHAN_RESULT0` reader"]
pub struct R(crate::R<CHAN_RESULT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHAN_RESULT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHAN_RESULT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHAN_RESULT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHAN_RESULT0` writer"]
pub struct W(crate::W<CHAN_RESULT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHAN_RESULT0_SPEC>;
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
impl From<crate::W<CHAN_RESULT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHAN_RESULT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESULT` reader - SAR conversion result of the channel. The data is copied here from the WORK field after all enabled channels in this scan have been sampled."]
pub type RESULT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SATURATE_INTR_MIR` reader - Mirror bit of corresponding bit in SAR_SATURATE_INTR register"]
pub type SATURATE_INTR_MIR_R = crate::BitReader<bool>;
#[doc = "Field `RANGE_INTR_MIR` reader - Mirror bit of corresponding bit in SAR_RANGE_INTR register"]
pub type RANGE_INTR_MIR_R = crate::BitReader<bool>;
#[doc = "Field `CHAN_RESULT_VALID_MIR` reader - Mirror bit of corresponding bit in SAR_SATURATE_INTR register"]
pub type CHAN_RESULT_VALID_MIR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:15 - SAR conversion result of the channel. The data is copied here from the WORK field after all enabled channels in this scan have been sampled."]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 29 - Mirror bit of corresponding bit in SAR_SATURATE_INTR register"]
    #[inline(always)]
    pub fn saturate_intr_mir(&self) -> SATURATE_INTR_MIR_R {
        SATURATE_INTR_MIR_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Mirror bit of corresponding bit in SAR_RANGE_INTR register"]
    #[inline(always)]
    pub fn range_intr_mir(&self) -> RANGE_INTR_MIR_R {
        RANGE_INTR_MIR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Mirror bit of corresponding bit in SAR_SATURATE_INTR register"]
    #[inline(always)]
    pub fn chan_result_valid_mir(&self) -> CHAN_RESULT_VALID_MIR_R {
        CHAN_RESULT_VALID_MIR_R::new(((self.bits >> 31) & 1) != 0)
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
#[doc = "Channel0 result data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chan_result0](index.html) module"]
pub struct CHAN_RESULT0_SPEC;
impl crate::RegisterSpec for CHAN_RESULT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chan_result0::R](R) reader structure"]
impl crate::Readable for CHAN_RESULT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chan_result0::W](W) writer structure"]
impl crate::Writable for CHAN_RESULT0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHAN_RESULT0 to value 0"]
impl crate::Resettable for CHAN_RESULT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
