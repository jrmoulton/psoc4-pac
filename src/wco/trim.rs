#[doc = "Register `TRIM` reader"]
pub struct R(crate::R<TRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIM` writer"]
pub struct W(crate::W<TRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIM_SPEC>;
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
impl From<crate::W<TRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XGM` reader - Amplifier GM setting - Used when WCO.LPM_AUTO=0 or when LPM_AUTO=1 and not in DeepSleep mode. 0x0 - 3370 nA 0x1 - 2620 nA 0x2 - 2250 nA 0x3 - 1500 nA 0x4 - 1870 nA 0x5 - 1120 nA 0x6 - 750 nA 0x7 - 0 nA"]
pub type XGM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XGM` writer - Amplifier GM setting - Used when WCO.LPM_AUTO=0 or when LPM_AUTO=1 and not in DeepSleep mode. 0x0 - 3370 nA 0x1 - 2620 nA 0x2 - 2250 nA 0x3 - 1500 nA 0x4 - 1870 nA 0x5 - 1120 nA 0x6 - 750 nA 0x7 - 0 nA"]
pub type XGM_W<'a> = crate::FieldWriter<'a, u32, TRIM_SPEC, u8, u8, 3, 0>;
#[doc = "Field `LPM_GM` reader - GM setting for LPM (bandwidth = DC/ms) - Used when WCO.LPM_AUTO=0 or when LPM_AUTO=1 and not in DeepSleep mode."]
pub type LPM_GM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPM_GM` writer - GM setting for LPM (bandwidth = DC/ms) - Used when WCO.LPM_AUTO=0 or when LPM_AUTO=1 and not in DeepSleep mode."]
pub type LPM_GM_W<'a> = crate::FieldWriter<'a, u32, TRIM_SPEC, u8, u8, 2, 4>;
impl R {
    #[doc = "Bits 0:2 - Amplifier GM setting - Used when WCO.LPM_AUTO=0 or when LPM_AUTO=1 and not in DeepSleep mode. 0x0 - 3370 nA 0x1 - 2620 nA 0x2 - 2250 nA 0x3 - 1500 nA 0x4 - 1870 nA 0x5 - 1120 nA 0x6 - 750 nA 0x7 - 0 nA"]
    #[inline(always)]
    pub fn xgm(&self) -> XGM_R {
        XGM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5 - GM setting for LPM (bandwidth = DC/ms) - Used when WCO.LPM_AUTO=0 or when LPM_AUTO=1 and not in DeepSleep mode."]
    #[inline(always)]
    pub fn lpm_gm(&self) -> LPM_GM_R {
        LPM_GM_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Amplifier GM setting - Used when WCO.LPM_AUTO=0 or when LPM_AUTO=1 and not in DeepSleep mode. 0x0 - 3370 nA 0x1 - 2620 nA 0x2 - 2250 nA 0x3 - 1500 nA 0x4 - 1870 nA 0x5 - 1120 nA 0x6 - 750 nA 0x7 - 0 nA"]
    #[inline(always)]
    pub fn xgm(&mut self) -> XGM_W {
        XGM_W::new(self)
    }
    #[doc = "Bits 4:5 - GM setting for LPM (bandwidth = DC/ms) - Used when WCO.LPM_AUTO=0 or when LPM_AUTO=1 and not in DeepSleep mode."]
    #[inline(always)]
    pub fn lpm_gm(&mut self) -> LPM_GM_W {
        LPM_GM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WCO Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trim](index.html) module"]
pub struct TRIM_SPEC;
impl crate::RegisterSpec for TRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trim::R](R) reader structure"]
impl crate::Readable for TRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trim::W](W) writer structure"]
impl crate::Writable for TRIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRIM to value 0x11"]
impl crate::Resettable for TRIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x11
    }
}
