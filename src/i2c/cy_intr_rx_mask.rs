#[doc = "Register `Cy_INTR_RX_MASK` reader"]
pub struct R(crate::R<CY_INTR_RX_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CY_INTR_RX_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CY_INTR_RX_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CY_INTR_RX_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Cy_INTR_RX_MASK` writer"]
pub struct W(crate::W<CY_INTR_RX_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CY_INTR_RX_MASK_SPEC>;
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
impl From<crate::W<CY_INTR_RX_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CY_INTR_RX_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIGGER` reader - Mask bit for corresponding bit in interrupt request register."]
pub type TRIGGER_R = crate::BitReader<bool>;
#[doc = "Field `TRIGGER` writer - Mask bit for corresponding bit in interrupt request register."]
pub type TRIGGER_W<'a> = crate::BitWriter<'a, u32, CY_INTR_RX_MASK_SPEC, bool, 0>;
#[doc = "Field `NOT_EMPTY` reader - Mask bit for corresponding bit in interrupt request register."]
pub type NOT_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `NOT_EMPTY` writer - Mask bit for corresponding bit in interrupt request register."]
pub type NOT_EMPTY_W<'a> = crate::BitWriter<'a, u32, CY_INTR_RX_MASK_SPEC, bool, 2>;
#[doc = "Field `FULL` reader - Mask bit for corresponding bit in interrupt request register."]
pub type FULL_R = crate::BitReader<bool>;
#[doc = "Field `FULL` writer - Mask bit for corresponding bit in interrupt request register."]
pub type FULL_W<'a> = crate::BitWriter<'a, u32, CY_INTR_RX_MASK_SPEC, bool, 3>;
#[doc = "Field `OVERFLOW` reader - Mask bit for corresponding bit in interrupt request register."]
pub type OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `OVERFLOW` writer - Mask bit for corresponding bit in interrupt request register."]
pub type OVERFLOW_W<'a> = crate::BitWriter<'a, u32, CY_INTR_RX_MASK_SPEC, bool, 5>;
#[doc = "Field `UNDERFLOW` reader - Mask bit for corresponding bit in interrupt request register."]
pub type UNDERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `UNDERFLOW` writer - Mask bit for corresponding bit in interrupt request register."]
pub type UNDERFLOW_W<'a> = crate::BitWriter<'a, u32, CY_INTR_RX_MASK_SPEC, bool, 6>;
#[doc = "Field `FRAME_ERR` reader - Mask bit for corresponding bit in interrupt request register."]
pub type FRAME_ERR_R = crate::BitReader<bool>;
#[doc = "Field `FRAME_ERR` writer - Mask bit for corresponding bit in interrupt request register."]
pub type FRAME_ERR_W<'a> = crate::BitWriter<'a, u32, CY_INTR_RX_MASK_SPEC, bool, 8>;
#[doc = "Field `PARITY_ERR` reader - Mask bit for corresponding bit in interrupt request register."]
pub type PARITY_ERR_R = crate::BitReader<bool>;
#[doc = "Field `PARITY_ERR` writer - Mask bit for corresponding bit in interrupt request register."]
pub type PARITY_ERR_W<'a> = crate::BitWriter<'a, u32, CY_INTR_RX_MASK_SPEC, bool, 9>;
#[doc = "Field `BAUD_DETECT` reader - Mask bit for corresponding bit in interrupt request register."]
pub type BAUD_DETECT_R = crate::BitReader<bool>;
#[doc = "Field `BAUD_DETECT` writer - Mask bit for corresponding bit in interrupt request register."]
pub type BAUD_DETECT_W<'a> = crate::BitWriter<'a, u32, CY_INTR_RX_MASK_SPEC, bool, 10>;
#[doc = "Field `BREAK_DETECT` reader - Mask bit for corresponding bit in interrupt request register."]
pub type BREAK_DETECT_R = crate::BitReader<bool>;
#[doc = "Field `BREAK_DETECT` writer - Mask bit for corresponding bit in interrupt request register."]
pub type BREAK_DETECT_W<'a> = crate::BitWriter<'a, u32, CY_INTR_RX_MASK_SPEC, bool, 11>;
impl R {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn trigger(&self) -> TRIGGER_R {
        TRIGGER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn not_empty(&self) -> NOT_EMPTY_R {
        NOT_EMPTY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn overflow(&self) -> OVERFLOW_R {
        OVERFLOW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn underflow(&self) -> UNDERFLOW_R {
        UNDERFLOW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn frame_err(&self) -> FRAME_ERR_R {
        FRAME_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn parity_err(&self) -> PARITY_ERR_R {
        PARITY_ERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn baud_detect(&self) -> BAUD_DETECT_R {
        BAUD_DETECT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn break_detect(&self) -> BREAK_DETECT_R {
        BREAK_DETECT_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn trigger(&mut self) -> TRIGGER_W {
        TRIGGER_W::new(self)
    }
    #[doc = "Bit 2 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn not_empty(&mut self) -> NOT_EMPTY_W {
        NOT_EMPTY_W::new(self)
    }
    #[doc = "Bit 3 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn full(&mut self) -> FULL_W {
        FULL_W::new(self)
    }
    #[doc = "Bit 5 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn overflow(&mut self) -> OVERFLOW_W {
        OVERFLOW_W::new(self)
    }
    #[doc = "Bit 6 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn underflow(&mut self) -> UNDERFLOW_W {
        UNDERFLOW_W::new(self)
    }
    #[doc = "Bit 8 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn frame_err(&mut self) -> FRAME_ERR_W {
        FRAME_ERR_W::new(self)
    }
    #[doc = "Bit 9 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn parity_err(&mut self) -> PARITY_ERR_W {
        PARITY_ERR_W::new(self)
    }
    #[doc = "Bit 10 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn baud_detect(&mut self) -> BAUD_DETECT_W {
        BAUD_DETECT_W::new(self)
    }
    #[doc = "Bit 11 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn break_detect(&mut self) -> BREAK_DETECT_W {
        BREAK_DETECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receiver interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cy_intr_rx_mask](index.html) module"]
pub struct CY_INTR_RX_MASK_SPEC;
impl crate::RegisterSpec for CY_INTR_RX_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cy_intr_rx_mask::R](R) reader structure"]
impl crate::Readable for CY_INTR_RX_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cy_intr_rx_mask::W](W) writer structure"]
impl crate::Writable for CY_INTR_RX_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Cy_INTR_RX_MASK to value 0"]
impl crate::Resettable for CY_INTR_RX_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
