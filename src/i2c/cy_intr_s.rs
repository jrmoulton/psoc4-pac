#[doc = "Register `Cy_INTR_S` reader"]
pub struct R(crate::R<CY_INTR_S_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CY_INTR_S_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CY_INTR_S_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CY_INTR_S_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Cy_INTR_S` writer"]
pub struct W(crate::W<CY_INTR_S_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CY_INTR_S_SPEC>;
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
impl From<crate::W<CY_INTR_S_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CY_INTR_S_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_ARB_LOST` reader - I2C slave lost arbitration: the value driven on the SDA line is not the same as the value observed on the SDA line (while the SCL line is '1')."]
pub type I2C_ARB_LOST_R = crate::BitReader<bool>;
#[doc = "Field `I2C_ARB_LOST` writer - I2C slave lost arbitration: the value driven on the SDA line is not the same as the value observed on the SDA line (while the SCL line is '1')."]
pub type I2C_ARB_LOST_W<'a> = crate::BitWriter<'a, u32, CY_INTR_S_SPEC, bool, 0>;
#[doc = "Field `I2C_NACK` reader - I2C slave negative acknowledgement received. Set to '1', when the slave receives a NACK (typically after the slave transmitted TX data)."]
pub type I2C_NACK_R = crate::BitReader<bool>;
#[doc = "Field `I2C_NACK` writer - I2C slave negative acknowledgement received. Set to '1', when the slave receives a NACK (typically after the slave transmitted TX data)."]
pub type I2C_NACK_W<'a> = crate::BitWriter<'a, u32, CY_INTR_S_SPEC, bool, 1>;
#[doc = "Field `I2C_ACK` reader - I2C slave acknowledgement received. Set to '1', when the slave receives a ACK (typically after the slave transmitted TX data)."]
pub type I2C_ACK_R = crate::BitReader<bool>;
#[doc = "Field `I2C_ACK` writer - I2C slave acknowledgement received. Set to '1', when the slave receives a ACK (typically after the slave transmitted TX data)."]
pub type I2C_ACK_W<'a> = crate::BitWriter<'a, u32, CY_INTR_S_SPEC, bool, 2>;
#[doc = "Field `I2C_WRITE_STOP` reader - I2C STOP event for I2C write transfer intended for this slave (address matching is performed).Set to '1', when STOP or REPEATED START event is detected."]
pub type I2C_WRITE_STOP_R = crate::BitReader<bool>;
#[doc = "Field `I2C_WRITE_STOP` writer - I2C STOP event for I2C write transfer intended for this slave (address matching is performed).Set to '1', when STOP or REPEATED START event is detected."]
pub type I2C_WRITE_STOP_W<'a> = crate::BitWriter<'a, u32, CY_INTR_S_SPEC, bool, 3>;
#[doc = "Field `I2C_STOP` reader - I2C STOP event for I2C (read or write) transfer intended for this slave (address matching is performed). Set to '1', when STOP or REPEATED START event is detected"]
pub type I2C_STOP_R = crate::BitReader<bool>;
#[doc = "Field `I2C_STOP` writer - I2C STOP event for I2C (read or write) transfer intended for this slave (address matching is performed). Set to '1', when STOP or REPEATED START event is detected"]
pub type I2C_STOP_W<'a> = crate::BitWriter<'a, u32, CY_INTR_S_SPEC, bool, 4>;
#[doc = "Field `I2C_START` reader - I2C slave START received. Set to '1', when START or REPEATED START event is detected."]
pub type I2C_START_R = crate::BitReader<bool>;
#[doc = "Field `I2C_START` writer - I2C slave START received. Set to '1', when START or REPEATED START event is detected."]
pub type I2C_START_W<'a> = crate::BitWriter<'a, u32, CY_INTR_S_SPEC, bool, 5>;
#[doc = "Field `I2C_ADDR_MATCH` reader - I2C slave matching address received. If CTRL.ADDR_ACCEPT, the received address (including the R/W bit) is available in the RX FIFO."]
pub type I2C_ADDR_MATCH_R = crate::BitReader<bool>;
#[doc = "Field `I2C_ADDR_MATCH` writer - I2C slave matching address received. If CTRL.ADDR_ACCEPT, the received address (including the R/W bit) is available in the RX FIFO."]
pub type I2C_ADDR_MATCH_W<'a> = crate::BitWriter<'a, u32, CY_INTR_S_SPEC, bool, 6>;
#[doc = "Field `I2C_GENERAL` reader - I2C slave general call address received. If CTRL.ADDR_ACCEPT, the received address 0x00 (including the R/W bit) is available in the RX FIFO"]
pub type I2C_GENERAL_R = crate::BitReader<bool>;
#[doc = "Field `I2C_GENERAL` writer - I2C slave general call address received. If CTRL.ADDR_ACCEPT, the received address 0x00 (including the R/W bit) is available in the RX FIFO"]
pub type I2C_GENERAL_W<'a> = crate::BitWriter<'a, u32, CY_INTR_S_SPEC, bool, 7>;
#[doc = "Field `I2C_BUS_ERR` reader - I2C slave bus error (unexpected detection of START or STOP condition)."]
pub type I2C_BUS_ERR_R = crate::BitReader<bool>;
#[doc = "Field `I2C_BUS_ERR` writer - I2C slave bus error (unexpected detection of START or STOP condition)."]
pub type I2C_BUS_ERR_W<'a> = crate::BitWriter<'a, u32, CY_INTR_S_SPEC, bool, 8>;
#[doc = "Field `SPI_BUS_ERR` reader - SPI slave deselected at an unexpected time in the SPI transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
pub type SPI_BUS_ERR_R = crate::BitReader<bool>;
#[doc = "Field `SPI_BUS_ERR` writer - SPI slave deselected at an unexpected time in the SPI transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
pub type SPI_BUS_ERR_W<'a> = crate::BitWriter<'a, u32, CY_INTR_S_SPEC, bool, 11>;
impl R {
    #[doc = "Bit 0 - I2C slave lost arbitration: the value driven on the SDA line is not the same as the value observed on the SDA line (while the SCL line is '1')."]
    #[inline(always)]
    pub fn i2c_arb_lost(&self) -> I2C_ARB_LOST_R {
        I2C_ARB_LOST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C slave negative acknowledgement received. Set to '1', when the slave receives a NACK (typically after the slave transmitted TX data)."]
    #[inline(always)]
    pub fn i2c_nack(&self) -> I2C_NACK_R {
        I2C_NACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C slave acknowledgement received. Set to '1', when the slave receives a ACK (typically after the slave transmitted TX data)."]
    #[inline(always)]
    pub fn i2c_ack(&self) -> I2C_ACK_R {
        I2C_ACK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C STOP event for I2C write transfer intended for this slave (address matching is performed).Set to '1', when STOP or REPEATED START event is detected."]
    #[inline(always)]
    pub fn i2c_write_stop(&self) -> I2C_WRITE_STOP_R {
        I2C_WRITE_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I2C STOP event for I2C (read or write) transfer intended for this slave (address matching is performed). Set to '1', when STOP or REPEATED START event is detected"]
    #[inline(always)]
    pub fn i2c_stop(&self) -> I2C_STOP_R {
        I2C_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C slave START received. Set to '1', when START or REPEATED START event is detected."]
    #[inline(always)]
    pub fn i2c_start(&self) -> I2C_START_R {
        I2C_START_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C slave matching address received. If CTRL.ADDR_ACCEPT, the received address (including the R/W bit) is available in the RX FIFO."]
    #[inline(always)]
    pub fn i2c_addr_match(&self) -> I2C_ADDR_MATCH_R {
        I2C_ADDR_MATCH_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C slave general call address received. If CTRL.ADDR_ACCEPT, the received address 0x00 (including the R/W bit) is available in the RX FIFO"]
    #[inline(always)]
    pub fn i2c_general(&self) -> I2C_GENERAL_R {
        I2C_GENERAL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C slave bus error (unexpected detection of START or STOP condition)."]
    #[inline(always)]
    pub fn i2c_bus_err(&self) -> I2C_BUS_ERR_R {
        I2C_BUS_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - SPI slave deselected at an unexpected time in the SPI transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
    #[inline(always)]
    pub fn spi_bus_err(&self) -> SPI_BUS_ERR_R {
        SPI_BUS_ERR_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C slave lost arbitration: the value driven on the SDA line is not the same as the value observed on the SDA line (while the SCL line is '1')."]
    #[inline(always)]
    pub fn i2c_arb_lost(&mut self) -> I2C_ARB_LOST_W {
        I2C_ARB_LOST_W::new(self)
    }
    #[doc = "Bit 1 - I2C slave negative acknowledgement received. Set to '1', when the slave receives a NACK (typically after the slave transmitted TX data)."]
    #[inline(always)]
    pub fn i2c_nack(&mut self) -> I2C_NACK_W {
        I2C_NACK_W::new(self)
    }
    #[doc = "Bit 2 - I2C slave acknowledgement received. Set to '1', when the slave receives a ACK (typically after the slave transmitted TX data)."]
    #[inline(always)]
    pub fn i2c_ack(&mut self) -> I2C_ACK_W {
        I2C_ACK_W::new(self)
    }
    #[doc = "Bit 3 - I2C STOP event for I2C write transfer intended for this slave (address matching is performed).Set to '1', when STOP or REPEATED START event is detected."]
    #[inline(always)]
    pub fn i2c_write_stop(&mut self) -> I2C_WRITE_STOP_W {
        I2C_WRITE_STOP_W::new(self)
    }
    #[doc = "Bit 4 - I2C STOP event for I2C (read or write) transfer intended for this slave (address matching is performed). Set to '1', when STOP or REPEATED START event is detected"]
    #[inline(always)]
    pub fn i2c_stop(&mut self) -> I2C_STOP_W {
        I2C_STOP_W::new(self)
    }
    #[doc = "Bit 5 - I2C slave START received. Set to '1', when START or REPEATED START event is detected."]
    #[inline(always)]
    pub fn i2c_start(&mut self) -> I2C_START_W {
        I2C_START_W::new(self)
    }
    #[doc = "Bit 6 - I2C slave matching address received. If CTRL.ADDR_ACCEPT, the received address (including the R/W bit) is available in the RX FIFO."]
    #[inline(always)]
    pub fn i2c_addr_match(&mut self) -> I2C_ADDR_MATCH_W {
        I2C_ADDR_MATCH_W::new(self)
    }
    #[doc = "Bit 7 - I2C slave general call address received. If CTRL.ADDR_ACCEPT, the received address 0x00 (including the R/W bit) is available in the RX FIFO"]
    #[inline(always)]
    pub fn i2c_general(&mut self) -> I2C_GENERAL_W {
        I2C_GENERAL_W::new(self)
    }
    #[doc = "Bit 8 - I2C slave bus error (unexpected detection of START or STOP condition)."]
    #[inline(always)]
    pub fn i2c_bus_err(&mut self) -> I2C_BUS_ERR_W {
        I2C_BUS_ERR_W::new(self)
    }
    #[doc = "Bit 11 - SPI slave deselected at an unexpected time in the SPI transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
    #[inline(always)]
    pub fn spi_bus_err(&mut self) -> SPI_BUS_ERR_W {
        SPI_BUS_ERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave interrupt request register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cy_intr_s](index.html) module"]
pub struct CY_INTR_S_SPEC;
impl crate::RegisterSpec for CY_INTR_S_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cy_intr_s::R](R) reader structure"]
impl crate::Readable for CY_INTR_S_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cy_intr_s::W](W) writer structure"]
impl crate::Writable for CY_INTR_S_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Cy_INTR_S to value 0"]
impl crate::Resettable for CY_INTR_S_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
