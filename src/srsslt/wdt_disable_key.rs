#[doc = "Register `WDT_DISABLE_KEY` reader"]
pub struct R(crate::R<WDT_DISABLE_KEY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDT_DISABLE_KEY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDT_DISABLE_KEY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDT_DISABLE_KEY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDT_DISABLE_KEY` writer"]
pub struct W(crate::W<WDT_DISABLE_KEY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDT_DISABLE_KEY_SPEC>;
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
impl From<crate::W<WDT_DISABLE_KEY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDT_DISABLE_KEY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY` reader - Disables WDT reset when equal to 0xACED8865. The WDT reset functions normally for any other setting."]
pub type KEY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `KEY` writer - Disables WDT reset when equal to 0xACED8865. The WDT reset functions normally for any other setting."]
pub type KEY_W<'a> = crate::FieldWriter<'a, u32, WDT_DISABLE_KEY_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Disables WDT reset when equal to 0xACED8865. The WDT reset functions normally for any other setting."]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Disables WDT reset when equal to 0xACED8865. The WDT reset functions normally for any other setting."]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Disable Key Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdt_disable_key](index.html) module"]
pub struct WDT_DISABLE_KEY_SPEC;
impl crate::RegisterSpec for WDT_DISABLE_KEY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdt_disable_key::R](R) reader structure"]
impl crate::Readable for WDT_DISABLE_KEY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdt_disable_key::W](W) writer structure"]
impl crate::Writable for WDT_DISABLE_KEY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDT_DISABLE_KEY to value 0"]
impl crate::Resettable for WDT_DISABLE_KEY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
