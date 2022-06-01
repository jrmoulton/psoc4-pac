#[doc = "Register `Cy_CONFIG` reader"]
pub struct R(crate::R<CY_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CY_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CY_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CY_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Cy_CONFIG` writer"]
pub struct W(crate::W<CY_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CY_CONFIG_SPEC>;
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
impl From<crate::W<CY_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CY_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Selects speed for the comparator #1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE1_A {
    #[doc = "0: Medium speed"]
    MED = 0,
    #[doc = "1: Fast speed"]
    FAST = 1,
    #[doc = "2: Slow speed"]
    SLOW = 2,
}
impl From<MODE1_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE1` reader - Selects speed for the comparator #1"]
pub type MODE1_R = crate::FieldReader<u8, MODE1_A>;
impl MODE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE1_A> {
        match self.bits {
            0 => Some(MODE1_A::MED),
            1 => Some(MODE1_A::FAST),
            2 => Some(MODE1_A::SLOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MED`"]
    #[inline(always)]
    pub fn is_med(&self) -> bool {
        *self == MODE1_A::MED
    }
    #[doc = "Checks if the value of the field is `FAST`"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == MODE1_A::FAST
    }
    #[doc = "Checks if the value of the field is `SLOW`"]
    #[inline(always)]
    pub fn is_slow(&self) -> bool {
        *self == MODE1_A::SLOW
    }
}
#[doc = "Field `MODE1` writer - Selects speed for the comparator #1"]
pub type MODE1_W<'a> = crate::FieldWriter<'a, u32, CY_CONFIG_SPEC, u8, MODE1_A, 2, 0>;
impl<'a> MODE1_W<'a> {
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn med(self) -> &'a mut W {
        self.variant(MODE1_A::MED)
    }
    #[doc = "Fast speed"]
    #[inline(always)]
    pub fn fast(self) -> &'a mut W {
        self.variant(MODE1_A::FAST)
    }
    #[doc = "Slow speed"]
    #[inline(always)]
    pub fn slow(self) -> &'a mut W {
        self.variant(MODE1_A::SLOW)
    }
}
#[doc = "Selects hysteresis (10mV) for the comparator #1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HYST1_A {
    #[doc = "0: Hysteresis enabled"]
    ENABLE = 0,
    #[doc = "1: Hysteresis disabled"]
    DISABLE = 1,
}
impl From<HYST1_A> for bool {
    #[inline(always)]
    fn from(variant: HYST1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HYST1` reader - Selects hysteresis (10mV) for the comparator #1"]
pub type HYST1_R = crate::BitReader<HYST1_A>;
impl HYST1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HYST1_A {
        match self.bits {
            false => HYST1_A::ENABLE,
            true => HYST1_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HYST1_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HYST1_A::DISABLE
    }
}
#[doc = "Field `HYST1` writer - Selects hysteresis (10mV) for the comparator #1"]
pub type HYST1_W<'a> = crate::BitWriter<'a, u32, CY_CONFIG_SPEC, HYST1_A, 2>;
impl<'a> HYST1_W<'a> {
    #[doc = "Hysteresis enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HYST1_A::ENABLE)
    }
    #[doc = "Hysteresis disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HYST1_A::DISABLE)
    }
}
#[doc = "Sets which edge will trigger an IRQ for the comparator #1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INTTYPE1_A {
    #[doc = "0: Disabled"]
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
#[doc = "Field `INTTYPE1` reader - Sets which edge will trigger an IRQ for the comparator #1"]
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
#[doc = "Field `INTTYPE1` writer - Sets which edge will trigger an IRQ for the comparator #1"]
pub type INTTYPE1_W<'a> = crate::FieldWriterSafe<'a, u32, CY_CONFIG_SPEC, u8, INTTYPE1_A, 2, 4>;
impl<'a> INTTYPE1_W<'a> {
    #[doc = "Disabled"]
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
#[doc = "Field `OUT1` reader - Current output value of the comparator"]
pub type OUT1_R = crate::BitReader<bool>;
#[doc = "Enable comparator #1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE1_A {
    #[doc = "0: Comparator disabled"]
    DISABLE = 0,
    #[doc = "1: Comparator enabled"]
    ENABLE = 1,
}
impl From<ENABLE1_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE1` reader - Enable comparator #1"]
pub type ENABLE1_R = crate::BitReader<ENABLE1_A>;
impl ENABLE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE1_A {
        match self.bits {
            false => ENABLE1_A::DISABLE,
            true => ENABLE1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENABLE1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENABLE1_A::ENABLE
    }
}
#[doc = "Field `ENABLE1` writer - Enable comparator #1"]
pub type ENABLE1_W<'a> = crate::BitWriter<'a, u32, CY_CONFIG_SPEC, ENABLE1_A, 7>;
impl<'a> ENABLE1_W<'a> {
    #[doc = "Comparator disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENABLE1_A::DISABLE)
    }
    #[doc = "Comparator enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENABLE1_A::ENABLE)
    }
}
#[doc = "Selects speed for the comparator #2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE2_A {
    #[doc = "0: Medium speed"]
    MED = 0,
    #[doc = "1: Fast speed"]
    FAST = 1,
    #[doc = "2: Slow speed"]
    SLOW = 2,
}
impl From<MODE2_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE2` reader - Selects speed for the comparator #2"]
pub type MODE2_R = crate::FieldReader<u8, MODE2_A>;
impl MODE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE2_A> {
        match self.bits {
            0 => Some(MODE2_A::MED),
            1 => Some(MODE2_A::FAST),
            2 => Some(MODE2_A::SLOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MED`"]
    #[inline(always)]
    pub fn is_med(&self) -> bool {
        *self == MODE2_A::MED
    }
    #[doc = "Checks if the value of the field is `FAST`"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == MODE2_A::FAST
    }
    #[doc = "Checks if the value of the field is `SLOW`"]
    #[inline(always)]
    pub fn is_slow(&self) -> bool {
        *self == MODE2_A::SLOW
    }
}
#[doc = "Field `MODE2` writer - Selects speed for the comparator #2"]
pub type MODE2_W<'a> = crate::FieldWriter<'a, u32, CY_CONFIG_SPEC, u8, MODE2_A, 2, 8>;
impl<'a> MODE2_W<'a> {
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn med(self) -> &'a mut W {
        self.variant(MODE2_A::MED)
    }
    #[doc = "Fast speed"]
    #[inline(always)]
    pub fn fast(self) -> &'a mut W {
        self.variant(MODE2_A::FAST)
    }
    #[doc = "Slow speed"]
    #[inline(always)]
    pub fn slow(self) -> &'a mut W {
        self.variant(MODE2_A::SLOW)
    }
}
#[doc = "Selects hysteresis (10mV) for the comparator #2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HYST2_A {
    #[doc = "0: Hysteresis enabled"]
    ENABLE = 0,
    #[doc = "1: Hysteresis disabled"]
    DISABLE = 1,
}
impl From<HYST2_A> for bool {
    #[inline(always)]
    fn from(variant: HYST2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HYST2` reader - Selects hysteresis (10mV) for the comparator #2"]
pub type HYST2_R = crate::BitReader<HYST2_A>;
impl HYST2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HYST2_A {
        match self.bits {
            false => HYST2_A::ENABLE,
            true => HYST2_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HYST2_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HYST2_A::DISABLE
    }
}
#[doc = "Field `HYST2` writer - Selects hysteresis (10mV) for the comparator #2"]
pub type HYST2_W<'a> = crate::BitWriter<'a, u32, CY_CONFIG_SPEC, HYST2_A, 10>;
impl<'a> HYST2_W<'a> {
    #[doc = "Hysteresis enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HYST2_A::ENABLE)
    }
    #[doc = "Hysteresis disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HYST2_A::DISABLE)
    }
}
#[doc = "Sets which edge will trigger an IRQ for the comparator #2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INTTYPE2_A {
    #[doc = "0: Disabled"]
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
#[doc = "Field `INTTYPE2` reader - Sets which edge will trigger an IRQ for the comparator #2"]
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
#[doc = "Field `INTTYPE2` writer - Sets which edge will trigger an IRQ for the comparator #2"]
pub type INTTYPE2_W<'a> = crate::FieldWriterSafe<'a, u32, CY_CONFIG_SPEC, u8, INTTYPE2_A, 2, 12>;
impl<'a> INTTYPE2_W<'a> {
    #[doc = "Disabled"]
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
#[doc = "Field `OUT2` reader - Current output value of the comparator #2"]
pub type OUT2_R = crate::BitReader<bool>;
#[doc = "Enable comparator #2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE2_A {
    #[doc = "0: Comparator disabled"]
    DISABLE = 0,
    #[doc = "1: Comparator enabled"]
    ENABLE = 1,
}
impl From<ENABLE2_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE2` reader - Enable comparator #2"]
pub type ENABLE2_R = crate::BitReader<ENABLE2_A>;
impl ENABLE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE2_A {
        match self.bits {
            false => ENABLE2_A::DISABLE,
            true => ENABLE2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENABLE2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENABLE2_A::ENABLE
    }
}
#[doc = "Field `ENABLE2` writer - Enable comparator #2"]
pub type ENABLE2_W<'a> = crate::BitWriter<'a, u32, CY_CONFIG_SPEC, ENABLE2_A, 15>;
impl<'a> ENABLE2_W<'a> {
    #[doc = "Comparator disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENABLE2_A::DISABLE)
    }
    #[doc = "Comparator enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENABLE2_A::ENABLE)
    }
}
#[doc = "Bypass comparator #1 output synchronization for DSI output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSI_BYPASS1_A {
    #[doc = "0: Synchronize (level or pulse)"]
    SYNCHRONIZE = 0,
    #[doc = "1: Bypass (output async)"]
    BYPASS = 1,
}
impl From<DSI_BYPASS1_A> for bool {
    #[inline(always)]
    fn from(variant: DSI_BYPASS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSI_BYPASS1` reader - Bypass comparator #1 output synchronization for DSI output"]
pub type DSI_BYPASS1_R = crate::BitReader<DSI_BYPASS1_A>;
impl DSI_BYPASS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSI_BYPASS1_A {
        match self.bits {
            false => DSI_BYPASS1_A::SYNCHRONIZE,
            true => DSI_BYPASS1_A::BYPASS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONIZE`"]
    #[inline(always)]
    pub fn is_synchronize(&self) -> bool {
        *self == DSI_BYPASS1_A::SYNCHRONIZE
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == DSI_BYPASS1_A::BYPASS
    }
}
#[doc = "Field `DSI_BYPASS1` writer - Bypass comparator #1 output synchronization for DSI output"]
pub type DSI_BYPASS1_W<'a> = crate::BitWriter<'a, u32, CY_CONFIG_SPEC, DSI_BYPASS1_A, 16>;
impl<'a> DSI_BYPASS1_W<'a> {
    #[doc = "Synchronize (level or pulse)"]
    #[inline(always)]
    pub fn synchronize(self) -> &'a mut W {
        self.variant(DSI_BYPASS1_A::SYNCHRONIZE)
    }
    #[doc = "Bypass (output async)"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(DSI_BYPASS1_A::BYPASS)
    }
}
#[doc = "Comparator #1 DSI (trigger) out level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSI_LEVEL1_A {
    #[doc = "0: Pulse"]
    PULSE = 0,
    #[doc = "1: Level"]
    LEVEL = 1,
}
impl From<DSI_LEVEL1_A> for bool {
    #[inline(always)]
    fn from(variant: DSI_LEVEL1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSI_LEVEL1` reader - Comparator #1 DSI (trigger) out level"]
pub type DSI_LEVEL1_R = crate::BitReader<DSI_LEVEL1_A>;
impl DSI_LEVEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSI_LEVEL1_A {
        match self.bits {
            false => DSI_LEVEL1_A::PULSE,
            true => DSI_LEVEL1_A::LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `PULSE`"]
    #[inline(always)]
    pub fn is_pulse(&self) -> bool {
        *self == DSI_LEVEL1_A::PULSE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == DSI_LEVEL1_A::LEVEL
    }
}
#[doc = "Field `DSI_LEVEL1` writer - Comparator #1 DSI (trigger) out level"]
pub type DSI_LEVEL1_W<'a> = crate::BitWriter<'a, u32, CY_CONFIG_SPEC, DSI_LEVEL1_A, 17>;
impl<'a> DSI_LEVEL1_W<'a> {
    #[doc = "Pulse"]
    #[inline(always)]
    pub fn pulse(self) -> &'a mut W {
        self.variant(DSI_LEVEL1_A::PULSE)
    }
    #[doc = "Level"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(DSI_LEVEL1_A::LEVEL)
    }
}
#[doc = "Bypass comparator #2 output synchronization for DSI output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSI_BYPASS2_A {
    #[doc = "0: Synchronize (level or pulse)"]
    SYNCHRONIZE = 0,
    #[doc = "1: Bypass (output async)"]
    BYPASS = 1,
}
impl From<DSI_BYPASS2_A> for bool {
    #[inline(always)]
    fn from(variant: DSI_BYPASS2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSI_BYPASS2` reader - Bypass comparator #2 output synchronization for DSI output"]
pub type DSI_BYPASS2_R = crate::BitReader<DSI_BYPASS2_A>;
impl DSI_BYPASS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSI_BYPASS2_A {
        match self.bits {
            false => DSI_BYPASS2_A::SYNCHRONIZE,
            true => DSI_BYPASS2_A::BYPASS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONIZE`"]
    #[inline(always)]
    pub fn is_synchronize(&self) -> bool {
        *self == DSI_BYPASS2_A::SYNCHRONIZE
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == DSI_BYPASS2_A::BYPASS
    }
}
#[doc = "Field `DSI_BYPASS2` writer - Bypass comparator #2 output synchronization for DSI output"]
pub type DSI_BYPASS2_W<'a> = crate::BitWriter<'a, u32, CY_CONFIG_SPEC, DSI_BYPASS2_A, 20>;
impl<'a> DSI_BYPASS2_W<'a> {
    #[doc = "Synchronize (level or pulse)"]
    #[inline(always)]
    pub fn synchronize(self) -> &'a mut W {
        self.variant(DSI_BYPASS2_A::SYNCHRONIZE)
    }
    #[doc = "Bypass (output async)"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(DSI_BYPASS2_A::BYPASS)
    }
}
#[doc = "Comparator #2 DSI (trigger) out level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSI_LEVEL2_A {
    #[doc = "0: Pulse"]
    PULSE = 0,
    #[doc = "1: Level"]
    LEVEL = 1,
}
impl From<DSI_LEVEL2_A> for bool {
    #[inline(always)]
    fn from(variant: DSI_LEVEL2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSI_LEVEL2` reader - Comparator #2 DSI (trigger) out level"]
pub type DSI_LEVEL2_R = crate::BitReader<DSI_LEVEL2_A>;
impl DSI_LEVEL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSI_LEVEL2_A {
        match self.bits {
            false => DSI_LEVEL2_A::PULSE,
            true => DSI_LEVEL2_A::LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `PULSE`"]
    #[inline(always)]
    pub fn is_pulse(&self) -> bool {
        *self == DSI_LEVEL2_A::PULSE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == DSI_LEVEL2_A::LEVEL
    }
}
#[doc = "Field `DSI_LEVEL2` writer - Comparator #2 DSI (trigger) out level"]
pub type DSI_LEVEL2_W<'a> = crate::BitWriter<'a, u32, CY_CONFIG_SPEC, DSI_LEVEL2_A, 21>;
impl<'a> DSI_LEVEL2_W<'a> {
    #[doc = "Pulse"]
    #[inline(always)]
    pub fn pulse(self) -> &'a mut W {
        self.variant(DSI_LEVEL2_A::PULSE)
    }
    #[doc = "Level"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(DSI_LEVEL2_A::LEVEL)
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects speed for the comparator #1"]
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Selects hysteresis (10mV) for the comparator #1"]
    #[inline(always)]
    pub fn hyst1(&self) -> HYST1_R {
        HYST1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Sets which edge will trigger an IRQ for the comparator #1"]
    #[inline(always)]
    pub fn inttype1(&self) -> INTTYPE1_R {
        INTTYPE1_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Current output value of the comparator"]
    #[inline(always)]
    pub fn out1(&self) -> OUT1_R {
        OUT1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable comparator #1"]
    #[inline(always)]
    pub fn enable1(&self) -> ENABLE1_R {
        ENABLE1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Selects speed for the comparator #2"]
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Selects hysteresis (10mV) for the comparator #2"]
    #[inline(always)]
    pub fn hyst2(&self) -> HYST2_R {
        HYST2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Sets which edge will trigger an IRQ for the comparator #2"]
    #[inline(always)]
    pub fn inttype2(&self) -> INTTYPE2_R {
        INTTYPE2_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Current output value of the comparator #2"]
    #[inline(always)]
    pub fn out2(&self) -> OUT2_R {
        OUT2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable comparator #2"]
    #[inline(always)]
    pub fn enable2(&self) -> ENABLE2_R {
        ENABLE2_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Bypass comparator #1 output synchronization for DSI output"]
    #[inline(always)]
    pub fn dsi_bypass1(&self) -> DSI_BYPASS1_R {
        DSI_BYPASS1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Comparator #1 DSI (trigger) out level"]
    #[inline(always)]
    pub fn dsi_level1(&self) -> DSI_LEVEL1_R {
        DSI_LEVEL1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - Bypass comparator #2 output synchronization for DSI output"]
    #[inline(always)]
    pub fn dsi_bypass2(&self) -> DSI_BYPASS2_R {
        DSI_BYPASS2_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Comparator #2 DSI (trigger) out level"]
    #[inline(always)]
    pub fn dsi_level2(&self) -> DSI_LEVEL2_R {
        DSI_LEVEL2_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects speed for the comparator #1"]
    #[inline(always)]
    pub fn mode1(&mut self) -> MODE1_W {
        MODE1_W::new(self)
    }
    #[doc = "Bit 2 - Selects hysteresis (10mV) for the comparator #1"]
    #[inline(always)]
    pub fn hyst1(&mut self) -> HYST1_W {
        HYST1_W::new(self)
    }
    #[doc = "Bits 4:5 - Sets which edge will trigger an IRQ for the comparator #1"]
    #[inline(always)]
    pub fn inttype1(&mut self) -> INTTYPE1_W {
        INTTYPE1_W::new(self)
    }
    #[doc = "Bit 7 - Enable comparator #1"]
    #[inline(always)]
    pub fn enable1(&mut self) -> ENABLE1_W {
        ENABLE1_W::new(self)
    }
    #[doc = "Bits 8:9 - Selects speed for the comparator #2"]
    #[inline(always)]
    pub fn mode2(&mut self) -> MODE2_W {
        MODE2_W::new(self)
    }
    #[doc = "Bit 10 - Selects hysteresis (10mV) for the comparator #2"]
    #[inline(always)]
    pub fn hyst2(&mut self) -> HYST2_W {
        HYST2_W::new(self)
    }
    #[doc = "Bits 12:13 - Sets which edge will trigger an IRQ for the comparator #2"]
    #[inline(always)]
    pub fn inttype2(&mut self) -> INTTYPE2_W {
        INTTYPE2_W::new(self)
    }
    #[doc = "Bit 15 - Enable comparator #2"]
    #[inline(always)]
    pub fn enable2(&mut self) -> ENABLE2_W {
        ENABLE2_W::new(self)
    }
    #[doc = "Bit 16 - Bypass comparator #1 output synchronization for DSI output"]
    #[inline(always)]
    pub fn dsi_bypass1(&mut self) -> DSI_BYPASS1_W {
        DSI_BYPASS1_W::new(self)
    }
    #[doc = "Bit 17 - Comparator #1 DSI (trigger) out level"]
    #[inline(always)]
    pub fn dsi_level1(&mut self) -> DSI_LEVEL1_W {
        DSI_LEVEL1_W::new(self)
    }
    #[doc = "Bit 20 - Bypass comparator #2 output synchronization for DSI output"]
    #[inline(always)]
    pub fn dsi_bypass2(&mut self) -> DSI_BYPASS2_W {
        DSI_BYPASS2_W::new(self)
    }
    #[doc = "Bit 21 - Comparator #2 DSI (trigger) out level"]
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
#[doc = "LPCOMP configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cy_config](index.html) module"]
pub struct CY_CONFIG_SPEC;
impl crate::RegisterSpec for CY_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cy_config::R](R) reader structure"]
impl crate::Readable for CY_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cy_config::W](W) writer structure"]
impl crate::Writable for CY_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Cy_CONFIG to value 0"]
impl crate::Resettable for CY_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
