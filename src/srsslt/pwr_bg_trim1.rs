#[doc = "Register `PWR_BG_TRIM1` reader"]
pub struct R(crate::R<PWR_BG_TRIM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_BG_TRIM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_BG_TRIM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_BG_TRIM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_BG_TRIM1` writer"]
pub struct W(crate::W<PWR_BG_TRIM1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_BG_TRIM1_SPEC>;
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
impl From<crate::W<PWR_BG_TRIM1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_BG_TRIM1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REF_VTRIM` reader - Trims the bandgap reference voltage output. Used to trim the VBG to the voltage where its temperature curvature is minimal. Bit \\[5\\]
is unused within the bandgap block."]
pub type REF_VTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REF_VTRIM` writer - Trims the bandgap reference voltage output. Used to trim the VBG to the voltage where its temperature curvature is minimal. Bit \\[5\\]
is unused within the bandgap block."]
pub type REF_VTRIM_W<'a> = crate::FieldWriter<'a, u32, PWR_BG_TRIM1_SPEC, u8, u8, 6, 0>;
impl R {
    #[doc = "Bits 0:5 - Trims the bandgap reference voltage output. Used to trim the VBG to the voltage where its temperature curvature is minimal. Bit \\[5\\]
is unused within the bandgap block."]
    #[inline(always)]
    pub fn ref_vtrim(&self) -> REF_VTRIM_R {
        REF_VTRIM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Trims the bandgap reference voltage output. Used to trim the VBG to the voltage where its temperature curvature is minimal. Bit \\[5\\]
is unused within the bandgap block."]
    #[inline(always)]
    pub fn ref_vtrim(&mut self) -> REF_VTRIM_W {
        REF_VTRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bandgap Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_bg_trim1](index.html) module"]
pub struct PWR_BG_TRIM1_SPEC;
impl crate::RegisterSpec for PWR_BG_TRIM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_bg_trim1::R](R) reader structure"]
impl crate::Readable for PWR_BG_TRIM1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_bg_trim1::W](W) writer structure"]
impl crate::Writable for PWR_BG_TRIM1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_BG_TRIM1 to value 0x10"]
impl crate::Resettable for PWR_BG_TRIM1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
