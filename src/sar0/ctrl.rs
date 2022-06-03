#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SARADC internal VREF selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VREF_SEL_A {
    #[doc = "0: VREF0 from PRB (VREF buffer on)"]
    VREF0 = 0,
    #[doc = "1: VREF1 from PRB (VREF buffer on)"]
    VREF1 = 1,
    #[doc = "2: VREF2 from PRB (VREF buffer on)"]
    VREF2 = 2,
    #[doc = "3: VREF from AROUTE (VREF buffer on)"]
    VREF_AROUTE = 3,
    #[doc = "4: 1.024V from BandGap (VREF buffer on)"]
    VBGR = 4,
    #[doc = "5: External precision Vref direct from a pin (low impedance path)."]
    VREF_EXT = 5,
    #[doc = "6: Vdda/2 (VREF buffer on)"]
    VDDA_DIV_2 = 6,
    #[doc = "7: Vdda."]
    VDDA = 7,
}
impl From<VREF_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: VREF_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `VREF_SEL` reader - SARADC internal VREF selection."]
pub type VREF_SEL_R = crate::FieldReader<u8, VREF_SEL_A>;
impl VREF_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREF_SEL_A {
        match self.bits {
            0 => VREF_SEL_A::VREF0,
            1 => VREF_SEL_A::VREF1,
            2 => VREF_SEL_A::VREF2,
            3 => VREF_SEL_A::VREF_AROUTE,
            4 => VREF_SEL_A::VBGR,
            5 => VREF_SEL_A::VREF_EXT,
            6 => VREF_SEL_A::VDDA_DIV_2,
            7 => VREF_SEL_A::VDDA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VREF0`"]
    #[inline(always)]
    pub fn is_vref0(&self) -> bool {
        *self == VREF_SEL_A::VREF0
    }
    #[doc = "Checks if the value of the field is `VREF1`"]
    #[inline(always)]
    pub fn is_vref1(&self) -> bool {
        *self == VREF_SEL_A::VREF1
    }
    #[doc = "Checks if the value of the field is `VREF2`"]
    #[inline(always)]
    pub fn is_vref2(&self) -> bool {
        *self == VREF_SEL_A::VREF2
    }
    #[doc = "Checks if the value of the field is `VREF_AROUTE`"]
    #[inline(always)]
    pub fn is_vref_aroute(&self) -> bool {
        *self == VREF_SEL_A::VREF_AROUTE
    }
    #[doc = "Checks if the value of the field is `VBGR`"]
    #[inline(always)]
    pub fn is_vbgr(&self) -> bool {
        *self == VREF_SEL_A::VBGR
    }
    #[doc = "Checks if the value of the field is `VREF_EXT`"]
    #[inline(always)]
    pub fn is_vref_ext(&self) -> bool {
        *self == VREF_SEL_A::VREF_EXT
    }
    #[doc = "Checks if the value of the field is `VDDA_DIV_2`"]
    #[inline(always)]
    pub fn is_vdda_div_2(&self) -> bool {
        *self == VREF_SEL_A::VDDA_DIV_2
    }
    #[doc = "Checks if the value of the field is `VDDA`"]
    #[inline(always)]
    pub fn is_vdda(&self) -> bool {
        *self == VREF_SEL_A::VDDA
    }
}
#[doc = "Field `VREF_SEL` writer - SARADC internal VREF selection."]
pub type VREF_SEL_W<'a> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, VREF_SEL_A, 3, 4>;
impl<'a> VREF_SEL_W<'a> {
    #[doc = "VREF0 from PRB (VREF buffer on)"]
    #[inline(always)]
    pub fn vref0(self) -> &'a mut W {
        self.variant(VREF_SEL_A::VREF0)
    }
    #[doc = "VREF1 from PRB (VREF buffer on)"]
    #[inline(always)]
    pub fn vref1(self) -> &'a mut W {
        self.variant(VREF_SEL_A::VREF1)
    }
    #[doc = "VREF2 from PRB (VREF buffer on)"]
    #[inline(always)]
    pub fn vref2(self) -> &'a mut W {
        self.variant(VREF_SEL_A::VREF2)
    }
    #[doc = "VREF from AROUTE (VREF buffer on)"]
    #[inline(always)]
    pub fn vref_aroute(self) -> &'a mut W {
        self.variant(VREF_SEL_A::VREF_AROUTE)
    }
    #[doc = "1.024V from BandGap (VREF buffer on)"]
    #[inline(always)]
    pub fn vbgr(self) -> &'a mut W {
        self.variant(VREF_SEL_A::VBGR)
    }
    #[doc = "External precision Vref direct from a pin (low impedance path)."]
    #[inline(always)]
    pub fn vref_ext(self) -> &'a mut W {
        self.variant(VREF_SEL_A::VREF_EXT)
    }
    #[doc = "Vdda/2 (VREF buffer on)"]
    #[inline(always)]
    pub fn vdda_div_2(self) -> &'a mut W {
        self.variant(VREF_SEL_A::VDDA_DIV_2)
    }
    #[doc = "Vdda."]
    #[inline(always)]
    pub fn vdda(self) -> &'a mut W {
        self.variant(VREF_SEL_A::VDDA)
    }
}
#[doc = "Field `VREF_BYP_CAP_EN` reader - VREF bypass cap enable for when VREF buffer is on"]
pub type VREF_BYP_CAP_EN_R = crate::BitReader<bool>;
#[doc = "Field `VREF_BYP_CAP_EN` writer - VREF bypass cap enable for when VREF buffer is on"]
pub type VREF_BYP_CAP_EN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 7>;
#[doc = "SARADC internal NEG selection for Single ended conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NEG_SEL_A {
    #[doc = "0: NEG input of SARADC is connected to 'vssa_kelvin', gives more precision around zero. Note this opens both SARADC internal switches, therefore use this value to insert a break-before-make cycle on those switches when SWITCH_DISABLE is high."]
    VSSA_KELVIN = 0,
    #[doc = "1: NEG input of SARADC is connected to VSSA in AROUTE close to the SARADC"]
    ART_VSSA = 1,
    #[doc = "2: NEG input of SARADC is connected to P1 pin of SARMUX"]
    P1 = 2,
    #[doc = "3: NEG input of SARADC is connected to P3 pin of SARMUX"]
    P3 = 3,
    #[doc = "4: NEG input of SARADC is connected to P5 pin of SARMUX"]
    P5 = 4,
    #[doc = "5: NEG input of SARADC is connected to P7 pin of SARMUX"]
    P7 = 5,
    #[doc = "6: NEG input of SARADC is connected to an ACORE in AROUTE"]
    ACORE = 6,
    #[doc = "7: NEG input of SARADC is shorted with VREF input of SARADC."]
    VREF = 7,
}
impl From<NEG_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: NEG_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NEG_SEL` reader - SARADC internal NEG selection for Single ended conversion"]
pub type NEG_SEL_R = crate::FieldReader<u8, NEG_SEL_A>;
impl NEG_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NEG_SEL_A {
        match self.bits {
            0 => NEG_SEL_A::VSSA_KELVIN,
            1 => NEG_SEL_A::ART_VSSA,
            2 => NEG_SEL_A::P1,
            3 => NEG_SEL_A::P3,
            4 => NEG_SEL_A::P5,
            5 => NEG_SEL_A::P7,
            6 => NEG_SEL_A::ACORE,
            7 => NEG_SEL_A::VREF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VSSA_KELVIN`"]
    #[inline(always)]
    pub fn is_vssa_kelvin(&self) -> bool {
        *self == NEG_SEL_A::VSSA_KELVIN
    }
    #[doc = "Checks if the value of the field is `ART_VSSA`"]
    #[inline(always)]
    pub fn is_art_vssa(&self) -> bool {
        *self == NEG_SEL_A::ART_VSSA
    }
    #[doc = "Checks if the value of the field is `P1`"]
    #[inline(always)]
    pub fn is_p1(&self) -> bool {
        *self == NEG_SEL_A::P1
    }
    #[doc = "Checks if the value of the field is `P3`"]
    #[inline(always)]
    pub fn is_p3(&self) -> bool {
        *self == NEG_SEL_A::P3
    }
    #[doc = "Checks if the value of the field is `P5`"]
    #[inline(always)]
    pub fn is_p5(&self) -> bool {
        *self == NEG_SEL_A::P5
    }
    #[doc = "Checks if the value of the field is `P7`"]
    #[inline(always)]
    pub fn is_p7(&self) -> bool {
        *self == NEG_SEL_A::P7
    }
    #[doc = "Checks if the value of the field is `ACORE`"]
    #[inline(always)]
    pub fn is_acore(&self) -> bool {
        *self == NEG_SEL_A::ACORE
    }
    #[doc = "Checks if the value of the field is `VREF`"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == NEG_SEL_A::VREF
    }
}
#[doc = "Field `NEG_SEL` writer - SARADC internal NEG selection for Single ended conversion"]
pub type NEG_SEL_W<'a> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, NEG_SEL_A, 3, 9>;
impl<'a> NEG_SEL_W<'a> {
    #[doc = "NEG input of SARADC is connected to 'vssa_kelvin', gives more precision around zero. Note this opens both SARADC internal switches, therefore use this value to insert a break-before-make cycle on those switches when SWITCH_DISABLE is high."]
    #[inline(always)]
    pub fn vssa_kelvin(self) -> &'a mut W {
        self.variant(NEG_SEL_A::VSSA_KELVIN)
    }
    #[doc = "NEG input of SARADC is connected to VSSA in AROUTE close to the SARADC"]
    #[inline(always)]
    pub fn art_vssa(self) -> &'a mut W {
        self.variant(NEG_SEL_A::ART_VSSA)
    }
    #[doc = "NEG input of SARADC is connected to P1 pin of SARMUX"]
    #[inline(always)]
    pub fn p1(self) -> &'a mut W {
        self.variant(NEG_SEL_A::P1)
    }
    #[doc = "NEG input of SARADC is connected to P3 pin of SARMUX"]
    #[inline(always)]
    pub fn p3(self) -> &'a mut W {
        self.variant(NEG_SEL_A::P3)
    }
    #[doc = "NEG input of SARADC is connected to P5 pin of SARMUX"]
    #[inline(always)]
    pub fn p5(self) -> &'a mut W {
        self.variant(NEG_SEL_A::P5)
    }
    #[doc = "NEG input of SARADC is connected to P7 pin of SARMUX"]
    #[inline(always)]
    pub fn p7(self) -> &'a mut W {
        self.variant(NEG_SEL_A::P7)
    }
    #[doc = "NEG input of SARADC is connected to an ACORE in AROUTE"]
    #[inline(always)]
    pub fn acore(self) -> &'a mut W {
        self.variant(NEG_SEL_A::ACORE)
    }
    #[doc = "NEG input of SARADC is shorted with VREF input of SARADC."]
    #[inline(always)]
    pub fn vref(self) -> &'a mut W {
        self.variant(NEG_SEL_A::VREF)
    }
}
#[doc = "Field `SAR_HW_CTRL_NEGVREF` reader - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for VREF to NEG switch."]
pub type SAR_HW_CTRL_NEGVREF_R = crate::BitReader<bool>;
#[doc = "Field `SAR_HW_CTRL_NEGVREF` writer - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for VREF to NEG switch."]
pub type SAR_HW_CTRL_NEGVREF_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 13>;
#[doc = "VREF buffer low power mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWR_CTRL_VREF_A {
    #[doc = "0: normal power (default), bypass cap, max clk_sar is 18MHz."]
    NORMAL_PWR = 0,
    #[doc = "1: deprecated"]
    HALF_PWR = 1,
    #[doc = "2: Invalid for PSoC4A, otherwise 2X power, no bypass cap, max clk_sar is 1.8MHz"]
    THIRD_PWR = 2,
    #[doc = "3: deprecated"]
    QUARTER_PWR = 3,
}
impl From<PWR_CTRL_VREF_A> for u8 {
    #[inline(always)]
    fn from(variant: PWR_CTRL_VREF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWR_CTRL_VREF` reader - VREF buffer low power mode."]
pub type PWR_CTRL_VREF_R = crate::FieldReader<u8, PWR_CTRL_VREF_A>;
impl PWR_CTRL_VREF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWR_CTRL_VREF_A {
        match self.bits {
            0 => PWR_CTRL_VREF_A::NORMAL_PWR,
            1 => PWR_CTRL_VREF_A::HALF_PWR,
            2 => PWR_CTRL_VREF_A::THIRD_PWR,
            3 => PWR_CTRL_VREF_A::QUARTER_PWR,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_PWR`"]
    #[inline(always)]
    pub fn is_normal_pwr(&self) -> bool {
        *self == PWR_CTRL_VREF_A::NORMAL_PWR
    }
    #[doc = "Checks if the value of the field is `HALF_PWR`"]
    #[inline(always)]
    pub fn is_half_pwr(&self) -> bool {
        *self == PWR_CTRL_VREF_A::HALF_PWR
    }
    #[doc = "Checks if the value of the field is `THIRD_PWR`"]
    #[inline(always)]
    pub fn is_third_pwr(&self) -> bool {
        *self == PWR_CTRL_VREF_A::THIRD_PWR
    }
    #[doc = "Checks if the value of the field is `QUARTER_PWR`"]
    #[inline(always)]
    pub fn is_quarter_pwr(&self) -> bool {
        *self == PWR_CTRL_VREF_A::QUARTER_PWR
    }
}
#[doc = "Field `PWR_CTRL_VREF` writer - VREF buffer low power mode."]
pub type PWR_CTRL_VREF_W<'a> =
    crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, PWR_CTRL_VREF_A, 2, 14>;
impl<'a> PWR_CTRL_VREF_W<'a> {
    #[doc = "normal power (default), bypass cap, max clk_sar is 18MHz."]
    #[inline(always)]
    pub fn normal_pwr(self) -> &'a mut W {
        self.variant(PWR_CTRL_VREF_A::NORMAL_PWR)
    }
    #[doc = "deprecated"]
    #[inline(always)]
    pub fn half_pwr(self) -> &'a mut W {
        self.variant(PWR_CTRL_VREF_A::HALF_PWR)
    }
    #[doc = "Invalid for PSoC4A, otherwise 2X power, no bypass cap, max clk_sar is 1.8MHz"]
    #[inline(always)]
    pub fn third_pwr(self) -> &'a mut W {
        self.variant(PWR_CTRL_VREF_A::THIRD_PWR)
    }
    #[doc = "deprecated"]
    #[inline(always)]
    pub fn quarter_pwr(self) -> &'a mut W {
        self.variant(PWR_CTRL_VREF_A::QUARTER_PWR)
    }
}
#[doc = "Field `SPARE` reader - Spare controls, not yet designated, for late changes done with an ECO"]
pub type SPARE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPARE` writer - Spare controls, not yet designated, for late changes done with an ECO"]
pub type SPARE_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, 16>;
#[doc = "Field `BOOSTPUMP_EN` reader - SARADC internal pump: 0=disabled: pump output is VDDA, 1=enabled: pump output is boosted."]
pub type BOOSTPUMP_EN_R = crate::BitReader<bool>;
#[doc = "Field `BOOSTPUMP_EN` writer - SARADC internal pump: 0=disabled: pump output is VDDA, 1=enabled: pump output is boosted."]
pub type BOOSTPUMP_EN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 20>;
#[doc = "SARADC low power mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ICONT_LV_A {
    #[doc = "0: normal power (default), max clk_sar is 18MHz."]
    NORMAL_PWR = 0,
    #[doc = "1: 1/2 power mode, max clk_sar is 9MHz."]
    HALF_PWR = 1,
    #[doc = "2: 1.333 power mode, max clk_sar is 18MHz."]
    MORE_PWR = 2,
    #[doc = "3: 1/4 power mode, max clk_sar is 4.5MHz."]
    QUARTER_PWR = 3,
}
impl From<ICONT_LV_A> for u8 {
    #[inline(always)]
    fn from(variant: ICONT_LV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ICONT_LV` reader - SARADC low power mode."]
pub type ICONT_LV_R = crate::FieldReader<u8, ICONT_LV_A>;
impl ICONT_LV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICONT_LV_A {
        match self.bits {
            0 => ICONT_LV_A::NORMAL_PWR,
            1 => ICONT_LV_A::HALF_PWR,
            2 => ICONT_LV_A::MORE_PWR,
            3 => ICONT_LV_A::QUARTER_PWR,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_PWR`"]
    #[inline(always)]
    pub fn is_normal_pwr(&self) -> bool {
        *self == ICONT_LV_A::NORMAL_PWR
    }
    #[doc = "Checks if the value of the field is `HALF_PWR`"]
    #[inline(always)]
    pub fn is_half_pwr(&self) -> bool {
        *self == ICONT_LV_A::HALF_PWR
    }
    #[doc = "Checks if the value of the field is `MORE_PWR`"]
    #[inline(always)]
    pub fn is_more_pwr(&self) -> bool {
        *self == ICONT_LV_A::MORE_PWR
    }
    #[doc = "Checks if the value of the field is `QUARTER_PWR`"]
    #[inline(always)]
    pub fn is_quarter_pwr(&self) -> bool {
        *self == ICONT_LV_A::QUARTER_PWR
    }
}
#[doc = "Field `ICONT_LV` writer - SARADC low power mode."]
pub type ICONT_LV_W<'a> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, ICONT_LV_A, 2, 24>;
impl<'a> ICONT_LV_W<'a> {
    #[doc = "normal power (default), max clk_sar is 18MHz."]
    #[inline(always)]
    pub fn normal_pwr(self) -> &'a mut W {
        self.variant(ICONT_LV_A::NORMAL_PWR)
    }
    #[doc = "1/2 power mode, max clk_sar is 9MHz."]
    #[inline(always)]
    pub fn half_pwr(self) -> &'a mut W {
        self.variant(ICONT_LV_A::HALF_PWR)
    }
    #[doc = "1.333 power mode, max clk_sar is 18MHz."]
    #[inline(always)]
    pub fn more_pwr(self) -> &'a mut W {
        self.variant(ICONT_LV_A::MORE_PWR)
    }
    #[doc = "1/4 power mode, max clk_sar is 4.5MHz."]
    #[inline(always)]
    pub fn quarter_pwr(self) -> &'a mut W {
        self.variant(ICONT_LV_A::QUARTER_PWR)
    }
}
#[doc = "Field `DEEPSLEEP_ON` reader - - 0: SARMUX IP disabled off during DeepSleep power mode - 1: SARMUX IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
pub type DEEPSLEEP_ON_R = crate::BitReader<bool>;
#[doc = "Field `DEEPSLEEP_ON` writer - - 0: SARMUX IP disabled off during DeepSleep power mode - 1: SARMUX IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
pub type DEEPSLEEP_ON_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 27>;
#[doc = "Field `DSI_SYNC_CONFIG` reader - - 0: bypass clock domain synchronisation of the DSI config signals. - 1: synchronize the DSI config signals to peripheral clock domain."]
pub type DSI_SYNC_CONFIG_R = crate::BitReader<bool>;
#[doc = "Field `DSI_SYNC_CONFIG` writer - - 0: bypass clock domain synchronisation of the DSI config signals. - 1: synchronize the DSI config signals to peripheral clock domain."]
pub type DSI_SYNC_CONFIG_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 28>;
#[doc = "Field `DSI_MODE` reader - SAR sequencer takes configuration from DSI signals (note this also has the same effect as SWITCH_DISABLE==1) - 0: Normal mode, SAR sequencer operates according to CHAN_EN enables and CHAN_CONFIG channel configurations - 1: CHAN_EN, INJ_START_EN and channel configurations in CHAN_CONFIG and INJ_CHAN_CONFIG are ignored"]
pub type DSI_MODE_R = crate::BitReader<bool>;
#[doc = "Field `DSI_MODE` writer - SAR sequencer takes configuration from DSI signals (note this also has the same effect as SWITCH_DISABLE==1) - 0: Normal mode, SAR sequencer operates according to CHAN_EN enables and CHAN_CONFIG channel configurations - 1: CHAN_EN, INJ_START_EN and channel configurations in CHAN_CONFIG and INJ_CHAN_CONFIG are ignored"]
pub type DSI_MODE_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 29>;
#[doc = "Field `SWITCH_DISABLE` reader - Disable SAR sequencer from enabling routing switches (note DSI and firmware can always close switches independent of this control) - 0: Normal mode, SAR sequencer changes switches according to pin address in channel configurations - 1: Switches disabled, SAR sequencer does not enable any switches, it is the responsibility of the firmware or UDBs (through DSI) to set the switches to route the signal to be converted through the SARMUX"]
pub type SWITCH_DISABLE_R = crate::BitReader<bool>;
#[doc = "Field `SWITCH_DISABLE` writer - Disable SAR sequencer from enabling routing switches (note DSI and firmware can always close switches independent of this control) - 0: Normal mode, SAR sequencer changes switches according to pin address in channel configurations - 1: Switches disabled, SAR sequencer does not enable any switches, it is the responsibility of the firmware or UDBs (through DSI) to set the switches to route the signal to be converted through the SARMUX"]
pub type SWITCH_DISABLE_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 30>;
#[doc = "Field `ENABLED` reader - Before enabling always make sure the SAR is idle (STATUS.BUSY==0) - 0: SAR IP disabled (put analog in power down and stop clocks), also can clear FW_TRIGGER and INJ_START_EN (if not tailgaiting) on write. - 1: SAR IP enabled."]
pub type ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ENABLED` writer - Before enabling always make sure the SAR is idle (STATUS.BUSY==0) - 0: SAR IP disabled (put analog in power down and stop clocks), also can clear FW_TRIGGER and INJ_START_EN (if not tailgaiting) on write. - 1: SAR IP enabled."]
pub type ENABLED_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 4:6 - SARADC internal VREF selection."]
    #[inline(always)]
    pub fn vref_sel(&self) -> VREF_SEL_R {
        VREF_SEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - VREF bypass cap enable for when VREF buffer is on"]
    #[inline(always)]
    pub fn vref_byp_cap_en(&self) -> VREF_BYP_CAP_EN_R {
        VREF_BYP_CAP_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 9:11 - SARADC internal NEG selection for Single ended conversion"]
    #[inline(always)]
    pub fn neg_sel(&self) -> NEG_SEL_R {
        NEG_SEL_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 13 - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for VREF to NEG switch."]
    #[inline(always)]
    pub fn sar_hw_ctrl_negvref(&self) -> SAR_HW_CTRL_NEGVREF_R {
        SAR_HW_CTRL_NEGVREF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - VREF buffer low power mode."]
    #[inline(always)]
    pub fn pwr_ctrl_vref(&self) -> PWR_CTRL_VREF_R {
        PWR_CTRL_VREF_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Spare controls, not yet designated, for late changes done with an ECO"]
    #[inline(always)]
    pub fn spare(&self) -> SPARE_R {
        SPARE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - SARADC internal pump: 0=disabled: pump output is VDDA, 1=enabled: pump output is boosted."]
    #[inline(always)]
    pub fn boostpump_en(&self) -> BOOSTPUMP_EN_R {
        BOOSTPUMP_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:25 - SARADC low power mode."]
    #[inline(always)]
    pub fn icont_lv(&self) -> ICONT_LV_R {
        ICONT_LV_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 27 - - 0: SARMUX IP disabled off during DeepSleep power mode - 1: SARMUX IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
    #[inline(always)]
    pub fn deepsleep_on(&self) -> DEEPSLEEP_ON_R {
        DEEPSLEEP_ON_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - - 0: bypass clock domain synchronisation of the DSI config signals. - 1: synchronize the DSI config signals to peripheral clock domain."]
    #[inline(always)]
    pub fn dsi_sync_config(&self) -> DSI_SYNC_CONFIG_R {
        DSI_SYNC_CONFIG_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SAR sequencer takes configuration from DSI signals (note this also has the same effect as SWITCH_DISABLE==1) - 0: Normal mode, SAR sequencer operates according to CHAN_EN enables and CHAN_CONFIG channel configurations - 1: CHAN_EN, INJ_START_EN and channel configurations in CHAN_CONFIG and INJ_CHAN_CONFIG are ignored"]
    #[inline(always)]
    pub fn dsi_mode(&self) -> DSI_MODE_R {
        DSI_MODE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Disable SAR sequencer from enabling routing switches (note DSI and firmware can always close switches independent of this control) - 0: Normal mode, SAR sequencer changes switches according to pin address in channel configurations - 1: Switches disabled, SAR sequencer does not enable any switches, it is the responsibility of the firmware or UDBs (through DSI) to set the switches to route the signal to be converted through the SARMUX"]
    #[inline(always)]
    pub fn switch_disable(&self) -> SWITCH_DISABLE_R {
        SWITCH_DISABLE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Before enabling always make sure the SAR is idle (STATUS.BUSY==0) - 0: SAR IP disabled (put analog in power down and stop clocks), also can clear FW_TRIGGER and INJ_START_EN (if not tailgaiting) on write. - 1: SAR IP enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:6 - SARADC internal VREF selection."]
    #[inline(always)]
    pub fn vref_sel(&mut self) -> VREF_SEL_W {
        VREF_SEL_W::new(self)
    }
    #[doc = "Bit 7 - VREF bypass cap enable for when VREF buffer is on"]
    #[inline(always)]
    pub fn vref_byp_cap_en(&mut self) -> VREF_BYP_CAP_EN_W {
        VREF_BYP_CAP_EN_W::new(self)
    }
    #[doc = "Bits 9:11 - SARADC internal NEG selection for Single ended conversion"]
    #[inline(always)]
    pub fn neg_sel(&mut self) -> NEG_SEL_W {
        NEG_SEL_W::new(self)
    }
    #[doc = "Bit 13 - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for VREF to NEG switch."]
    #[inline(always)]
    pub fn sar_hw_ctrl_negvref(&mut self) -> SAR_HW_CTRL_NEGVREF_W {
        SAR_HW_CTRL_NEGVREF_W::new(self)
    }
    #[doc = "Bits 14:15 - VREF buffer low power mode."]
    #[inline(always)]
    pub fn pwr_ctrl_vref(&mut self) -> PWR_CTRL_VREF_W {
        PWR_CTRL_VREF_W::new(self)
    }
    #[doc = "Bits 16:19 - Spare controls, not yet designated, for late changes done with an ECO"]
    #[inline(always)]
    pub fn spare(&mut self) -> SPARE_W {
        SPARE_W::new(self)
    }
    #[doc = "Bit 20 - SARADC internal pump: 0=disabled: pump output is VDDA, 1=enabled: pump output is boosted."]
    #[inline(always)]
    pub fn boostpump_en(&mut self) -> BOOSTPUMP_EN_W {
        BOOSTPUMP_EN_W::new(self)
    }
    #[doc = "Bits 24:25 - SARADC low power mode."]
    #[inline(always)]
    pub fn icont_lv(&mut self) -> ICONT_LV_W {
        ICONT_LV_W::new(self)
    }
    #[doc = "Bit 27 - - 0: SARMUX IP disabled off during DeepSleep power mode - 1: SARMUX IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
    #[inline(always)]
    pub fn deepsleep_on(&mut self) -> DEEPSLEEP_ON_W {
        DEEPSLEEP_ON_W::new(self)
    }
    #[doc = "Bit 28 - - 0: bypass clock domain synchronisation of the DSI config signals. - 1: synchronize the DSI config signals to peripheral clock domain."]
    #[inline(always)]
    pub fn dsi_sync_config(&mut self) -> DSI_SYNC_CONFIG_W {
        DSI_SYNC_CONFIG_W::new(self)
    }
    #[doc = "Bit 29 - SAR sequencer takes configuration from DSI signals (note this also has the same effect as SWITCH_DISABLE==1) - 0: Normal mode, SAR sequencer operates according to CHAN_EN enables and CHAN_CONFIG channel configurations - 1: CHAN_EN, INJ_START_EN and channel configurations in CHAN_CONFIG and INJ_CHAN_CONFIG are ignored"]
    #[inline(always)]
    pub fn dsi_mode(&mut self) -> DSI_MODE_W {
        DSI_MODE_W::new(self)
    }
    #[doc = "Bit 30 - Disable SAR sequencer from enabling routing switches (note DSI and firmware can always close switches independent of this control) - 0: Normal mode, SAR sequencer changes switches according to pin address in channel configurations - 1: Switches disabled, SAR sequencer does not enable any switches, it is the responsibility of the firmware or UDBs (through DSI) to set the switches to route the signal to be converted through the SARMUX"]
    #[inline(always)]
    pub fn switch_disable(&mut self) -> SWITCH_DISABLE_W {
        SWITCH_DISABLE_W::new(self)
    }
    #[doc = "Bit 31 - Before enabling always make sure the SAR is idle (STATUS.BUSY==0) - 0: SAR IP disabled (put analog in power down and stop clocks), also can clear FW_TRIGGER and INJ_START_EN (if not tailgaiting) on write. - 1: SAR IP enabled."]
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
#[doc = "Analog control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0x1000_0000"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1000_0000
    }
}
