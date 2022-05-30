#[doc = "Register `DMAC_CTL` reader"]
pub struct R(crate::R<DMAC_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAC_CTL` writer"]
pub struct W(crate::W<DMAC_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAC_CTL_SPEC>;
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
impl From<crate::W<DMAC_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAC_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Global DMAC enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLED_A {
    #[doc = "0: DMA controller is disabled."]
    DISABLED = 0,
    #[doc = "1: DMA controller is enabled."]
    ENABLED = 1,
}
impl From<ENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLED` reader - Global DMAC enable"]
pub type ENABLED_R = crate::BitReader<ENABLED_A>;
impl ENABLED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLED_A {
        match self.bits {
            false => ENABLED_A::DISABLED,
            true => ENABLED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLED_A::ENABLED
    }
}
#[doc = "Field `ENABLED` writer - Global DMAC enable"]
pub type ENABLED_W<'a> = crate::BitWriter<'a, u32, DMAC_CTL_SPEC, ENABLED_A, 31>;
impl<'a> ENABLED_W<'a> {
    #[doc = "DMA controller is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLED_A::DISABLED)
    }
    #[doc = "DMA controller is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLED_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 31 - Global DMAC enable"]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Global DMAC enable"]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W {
        ENABLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA controller control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_ctl](index.html) module"]
pub struct DMAC_CTL_SPEC;
impl crate::RegisterSpec for DMAC_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_ctl::R](R) reader structure"]
impl crate::Readable for DMAC_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmac_ctl::W](W) writer structure"]
impl crate::Writable for DMAC_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAC_CTL to value 0"]
impl crate::Resettable for DMAC_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
