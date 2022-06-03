#[doc = "Register `WDT_CTRLOW` reader"]
pub struct R(crate::R<WDT_CTRLOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDT_CTRLOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDT_CTRLOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDT_CTRLOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WDT_CTR0` reader - Current value of WDT Counter 0"]
pub type WDT_CTR0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WDT_CTR1` reader - Current value of WDT Counter 1"]
pub type WDT_CTR1_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Current value of WDT Counter 0"]
    #[inline(always)]
    pub fn wdt_ctr0(&self) -> WDT_CTR0_R {
        WDT_CTR0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Current value of WDT Counter 1"]
    #[inline(always)]
    pub fn wdt_ctr1(&self) -> WDT_CTR1_R {
        WDT_CTR1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Watchdog Counters 0/1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdt_ctrlow](index.html) module"]
pub struct WDT_CTRLOW_SPEC;
impl crate::RegisterSpec for WDT_CTRLOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdt_ctrlow::R](R) reader structure"]
impl crate::Readable for WDT_CTRLOW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WDT_CTRLOW to value 0"]
impl crate::Resettable for WDT_CTRLOW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
