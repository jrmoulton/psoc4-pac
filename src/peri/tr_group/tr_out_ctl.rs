#[doc = "Register `TR_OUT_CTL[%s]` reader"]
pub struct R(crate::R<TR_OUT_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR_OUT_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR_OUT_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR_OUT_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TR_OUT_CTL[%s]` writer"]
pub struct W(crate::W<TR_OUT_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TR_OUT_CTL_SPEC>;
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
impl From<crate::W<TR_OUT_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TR_OUT_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - Specifies input trigger. This field is typically set during the setup of a chip use case scenario. Changing this field while activated triggers are present on the input triggers may result in unpredictable behavior. Note that input trigger 0 (default value) is typically connected to a constant signal level of '0', and as a result will not cause HW activation of the output trigger."]
pub type SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL` writer - Specifies input trigger. This field is typically set during the setup of a chip use case scenario. Changing this field while activated triggers are present on the input triggers may result in unpredictable behavior. Note that input trigger 0 (default value) is typically connected to a constant signal level of '0', and as a result will not cause HW activation of the output trigger."]
pub type SEL_W<'a> = crate::FieldWriter<'a, u32, TR_OUT_CTL_SPEC, u8, u8, 7, 0>;
impl R {
    #[doc = "Bits 0:6 - Specifies input trigger. This field is typically set during the setup of a chip use case scenario. Changing this field while activated triggers are present on the input triggers may result in unpredictable behavior. Note that input trigger 0 (default value) is typically connected to a constant signal level of '0', and as a result will not cause HW activation of the output trigger."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Specifies input trigger. This field is typically set during the setup of a chip use case scenario. Changing this field while activated triggers are present on the input triggers may result in unpredictable behavior. Note that input trigger 0 (default value) is typically connected to a constant signal level of '0', and as a result will not cause HW activation of the output trigger."]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trigger control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_out_ctl](index.html) module"]
pub struct TR_OUT_CTL_SPEC;
impl crate::RegisterSpec for TR_OUT_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tr_out_ctl::R](R) reader structure"]
impl crate::Readable for TR_OUT_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tr_out_ctl::W](W) writer structure"]
impl crate::Writable for TR_OUT_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TR_OUT_CTL[%s]
to value 0"]
impl crate::Resettable for TR_OUT_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
