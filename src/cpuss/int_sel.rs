#[doc = "Register `INT_SEL` reader"]
pub struct R(crate::R<INT_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_SEL` writer"]
pub struct W(crate::W<INT_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_SEL_SPEC>;
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
impl From<crate::W<INT_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSI` reader - Specifies interrupt source: '0': Fixed Function. '1': DSI. When changing the source of a specific interrupt, it is advised to temporarily disable the interrupt using the CM0 NVIC's CLRENA and SETENA interrupt enable clear and set registers to prevent a spurious interrupt activation. In addition, the CM0 NVIC's CLRPEND interrupt pending clear register should be used clear a pending interrupt before re-enabling the interrupt."]
pub type DSI_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DSI` writer - Specifies interrupt source: '0': Fixed Function. '1': DSI. When changing the source of a specific interrupt, it is advised to temporarily disable the interrupt using the CM0 NVIC's CLRENA and SETENA interrupt enable clear and set registers to prevent a spurious interrupt activation. In addition, the CM0 NVIC's CLRPEND interrupt pending clear register should be used clear a pending interrupt before re-enabling the interrupt."]
pub type DSI_W<'a> = crate::FieldWriter<'a, u32, INT_SEL_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Specifies interrupt source: '0': Fixed Function. '1': DSI. When changing the source of a specific interrupt, it is advised to temporarily disable the interrupt using the CM0 NVIC's CLRENA and SETENA interrupt enable clear and set registers to prevent a spurious interrupt activation. In addition, the CM0 NVIC's CLRPEND interrupt pending clear register should be used clear a pending interrupt before re-enabling the interrupt."]
    #[inline(always)]
    pub fn dsi(&self) -> DSI_R {
        DSI_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies interrupt source: '0': Fixed Function. '1': DSI. When changing the source of a specific interrupt, it is advised to temporarily disable the interrupt using the CM0 NVIC's CLRENA and SETENA interrupt enable clear and set registers to prevent a spurious interrupt activation. In addition, the CM0 NVIC's CLRPEND interrupt pending clear register should be used clear a pending interrupt before re-enabling the interrupt."]
    #[inline(always)]
    pub fn dsi(&mut self) -> DSI_W {
        DSI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt multiplexer select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_sel](index.html) module"]
pub struct INT_SEL_SPEC;
impl crate::RegisterSpec for INT_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_sel::R](R) reader structure"]
impl crate::Readable for INT_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_sel::W](W) writer structure"]
impl crate::Writable for INT_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_SEL to value 0"]
impl crate::Resettable for INT_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
