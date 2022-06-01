#[doc = "Register `Cy_INTR_TX_MASK` reader"]
pub struct R(crate::R<CY_INTR_TX_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CY_INTR_TX_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CY_INTR_TX_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CY_INTR_TX_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Cy_INTR_TX_MASK` writer"]
pub struct W(crate::W<CY_INTR_TX_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CY_INTR_TX_MASK_SPEC>;
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
impl From<crate::W<CY_INTR_TX_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CY_INTR_TX_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIGGER` reader - Mask bit for corresponding bit in interrupt request register."]
pub type TRIGGER_R = crate::BitReader<bool>;
#[doc = "Field `TRIGGER` writer - Mask bit for corresponding bit in interrupt request register."]
pub type TRIGGER_W<'a> = crate::BitWriter<'a, u32, CY_INTR_TX_MASK_SPEC, bool, 0>;
#[doc = "Field `NOT_FULL` reader - Mask bit for corresponding bit in interrupt request register."]
pub type NOT_FULL_R = crate::BitReader<bool>;
#[doc = "Field `NOT_FULL` writer - Mask bit for corresponding bit in interrupt request register."]
pub type NOT_FULL_W<'a> = crate::BitWriter<'a, u32, CY_INTR_TX_MASK_SPEC, bool, 1>;
#[doc = "Field `EMPTY` reader - Mask bit for corresponding bit in interrupt request register."]
pub type EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `EMPTY` writer - Mask bit for corresponding bit in interrupt request register."]
pub type EMPTY_W<'a> = crate::BitWriter<'a, u32, CY_INTR_TX_MASK_SPEC, bool, 4>;
#[doc = "Field `OVERFLOW` reader - Mask bit for corresponding bit in interrupt request register."]
pub type OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `OVERFLOW` writer - Mask bit for corresponding bit in interrupt request register."]
pub type OVERFLOW_W<'a> = crate::BitWriter<'a, u32, CY_INTR_TX_MASK_SPEC, bool, 5>;
#[doc = "Field `UNDERFLOW` reader - Mask bit for corresponding bit in interrupt request register."]
pub type UNDERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `UNDERFLOW` writer - Mask bit for corresponding bit in interrupt request register."]
pub type UNDERFLOW_W<'a> = crate::BitWriter<'a, u32, CY_INTR_TX_MASK_SPEC, bool, 6>;
#[doc = "Field `UART_NACK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type UART_NACK_R = crate::BitReader<bool>;
#[doc = "Field `UART_NACK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type UART_NACK_W<'a> = crate::BitWriter<'a, u32, CY_INTR_TX_MASK_SPEC, bool, 8>;
#[doc = "Field `UART_DONE` reader - Mask bit for corresponding bit in interrupt request register."]
pub type UART_DONE_R = crate::BitReader<bool>;
#[doc = "Field `UART_DONE` writer - Mask bit for corresponding bit in interrupt request register."]
pub type UART_DONE_W<'a> = crate::BitWriter<'a, u32, CY_INTR_TX_MASK_SPEC, bool, 9>;
#[doc = "Field `UART_ARB_LOST` reader - Mask bit for corresponding bit in interrupt request register."]
pub type UART_ARB_LOST_R = crate::BitReader<bool>;
#[doc = "Field `UART_ARB_LOST` writer - Mask bit for corresponding bit in interrupt request register."]
pub type UART_ARB_LOST_W<'a> = crate::BitWriter<'a, u32, CY_INTR_TX_MASK_SPEC, bool, 10>;
impl R {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn trigger(&self) -> TRIGGER_R {
        TRIGGER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn not_full(&self) -> NOT_FULL_R {
        NOT_FULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn empty(&self) -> EMPTY_R {
        EMPTY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn overflow(&self) -> OVERFLOW_R {
        OVERFLOW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn underflow(&self) -> UNDERFLOW_R {
        UNDERFLOW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn uart_nack(&self) -> UART_NACK_R {
        UART_NACK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn uart_done(&self) -> UART_DONE_R {
        UART_DONE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn uart_arb_lost(&self) -> UART_ARB_LOST_R {
        UART_ARB_LOST_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn trigger(&mut self) -> TRIGGER_W {
        TRIGGER_W::new(self)
    }
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn not_full(&mut self) -> NOT_FULL_W {
        NOT_FULL_W::new(self)
    }
    #[doc = "Bit 4 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn empty(&mut self) -> EMPTY_W {
        EMPTY_W::new(self)
    }
    #[doc = "Bit 5 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn overflow(&mut self) -> OVERFLOW_W {
        OVERFLOW_W::new(self)
    }
    #[doc = "Bit 6 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn underflow(&mut self) -> UNDERFLOW_W {
        UNDERFLOW_W::new(self)
    }
    #[doc = "Bit 8 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn uart_nack(&mut self) -> UART_NACK_W {
        UART_NACK_W::new(self)
    }
    #[doc = "Bit 9 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn uart_done(&mut self) -> UART_DONE_W {
        UART_DONE_W::new(self)
    }
    #[doc = "Bit 10 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn uart_arb_lost(&mut self) -> UART_ARB_LOST_W {
        UART_ARB_LOST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmitter interrupt mask request register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cy_intr_tx_mask](index.html) module"]
pub struct CY_INTR_TX_MASK_SPEC;
impl crate::RegisterSpec for CY_INTR_TX_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cy_intr_tx_mask::R](R) reader structure"]
impl crate::Readable for CY_INTR_TX_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cy_intr_tx_mask::W](W) writer structure"]
impl crate::Writable for CY_INTR_TX_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Cy_INTR_TX_MASK to value 0"]
impl crate::Resettable for CY_INTR_TX_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
