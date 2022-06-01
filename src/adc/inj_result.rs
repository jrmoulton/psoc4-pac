#[doc = "Register `INJ_RESULT` reader"]
pub struct R(crate::R<INJ_RESULT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INJ_RESULT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INJ_RESULT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INJ_RESULT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INJ_RESULT` writer"]
pub struct W(crate::W<INJ_RESULT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INJ_RESULT_SPEC>;
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
impl From<crate::W<INJ_RESULT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INJ_RESULT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INJ_RESULT` reader - SAR conversion result of the channel"]
pub type INJ_RESULT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INJ_COLLISION_INTR_MIR` reader - Mirror bit of corresponding bit in SAR_INTR register"]
pub type INJ_COLLISION_INTR_MIR_R = crate::BitReader<bool>;
#[doc = "Field `INJ_SATURATE_INTR_MIR` reader - Mirror bit of corresponding bit in SAR_INTR register"]
pub type INJ_SATURATE_INTR_MIR_R = crate::BitReader<bool>;
#[doc = "Field `INJ_RANGE_INTR_MIR` reader - Mirror bit of corresponding bit in SAR_INTR register"]
pub type INJ_RANGE_INTR_MIR_R = crate::BitReader<bool>;
#[doc = "Field `INJ_EOC_INTR_MIR` reader - Mirror bit of corresponding bit in SAR_INTR register"]
pub type INJ_EOC_INTR_MIR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:15 - SAR conversion result of the channel"]
    #[inline(always)]
    pub fn inj_result(&self) -> INJ_RESULT_R {
        INJ_RESULT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 28 - Mirror bit of corresponding bit in SAR_INTR register"]
    #[inline(always)]
    pub fn inj_collision_intr_mir(&self) -> INJ_COLLISION_INTR_MIR_R {
        INJ_COLLISION_INTR_MIR_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Mirror bit of corresponding bit in SAR_INTR register"]
    #[inline(always)]
    pub fn inj_saturate_intr_mir(&self) -> INJ_SATURATE_INTR_MIR_R {
        INJ_SATURATE_INTR_MIR_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Mirror bit of corresponding bit in SAR_INTR register"]
    #[inline(always)]
    pub fn inj_range_intr_mir(&self) -> INJ_RANGE_INTR_MIR_R {
        INJ_RANGE_INTR_MIR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Mirror bit of corresponding bit in SAR_INTR register"]
    #[inline(always)]
    pub fn inj_eoc_intr_mir(&self) -> INJ_EOC_INTR_MIR_R {
        INJ_EOC_INTR_MIR_R::new(((self.bits >> 31) & 1) != 0)
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
#[doc = "Injection channel result register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inj_result](index.html) module"]
pub struct INJ_RESULT_SPEC;
impl crate::RegisterSpec for INJ_RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inj_result::R](R) reader structure"]
impl crate::Readable for INJ_RESULT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inj_result::W](W) writer structure"]
impl crate::Writable for INJ_RESULT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INJ_RESULT to value 0"]
impl crate::Resettable for INJ_RESULT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
