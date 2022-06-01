#[doc = "Register `Cy_RX_FIFO_RD` reader"]
pub struct R(crate::R<CY_RX_FIFO_RD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CY_RX_FIFO_RD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CY_RX_FIFO_RD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CY_RX_FIFO_RD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Cy_RX_FIFO_RD` writer"]
pub struct W(crate::W<CY_RX_FIFO_RD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CY_RX_FIFO_RD_SPEC>;
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
impl From<crate::W<CY_RX_FIFO_RD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CY_RX_FIFO_RD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - Data read from the receiver FIFO. Reading a data frame will remove the data frame from the FIFO."]
pub type DATA_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data read from the receiver FIFO. Reading a data frame will remove the data frame from the FIFO."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
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
#[doc = "Receiver FIFO read register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cy_rx_fifo_rd](index.html) module"]
pub struct CY_RX_FIFO_RD_SPEC;
impl crate::RegisterSpec for CY_RX_FIFO_RD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cy_rx_fifo_rd::R](R) reader structure"]
impl crate::Readable for CY_RX_FIFO_RD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cy_rx_fifo_rd::W](W) writer structure"]
impl crate::Writable for CY_RX_FIFO_RD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Cy_RX_FIFO_RD to value 0"]
impl crate::Resettable for CY_RX_FIFO_RD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
