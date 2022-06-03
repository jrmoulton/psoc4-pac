#[doc = "Register `PWR_KEY_DELAY` reader"]
pub struct R(crate::R<PWR_KEY_DELAY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_KEY_DELAY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_KEY_DELAY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_KEY_DELAY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_KEY_DELAY` writer"]
pub struct W(crate::W<PWR_KEY_DELAY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_KEY_DELAY_SPEC>;
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
impl From<crate::W<PWR_KEY_DELAY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_KEY_DELAY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAKEUP_HOLDOFF` reader - Delay to wait for references to settle on wakeup from deepsleep. BOD is ignored and system does not resume until this delay expires. Note that the same delay on POR is hard-coded. The default assumes the output of the predivider is 48MHz + 3 percent. Firmware may scale this setting according to the fastest actual clock frequency that can occur when waking from DEEPSLEEP."]
pub type WAKEUP_HOLDOFF_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WAKEUP_HOLDOFF` writer - Delay to wait for references to settle on wakeup from deepsleep. BOD is ignored and system does not resume until this delay expires. Note that the same delay on POR is hard-coded. The default assumes the output of the predivider is 48MHz + 3 percent. Firmware may scale this setting according to the fastest actual clock frequency that can occur when waking from DEEPSLEEP."]
pub type WAKEUP_HOLDOFF_W<'a> = crate::FieldWriter<'a, u32, PWR_KEY_DELAY_SPEC, u16, u16, 10, 0>;
impl R {
    #[doc = "Bits 0:9 - Delay to wait for references to settle on wakeup from deepsleep. BOD is ignored and system does not resume until this delay expires. Note that the same delay on POR is hard-coded. The default assumes the output of the predivider is 48MHz + 3 percent. Firmware may scale this setting according to the fastest actual clock frequency that can occur when waking from DEEPSLEEP."]
    #[inline(always)]
    pub fn wakeup_holdoff(&self) -> WAKEUP_HOLDOFF_R {
        WAKEUP_HOLDOFF_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Delay to wait for references to settle on wakeup from deepsleep. BOD is ignored and system does not resume until this delay expires. Note that the same delay on POR is hard-coded. The default assumes the output of the predivider is 48MHz + 3 percent. Firmware may scale this setting according to the fastest actual clock frequency that can occur when waking from DEEPSLEEP."]
    #[inline(always)]
    pub fn wakeup_holdoff(&mut self) -> WAKEUP_HOLDOFF_W {
        WAKEUP_HOLDOFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power System Key&Delay Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_key_delay](index.html) module"]
pub struct PWR_KEY_DELAY_SPEC;
impl crate::RegisterSpec for PWR_KEY_DELAY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_key_delay::R](R) reader structure"]
impl crate::Readable for PWR_KEY_DELAY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_key_delay::W](W) writer structure"]
impl crate::Writable for PWR_KEY_DELAY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_KEY_DELAY to value 0xf8"]
impl crate::Resettable for PWR_KEY_DELAY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf8
    }
}
