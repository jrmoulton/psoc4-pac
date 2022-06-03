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
#[doc = "Field `FILTER_DELAY` reader - This value determines the number of cycles that the digital filter makes the CSDCMP output ignored while the counter counts and IDAC is on. When set to 0 the digital filter is off. When set to any other value the ignoring will last for FILTER_DELAY clk_csd cycles after the start of each measurement and from the first comparator trip to the end of each measurement."]
pub type FILTER_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FILTER_DELAY` writer - This value determines the number of cycles that the digital filter makes the CSDCMP output ignored while the counter counts and IDAC is on. When set to 0 the digital filter is off. When set to any other value the ignoring will last for FILTER_DELAY clk_csd cycles after the start of each measurement and from the first comparator trip to the end of each measurement."]
pub type FILTER_DELAY_W<'a> = crate::FieldWriter<'a, u32, CONFIG_SPEC, u8, u8, 3, 4>;
#[doc = "Selects the delay by which csd_shield is delayed relative to csd_sense.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SHIELD_DELAY_A {
    #[doc = "0: Delay line is off, csd_shield=csd_sense"]
    OFF = 0,
    #[doc = "1: Introduces a 5ns delay (typ)"]
    D5NS = 1,
    #[doc = "2: Introduces a 10ns delay (typ)"]
    D10NS = 2,
    #[doc = "3: Introduces a 20ns delay (typ)"]
    D20NS = 3,
}
impl From<SHIELD_DELAY_A> for u8 {
    #[inline(always)]
    fn from(variant: SHIELD_DELAY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SHIELD_DELAY` reader - Selects the delay by which csd_shield is delayed relative to csd_sense."]
pub type SHIELD_DELAY_R = crate::FieldReader<u8, SHIELD_DELAY_A>;
impl SHIELD_DELAY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHIELD_DELAY_A {
        match self.bits {
            0 => SHIELD_DELAY_A::OFF,
            1 => SHIELD_DELAY_A::D5NS,
            2 => SHIELD_DELAY_A::D10NS,
            3 => SHIELD_DELAY_A::D20NS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == SHIELD_DELAY_A::OFF
    }
    #[doc = "Checks if the value of the field is `D5NS`"]
    #[inline(always)]
    pub fn is_d5ns(&self) -> bool {
        *self == SHIELD_DELAY_A::D5NS
    }
    #[doc = "Checks if the value of the field is `D10NS`"]
    #[inline(always)]
    pub fn is_d10ns(&self) -> bool {
        *self == SHIELD_DELAY_A::D10NS
    }
    #[doc = "Checks if the value of the field is `D20NS`"]
    #[inline(always)]
    pub fn is_d20ns(&self) -> bool {
        *self == SHIELD_DELAY_A::D20NS
    }
}
#[doc = "Field `SHIELD_DELAY` writer - Selects the delay by which csd_shield is delayed relative to csd_sense."]
pub type SHIELD_DELAY_W<'a> =
    crate::FieldWriterSafe<'a, u32, CONFIG_SPEC, u8, SHIELD_DELAY_A, 2, 8>;
impl<'a> SHIELD_DELAY_W<'a> {
    #[doc = "Delay line is off, csd_shield=csd_sense"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(SHIELD_DELAY_A::OFF)
    }
    #[doc = "Introduces a 5ns delay (typ)"]
    #[inline(always)]
    pub fn d5ns(self) -> &'a mut W {
        self.variant(SHIELD_DELAY_A::D5NS)
    }
    #[doc = "Introduces a 10ns delay (typ)"]
    #[inline(always)]
    pub fn d10ns(self) -> &'a mut W {
        self.variant(SHIELD_DELAY_A::D10NS)
    }
    #[doc = "Introduces a 20ns delay (typ)"]
    #[inline(always)]
    pub fn d20ns(self) -> &'a mut W {
        self.variant(SHIELD_DELAY_A::D20NS)
    }
}
#[doc = "Field `SENSE_EN` reader - Enables the sense modulator output. 0: all switches, static or dynamic, are open and IDAC in CSD mode is off 1: switches and IDAC can be closed/on as per MMIO setting and CSD sequencer."]
pub type SENSE_EN_R = crate::BitReader<bool>;
#[doc = "Field `SENSE_EN` writer - Enables the sense modulator output. 0: all switches, static or dynamic, are open and IDAC in CSD mode is off 1: switches and IDAC can be closed/on as per MMIO setting and CSD sequencer."]
pub type SENSE_EN_W<'a> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, 12>;
#[doc = "Enable charging of the Cmod/Csh_tank capacitor using the GPIO digital output buffer using the csd_charge signal. Note that using the GPIO requires proper configuraiton of the GPIO pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHARGE_MODE_A {
    #[doc = "0: Use this to keep csd_charge signal low. For charging Cmod/Csh_tank capacitor CSD internal switches (HCBV) can be used but that is a separate configuration."]
    CHARGE_OFF = 0,
    #[doc = "1: Use csd_charge to enable the GPIO Driver to charge capacitor. The capacitor must be sensed with HSCMP using the appropriate switches (HMPM or HMPT)."]
    CHARGE_IO = 1,
}
impl From<CHARGE_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: CHARGE_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHARGE_MODE` reader - Enable charging of the Cmod/Csh_tank capacitor using the GPIO digital output buffer using the csd_charge signal. Note that using the GPIO requires proper configuraiton of the GPIO pin."]
pub type CHARGE_MODE_R = crate::BitReader<CHARGE_MODE_A>;
impl CHARGE_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHARGE_MODE_A {
        match self.bits {
            false => CHARGE_MODE_A::CHARGE_OFF,
            true => CHARGE_MODE_A::CHARGE_IO,
        }
    }
    #[doc = "Checks if the value of the field is `CHARGE_OFF`"]
    #[inline(always)]
    pub fn is_charge_off(&self) -> bool {
        *self == CHARGE_MODE_A::CHARGE_OFF
    }
    #[doc = "Checks if the value of the field is `CHARGE_IO`"]
    #[inline(always)]
    pub fn is_charge_io(&self) -> bool {
        *self == CHARGE_MODE_A::CHARGE_IO
    }
}
#[doc = "Field `CHARGE_MODE` writer - Enable charging of the Cmod/Csh_tank capacitor using the GPIO digital output buffer using the csd_charge signal. Note that using the GPIO requires proper configuraiton of the GPIO pin."]
pub type CHARGE_MODE_W<'a> = crate::BitWriter<'a, u32, CONFIG_SPEC, CHARGE_MODE_A, 14>;
impl<'a> CHARGE_MODE_W<'a> {
    #[doc = "Use this to keep csd_charge signal low. For charging Cmod/Csh_tank capacitor CSD internal switches (HCBV) can be used but that is a separate configuration."]
    #[inline(always)]
    pub fn charge_off(self) -> &'a mut W {
        self.variant(CHARGE_MODE_A::CHARGE_OFF)
    }
    #[doc = "Use csd_charge to enable the GPIO Driver to charge capacitor. The capacitor must be sensed with HSCMP using the appropriate switches (HMPM or HMPT)."]
    #[inline(always)]
    pub fn charge_io(self) -> &'a mut W {
        self.variant(CHARGE_MODE_A::CHARGE_IO)
    }
}
#[doc = "Enables mutual cap sensing mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUTUAL_CAP_A {
    #[doc = "0: Self-cap mode (configure sense line as CSD_SENSE)"]
    SELFCAP = 0,
    #[doc = "1: Mutual-cap mode (configure Tx line as CSD_SENSE, inverted Tx line as CSD_SHIELD and Rx Line as AMUXA). In this mode the polarity bit of the IDAC is controlled by csd_sense."]
    MUTUALCAP = 1,
}
impl From<MUTUAL_CAP_A> for bool {
    #[inline(always)]
    fn from(variant: MUTUAL_CAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MUTUAL_CAP` reader - Enables mutual cap sensing mode"]
pub type MUTUAL_CAP_R = crate::BitReader<MUTUAL_CAP_A>;
impl MUTUAL_CAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUTUAL_CAP_A {
        match self.bits {
            false => MUTUAL_CAP_A::SELFCAP,
            true => MUTUAL_CAP_A::MUTUALCAP,
        }
    }
    #[doc = "Checks if the value of the field is `SELFCAP`"]
    #[inline(always)]
    pub fn is_selfcap(&self) -> bool {
        *self == MUTUAL_CAP_A::SELFCAP
    }
    #[doc = "Checks if the value of the field is `MUTUALCAP`"]
    #[inline(always)]
    pub fn is_mutualcap(&self) -> bool {
        *self == MUTUAL_CAP_A::MUTUALCAP
    }
}
#[doc = "Field `MUTUAL_CAP` writer - Enables mutual cap sensing mode"]
pub type MUTUAL_CAP_W<'a> = crate::BitWriter<'a, u32, CONFIG_SPEC, MUTUAL_CAP_A, 18>;
impl<'a> MUTUAL_CAP_W<'a> {
    #[doc = "Self-cap mode (configure sense line as CSD_SENSE)"]
    #[inline(always)]
    pub fn selfcap(self) -> &'a mut W {
        self.variant(MUTUAL_CAP_A::SELFCAP)
    }
    #[doc = "Mutual-cap mode (configure Tx line as CSD_SENSE, inverted Tx line as CSD_SHIELD and Rx Line as AMUXA). In this mode the polarity bit of the IDAC is controlled by csd_sense."]
    #[inline(always)]
    pub fn mutualcap(self) -> &'a mut W {
        self.variant(MUTUAL_CAP_A::MUTUALCAP)
    }
}
#[doc = "Enable the use of two counters for MUTUAL cap sensing mode (CSX), do not use when MUTUAL_CAP=0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSX_DUAL_CNT_A {
    #[doc = "0: Use one counter for both phases (source and sink)."]
    ONE = 0,
    #[doc = "1: Use two counters, separate count for when csd_sense is high and when csd_sense is low."]
    TWO = 1,
}
impl From<CSX_DUAL_CNT_A> for bool {
    #[inline(always)]
    fn from(variant: CSX_DUAL_CNT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSX_DUAL_CNT` reader - Enable the use of two counters for MUTUAL cap sensing mode (CSX), do not use when MUTUAL_CAP=0"]
pub type CSX_DUAL_CNT_R = crate::BitReader<CSX_DUAL_CNT_A>;
impl CSX_DUAL_CNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSX_DUAL_CNT_A {
        match self.bits {
            false => CSX_DUAL_CNT_A::ONE,
            true => CSX_DUAL_CNT_A::TWO,
        }
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == CSX_DUAL_CNT_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == CSX_DUAL_CNT_A::TWO
    }
}
#[doc = "Field `CSX_DUAL_CNT` writer - Enable the use of two counters for MUTUAL cap sensing mode (CSX), do not use when MUTUAL_CAP=0"]
pub type CSX_DUAL_CNT_W<'a> = crate::BitWriter<'a, u32, CONFIG_SPEC, CSX_DUAL_CNT_A, 19>;
impl<'a> CSX_DUAL_CNT_W<'a> {
    #[doc = "Use one counter for both phases (source and sink)."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CSX_DUAL_CNT_A::ONE)
    }
    #[doc = "Use two counters, separate count for when csd_sense is high and when csd_sense is low."]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(CSX_DUAL_CNT_A::TWO)
    }
}
#[doc = "Select what to output on the dsi_count bus.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSI_COUNT_SEL_A {
    #[doc = "0: depending on the dsi_count_val_sel input either output RESULT_VAL1.VALUE (0) or RESULT_VAL2.VALUE (1) on the dsi_count bus. Note that dsi_count_val_sel is not synchronized, i.e. it controls the mux combinatorially."]
    CSD_RESULT = 0,
    #[doc = "1: output ADC_RES.VIN_CNT on the dsi_count bus"]
    ADC_RESULT = 1,
}
impl From<DSI_COUNT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: DSI_COUNT_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSI_COUNT_SEL` reader - Select what to output on the dsi_count bus."]
pub type DSI_COUNT_SEL_R = crate::BitReader<DSI_COUNT_SEL_A>;
impl DSI_COUNT_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSI_COUNT_SEL_A {
        match self.bits {
            false => DSI_COUNT_SEL_A::CSD_RESULT,
            true => DSI_COUNT_SEL_A::ADC_RESULT,
        }
    }
    #[doc = "Checks if the value of the field is `CSD_RESULT`"]
    #[inline(always)]
    pub fn is_csd_result(&self) -> bool {
        *self == DSI_COUNT_SEL_A::CSD_RESULT
    }
    #[doc = "Checks if the value of the field is `ADC_RESULT`"]
    #[inline(always)]
    pub fn is_adc_result(&self) -> bool {
        *self == DSI_COUNT_SEL_A::ADC_RESULT
    }
}
#[doc = "Field `DSI_COUNT_SEL` writer - Select what to output on the dsi_count bus."]
pub type DSI_COUNT_SEL_W<'a> = crate::BitWriter<'a, u32, CONFIG_SPEC, DSI_COUNT_SEL_A, 24>;
impl<'a> DSI_COUNT_SEL_W<'a> {
    #[doc = "depending on the dsi_count_val_sel input either output RESULT_VAL1.VALUE (0) or RESULT_VAL2.VALUE (1) on the dsi_count bus. Note that dsi_count_val_sel is not synchronized, i.e. it controls the mux combinatorially."]
    #[inline(always)]
    pub fn csd_result(self) -> &'a mut W {
        self.variant(DSI_COUNT_SEL_A::CSD_RESULT)
    }
    #[doc = "output ADC_RES.VIN_CNT on the dsi_count bus"]
    #[inline(always)]
    pub fn adc_result(self) -> &'a mut W {
        self.variant(DSI_COUNT_SEL_A::ADC_RESULT)
    }
}
#[doc = "Field `DSI_SAMPLE_EN` reader - Enables the use of the dsi_sample_in input instead of the comparator output to strobe COUNTER."]
pub type DSI_SAMPLE_EN_R = crate::BitReader<bool>;
#[doc = "Field `DSI_SAMPLE_EN` writer - Enables the use of the dsi_sample_in input instead of the comparator output to strobe COUNTER."]
pub type DSI_SAMPLE_EN_W<'a> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, 25>;
#[doc = "Field `SAMPLE_SYNC` reader - Enables double synchronizing of sample input from DSI (only relevant when DSI_SAMPLE_EN=1)."]
pub type SAMPLE_SYNC_R = crate::BitReader<bool>;
#[doc = "Field `SAMPLE_SYNC` writer - Enables double synchronizing of sample input from DSI (only relevant when DSI_SAMPLE_EN=1)."]
pub type SAMPLE_SYNC_W<'a> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, 26>;
#[doc = "Field `DSI_SENSE_EN` reader - Enables the use of the dsi_sense_in input instead of the internally generated modulation signal to drive csd_sense and csd_shield signals."]
pub type DSI_SENSE_EN_R = crate::BitReader<bool>;
#[doc = "Field `DSI_SENSE_EN` writer - Enables the use of the dsi_sense_in input instead of the internally generated modulation signal to drive csd_sense and csd_shield signals."]
pub type DSI_SENSE_EN_W<'a> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, 27>;
#[doc = "Field `LP_MODE` reader - Select the power mode for the CSD components (REFGEN, AMBUF, CSDCMP, HSCMP): 0: High Power mode 1: Low Power mode"]
pub type LP_MODE_R = crate::BitReader<bool>;
#[doc = "Field `LP_MODE` writer - Select the power mode for the CSD components (REFGEN, AMBUF, CSDCMP, HSCMP): 0: High Power mode 1: Low Power mode"]
pub type LP_MODE_W<'a> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, 30>;
#[doc = "Field `ENABLE` reader - Master enable of the CSDv2 IP. Must be set to 1 for any CSDv2, ADC or IDAC operation to function. When 0 all analog components will be off and all switches will be open."]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Master enable of the CSDv2 IP. Must be set to 1 for any CSDv2, ADC or IDAC operation to function. When 0 all analog components will be off and all switches will be open."]
pub type ENABLE_W<'a> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 4:6 - This value determines the number of cycles that the digital filter makes the CSDCMP output ignored while the counter counts and IDAC is on. When set to 0 the digital filter is off. When set to any other value the ignoring will last for FILTER_DELAY clk_csd cycles after the start of each measurement and from the first comparator trip to the end of each measurement."]
    #[inline(always)]
    pub fn filter_delay(&self) -> FILTER_DELAY_R {
        FILTER_DELAY_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9 - Selects the delay by which csd_shield is delayed relative to csd_sense."]
    #[inline(always)]
    pub fn shield_delay(&self) -> SHIELD_DELAY_R {
        SHIELD_DELAY_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - Enables the sense modulator output. 0: all switches, static or dynamic, are open and IDAC in CSD mode is off 1: switches and IDAC can be closed/on as per MMIO setting and CSD sequencer."]
    #[inline(always)]
    pub fn sense_en(&self) -> SENSE_EN_R {
        SENSE_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable charging of the Cmod/Csh_tank capacitor using the GPIO digital output buffer using the csd_charge signal. Note that using the GPIO requires proper configuraiton of the GPIO pin."]
    #[inline(always)]
    pub fn charge_mode(&self) -> CHARGE_MODE_R {
        CHARGE_MODE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - Enables mutual cap sensing mode"]
    #[inline(always)]
    pub fn mutual_cap(&self) -> MUTUAL_CAP_R {
        MUTUAL_CAP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable the use of two counters for MUTUAL cap sensing mode (CSX), do not use when MUTUAL_CAP=0"]
    #[inline(always)]
    pub fn csx_dual_cnt(&self) -> CSX_DUAL_CNT_R {
        CSX_DUAL_CNT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Select what to output on the dsi_count bus."]
    #[inline(always)]
    pub fn dsi_count_sel(&self) -> DSI_COUNT_SEL_R {
        DSI_COUNT_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enables the use of the dsi_sample_in input instead of the comparator output to strobe COUNTER."]
    #[inline(always)]
    pub fn dsi_sample_en(&self) -> DSI_SAMPLE_EN_R {
        DSI_SAMPLE_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enables double synchronizing of sample input from DSI (only relevant when DSI_SAMPLE_EN=1)."]
    #[inline(always)]
    pub fn sample_sync(&self) -> SAMPLE_SYNC_R {
        SAMPLE_SYNC_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enables the use of the dsi_sense_in input instead of the internally generated modulation signal to drive csd_sense and csd_shield signals."]
    #[inline(always)]
    pub fn dsi_sense_en(&self) -> DSI_SENSE_EN_R {
        DSI_SENSE_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - Select the power mode for the CSD components (REFGEN, AMBUF, CSDCMP, HSCMP): 0: High Power mode 1: Low Power mode"]
    #[inline(always)]
    pub fn lp_mode(&self) -> LP_MODE_R {
        LP_MODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Master enable of the CSDv2 IP. Must be set to 1 for any CSDv2, ADC or IDAC operation to function. When 0 all analog components will be off and all switches will be open."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:6 - This value determines the number of cycles that the digital filter makes the CSDCMP output ignored while the counter counts and IDAC is on. When set to 0 the digital filter is off. When set to any other value the ignoring will last for FILTER_DELAY clk_csd cycles after the start of each measurement and from the first comparator trip to the end of each measurement."]
    #[inline(always)]
    pub fn filter_delay(&mut self) -> FILTER_DELAY_W {
        FILTER_DELAY_W::new(self)
    }
    #[doc = "Bits 8:9 - Selects the delay by which csd_shield is delayed relative to csd_sense."]
    #[inline(always)]
    pub fn shield_delay(&mut self) -> SHIELD_DELAY_W {
        SHIELD_DELAY_W::new(self)
    }
    #[doc = "Bit 12 - Enables the sense modulator output. 0: all switches, static or dynamic, are open and IDAC in CSD mode is off 1: switches and IDAC can be closed/on as per MMIO setting and CSD sequencer."]
    #[inline(always)]
    pub fn sense_en(&mut self) -> SENSE_EN_W {
        SENSE_EN_W::new(self)
    }
    #[doc = "Bit 14 - Enable charging of the Cmod/Csh_tank capacitor using the GPIO digital output buffer using the csd_charge signal. Note that using the GPIO requires proper configuraiton of the GPIO pin."]
    #[inline(always)]
    pub fn charge_mode(&mut self) -> CHARGE_MODE_W {
        CHARGE_MODE_W::new(self)
    }
    #[doc = "Bit 18 - Enables mutual cap sensing mode"]
    #[inline(always)]
    pub fn mutual_cap(&mut self) -> MUTUAL_CAP_W {
        MUTUAL_CAP_W::new(self)
    }
    #[doc = "Bit 19 - Enable the use of two counters for MUTUAL cap sensing mode (CSX), do not use when MUTUAL_CAP=0"]
    #[inline(always)]
    pub fn csx_dual_cnt(&mut self) -> CSX_DUAL_CNT_W {
        CSX_DUAL_CNT_W::new(self)
    }
    #[doc = "Bit 24 - Select what to output on the dsi_count bus."]
    #[inline(always)]
    pub fn dsi_count_sel(&mut self) -> DSI_COUNT_SEL_W {
        DSI_COUNT_SEL_W::new(self)
    }
    #[doc = "Bit 25 - Enables the use of the dsi_sample_in input instead of the comparator output to strobe COUNTER."]
    #[inline(always)]
    pub fn dsi_sample_en(&mut self) -> DSI_SAMPLE_EN_W {
        DSI_SAMPLE_EN_W::new(self)
    }
    #[doc = "Bit 26 - Enables double synchronizing of sample input from DSI (only relevant when DSI_SAMPLE_EN=1)."]
    #[inline(always)]
    pub fn sample_sync(&mut self) -> SAMPLE_SYNC_W {
        SAMPLE_SYNC_W::new(self)
    }
    #[doc = "Bit 27 - Enables the use of the dsi_sense_in input instead of the internally generated modulation signal to drive csd_sense and csd_shield signals."]
    #[inline(always)]
    pub fn dsi_sense_en(&mut self) -> DSI_SENSE_EN_W {
        DSI_SENSE_EN_W::new(self)
    }
    #[doc = "Bit 30 - Select the power mode for the CSD components (REFGEN, AMBUF, CSDCMP, HSCMP): 0: High Power mode 1: Low Power mode"]
    #[inline(always)]
    pub fn lp_mode(&mut self) -> LP_MODE_W {
        LP_MODE_W::new(self)
    }
    #[doc = "Bit 31 - Master enable of the CSDv2 IP. Must be set to 1 for any CSDv2, ADC or IDAC operation to function. When 0 all analog components will be off and all switches will be open."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration and Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](index.html) module"]
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
#[doc = "`reset()` method sets CONFIG to value 0x0400_0000"]
impl crate::Resettable for CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0400_0000
    }
}
