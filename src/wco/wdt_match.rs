#[doc = "Register `WDT_MATCH` reader"]
pub struct R(crate::R<WDT_MATCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDT_MATCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDT_MATCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDT_MATCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDT_MATCH` writer"]
pub struct W(crate::W<WDT_MATCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDT_MATCH_SPEC>;
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
impl From<crate::W<WDT_MATCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDT_MATCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT_MATCH0` reader - Match value for Watchdog Counter 0"]
pub type WDT_MATCH0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WDT_MATCH0` writer - Match value for Watchdog Counter 0"]
pub type WDT_MATCH0_W<'a> = crate::FieldWriter<'a, u32, WDT_MATCH_SPEC, u16, u16, 16, 0>;
#[doc = "Field `WDT_MATCH1` reader - Match value for Watchdog Counter 1"]
pub type WDT_MATCH1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WDT_MATCH1` writer - Match value for Watchdog Counter 1"]
pub type WDT_MATCH1_W<'a> = crate::FieldWriter<'a, u32, WDT_MATCH_SPEC, u16, u16, 16, 16>;
impl R {
    #[doc = "Bits 0:15 - Match value for Watchdog Counter 0"]
    #[inline(always)]
    pub fn wdt_match0(&self) -> WDT_MATCH0_R {
        WDT_MATCH0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Match value for Watchdog Counter 1"]
    #[inline(always)]
    pub fn wdt_match1(&self) -> WDT_MATCH1_R {
        WDT_MATCH1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Match value for Watchdog Counter 0"]
    #[inline(always)]
    pub fn wdt_match0(&mut self) -> WDT_MATCH0_W {
        WDT_MATCH0_W::new(self)
    }
    #[doc = "Bits 16:31 - Match value for Watchdog Counter 1"]
    #[inline(always)]
    pub fn wdt_match1(&mut self) -> WDT_MATCH1_W {
        WDT_MATCH1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog counter match values\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdt_match](index.html) module"]
pub struct WDT_MATCH_SPEC;
impl crate::RegisterSpec for WDT_MATCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdt_match::R](R) reader structure"]
impl crate::Readable for WDT_MATCH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdt_match::W](W) writer structure"]
impl crate::Writable for WDT_MATCH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDT_MATCH to value 0"]
impl crate::Resettable for WDT_MATCH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
