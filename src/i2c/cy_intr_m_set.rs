#[doc = "Register `Cy_INTR_M_SET` reader"]
pub struct R(crate::R<CY_INTR_M_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CY_INTR_M_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CY_INTR_M_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CY_INTR_M_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Cy_INTR_M_SET` writer"]
pub struct W(crate::W<CY_INTR_M_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CY_INTR_M_SET_SPEC>;
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
impl From<crate::W<CY_INTR_M_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CY_INTR_M_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_LOST_ARB` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type I2C_LOST_ARB_R = crate::BitReader<bool>;
#[doc = "Field `I2C_LOST_ARB` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type I2C_LOST_ARB_W<'a> = crate::BitWriter<'a, u32, CY_INTR_M_SET_SPEC, bool, 0>;
#[doc = "Field `I2C_NACK` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type I2C_NACK_R = crate::BitReader<bool>;
#[doc = "Field `I2C_NACK` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type I2C_NACK_W<'a> = crate::BitWriter<'a, u32, CY_INTR_M_SET_SPEC, bool, 1>;
#[doc = "Field `I2C_ACK` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type I2C_ACK_R = crate::BitReader<bool>;
#[doc = "Field `I2C_ACK` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type I2C_ACK_W<'a> = crate::BitWriter<'a, u32, CY_INTR_M_SET_SPEC, bool, 2>;
#[doc = "Field `I2C_STOP` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type I2C_STOP_R = crate::BitReader<bool>;
#[doc = "Field `I2C_STOP` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type I2C_STOP_W<'a> = crate::BitWriter<'a, u32, CY_INTR_M_SET_SPEC, bool, 4>;
#[doc = "Field `I2C_BUS_ERR` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type I2C_BUS_ERR_R = crate::BitReader<bool>;
#[doc = "Field `I2C_BUS_ERR` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type I2C_BUS_ERR_W<'a> = crate::BitWriter<'a, u32, CY_INTR_M_SET_SPEC, bool, 8>;
#[doc = "Field `SPI_DONE` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type SPI_DONE_R = crate::BitReader<bool>;
#[doc = "Field `SPI_DONE` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type SPI_DONE_W<'a> = crate::BitWriter<'a, u32, CY_INTR_M_SET_SPEC, bool, 9>;
impl R {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_lost_arb(&self) -> I2C_LOST_ARB_R {
        I2C_LOST_ARB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_nack(&self) -> I2C_NACK_R {
        I2C_NACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_ack(&self) -> I2C_ACK_R {
        I2C_ACK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_stop(&self) -> I2C_STOP_R {
        I2C_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_bus_err(&self) -> I2C_BUS_ERR_R {
        I2C_BUS_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn spi_done(&self) -> SPI_DONE_R {
        SPI_DONE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_lost_arb(&mut self) -> I2C_LOST_ARB_W {
        I2C_LOST_ARB_W::new(self)
    }
    #[doc = "Bit 1 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_nack(&mut self) -> I2C_NACK_W {
        I2C_NACK_W::new(self)
    }
    #[doc = "Bit 2 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_ack(&mut self) -> I2C_ACK_W {
        I2C_ACK_W::new(self)
    }
    #[doc = "Bit 4 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_stop(&mut self) -> I2C_STOP_W {
        I2C_STOP_W::new(self)
    }
    #[doc = "Bit 8 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_bus_err(&mut self) -> I2C_BUS_ERR_W {
        I2C_BUS_ERR_W::new(self)
    }
    #[doc = "Bit 9 - Write with '1' to set corresponding bit in interrupt request register."]
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
#[doc = "Master interrupt set request register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cy_intr_m_set](index.html) module"]
pub struct CY_INTR_M_SET_SPEC;
impl crate::RegisterSpec for CY_INTR_M_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cy_intr_m_set::R](R) reader structure"]
impl crate::Readable for CY_INTR_M_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cy_intr_m_set::W](W) writer structure"]
impl crate::Writable for CY_INTR_M_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Cy_INTR_M_SET to value 0"]
impl crate::Resettable for CY_INTR_M_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
