#[doc = "Register `ROM_CTL` reader"]
pub struct R(crate::R<ROM_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROM_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROM_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROM_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROM_CTL` writer"]
pub struct W(crate::W<ROM_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROM_CTL_SPEC>;
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
impl From<crate::W<ROM_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROM_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROM_WS` reader - Amount of ROM wait states: '0': 0 wait states. Use this setting for newer, faster ROM design. Use this setting for older, slower ROM design and frequencies in the range \\[0, 24\\]
MHz. '1': 1 wait state. Use this setting for older, slower ROM design and frequencies in the range <24, 48\\]
MHz. CPUSSv2 supports two types of ROM memory: an older, slower design (operating at up to 24 MHz) and a newer, faster design (operating at up to 48 MHz). The older design requires 1 wait state for frequencies above 24 MHz. The newer design never requires wait states. All chips after Street Fighter will use the newer design. As a result, all chips after Street Fighter can always use 0 wait states."]
pub type ROM_WS_R = crate::BitReader<bool>;
#[doc = "Field `ROM_WS` writer - Amount of ROM wait states: '0': 0 wait states. Use this setting for newer, faster ROM design. Use this setting for older, slower ROM design and frequencies in the range \\[0, 24\\]
MHz. '1': 1 wait state. Use this setting for older, slower ROM design and frequencies in the range <24, 48\\]
MHz. CPUSSv2 supports two types of ROM memory: an older, slower design (operating at up to 24 MHz) and a newer, faster design (operating at up to 48 MHz). The older design requires 1 wait state for frequencies above 24 MHz. The newer design never requires wait states. All chips after Street Fighter will use the newer design. As a result, all chips after Street Fighter can always use 0 wait states."]
pub type ROM_WS_W<'a> = crate::BitWriter<'a, u32, ROM_CTL_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Amount of ROM wait states: '0': 0 wait states. Use this setting for newer, faster ROM design. Use this setting for older, slower ROM design and frequencies in the range \\[0, 24\\]
MHz. '1': 1 wait state. Use this setting for older, slower ROM design and frequencies in the range <24, 48\\]
MHz. CPUSSv2 supports two types of ROM memory: an older, slower design (operating at up to 24 MHz) and a newer, faster design (operating at up to 48 MHz). The older design requires 1 wait state for frequencies above 24 MHz. The newer design never requires wait states. All chips after Street Fighter will use the newer design. As a result, all chips after Street Fighter can always use 0 wait states."]
    #[inline(always)]
    pub fn rom_ws(&self) -> ROM_WS_R {
        ROM_WS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Amount of ROM wait states: '0': 0 wait states. Use this setting for newer, faster ROM design. Use this setting for older, slower ROM design and frequencies in the range \\[0, 24\\]
MHz. '1': 1 wait state. Use this setting for older, slower ROM design and frequencies in the range <24, 48\\]
MHz. CPUSSv2 supports two types of ROM memory: an older, slower design (operating at up to 24 MHz) and a newer, faster design (operating at up to 48 MHz). The older design requires 1 wait state for frequencies above 24 MHz. The newer design never requires wait states. All chips after Street Fighter will use the newer design. As a result, all chips after Street Fighter can always use 0 wait states."]
    #[inline(always)]
    pub fn rom_ws(&mut self) -> ROM_WS_W {
        ROM_WS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ROM control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rom_ctl](index.html) module"]
pub struct ROM_CTL_SPEC;
impl crate::RegisterSpec for ROM_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rom_ctl::R](R) reader structure"]
impl crate::Readable for ROM_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rom_ctl::W](W) writer structure"]
impl crate::Writable for ROM_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ROM_CTL to value 0"]
impl crate::Resettable for ROM_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
