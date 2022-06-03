#[doc = "Register `DSAB_DFT` reader"]
pub struct R(crate::R<DSAB_DFT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSAB_DFT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSAB_DFT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSAB_DFT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSAB_DFT` writer"]
pub struct W(crate::W<DSAB_DFT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSAB_DFT_SPEC>;
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
impl From<crate::W<DSAB_DFT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSAB_DFT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN_DFT` reader - - 0: DSAB DFT disabled - 1: DSAB DFT enabled (connect output to amuxbus) 0001 - PTAT<0> 0010 - PTAT<1> 0011 - PTAT<1:0> 0100 - PTAT<2> 0111 - PTAT<2:0> 1000 - PTAT<3> 1111 - PTAT<3:0> 1001 - DSAB Reg Out"]
pub type EN_DFT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EN_DFT` writer - - 0: DSAB DFT disabled - 1: DSAB DFT enabled (connect output to amuxbus) 0001 - PTAT<0> 0010 - PTAT<1> 0011 - PTAT<1:0> 0100 - PTAT<2> 0111 - PTAT<2:0> 1000 - PTAT<3> 1111 - PTAT<3:0> 1001 - DSAB Reg Out"]
pub type EN_DFT_W<'a> = crate::FieldWriter<'a, u32, DSAB_DFT_SPEC, u8, u8, 4, 0>;
impl R {
    #[doc = "Bits 0:3 - - 0: DSAB DFT disabled - 1: DSAB DFT enabled (connect output to amuxbus) 0001 - PTAT<0> 0010 - PTAT<1> 0011 - PTAT<1:0> 0100 - PTAT<2> 0111 - PTAT<2:0> 1000 - PTAT<3> 1111 - PTAT<3:0> 1001 - DSAB Reg Out"]
    #[inline(always)]
    pub fn en_dft(&self) -> EN_DFT_R {
        EN_DFT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - - 0: DSAB DFT disabled - 1: DSAB DFT enabled (connect output to amuxbus) 0001 - PTAT<0> 0010 - PTAT<1> 0011 - PTAT<1:0> 0100 - PTAT<2> 0111 - PTAT<2:0> 1000 - PTAT<3> 1111 - PTAT<3:0> 1001 - DSAB Reg Out"]
    #[inline(always)]
    pub fn en_dft(&mut self) -> EN_DFT_W {
        EN_DFT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFT bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsab_dft](index.html) module"]
pub struct DSAB_DFT_SPEC;
impl crate::RegisterSpec for DSAB_DFT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsab_dft::R](R) reader structure"]
impl crate::Readable for DSAB_DFT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsab_dft::W](W) writer structure"]
impl crate::Writable for DSAB_DFT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DSAB_DFT to value 0"]
impl crate::Resettable for DSAB_DFT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
