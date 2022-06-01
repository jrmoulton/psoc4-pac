#[doc = "Register `Cy_TX_FIFO_WR` reader"]
pub struct R(crate::R<CY_TX_FIFO_WR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CY_TX_FIFO_WR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CY_TX_FIFO_WR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CY_TX_FIFO_WR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Cy_TX_FIFO_WR` writer"]
pub struct W(crate::W<CY_TX_FIFO_WR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CY_TX_FIFO_WR_SPEC>;
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
impl From<crate::W<CY_TX_FIFO_WR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CY_TX_FIFO_WR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` writer - Data frame written into the transmitter FIFO. Behavior is similar to that of a PUSH operation."]
pub type DATA_W<'a> = crate::FieldWriter<'a, u32, CY_TX_FIFO_WR_SPEC, u16, u16, 16, 0>;
impl W {
    #[doc = "Bits 0:15 - Data frame written into the transmitter FIFO. Behavior is similar to that of a PUSH operation."]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmitter FIFO write register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cy_tx_fifo_wr](index.html) module"]
pub struct CY_TX_FIFO_WR_SPEC;
impl crate::RegisterSpec for CY_TX_FIFO_WR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cy_tx_fifo_wr::R](R) reader structure"]
impl crate::Readable for CY_TX_FIFO_WR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cy_tx_fifo_wr::W](W) writer structure"]
impl crate::Writable for CY_TX_FIFO_WR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Cy_TX_FIFO_WR to value 0"]
impl crate::Resettable for CY_TX_FIFO_WR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
