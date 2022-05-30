#[doc = "Register `DMAC_DESCR1_PONG_SRC` reader"]
pub struct R(crate::R<DMAC_DESCR1_PONG_SRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_DESCR1_PONG_SRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_DESCR1_PONG_SRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_DESCR1_PONG_SRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAC_DESCR1_PONG_SRC` writer"]
pub struct W(crate::W<DMAC_DESCR1_PONG_SRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAC_DESCR1_PONG_SRC_SPEC>;
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
impl From<crate::W<DMAC_DESCR1_PONG_SRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAC_DESCR1_PONG_SRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRC` reader - Source address."]
pub type SRC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SRC` writer - Source address."]
pub type SRC_W<'a> = crate::FieldWriter<'a, u32, DMAC_DESCR1_PONG_SRC_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Source address."]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source address."]
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W {
        SRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Descriptor 1 source address location for channel 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_descr1_pong_src](index.html) module"]
pub struct DMAC_DESCR1_PONG_SRC_SPEC;
impl crate::RegisterSpec for DMAC_DESCR1_PONG_SRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_descr1_pong_src::R](R) reader structure"]
impl crate::Readable for DMAC_DESCR1_PONG_SRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmac_descr1_pong_src::W](W) writer structure"]
impl crate::Writable for DMAC_DESCR1_PONG_SRC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAC_DESCR1_PONG_SRC to value 0"]
impl crate::Resettable for DMAC_DESCR1_PONG_SRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
