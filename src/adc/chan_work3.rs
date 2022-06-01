#[doc = "Register `CHAN_WORK3` reader"]
pub struct R(crate::R<CHAN_WORK3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHAN_WORK3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHAN_WORK3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHAN_WORK3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHAN_WORK3` writer"]
pub struct W(crate::W<CHAN_WORK3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHAN_WORK3_SPEC>;
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
impl From<crate::W<CHAN_WORK3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHAN_WORK3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WORK` reader - SAR conversion working data of the channel"]
pub type WORK_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CHAN_WORK_VALID_MIR` reader - Mirror bit of corresponding bit in SAR_CHAN_WORK_VALID register"]
pub type CHAN_WORK_VALID_MIR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:15 - SAR conversion working data of the channel"]
    #[inline(always)]
    pub fn work(&self) -> WORK_R {
        WORK_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Mirror bit of corresponding bit in SAR_CHAN_WORK_VALID register"]
    #[inline(always)]
    pub fn chan_work_valid_mir(&self) -> CHAN_WORK_VALID_MIR_R {
        CHAN_WORK_VALID_MIR_R::new(((self.bits >> 31) & 1) != 0)
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
#[doc = "Channel3 working data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chan_work3](index.html) module"]
pub struct CHAN_WORK3_SPEC;
impl crate::RegisterSpec for CHAN_WORK3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chan_work3::R](R) reader structure"]
impl crate::Readable for CHAN_WORK3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chan_work3::W](W) writer structure"]
impl crate::Writable for CHAN_WORK3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHAN_WORK3 to value 0"]
impl crate::Resettable for CHAN_WORK3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
