#[doc = "Register `CONFIG` reader"]
pub struct R(crate::R<CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG` writer"]
pub struct W(crate::W<CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_SPEC>;
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
impl From<crate::W<CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPM_EN` reader - Force block into Low Power Mode: 0: Do not force low power mode (LPM) on 1: Force low power mode (LPM) on"]
pub type LPM_EN_R = crate::BitReader<bool>;
#[doc = "Field `LPM_EN` writer - Force block into Low Power Mode: 0: Do not force low power mode (LPM) on 1: Force low power mode (LPM) on"]
pub type LPM_EN_W<'a> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, 0>;
#[doc = "Field `LPM_AUTO` reader - Automatically control low power mode (only relevant when LPM_EN=0): 0: Do not enter low power mode (LPM) in DeepSleep 1: Enter low power mode (LPM) in DeepSleep. The logic monitors !act_power_en to determine the device has entered DeepSleep."]
pub type LPM_AUTO_R = crate::BitReader<bool>;
#[doc = "Field `LPM_AUTO` writer - Automatically control low power mode (only relevant when LPM_EN=0): 0: Do not enter low power mode (LPM) in DeepSleep 1: Enter low power mode (LPM) in DeepSleep. The logic monitors !act_power_en to determine the device has entered DeepSleep."]
pub type LPM_AUTO_W<'a> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, 1>;
#[doc = "Field `EXT_INPUT_EN` reader - Disables the load resistor and allows external clock input for pad_xin"]
pub type EXT_INPUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `EXT_INPUT_EN` writer - Disables the load resistor and allows external clock input for pad_xin"]
pub type EXT_INPUT_EN_W<'a> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, 2>;
#[doc = "Field `ENBUS` reader - Test Mode Control bits enbus\\[7\\]
- N/A enbus\\[6\\]
- 1=enable both primary Beta Multipliers enbus\\[5\\]
- N/A enbus\\[4\\]
- N/A enbus\\[3\\]
- Load Resistor Control enbus\\[2\\]
- Load Resistor Control enbus\\[1\\]
- Load Resistor Control enbus\\[0\\]
- Load Resistor Control"]
pub type ENBUS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ENBUS` writer - Test Mode Control bits enbus\\[7\\]
- N/A enbus\\[6\\]
- 1=enable both primary Beta Multipliers enbus\\[5\\]
- N/A enbus\\[4\\]
- N/A enbus\\[3\\]
- Load Resistor Control enbus\\[2\\]
- Load Resistor Control enbus\\[1\\]
- Load Resistor Control enbus\\[0\\]
- Load Resistor Control"]
pub type ENBUS_W<'a> = crate::FieldWriter<'a, u32, CONFIG_SPEC, u8, u8, 8, 16>;
#[doc = "Field `DPLL_ENABLE` reader - Enable DPLL operation. The Oscillator is specified to be stable after 500 ms thus the DPLL should be asserted no sooner than that after IP_ENABLE is set."]
pub type DPLL_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `DPLL_ENABLE` writer - Enable DPLL operation. The Oscillator is specified to be stable after 500 ms thus the DPLL should be asserted no sooner than that after IP_ENABLE is set."]
pub type DPLL_ENABLE_W<'a> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, 30>;
#[doc = "Field `IP_ENABLE` reader - Master enable for IP - disables both WCO and DPLL"]
pub type IP_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `IP_ENABLE` writer - Master enable for IP - disables both WCO and DPLL"]
pub type IP_ENABLE_W<'a> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - Force block into Low Power Mode: 0: Do not force low power mode (LPM) on 1: Force low power mode (LPM) on"]
    #[inline(always)]
    pub fn lpm_en(&self) -> LPM_EN_R {
        LPM_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Automatically control low power mode (only relevant when LPM_EN=0): 0: Do not enter low power mode (LPM) in DeepSleep 1: Enter low power mode (LPM) in DeepSleep. The logic monitors !act_power_en to determine the device has entered DeepSleep."]
    #[inline(always)]
    pub fn lpm_auto(&self) -> LPM_AUTO_R {
        LPM_AUTO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disables the load resistor and allows external clock input for pad_xin"]
    #[inline(always)]
    pub fn ext_input_en(&self) -> EXT_INPUT_EN_R {
        EXT_INPUT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Test Mode Control bits enbus\\[7\\]
- N/A enbus\\[6\\]
- 1=enable both primary Beta Multipliers enbus\\[5\\]
- N/A enbus\\[4\\]
- N/A enbus\\[3\\]
- Load Resistor Control enbus\\[2\\]
- Load Resistor Control enbus\\[1\\]
- Load Resistor Control enbus\\[0\\]
- Load Resistor Control"]
    #[inline(always)]
    pub fn enbus(&self) -> ENBUS_R {
        ENBUS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 30 - Enable DPLL operation. The Oscillator is specified to be stable after 500 ms thus the DPLL should be asserted no sooner than that after IP_ENABLE is set."]
    #[inline(always)]
    pub fn dpll_enable(&self) -> DPLL_ENABLE_R {
        DPLL_ENABLE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Master enable for IP - disables both WCO and DPLL"]
    #[inline(always)]
    pub fn ip_enable(&self) -> IP_ENABLE_R {
        IP_ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force block into Low Power Mode: 0: Do not force low power mode (LPM) on 1: Force low power mode (LPM) on"]
    #[inline(always)]
    pub fn lpm_en(&mut self) -> LPM_EN_W {
        LPM_EN_W::new(self)
    }
    #[doc = "Bit 1 - Automatically control low power mode (only relevant when LPM_EN=0): 0: Do not enter low power mode (LPM) in DeepSleep 1: Enter low power mode (LPM) in DeepSleep. The logic monitors !act_power_en to determine the device has entered DeepSleep."]
    #[inline(always)]
    pub fn lpm_auto(&mut self) -> LPM_AUTO_W {
        LPM_AUTO_W::new(self)
    }
    #[doc = "Bit 2 - Disables the load resistor and allows external clock input for pad_xin"]
    #[inline(always)]
    pub fn ext_input_en(&mut self) -> EXT_INPUT_EN_W {
        EXT_INPUT_EN_W::new(self)
    }
    #[doc = "Bits 16:23 - Test Mode Control bits enbus\\[7\\]
- N/A enbus\\[6\\]
- 1=enable both primary Beta Multipliers enbus\\[5\\]
- N/A enbus\\[4\\]
- N/A enbus\\[3\\]
- Load Resistor Control enbus\\[2\\]
- Load Resistor Control enbus\\[1\\]
- Load Resistor Control enbus\\[0\\]
- Load Resistor Control"]
    #[inline(always)]
    pub fn enbus(&mut self) -> ENBUS_W {
        ENBUS_W::new(self)
    }
    #[doc = "Bit 30 - Enable DPLL operation. The Oscillator is specified to be stable after 500 ms thus the DPLL should be asserted no sooner than that after IP_ENABLE is set."]
    #[inline(always)]
    pub fn dpll_enable(&mut self) -> DPLL_ENABLE_W {
        DPLL_ENABLE_W::new(self)
    }
    #[doc = "Bit 31 - Master enable for IP - disables both WCO and DPLL"]
    #[inline(always)]
    pub fn ip_enable(&mut self) -> IP_ENABLE_W {
        IP_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WCO Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](index.html) module"]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config::R](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config::W](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONFIG to value 0x0047_0002"]
impl crate::Resettable for CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0047_0002
    }
}
