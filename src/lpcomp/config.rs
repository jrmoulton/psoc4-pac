#[doc = "Register `CONFIG` reader"]
pub struct R(crate::R<CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG` writer"]
pub struct W(crate::W<CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_SPEC>;
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
impl From<crate::W<CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Operating mode for the comparator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE1_A {
    #[doc = "0: Slow operating mode (uses less power, <50uA)"]
    SLOW = 0,
    #[doc = "1: Fast operating mode (uses more power, <400uA)"]
    FAST = 1,
    #[doc = "2: Ultra low power operting mode (uses ~2-4uA)"]
    ULP = 2,
}
impl From<MODE1_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE1` reader - Operating mode for the comparator"]
pub type MODE1_R = crate::FieldReader<u8, MODE1_A>;
impl MODE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE1_A> {
        match self.bits {
            0 => Some(MODE1_A::SLOW),
            1 => Some(MODE1_A::FAST),
            2 => Some(MODE1_A::ULP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SLOW`"]
    #[inline(always)]
    pub fn is_slow(&self) -> bool {
        *self == MODE1_A::SLOW
    }
    #[doc = "Checks if the value of the field is `FAST`"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == MODE1_A::FAST
    }
    #[doc = "Checks if the value of the field is `ULP`"]
    #[inline(always)]
    pub fn is_ulp(&self) -> bool {
        *self == MODE1_A::ULP
    }
}
#[doc = "Field `MODE1` writer - Operating mode for the comparator"]
pub type MODE1_W<'a> = crate::FieldWriter<'a, u32, CONFIG_SPEC, u8, MODE1_A, 2, 0>;
impl<'a> MODE1_W<'a> {
    #[doc = "Slow operating mode (uses less power, <50uA)"]
    #[inline(always)]
    pub fn slow(self) -> &'a mut W {
        self.variant(MODE1_A::SLOW)
    }
    #[doc = "Fast operating mode (uses more power, <400uA)"]
    #[inline(always)]
    pub fn fast(self) -> &'a mut W {
        self.variant(MODE1_A::FAST)
    }
    #[doc = "Ultra low power operting mode (uses ~2-4uA)"]
    #[inline(always)]
    pub fn ulp(self) -> &'a mut W {
        self.variant(MODE1_A::ULP)
    }
}
#[doc = "Field `HYST1` reader - Add 10mV hysteresis to the comparator - 0: Enable Hysteresis - 1: Disable Hysteresis"]
pub type HYST1_R = crate::BitReader<bool>;
#[doc = "Field `HYST1` writer - Add 10mV hysteresis to the comparator - 0: Enable Hysteresis - 1: Disable Hysteresis"]
pub type HYST1_W<'a> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, 2>;
#[doc = "Field `FILTER1` reader - N/A"]
pub type FILTER1_R = crate::BitReader<bool>;
#[doc = "Field `FILTER1` writer - N/A"]
pub type FILTER1_W<'a> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, 3>;
#[doc = "Sets which edge will trigger an IRQ\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INTTYPE1_A {
    #[doc = "0: Disabled, no interrupts will be detected"]
    DISABLE = 0,
    #[doc = "1: Rising edge"]
    RISING = 1,
    #[doc = "2: Falling edge"]
    FALLING = 2,
    #[doc = "3: Both rising and falling edges"]
    BOTH = 3,
}
impl From<INTTYPE1_A> for u8 {
    #[inline(always)]
    fn from(variant: INTTYPE1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INTTYPE1` reader - Sets which edge will trigger an IRQ"]
pub type INTTYPE1_R = crate::FieldReader<u8, INTTYPE1_A>;
impl INTTYPE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTTYPE1_A {
        match self.bits {
            0 => INTTYPE1_A::DISABLE,
            1 => INTTYPE1_A::RISING,
            2 => INTTYPE1_A::FALLING,
            3 => INTTYPE1_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == INTTYPE1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == INTTYPE1_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == INTTYPE1_A::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == INTTYPE1_A::BOTH
    }
}
#[doc = "Field `INTTYPE1` writer - Sets which edge will trigger an IRQ"]
pub type INTTYPE1_W<'a> = crate::FieldWriterSafe<'a, u32, CONFIG_SPEC, u8, INTTYPE1_A, 2, 4>;
impl<'a> INTTYPE1_W<'a> {
    #[doc = "Disabled, no interrupts will be detected"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(INTTYPE1_A::DISABLE)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(INTTYPE1_A::RISING)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(INTTYPE1_A::FALLING)
    }
    #[doc = "Both rising and falling edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(INTTYPE1_A::BOTH)
    }
}
#[doc = "Field `OUT1` reader - Current output value of the comparator."]
pub type OUT1_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE1` reader - Enable comparator #1"]
pub type ENABLE1_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE1` writer - Enable comparator #1"]
pub type ENABLE1_W<'a> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, 7>;
#[doc = "Operating mode for the comparator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE2_A {
    #[doc = "0: Slow operating mode (uses less power, <50uA)"]
    SLOW = 0,
    #[doc = "1: Fast operating mode (uses more power, <400uA)"]
    FAST = 1,
    #[doc = "2: Ultra low power operting mode (uses ~2-4uA)"]
    ULP = 2,
}
impl From<MODE2_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE2` reader - Operating mode for the comparator"]
pub type MODE2_R = crate::FieldReader<u8, MODE2_A>;
impl MODE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE2_A> {
        match self.bits {
            0 => Some(MODE2_A::SLOW),
            1 => Some(MODE2_A::FAST),
            2 => Some(MODE2_A::ULP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SLOW`"]
    #[inline(always)]
    pub fn is_slow(&self) -> bool {
        *self == MODE2_A::SLOW
    }
    #[doc = "Checks if the value of the field is `FAST`"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == MODE2_A::FAST
    }
    #[doc = "Checks if the value of the field is `ULP`"]
    #[inline(always)]
    pub fn is_ulp(&self) -> bool {
        *self == MODE2_A::ULP
    }
}
#[doc = "Field `MODE2` writer - Operating mode for the comparator"]
pub type MODE2_W<'a> = crate::FieldWriter<'a, u32, CONFIG_SPEC, u8, MODE2_A, 2, 8>;
impl<'a> MODE2_W<'a> {
    #[doc = "Slow operating mode (uses less power, <50uA)"]
    #[inline(always)]
    pub fn slow(self) -> &'a mut W {
        self.variant(MODE2_A::SLOW)
    }
    #[doc = "Fast operating mode (uses more power, <400uA)"]
    #[inline(always)]
    pub fn fast(self) -> &'a mut W {
        self.variant(MODE2_A::FAST)
    }
    #[doc = "Ultra low power operting mode (uses ~2-4uA)"]
    #[inline(always)]
    pub fn ulp(self) -> &'a mut W {
        self.variant(MODE2_A::ULP)
    }
}
#[doc = "Field `HYST2` reader - Add 10mV hysteresis to the comparator - 0: Enable Hysteresis - 1: Disable Hysteresis"]
pub type HYST2_R = crate::BitReader<bool>;
#[doc = "Field `HYST2` writer - Add 10mV hysteresis to the comparator - 0: Enable Hysteresis - 1: Disable Hysteresis"]
pub type HYST2_W<'a> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, 10>;
#[doc = "Field `FILTER2` reader - N/A"]
pub type FILTER2_R = crate::BitReader<bool>;
#[doc = "Field `FILTER2` writer - N/A"]
pub type FILTER2_W<'a> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, 11>;
#[doc = "Sets which edge will trigger an IRQ\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INTTYPE2_A {
    #[doc = "0: Disabled, no interrupts will be detected"]
    DISABLE = 0,
    #[doc = "1: Rising edge"]
    RISING = 1,
    #[doc = "2: Falling edge"]
    FALLING = 2,
    #[doc = "3: Both rising and falling edges"]
    BOTH = 3,
}
impl From<INTTYPE2_A> for u8 {
    #[inline(always)]
    fn from(variant: INTTYPE2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INTTYPE2` reader - Sets which edge will trigger an IRQ"]
pub type INTTYPE2_R = crate::FieldReader<u8, INTTYPE2_A>;
impl INTTYPE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTTYPE2_A {
        match self.bits {
            0 => INTTYPE2_A::DISABLE,
            1 => INTTYPE2_A::RISING,
            2 => INTTYPE2_A::FALLING,
            3 => INTTYPE2_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == INTTYPE2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == INTTYPE2_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == INTTYPE2_A::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == INTTYPE2_A::BOTH
    }
}
#[doc = "Field `INTTYPE2` writer - Sets which edge will trigger an IRQ"]
pub type INTTYPE2_W<'a> = crate::FieldWriterSafe<'a, u32, CONFIG_SPEC, u8, INTTYPE2_A, 2, 12>;
impl<'a> INTTYPE2_W<'a> {
    #[doc = "Disabled, no interrupts will be detected"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(INTTYPE2_A::DISABLE)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(INTTYPE2_A::RISING)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(INTTYPE2_A::FALLING)
    }
    #[doc = "Both rising and falling edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(INTTYPE2_A::BOTH)
    }
}
#[doc = "Field `OUT2` reader - Current output value of the comparator."]
pub type OUT2_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE2` reader - Enable comparator #2"]
pub type ENABLE2_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE2` writer - Enable comparator #2"]
pub type ENABLE2_W<'a> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, 15>;
#[doc = "Field `DSI_BYPASS1` reader - Opamp1 bypass comparator output synchronization for DSI output: 0=synchronize (level or pulse), 1=bypass (output async) Note that in DeepSleep mode this bit needs to be set to observe the DSI output on the dedicated pin."]
pub type DSI_BYPASS1_R = crate::BitReader<bool>;
#[doc = "Field `DSI_BYPASS1` writer - Opamp1 bypass comparator output synchronization for DSI output: 0=synchronize (level or pulse), 1=bypass (output async) Note that in DeepSleep mode this bit needs to be set to observe the DSI output on the dedicated pin."]
pub type DSI_BYPASS1_W<'a> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, 16>;
#[doc = "Field `DSI_LEVEL1` reader - Opamp1 comparator DSI (trigger) out level : 0=pulse, 1=level"]
pub type DSI_LEVEL1_R = crate::BitReader<bool>;
#[doc = "Field `DSI_LEVEL1` writer - Opamp1 comparator DSI (trigger) out level : 0=pulse, 1=level"]
pub type DSI_LEVEL1_W<'a> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, 17>;
#[doc = "Field `DSI_BYPASS2` reader - Opamp2 bypass comparator output synchronization for DSI output: 0=synchronize (level or pulse), 1=bypass (output async) Note that in DeepSleep mode this bit needs to be set to observe the DSI output on the dedicated pin."]
pub type DSI_BYPASS2_R = crate::BitReader<bool>;
#[doc = "Field `DSI_BYPASS2` writer - Opamp2 bypass comparator output synchronization for DSI output: 0=synchronize (level or pulse), 1=bypass (output async) Note that in DeepSleep mode this bit needs to be set to observe the DSI output on the dedicated pin."]
pub type DSI_BYPASS2_W<'a> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, 20>;
#[doc = "Field `DSI_LEVEL2` reader - Opamp2 comparator DSI (trigger) out level : 0=pulse, 1=level"]
pub type DSI_LEVEL2_R = crate::BitReader<bool>;
#[doc = "Field `DSI_LEVEL2` writer - Opamp2 comparator DSI (trigger) out level : 0=pulse, 1=level"]
pub type DSI_LEVEL2_W<'a> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, 21>;
impl R {
    #[doc = "Bits 0:1 - Operating mode for the comparator"]
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Add 10mV hysteresis to the comparator - 0: Enable Hysteresis - 1: Disable Hysteresis"]
    #[inline(always)]
    pub fn hyst1(&self) -> HYST1_R {
        HYST1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn filter1(&self) -> FILTER1_R {
        FILTER1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Sets which edge will trigger an IRQ"]
    #[inline(always)]
    pub fn inttype1(&self) -> INTTYPE1_R {
        INTTYPE1_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Current output value of the comparator."]
    #[inline(always)]
    pub fn out1(&self) -> OUT1_R {
        OUT1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable comparator #1"]
    #[inline(always)]
    pub fn enable1(&self) -> ENABLE1_R {
        ENABLE1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Operating mode for the comparator"]
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Add 10mV hysteresis to the comparator - 0: Enable Hysteresis - 1: Disable Hysteresis"]
    #[inline(always)]
    pub fn hyst2(&self) -> HYST2_R {
        HYST2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - N/A"]
    #[inline(always)]
    pub fn filter2(&self) -> FILTER2_R {
        FILTER2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Sets which edge will trigger an IRQ"]
    #[inline(always)]
    pub fn inttype2(&self) -> INTTYPE2_R {
        INTTYPE2_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Current output value of the comparator."]
    #[inline(always)]
    pub fn out2(&self) -> OUT2_R {
        OUT2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable comparator #2"]
    #[inline(always)]
    pub fn enable2(&self) -> ENABLE2_R {
        ENABLE2_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Opamp1 bypass comparator output synchronization for DSI output: 0=synchronize (level or pulse), 1=bypass (output async) Note that in DeepSleep mode this bit needs to be set to observe the DSI output on the dedicated pin."]
    #[inline(always)]
    pub fn dsi_bypass1(&self) -> DSI_BYPASS1_R {
        DSI_BYPASS1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Opamp1 comparator DSI (trigger) out level : 0=pulse, 1=level"]
    #[inline(always)]
    pub fn dsi_level1(&self) -> DSI_LEVEL1_R {
        DSI_LEVEL1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - Opamp2 bypass comparator output synchronization for DSI output: 0=synchronize (level or pulse), 1=bypass (output async) Note that in DeepSleep mode this bit needs to be set to observe the DSI output on the dedicated pin."]
    #[inline(always)]
    pub fn dsi_bypass2(&self) -> DSI_BYPASS2_R {
        DSI_BYPASS2_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Opamp2 comparator DSI (trigger) out level : 0=pulse, 1=level"]
    #[inline(always)]
    pub fn dsi_level2(&self) -> DSI_LEVEL2_R {
        DSI_LEVEL2_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Operating mode for the comparator"]
    #[inline(always)]
    pub fn mode1(&mut self) -> MODE1_W {
        MODE1_W::new(self)
    }
    #[doc = "Bit 2 - Add 10mV hysteresis to the comparator - 0: Enable Hysteresis - 1: Disable Hysteresis"]
    #[inline(always)]
    pub fn hyst1(&mut self) -> HYST1_W {
        HYST1_W::new(self)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn filter1(&mut self) -> FILTER1_W {
        FILTER1_W::new(self)
    }
    #[doc = "Bits 4:5 - Sets which edge will trigger an IRQ"]
    #[inline(always)]
    pub fn inttype1(&mut self) -> INTTYPE1_W {
        INTTYPE1_W::new(self)
    }
    #[doc = "Bit 7 - Enable comparator #1"]
    #[inline(always)]
    pub fn enable1(&mut self) -> ENABLE1_W {
        ENABLE1_W::new(self)
    }
    #[doc = "Bits 8:9 - Operating mode for the comparator"]
    #[inline(always)]
    pub fn mode2(&mut self) -> MODE2_W {
        MODE2_W::new(self)
    }
    #[doc = "Bit 10 - Add 10mV hysteresis to the comparator - 0: Enable Hysteresis - 1: Disable Hysteresis"]
    #[inline(always)]
    pub fn hyst2(&mut self) -> HYST2_W {
        HYST2_W::new(self)
    }
    #[doc = "Bit 11 - N/A"]
    #[inline(always)]
    pub fn filter2(&mut self) -> FILTER2_W {
        FILTER2_W::new(self)
    }
    #[doc = "Bits 12:13 - Sets which edge will trigger an IRQ"]
    #[inline(always)]
    pub fn inttype2(&mut self) -> INTTYPE2_W {
        INTTYPE2_W::new(self)
    }
    #[doc = "Bit 15 - Enable comparator #2"]
    #[inline(always)]
    pub fn enable2(&mut self) -> ENABLE2_W {
        ENABLE2_W::new(self)
    }
    #[doc = "Bit 16 - Opamp1 bypass comparator output synchronization for DSI output: 0=synchronize (level or pulse), 1=bypass (output async) Note that in DeepSleep mode this bit needs to be set to observe the DSI output on the dedicated pin."]
    #[inline(always)]
    pub fn dsi_bypass1(&mut self) -> DSI_BYPASS1_W {
        DSI_BYPASS1_W::new(self)
    }
    #[doc = "Bit 17 - Opamp1 comparator DSI (trigger) out level : 0=pulse, 1=level"]
    #[inline(always)]
    pub fn dsi_level1(&mut self) -> DSI_LEVEL1_W {
        DSI_LEVEL1_W::new(self)
    }
    #[doc = "Bit 20 - Opamp2 bypass comparator output synchronization for DSI output: 0=synchronize (level or pulse), 1=bypass (output async) Note that in DeepSleep mode this bit needs to be set to observe the DSI output on the dedicated pin."]
    #[inline(always)]
    pub fn dsi_bypass2(&mut self) -> DSI_BYPASS2_W {
        DSI_BYPASS2_W::new(self)
    }
    #[doc = "Bit 21 - Opamp2 comparator DSI (trigger) out level : 0=pulse, 1=level"]
    #[inline(always)]
    pub fn dsi_level2(&mut self) -> DSI_LEVEL2_W {
        DSI_LEVEL2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPCOMP Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](index.html) module"]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config::R](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config::W](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONFIG to value 0"]
impl crate::Resettable for CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
