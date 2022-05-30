#[doc = "Register `DMAC_STATUS_CH_ACT` reader"]
pub struct R(crate::R<DMAC_STATUS_CH_ACT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_STATUS_CH_ACT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_STATUS_CH_ACT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_STATUS_CH_ACT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAC_STATUS_CH_ACT` writer"]
pub struct W(crate::W<DMAC_STATUS_CH_ACT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAC_STATUS_CH_ACT_SPEC>;
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
impl From<crate::W<DMAC_STATUS_CH_ACT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAC_STATUS_CH_ACT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH` reader - Channel activation status. Bit i is associated to channel i. Software reads this field to get information on all actively pending channels (either in pending or in the data transfer engine)."]
pub type CH_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel activation status. Bit i is associated to channel i. Software reads this field to get information on all actively pending channels (either in pending or in the data transfer engine)."]
    #[inline(always)]
    pub fn ch(&self) -> CH_R {
        CH_R::new(self.bits)
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
#[doc = "Channel activation status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_status_ch_act](index.html) module"]
pub struct DMAC_STATUS_CH_ACT_SPEC;
impl crate::RegisterSpec for DMAC_STATUS_CH_ACT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_status_ch_act::R](R) reader structure"]
impl crate::Readable for DMAC_STATUS_CH_ACT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmac_status_ch_act::W](W) writer structure"]
impl crate::Writable for DMAC_STATUS_CH_ACT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAC_STATUS_CH_ACT to value 0"]
impl crate::Resettable for DMAC_STATUS_CH_ACT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
