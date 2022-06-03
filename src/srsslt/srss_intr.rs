#[doc = "Register `SRSS_INTR` reader"]
pub struct R(crate::R<SRSS_INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRSS_INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRSS_INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRSS_INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRSS_INTR` writer"]
pub struct W(crate::W<SRSS_INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRSS_INTR_SPEC>;
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
impl From<crate::W<SRSS_INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRSS_INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT_MATCH` reader - WDT Interrupt Request. This bit is set each time WDT_COUNTR==WDT_MATCH. Clearing this bit also feeds the watch dog. Missing 2 interrupts in a row will generate brown-out reset. Due to internal synchronization, it takes 2 SYSCLK cycles to update after a W1C."]
pub type WDT_MATCH_R = crate::BitReader<bool>;
#[doc = "Field `WDT_MATCH` writer - WDT Interrupt Request. This bit is set each time WDT_COUNTR==WDT_MATCH. Clearing this bit also feeds the watch dog. Missing 2 interrupts in a row will generate brown-out reset. Due to internal synchronization, it takes 2 SYSCLK cycles to update after a W1C."]
pub type WDT_MATCH_W<'a> = crate::BitWriter<'a, u32, SRSS_INTR_SPEC, bool, 0>;
#[doc = "Field `TEMP_HIGH` reader - Regulator over-temp interrupt. This interrupt can occur when a short circuit exists on the vccd pin or when extreme loads are applied on IO-cells causing the die to overheat. Firmware is encourage to shutdown all IO cells and then go to DeepSleep mode when this interrupt occurs if protection against such conditions is desired."]
pub type TEMP_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `TEMP_HIGH` writer - Regulator over-temp interrupt. This interrupt can occur when a short circuit exists on the vccd pin or when extreme loads are applied on IO-cells causing the die to overheat. Firmware is encourage to shutdown all IO cells and then go to DeepSleep mode when this interrupt occurs if protection against such conditions is desired."]
pub type TEMP_HIGH_W<'a> = crate::BitWriter<'a, u32, SRSS_INTR_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - WDT Interrupt Request. This bit is set each time WDT_COUNTR==WDT_MATCH. Clearing this bit also feeds the watch dog. Missing 2 interrupts in a row will generate brown-out reset. Due to internal synchronization, it takes 2 SYSCLK cycles to update after a W1C."]
    #[inline(always)]
    pub fn wdt_match(&self) -> WDT_MATCH_R {
        WDT_MATCH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Regulator over-temp interrupt. This interrupt can occur when a short circuit exists on the vccd pin or when extreme loads are applied on IO-cells causing the die to overheat. Firmware is encourage to shutdown all IO cells and then go to DeepSleep mode when this interrupt occurs if protection against such conditions is desired."]
    #[inline(always)]
    pub fn temp_high(&self) -> TEMP_HIGH_R {
        TEMP_HIGH_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WDT Interrupt Request. This bit is set each time WDT_COUNTR==WDT_MATCH. Clearing this bit also feeds the watch dog. Missing 2 interrupts in a row will generate brown-out reset. Due to internal synchronization, it takes 2 SYSCLK cycles to update after a W1C."]
    #[inline(always)]
    pub fn wdt_match(&mut self) -> WDT_MATCH_W {
        WDT_MATCH_W::new(self)
    }
    #[doc = "Bit 1 - Regulator over-temp interrupt. This interrupt can occur when a short circuit exists on the vccd pin or when extreme loads are applied on IO-cells causing the die to overheat. Firmware is encourage to shutdown all IO cells and then go to DeepSleep mode when this interrupt occurs if protection against such conditions is desired."]
    #[inline(always)]
    pub fn temp_high(&mut self) -> TEMP_HIGH_W {
        TEMP_HIGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRSS Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srss_intr](index.html) module"]
pub struct SRSS_INTR_SPEC;
impl crate::RegisterSpec for SRSS_INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srss_intr::R](R) reader structure"]
impl crate::Readable for SRSS_INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srss_intr::W](W) writer structure"]
impl crate::Writable for SRSS_INTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRSS_INTR to value 0"]
impl crate::Resettable for SRSS_INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
