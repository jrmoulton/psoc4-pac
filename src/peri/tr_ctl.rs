#[doc = "Register `TR_CTL` reader"]
pub struct R(crate::R<TR_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TR_CTL` writer"]
pub struct W(crate::W<TR_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TR_CTL_SPEC>;
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
impl From<crate::W<TR_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TR_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TR_SEL` reader - Specifies the activated trigger when TR_ACT is '1'. TR_OUT specifies whether the activated trigger is an input trigger or output trigger to the trigger multiplexer. During activation (TR_ACT is '1'), SW should not modify this register field. If the specified trigger is not present, the trigger activation has no effect."]
pub type TR_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TR_SEL` writer - Specifies the activated trigger when TR_ACT is '1'. TR_OUT specifies whether the activated trigger is an input trigger or output trigger to the trigger multiplexer. During activation (TR_ACT is '1'), SW should not modify this register field. If the specified trigger is not present, the trigger activation has no effect."]
pub type TR_SEL_W<'a> = crate::FieldWriter<'a, u32, TR_CTL_SPEC, u8, u8, 7, 0>;
#[doc = "Field `TR_GROUP` reader - Specifies the trigger group."]
pub type TR_GROUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TR_GROUP` writer - Specifies the trigger group."]
pub type TR_GROUP_W<'a> = crate::FieldWriter<'a, u32, TR_CTL_SPEC, u8, u8, 4, 8>;
#[doc = "Field `TR_COUNT` reader - Amount of cycles a specific trigger is activated. During activation (TR_ACT is '1'), HW decrements this field to '0' using a cycle counter. During activation, SW should not modify this register field. A value of 255 is a special case: HW does NOT decrement this field to '0' and trigger activation is under direct control of TR_ACT: when TR_ACT is '1' the trigger is activated and when TR_ACT is '0' the trigger is deactivated."]
pub type TR_COUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TR_COUNT` writer - Amount of cycles a specific trigger is activated. During activation (TR_ACT is '1'), HW decrements this field to '0' using a cycle counter. During activation, SW should not modify this register field. A value of 255 is a special case: HW does NOT decrement this field to '0' and trigger activation is under direct control of TR_ACT: when TR_ACT is '1' the trigger is activated and when TR_ACT is '0' the trigger is deactivated."]
pub type TR_COUNT_W<'a> = crate::FieldWriter<'a, u32, TR_CTL_SPEC, u8, u8, 8, 16>;
#[doc = "Field `TR_OUT` reader - Specifies whether trigger activation is for a specific input or ouput trigger of the trigger multiplexer. Activation of a specific input trigger, will result in activation of all output triggers that have the specific input trigger selected through their TR_OUT_CTL.SEL field. Activation of a specific output trigger, will result in activation of the specified TR_SEL output trigger only. '0': TR_SEL selection and trigger activation is for an input trigger to the trigger multiplexer. '1': TR_SEL selection and trigger activation is for an output trigger from the trigger multiplexer."]
pub type TR_OUT_R = crate::BitReader<bool>;
#[doc = "Field `TR_OUT` writer - Specifies whether trigger activation is for a specific input or ouput trigger of the trigger multiplexer. Activation of a specific input trigger, will result in activation of all output triggers that have the specific input trigger selected through their TR_OUT_CTL.SEL field. Activation of a specific output trigger, will result in activation of the specified TR_SEL output trigger only. '0': TR_SEL selection and trigger activation is for an input trigger to the trigger multiplexer. '1': TR_SEL selection and trigger activation is for an output trigger from the trigger multiplexer."]
pub type TR_OUT_W<'a> = crate::BitWriter<'a, u32, TR_CTL_SPEC, bool, 30>;
#[doc = "Field `TR_ACT` reader - SW sets this field to '1' by to activate (set to '1') a trigger as identified by TR_SEL and TR_OUT for TR_COUNT cycles. HW sets this field to '0' when the cycle counter is decremented to '0'. Note: a TR_COUNT value of 255 is a special case and trigger activation is under direct control of the TR_ACT field (the counter is not decremented). Note: when TR_ACT is '1', SW should not modify the other register fields. SW MUST NOT set TR_ACT bit to '1' while updating the other register bits simultaneously. At first the SW MUST update the other register bits as needed, and then set TR_ACT to '1' with a new register write.'"]
pub type TR_ACT_R = crate::BitReader<bool>;
#[doc = "Field `TR_ACT` writer - SW sets this field to '1' by to activate (set to '1') a trigger as identified by TR_SEL and TR_OUT for TR_COUNT cycles. HW sets this field to '0' when the cycle counter is decremented to '0'. Note: a TR_COUNT value of 255 is a special case and trigger activation is under direct control of the TR_ACT field (the counter is not decremented). Note: when TR_ACT is '1', SW should not modify the other register fields. SW MUST NOT set TR_ACT bit to '1' while updating the other register bits simultaneously. At first the SW MUST update the other register bits as needed, and then set TR_ACT to '1' with a new register write.'"]
pub type TR_ACT_W<'a> = crate::BitWriter<'a, u32, TR_CTL_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:6 - Specifies the activated trigger when TR_ACT is '1'. TR_OUT specifies whether the activated trigger is an input trigger or output trigger to the trigger multiplexer. During activation (TR_ACT is '1'), SW should not modify this register field. If the specified trigger is not present, the trigger activation has no effect."]
    #[inline(always)]
    pub fn tr_sel(&self) -> TR_SEL_R {
        TR_SEL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:11 - Specifies the trigger group."]
    #[inline(always)]
    pub fn tr_group(&self) -> TR_GROUP_R {
        TR_GROUP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Amount of cycles a specific trigger is activated. During activation (TR_ACT is '1'), HW decrements this field to '0' using a cycle counter. During activation, SW should not modify this register field. A value of 255 is a special case: HW does NOT decrement this field to '0' and trigger activation is under direct control of TR_ACT: when TR_ACT is '1' the trigger is activated and when TR_ACT is '0' the trigger is deactivated."]
    #[inline(always)]
    pub fn tr_count(&self) -> TR_COUNT_R {
        TR_COUNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 30 - Specifies whether trigger activation is for a specific input or ouput trigger of the trigger multiplexer. Activation of a specific input trigger, will result in activation of all output triggers that have the specific input trigger selected through their TR_OUT_CTL.SEL field. Activation of a specific output trigger, will result in activation of the specified TR_SEL output trigger only. '0': TR_SEL selection and trigger activation is for an input trigger to the trigger multiplexer. '1': TR_SEL selection and trigger activation is for an output trigger from the trigger multiplexer."]
    #[inline(always)]
    pub fn tr_out(&self) -> TR_OUT_R {
        TR_OUT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SW sets this field to '1' by to activate (set to '1') a trigger as identified by TR_SEL and TR_OUT for TR_COUNT cycles. HW sets this field to '0' when the cycle counter is decremented to '0'. Note: a TR_COUNT value of 255 is a special case and trigger activation is under direct control of the TR_ACT field (the counter is not decremented). Note: when TR_ACT is '1', SW should not modify the other register fields. SW MUST NOT set TR_ACT bit to '1' while updating the other register bits simultaneously. At first the SW MUST update the other register bits as needed, and then set TR_ACT to '1' with a new register write.'"]
    #[inline(always)]
    pub fn tr_act(&self) -> TR_ACT_R {
        TR_ACT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Specifies the activated trigger when TR_ACT is '1'. TR_OUT specifies whether the activated trigger is an input trigger or output trigger to the trigger multiplexer. During activation (TR_ACT is '1'), SW should not modify this register field. If the specified trigger is not present, the trigger activation has no effect."]
    #[inline(always)]
    pub fn tr_sel(&mut self) -> TR_SEL_W {
        TR_SEL_W::new(self)
    }
    #[doc = "Bits 8:11 - Specifies the trigger group."]
    #[inline(always)]
    pub fn tr_group(&mut self) -> TR_GROUP_W {
        TR_GROUP_W::new(self)
    }
    #[doc = "Bits 16:23 - Amount of cycles a specific trigger is activated. During activation (TR_ACT is '1'), HW decrements this field to '0' using a cycle counter. During activation, SW should not modify this register field. A value of 255 is a special case: HW does NOT decrement this field to '0' and trigger activation is under direct control of TR_ACT: when TR_ACT is '1' the trigger is activated and when TR_ACT is '0' the trigger is deactivated."]
    #[inline(always)]
    pub fn tr_count(&mut self) -> TR_COUNT_W {
        TR_COUNT_W::new(self)
    }
    #[doc = "Bit 30 - Specifies whether trigger activation is for a specific input or ouput trigger of the trigger multiplexer. Activation of a specific input trigger, will result in activation of all output triggers that have the specific input trigger selected through their TR_OUT_CTL.SEL field. Activation of a specific output trigger, will result in activation of the specified TR_SEL output trigger only. '0': TR_SEL selection and trigger activation is for an input trigger to the trigger multiplexer. '1': TR_SEL selection and trigger activation is for an output trigger from the trigger multiplexer."]
    #[inline(always)]
    pub fn tr_out(&mut self) -> TR_OUT_W {
        TR_OUT_W::new(self)
    }
    #[doc = "Bit 31 - SW sets this field to '1' by to activate (set to '1') a trigger as identified by TR_SEL and TR_OUT for TR_COUNT cycles. HW sets this field to '0' when the cycle counter is decremented to '0'. Note: a TR_COUNT value of 255 is a special case and trigger activation is under direct control of the TR_ACT field (the counter is not decremented). Note: when TR_ACT is '1', SW should not modify the other register fields. SW MUST NOT set TR_ACT bit to '1' while updating the other register bits simultaneously. At first the SW MUST update the other register bits as needed, and then set TR_ACT to '1' with a new register write.'"]
    #[inline(always)]
    pub fn tr_act(&mut self) -> TR_ACT_W {
        TR_ACT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trigger control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_ctl](index.html) module"]
pub struct TR_CTL_SPEC;
impl crate::RegisterSpec for TR_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tr_ctl::R](R) reader structure"]
impl crate::Readable for TR_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tr_ctl::W](W) writer structure"]
impl crate::Writable for TR_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TR_CTL to value 0"]
impl crate::Resettable for TR_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
