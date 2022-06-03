#[doc = "Register `DSAB_TRIM` reader"]
pub struct R(crate::R<DSAB_TRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSAB_TRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSAB_TRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSAB_TRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSAB_TRIM` writer"]
pub struct W(crate::W<DSAB_TRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSAB_TRIM_SPEC>;
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
impl From<crate::W<DSAB_TRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSAB_TRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IBIAS_TRIM` reader - 1111=lowest, 0000=highest"]
pub type IBIAS_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IBIAS_TRIM` writer - 1111=lowest, 0000=highest"]
pub type IBIAS_TRIM_W<'a> = crate::FieldWriter<'a, u32, DSAB_TRIM_SPEC, u8, u8, 4, 0>;
#[doc = "Field `DSAB_RMB_BITS` reader - Risk mitigation bits"]
pub type DSAB_RMB_BITS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DSAB_RMB_BITS` writer - Risk mitigation bits"]
pub type DSAB_RMB_BITS_W<'a> = crate::FieldWriter<'a, u32, DSAB_TRIM_SPEC, u8, u8, 2, 4>;
impl R {
    #[doc = "Bits 0:3 - 1111=lowest, 0000=highest"]
    #[inline(always)]
    pub fn ibias_trim(&self) -> IBIAS_TRIM_R {
        IBIAS_TRIM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Risk mitigation bits"]
    #[inline(always)]
    pub fn dsab_rmb_bits(&self) -> DSAB_RMB_BITS_R {
        DSAB_RMB_BITS_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1111=lowest, 0000=highest"]
    #[inline(always)]
    pub fn ibias_trim(&mut self) -> IBIAS_TRIM_W {
        IBIAS_TRIM_W::new(self)
    }
    #[doc = "Bits 4:5 - Risk mitigation bits"]
    #[inline(always)]
    pub fn dsab_rmb_bits(&mut self) -> DSAB_RMB_BITS_W {
        DSAB_RMB_BITS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSAB Trim bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsab_trim](index.html) module"]
pub struct DSAB_TRIM_SPEC;
impl crate::RegisterSpec for DSAB_TRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsab_trim::R](R) reader structure"]
impl crate::Readable for DSAB_TRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsab_trim::W](W) writer structure"]
impl crate::Writable for DSAB_TRIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DSAB_TRIM to value 0"]
impl crate::Resettable for DSAB_TRIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
