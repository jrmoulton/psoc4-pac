#[doc = "Register `Cy_SPI_STATUS` reader"]
pub struct R(crate::R<CY_SPI_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CY_SPI_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CY_SPI_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CY_SPI_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Cy_SPI_STATUS` writer"]
pub struct W(crate::W<CY_SPI_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CY_SPI_STATUS_SPEC>;
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
impl From<crate::W<CY_SPI_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CY_SPI_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUS_BUSY` reader - SPI bus is busy. The bus is considered busy ('1') during an ongoing transaction"]
pub type BUS_BUSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - SPI bus is busy. The bus is considered busy ('1') during an ongoing transaction"]
    #[inline(always)]
    pub fn bus_busy(&self) -> BUS_BUSY_R {
        BUS_BUSY_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cy_spi_status](index.html) module"]
pub struct CY_SPI_STATUS_SPEC;
impl crate::RegisterSpec for CY_SPI_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cy_spi_status::R](R) reader structure"]
impl crate::Readable for CY_SPI_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cy_spi_status::W](W) writer structure"]
impl crate::Writable for CY_SPI_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Cy_SPI_STATUS to value 0"]
impl crate::Resettable for CY_SPI_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
