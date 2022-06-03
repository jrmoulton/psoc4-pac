#[doc = "Register `VREFGEN` reader"]
pub struct R(crate::R<VREFGEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VREFGEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VREFGEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VREFGEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VREFGEN` writer"]
pub struct W(crate::W<VREFGEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VREFGEN_SPEC>;
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
impl From<crate::W<VREFGEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VREFGEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REF_SEL` reader - Reference selection. A reference Voltage vinref is created using a Voltage vddio: '0': vinref = (0 * 13 + 184)/600 * vddio = 184/600 * vddio. '1': vinref = (1 * 13 + 184)/600 * vddio = 197/600 * vddio. '2': vinref = (2 * 13 + 184)/600 * vddio = 210/600 * vddio. ... '31': vinref = (31 * 13 + 184)/600 * vddio = 587/600 * vddio."]
pub type REF_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REF_SEL` writer - Reference selection. A reference Voltage vinref is created using a Voltage vddio: '0': vinref = (0 * 13 + 184)/600 * vddio = 184/600 * vddio. '1': vinref = (1 * 13 + 184)/600 * vddio = 197/600 * vddio. '2': vinref = (2 * 13 + 184)/600 * vddio = 210/600 * vddio. ... '31': vinref = (31 * 13 + 184)/600 * vddio = 587/600 * vddio."]
pub type REF_SEL_W<'a> = crate::FieldWriter<'a, u32, VREFGEN_SPEC, u8, u8, 5, 0>;
#[doc = "Field `VREFGEN_EN` reader - Reference generator enable: '0': Disabled. '1': Enabled."]
pub type VREFGEN_EN_R = crate::BitReader<bool>;
#[doc = "Field `VREFGEN_EN` writer - Reference generator enable: '0': Disabled. '1': Enabled."]
pub type VREFGEN_EN_W<'a> = crate::BitWriter<'a, u32, VREFGEN_SPEC, bool, 8>;
impl R {
    #[doc = "Bits 0:4 - Reference selection. A reference Voltage vinref is created using a Voltage vddio: '0': vinref = (0 * 13 + 184)/600 * vddio = 184/600 * vddio. '1': vinref = (1 * 13 + 184)/600 * vddio = 197/600 * vddio. '2': vinref = (2 * 13 + 184)/600 * vddio = 210/600 * vddio. ... '31': vinref = (31 * 13 + 184)/600 * vddio = 587/600 * vddio."]
    #[inline(always)]
    pub fn ref_sel(&self) -> REF_SEL_R {
        REF_SEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Reference generator enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn vrefgen_en(&self) -> VREFGEN_EN_R {
        VREFGEN_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Reference selection. A reference Voltage vinref is created using a Voltage vddio: '0': vinref = (0 * 13 + 184)/600 * vddio = 184/600 * vddio. '1': vinref = (1 * 13 + 184)/600 * vddio = 197/600 * vddio. '2': vinref = (2 * 13 + 184)/600 * vddio = 210/600 * vddio. ... '31': vinref = (31 * 13 + 184)/600 * vddio = 587/600 * vddio."]
    #[inline(always)]
    pub fn ref_sel(&mut self) -> REF_SEL_W {
        REF_SEL_W::new(self)
    }
    #[doc = "Bit 8 - Reference generator enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn vrefgen_en(&mut self) -> VREFGEN_EN_W {
        VREFGEN_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reference generator configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vrefgen](index.html) module"]
pub struct VREFGEN_SPEC;
impl crate::RegisterSpec for VREFGEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vrefgen::R](R) reader structure"]
impl crate::Readable for VREFGEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vrefgen::W](W) writer structure"]
impl crate::Writable for VREFGEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VREFGEN to value 0"]
impl crate::Resettable for VREFGEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
