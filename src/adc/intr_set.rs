#[doc = "Register `INTR_SET` reader"]
pub struct R(crate::R<INTR_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_SET` writer"]
pub struct W(crate::W<INTR_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_SET_SPEC>;
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
impl From<crate::W<INTR_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EOS_INTR` reader - End Of Scan Interrupt"]
pub type EOS_INTR_R = crate::BitReader<bool>;
#[doc = "Field `EOS_INTR` writer - End Of Scan Interrupt"]
pub type EOS_INTR_W<'a> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, 0>;
#[doc = "Field `OVERFLOW_INTR` reader - Overflow Interrupt"]
pub type OVERFLOW_INTR_R = crate::BitReader<bool>;
#[doc = "Field `OVERFLOW_INTR` writer - Overflow Interrupt"]
pub type OVERFLOW_INTR_W<'a> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, 1>;
#[doc = "Field `FW_COLLISION_INTR` reader - Firmware Collision Interrupt"]
pub type FW_COLLISION_INTR_R = crate::BitReader<bool>;
#[doc = "Field `FW_COLLISION_INTR` writer - Firmware Collision Interrupt"]
pub type FW_COLLISION_INTR_W<'a> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, 2>;
#[doc = "Field `DSI_COLLISION_INTR` reader - DSI Collision Interrupt"]
pub type DSI_COLLISION_INTR_R = crate::BitReader<bool>;
#[doc = "Field `DSI_COLLISION_INTR` writer - DSI Collision Interrupt"]
pub type DSI_COLLISION_INTR_W<'a> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, 3>;
#[doc = "Field `INJ_EOC_INTR` reader - Injection End of Conversion Interrupt"]
pub type INJ_EOC_INTR_R = crate::BitReader<bool>;
#[doc = "Field `INJ_EOC_INTR` writer - Injection End of Conversion Interrupt"]
pub type INJ_EOC_INTR_W<'a> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, 4>;
#[doc = "Field `INJ_SATURATE_INTR` reader - Injection Saturation Interrupt"]
pub type INJ_SATURATE_INTR_R = crate::BitReader<bool>;
#[doc = "Field `INJ_SATURATE_INTR` writer - Injection Saturation Interrupt"]
pub type INJ_SATURATE_INTR_W<'a> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, 5>;
#[doc = "Field `INJ_RANGE_INTR` reader - Injection Range detect Interrupt"]
pub type INJ_RANGE_INTR_R = crate::BitReader<bool>;
#[doc = "Field `INJ_RANGE_INTR` writer - Injection Range detect Interrupt"]
pub type INJ_RANGE_INTR_W<'a> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, 6>;
#[doc = "Field `INJ_COLLISION_INTR` reader - Injection Collision Interrupt"]
pub type INJ_COLLISION_INTR_R = crate::BitReader<bool>;
#[doc = "Field `INJ_COLLISION_INTR` writer - Injection Collision Interrupt"]
pub type INJ_COLLISION_INTR_W<'a> = crate::BitWriter<'a, u32, INTR_SET_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - End Of Scan Interrupt"]
    #[inline(always)]
    pub fn eos_intr(&self) -> EOS_INTR_R {
        EOS_INTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overflow Interrupt"]
    #[inline(always)]
    pub fn overflow_intr(&self) -> OVERFLOW_INTR_R {
        OVERFLOW_INTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Firmware Collision Interrupt"]
    #[inline(always)]
    pub fn fw_collision_intr(&self) -> FW_COLLISION_INTR_R {
        FW_COLLISION_INTR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DSI Collision Interrupt"]
    #[inline(always)]
    pub fn dsi_collision_intr(&self) -> DSI_COLLISION_INTR_R {
        DSI_COLLISION_INTR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Injection End of Conversion Interrupt"]
    #[inline(always)]
    pub fn inj_eoc_intr(&self) -> INJ_EOC_INTR_R {
        INJ_EOC_INTR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Injection Saturation Interrupt"]
    #[inline(always)]
    pub fn inj_saturate_intr(&self) -> INJ_SATURATE_INTR_R {
        INJ_SATURATE_INTR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Injection Range detect Interrupt"]
    #[inline(always)]
    pub fn inj_range_intr(&self) -> INJ_RANGE_INTR_R {
        INJ_RANGE_INTR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Injection Collision Interrupt"]
    #[inline(always)]
    pub fn inj_collision_intr(&self) -> INJ_COLLISION_INTR_R {
        INJ_COLLISION_INTR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End Of Scan Interrupt"]
    #[inline(always)]
    pub fn eos_intr(&mut self) -> EOS_INTR_W {
        EOS_INTR_W::new(self)
    }
    #[doc = "Bit 1 - Overflow Interrupt"]
    #[inline(always)]
    pub fn overflow_intr(&mut self) -> OVERFLOW_INTR_W {
        OVERFLOW_INTR_W::new(self)
    }
    #[doc = "Bit 2 - Firmware Collision Interrupt"]
    #[inline(always)]
    pub fn fw_collision_intr(&mut self) -> FW_COLLISION_INTR_W {
        FW_COLLISION_INTR_W::new(self)
    }
    #[doc = "Bit 3 - DSI Collision Interrupt"]
    #[inline(always)]
    pub fn dsi_collision_intr(&mut self) -> DSI_COLLISION_INTR_W {
        DSI_COLLISION_INTR_W::new(self)
    }
    #[doc = "Bit 4 - Injection End of Conversion Interrupt"]
    #[inline(always)]
    pub fn inj_eoc_intr(&mut self) -> INJ_EOC_INTR_W {
        INJ_EOC_INTR_W::new(self)
    }
    #[doc = "Bit 5 - Injection Saturation Interrupt"]
    #[inline(always)]
    pub fn inj_saturate_intr(&mut self) -> INJ_SATURATE_INTR_W {
        INJ_SATURATE_INTR_W::new(self)
    }
    #[doc = "Bit 6 - Injection Range detect Interrupt"]
    #[inline(always)]
    pub fn inj_range_intr(&mut self) -> INJ_RANGE_INTR_W {
        INJ_RANGE_INTR_W::new(self)
    }
    #[doc = "Bit 7 - Injection Collision Interrupt"]
    #[inline(always)]
    pub fn inj_collision_intr(&mut self) -> INJ_COLLISION_INTR_W {
        INJ_COLLISION_INTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Not really a register, intended for verification/debug. When read, this register reflects the interrupt request register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_set](index.html) module"]
pub struct INTR_SET_SPEC;
impl crate::RegisterSpec for INTR_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_set::R](R) reader structure"]
impl crate::Readable for INTR_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_set::W](W) writer structure"]
impl crate::Writable for INTR_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR_SET to value 0"]
impl crate::Resettable for INTR_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
