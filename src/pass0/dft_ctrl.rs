#[doc = "Register `DFT_CTRL` reader"]
pub struct R(crate::R<DFT_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFT_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFT_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFT_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFT_CTRL` writer"]
pub struct W(crate::W<DFT_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFT_CTRL_SPEC>;
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
impl From<crate::W<DFT_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFT_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSAB_ADFT_RES_EN` reader - Close the switch to connect the DSAB ADFT resistor to the AMUXBUS"]
pub type DSAB_ADFT_RES_EN_R = crate::BitReader<bool>;
#[doc = "Field `DSAB_ADFT_RES_EN` writer - Close the switch to connect the DSAB ADFT resistor to the AMUXBUS"]
pub type DSAB_ADFT_RES_EN_W<'a> = crate::BitWriter<'a, u32, DFT_CTRL_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Close the switch to connect the DSAB ADFT resistor to the AMUXBUS"]
    #[inline(always)]
    pub fn dsab_adft_res_en(&self) -> DSAB_ADFT_RES_EN_R {
        DSAB_ADFT_RES_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Close the switch to connect the DSAB ADFT resistor to the AMUXBUS"]
    #[inline(always)]
    pub fn dsab_adft_res_en(&mut self) -> DSAB_ADFT_RES_EN_W {
        DSAB_ADFT_RES_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFT control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dft_ctrl](index.html) module"]
pub struct DFT_CTRL_SPEC;
impl crate::RegisterSpec for DFT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dft_ctrl::R](R) reader structure"]
impl crate::Readable for DFT_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dft_ctrl::W](W) writer structure"]
impl crate::Writable for DFT_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DFT_CTRL to value 0"]
impl crate::Resettable for DFT_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
