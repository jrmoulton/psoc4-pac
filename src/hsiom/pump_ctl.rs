#[doc = "Register `PUMP_CTL` reader"]
pub struct R(crate::R<PUMP_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUMP_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUMP_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUMP_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PUMP_CTL` writer"]
pub struct W(crate::W<PUMP_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUMP_CTL_SPEC>;
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
impl From<crate::W<PUMP_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUMP_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLOCK_SEL` reader - Clock select: '0': External clock. '1': Internal clock (deprecated)."]
pub type CLOCK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `CLOCK_SEL` writer - Clock select: '0': External clock. '1': Internal clock (deprecated)."]
pub type CLOCK_SEL_W<'a> = crate::BitWriter<'a, u32, PUMP_CTL_SPEC, bool, 0>;
#[doc = "Field `ENABLED` reader - Pump enabled: '0': Disabled. '1': Enabled."]
pub type ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ENABLED` writer - Pump enabled: '0': Disabled. '1': Enabled."]
pub type ENABLED_W<'a> = crate::BitWriter<'a, u32, PUMP_CTL_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - Clock select: '0': External clock. '1': Internal clock (deprecated)."]
    #[inline(always)]
    pub fn clock_sel(&self) -> CLOCK_SEL_R {
        CLOCK_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 31 - Pump enabled: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock select: '0': External clock. '1': Internal clock (deprecated)."]
    #[inline(always)]
    pub fn clock_sel(&mut self) -> CLOCK_SEL_W {
        CLOCK_SEL_W::new(self)
    }
    #[doc = "Bit 31 - Pump enabled: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W {
        ENABLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pump control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pump_ctl](index.html) module"]
pub struct PUMP_CTL_SPEC;
impl crate::RegisterSpec for PUMP_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pump_ctl::R](R) reader structure"]
impl crate::Readable for PUMP_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pump_ctl::W](W) writer structure"]
impl crate::Writable for PUMP_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PUMP_CTL to value 0"]
impl crate::Resettable for PUMP_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
