#[doc = "Register `FLASH_CTL` reader"]
pub struct R(crate::R<FLASH_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_CTL` writer"]
pub struct W(crate::W<FLASH_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_CTL_SPEC>;
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
impl From<crate::W<FLASH_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_WS` reader - Amount of ROM wait states: '0': 0 wait states (fast flash: \\[0, 24\\]
MHz system frequency, slow flash: \\[0, 16\\]
MHz system frequency) '1': 1 wait state (fast flash: \\[24, 48\\]
MHz system frequency, slow flash: \\[16, 32\\]
MHz system frequency) '2': 2 wait states (slow flash: \\[32, 48\\]
MHz system frequency) '3': 3 wait states (can be used to give more time for flash access if 2 wait states are not sufficient)"]
pub type FLASH_WS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLASH_WS` writer - Amount of ROM wait states: '0': 0 wait states (fast flash: \\[0, 24\\]
MHz system frequency, slow flash: \\[0, 16\\]
MHz system frequency) '1': 1 wait state (fast flash: \\[24, 48\\]
MHz system frequency, slow flash: \\[16, 32\\]
MHz system frequency) '2': 2 wait states (slow flash: \\[32, 48\\]
MHz system frequency) '3': 3 wait states (can be used to give more time for flash access if 2 wait states are not sufficient)"]
pub type FLASH_WS_W<'a> = crate::FieldWriter<'a, u32, FLASH_CTL_SPEC, u8, u8, 2, 0>;
#[doc = "Field `PREF_EN` reader - Prefetch enable: '0': disabled. This is a desirable seeting when FLASH_WS is '0' or when predictable execution behavior is required. '1': enabled."]
pub type PREF_EN_R = crate::BitReader<bool>;
#[doc = "Field `PREF_EN` writer - Prefetch enable: '0': disabled. This is a desirable seeting when FLASH_WS is '0' or when predictable execution behavior is required. '1': enabled."]
pub type PREF_EN_W<'a> = crate::BitWriter<'a, u32, FLASH_CTL_SPEC, bool, 4>;
#[doc = "Field `FLASH_INVALIDATE` reader - 1': Invalidates the content of the flash controller's buffers."]
pub type FLASH_INVALIDATE_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_INVALIDATE` writer - 1': Invalidates the content of the flash controller's buffers."]
pub type FLASH_INVALIDATE_W<'a> = crate::BitWriter<'a, u32, FLASH_CTL_SPEC, bool, 8>;
#[doc = "Field `ARB` reader - Arbitration policy: '0': CPU has priority '1': DW/DMA has priority '2': Roundrobin '3': Roundrobin - sticky"]
pub type ARB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ARB` writer - Arbitration policy: '0': CPU has priority '1': DW/DMA has priority '2': Roundrobin '3': Roundrobin - sticky"]
pub type ARB_W<'a> = crate::FieldWriter<'a, u32, FLASH_CTL_SPEC, u8, u8, 2, 16>;
impl R {
    #[doc = "Bits 0:1 - Amount of ROM wait states: '0': 0 wait states (fast flash: \\[0, 24\\]
MHz system frequency, slow flash: \\[0, 16\\]
MHz system frequency) '1': 1 wait state (fast flash: \\[24, 48\\]
MHz system frequency, slow flash: \\[16, 32\\]
MHz system frequency) '2': 2 wait states (slow flash: \\[32, 48\\]
MHz system frequency) '3': 3 wait states (can be used to give more time for flash access if 2 wait states are not sufficient)"]
    #[inline(always)]
    pub fn flash_ws(&self) -> FLASH_WS_R {
        FLASH_WS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Prefetch enable: '0': disabled. This is a desirable seeting when FLASH_WS is '0' or when predictable execution behavior is required. '1': enabled."]
    #[inline(always)]
    pub fn pref_en(&self) -> PREF_EN_R {
        PREF_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 1': Invalidates the content of the flash controller's buffers."]
    #[inline(always)]
    pub fn flash_invalidate(&self) -> FLASH_INVALIDATE_R {
        FLASH_INVALIDATE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Arbitration policy: '0': CPU has priority '1': DW/DMA has priority '2': Roundrobin '3': Roundrobin - sticky"]
    #[inline(always)]
    pub fn arb(&self) -> ARB_R {
        ARB_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Amount of ROM wait states: '0': 0 wait states (fast flash: \\[0, 24\\]
MHz system frequency, slow flash: \\[0, 16\\]
MHz system frequency) '1': 1 wait state (fast flash: \\[24, 48\\]
MHz system frequency, slow flash: \\[16, 32\\]
MHz system frequency) '2': 2 wait states (slow flash: \\[32, 48\\]
MHz system frequency) '3': 3 wait states (can be used to give more time for flash access if 2 wait states are not sufficient)"]
    #[inline(always)]
    pub fn flash_ws(&mut self) -> FLASH_WS_W {
        FLASH_WS_W::new(self)
    }
    #[doc = "Bit 4 - Prefetch enable: '0': disabled. This is a desirable seeting when FLASH_WS is '0' or when predictable execution behavior is required. '1': enabled."]
    #[inline(always)]
    pub fn pref_en(&mut self) -> PREF_EN_W {
        PREF_EN_W::new(self)
    }
    #[doc = "Bit 8 - 1': Invalidates the content of the flash controller's buffers."]
    #[inline(always)]
    pub fn flash_invalidate(&mut self) -> FLASH_INVALIDATE_W {
        FLASH_INVALIDATE_W::new(self)
    }
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
#[doc = "FLASH control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_ctl](index.html) module"]
pub struct FLASH_CTL_SPEC;
impl crate::RegisterSpec for FLASH_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_ctl::R](R) reader structure"]
impl crate::Readable for FLASH_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_ctl::W](W) writer structure"]
impl crate::Writable for FLASH_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASH_CTL to value 0"]
impl crate::Resettable for FLASH_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
