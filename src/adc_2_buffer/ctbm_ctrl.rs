#[doc = "Register `CTBM_CTRL` reader"]
pub struct R(crate::R<CTBM_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTBM_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTBM_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTBM_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTBM_CTRL` writer"]
pub struct W(crate::W<CTBM_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTBM_CTRL_SPEC>;
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
impl From<crate::W<CTBM_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTBM_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Selects component behavior in DeepSleep power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEEPSLEEP_ON_A {
    #[doc = "0: CTB IP disabled off during DeepSleep power mode"]
    DISABLE = 0,
    #[doc = "1: CTB IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
    ENABLE = 1,
}
impl From<DEEPSLEEP_ON_A> for bool {
    #[inline(always)]
    fn from(variant: DEEPSLEEP_ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEEPSLEEP_ON` reader - Selects component behavior in DeepSleep power mode"]
pub type DEEPSLEEP_ON_R = crate::BitReader<DEEPSLEEP_ON_A>;
impl DEEPSLEEP_ON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEEPSLEEP_ON_A {
        match self.bits {
            false => DEEPSLEEP_ON_A::DISABLE,
            true => DEEPSLEEP_ON_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DEEPSLEEP_ON_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DEEPSLEEP_ON_A::ENABLE
    }
}
#[doc = "Field `DEEPSLEEP_ON` writer - Selects component behavior in DeepSleep power mode"]
pub type DEEPSLEEP_ON_W<'a> = crate::BitWriter<'a, u32, CTBM_CTRL_SPEC, DEEPSLEEP_ON_A, 30>;
impl<'a> DEEPSLEEP_ON_W<'a> {
    #[doc = "CTB IP disabled off during DeepSleep power mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DEEPSLEEP_ON_A::DISABLE)
    }
    #[doc = "CTB IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DEEPSLEEP_ON_A::ENABLE)
    }
}
#[doc = "Selects component behavior in DeepSleep power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLED_A {
    #[doc = "0: CTB IP disabled (put analog in power down, open all switches)"]
    DISABLE = 0,
    #[doc = "1: CTB IP enabledCTB IP enabled"]
    ENABLE = 1,
}
impl From<ENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLED` reader - Selects component behavior in DeepSleep power mode"]
pub type ENABLED_R = crate::BitReader<ENABLED_A>;
impl ENABLED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLED_A {
        match self.bits {
            false => ENABLED_A::DISABLE,
            true => ENABLED_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENABLED_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENABLED_A::ENABLE
    }
}
#[doc = "Field `ENABLED` writer - Selects component behavior in DeepSleep power mode"]
pub type ENABLED_W<'a> = crate::BitWriter<'a, u32, CTBM_CTRL_SPEC, ENABLED_A, 31>;
impl<'a> ENABLED_W<'a> {
    #[doc = "CTB IP disabled (put analog in power down, open all switches)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENABLED_A::DISABLE)
    }
    #[doc = "CTB IP enabledCTB IP enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENABLED_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 30 - Selects component behavior in DeepSleep power mode"]
    #[inline(always)]
    pub fn deepsleep_on(&self) -> DEEPSLEEP_ON_R {
        DEEPSLEEP_ON_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Selects component behavior in DeepSleep power mode"]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - Selects component behavior in DeepSleep power mode"]
    #[inline(always)]
    pub fn deepsleep_on(&mut self) -> DEEPSLEEP_ON_W {
        DEEPSLEEP_ON_W::new(self)
    }
    #[doc = "Bit 31 - Selects component behavior in DeepSleep power mode"]
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
#[doc = "Global CTB IP and power control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctbm_ctrl](index.html) module"]
pub struct CTBM_CTRL_SPEC;
impl crate::RegisterSpec for CTBM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctbm_ctrl::R](R) reader structure"]
impl crate::Readable for CTBM_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctbm_ctrl::W](W) writer structure"]
impl crate::Writable for CTBM_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTBM_CTRL to value 0"]
impl crate::Resettable for CTBM_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
