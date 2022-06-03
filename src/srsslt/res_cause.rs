#[doc = "Register `RES_CAUSE` reader"]
pub struct R(crate::R<RES_CAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RES_CAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RES_CAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RES_CAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RES_CAUSE` writer"]
pub struct W(crate::W<RES_CAUSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RES_CAUSE_SPEC>;
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
impl From<crate::W<RES_CAUSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RES_CAUSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESET_WDT` reader - A WatchDog Timer reset has occurred since last power cycle."]
pub type RESET_WDT_R = crate::BitReader<bool>;
#[doc = "Field `RESET_WDT` writer - A WatchDog Timer reset has occurred since last power cycle."]
pub type RESET_WDT_W<'a> = crate::BitWriter<'a, u32, RES_CAUSE_SPEC, bool, 0>;
#[doc = "Field `RESET_PROT_FAULT` reader - A protection violation occurred that requires a RESET. This includes, but is not limited to, hitting a debug breakpoint while in Privileged Mode."]
pub type RESET_PROT_FAULT_R = crate::BitReader<bool>;
#[doc = "Field `RESET_PROT_FAULT` writer - A protection violation occurred that requires a RESET. This includes, but is not limited to, hitting a debug breakpoint while in Privileged Mode."]
pub type RESET_PROT_FAULT_W<'a> = crate::BitWriter<'a, u32, RES_CAUSE_SPEC, bool, 3>;
#[doc = "Field `RESET_SOFT` reader - Cortex-M0 requested a system reset through it's SYSRESETREQ. This can be done via a debugger probe or in firmware."]
pub type RESET_SOFT_R = crate::BitReader<bool>;
#[doc = "Field `RESET_SOFT` writer - Cortex-M0 requested a system reset through it's SYSRESETREQ. This can be done via a debugger probe or in firmware."]
pub type RESET_SOFT_W<'a> = crate::BitWriter<'a, u32, RES_CAUSE_SPEC, bool, 4>;
impl R {
    #[doc = "Bit 0 - A WatchDog Timer reset has occurred since last power cycle."]
    #[inline(always)]
    pub fn reset_wdt(&self) -> RESET_WDT_R {
        RESET_WDT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - A protection violation occurred that requires a RESET. This includes, but is not limited to, hitting a debug breakpoint while in Privileged Mode."]
    #[inline(always)]
    pub fn reset_prot_fault(&self) -> RESET_PROT_FAULT_R {
        RESET_PROT_FAULT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Cortex-M0 requested a system reset through it's SYSRESETREQ. This can be done via a debugger probe or in firmware."]
    #[inline(always)]
    pub fn reset_soft(&self) -> RESET_SOFT_R {
        RESET_SOFT_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A WatchDog Timer reset has occurred since last power cycle."]
    #[inline(always)]
    pub fn reset_wdt(&mut self) -> RESET_WDT_W {
        RESET_WDT_W::new(self)
    }
    #[doc = "Bit 3 - A protection violation occurred that requires a RESET. This includes, but is not limited to, hitting a debug breakpoint while in Privileged Mode."]
    #[inline(always)]
    pub fn reset_prot_fault(&mut self) -> RESET_PROT_FAULT_W {
        RESET_PROT_FAULT_W::new(self)
    }
    #[doc = "Bit 4 - Cortex-M0 requested a system reset through it's SYSRESETREQ. This can be done via a debugger probe or in firmware."]
    #[inline(always)]
    pub fn reset_soft(&mut self) -> RESET_SOFT_W {
        RESET_SOFT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Cause Observation Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [res_cause](index.html) module"]
pub struct RES_CAUSE_SPEC;
impl crate::RegisterSpec for RES_CAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [res_cause::R](R) reader structure"]
impl crate::Readable for RES_CAUSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [res_cause::W](W) writer structure"]
impl crate::Writable for RES_CAUSE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RES_CAUSE to value 0"]
impl crate::Resettable for RES_CAUSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
