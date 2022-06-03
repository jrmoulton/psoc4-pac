#[doc = "Register `DPLL` reader"]
pub struct R(crate::R<DPLL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPLL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPLL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPLL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPLL` writer"]
pub struct W(crate::W<DPLL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPLL_SPEC>;
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
impl From<crate::W<DPLL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPLL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DPLL_MULT` reader - Multiplier to determine IMO frequency in multiples of the WCO frequency Fimo = (DPLL_MULT + 1) * Fwco"]
pub type DPLL_MULT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DPLL_MULT` writer - Multiplier to determine IMO frequency in multiples of the WCO frequency Fimo = (DPLL_MULT + 1) * Fwco"]
pub type DPLL_MULT_W<'a> = crate::FieldWriter<'a, u32, DPLL_SPEC, u16, u16, 11, 0>;
#[doc = "Field `DPLL_LF_IGAIN` reader - DPLL Loop Filter Integral Gain Setting 0x0 - 0.0625 0x1 - 0.125 0x2 - 0.25 0x3 - 0.5 0x4 - 1.0 0x5 - 2.0 0x6 - 4.0 0x7 - 8.0"]
pub type DPLL_LF_IGAIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DPLL_LF_IGAIN` writer - DPLL Loop Filter Integral Gain Setting 0x0 - 0.0625 0x1 - 0.125 0x2 - 0.25 0x3 - 0.5 0x4 - 1.0 0x5 - 2.0 0x6 - 4.0 0x7 - 8.0"]
pub type DPLL_LF_IGAIN_W<'a> = crate::FieldWriter<'a, u32, DPLL_SPEC, u8, u8, 3, 16>;
#[doc = "Field `DPLL_LF_PGAIN` reader - DPLL Loop Filter Proportionial Gain Setting 0x0 - 0.0625 0x1 - 0.125 0x2 - 0.25 0x3 - 0.5 0x4 - 1.0 0x5 - 2.0 0x6 - 4.0 0x7 - 8.0"]
pub type DPLL_LF_PGAIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DPLL_LF_PGAIN` writer - DPLL Loop Filter Proportionial Gain Setting 0x0 - 0.0625 0x1 - 0.125 0x2 - 0.25 0x3 - 0.5 0x4 - 1.0 0x5 - 2.0 0x6 - 4.0 0x7 - 8.0"]
pub type DPLL_LF_PGAIN_W<'a> = crate::FieldWriter<'a, u32, DPLL_SPEC, u8, u8, 3, 19>;
#[doc = "Field `DPLL_LF_LIMIT` reader - Maximum IMO offset allowed (used to prevent DPLL dynamics from selecting an IMO frequency that the logic cannot support)"]
pub type DPLL_LF_LIMIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DPLL_LF_LIMIT` writer - Maximum IMO offset allowed (used to prevent DPLL dynamics from selecting an IMO frequency that the logic cannot support)"]
pub type DPLL_LF_LIMIT_W<'a> = crate::FieldWriter<'a, u32, DPLL_SPEC, u8, u8, 8, 22>;
impl R {
    #[doc = "Bits 0:10 - Multiplier to determine IMO frequency in multiples of the WCO frequency Fimo = (DPLL_MULT + 1) * Fwco"]
    #[inline(always)]
    pub fn dpll_mult(&self) -> DPLL_MULT_R {
        DPLL_MULT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:18 - DPLL Loop Filter Integral Gain Setting 0x0 - 0.0625 0x1 - 0.125 0x2 - 0.25 0x3 - 0.5 0x4 - 1.0 0x5 - 2.0 0x6 - 4.0 0x7 - 8.0"]
    #[inline(always)]
    pub fn dpll_lf_igain(&self) -> DPLL_LF_IGAIN_R {
        DPLL_LF_IGAIN_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - DPLL Loop Filter Proportionial Gain Setting 0x0 - 0.0625 0x1 - 0.125 0x2 - 0.25 0x3 - 0.5 0x4 - 1.0 0x5 - 2.0 0x6 - 4.0 0x7 - 8.0"]
    #[inline(always)]
    pub fn dpll_lf_pgain(&self) -> DPLL_LF_PGAIN_R {
        DPLL_LF_PGAIN_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:29 - Maximum IMO offset allowed (used to prevent DPLL dynamics from selecting an IMO frequency that the logic cannot support)"]
    #[inline(always)]
    pub fn dpll_lf_limit(&self) -> DPLL_LF_LIMIT_R {
        DPLL_LF_LIMIT_R::new(((self.bits >> 22) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - Multiplier to determine IMO frequency in multiples of the WCO frequency Fimo = (DPLL_MULT + 1) * Fwco"]
    #[inline(always)]
    pub fn dpll_mult(&mut self) -> DPLL_MULT_W {
        DPLL_MULT_W::new(self)
    }
    #[doc = "Bits 16:18 - DPLL Loop Filter Integral Gain Setting 0x0 - 0.0625 0x1 - 0.125 0x2 - 0.25 0x3 - 0.5 0x4 - 1.0 0x5 - 2.0 0x6 - 4.0 0x7 - 8.0"]
    #[inline(always)]
    pub fn dpll_lf_igain(&mut self) -> DPLL_LF_IGAIN_W {
        DPLL_LF_IGAIN_W::new(self)
    }
    #[doc = "Bits 19:21 - DPLL Loop Filter Proportionial Gain Setting 0x0 - 0.0625 0x1 - 0.125 0x2 - 0.25 0x3 - 0.5 0x4 - 1.0 0x5 - 2.0 0x6 - 4.0 0x7 - 8.0"]
    #[inline(always)]
    pub fn dpll_lf_pgain(&mut self) -> DPLL_LF_PGAIN_W {
        DPLL_LF_PGAIN_W::new(self)
    }
    #[doc = "Bits 22:29 - Maximum IMO offset allowed (used to prevent DPLL dynamics from selecting an IMO frequency that the logic cannot support)"]
    #[inline(always)]
    pub fn dpll_lf_limit(&mut self) -> DPLL_LF_LIMIT_W {
        DPLL_LF_LIMIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WCO DPLL Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpll](index.html) module"]
pub struct DPLL_SPEC;
impl crate::RegisterSpec for DPLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dpll::R](R) reader structure"]
impl crate::Readable for DPLL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dpll::W](W) writer structure"]
impl crate::Writable for DPLL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DPLL to value 0x3fc0_0000"]
impl crate::Resettable for DPLL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3fc0_0000
    }
}
