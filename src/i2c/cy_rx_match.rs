#[doc = "Register `Cy_RX_MATCH` reader"]
pub struct R(crate::R<CY_RX_MATCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CY_RX_MATCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CY_RX_MATCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CY_RX_MATCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Cy_RX_MATCH` writer"]
pub struct W(crate::W<CY_RX_MATCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CY_RX_MATCH_SPEC>;
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
impl From<crate::W<CY_RX_MATCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CY_RX_MATCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - Slave device address. For UART multi-processor mode all eight bits a reused. For I2C, bit 0 of the register is not used."]
pub type ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR` writer - Slave device address. For UART multi-processor mode all eight bits a reused. For I2C, bit 0 of the register is not used."]
pub type ADDR_W<'a> = crate::FieldWriter<'a, u32, CY_RX_MATCH_SPEC, u8, u8, 8, 0>;
#[doc = "Field `MASK` reader - Slave device address mask. This field is a 8 bit mask that specifies which of the ADDR field bits in the SCB_RX_MATCH_ADDR register take part in the matching of the slave address."]
pub type MASK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASK` writer - Slave device address mask. This field is a 8 bit mask that specifies which of the ADDR field bits in the SCB_RX_MATCH_ADDR register take part in the matching of the slave address."]
pub type MASK_W<'a> = crate::FieldWriter<'a, u32, CY_RX_MATCH_SPEC, u8, u8, 8, 16>;
impl R {
    #[doc = "Bits 0:7 - Slave device address. For UART multi-processor mode all eight bits a reused. For I2C, bit 0 of the register is not used."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Slave device address mask. This field is a 8 bit mask that specifies which of the ADDR field bits in the SCB_RX_MATCH_ADDR register take part in the matching of the slave address."]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Slave device address. For UART multi-processor mode all eight bits a reused. For I2C, bit 0 of the register is not used."]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W::new(self)
    }
    #[doc = "Bits 16:23 - Slave device address mask. This field is a 8 bit mask that specifies which of the ADDR field bits in the SCB_RX_MATCH_ADDR register take part in the matching of the slave address."]
    #[inline(always)]
    pub fn mask(&mut self) -> MASK_W {
        MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave address and mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cy_rx_match](index.html) module"]
pub struct CY_RX_MATCH_SPEC;
impl crate::RegisterSpec for CY_RX_MATCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cy_rx_match::R](R) reader structure"]
impl crate::Readable for CY_RX_MATCH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cy_rx_match::W](W) writer structure"]
impl crate::Writable for CY_RX_MATCH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Cy_RX_MATCH to value 0"]
impl crate::Resettable for CY_RX_MATCH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
