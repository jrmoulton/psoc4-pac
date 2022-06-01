#[doc = "Register `Cy_INTR_CAUSE` reader"]
pub struct R(crate::R<CY_INTR_CAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CY_INTR_CAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CY_INTR_CAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CY_INTR_CAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Cy_INTR_CAUSE` writer"]
pub struct W(crate::W<CY_INTR_CAUSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CY_INTR_CAUSE_SPEC>;
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
impl From<crate::W<CY_INTR_CAUSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CY_INTR_CAUSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASTER` reader - Master interrupt active."]
pub type MASTER_R = crate::BitReader<bool>;
#[doc = "Field `SLAVE` reader - Slave interrupt active."]
pub type SLAVE_R = crate::BitReader<bool>;
#[doc = "Field `TX` reader - Transmitter interrupt active."]
pub type TX_R = crate::BitReader<bool>;
#[doc = "Field `RX` reader - Receiver interrupt active."]
pub type RX_R = crate::BitReader<bool>;
#[doc = "Field `I2C_EC` reader - Externally clock I2C interrupt active."]
pub type I2C_EC_R = crate::BitReader<bool>;
#[doc = "Field `SPI_EC` reader - Externally clocked SPI interrupt active."]
pub type SPI_EC_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Master interrupt active."]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Slave interrupt active."]
    #[inline(always)]
    pub fn slave(&self) -> SLAVE_R {
        SLAVE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmitter interrupt active."]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receiver interrupt active."]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Externally clock I2C interrupt active."]
    #[inline(always)]
    pub fn i2c_ec(&self) -> I2C_EC_R {
        I2C_EC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Externally clocked SPI interrupt active."]
    #[inline(always)]
    pub fn spi_ec(&self) -> SPI_EC_R {
        SPI_EC_R::new(((self.bits >> 5) & 1) != 0)
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
#[doc = "Interrupt cause register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cy_intr_cause](index.html) module"]
pub struct CY_INTR_CAUSE_SPEC;
impl crate::RegisterSpec for CY_INTR_CAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cy_intr_cause::R](R) reader structure"]
impl crate::Readable for CY_INTR_CAUSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cy_intr_cause::W](W) writer structure"]
impl crate::Writable for CY_INTR_CAUSE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Cy_INTR_CAUSE to value 0"]
impl crate::Resettable for CY_INTR_CAUSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
