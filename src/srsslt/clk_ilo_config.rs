#[doc = "Register `CLK_ILO_CONFIG` reader"]
pub struct R(crate::R<CLK_ILO_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_ILO_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_ILO_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_ILO_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_ILO_CONFIG` writer"]
pub struct W(crate::W<CLK_ILO_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_ILO_CONFIG_SPEC>;
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
impl From<crate::W<CLK_ILO_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_ILO_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Master enable for ILO oscillator. This bit is hardware set whenever the WDT_DISABLE_KEY is not set to the magic value."]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Master enable for ILO oscillator. This bit is hardware set whenever the WDT_DISABLE_KEY is not set to the magic value."]
pub type ENABLE_W<'a> = crate::BitWriter<'a, u32, CLK_ILO_CONFIG_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 31 - Master enable for ILO oscillator. This bit is hardware set whenever the WDT_DISABLE_KEY is not set to the magic value."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Master enable for ILO oscillator. This bit is hardware set whenever the WDT_DISABLE_KEY is not set to the magic value."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ILO Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_ilo_config](index.html) module"]
pub struct CLK_ILO_CONFIG_SPEC;
impl crate::RegisterSpec for CLK_ILO_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_ilo_config::R](R) reader structure"]
impl crate::Readable for CLK_ILO_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_ilo_config::W](W) writer structure"]
impl crate::Writable for CLK_ILO_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_ILO_CONFIG to value 0x8000_0000"]
impl crate::Resettable for CLK_ILO_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
