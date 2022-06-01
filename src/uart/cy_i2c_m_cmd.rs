#[doc = "Register `Cy_I2C_M_CMD` reader"]
pub struct R(crate::R<CY_I2C_M_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CY_I2C_M_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CY_I2C_M_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CY_I2C_M_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Cy_I2C_M_CMD` writer"]
pub struct W(crate::W<CY_I2C_M_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CY_I2C_M_CMD_SPEC>;
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
impl From<crate::W<CY_I2C_M_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CY_I2C_M_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `M_START` reader - When '1', transmit a START or REPEATED START. Whether a START or REPEATED START is transmitted depends on the state of the master state machine. When this action is performed, the hardware sets this field to '0'."]
pub type M_START_R = crate::BitReader<bool>;
#[doc = "Field `M_START` writer - When '1', transmit a START or REPEATED START. Whether a START or REPEATED START is transmitted depends on the state of the master state machine. When this action is performed, the hardware sets this field to '0'."]
pub type M_START_W<'a> = crate::BitWriter<'a, u32, CY_I2C_M_CMD_SPEC, bool, 0>;
#[doc = "Field `M_IDLE_START` reader - When '1', transmit a START as soon as the bus is idle (I2C_STATUS.BUS_BUSY is '0', note that BUSY has a default value of '0'). When this action is performed, the hardware sets this field to '0'."]
pub type M_IDLE_START_R = crate::BitReader<bool>;
#[doc = "Field `M_IDLE_START` writer - When '1', transmit a START as soon as the bus is idle (I2C_STATUS.BUS_BUSY is '0', note that BUSY has a default value of '0'). When this action is performed, the hardware sets this field to '0'."]
pub type M_IDLE_START_W<'a> = crate::BitWriter<'a, u32, CY_I2C_M_CMD_SPEC, bool, 1>;
#[doc = "Field `M_ACK` reader - When '1', attempt to transmit an acknowledgement (ACK). When this action is performed, the hardware sets this field to '0'."]
pub type M_ACK_R = crate::BitReader<bool>;
#[doc = "Field `M_ACK` writer - When '1', attempt to transmit an acknowledgement (ACK). When this action is performed, the hardware sets this field to '0'."]
pub type M_ACK_W<'a> = crate::BitWriter<'a, u32, CY_I2C_M_CMD_SPEC, bool, 2>;
#[doc = "Field `M_NACK` reader - When '1', attempt to transmit a negative acknowledgement (NACK). When this action is performed, the hardware sets this field to '0'."]
pub type M_NACK_R = crate::BitReader<bool>;
#[doc = "Field `M_NACK` writer - When '1', attempt to transmit a negative acknowledgement (NACK). When this action is performed, the hardware sets this field to '0'."]
pub type M_NACK_W<'a> = crate::BitWriter<'a, u32, CY_I2C_M_CMD_SPEC, bool, 3>;
#[doc = "Field `M_STOP` reader - When '1', attempt to transmit a STOP. When this action is performed, the hardware sets this field to '0'. This command has a higher priority than I2C_M_CMD.M_START: in situations where both a STOP and a REPEATED START could be transmitted, M_STOP takes precedence over M_START."]
pub type M_STOP_R = crate::BitReader<bool>;
#[doc = "Field `M_STOP` writer - When '1', attempt to transmit a STOP. When this action is performed, the hardware sets this field to '0'. This command has a higher priority than I2C_M_CMD.M_START: in situations where both a STOP and a REPEATED START could be transmitted, M_STOP takes precedence over M_START."]
pub type M_STOP_W<'a> = crate::BitWriter<'a, u32, CY_I2C_M_CMD_SPEC, bool, 4>;
impl R {
    #[doc = "Bit 0 - When '1', transmit a START or REPEATED START. Whether a START or REPEATED START is transmitted depends on the state of the master state machine. When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    pub fn m_start(&self) -> M_START_R {
        M_START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When '1', transmit a START as soon as the bus is idle (I2C_STATUS.BUS_BUSY is '0', note that BUSY has a default value of '0'). When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    pub fn m_idle_start(&self) -> M_IDLE_START_R {
        M_IDLE_START_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When '1', attempt to transmit an acknowledgement (ACK). When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    pub fn m_ack(&self) -> M_ACK_R {
        M_ACK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When '1', attempt to transmit a negative acknowledgement (NACK). When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    pub fn m_nack(&self) -> M_NACK_R {
        M_NACK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When '1', attempt to transmit a STOP. When this action is performed, the hardware sets this field to '0'. This command has a higher priority than I2C_M_CMD.M_START: in situations where both a STOP and a REPEATED START could be transmitted, M_STOP takes precedence over M_START."]
    #[inline(always)]
    pub fn m_stop(&self) -> M_STOP_R {
        M_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When '1', transmit a START or REPEATED START. Whether a START or REPEATED START is transmitted depends on the state of the master state machine. When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    pub fn m_start(&mut self) -> M_START_W {
        M_START_W::new(self)
    }
    #[doc = "Bit 1 - When '1', transmit a START as soon as the bus is idle (I2C_STATUS.BUS_BUSY is '0', note that BUSY has a default value of '0'). When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    pub fn m_idle_start(&mut self) -> M_IDLE_START_W {
        M_IDLE_START_W::new(self)
    }
    #[doc = "Bit 2 - When '1', attempt to transmit an acknowledgement (ACK). When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    pub fn m_ack(&mut self) -> M_ACK_W {
        M_ACK_W::new(self)
    }
    #[doc = "Bit 3 - When '1', attempt to transmit a negative acknowledgement (NACK). When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    pub fn m_nack(&mut self) -> M_NACK_W {
        M_NACK_W::new(self)
    }
    #[doc = "Bit 4 - When '1', attempt to transmit a STOP. When this action is performed, the hardware sets this field to '0'. This command has a higher priority than I2C_M_CMD.M_START: in situations where both a STOP and a REPEATED START could be transmitted, M_STOP takes precedence over M_START."]
    #[inline(always)]
    pub fn m_stop(&mut self) -> M_STOP_W {
        M_STOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C master command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cy_i2c_m_cmd](index.html) module"]
pub struct CY_I2C_M_CMD_SPEC;
impl crate::RegisterSpec for CY_I2C_M_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cy_i2c_m_cmd::R](R) reader structure"]
impl crate::Readable for CY_I2C_M_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cy_i2c_m_cmd::W](W) writer structure"]
impl crate::Writable for CY_I2C_M_CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Cy_I2C_M_CMD to value 0"]
impl crate::Resettable for CY_I2C_M_CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
