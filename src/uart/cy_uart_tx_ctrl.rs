#[doc = "Register `Cy_UART_TX_CTRL` reader"]
pub struct R(crate::R<CY_UART_TX_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CY_UART_TX_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CY_UART_TX_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CY_UART_TX_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Cy_UART_TX_CTRL` writer"]
pub struct W(crate::W<CY_UART_TX_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CY_UART_TX_CTRL_SPEC>;
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
impl From<crate::W<CY_UART_TX_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CY_UART_TX_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STOP_BTIS` reader - Stop bits. STOP_BITS + 1 is the duration of the stop period in terms of halve bit periods."]
pub type STOP_BTIS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STOP_BTIS` writer - Stop bits. STOP_BITS + 1 is the duration of the stop period in terms of halve bit periods."]
pub type STOP_BTIS_W<'a> = crate::FieldWriter<'a, u32, CY_UART_TX_CTRL_SPEC, u8, u8, 3, 0>;
#[doc = "Field `PARITY` reader - Parity bit. When '0', the transmitter generates an even parity. When '1', the transmitter generates an odd parity"]
pub type PARITY_R = crate::BitReader<bool>;
#[doc = "Field `PARITY` writer - Parity bit. When '0', the transmitter generates an even parity. When '1', the transmitter generates an odd parity"]
pub type PARITY_W<'a> = crate::BitWriter<'a, u32, CY_UART_TX_CTRL_SPEC, bool, 4>;
#[doc = "Field `PARITY_ENABLED` reader - Parity generation enabled ('1') or not ('0'). Only applicable in standard UART submodes. In SmartCard submode, parity generation is always enabled through hardware. In IrDA submode, parity generation is always disabled through hardware."]
pub type PARITY_ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `PARITY_ENABLED` writer - Parity generation enabled ('1') or not ('0'). Only applicable in standard UART submodes. In SmartCard submode, parity generation is always enabled through hardware. In IrDA submode, parity generation is always disabled through hardware."]
pub type PARITY_ENABLED_W<'a> = crate::BitWriter<'a, u32, CY_UART_TX_CTRL_SPEC, bool, 5>;
#[doc = "Field `RETRY_ON_NACK` reader - When '1', a data frame is retransmitted when a negative acknowledgement is received. Only applicable to the SmartCard submode."]
pub type RETRY_ON_NACK_R = crate::BitReader<bool>;
#[doc = "Field `RETRY_ON_NACK` writer - When '1', a data frame is retransmitted when a negative acknowledgement is received. Only applicable to the SmartCard submode."]
pub type RETRY_ON_NACK_W<'a> = crate::BitWriter<'a, u32, CY_UART_TX_CTRL_SPEC, bool, 8>;
impl R {
    #[doc = "Bits 0:2 - Stop bits. STOP_BITS + 1 is the duration of the stop period in terms of halve bit periods."]
    #[inline(always)]
    pub fn stop_btis(&self) -> STOP_BTIS_R {
        STOP_BTIS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Parity bit. When '0', the transmitter generates an even parity. When '1', the transmitter generates an odd parity"]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Parity generation enabled ('1') or not ('0'). Only applicable in standard UART submodes. In SmartCard submode, parity generation is always enabled through hardware. In IrDA submode, parity generation is always disabled through hardware."]
    #[inline(always)]
    pub fn parity_enabled(&self) -> PARITY_ENABLED_R {
        PARITY_ENABLED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - When '1', a data frame is retransmitted when a negative acknowledgement is received. Only applicable to the SmartCard submode."]
    #[inline(always)]
    pub fn retry_on_nack(&self) -> RETRY_ON_NACK_R {
        RETRY_ON_NACK_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Stop bits. STOP_BITS + 1 is the duration of the stop period in terms of halve bit periods."]
    #[inline(always)]
    pub fn stop_btis(&mut self) -> STOP_BTIS_W {
        STOP_BTIS_W::new(self)
    }
    #[doc = "Bit 4 - Parity bit. When '0', the transmitter generates an even parity. When '1', the transmitter generates an odd parity"]
    #[inline(always)]
    pub fn parity(&mut self) -> PARITY_W {
        PARITY_W::new(self)
    }
    #[doc = "Bit 5 - Parity generation enabled ('1') or not ('0'). Only applicable in standard UART submodes. In SmartCard submode, parity generation is always enabled through hardware. In IrDA submode, parity generation is always disabled through hardware."]
    #[inline(always)]
    pub fn parity_enabled(&mut self) -> PARITY_ENABLED_W {
        PARITY_ENABLED_W::new(self)
    }
    #[doc = "Bit 8 - When '1', a data frame is retransmitted when a negative acknowledgement is received. Only applicable to the SmartCard submode."]
    #[inline(always)]
    pub fn retry_on_nack(&mut self) -> RETRY_ON_NACK_W {
        RETRY_ON_NACK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART transmitter control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cy_uart_tx_ctrl](index.html) module"]
pub struct CY_UART_TX_CTRL_SPEC;
impl crate::RegisterSpec for CY_UART_TX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cy_uart_tx_ctrl::R](R) reader structure"]
impl crate::Readable for CY_UART_TX_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cy_uart_tx_ctrl::W](W) writer structure"]
impl crate::Writable for CY_UART_TX_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Cy_UART_TX_CTRL to value 0"]
impl crate::Resettable for CY_UART_TX_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
