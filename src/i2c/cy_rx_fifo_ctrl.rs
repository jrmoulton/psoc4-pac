#[doc = "Register `Cy_RX_FIFO_CTRL` reader"]
pub struct R(crate::R<CY_RX_FIFO_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CY_RX_FIFO_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CY_RX_FIFO_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CY_RX_FIFO_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Cy_RX_FIFO_CTRL` writer"]
pub struct W(crate::W<CY_RX_FIFO_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CY_RX_FIFO_CTRL_SPEC>;
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
impl From<crate::W<CY_RX_FIFO_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CY_RX_FIFO_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIGGER_LEVEL` reader - Trigger level. When the receiver FIFO has more entries than the amount of this field, a receiver trigger event is generated."]
pub type TRIGGER_LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIGGER_LEVEL` writer - Trigger level. When the receiver FIFO has more entries than the amount of this field, a receiver trigger event is generated."]
pub type TRIGGER_LEVEL_W<'a> = crate::FieldWriter<'a, u32, CY_RX_FIFO_CTRL_SPEC, u8, u8, 4, 0>;
#[doc = "Field `CLEAR` reader - When '1', the receiver FIFO and receiver shift register are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
pub type CLEAR_R = crate::BitReader<bool>;
#[doc = "Field `CLEAR` writer - When '1', the receiver FIFO and receiver shift register are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
pub type CLEAR_W<'a> = crate::BitWriter<'a, u32, CY_RX_FIFO_CTRL_SPEC, bool, 16>;
#[doc = "Field `FREEZE` reader - When '1', hardware writes to the receiver FIFO have no effect. Freeze will not advance the RX FIFO write pointer."]
pub type FREEZE_R = crate::BitReader<bool>;
#[doc = "Field `FREEZE` writer - When '1', hardware writes to the receiver FIFO have no effect. Freeze will not advance the RX FIFO write pointer."]
pub type FREEZE_W<'a> = crate::BitWriter<'a, u32, CY_RX_FIFO_CTRL_SPEC, bool, 17>;
impl R {
    #[doc = "Bits 0:3 - Trigger level. When the receiver FIFO has more entries than the amount of this field, a receiver trigger event is generated."]
    #[inline(always)]
    pub fn trigger_level(&self) -> TRIGGER_LEVEL_R {
        TRIGGER_LEVEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 16 - When '1', the receiver FIFO and receiver shift register are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
    #[inline(always)]
    pub fn clear(&self) -> CLEAR_R {
        CLEAR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - When '1', hardware writes to the receiver FIFO have no effect. Freeze will not advance the RX FIFO write pointer."]
    #[inline(always)]
    pub fn freeze(&self) -> FREEZE_R {
        FREEZE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Trigger level. When the receiver FIFO has more entries than the amount of this field, a receiver trigger event is generated."]
    #[inline(always)]
    pub fn trigger_level(&mut self) -> TRIGGER_LEVEL_W {
        TRIGGER_LEVEL_W::new(self)
    }
    #[doc = "Bit 16 - When '1', the receiver FIFO and receiver shift register are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
    #[inline(always)]
    pub fn clear(&mut self) -> CLEAR_W {
        CLEAR_W::new(self)
    }
    #[doc = "Bit 17 - When '1', hardware writes to the receiver FIFO have no effect. Freeze will not advance the RX FIFO write pointer."]
    #[inline(always)]
    pub fn freeze(&mut self) -> FREEZE_W {
        FREEZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receiver FIFO control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cy_rx_fifo_ctrl](index.html) module"]
pub struct CY_RX_FIFO_CTRL_SPEC;
impl crate::RegisterSpec for CY_RX_FIFO_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cy_rx_fifo_ctrl::R](R) reader structure"]
impl crate::Readable for CY_RX_FIFO_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cy_rx_fifo_ctrl::W](W) writer structure"]
impl crate::Writable for CY_RX_FIFO_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Cy_RX_FIFO_CTRL to value 0"]
impl crate::Resettable for CY_RX_FIFO_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}