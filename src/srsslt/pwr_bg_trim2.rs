#[doc = "Register `PWR_BG_TRIM2` reader"]
pub struct R(crate::R<PWR_BG_TRIM2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_BG_TRIM2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_BG_TRIM2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_BG_TRIM2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_BG_TRIM2` writer"]
pub struct W(crate::W<PWR_BG_TRIM2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_BG_TRIM2_SPEC>;
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
impl From<crate::W<PWR_BG_TRIM2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_BG_TRIM2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REF_ITRIM` reader - Trims the bandgap reference current output. Used to trim the IBG to the voltage where its temperature curvature is minimal."]
pub type REF_ITRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REF_ITRIM` writer - Trims the bandgap reference current output. Used to trim the IBG to the voltage where its temperature curvature is minimal."]
pub type REF_ITRIM_W<'a> = crate::FieldWriter<'a, u32, PWR_BG_TRIM2_SPEC, u8, u8, 6, 0>;
impl R {
    #[doc = "Bits 0:5 - Trims the bandgap reference current output. Used to trim the IBG to the voltage where its temperature curvature is minimal."]
    #[inline(always)]
    pub fn ref_itrim(&self) -> REF_ITRIM_R {
        REF_ITRIM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Trims the bandgap reference current output. Used to trim the IBG to the voltage where its temperature curvature is minimal."]
    #[inline(always)]
    pub fn ref_itrim(&mut self) -> REF_ITRIM_W {
        REF_ITRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bandgap Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_bg_trim2](index.html) module"]
pub struct PWR_BG_TRIM2_SPEC;
impl crate::RegisterSpec for PWR_BG_TRIM2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_bg_trim2::R](R) reader structure"]
impl crate::Readable for PWR_BG_TRIM2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_bg_trim2::W](W) writer structure"]
impl crate::Writable for PWR_BG_TRIM2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_BG_TRIM2 to value 0x1c"]
impl crate::Resettable for PWR_BG_TRIM2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1c
    }
}
