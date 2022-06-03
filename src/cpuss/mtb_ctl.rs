#[doc = "Register `MTB_CTL` reader"]
pub struct R(crate::R<MTB_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTB_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTB_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTB_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTB_CTL` writer"]
pub struct W(crate::W<MTB_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTB_CTL_SPEC>;
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
impl From<crate::W<MTB_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTB_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPU_HALT_TSTOP_EN` reader - 1': Enable CPU Halt to stop MTB trace. ('HALTED' output of CM0+ can stop the trace when high/'1') '0': 'HALTED' output of CM0+ can not strop trace."]
pub type CPU_HALT_TSTOP_EN_R = crate::BitReader<bool>;
#[doc = "Field `CPU_HALT_TSTOP_EN` writer - 1': Enable CPU Halt to stop MTB trace. ('HALTED' output of CM0+ can stop the trace when high/'1') '0': 'HALTED' output of CM0+ can not strop trace."]
pub type CPU_HALT_TSTOP_EN_W<'a> = crate::BitWriter<'a, u32, MTB_CTL_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - 1': Enable CPU Halt to stop MTB trace. ('HALTED' output of CM0+ can stop the trace when high/'1') '0': 'HALTED' output of CM0+ can not strop trace."]
    #[inline(always)]
    pub fn cpu_halt_tstop_en(&self) -> CPU_HALT_TSTOP_EN_R {
        CPU_HALT_TSTOP_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1': Enable CPU Halt to stop MTB trace. ('HALTED' output of CM0+ can stop the trace when high/'1') '0': 'HALTED' output of CM0+ can not strop trace."]
    #[inline(always)]
    pub fn cpu_halt_tstop_en(&mut self) -> CPU_HALT_TSTOP_EN_W {
        CPU_HALT_TSTOP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MTB control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtb_ctl](index.html) module"]
pub struct MTB_CTL_SPEC;
impl crate::RegisterSpec for MTB_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtb_ctl::R](R) reader structure"]
impl crate::Readable for MTB_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtb_ctl::W](W) writer structure"]
impl crate::Writable for MTB_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTB_CTL to value 0"]
impl crate::Resettable for MTB_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
