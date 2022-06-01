#[doc = "Register `Cy_UART_RX_CTRL` reader"]
pub struct R(crate::R<CY_UART_RX_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CY_UART_RX_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CY_UART_RX_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CY_UART_RX_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Cy_UART_RX_CTRL` writer"]
pub struct W(crate::W<CY_UART_RX_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CY_UART_RX_CTRL_SPEC>;
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
impl From<crate::W<CY_UART_RX_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CY_UART_RX_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STOP_BITS` reader - Stop bits. STOP_BITS + 1 is the duration of the stop period in terms of halve bit periods."]
pub type STOP_BITS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STOP_BITS` writer - Stop bits. STOP_BITS + 1 is the duration of the stop period in terms of halve bit periods."]
pub type STOP_BITS_W<'a> = crate::FieldWriter<'a, u32, CY_UART_RX_CTRL_SPEC, u8, u8, 3, 0>;
#[doc = "Field `PARITY` reader - Parity bit. When '0', the receiver expects an even parity. When '1', the receiver expects an odd parity."]
pub type PARITY_R = crate::BitReader<bool>;
#[doc = "Field `PARITY` writer - Parity bit. When '0', the receiver expects an even parity. When '1', the receiver expects an odd parity."]
pub type PARITY_W<'a> = crate::BitWriter<'a, u32, CY_UART_RX_CTRL_SPEC, bool, 4>;
#[doc = "Field `PARITY_ENABLED` reader - Parity checking enabled ('1') or not ('0'). Only applicable in standard UART submode. In SmartCard submode, parity checking is always enabled through hardware. In IrDA submode, parity checking is always disabled through hardware."]
pub type PARITY_ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `PARITY_ENABLED` writer - Parity checking enabled ('1') or not ('0'). Only applicable in standard UART submode. In SmartCard submode, parity checking is always enabled through hardware. In IrDA submode, parity checking is always disabled through hardware."]
pub type PARITY_ENABLED_W<'a> = crate::BitWriter<'a, u32, CY_UART_RX_CTRL_SPEC, bool, 5>;
#[doc = "Field `POLARITY` reader - Inverts incoming RX line signal. Inversion is after local loopback. This functionality is intended for IrDA receiver functionality."]
pub type POLARITY_R = crate::BitReader<bool>;
#[doc = "Field `POLARITY` writer - Inverts incoming RX line signal. Inversion is after local loopback. This functionality is intended for IrDA receiver functionality."]
pub type POLARITY_W<'a> = crate::BitWriter<'a, u32, CY_UART_RX_CTRL_SPEC, bool, 6>;
#[doc = "Field `DROP_ON_PARITY_ERR` reader - Behaviour when a parity check fails. When '0', received data is send to the RX FIFO. When '1', received data is dropped and lost."]
pub type DROP_ON_PARITY_ERR_R = crate::BitReader<bool>;
#[doc = "Field `DROP_ON_PARITY_ERR` writer - Behaviour when a parity check fails. When '0', received data is send to the RX FIFO. When '1', received data is dropped and lost."]
pub type DROP_ON_PARITY_ERR_W<'a> = crate::BitWriter<'a, u32, CY_UART_RX_CTRL_SPEC, bool, 8>;
#[doc = "Field `DROP_ON_FRAME_ERR` reader - Behaviour when an error is detected in a start or stop period. When '0', received data is send to the RX FIFO. When '1', received data is dropped and lost."]
pub type DROP_ON_FRAME_ERR_R = crate::BitReader<bool>;
#[doc = "Field `DROP_ON_FRAME_ERR` writer - Behaviour when an error is detected in a start or stop period. When '0', received data is send to the RX FIFO. When '1', received data is dropped and lost."]
pub type DROP_ON_FRAME_ERR_W<'a> = crate::BitWriter<'a, u32, CY_UART_RX_CTRL_SPEC, bool, 9>;
#[doc = "Field `MP_MODE` reader - Multi-processor mode. When '1', multi-processor mode is enabled. In this mode, RX_CTRL.DATA_WIDTH should indicate a 9-bit data frame."]
pub type MP_MODE_R = crate::BitReader<bool>;
#[doc = "Field `MP_MODE` writer - Multi-processor mode. When '1', multi-processor mode is enabled. In this mode, RX_CTRL.DATA_WIDTH should indicate a 9-bit data frame."]
pub type MP_MODE_W<'a> = crate::BitWriter<'a, u32, CY_UART_RX_CTRL_SPEC, bool, 10>;
#[doc = "Field `LIN_MODE` reader - Only applicable in standard UART submode. When '1', the receiver performs break detection and baud rate detection on the incoming data"]
pub type LIN_MODE_R = crate::BitReader<bool>;
#[doc = "Field `LIN_MODE` writer - Only applicable in standard UART submode. When '1', the receiver performs break detection and baud rate detection on the incoming data"]
pub type LIN_MODE_W<'a> = crate::BitWriter<'a, u32, CY_UART_RX_CTRL_SPEC, bool, 12>;
#[doc = "Field `SKIP_START` reader - Only applicable in standard UART submode. When '1', the receiver skips start bit detection for the first received data frame. Instead, it synchronizes on the first received data frame bit, which should be a '1'."]
pub type SKIP_START_R = crate::BitReader<bool>;
#[doc = "Field `SKIP_START` writer - Only applicable in standard UART submode. When '1', the receiver skips start bit detection for the first received data frame. Instead, it synchronizes on the first received data frame bit, which should be a '1'."]
pub type SKIP_START_W<'a> = crate::BitWriter<'a, u32, CY_UART_RX_CTRL_SPEC, bool, 13>;
#[doc = "Field `BREAK_WIDTH` reader - Break width. BREAK_WIDTH + 1 is the minimum width in bit periods of a break."]
pub type BREAK_WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BREAK_WIDTH` writer - Break width. BREAK_WIDTH + 1 is the minimum width in bit periods of a break."]
pub type BREAK_WIDTH_W<'a> = crate::FieldWriter<'a, u32, CY_UART_RX_CTRL_SPEC, u8, u8, 4, 16>;
impl R {
    #[doc = "Bits 0:2 - Stop bits. STOP_BITS + 1 is the duration of the stop period in terms of halve bit periods."]
    #[inline(always)]
    pub fn stop_bits(&self) -> STOP_BITS_R {
        STOP_BITS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Parity bit. When '0', the receiver expects an even parity. When '1', the receiver expects an odd parity."]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Parity checking enabled ('1') or not ('0'). Only applicable in standard UART submode. In SmartCard submode, parity checking is always enabled through hardware. In IrDA submode, parity checking is always disabled through hardware."]
    #[inline(always)]
    pub fn parity_enabled(&self) -> PARITY_ENABLED_R {
        PARITY_ENABLED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Inverts incoming RX line signal. Inversion is after local loopback. This functionality is intended for IrDA receiver functionality."]
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Behaviour when a parity check fails. When '0', received data is send to the RX FIFO. When '1', received data is dropped and lost."]
    #[inline(always)]
    pub fn drop_on_parity_err(&self) -> DROP_ON_PARITY_ERR_R {
        DROP_ON_PARITY_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Behaviour when an error is detected in a start or stop period. When '0', received data is send to the RX FIFO. When '1', received data is dropped and lost."]
    #[inline(always)]
    pub fn drop_on_frame_err(&self) -> DROP_ON_FRAME_ERR_R {
        DROP_ON_FRAME_ERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Multi-processor mode. When '1', multi-processor mode is enabled. In this mode, RX_CTRL.DATA_WIDTH should indicate a 9-bit data frame."]
    #[inline(always)]
    pub fn mp_mode(&self) -> MP_MODE_R {
        MP_MODE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Only applicable in standard UART submode. When '1', the receiver performs break detection and baud rate detection on the incoming data"]
    #[inline(always)]
    pub fn lin_mode(&self) -> LIN_MODE_R {
        LIN_MODE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Only applicable in standard UART submode. When '1', the receiver skips start bit detection for the first received data frame. Instead, it synchronizes on the first received data frame bit, which should be a '1'."]
    #[inline(always)]
    pub fn skip_start(&self) -> SKIP_START_R {
        SKIP_START_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Break width. BREAK_WIDTH + 1 is the minimum width in bit periods of a break."]
    #[inline(always)]
    pub fn break_width(&self) -> BREAK_WIDTH_R {
        BREAK_WIDTH_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Stop bits. STOP_BITS + 1 is the duration of the stop period in terms of halve bit periods."]
    #[inline(always)]
    pub fn stop_bits(&mut self) -> STOP_BITS_W {
        STOP_BITS_W::new(self)
    }
    #[doc = "Bit 4 - Parity bit. When '0', the receiver expects an even parity. When '1', the receiver expects an odd parity."]
    #[inline(always)]
    pub fn parity(&mut self) -> PARITY_W {
        PARITY_W::new(self)
    }
    #[doc = "Bit 5 - Parity checking enabled ('1') or not ('0'). Only applicable in standard UART submode. In SmartCard submode, parity checking is always enabled through hardware. In IrDA submode, parity checking is always disabled through hardware."]
    #[inline(always)]
    pub fn parity_enabled(&mut self) -> PARITY_ENABLED_W {
        PARITY_ENABLED_W::new(self)
    }
    #[doc = "Bit 6 - Inverts incoming RX line signal. Inversion is after local loopback. This functionality is intended for IrDA receiver functionality."]
    #[inline(always)]
    pub fn polarity(&mut self) -> POLARITY_W {
        POLARITY_W::new(self)
    }
    #[doc = "Bit 8 - Behaviour when a parity check fails. When '0', received data is send to the RX FIFO. When '1', received data is dropped and lost."]
    #[inline(always)]
    pub fn drop_on_parity_err(&mut self) -> DROP_ON_PARITY_ERR_W {
        DROP_ON_PARITY_ERR_W::new(self)
    }
    #[doc = "Bit 9 - Behaviour when an error is detected in a start or stop period. When '0', received data is send to the RX FIFO. When '1', received data is dropped and lost."]
    #[inline(always)]
    pub fn drop_on_frame_err(&mut self) -> DROP_ON_FRAME_ERR_W {
        DROP_ON_FRAME_ERR_W::new(self)
    }
    #[doc = "Bit 10 - Multi-processor mode. When '1', multi-processor mode is enabled. In this mode, RX_CTRL.DATA_WIDTH should indicate a 9-bit data frame."]
    #[inline(always)]
    pub fn mp_mode(&mut self) -> MP_MODE_W {
        MP_MODE_W::new(self)
    }
    #[doc = "Bit 12 - Only applicable in standard UART submode. When '1', the receiver performs break detection and baud rate detection on the incoming data"]
    #[inline(always)]
    pub fn lin_mode(&mut self) -> LIN_MODE_W {
        LIN_MODE_W::new(self)
    }
    #[doc = "Bit 13 - Only applicable in standard UART submode. When '1', the receiver skips start bit detection for the first received data frame. Instead, it synchronizes on the first received data frame bit, which should be a '1'."]
    #[inline(always)]
    pub fn skip_start(&mut self) -> SKIP_START_W {
        SKIP_START_W::new(self)
    }
    #[doc = "Bits 16:19 - Break width. BREAK_WIDTH + 1 is the minimum width in bit periods of a break."]
    #[inline(always)]
    pub fn break_width(&mut self) -> BREAK_WIDTH_W {
        BREAK_WIDTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART receiver control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cy_uart_rx_ctrl](index.html) module"]
pub struct CY_UART_RX_CTRL_SPEC;
impl crate::RegisterSpec for CY_UART_RX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cy_uart_rx_ctrl::R](R) reader structure"]
impl crate::Readable for CY_UART_RX_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cy_uart_rx_ctrl::W](W) writer structure"]
impl crate::Writable for CY_UART_RX_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Cy_UART_RX_CTRL to value 0"]
impl crate::Resettable for CY_UART_RX_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
