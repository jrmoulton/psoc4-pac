#[doc = "Register `CLK_SELECT` reader"]
pub struct R(crate::R<CLK_SELECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_SELECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_SELECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_SELECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_SELECT` writer"]
pub struct W(crate::W<CLK_SELECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_SELECT_SPEC>;
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
impl From<crate::W<CLK_SELECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_SELECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Selects a source for clk_hf and dsi_in\\[0\\]. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HFCLK_SEL_A {
    #[doc = "0: IMO - Internal R/C Oscillator"]
    IMO = 0,
    #[doc = "1: EXTCLK - External Clock Pin"]
    EXTCLK = 1,
    #[doc = "2: ECO - External-Crystal Oscillator or PLL subsystem output"]
    ECO = 2,
}
impl From<HFCLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: HFCLK_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HFCLK_SEL` reader - Selects a source for clk_hf and dsi_in\\[0\\]. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior."]
pub type HFCLK_SEL_R = crate::FieldReader<u8, HFCLK_SEL_A>;
impl HFCLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HFCLK_SEL_A> {
        match self.bits {
            0 => Some(HFCLK_SEL_A::IMO),
            1 => Some(HFCLK_SEL_A::EXTCLK),
            2 => Some(HFCLK_SEL_A::ECO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IMO`"]
    #[inline(always)]
    pub fn is_imo(&self) -> bool {
        *self == HFCLK_SEL_A::IMO
    }
    #[doc = "Checks if the value of the field is `EXTCLK`"]
    #[inline(always)]
    pub fn is_extclk(&self) -> bool {
        *self == HFCLK_SEL_A::EXTCLK
    }
    #[doc = "Checks if the value of the field is `ECO`"]
    #[inline(always)]
    pub fn is_eco(&self) -> bool {
        *self == HFCLK_SEL_A::ECO
    }
}
#[doc = "Field `HFCLK_SEL` writer - Selects a source for clk_hf and dsi_in\\[0\\]. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior."]
pub type HFCLK_SEL_W<'a> = crate::FieldWriter<'a, u32, CLK_SELECT_SPEC, u8, HFCLK_SEL_A, 2, 0>;
impl<'a> HFCLK_SEL_W<'a> {
    #[doc = "IMO - Internal R/C Oscillator"]
    #[inline(always)]
    pub fn imo(self) -> &'a mut W {
        self.variant(HFCLK_SEL_A::IMO)
    }
    #[doc = "EXTCLK - External Clock Pin"]
    #[inline(always)]
    pub fn extclk(self) -> &'a mut W {
        self.variant(HFCLK_SEL_A::EXTCLK)
    }
    #[doc = "ECO - External-Crystal Oscillator or PLL subsystem output"]
    #[inline(always)]
    pub fn eco(self) -> &'a mut W {
        self.variant(HFCLK_SEL_A::ECO)
    }
}
#[doc = "Selects clk_hf predivider value.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HFCLK_DIV_A {
    #[doc = "0: Transparent mode, feed through selected clock source w/o dividing."]
    NO_DIV = 0,
    #[doc = "1: Divide selected clock source by 2"]
    DIV_BY_2 = 1,
    #[doc = "2: Divide selected clock source by 4"]
    DIV_BY_4 = 2,
    #[doc = "3: Divide selected clock source by 8"]
    DIV_BY_8 = 3,
}
impl From<HFCLK_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: HFCLK_DIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HFCLK_DIV` reader - Selects clk_hf predivider value."]
pub type HFCLK_DIV_R = crate::FieldReader<u8, HFCLK_DIV_A>;
impl HFCLK_DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFCLK_DIV_A {
        match self.bits {
            0 => HFCLK_DIV_A::NO_DIV,
            1 => HFCLK_DIV_A::DIV_BY_2,
            2 => HFCLK_DIV_A::DIV_BY_4,
            3 => HFCLK_DIV_A::DIV_BY_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_DIV`"]
    #[inline(always)]
    pub fn is_no_div(&self) -> bool {
        *self == HFCLK_DIV_A::NO_DIV
    }
    #[doc = "Checks if the value of the field is `DIV_BY_2`"]
    #[inline(always)]
    pub fn is_div_by_2(&self) -> bool {
        *self == HFCLK_DIV_A::DIV_BY_2
    }
    #[doc = "Checks if the value of the field is `DIV_BY_4`"]
    #[inline(always)]
    pub fn is_div_by_4(&self) -> bool {
        *self == HFCLK_DIV_A::DIV_BY_4
    }
    #[doc = "Checks if the value of the field is `DIV_BY_8`"]
    #[inline(always)]
    pub fn is_div_by_8(&self) -> bool {
        *self == HFCLK_DIV_A::DIV_BY_8
    }
}
#[doc = "Field `HFCLK_DIV` writer - Selects clk_hf predivider value."]
pub type HFCLK_DIV_W<'a> = crate::FieldWriterSafe<'a, u32, CLK_SELECT_SPEC, u8, HFCLK_DIV_A, 2, 2>;
impl<'a> HFCLK_DIV_W<'a> {
    #[doc = "Transparent mode, feed through selected clock source w/o dividing."]
    #[inline(always)]
    pub fn no_div(self) -> &'a mut W {
        self.variant(HFCLK_DIV_A::NO_DIV)
    }
    #[doc = "Divide selected clock source by 2"]
    #[inline(always)]
    pub fn div_by_2(self) -> &'a mut W {
        self.variant(HFCLK_DIV_A::DIV_BY_2)
    }
    #[doc = "Divide selected clock source by 4"]
    #[inline(always)]
    pub fn div_by_4(self) -> &'a mut W {
        self.variant(HFCLK_DIV_A::DIV_BY_4)
    }
    #[doc = "Divide selected clock source by 8"]
    #[inline(always)]
    pub fn div_by_8(self) -> &'a mut W {
        self.variant(HFCLK_DIV_A::DIV_BY_8)
    }
}
#[doc = "Selects clock source for charge pump clock. This clock is not guaranteed to be glitch free when changing any of its sources or settings.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PUMP_SEL_A {
    #[doc = "0: No clock, connect to gnd"]
    GND = 0,
    #[doc = "1: Use main IMO output"]
    IMO = 1,
    #[doc = "2: Use clk_hf (using selected source after predivider but before prescaler)"]
    HFCLK = 2,
}
impl From<PUMP_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PUMP_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PUMP_SEL` reader - Selects clock source for charge pump clock. This clock is not guaranteed to be glitch free when changing any of its sources or settings."]
pub type PUMP_SEL_R = crate::FieldReader<u8, PUMP_SEL_A>;
impl PUMP_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PUMP_SEL_A> {
        match self.bits {
            0 => Some(PUMP_SEL_A::GND),
            1 => Some(PUMP_SEL_A::IMO),
            2 => Some(PUMP_SEL_A::HFCLK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GND`"]
    #[inline(always)]
    pub fn is_gnd(&self) -> bool {
        *self == PUMP_SEL_A::GND
    }
    #[doc = "Checks if the value of the field is `IMO`"]
    #[inline(always)]
    pub fn is_imo(&self) -> bool {
        *self == PUMP_SEL_A::IMO
    }
    #[doc = "Checks if the value of the field is `HFCLK`"]
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        *self == PUMP_SEL_A::HFCLK
    }
}
#[doc = "Field `PUMP_SEL` writer - Selects clock source for charge pump clock. This clock is not guaranteed to be glitch free when changing any of its sources or settings."]
pub type PUMP_SEL_W<'a> = crate::FieldWriter<'a, u32, CLK_SELECT_SPEC, u8, PUMP_SEL_A, 2, 4>;
impl<'a> PUMP_SEL_W<'a> {
    #[doc = "No clock, connect to gnd"]
    #[inline(always)]
    pub fn gnd(self) -> &'a mut W {
        self.variant(PUMP_SEL_A::GND)
    }
    #[doc = "Use main IMO output"]
    #[inline(always)]
    pub fn imo(self) -> &'a mut W {
        self.variant(PUMP_SEL_A::IMO)
    }
    #[doc = "Use clk_hf (using selected source after predivider but before prescaler)"]
    #[inline(always)]
    pub fn hfclk(self) -> &'a mut W {
        self.variant(PUMP_SEL_A::HFCLK)
    }
}
#[doc = "Select clk_sys prescaler value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCLK_DIV_A {
    #[doc = "0: clk_sys= clk_hf/1"]
    NO_DIV = 0,
    #[doc = "1: clk_sys= clk_hf/2"]
    DIV_BY_2 = 1,
    #[doc = "2: clk_sys= clk_hf/4"]
    DIV_BY_4 = 2,
    #[doc = "3: clk_sys= clk_hf/8"]
    DIV_BY_8 = 3,
}
impl From<SYSCLK_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCLK_DIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYSCLK_DIV` reader - Select clk_sys prescaler value."]
pub type SYSCLK_DIV_R = crate::FieldReader<u8, SYSCLK_DIV_A>;
impl SYSCLK_DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSCLK_DIV_A {
        match self.bits {
            0 => SYSCLK_DIV_A::NO_DIV,
            1 => SYSCLK_DIV_A::DIV_BY_2,
            2 => SYSCLK_DIV_A::DIV_BY_4,
            3 => SYSCLK_DIV_A::DIV_BY_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_DIV`"]
    #[inline(always)]
    pub fn is_no_div(&self) -> bool {
        *self == SYSCLK_DIV_A::NO_DIV
    }
    #[doc = "Checks if the value of the field is `DIV_BY_2`"]
    #[inline(always)]
    pub fn is_div_by_2(&self) -> bool {
        *self == SYSCLK_DIV_A::DIV_BY_2
    }
    #[doc = "Checks if the value of the field is `DIV_BY_4`"]
    #[inline(always)]
    pub fn is_div_by_4(&self) -> bool {
        *self == SYSCLK_DIV_A::DIV_BY_4
    }
    #[doc = "Checks if the value of the field is `DIV_BY_8`"]
    #[inline(always)]
    pub fn is_div_by_8(&self) -> bool {
        *self == SYSCLK_DIV_A::DIV_BY_8
    }
}
#[doc = "Field `SYSCLK_DIV` writer - Select clk_sys prescaler value."]
pub type SYSCLK_DIV_W<'a> =
    crate::FieldWriterSafe<'a, u32, CLK_SELECT_SPEC, u8, SYSCLK_DIV_A, 2, 6>;
impl<'a> SYSCLK_DIV_W<'a> {
    #[doc = "clk_sys= clk_hf/1"]
    #[inline(always)]
    pub fn no_div(self) -> &'a mut W {
        self.variant(SYSCLK_DIV_A::NO_DIV)
    }
    #[doc = "clk_sys= clk_hf/2"]
    #[inline(always)]
    pub fn div_by_2(self) -> &'a mut W {
        self.variant(SYSCLK_DIV_A::DIV_BY_2)
    }
    #[doc = "clk_sys= clk_hf/4"]
    #[inline(always)]
    pub fn div_by_4(self) -> &'a mut W {
        self.variant(SYSCLK_DIV_A::DIV_BY_4)
    }
    #[doc = "clk_sys= clk_hf/8"]
    #[inline(always)]
    pub fn div_by_8(self) -> &'a mut W {
        self.variant(SYSCLK_DIV_A::DIV_BY_8)
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects a source for clk_hf and dsi_in\\[0\\]. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior."]
    #[inline(always)]
    pub fn hfclk_sel(&self) -> HFCLK_SEL_R {
        HFCLK_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Selects clk_hf predivider value."]
    #[inline(always)]
    pub fn hfclk_div(&self) -> HFCLK_DIV_R {
        HFCLK_DIV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Selects clock source for charge pump clock. This clock is not guaranteed to be glitch free when changing any of its sources or settings."]
    #[inline(always)]
    pub fn pump_sel(&self) -> PUMP_SEL_R {
        PUMP_SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Select clk_sys prescaler value."]
    #[inline(always)]
    pub fn sysclk_div(&self) -> SYSCLK_DIV_R {
        SYSCLK_DIV_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects a source for clk_hf and dsi_in\\[0\\]. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior."]
    #[inline(always)]
    pub fn hfclk_sel(&mut self) -> HFCLK_SEL_W {
        HFCLK_SEL_W::new(self)
    }
    #[doc = "Bits 2:3 - Selects clk_hf predivider value."]
    #[inline(always)]
    pub fn hfclk_div(&mut self) -> HFCLK_DIV_W {
        HFCLK_DIV_W::new(self)
    }
    #[doc = "Bits 4:5 - Selects clock source for charge pump clock. This clock is not guaranteed to be glitch free when changing any of its sources or settings."]
    #[inline(always)]
    pub fn pump_sel(&mut self) -> PUMP_SEL_W {
        PUMP_SEL_W::new(self)
    }
    #[doc = "Bits 6:7 - Select clk_sys prescaler value."]
    #[inline(always)]
    pub fn sysclk_div(&mut self) -> SYSCLK_DIV_W {
        SYSCLK_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_select](index.html) module"]
pub struct CLK_SELECT_SPEC;
impl crate::RegisterSpec for CLK_SELECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_select::R](R) reader structure"]
impl crate::Readable for CLK_SELECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_select::W](W) writer structure"]
impl crate::Writable for CLK_SELECT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_SELECT to value 0x08"]
impl crate::Resettable for CLK_SELECT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
