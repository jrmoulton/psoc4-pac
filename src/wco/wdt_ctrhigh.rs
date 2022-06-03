#[doc = "Register `WDT_CTRHIGH` reader"]
pub struct R(crate::R<WDT_CTRHIGH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDT_CTRHIGH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDT_CTRHIGH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDT_CTRHIGH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WDT_CTR2` reader - Current value of WDT Counter 2"]
pub type WDT_CTR2_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Current value of WDT Counter 2"]
    #[inline(always)]
    pub fn wdt_ctr2(&self) -> WDT_CTR2_R {
        WDT_CTR2_R::new(self.bits)
    }
}
#[doc = "Watchdog Counter 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdt_ctrhigh](index.html) module"]
pub struct WDT_CTRHIGH_SPEC;
impl crate::RegisterSpec for WDT_CTRHIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdt_ctrhigh::R](R) reader structure"]
impl crate::Readable for WDT_CTRHIGH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WDT_CTRHIGH to value 0"]
impl crate::Resettable for WDT_CTRHIGH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
