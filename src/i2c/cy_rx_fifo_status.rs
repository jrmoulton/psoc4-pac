#[doc = "Register `Cy_RX_FIFO_STATUS` reader"]
pub struct R(crate::R<CY_RX_FIFO_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CY_RX_FIFO_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CY_RX_FIFO_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CY_RX_FIFO_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Cy_RX_FIFO_STATUS` writer"]
pub struct W(crate::W<CY_RX_FIFO_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CY_RX_FIFO_STATUS_SPEC>;
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
impl From<crate::W<CY_RX_FIFO_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CY_RX_FIFO_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USED` reader - Amount of entries in the receiver FIFO. The value of this field ranges from 0 to 8."]
pub type USED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SR_VALID` reader - Indicates whether the RX shift registers holds a (partial) valid data frame ('1') or not ('0')."]
pub type SR_VALID_R = crate::BitReader<bool>;
#[doc = "Field `RD_PTR` reader - FIFO read pointer: FIFO location from which a data frame is read."]
pub type RD_PTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WR_PTR` reader - FIFO write pointer: FIFO location at which a new data frame is written by the hardware."]
pub type WR_PTR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - Amount of entries in the receiver FIFO. The value of this field ranges from 0 to 8."]
    #[inline(always)]
    pub fn used(&self) -> USED_R {
        USED_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Indicates whether the RX shift registers holds a (partial) valid data frame ('1') or not ('0')."]
    #[inline(always)]
    pub fn sr_valid(&self) -> SR_VALID_R {
        SR_VALID_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - FIFO read pointer: FIFO location from which a data frame is read."]
    #[inline(always)]
    pub fn rd_ptr(&self) -> RD_PTR_R {
        RD_PTR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - FIFO write pointer: FIFO location at which a new data frame is written by the hardware."]
    #[inline(always)]
    pub fn wr_ptr(&self) -> WR_PTR_R {
        WR_PTR_R::new(((self.bits >> 24) & 0x1f) as u8)
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
#[doc = "Receiver FIFO status registerS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cy_rx_fifo_status](index.html) module"]
pub struct CY_RX_FIFO_STATUS_SPEC;
impl crate::RegisterSpec for CY_RX_FIFO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cy_rx_fifo_status::R](R) reader structure"]
impl crate::Readable for CY_RX_FIFO_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cy_rx_fifo_status::W](W) writer structure"]
impl crate::Writable for CY_RX_FIFO_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Cy_RX_FIFO_STATUS to value 0"]
impl crate::Resettable for CY_RX_FIFO_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
