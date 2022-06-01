#[doc = "Register `Cy_CTRL` reader"]
pub struct R(crate::R<CY_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CY_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CY_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CY_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Cy_CTRL` writer"]
pub struct W(crate::W<CY_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CY_CTRL_SPEC>;
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
impl From<crate::W<CY_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CY_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVS` reader - Serial interface bit period oversampling factor expressed in lP clock cycles. Used for SPI and UART functionality. OVS + 1 IP clock cycles constitute a single serial interface clock/bit cycle."]
pub type OVS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OVS` writer - Serial interface bit period oversampling factor expressed in lP clock cycles. Used for SPI and UART functionality. OVS + 1 IP clock cycles constitute a single serial interface clock/bit cycle."]
pub type OVS_W<'a> = crate::FieldWriter<'a, u32, CY_CTRL_SPEC, u8, u8, 4, 0>;
#[doc = "Field `EC_AM_MODE` reader - Internally clocked mode ('0') or externally clocked mode ('1') address matching (I2C) or selection (SPI).In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface."]
pub type EC_AM_MODE_R = crate::BitReader<bool>;
#[doc = "Field `EC_AM_MODE` writer - Internally clocked mode ('0') or externally clocked mode ('1') address matching (I2C) or selection (SPI).In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface."]
pub type EC_AM_MODE_W<'a> = crate::BitWriter<'a, u32, CY_CTRL_SPEC, bool, 8>;
#[doc = "Field `EC_OP_MODE` reader - Internally clocked mode ('0') or externally clocked mode ('1') operation. In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface."]
pub type EC_OP_MODE_R = crate::BitReader<bool>;
#[doc = "Field `EC_OP_MODE` writer - Internally clocked mode ('0') or externally clocked mode ('1') operation. In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface."]
pub type EC_OP_MODE_W<'a> = crate::BitWriter<'a, u32, CY_CTRL_SPEC, bool, 9>;
#[doc = "Field `EZ_MODE` reader - Non EZ mode ('0') or EZ mode ('1'). In EZ mode, a meta protocol is applied to the serial interface protocol."]
pub type EZ_MODE_R = crate::BitReader<bool>;
#[doc = "Field `EZ_MODE` writer - Non EZ mode ('0') or EZ mode ('1'). In EZ mode, a meta protocol is applied to the serial interface protocol."]
pub type EZ_MODE_W<'a> = crate::BitWriter<'a, u32, CY_CTRL_SPEC, bool, 10>;
#[doc = "Field `BYTE_MODE` reader - Determines the number of bits per FIFO data element: '0' - 16-bit FIFO data elements (FIFO entries 8), '1' - 8-bit FIFO data elements (FIFO entries 16). Setting data elemelents to 8-bits doubles the amount of FIFO entries."]
pub type BYTE_MODE_R = crate::BitReader<bool>;
#[doc = "Field `BYTE_MODE` writer - Determines the number of bits per FIFO data element: '0' - 16-bit FIFO data elements (FIFO entries 8), '1' - 8-bit FIFO data elements (FIFO entries 16). Setting data elemelents to 8-bits doubles the amount of FIFO entries."]
pub type BYTE_MODE_W<'a> = crate::BitWriter<'a, u32, CY_CTRL_SPEC, bool, 11>;
#[doc = "Field `ADDR_ACCEPT` reader - Determines whether a received matching address is accepted in the RX FIFO ('1') or not ('0').his field is used in the I2C mode."]
pub type ADDR_ACCEPT_R = crate::BitReader<bool>;
#[doc = "Field `ADDR_ACCEPT` writer - Determines whether a received matching address is accepted in the RX FIFO ('1') or not ('0').his field is used in the I2C mode."]
pub type ADDR_ACCEPT_W<'a> = crate::BitWriter<'a, u32, CY_CTRL_SPEC, bool, 16>;
#[doc = "Field `BLOCK` reader - If the externally clocked logic and the MMIO SW accesses to EZ memory coincide/collide, this bit determines whether a SW access should block and result in bus wait states ('BLOCK is 1') or not (BLOCK is '0')"]
pub type BLOCK_R = crate::BitReader<bool>;
#[doc = "Field `BLOCK` writer - If the externally clocked logic and the MMIO SW accesses to EZ memory coincide/collide, this bit determines whether a SW access should block and result in bus wait states ('BLOCK is 1') or not (BLOCK is '0')"]
pub type BLOCK_W<'a> = crate::BitWriter<'a, u32, CY_CTRL_SPEC, bool, 17>;
#[doc = "Field `MODE` reader - Mode of operation: I2C = 0, SPI = 1, UART = 2"]
pub type MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE` writer - Mode of operation: I2C = 0, SPI = 1, UART = 2"]
pub type MODE_W<'a> = crate::FieldWriter<'a, u32, CY_CTRL_SPEC, u8, u8, 2, 24>;
#[doc = "Field `ENABLED` reader - IP enabled ('1') or not ('0')."]
pub type ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ENABLED` writer - IP enabled ('1') or not ('0')."]
pub type ENABLED_W<'a> = crate::BitWriter<'a, u32, CY_CTRL_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:3 - Serial interface bit period oversampling factor expressed in lP clock cycles. Used for SPI and UART functionality. OVS + 1 IP clock cycles constitute a single serial interface clock/bit cycle."]
    #[inline(always)]
    pub fn ovs(&self) -> OVS_R {
        OVS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Internally clocked mode ('0') or externally clocked mode ('1') address matching (I2C) or selection (SPI).In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface."]
    #[inline(always)]
    pub fn ec_am_mode(&self) -> EC_AM_MODE_R {
        EC_AM_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Internally clocked mode ('0') or externally clocked mode ('1') operation. In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface."]
    #[inline(always)]
    pub fn ec_op_mode(&self) -> EC_OP_MODE_R {
        EC_OP_MODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Non EZ mode ('0') or EZ mode ('1'). In EZ mode, a meta protocol is applied to the serial interface protocol."]
    #[inline(always)]
    pub fn ez_mode(&self) -> EZ_MODE_R {
        EZ_MODE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Determines the number of bits per FIFO data element: '0' - 16-bit FIFO data elements (FIFO entries 8), '1' - 8-bit FIFO data elements (FIFO entries 16). Setting data elemelents to 8-bits doubles the amount of FIFO entries."]
    #[inline(always)]
    pub fn byte_mode(&self) -> BYTE_MODE_R {
        BYTE_MODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Determines whether a received matching address is accepted in the RX FIFO ('1') or not ('0').his field is used in the I2C mode."]
    #[inline(always)]
    pub fn addr_accept(&self) -> ADDR_ACCEPT_R {
        ADDR_ACCEPT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - If the externally clocked logic and the MMIO SW accesses to EZ memory coincide/collide, this bit determines whether a SW access should block and result in bus wait states ('BLOCK is 1') or not (BLOCK is '0')"]
    #[inline(always)]
    pub fn block(&self) -> BLOCK_R {
        BLOCK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Mode of operation: I2C = 0, SPI = 1, UART = 2"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 31 - IP enabled ('1') or not ('0')."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Serial interface bit period oversampling factor expressed in lP clock cycles. Used for SPI and UART functionality. OVS + 1 IP clock cycles constitute a single serial interface clock/bit cycle."]
    #[inline(always)]
    pub fn ovs(&mut self) -> OVS_W {
        OVS_W::new(self)
    }
    #[doc = "Bit 8 - Internally clocked mode ('0') or externally clocked mode ('1') address matching (I2C) or selection (SPI).In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface."]
    #[inline(always)]
    pub fn ec_am_mode(&mut self) -> EC_AM_MODE_W {
        EC_AM_MODE_W::new(self)
    }
    #[doc = "Bit 9 - Internally clocked mode ('0') or externally clocked mode ('1') operation. In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface."]
    #[inline(always)]
    pub fn ec_op_mode(&mut self) -> EC_OP_MODE_W {
        EC_OP_MODE_W::new(self)
    }
    #[doc = "Bit 10 - Non EZ mode ('0') or EZ mode ('1'). In EZ mode, a meta protocol is applied to the serial interface protocol."]
    #[inline(always)]
    pub fn ez_mode(&mut self) -> EZ_MODE_W {
        EZ_MODE_W::new(self)
    }
    #[doc = "Bit 11 - Determines the number of bits per FIFO data element: '0' - 16-bit FIFO data elements (FIFO entries 8), '1' - 8-bit FIFO data elements (FIFO entries 16). Setting data elemelents to 8-bits doubles the amount of FIFO entries."]
    #[inline(always)]
    pub fn byte_mode(&mut self) -> BYTE_MODE_W {
        BYTE_MODE_W::new(self)
    }
    #[doc = "Bit 16 - Determines whether a received matching address is accepted in the RX FIFO ('1') or not ('0').his field is used in the I2C mode."]
    #[inline(always)]
    pub fn addr_accept(&mut self) -> ADDR_ACCEPT_W {
        ADDR_ACCEPT_W::new(self)
    }
    #[doc = "Bit 17 - If the externally clocked logic and the MMIO SW accesses to EZ memory coincide/collide, this bit determines whether a SW access should block and result in bus wait states ('BLOCK is 1') or not (BLOCK is '0')"]
    #[inline(always)]
    pub fn block(&mut self) -> BLOCK_W {
        BLOCK_W::new(self)
    }
    #[doc = "Bits 24:25 - Mode of operation: I2C = 0, SPI = 1, UART = 2"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W::new(self)
    }
    #[doc = "Bit 31 - IP enabled ('1') or not ('0')."]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W {
        ENABLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Generic control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cy_ctrl](index.html) module"]
pub struct CY_CTRL_SPEC;
impl crate::RegisterSpec for CY_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cy_ctrl::R](R) reader structure"]
impl crate::Readable for CY_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cy_ctrl::W](W) writer structure"]
impl crate::Writable for CY_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Cy_CTRL to value 0"]
impl crate::Resettable for CY_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
