#[doc = "Register `CMD` reader"]
pub struct R(crate::R<CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
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
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNTER_CAPTURE` reader - Counters SW capture trigger. When written with '1', a capture trigger is generated and the HW sets the field to '0' when the SW trigger has taken effect. It should be noted that the HW operates on the counter frequency. If the counter is disabled through CTRL.COUNTER_ENABLED, the field is immediately set to '0'."]
pub type COUNTER_CAPTURE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COUNTER_CAPTURE` writer - Counters SW capture trigger. When written with '1', a capture trigger is generated and the HW sets the field to '0' when the SW trigger has taken effect. It should be noted that the HW operates on the counter frequency. If the counter is disabled through CTRL.COUNTER_ENABLED, the field is immediately set to '0'."]
pub type COUNTER_CAPTURE_W<'a> = crate::FieldWriter<'a, u32, CMD_SPEC, u8, u8, 8, 0>;
#[doc = "Field `COUNTER_RELOAD` reader - Counters SW reload trigger. For HW behavior, see COUNTER_CAPTURE field."]
pub type COUNTER_RELOAD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COUNTER_RELOAD` writer - Counters SW reload trigger. For HW behavior, see COUNTER_CAPTURE field."]
pub type COUNTER_RELOAD_W<'a> = crate::FieldWriter<'a, u32, CMD_SPEC, u8, u8, 8, 8>;
#[doc = "Field `COUNTER_STOP` reader - Counters SW stop trigger. For HW behavior, see COUNTER_CAPTURE field."]
pub type COUNTER_STOP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COUNTER_STOP` writer - Counters SW stop trigger. For HW behavior, see COUNTER_CAPTURE field."]
pub type COUNTER_STOP_W<'a> = crate::FieldWriter<'a, u32, CMD_SPEC, u8, u8, 8, 16>;
#[doc = "Field `COUNTER_START` reader - Counters SW start trigger. For HW behavior, see COUNTER_CAPTURE field."]
pub type COUNTER_START_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COUNTER_START` writer - Counters SW start trigger. For HW behavior, see COUNTER_CAPTURE field."]
pub type COUNTER_START_W<'a> = crate::FieldWriter<'a, u32, CMD_SPEC, u8, u8, 8, 24>;
impl R {
    #[doc = "Bits 0:7 - Counters SW capture trigger. When written with '1', a capture trigger is generated and the HW sets the field to '0' when the SW trigger has taken effect. It should be noted that the HW operates on the counter frequency. If the counter is disabled through CTRL.COUNTER_ENABLED, the field is immediately set to '0'."]
    #[inline(always)]
    pub fn counter_capture(&self) -> COUNTER_CAPTURE_R {
        COUNTER_CAPTURE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Counters SW reload trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    pub fn counter_reload(&self) -> COUNTER_RELOAD_R {
        COUNTER_RELOAD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Counters SW stop trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    pub fn counter_stop(&self) -> COUNTER_STOP_R {
        COUNTER_STOP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Counters SW start trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    pub fn counter_start(&self) -> COUNTER_START_R {
        COUNTER_START_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Counters SW capture trigger. When written with '1', a capture trigger is generated and the HW sets the field to '0' when the SW trigger has taken effect. It should be noted that the HW operates on the counter frequency. If the counter is disabled through CTRL.COUNTER_ENABLED, the field is immediately set to '0'."]
    #[inline(always)]
    pub fn counter_capture(&mut self) -> COUNTER_CAPTURE_W {
        COUNTER_CAPTURE_W::new(self)
    }
    #[doc = "Bits 8:15 - Counters SW reload trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    pub fn counter_reload(&mut self) -> COUNTER_RELOAD_W {
        COUNTER_RELOAD_W::new(self)
    }
    #[doc = "Bits 16:23 - Counters SW stop trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    pub fn counter_stop(&mut self) -> COUNTER_STOP_W {
        COUNTER_STOP_W::new(self)
    }
    #[doc = "Bits 24:31 - Counters SW start trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    pub fn counter_start(&mut self) -> COUNTER_START_W {
        COUNTER_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCPWM command register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd::R](R) reader structure"]
impl crate::Readable for CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd::W](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
