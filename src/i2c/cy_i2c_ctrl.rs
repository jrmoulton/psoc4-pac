#[doc = "Register `Cy_I2C_CTRL` reader"]
pub struct R(crate::R<CY_I2C_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CY_I2C_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CY_I2C_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CY_I2C_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Cy_I2C_CTRL` writer"]
pub struct W(crate::W<CY_I2C_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CY_I2C_CTRL_SPEC>;
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
impl From<crate::W<CY_I2C_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CY_I2C_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HIGH_PHASE_OVS` reader - Serial I2C interface high phase oversampling factor. HIGH_PHASE_OVS + 1 peripheral clock periods constitute the high phase of a bit period."]
pub type HIGH_PHASE_OVS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HIGH_PHASE_OVS` writer - Serial I2C interface high phase oversampling factor. HIGH_PHASE_OVS + 1 peripheral clock periods constitute the high phase of a bit period."]
pub type HIGH_PHASE_OVS_W<'a> = crate::FieldWriter<'a, u32, CY_I2C_CTRL_SPEC, u8, u8, 4, 0>;
#[doc = "Field `LOW_PHASE_OVS` reader - Serial I2C interface low phase oversampling factor. LOW_PHASE_OVS + 1 peripheral clock periods constitute the low phase of a bit period"]
pub type LOW_PHASE_OVS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOW_PHASE_OVS` writer - Serial I2C interface low phase oversampling factor. LOW_PHASE_OVS + 1 peripheral clock periods constitute the low phase of a bit period"]
pub type LOW_PHASE_OVS_W<'a> = crate::FieldWriter<'a, u32, CY_I2C_CTRL_SPEC, u8, u8, 4, 4>;
#[doc = "Field `M_READY_DATA_ACK` reader - When '1', a received data element by the master is immediately ACK'd when the receiver FIFO is not full."]
pub type M_READY_DATA_ACK_R = crate::BitReader<bool>;
#[doc = "Field `M_READY_DATA_ACK` writer - When '1', a received data element by the master is immediately ACK'd when the receiver FIFO is not full."]
pub type M_READY_DATA_ACK_W<'a> = crate::BitWriter<'a, u32, CY_I2C_CTRL_SPEC, bool, 8>;
#[doc = "Field `M_NOT_READY_DATA_NACK` reader - When '1', a received data element byte the master is immediately NACK'd when the receiver FIFO is full. When '0', clock stretching is used instead (till the receiver FIFO is no longer full)."]
pub type M_NOT_READY_DATA_NACK_R = crate::BitReader<bool>;
#[doc = "Field `M_NOT_READY_DATA_NACK` writer - When '1', a received data element byte the master is immediately NACK'd when the receiver FIFO is full. When '0', clock stretching is used instead (till the receiver FIFO is no longer full)."]
pub type M_NOT_READY_DATA_NACK_W<'a> = crate::BitWriter<'a, u32, CY_I2C_CTRL_SPEC, bool, 9>;
#[doc = "Field `S_GENERAL_IGNORE` reader - When '1', a received general call slave address is immediately NACK'd (no ACK or clock stretching) and treated as a non matching slave address."]
pub type S_GENERAL_IGNORE_R = crate::BitReader<bool>;
#[doc = "Field `S_GENERAL_IGNORE` writer - When '1', a received general call slave address is immediately NACK'd (no ACK or clock stretching) and treated as a non matching slave address."]
pub type S_GENERAL_IGNORE_W<'a> = crate::BitWriter<'a, u32, CY_I2C_CTRL_SPEC, bool, 11>;
#[doc = "Field `S_READY_ADDR_ACK` reader - When '1', a received (matching) slave address is immediately ACK'd when the receiver FIFO is not full"]
pub type S_READY_ADDR_ACK_R = crate::BitReader<bool>;
#[doc = "Field `S_READY_ADDR_ACK` writer - When '1', a received (matching) slave address is immediately ACK'd when the receiver FIFO is not full"]
pub type S_READY_ADDR_ACK_W<'a> = crate::BitWriter<'a, u32, CY_I2C_CTRL_SPEC, bool, 12>;
#[doc = "Field `S_READY_DATA_ACK` reader - When '1', a received data element by the slave is immediately ACK'd when the receiver FIFO is not full"]
pub type S_READY_DATA_ACK_R = crate::BitReader<bool>;
#[doc = "Field `S_READY_DATA_ACK` writer - When '1', a received data element by the slave is immediately ACK'd when the receiver FIFO is not full"]
pub type S_READY_DATA_ACK_W<'a> = crate::BitWriter<'a, u32, CY_I2C_CTRL_SPEC, bool, 13>;
#[doc = "Field `S_NOT_READY_ADDR_NACK` reader - When '1', a received address by the slave is immediately ACK'd when the receiver FIFO is not full"]
pub type S_NOT_READY_ADDR_NACK_R = crate::BitReader<bool>;
#[doc = "Field `S_NOT_READY_ADDR_NACK` writer - When '1', a received address by the slave is immediately ACK'd when the receiver FIFO is not full"]
pub type S_NOT_READY_ADDR_NACK_W<'a> = crate::BitWriter<'a, u32, CY_I2C_CTRL_SPEC, bool, 14>;
#[doc = "Field `S_NOT_READY_DATA_NACK` reader - When '1' a received data element byte the slave is immediately NACK'd when the receiver FIFO is full. When '1' clock stretching is performed (till the receiver FIFO is no longer full)."]
pub type S_NOT_READY_DATA_NACK_R = crate::BitReader<bool>;
#[doc = "Field `S_NOT_READY_DATA_NACK` writer - When '1' a received data element byte the slave is immediately NACK'd when the receiver FIFO is full. When '1' clock stretching is performed (till the receiver FIFO is no longer full)."]
pub type S_NOT_READY_DATA_NACK_W<'a> = crate::BitWriter<'a, u32, CY_I2C_CTRL_SPEC, bool, 15>;
#[doc = "Field `LOOPBACK` reader - Local loopback control"]
pub type LOOPBACK_R = crate::BitReader<bool>;
#[doc = "Field `LOOPBACK` writer - Local loopback control"]
pub type LOOPBACK_W<'a> = crate::BitWriter<'a, u32, CY_I2C_CTRL_SPEC, bool, 16>;
#[doc = "Field `SLAVE_MODE` reader - Slave mode enabled ('1') or not ('0')."]
pub type SLAVE_MODE_R = crate::BitReader<bool>;
#[doc = "Field `SLAVE_MODE` writer - Slave mode enabled ('1') or not ('0')."]
pub type SLAVE_MODE_W<'a> = crate::BitWriter<'a, u32, CY_I2C_CTRL_SPEC, bool, 30>;
#[doc = "Field `MASTER_MODE` reader - Master mode enabled ('1') or not ('0'). Note that both master and slave modes can be enabled at the same time. This allows the IP to address itself."]
pub type MASTER_MODE_R = crate::BitReader<bool>;
#[doc = "Field `MASTER_MODE` writer - Master mode enabled ('1') or not ('0'). Note that both master and slave modes can be enabled at the same time. This allows the IP to address itself."]
pub type MASTER_MODE_W<'a> = crate::BitWriter<'a, u32, CY_I2C_CTRL_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:3 - Serial I2C interface high phase oversampling factor. HIGH_PHASE_OVS + 1 peripheral clock periods constitute the high phase of a bit period."]
    #[inline(always)]
    pub fn high_phase_ovs(&self) -> HIGH_PHASE_OVS_R {
        HIGH_PHASE_OVS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Serial I2C interface low phase oversampling factor. LOW_PHASE_OVS + 1 peripheral clock periods constitute the low phase of a bit period"]
    #[inline(always)]
    pub fn low_phase_ovs(&self) -> LOW_PHASE_OVS_R {
        LOW_PHASE_OVS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - When '1', a received data element by the master is immediately ACK'd when the receiver FIFO is not full."]
    #[inline(always)]
    pub fn m_ready_data_ack(&self) -> M_READY_DATA_ACK_R {
        M_READY_DATA_ACK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When '1', a received data element byte the master is immediately NACK'd when the receiver FIFO is full. When '0', clock stretching is used instead (till the receiver FIFO is no longer full)."]
    #[inline(always)]
    pub fn m_not_ready_data_nack(&self) -> M_NOT_READY_DATA_NACK_R {
        M_NOT_READY_DATA_NACK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - When '1', a received general call slave address is immediately NACK'd (no ACK or clock stretching) and treated as a non matching slave address."]
    #[inline(always)]
    pub fn s_general_ignore(&self) -> S_GENERAL_IGNORE_R {
        S_GENERAL_IGNORE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - When '1', a received (matching) slave address is immediately ACK'd when the receiver FIFO is not full"]
    #[inline(always)]
    pub fn s_ready_addr_ack(&self) -> S_READY_ADDR_ACK_R {
        S_READY_ADDR_ACK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - When '1', a received data element by the slave is immediately ACK'd when the receiver FIFO is not full"]
    #[inline(always)]
    pub fn s_ready_data_ack(&self) -> S_READY_DATA_ACK_R {
        S_READY_DATA_ACK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - When '1', a received address by the slave is immediately ACK'd when the receiver FIFO is not full"]
    #[inline(always)]
    pub fn s_not_ready_addr_nack(&self) -> S_NOT_READY_ADDR_NACK_R {
        S_NOT_READY_ADDR_NACK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - When '1' a received data element byte the slave is immediately NACK'd when the receiver FIFO is full. When '1' clock stretching is performed (till the receiver FIFO is no longer full)."]
    #[inline(always)]
    pub fn s_not_ready_data_nack(&self) -> S_NOT_READY_DATA_NACK_R {
        S_NOT_READY_DATA_NACK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Local loopback control"]
    #[inline(always)]
    pub fn loopback(&self) -> LOOPBACK_R {
        LOOPBACK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 30 - Slave mode enabled ('1') or not ('0')."]
    #[inline(always)]
    pub fn slave_mode(&self) -> SLAVE_MODE_R {
        SLAVE_MODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Master mode enabled ('1') or not ('0'). Note that both master and slave modes can be enabled at the same time. This allows the IP to address itself."]
    #[inline(always)]
    pub fn master_mode(&self) -> MASTER_MODE_R {
        MASTER_MODE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Serial I2C interface high phase oversampling factor. HIGH_PHASE_OVS + 1 peripheral clock periods constitute the high phase of a bit period."]
    #[inline(always)]
    pub fn high_phase_ovs(&mut self) -> HIGH_PHASE_OVS_W {
        HIGH_PHASE_OVS_W::new(self)
    }
    #[doc = "Bits 4:7 - Serial I2C interface low phase oversampling factor. LOW_PHASE_OVS + 1 peripheral clock periods constitute the low phase of a bit period"]
    #[inline(always)]
    pub fn low_phase_ovs(&mut self) -> LOW_PHASE_OVS_W {
        LOW_PHASE_OVS_W::new(self)
    }
    #[doc = "Bit 8 - When '1', a received data element by the master is immediately ACK'd when the receiver FIFO is not full."]
    #[inline(always)]
    pub fn m_ready_data_ack(&mut self) -> M_READY_DATA_ACK_W {
        M_READY_DATA_ACK_W::new(self)
    }
    #[doc = "Bit 9 - When '1', a received data element byte the master is immediately NACK'd when the receiver FIFO is full. When '0', clock stretching is used instead (till the receiver FIFO is no longer full)."]
    #[inline(always)]
    pub fn m_not_ready_data_nack(&mut self) -> M_NOT_READY_DATA_NACK_W {
        M_NOT_READY_DATA_NACK_W::new(self)
    }
    #[doc = "Bit 11 - When '1', a received general call slave address is immediately NACK'd (no ACK or clock stretching) and treated as a non matching slave address."]
    #[inline(always)]
    pub fn s_general_ignore(&mut self) -> S_GENERAL_IGNORE_W {
        S_GENERAL_IGNORE_W::new(self)
    }
    #[doc = "Bit 12 - When '1', a received (matching) slave address is immediately ACK'd when the receiver FIFO is not full"]
    #[inline(always)]
    pub fn s_ready_addr_ack(&mut self) -> S_READY_ADDR_ACK_W {
        S_READY_ADDR_ACK_W::new(self)
    }
    #[doc = "Bit 13 - When '1', a received data element by the slave is immediately ACK'd when the receiver FIFO is not full"]
    #[inline(always)]
    pub fn s_ready_data_ack(&mut self) -> S_READY_DATA_ACK_W {
        S_READY_DATA_ACK_W::new(self)
    }
    #[doc = "Bit 14 - When '1', a received address by the slave is immediately ACK'd when the receiver FIFO is not full"]
    #[inline(always)]
    pub fn s_not_ready_addr_nack(&mut self) -> S_NOT_READY_ADDR_NACK_W {
        S_NOT_READY_ADDR_NACK_W::new(self)
    }
    #[doc = "Bit 15 - When '1' a received data element byte the slave is immediately NACK'd when the receiver FIFO is full. When '1' clock stretching is performed (till the receiver FIFO is no longer full)."]
    #[inline(always)]
    pub fn s_not_ready_data_nack(&mut self) -> S_NOT_READY_DATA_NACK_W {
        S_NOT_READY_DATA_NACK_W::new(self)
    }
    #[doc = "Bit 16 - Local loopback control"]
    #[inline(always)]
    pub fn loopback(&mut self) -> LOOPBACK_W {
        LOOPBACK_W::new(self)
    }
    #[doc = "Bit 30 - Slave mode enabled ('1') or not ('0')."]
    #[inline(always)]
    pub fn slave_mode(&mut self) -> SLAVE_MODE_W {
        SLAVE_MODE_W::new(self)
    }
    #[doc = "Bit 31 - Master mode enabled ('1') or not ('0'). Note that both master and slave modes can be enabled at the same time. This allows the IP to address itself."]
    #[inline(always)]
    pub fn master_mode(&mut self) -> MASTER_MODE_W {
        MASTER_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cy_i2c_ctrl](index.html) module"]
pub struct CY_I2C_CTRL_SPEC;
impl crate::RegisterSpec for CY_I2C_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cy_i2c_ctrl::R](R) reader structure"]
impl crate::Readable for CY_I2C_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cy_i2c_ctrl::W](W) writer structure"]
impl crate::Writable for CY_I2C_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Cy_I2C_CTRL to value 0"]
impl crate::Resettable for CY_I2C_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
