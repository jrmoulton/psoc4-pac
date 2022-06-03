#[doc = "Register `RAM_CTL` reader"]
pub struct R(crate::R<RAM_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAM_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAM_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAM_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAM_CTL` writer"]
pub struct W(crate::W<RAM_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAM_CTL_SPEC>;
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
impl From<crate::W<RAM_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAM_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARB` reader - Arbitration policy: '0': CPU has priority '1': DW/DMA has priority '2': Roundrobin '3': Roundrobin - sticky"]
pub type ARB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ARB` writer - Arbitration policy: '0': CPU has priority '1': DW/DMA has priority '2': Roundrobin '3': Roundrobin - sticky"]
pub type ARB_W<'a> = crate::FieldWriter<'a, u32, RAM_CTL_SPEC, u8, u8, 2, 16>;
impl R {
    #[doc = "Bits 16:17 - Arbitration policy: '0': CPU has priority '1': DW/DMA has priority '2': Roundrobin '3': Roundrobin - sticky"]
    #[inline(always)]
    pub fn arb(&self) -> ARB_R {
        ARB_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 16:17 - Arbitration policy: '0': CPU has priority '1': DW/DMA has priority '2': Roundrobin '3': Roundrobin - sticky"]
    #[inline(always)]
    pub fn arb(&mut self) -> ARB_W {
        ARB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAM control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram_ctl](index.html) module"]
pub struct RAM_CTL_SPEC;
impl crate::RegisterSpec for RAM_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ram_ctl::R](R) reader structure"]
impl crate::Readable for RAM_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ram_ctl::W](W) writer structure"]
impl crate::Writable for RAM_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RAM_CTL to value 0"]
impl crate::Resettable for RAM_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
