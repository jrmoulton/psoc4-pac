#[doc = "Register `Cy_INTR_INTR_SPI_EC_MASK` reader"]
pub struct R(crate::R<CY_INTR_INTR_SPI_EC_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CY_INTR_INTR_SPI_EC_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CY_INTR_INTR_SPI_EC_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CY_INTR_INTR_SPI_EC_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Cy_INTR_INTR_SPI_EC_MASK` writer"]
pub struct W(crate::W<CY_INTR_INTR_SPI_EC_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CY_INTR_INTR_SPI_EC_MASK_SPEC>;
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
impl From<crate::W<CY_INTR_INTR_SPI_EC_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CY_INTR_INTR_SPI_EC_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAKE_UP` reader - Mask bit for corresponding bit in interrupt request register."]
pub type WAKE_UP_R = crate::BitReader<bool>;
#[doc = "Field `WAKE_UP` writer - Mask bit for corresponding bit in interrupt request register."]
pub type WAKE_UP_W<'a> = crate::BitWriter<'a, u32, CY_INTR_INTR_SPI_EC_MASK_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn wake_up(&self) -> WAKE_UP_R {
        WAKE_UP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn wake_up(&mut self) -> WAKE_UP_W {
        WAKE_UP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Externally clocked SPI interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cy_intr_intr_spi_ec_mask](index.html) module"]
pub struct CY_INTR_INTR_SPI_EC_MASK_SPEC;
impl crate::RegisterSpec for CY_INTR_INTR_SPI_EC_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cy_intr_intr_spi_ec_mask::R](R) reader structure"]
impl crate::Readable for CY_INTR_INTR_SPI_EC_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cy_intr_intr_spi_ec_mask::W](W) writer structure"]
impl crate::Writable for CY_INTR_INTR_SPI_EC_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Cy_INTR_INTR_SPI_EC_MASK to value 0"]
impl crate::Resettable for CY_INTR_INTR_SPI_EC_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
