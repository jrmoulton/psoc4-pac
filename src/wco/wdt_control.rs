#[doc = "Register `WDT_CONTROL` reader"]
pub struct R(crate::R<WDT_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDT_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDT_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDT_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDT_CONTROL` writer"]
pub struct W(crate::W<WDT_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDT_CONTROL_SPEC>;
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
impl From<crate::W<WDT_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDT_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT_ENABLE0` reader - Enable Counter 0 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up) Note: This field takes considerable time (up to 3 LFCLK cycles) to take effect. It must not be changed more than once in that period."]
pub type WDT_ENABLE0_R = crate::BitReader<bool>;
#[doc = "Field `WDT_ENABLE0` writer - Enable Counter 0 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up) Note: This field takes considerable time (up to 3 LFCLK cycles) to take effect. It must not be changed more than once in that period."]
pub type WDT_ENABLE0_W<'a> = crate::BitWriter<'a, u32, WDT_CONTROL_SPEC, bool, 0>;
#[doc = "Field `WDT_ENABLED0` reader - Indicates actual state of counter. May lag WDT_ENABLE0 by up to 3 LFCLK cycles. After changing WDT_ENABLE0, do not enter DEEPSLEEP mode until this field acknowledges the change."]
pub type WDT_ENABLED0_R = crate::BitReader<bool>;
#[doc = "Field `WDT_INT0` reader - WDT Interrupt Request. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODEx=3. After W1C, WDT_CONTROL must be read for the hardware to internally remove the clear flag. Failure to do this may result in missing the next interrupt."]
pub type WDT_INT0_R = crate::BitReader<bool>;
#[doc = "Field `WDT_INT0` writer - WDT Interrupt Request. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODEx=3. After W1C, WDT_CONTROL must be read for the hardware to internally remove the clear flag. Failure to do this may result in missing the next interrupt."]
pub type WDT_INT0_W<'a> = crate::BitWriter<'a, u32, WDT_CONTROL_SPEC, bool, 2>;
#[doc = "Field `WDT_RESET0` reader - Resets counter 0 back to 0000. Hardware will reset this bit after counter was reset. This will take several LFCLK cycles to take effect. Wait until the reset completes before enabling the WDT."]
pub type WDT_RESET0_R = crate::BitReader<bool>;
#[doc = "Field `WDT_RESET0` writer - Resets counter 0 back to 0000. Hardware will reset this bit after counter was reset. This will take several LFCLK cycles to take effect. Wait until the reset completes before enabling the WDT."]
pub type WDT_RESET0_W<'a> = crate::BitWriter<'a, u32, WDT_CONTROL_SPEC, bool, 3>;
#[doc = "Field `WDT_ENABLE1` reader - Enable Counter 1 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up) Note: This field takes considerable time (up to 3 LFCLK cycles) to take effect. It must not be changed more than once in that period."]
pub type WDT_ENABLE1_R = crate::BitReader<bool>;
#[doc = "Field `WDT_ENABLE1` writer - Enable Counter 1 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up) Note: This field takes considerable time (up to 3 LFCLK cycles) to take effect. It must not be changed more than once in that period."]
pub type WDT_ENABLE1_W<'a> = crate::BitWriter<'a, u32, WDT_CONTROL_SPEC, bool, 8>;
#[doc = "Field `WDT_ENABLED1` reader - Indicates actual state of counter. May lag WDT_ENABLE1 by up to 3 LFCLK cycles. After changing WDT_ENABLE1, do not enter DEEPSLEEP mode until this field acknowledges the change."]
pub type WDT_ENABLED1_R = crate::BitReader<bool>;
#[doc = "Field `WDT_INT1` reader - WDT Interrupt Request. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. After W1C, WDT_CONTROL must be read for the hardware to internally remove the clear flag. Failure to do this may result in missing the next interrupt."]
pub type WDT_INT1_R = crate::BitReader<bool>;
#[doc = "Field `WDT_INT1` writer - WDT Interrupt Request. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. After W1C, WDT_CONTROL must be read for the hardware to internally remove the clear flag. Failure to do this may result in missing the next interrupt."]
pub type WDT_INT1_W<'a> = crate::BitWriter<'a, u32, WDT_CONTROL_SPEC, bool, 10>;
#[doc = "Field `WDT_RESET1` reader - Resets counter 1 back to 0000. Hardware will reset this bit after counter was reset. This will take several LFCLK cycles to take effect. Wait until the reset completes before enabling the WDT."]
pub type WDT_RESET1_R = crate::BitReader<bool>;
#[doc = "Field `WDT_RESET1` writer - Resets counter 1 back to 0000. Hardware will reset this bit after counter was reset. This will take several LFCLK cycles to take effect. Wait until the reset completes before enabling the WDT."]
pub type WDT_RESET1_W<'a> = crate::BitWriter<'a, u32, WDT_CONTROL_SPEC, bool, 11>;
#[doc = "Field `WDT_ENABLE2` reader - Enable Counter 2 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up) Note: This field takes considerable time (up to 3 LFCLK cycles) to take effect. It must not be changed more than once in that period."]
pub type WDT_ENABLE2_R = crate::BitReader<bool>;
#[doc = "Field `WDT_ENABLE2` writer - Enable Counter 2 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up) Note: This field takes considerable time (up to 3 LFCLK cycles) to take effect. It must not be changed more than once in that period."]
pub type WDT_ENABLE2_W<'a> = crate::BitWriter<'a, u32, WDT_CONTROL_SPEC, bool, 16>;
#[doc = "Field `WDT_ENABLED2` reader - Indicates actual state of counter. May lag WDT_ENABLE2 by up to 3 LFCLK cycles. After changing WDT_ENABLE2, do not enter DEEPSLEEP mode until this field acknowledges the change."]
pub type WDT_ENABLED2_R = crate::BitReader<bool>;
#[doc = "Field `WDT_INT2` reader - WDT Interrupt Request. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. After W1C, WDT_CONTROL must be read for the hardware to internally remove the clear flag. Failure to do this may result in missing the next interrupt."]
pub type WDT_INT2_R = crate::BitReader<bool>;
#[doc = "Field `WDT_INT2` writer - WDT Interrupt Request. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. After W1C, WDT_CONTROL must be read for the hardware to internally remove the clear flag. Failure to do this may result in missing the next interrupt."]
pub type WDT_INT2_W<'a> = crate::BitWriter<'a, u32, WDT_CONTROL_SPEC, bool, 18>;
#[doc = "Field `WDT_RESET2` reader - Resets counter 2 back to 0000_0000. Hardware will reset this bit after counter was reset. This will take several LFCLK cycles to take effect. Wait until the reset completes before enabling the WDT."]
pub type WDT_RESET2_R = crate::BitReader<bool>;
#[doc = "Field `WDT_RESET2` writer - Resets counter 2 back to 0000_0000. Hardware will reset this bit after counter was reset. This will take several LFCLK cycles to take effect. Wait until the reset completes before enabling the WDT."]
pub type WDT_RESET2_W<'a> = crate::BitWriter<'a, u32, WDT_CONTROL_SPEC, bool, 19>;
impl R {
    #[doc = "Bit 0 - Enable Counter 0 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up) Note: This field takes considerable time (up to 3 LFCLK cycles) to take effect. It must not be changed more than once in that period."]
    #[inline(always)]
    pub fn wdt_enable0(&self) -> WDT_ENABLE0_R {
        WDT_ENABLE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates actual state of counter. May lag WDT_ENABLE0 by up to 3 LFCLK cycles. After changing WDT_ENABLE0, do not enter DEEPSLEEP mode until this field acknowledges the change."]
    #[inline(always)]
    pub fn wdt_enabled0(&self) -> WDT_ENABLED0_R {
        WDT_ENABLED0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WDT Interrupt Request. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODEx=3. After W1C, WDT_CONTROL must be read for the hardware to internally remove the clear flag. Failure to do this may result in missing the next interrupt."]
    #[inline(always)]
    pub fn wdt_int0(&self) -> WDT_INT0_R {
        WDT_INT0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Resets counter 0 back to 0000. Hardware will reset this bit after counter was reset. This will take several LFCLK cycles to take effect. Wait until the reset completes before enabling the WDT."]
    #[inline(always)]
    pub fn wdt_reset0(&self) -> WDT_RESET0_R {
        WDT_RESET0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Counter 1 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up) Note: This field takes considerable time (up to 3 LFCLK cycles) to take effect. It must not be changed more than once in that period."]
    #[inline(always)]
    pub fn wdt_enable1(&self) -> WDT_ENABLE1_R {
        WDT_ENABLE1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Indicates actual state of counter. May lag WDT_ENABLE1 by up to 3 LFCLK cycles. After changing WDT_ENABLE1, do not enter DEEPSLEEP mode until this field acknowledges the change."]
    #[inline(always)]
    pub fn wdt_enabled1(&self) -> WDT_ENABLED1_R {
        WDT_ENABLED1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - WDT Interrupt Request. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. After W1C, WDT_CONTROL must be read for the hardware to internally remove the clear flag. Failure to do this may result in missing the next interrupt."]
    #[inline(always)]
    pub fn wdt_int1(&self) -> WDT_INT1_R {
        WDT_INT1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Resets counter 1 back to 0000. Hardware will reset this bit after counter was reset. This will take several LFCLK cycles to take effect. Wait until the reset completes before enabling the WDT."]
    #[inline(always)]
    pub fn wdt_reset1(&self) -> WDT_RESET1_R {
        WDT_RESET1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Counter 2 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up) Note: This field takes considerable time (up to 3 LFCLK cycles) to take effect. It must not be changed more than once in that period."]
    #[inline(always)]
    pub fn wdt_enable2(&self) -> WDT_ENABLE2_R {
        WDT_ENABLE2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Indicates actual state of counter. May lag WDT_ENABLE2 by up to 3 LFCLK cycles. After changing WDT_ENABLE2, do not enter DEEPSLEEP mode until this field acknowledges the change."]
    #[inline(always)]
    pub fn wdt_enabled2(&self) -> WDT_ENABLED2_R {
        WDT_ENABLED2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - WDT Interrupt Request. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. After W1C, WDT_CONTROL must be read for the hardware to internally remove the clear flag. Failure to do this may result in missing the next interrupt."]
    #[inline(always)]
    pub fn wdt_int2(&self) -> WDT_INT2_R {
        WDT_INT2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Resets counter 2 back to 0000_0000. Hardware will reset this bit after counter was reset. This will take several LFCLK cycles to take effect. Wait until the reset completes before enabling the WDT."]
    #[inline(always)]
    pub fn wdt_reset2(&self) -> WDT_RESET2_R {
        WDT_RESET2_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Counter 0 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up) Note: This field takes considerable time (up to 3 LFCLK cycles) to take effect. It must not be changed more than once in that period."]
    #[inline(always)]
    pub fn wdt_enable0(&mut self) -> WDT_ENABLE0_W {
        WDT_ENABLE0_W::new(self)
    }
    #[doc = "Bit 2 - WDT Interrupt Request. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODEx=3. After W1C, WDT_CONTROL must be read for the hardware to internally remove the clear flag. Failure to do this may result in missing the next interrupt."]
    #[inline(always)]
    pub fn wdt_int0(&mut self) -> WDT_INT0_W {
        WDT_INT0_W::new(self)
    }
    #[doc = "Bit 3 - Resets counter 0 back to 0000. Hardware will reset this bit after counter was reset. This will take several LFCLK cycles to take effect. Wait until the reset completes before enabling the WDT."]
    #[inline(always)]
    pub fn wdt_reset0(&mut self) -> WDT_RESET0_W {
        WDT_RESET0_W::new(self)
    }
    #[doc = "Bit 8 - Enable Counter 1 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up) Note: This field takes considerable time (up to 3 LFCLK cycles) to take effect. It must not be changed more than once in that period."]
    #[inline(always)]
    pub fn wdt_enable1(&mut self) -> WDT_ENABLE1_W {
        WDT_ENABLE1_W::new(self)
    }
    #[doc = "Bit 10 - WDT Interrupt Request. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. After W1C, WDT_CONTROL must be read for the hardware to internally remove the clear flag. Failure to do this may result in missing the next interrupt."]
    #[inline(always)]
    pub fn wdt_int1(&mut self) -> WDT_INT1_W {
        WDT_INT1_W::new(self)
    }
    #[doc = "Bit 11 - Resets counter 1 back to 0000. Hardware will reset this bit after counter was reset. This will take several LFCLK cycles to take effect. Wait until the reset completes before enabling the WDT."]
    #[inline(always)]
    pub fn wdt_reset1(&mut self) -> WDT_RESET1_W {
        WDT_RESET1_W::new(self)
    }
    #[doc = "Bit 16 - Enable Counter 2 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up) Note: This field takes considerable time (up to 3 LFCLK cycles) to take effect. It must not be changed more than once in that period."]
    #[inline(always)]
    pub fn wdt_enable2(&mut self) -> WDT_ENABLE2_W {
        WDT_ENABLE2_W::new(self)
    }
    #[doc = "Bit 18 - WDT Interrupt Request. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. After W1C, WDT_CONTROL must be read for the hardware to internally remove the clear flag. Failure to do this may result in missing the next interrupt."]
    #[inline(always)]
    pub fn wdt_int2(&mut self) -> WDT_INT2_W {
        WDT_INT2_W::new(self)
    }
    #[doc = "Bit 19 - Resets counter 2 back to 0000_0000. Hardware will reset this bit after counter was reset. This will take several LFCLK cycles to take effect. Wait until the reset completes before enabling the WDT."]
    #[inline(always)]
    pub fn wdt_reset2(&mut self) -> WDT_RESET2_W {
        WDT_RESET2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Counters Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdt_control](index.html) module"]
pub struct WDT_CONTROL_SPEC;
impl crate::RegisterSpec for WDT_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdt_control::R](R) reader structure"]
impl crate::Readable for WDT_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdt_control::W](W) writer structure"]
impl crate::Writable for WDT_CONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDT_CONTROL to value 0"]
impl crate::Resettable for WDT_CONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
