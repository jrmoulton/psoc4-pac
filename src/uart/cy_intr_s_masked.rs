#[doc = "Register `Cy_INTR_S_MASKED` reader"]
pub struct R(crate::R<CY_INTR_S_MASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CY_INTR_S_MASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CY_INTR_S_MASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CY_INTR_S_MASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Cy_INTR_S_MASKED` writer"]
pub struct W(crate::W<CY_INTR_S_MASKED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CY_INTR_S_MASKED_SPEC>;
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
impl From<crate::W<CY_INTR_S_MASKED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CY_INTR_S_MASKED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_ARB_LOST` reader - Logical and of corresponding request and mask bits."]
pub type I2C_ARB_LOST_R = crate::BitReader<bool>;
#[doc = "Field `I2C_ARB_LOST` writer - Logical and of corresponding request and mask bits."]
pub type I2C_ARB_LOST_W<'a> = crate::BitWriter<'a, u32, CY_INTR_S_MASKED_SPEC, bool, 0>;
#[doc = "Field `I2C_NACK` reader - Logical and of corresponding request and mask bits."]
pub type I2C_NACK_R = crate::BitReader<bool>;
#[doc = "Field `I2C_NACK` writer - Logical and of corresponding request and mask bits."]
pub type I2C_NACK_W<'a> = crate::BitWriter<'a, u32, CY_INTR_S_MASKED_SPEC, bool, 1>;
#[doc = "Field `I2C_ACK` reader - Logical and of corresponding request and mask bits."]
pub type I2C_ACK_R = crate::BitReader<bool>;
#[doc = "Field `I2C_ACK` writer - Logical and of corresponding request and mask bits."]
pub type I2C_ACK_W<'a> = crate::BitWriter<'a, u32, CY_INTR_S_MASKED_SPEC, bool, 2>;
#[doc = "Field `I2C_WRITE_STOP` reader - Logical and of corresponding request and mask bits."]
pub type I2C_WRITE_STOP_R = crate::BitReader<bool>;
#[doc = "Field `I2C_WRITE_STOP` writer - Logical and of corresponding request and mask bits."]
pub type I2C_WRITE_STOP_W<'a> = crate::BitWriter<'a, u32, CY_INTR_S_MASKED_SPEC, bool, 3>;
#[doc = "Field `I2C_STOP` reader - Logical and of corresponding request and mask bits."]
pub type I2C_STOP_R = crate::BitReader<bool>;
#[doc = "Field `I2C_STOP` writer - Logical and of corresponding request and mask bits."]
pub type I2C_STOP_W<'a> = crate::BitWriter<'a, u32, CY_INTR_S_MASKED_SPEC, bool, 4>;
#[doc = "Field `I2C_START` reader - Logical and of corresponding request and mask bits."]
pub type I2C_START_R = crate::BitReader<bool>;
#[doc = "Field `I2C_START` writer - Logical and of corresponding request and mask bits."]
pub type I2C_START_W<'a> = crate::BitWriter<'a, u32, CY_INTR_S_MASKED_SPEC, bool, 5>;
#[doc = "Field `I2C_ADDR_MATCH` reader - Logical and of corresponding request and mask bits."]
pub type I2C_ADDR_MATCH_R = crate::BitReader<bool>;
#[doc = "Field `I2C_ADDR_MATCH` writer - Logical and of corresponding request and mask bits."]
pub type I2C_ADDR_MATCH_W<'a> = crate::BitWriter<'a, u32, CY_INTR_S_MASKED_SPEC, bool, 6>;
#[doc = "Field `I2C_GENERAL` reader - Logical and of corresponding request and mask bits."]
pub type I2C_GENERAL_R = crate::BitReader<bool>;
#[doc = "Field `I2C_GENERAL` writer - Logical and of corresponding request and mask bits."]
pub type I2C_GENERAL_W<'a> = crate::BitWriter<'a, u32, CY_INTR_S_MASKED_SPEC, bool, 7>;
#[doc = "Field `SPI_BUS_ERR` reader - Logical and of corresponding request and mask bits."]
pub type SPI_BUS_ERR_R = crate::BitReader<bool>;
#[doc = "Field `SPI_BUS_ERR` writer - Logical and of corresponding request and mask bits."]
pub type SPI_BUS_ERR_W<'a> = crate::BitWriter<'a, u32, CY_INTR_S_MASKED_SPEC, bool, 11>;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_arb_lost(&self) -> I2C_ARB_LOST_R {
        I2C_ARB_LOST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_nack(&self) -> I2C_NACK_R {
        I2C_NACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_ack(&self) -> I2C_ACK_R {
        I2C_ACK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_write_stop(&self) -> I2C_WRITE_STOP_R {
        I2C_WRITE_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_stop(&self) -> I2C_STOP_R {
        I2C_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_start(&self) -> I2C_START_R {
        I2C_START_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_addr_match(&self) -> I2C_ADDR_MATCH_R {
        I2C_ADDR_MATCH_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_general(&self) -> I2C_GENERAL_R {
        I2C_GENERAL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn spi_bus_err(&self) -> SPI_BUS_ERR_R {
        SPI_BUS_ERR_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_arb_lost(&mut self) -> I2C_ARB_LOST_W {
        I2C_ARB_LOST_W::new(self)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_nack(&mut self) -> I2C_NACK_W {
        I2C_NACK_W::new(self)
    }
    #[doc = "Bit 2 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_ack(&mut self) -> I2C_ACK_W {
        I2C_ACK_W::new(self)
    }
    #[doc = "Bit 3 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_write_stop(&mut self) -> I2C_WRITE_STOP_W {
        I2C_WRITE_STOP_W::new(self)
    }
    #[doc = "Bit 4 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_stop(&mut self) -> I2C_STOP_W {
        I2C_STOP_W::new(self)
    }
    #[doc = "Bit 5 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_start(&mut self) -> I2C_START_W {
        I2C_START_W::new(self)
    }
    #[doc = "Bit 6 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_addr_match(&mut self) -> I2C_ADDR_MATCH_W {
        I2C_ADDR_MATCH_W::new(self)
    }
    #[doc = "Bit 7 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_general(&mut self) -> I2C_GENERAL_W {
        I2C_GENERAL_W::new(self)
    }
    #[doc = "Bit 11 - Logical and of corresponding request and mask bits."]
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
#[doc = "Slave interrupt masked register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cy_intr_s_masked](index.html) module"]
pub struct CY_INTR_S_MASKED_SPEC;
impl crate::RegisterSpec for CY_INTR_S_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cy_intr_s_masked::R](R) reader structure"]
impl crate::Readable for CY_INTR_S_MASKED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cy_intr_s_masked::W](W) writer structure"]
impl crate::Writable for CY_INTR_S_MASKED_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Cy_INTR_S_MASKED to value 0"]
impl crate::Resettable for CY_INTR_S_MASKED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
