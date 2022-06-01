#[doc = "Register `Cy_UART_FLOW_CTRL` reader"]
pub struct R(crate::R<CY_UART_FLOW_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CY_UART_FLOW_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CY_UART_FLOW_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CY_UART_FLOW_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Cy_UART_FLOW_CTRL` writer"]
pub struct W(crate::W<CY_UART_FLOW_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CY_UART_FLOW_CTRL_SPEC>;
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
impl From<crate::W<CY_UART_FLOW_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CY_UART_FLOW_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIGGER_LEVEL` reader - Trigger level. When the receiver FIFO has less entries than the amount of this field, a RTS signal is activated. Setting this field to 0 disables RTS generation."]
pub type TRIGGER_LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIGGER_LEVEL` writer - Trigger level. When the receiver FIFO has less entries than the amount of this field, a RTS signal is activated. Setting this field to 0 disables RTS generation."]
pub type TRIGGER_LEVEL_W<'a> = crate::FieldWriter<'a, u32, CY_UART_FLOW_CTRL_SPEC, u8, u8, 4, 0>;
#[doc = "Field `RTS_POLARITY` reader - Polarity of the RTS output signal"]
pub type RTS_POLARITY_R = crate::BitReader<bool>;
#[doc = "Field `RTS_POLARITY` writer - Polarity of the RTS output signal"]
pub type RTS_POLARITY_W<'a> = crate::BitWriter<'a, u32, CY_UART_FLOW_CTRL_SPEC, bool, 16>;
#[doc = "Field `CTS_POLARITY` reader - Polarity of the CTS input signal"]
pub type CTS_POLARITY_R = crate::BitReader<bool>;
#[doc = "Field `CTS_POLARITY` writer - Polarity of the CTS input signal"]
pub type CTS_POLARITY_W<'a> = crate::BitWriter<'a, u32, CY_UART_FLOW_CTRL_SPEC, bool, 24>;
#[doc = "Field `CTS_ENABLED` reader - Enable use of CTS input signal by the UART transmitter"]
pub type CTS_ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `CTS_ENABLED` writer - Enable use of CTS input signal by the UART transmitter"]
pub type CTS_ENABLED_W<'a> = crate::BitWriter<'a, u32, CY_UART_FLOW_CTRL_SPEC, bool, 25>;
impl R {
    #[doc = "Bits 0:3 - Trigger level. When the receiver FIFO has less entries than the amount of this field, a RTS signal is activated. Setting this field to 0 disables RTS generation."]
    #[inline(always)]
    pub fn trigger_level(&self) -> TRIGGER_LEVEL_R {
        TRIGGER_LEVEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Polarity of the RTS output signal"]
    #[inline(always)]
    pub fn rts_polarity(&self) -> RTS_POLARITY_R {
        RTS_POLARITY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Polarity of the CTS input signal"]
    #[inline(always)]
    pub fn cts_polarity(&self) -> CTS_POLARITY_R {
        CTS_POLARITY_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable use of CTS input signal by the UART transmitter"]
    #[inline(always)]
    pub fn cts_enabled(&self) -> CTS_ENABLED_R {
        CTS_ENABLED_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Trigger level. When the receiver FIFO has less entries than the amount of this field, a RTS signal is activated. Setting this field to 0 disables RTS generation."]
    #[inline(always)]
    pub fn trigger_level(&mut self) -> TRIGGER_LEVEL_W {
        TRIGGER_LEVEL_W::new(self)
    }
    #[doc = "Bit 16 - Polarity of the RTS output signal"]
    #[inline(always)]
    pub fn rts_polarity(&mut self) -> RTS_POLARITY_W {
        RTS_POLARITY_W::new(self)
    }
    #[doc = "Bit 24 - Polarity of the CTS input signal"]
    #[inline(always)]
    pub fn cts_polarity(&mut self) -> CTS_POLARITY_W {
        CTS_POLARITY_W::new(self)
    }
    #[doc = "Bit 25 - Enable use of CTS input signal by the UART transmitter"]
    #[inline(always)]
    pub fn cts_enabled(&mut self) -> CTS_ENABLED_W {
        CTS_ENABLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART flow control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cy_uart_flow_ctrl](index.html) module"]
pub struct CY_UART_FLOW_CTRL_SPEC;
impl crate::RegisterSpec for CY_UART_FLOW_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cy_uart_flow_ctrl::R](R) reader structure"]
impl crate::Readable for CY_UART_FLOW_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cy_uart_flow_ctrl::W](W) writer structure"]
impl crate::Writable for CY_UART_FLOW_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Cy_UART_FLOW_CTRL to value 0"]
impl crate::Resettable for CY_UART_FLOW_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
