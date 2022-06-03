#[doc = "Register `PWR_DDFT_SELECT` reader"]
pub struct R(crate::R<PWR_DDFT_SELECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_DDFT_SELECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_DDFT_SELECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_DDFT_SELECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_DDFT_SELECT` writer"]
pub struct W(crate::W<PWR_DDFT_SELECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_DDFT_SELECT_SPEC>;
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
impl From<crate::W<PWR_DDFT_SELECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_DDFT_SELECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Select signal for power DDFT output #0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DDFT0_SEL_A {
    #[doc = "0: wakeup"]
    WAKEUP = 0,
    #[doc = "1: awake"]
    AWAKE = 1,
    #[doc = "2: act_power_en"]
    ACT_POWER_EN = 2,
    #[doc = "3: act_power_up"]
    ACT_POWER_UP = 3,
    #[doc = "4: act_power_good"]
    ACT_POWER_GOOD = 4,
    #[doc = "5: srss_adft_control_act_ref_en"]
    ACT_REF_EN = 5,
    #[doc = "6: srss_adft_control_act_comp_en"]
    ACT_COMP_EN = 6,
    #[doc = "7: srss_adft_control_dpslp_ref_en"]
    DPSLP_REF_EN = 7,
    #[doc = "8: srss_adft_control_dpslp_reg_en"]
    DPSLP_REG_EN = 8,
    #[doc = "9: srss_adft_control_dpslp_comp_en"]
    DPSLP_COMP_EN = 9,
    #[doc = "10: pwr_control_over_temp_en"]
    OVER_TEMP_EN = 10,
    #[doc = "11: sleepholdreq_n"]
    SLEEPHOLDREQ_N = 11,
    #[doc = "12: adft_buf_en"]
    ADFT_BUF_EN = 12,
    #[doc = "13: ATPG observe point (no functional purpose)"]
    ATPG_OBSERVE = 13,
    #[doc = "14: 1'b0"]
    GND = 14,
    #[doc = "15: 1'b1"]
    PWR = 15,
}
impl From<DDFT0_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DDFT0_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DDFT0_SEL` reader - Select signal for power DDFT output #0"]
pub type DDFT0_SEL_R = crate::FieldReader<u8, DDFT0_SEL_A>;
impl DDFT0_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDFT0_SEL_A {
        match self.bits {
            0 => DDFT0_SEL_A::WAKEUP,
            1 => DDFT0_SEL_A::AWAKE,
            2 => DDFT0_SEL_A::ACT_POWER_EN,
            3 => DDFT0_SEL_A::ACT_POWER_UP,
            4 => DDFT0_SEL_A::ACT_POWER_GOOD,
            5 => DDFT0_SEL_A::ACT_REF_EN,
            6 => DDFT0_SEL_A::ACT_COMP_EN,
            7 => DDFT0_SEL_A::DPSLP_REF_EN,
            8 => DDFT0_SEL_A::DPSLP_REG_EN,
            9 => DDFT0_SEL_A::DPSLP_COMP_EN,
            10 => DDFT0_SEL_A::OVER_TEMP_EN,
            11 => DDFT0_SEL_A::SLEEPHOLDREQ_N,
            12 => DDFT0_SEL_A::ADFT_BUF_EN,
            13 => DDFT0_SEL_A::ATPG_OBSERVE,
            14 => DDFT0_SEL_A::GND,
            15 => DDFT0_SEL_A::PWR,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `WAKEUP`"]
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == DDFT0_SEL_A::WAKEUP
    }
    #[doc = "Checks if the value of the field is `AWAKE`"]
    #[inline(always)]
    pub fn is_awake(&self) -> bool {
        *self == DDFT0_SEL_A::AWAKE
    }
    #[doc = "Checks if the value of the field is `ACT_POWER_EN`"]
    #[inline(always)]
    pub fn is_act_power_en(&self) -> bool {
        *self == DDFT0_SEL_A::ACT_POWER_EN
    }
    #[doc = "Checks if the value of the field is `ACT_POWER_UP`"]
    #[inline(always)]
    pub fn is_act_power_up(&self) -> bool {
        *self == DDFT0_SEL_A::ACT_POWER_UP
    }
    #[doc = "Checks if the value of the field is `ACT_POWER_GOOD`"]
    #[inline(always)]
    pub fn is_act_power_good(&self) -> bool {
        *self == DDFT0_SEL_A::ACT_POWER_GOOD
    }
    #[doc = "Checks if the value of the field is `ACT_REF_EN`"]
    #[inline(always)]
    pub fn is_act_ref_en(&self) -> bool {
        *self == DDFT0_SEL_A::ACT_REF_EN
    }
    #[doc = "Checks if the value of the field is `ACT_COMP_EN`"]
    #[inline(always)]
    pub fn is_act_comp_en(&self) -> bool {
        *self == DDFT0_SEL_A::ACT_COMP_EN
    }
    #[doc = "Checks if the value of the field is `DPSLP_REF_EN`"]
    #[inline(always)]
    pub fn is_dpslp_ref_en(&self) -> bool {
        *self == DDFT0_SEL_A::DPSLP_REF_EN
    }
    #[doc = "Checks if the value of the field is `DPSLP_REG_EN`"]
    #[inline(always)]
    pub fn is_dpslp_reg_en(&self) -> bool {
        *self == DDFT0_SEL_A::DPSLP_REG_EN
    }
    #[doc = "Checks if the value of the field is `DPSLP_COMP_EN`"]
    #[inline(always)]
    pub fn is_dpslp_comp_en(&self) -> bool {
        *self == DDFT0_SEL_A::DPSLP_COMP_EN
    }
    #[doc = "Checks if the value of the field is `OVER_TEMP_EN`"]
    #[inline(always)]
    pub fn is_over_temp_en(&self) -> bool {
        *self == DDFT0_SEL_A::OVER_TEMP_EN
    }
    #[doc = "Checks if the value of the field is `SLEEPHOLDREQ_N`"]
    #[inline(always)]
    pub fn is_sleepholdreq_n(&self) -> bool {
        *self == DDFT0_SEL_A::SLEEPHOLDREQ_N
    }
    #[doc = "Checks if the value of the field is `ADFT_BUF_EN`"]
    #[inline(always)]
    pub fn is_adft_buf_en(&self) -> bool {
        *self == DDFT0_SEL_A::ADFT_BUF_EN
    }
    #[doc = "Checks if the value of the field is `ATPG_OBSERVE`"]
    #[inline(always)]
    pub fn is_atpg_observe(&self) -> bool {
        *self == DDFT0_SEL_A::ATPG_OBSERVE
    }
    #[doc = "Checks if the value of the field is `GND`"]
    #[inline(always)]
    pub fn is_gnd(&self) -> bool {
        *self == DDFT0_SEL_A::GND
    }
    #[doc = "Checks if the value of the field is `PWR`"]
    #[inline(always)]
    pub fn is_pwr(&self) -> bool {
        *self == DDFT0_SEL_A::PWR
    }
}
#[doc = "Field `DDFT0_SEL` writer - Select signal for power DDFT output #0"]
pub type DDFT0_SEL_W<'a> =
    crate::FieldWriterSafe<'a, u32, PWR_DDFT_SELECT_SPEC, u8, DDFT0_SEL_A, 4, 0>;
impl<'a> DDFT0_SEL_W<'a> {
    #[doc = "wakeup"]
    #[inline(always)]
    pub fn wakeup(self) -> &'a mut W {
        self.variant(DDFT0_SEL_A::WAKEUP)
    }
    #[doc = "awake"]
    #[inline(always)]
    pub fn awake(self) -> &'a mut W {
        self.variant(DDFT0_SEL_A::AWAKE)
    }
    #[doc = "act_power_en"]
    #[inline(always)]
    pub fn act_power_en(self) -> &'a mut W {
        self.variant(DDFT0_SEL_A::ACT_POWER_EN)
    }
    #[doc = "act_power_up"]
    #[inline(always)]
    pub fn act_power_up(self) -> &'a mut W {
        self.variant(DDFT0_SEL_A::ACT_POWER_UP)
    }
    #[doc = "act_power_good"]
    #[inline(always)]
    pub fn act_power_good(self) -> &'a mut W {
        self.variant(DDFT0_SEL_A::ACT_POWER_GOOD)
    }
    #[doc = "srss_adft_control_act_ref_en"]
    #[inline(always)]
    pub fn act_ref_en(self) -> &'a mut W {
        self.variant(DDFT0_SEL_A::ACT_REF_EN)
    }
    #[doc = "srss_adft_control_act_comp_en"]
    #[inline(always)]
    pub fn act_comp_en(self) -> &'a mut W {
        self.variant(DDFT0_SEL_A::ACT_COMP_EN)
    }
    #[doc = "srss_adft_control_dpslp_ref_en"]
    #[inline(always)]
    pub fn dpslp_ref_en(self) -> &'a mut W {
        self.variant(DDFT0_SEL_A::DPSLP_REF_EN)
    }
    #[doc = "srss_adft_control_dpslp_reg_en"]
    #[inline(always)]
    pub fn dpslp_reg_en(self) -> &'a mut W {
        self.variant(DDFT0_SEL_A::DPSLP_REG_EN)
    }
    #[doc = "srss_adft_control_dpslp_comp_en"]
    #[inline(always)]
    pub fn dpslp_comp_en(self) -> &'a mut W {
        self.variant(DDFT0_SEL_A::DPSLP_COMP_EN)
    }
    #[doc = "pwr_control_over_temp_en"]
    #[inline(always)]
    pub fn over_temp_en(self) -> &'a mut W {
        self.variant(DDFT0_SEL_A::OVER_TEMP_EN)
    }
    #[doc = "sleepholdreq_n"]
    #[inline(always)]
    pub fn sleepholdreq_n(self) -> &'a mut W {
        self.variant(DDFT0_SEL_A::SLEEPHOLDREQ_N)
    }
    #[doc = "adft_buf_en"]
    #[inline(always)]
    pub fn adft_buf_en(self) -> &'a mut W {
        self.variant(DDFT0_SEL_A::ADFT_BUF_EN)
    }
    #[doc = "ATPG observe point (no functional purpose)"]
    #[inline(always)]
    pub fn atpg_observe(self) -> &'a mut W {
        self.variant(DDFT0_SEL_A::ATPG_OBSERVE)
    }
    #[doc = "1'b0"]
    #[inline(always)]
    pub fn gnd(self) -> &'a mut W {
        self.variant(DDFT0_SEL_A::GND)
    }
    #[doc = "1'b1"]
    #[inline(always)]
    pub fn pwr(self) -> &'a mut W {
        self.variant(DDFT0_SEL_A::PWR)
    }
}
#[doc = "Select signal for power DDFT output #1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DDFT1_SEL_A {
    #[doc = "0: wakeup"]
    WAKEUP = 0,
    #[doc = "1: awake"]
    AWAKE = 1,
    #[doc = "2: act_power_en"]
    ACT_POWER_EN = 2,
    #[doc = "3: act_power_up"]
    ACT_POWER_UP = 3,
    #[doc = "4: act_power_good"]
    ACT_POWER_GOOD = 4,
    #[doc = "5: act_ref_valid"]
    ACT_REF_VALID = 5,
    #[doc = "6: act_reg_valid"]
    ACT_REG_VALID = 6,
    #[doc = "7: act_comp_out"]
    ACT_COMP_OUT = 7,
    #[doc = "8: act_temp_high"]
    ACT_TEMP_HIGH = 8,
    #[doc = "9: dpslp_comp_out"]
    DPSLP_COMP_OUT = 9,
    #[doc = "10: dpslp_power_up"]
    DPSLP_POWER_UP = 10,
    #[doc = "11: awake_delayed"]
    AWAKE_DELAYED = 11,
    #[doc = "12: lpm_ready"]
    LPM_READY = 12,
    #[doc = "13: sleepholdack_n"]
    SLEEPHOLDACK_N = 13,
    #[doc = "14: 1'b0"]
    GND = 14,
    #[doc = "15: 1'b1"]
    PWR = 15,
}
impl From<DDFT1_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DDFT1_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DDFT1_SEL` reader - Select signal for power DDFT output #1"]
pub type DDFT1_SEL_R = crate::FieldReader<u8, DDFT1_SEL_A>;
impl DDFT1_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDFT1_SEL_A {
        match self.bits {
            0 => DDFT1_SEL_A::WAKEUP,
            1 => DDFT1_SEL_A::AWAKE,
            2 => DDFT1_SEL_A::ACT_POWER_EN,
            3 => DDFT1_SEL_A::ACT_POWER_UP,
            4 => DDFT1_SEL_A::ACT_POWER_GOOD,
            5 => DDFT1_SEL_A::ACT_REF_VALID,
            6 => DDFT1_SEL_A::ACT_REG_VALID,
            7 => DDFT1_SEL_A::ACT_COMP_OUT,
            8 => DDFT1_SEL_A::ACT_TEMP_HIGH,
            9 => DDFT1_SEL_A::DPSLP_COMP_OUT,
            10 => DDFT1_SEL_A::DPSLP_POWER_UP,
            11 => DDFT1_SEL_A::AWAKE_DELAYED,
            12 => DDFT1_SEL_A::LPM_READY,
            13 => DDFT1_SEL_A::SLEEPHOLDACK_N,
            14 => DDFT1_SEL_A::GND,
            15 => DDFT1_SEL_A::PWR,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `WAKEUP`"]
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == DDFT1_SEL_A::WAKEUP
    }
    #[doc = "Checks if the value of the field is `AWAKE`"]
    #[inline(always)]
    pub fn is_awake(&self) -> bool {
        *self == DDFT1_SEL_A::AWAKE
    }
    #[doc = "Checks if the value of the field is `ACT_POWER_EN`"]
    #[inline(always)]
    pub fn is_act_power_en(&self) -> bool {
        *self == DDFT1_SEL_A::ACT_POWER_EN
    }
    #[doc = "Checks if the value of the field is `ACT_POWER_UP`"]
    #[inline(always)]
    pub fn is_act_power_up(&self) -> bool {
        *self == DDFT1_SEL_A::ACT_POWER_UP
    }
    #[doc = "Checks if the value of the field is `ACT_POWER_GOOD`"]
    #[inline(always)]
    pub fn is_act_power_good(&self) -> bool {
        *self == DDFT1_SEL_A::ACT_POWER_GOOD
    }
    #[doc = "Checks if the value of the field is `ACT_REF_VALID`"]
    #[inline(always)]
    pub fn is_act_ref_valid(&self) -> bool {
        *self == DDFT1_SEL_A::ACT_REF_VALID
    }
    #[doc = "Checks if the value of the field is `ACT_REG_VALID`"]
    #[inline(always)]
    pub fn is_act_reg_valid(&self) -> bool {
        *self == DDFT1_SEL_A::ACT_REG_VALID
    }
    #[doc = "Checks if the value of the field is `ACT_COMP_OUT`"]
    #[inline(always)]
    pub fn is_act_comp_out(&self) -> bool {
        *self == DDFT1_SEL_A::ACT_COMP_OUT
    }
    #[doc = "Checks if the value of the field is `ACT_TEMP_HIGH`"]
    #[inline(always)]
    pub fn is_act_temp_high(&self) -> bool {
        *self == DDFT1_SEL_A::ACT_TEMP_HIGH
    }
    #[doc = "Checks if the value of the field is `DPSLP_COMP_OUT`"]
    #[inline(always)]
    pub fn is_dpslp_comp_out(&self) -> bool {
        *self == DDFT1_SEL_A::DPSLP_COMP_OUT
    }
    #[doc = "Checks if the value of the field is `DPSLP_POWER_UP`"]
    #[inline(always)]
    pub fn is_dpslp_power_up(&self) -> bool {
        *self == DDFT1_SEL_A::DPSLP_POWER_UP
    }
    #[doc = "Checks if the value of the field is `AWAKE_DELAYED`"]
    #[inline(always)]
    pub fn is_awake_delayed(&self) -> bool {
        *self == DDFT1_SEL_A::AWAKE_DELAYED
    }
    #[doc = "Checks if the value of the field is `LPM_READY`"]
    #[inline(always)]
    pub fn is_lpm_ready(&self) -> bool {
        *self == DDFT1_SEL_A::LPM_READY
    }
    #[doc = "Checks if the value of the field is `SLEEPHOLDACK_N`"]
    #[inline(always)]
    pub fn is_sleepholdack_n(&self) -> bool {
        *self == DDFT1_SEL_A::SLEEPHOLDACK_N
    }
    #[doc = "Checks if the value of the field is `GND`"]
    #[inline(always)]
    pub fn is_gnd(&self) -> bool {
        *self == DDFT1_SEL_A::GND
    }
    #[doc = "Checks if the value of the field is `PWR`"]
    #[inline(always)]
    pub fn is_pwr(&self) -> bool {
        *self == DDFT1_SEL_A::PWR
    }
}
#[doc = "Field `DDFT1_SEL` writer - Select signal for power DDFT output #1"]
pub type DDFT1_SEL_W<'a> =
    crate::FieldWriterSafe<'a, u32, PWR_DDFT_SELECT_SPEC, u8, DDFT1_SEL_A, 4, 4>;
impl<'a> DDFT1_SEL_W<'a> {
    #[doc = "wakeup"]
    #[inline(always)]
    pub fn wakeup(self) -> &'a mut W {
        self.variant(DDFT1_SEL_A::WAKEUP)
    }
    #[doc = "awake"]
    #[inline(always)]
    pub fn awake(self) -> &'a mut W {
        self.variant(DDFT1_SEL_A::AWAKE)
    }
    #[doc = "act_power_en"]
    #[inline(always)]
    pub fn act_power_en(self) -> &'a mut W {
        self.variant(DDFT1_SEL_A::ACT_POWER_EN)
    }
    #[doc = "act_power_up"]
    #[inline(always)]
    pub fn act_power_up(self) -> &'a mut W {
        self.variant(DDFT1_SEL_A::ACT_POWER_UP)
    }
    #[doc = "act_power_good"]
    #[inline(always)]
    pub fn act_power_good(self) -> &'a mut W {
        self.variant(DDFT1_SEL_A::ACT_POWER_GOOD)
    }
    #[doc = "act_ref_valid"]
    #[inline(always)]
    pub fn act_ref_valid(self) -> &'a mut W {
        self.variant(DDFT1_SEL_A::ACT_REF_VALID)
    }
    #[doc = "act_reg_valid"]
    #[inline(always)]
    pub fn act_reg_valid(self) -> &'a mut W {
        self.variant(DDFT1_SEL_A::ACT_REG_VALID)
    }
    #[doc = "act_comp_out"]
    #[inline(always)]
    pub fn act_comp_out(self) -> &'a mut W {
        self.variant(DDFT1_SEL_A::ACT_COMP_OUT)
    }
    #[doc = "act_temp_high"]
    #[inline(always)]
    pub fn act_temp_high(self) -> &'a mut W {
        self.variant(DDFT1_SEL_A::ACT_TEMP_HIGH)
    }
    #[doc = "dpslp_comp_out"]
    #[inline(always)]
    pub fn dpslp_comp_out(self) -> &'a mut W {
        self.variant(DDFT1_SEL_A::DPSLP_COMP_OUT)
    }
    #[doc = "dpslp_power_up"]
    #[inline(always)]
    pub fn dpslp_power_up(self) -> &'a mut W {
        self.variant(DDFT1_SEL_A::DPSLP_POWER_UP)
    }
    #[doc = "awake_delayed"]
    #[inline(always)]
    pub fn awake_delayed(self) -> &'a mut W {
        self.variant(DDFT1_SEL_A::AWAKE_DELAYED)
    }
    #[doc = "lpm_ready"]
    #[inline(always)]
    pub fn lpm_ready(self) -> &'a mut W {
        self.variant(DDFT1_SEL_A::LPM_READY)
    }
    #[doc = "sleepholdack_n"]
    #[inline(always)]
    pub fn sleepholdack_n(self) -> &'a mut W {
        self.variant(DDFT1_SEL_A::SLEEPHOLDACK_N)
    }
    #[doc = "1'b0"]
    #[inline(always)]
    pub fn gnd(self) -> &'a mut W {
        self.variant(DDFT1_SEL_A::GND)
    }
    #[doc = "1'b1"]
    #[inline(always)]
    pub fn pwr(self) -> &'a mut W {
        self.variant(DDFT1_SEL_A::PWR)
    }
}
impl R {
    #[doc = "Bits 0:3 - Select signal for power DDFT output #0"]
    #[inline(always)]
    pub fn ddft0_sel(&self) -> DDFT0_SEL_R {
        DDFT0_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Select signal for power DDFT output #1"]
    #[inline(always)]
    pub fn ddft1_sel(&self) -> DDFT1_SEL_R {
        DDFT1_SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Select signal for power DDFT output #0"]
    #[inline(always)]
    pub fn ddft0_sel(&mut self) -> DDFT0_SEL_W {
        DDFT0_SEL_W::new(self)
    }
    #[doc = "Bits 4:7 - Select signal for power DDFT output #1"]
    #[inline(always)]
    pub fn ddft1_sel(&mut self) -> DDFT1_SEL_W {
        DDFT1_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power DDFT Mode Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_ddft_select](index.html) module"]
pub struct PWR_DDFT_SELECT_SPEC;
impl crate::RegisterSpec for PWR_DDFT_SELECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_ddft_select::R](R) reader structure"]
impl crate::Readable for PWR_DDFT_SELECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_ddft_select::W](W) writer structure"]
impl crate::Writable for PWR_DDFT_SELECT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_DDFT_SELECT to value 0"]
impl crate::Resettable for PWR_DDFT_SELECT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
