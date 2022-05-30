#[doc = "Register `DMAC_CH_CTL2` reader"]
pub struct R(crate::R<DMAC_CH_CTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_CH_CTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_CH_CTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_CH_CTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAC_CH_CTL2` writer"]
pub struct W(crate::W<DMAC_CH_CTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAC_CH_CTL2_SPEC>;
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
impl From<crate::W<DMAC_CH_CTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAC_CH_CTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel enable control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLED_A {
    #[doc = "0: Channel is currently disabled."]
    DISABLED = 0,
    #[doc = "1: Channel is currently enabled."]
    ENABLED = 1,
}
impl From<ENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLED` reader - Channel enable control."]
pub type ENABLED_R = crate::BitReader<ENABLED_A>;
impl ENABLED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLED_A {
        match self.bits {
            false => ENABLED_A::DISABLED,
            true => ENABLED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLED_A::ENABLED
    }
}
#[doc = "Field `ENABLED` writer - Channel enable control."]
pub type ENABLED_W<'a> = crate::BitWriter<'a, u32, DMAC_CH_CTL2_SPEC, ENABLED_A, 31>;
impl<'a> ENABLED_W<'a> {
    #[doc = "Channel is currently disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLED_A::DISABLED)
    }
    #[doc = "Channel is currently enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLED_A::ENABLED)
    }
}
#[doc = "Identifies the descriptor structure that is currently in use by the DMA controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PING_PONG_A {
    #[doc = "0: Descriptor 0 (PING) is currently in use."]
    DESCR0 = 0,
    #[doc = "1: Descriptor 1 (PONG) is currently in use."]
    DESCR1 = 1,
}
impl From<PING_PONG_A> for bool {
    #[inline(always)]
    fn from(variant: PING_PONG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PING_PONG` reader - Identifies the descriptor structure that is currently in use by the DMA controller."]
pub type PING_PONG_R = crate::BitReader<PING_PONG_A>;
impl PING_PONG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PING_PONG_A {
        match self.bits {
            false => PING_PONG_A::DESCR0,
            true => PING_PONG_A::DESCR1,
        }
    }
    #[doc = "Checks if the value of the field is `DESCR0`"]
    #[inline(always)]
    pub fn is_descr0(&self) -> bool {
        *self == PING_PONG_A::DESCR0
    }
    #[doc = "Checks if the value of the field is `DESCR1`"]
    #[inline(always)]
    pub fn is_descr1(&self) -> bool {
        *self == PING_PONG_A::DESCR1
    }
}
#[doc = "Field `PING_PONG` writer - Identifies the descriptor structure that is currently in use by the DMA controller."]
pub type PING_PONG_W<'a> = crate::BitWriter<'a, u32, DMAC_CH_CTL2_SPEC, PING_PONG_A, 30>;
impl<'a> PING_PONG_W<'a> {
    #[doc = "Descriptor 0 (PING) is currently in use."]
    #[inline(always)]
    pub fn descr0(self) -> &'a mut W {
        self.variant(PING_PONG_A::DESCR0)
    }
    #[doc = "Descriptor 1 (PONG) is currently in use."]
    #[inline(always)]
    pub fn descr1(self) -> &'a mut W {
        self.variant(PING_PONG_A::DESCR1)
    }
}
#[doc = "Field `PRIO` reader - Channel priority. Priority can be 0,1,2 or 3. 0 is the highest."]
pub type PRIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRIO` writer - Channel priority. Priority can be 0,1,2 or 3. 0 is the highest."]
pub type PRIO_W<'a> = crate::FieldWriter<'a, u32, DMAC_CH_CTL2_SPEC, u8, u8, 2, 28>;
impl R {
    #[doc = "Bit 31 - Channel enable control."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - Identifies the descriptor structure that is currently in use by the DMA controller."]
    #[inline(always)]
    pub fn ping_pong(&self) -> PING_PONG_R {
        PING_PONG_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Channel priority. Priority can be 0,1,2 or 3. 0 is the highest."]
    #[inline(always)]
    pub fn prio(&self) -> PRIO_R {
        PRIO_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Channel enable control."]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W {
        ENABLED_W::new(self)
    }
    #[doc = "Bit 30 - Identifies the descriptor structure that is currently in use by the DMA controller."]
    #[inline(always)]
    pub fn ping_pong(&mut self) -> PING_PONG_W {
        PING_PONG_W::new(self)
    }
    #[doc = "Bits 28:29 - Channel priority. Priority can be 0,1,2 or 3. 0 is the highest."]
    #[inline(always)]
    pub fn prio(&mut self) -> PRIO_W {
        PRIO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA channel 2 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_ch_ctl2](index.html) module"]
pub struct DMAC_CH_CTL2_SPEC;
impl crate::RegisterSpec for DMAC_CH_CTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_ch_ctl2::R](R) reader structure"]
impl crate::Readable for DMAC_CH_CTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmac_ch_ctl2::W](W) writer structure"]
impl crate::Writable for DMAC_CH_CTL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAC_CH_CTL2 to value 0"]
impl crate::Resettable for DMAC_CH_CTL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
