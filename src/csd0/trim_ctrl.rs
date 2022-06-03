#[doc = "Register `TRIM_CTRL` reader"]
pub struct R(crate::R<TRIM_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIM_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIM_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIM_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIM_CTRL` writer"]
pub struct W(crate::W<TRIM_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIM_CTRL_SPEC>;
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
impl From<crate::W<TRIM_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIM_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DELAY_TRIM` reader - Trim input for Shield Delay block. Risk mitigation only; no test program or calpairs are needed for this"]
pub type DELAY_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DELAY_TRIM` writer - Trim input for Shield Delay block. Risk mitigation only; no test program or calpairs are needed for this"]
pub type DELAY_TRIM_W<'a> = crate::FieldWriter<'a, u32, TRIM_CTRL_SPEC, u8, u8, 2, 0>;
#[doc = "Field `DELAY_HYS` reader - Hystersis input for Shield Delay block. Risk mitigation only; no test program or calpairs are needed for this"]
pub type DELAY_HYS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DELAY_HYS` writer - Hystersis input for Shield Delay block. Risk mitigation only; no test program or calpairs are needed for this"]
pub type DELAY_HYS_W<'a> = crate::FieldWriter<'a, u32, TRIM_CTRL_SPEC, u8, u8, 2, 4>;
impl R {
    #[doc = "Bits 0:1 - Trim input for Shield Delay block. Risk mitigation only; no test program or calpairs are needed for this"]
    #[inline(always)]
    pub fn delay_trim(&self) -> DELAY_TRIM_R {
        DELAY_TRIM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Hystersis input for Shield Delay block. Risk mitigation only; no test program or calpairs are needed for this"]
    #[inline(always)]
    pub fn delay_hys(&self) -> DELAY_HYS_R {
        DELAY_HYS_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Trim input for Shield Delay block. Risk mitigation only; no test program or calpairs are needed for this"]
    #[inline(always)]
    pub fn delay_trim(&mut self) -> DELAY_TRIM_W {
        DELAY_TRIM_W::new(self)
    }
    #[doc = "Bits 4:5 - Hystersis input for Shield Delay block. Risk mitigation only; no test program or calpairs are needed for this"]
    #[inline(always)]
    pub fn delay_hys(&mut self) -> DELAY_HYS_W {
        DELAY_HYS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trim control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trim_ctrl](index.html) module"]
pub struct TRIM_CTRL_SPEC;
impl crate::RegisterSpec for TRIM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trim_ctrl::R](R) reader structure"]
impl crate::Readable for TRIM_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trim_ctrl::W](W) writer structure"]
impl crate::Writable for TRIM_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRIM_CTRL to value 0x22"]
impl crate::Resettable for TRIM_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x22
    }
}
