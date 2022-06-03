#[doc = "Register `WDT_COUNTER` reader"]
pub struct R(crate::R<WDT_COUNTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDT_COUNTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDT_COUNTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDT_COUNTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNTER` reader - Current value of WDT Counter"]
pub type COUNTER_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Current value of WDT Counter"]
    #[inline(always)]
    pub fn counter(&self) -> COUNTER_R {
        COUNTER_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Watchdog Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdt_counter](index.html) module"]
pub struct WDT_COUNTER_SPEC;
impl crate::RegisterSpec for WDT_COUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdt_counter::R](R) reader structure"]
impl crate::Readable for WDT_COUNTER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WDT_COUNTER to value 0"]
impl crate::Resettable for WDT_COUNTER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
