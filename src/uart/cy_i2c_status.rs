#[doc = "Register `Cy_I2C_STATUS` reader"]
pub struct R(crate::R<CY_I2C_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CY_I2C_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CY_I2C_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CY_I2C_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Cy_I2C_STATUS` writer"]
pub struct W(crate::W<CY_I2C_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CY_I2C_STATUS_SPEC>;
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
impl From<crate::W<CY_I2C_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CY_I2C_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUS_BUSY` reader - I2C bus is busy. The bus is considered busy ('1')."]
pub type BUS_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `S_READ` reader - I2C slave read transfer ('1') or I2C slave write transfer ('0'). When the I2C slave is inactive/idle or receiving START, REPEATED START, STOP or an address, this field is '0'."]
pub type S_READ_R = crate::BitReader<bool>;
#[doc = "Field `M_READ` reader - I2C master read transfer ('1') or I2C master write transfer ('0'). When the I2C master is inactive/idle or transmitting START, REPEATED START, STOP or an address, this field is '0''."]
pub type M_READ_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - I2C bus is busy. The bus is considered busy ('1')."]
    #[inline(always)]
    pub fn bus_busy(&self) -> BUS_BUSY_R {
        BUS_BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - I2C slave read transfer ('1') or I2C slave write transfer ('0'). When the I2C slave is inactive/idle or receiving START, REPEATED START, STOP or an address, this field is '0'."]
    #[inline(always)]
    pub fn s_read(&self) -> S_READ_R {
        S_READ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C master read transfer ('1') or I2C master write transfer ('0'). When the I2C master is inactive/idle or transmitting START, REPEATED START, STOP or an address, this field is '0''."]
    #[inline(always)]
    pub fn m_read(&self) -> M_READ_R {
        M_READ_R::new(((self.bits >> 5) & 1) != 0)
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
#[doc = "I2C status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cy_i2c_status](index.html) module"]
pub struct CY_I2C_STATUS_SPEC;
impl crate::RegisterSpec for CY_I2C_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cy_i2c_status::R](R) reader structure"]
impl crate::Readable for CY_I2C_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cy_i2c_status::W](W) writer structure"]
impl crate::Writable for CY_I2C_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Cy_I2C_STATUS to value 0"]
impl crate::Resettable for CY_I2C_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
