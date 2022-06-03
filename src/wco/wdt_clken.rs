#[doc = "Register `WDT_CLKEN` reader"]
pub struct R(crate::R<WDT_CLKEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDT_CLKEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDT_CLKEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDT_CLKEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDT_CLKEN` writer"]
pub struct W(crate::W<WDT_CLKEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDT_CLKEN_SPEC>;
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
impl From<crate::W<WDT_CLKEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDT_CLKEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_WCO_EN_FOR_WDT` reader - Enables the WCO clock for use by the WDT logic. Wait at least 4 WCO clock cycles for a change to take effect. Must be 0 when switching WDT_CONFIG.LFCLK_SEL. Should be 0 if CLK_ILO_EN_FOR_WDT=1"]
pub type CLK_WCO_EN_FOR_WDT_R = crate::BitReader<bool>;
#[doc = "Field `CLK_WCO_EN_FOR_WDT` writer - Enables the WCO clock for use by the WDT logic. Wait at least 4 WCO clock cycles for a change to take effect. Must be 0 when switching WDT_CONFIG.LFCLK_SEL. Should be 0 if CLK_ILO_EN_FOR_WDT=1"]
pub type CLK_WCO_EN_FOR_WDT_W<'a> = crate::BitWriter<'a, u32, WDT_CLKEN_SPEC, bool, 0>;
#[doc = "Field `CLK_ILO_EN_FOR_WDT` reader - Enables the ILO clock for use by the WDT logic. Wait at least 4 ILO clock cycles for a change to take effect. Must be 0 when switching WDT_CONFIG.LFCLK_SEL. Should be 0 if CLK_WCO_EN_FOR_WDT=1."]
pub type CLK_ILO_EN_FOR_WDT_R = crate::BitReader<bool>;
#[doc = "Field `CLK_ILO_EN_FOR_WDT` writer - Enables the ILO clock for use by the WDT logic. Wait at least 4 ILO clock cycles for a change to take effect. Must be 0 when switching WDT_CONFIG.LFCLK_SEL. Should be 0 if CLK_WCO_EN_FOR_WDT=1."]
pub type CLK_ILO_EN_FOR_WDT_W<'a> = crate::BitWriter<'a, u32, WDT_CLKEN_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - Enables the WCO clock for use by the WDT logic. Wait at least 4 WCO clock cycles for a change to take effect. Must be 0 when switching WDT_CONFIG.LFCLK_SEL. Should be 0 if CLK_ILO_EN_FOR_WDT=1"]
    #[inline(always)]
    pub fn clk_wco_en_for_wdt(&self) -> CLK_WCO_EN_FOR_WDT_R {
        CLK_WCO_EN_FOR_WDT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables the ILO clock for use by the WDT logic. Wait at least 4 ILO clock cycles for a change to take effect. Must be 0 when switching WDT_CONFIG.LFCLK_SEL. Should be 0 if CLK_WCO_EN_FOR_WDT=1."]
    #[inline(always)]
    pub fn clk_ilo_en_for_wdt(&self) -> CLK_ILO_EN_FOR_WDT_R {
        CLK_ILO_EN_FOR_WDT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the WCO clock for use by the WDT logic. Wait at least 4 WCO clock cycles for a change to take effect. Must be 0 when switching WDT_CONFIG.LFCLK_SEL. Should be 0 if CLK_ILO_EN_FOR_WDT=1"]
    #[inline(always)]
    pub fn clk_wco_en_for_wdt(&mut self) -> CLK_WCO_EN_FOR_WDT_W {
        CLK_WCO_EN_FOR_WDT_W::new(self)
    }
    #[doc = "Bit 1 - Enables the ILO clock for use by the WDT logic. Wait at least 4 ILO clock cycles for a change to take effect. Must be 0 when switching WDT_CONFIG.LFCLK_SEL. Should be 0 if CLK_WCO_EN_FOR_WDT=1."]
    #[inline(always)]
    pub fn clk_ilo_en_for_wdt(&mut self) -> CLK_ILO_EN_FOR_WDT_W {
        CLK_ILO_EN_FOR_WDT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Counters Clock Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdt_clken](index.html) module"]
pub struct WDT_CLKEN_SPEC;
impl crate::RegisterSpec for WDT_CLKEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdt_clken::R](R) reader structure"]
impl crate::Readable for WDT_CLKEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdt_clken::W](W) writer structure"]
impl crate::Writable for WDT_CLKEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDT_CLKEN to value 0"]
impl crate::Resettable for WDT_CLKEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
