#[doc = "Register `Cy_INTR_M` reader"]
pub struct R(crate::R<CY_INTR_M_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CY_INTR_M_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CY_INTR_M_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CY_INTR_M_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Cy_INTR_M` writer"]
pub struct W(crate::W<CY_INTR_M_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CY_INTR_M_SPEC>;
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
impl From<crate::W<CY_INTR_M_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CY_INTR_M_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_LOST_ARB` reader - I2C master lost arbitration: the value driven by the master on the SDA line is not the same as the value observed on the SDA line."]
pub type I2C_LOST_ARB_R = crate::BitReader<bool>;
#[doc = "Field `I2C_LOST_ARB` writer - I2C master lost arbitration: the value driven by the master on the SDA line is not the same as the value observed on the SDA line."]
pub type I2C_LOST_ARB_W<'a> = crate::BitWriter<'a, u32, CY_INTR_M_SPEC, bool, 0>;
#[doc = "Field `I2C_NACK` reader - I2C master negative acknowledgement. Set to '1', when the master receives a NACK (typically after the master transmitted the slave address or TX data)."]
pub type I2C_NACK_R = crate::BitReader<bool>;
#[doc = "Field `I2C_NACK` writer - I2C master negative acknowledgement. Set to '1', when the master receives a NACK (typically after the master transmitted the slave address or TX data)."]
pub type I2C_NACK_W<'a> = crate::BitWriter<'a, u32, CY_INTR_M_SPEC, bool, 1>;
#[doc = "Field `I2C_ACK` reader - I2C master acknowledgement. Set to '1', when the master receives a ACK (typically after the master transmitted the slave address or TX data)."]
pub type I2C_ACK_R = crate::BitReader<bool>;
#[doc = "Field `I2C_ACK` writer - I2C master acknowledgement. Set to '1', when the master receives a ACK (typically after the master transmitted the slave address or TX data)."]
pub type I2C_ACK_W<'a> = crate::BitWriter<'a, u32, CY_INTR_M_SPEC, bool, 2>;
#[doc = "Field `I2C_STOP` reader - I2C master STOP. Set to '1', when the master has transmitted a STOP."]
pub type I2C_STOP_R = crate::BitReader<bool>;
#[doc = "Field `I2C_STOP` writer - I2C master STOP. Set to '1', when the master has transmitted a STOP."]
pub type I2C_STOP_W<'a> = crate::BitWriter<'a, u32, CY_INTR_M_SPEC, bool, 4>;
#[doc = "Field `I2C_BUS_ERR` reader - I2C master bus error (unexpected detection of START or STOP condition)."]
pub type I2C_BUS_ERR_R = crate::BitReader<bool>;
#[doc = "Field `I2C_BUS_ERR` writer - I2C master bus error (unexpected detection of START or STOP condition)."]
pub type I2C_BUS_ERR_W<'a> = crate::BitWriter<'a, u32, CY_INTR_M_SPEC, bool, 8>;
#[doc = "Field `SPI_DONE` reader - SPI master transfer done event: all data frames in the transmit FIFO are sent and the transmit FIFO is empty."]
pub type SPI_DONE_R = crate::BitReader<bool>;
#[doc = "Field `SPI_DONE` writer - SPI master transfer done event: all data frames in the transmit FIFO are sent and the transmit FIFO is empty."]
pub type SPI_DONE_W<'a> = crate::BitWriter<'a, u32, CY_INTR_M_SPEC, bool, 9>;
impl R {
    #[doc = "Bit 0 - I2C master lost arbitration: the value driven by the master on the SDA line is not the same as the value observed on the SDA line."]
    #[inline(always)]
    pub fn i2c_lost_arb(&self) -> I2C_LOST_ARB_R {
        I2C_LOST_ARB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C master negative acknowledgement. Set to '1', when the master receives a NACK (typically after the master transmitted the slave address or TX data)."]
    #[inline(always)]
    pub fn i2c_nack(&self) -> I2C_NACK_R {
        I2C_NACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C master acknowledgement. Set to '1', when the master receives a ACK (typically after the master transmitted the slave address or TX data)."]
    #[inline(always)]
    pub fn i2c_ack(&self) -> I2C_ACK_R {
        I2C_ACK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - I2C master STOP. Set to '1', when the master has transmitted a STOP."]
    #[inline(always)]
    pub fn i2c_stop(&self) -> I2C_STOP_R {
        I2C_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C master bus error (unexpected detection of START or STOP condition)."]
    #[inline(always)]
    pub fn i2c_bus_err(&self) -> I2C_BUS_ERR_R {
        I2C_BUS_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SPI master transfer done event: all data frames in the transmit FIFO are sent and the transmit FIFO is empty."]
    #[inline(always)]
    pub fn spi_done(&self) -> SPI_DONE_R {
        SPI_DONE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C master lost arbitration: the value driven by the master on the SDA line is not the same as the value observed on the SDA line."]
    #[inline(always)]
    pub fn i2c_lost_arb(&mut self) -> I2C_LOST_ARB_W {
        I2C_LOST_ARB_W::new(self)
    }
    #[doc = "Bit 1 - I2C master negative acknowledgement. Set to '1', when the master receives a NACK (typically after the master transmitted the slave address or TX data)."]
    #[inline(always)]
    pub fn i2c_nack(&mut self) -> I2C_NACK_W {
        I2C_NACK_W::new(self)
    }
    #[doc = "Bit 2 - I2C master acknowledgement. Set to '1', when the master receives a ACK (typically after the master transmitted the slave address or TX data)."]
    #[inline(always)]
    pub fn i2c_ack(&mut self) -> I2C_ACK_W {
        I2C_ACK_W::new(self)
    }
    #[doc = "Bit 4 - I2C master STOP. Set to '1', when the master has transmitted a STOP."]
    #[inline(always)]
    pub fn i2c_stop(&mut self) -> I2C_STOP_W {
        I2C_STOP_W::new(self)
    }
    #[doc = "Bit 8 - I2C master bus error (unexpected detection of START or STOP condition)."]
    #[inline(always)]
    pub fn i2c_bus_err(&mut self) -> I2C_BUS_ERR_W {
        I2C_BUS_ERR_W::new(self)
    }
    #[doc = "Bit 9 - SPI master transfer done event: all data frames in the transmit FIFO are sent and the transmit FIFO is empty."]
    #[inline(always)]
    pub fn spi_done(&mut self) -> SPI_DONE_W {
        SPI_DONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master interrupt request register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cy_intr_m](index.html) module"]
pub struct CY_INTR_M_SPEC;
impl crate::RegisterSpec for CY_INTR_M_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cy_intr_m::R](R) reader structure"]
impl crate::Readable for CY_INTR_M_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cy_intr_m::W](W) writer structure"]
impl crate::Writable for CY_INTR_M_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Cy_INTR_M to value 0"]
impl crate::Resettable for CY_INTR_M_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
