#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BYPASS` reader - Bypass of the programmable IO, one bit for each IO pin: BYPASS\\[i\\]
is for IO pin i. When ENABLED is '1', this field is used. When ENABLED is '0', this field is NOT used and PRGIO is always bypassed. '0': No bypass (programmable IO fabric is exposed). '1': Bypass (programmable IO fabric is hidden)."]
pub type BYPASS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BYPASS` writer - Bypass of the programmable IO, one bit for each IO pin: BYPASS\\[i\\]
is for IO pin i. When ENABLED is '1', this field is used. When ENABLED is '0', this field is NOT used and PRGIO is always bypassed. '0': No bypass (programmable IO fabric is exposed). '1': Bypass (programmable IO fabric is hidden)."]
pub type BYPASS_W<'a> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 8, 0>;
#[doc = "Field `CLOCK_SRC` reader - Clock ('clk_fabric') and reset ('rst_fabric_n') source selection: '0': io_data_in\\[0\\]/'1'. ... '7': io_data_in\\[7\\]/'1'. '8': chip_data\\[0\\]/'1'. ... '15': chip_data\\[7\\]/'1'. '16': clk_prgio/rst_sys_act_n. Used for both Active functionality synchronous logic on 'clk_prgio'. This selection is intended for synchronous operation on a PCLK specified clock frequency ('clock_prgio_en'). Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to 'clk_sys'. '17': clk_prgio/rst_sys_dpslp_n. Used for both DeepSleep functionality synchronous logic on 'clk_prgio' (note that 'clk_prgio' is NOT available in DeepSleep and Hibernate power modes). This selection is intended for synchronous operation on a PCLK specified clock frequency ('clock_prgio_en'). Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to 'clk_sys'. '18': clk_prgio/rst_sys_hib_n. Used for both Hibernate functionality synchronous logic on 'clk_prgio' (note that 'clk_prgio' is NOT available in DeepSleep and Hibernate power modes). This selection is intended for synchronous operation on a PCLK specified clock frequency ('clock_prgio_en'). Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to 'clk_sys'. '19': clk_lf/rst_lf_dpslp_n (note that 'clk_lf' is only available in DeepSleep power mode). This selection is intended for synchronous operation on'clk_lf'. Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to other 'clk_lf' clocked elements. '20'-'30': Clock source is constant '0'. Any of these clock sources should be selected when the IP is disabled to ensure low power consumption. '31': clk_sys/'1'. This selection is NOT intended for 'clk_sys' operation, but for asynchronous operation: three 'clk_sys' cycles after enabling the IP, the IP is fully functional (reset is de-activated). To be used for asynchronous (clockless) fabric functionality."]
pub type CLOCK_SRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLOCK_SRC` writer - Clock ('clk_fabric') and reset ('rst_fabric_n') source selection: '0': io_data_in\\[0\\]/'1'. ... '7': io_data_in\\[7\\]/'1'. '8': chip_data\\[0\\]/'1'. ... '15': chip_data\\[7\\]/'1'. '16': clk_prgio/rst_sys_act_n. Used for both Active functionality synchronous logic on 'clk_prgio'. This selection is intended for synchronous operation on a PCLK specified clock frequency ('clock_prgio_en'). Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to 'clk_sys'. '17': clk_prgio/rst_sys_dpslp_n. Used for both DeepSleep functionality synchronous logic on 'clk_prgio' (note that 'clk_prgio' is NOT available in DeepSleep and Hibernate power modes). This selection is intended for synchronous operation on a PCLK specified clock frequency ('clock_prgio_en'). Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to 'clk_sys'. '18': clk_prgio/rst_sys_hib_n. Used for both Hibernate functionality synchronous logic on 'clk_prgio' (note that 'clk_prgio' is NOT available in DeepSleep and Hibernate power modes). This selection is intended for synchronous operation on a PCLK specified clock frequency ('clock_prgio_en'). Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to 'clk_sys'. '19': clk_lf/rst_lf_dpslp_n (note that 'clk_lf' is only available in DeepSleep power mode). This selection is intended for synchronous operation on'clk_lf'. Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to other 'clk_lf' clocked elements. '20'-'30': Clock source is constant '0'. Any of these clock sources should be selected when the IP is disabled to ensure low power consumption. '31': clk_sys/'1'. This selection is NOT intended for 'clk_sys' operation, but for asynchronous operation: three 'clk_sys' cycles after enabling the IP, the IP is fully functional (reset is de-activated). To be used for asynchronous (clockless) fabric functionality."]
pub type CLOCK_SRC_W<'a> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 5, 8>;
#[doc = "Field `HLD_OVR` reader - IO cell hold override functionality. In DeepSleep and Hibernate power modes, the HSIOM holds the IO cell output and output enable signals if Active functionality is connected to the IO pads. This is undesirable if the PRGIO is supposed to deliver DeepSleep or Hibernate output functionality on these IO pads. This field is used to control the hold override functionality from the PRGIO: '0': The HSIOM controls the IO cell hold override functionality ('hsiom_hld_ovr'). '1': The PRGIO controls the IO cel hold override functionality: - In bypass mode (ENABLED is '0' or BYPASS\\[i\\]
is '1'), the HSIOM control is used. - In NON bypass mode (ENABLED is '1' and BYPASS\\[i\\]
is '0'), the PRGIO sets hold override to 'pwr_hld_ovr_hib' to enable PRGIO functionality in DeepSleep and Hibernate power modes (but disables it in Stop power mode). Note that in Hibernate power mode, the PRGIO should not rely on the state of Active or DeepSleep functionality signals from the HSIOM: these signals are clamped to '0' in Hibernate'"]
pub type HLD_OVR_R = crate::BitReader<bool>;
#[doc = "Field `HLD_OVR` writer - IO cell hold override functionality. In DeepSleep and Hibernate power modes, the HSIOM holds the IO cell output and output enable signals if Active functionality is connected to the IO pads. This is undesirable if the PRGIO is supposed to deliver DeepSleep or Hibernate output functionality on these IO pads. This field is used to control the hold override functionality from the PRGIO: '0': The HSIOM controls the IO cell hold override functionality ('hsiom_hld_ovr'). '1': The PRGIO controls the IO cel hold override functionality: - In bypass mode (ENABLED is '0' or BYPASS\\[i\\]
is '1'), the HSIOM control is used. - In NON bypass mode (ENABLED is '1' and BYPASS\\[i\\]
is '0'), the PRGIO sets hold override to 'pwr_hld_ovr_hib' to enable PRGIO functionality in DeepSleep and Hibernate power modes (but disables it in Stop power mode). Note that in Hibernate power mode, the PRGIO should not rely on the state of Active or DeepSleep functionality signals from the HSIOM: these signals are clamped to '0' in Hibernate'"]
pub type HLD_OVR_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 24>;
#[doc = "Field `PIPELINE_EN` reader - Enable for pipeline register: '0': Disabled (register is bypassed). '1': Enabled."]
pub type PIPELINE_EN_R = crate::BitReader<bool>;
#[doc = "Field `PIPELINE_EN` writer - Enable for pipeline register: '0': Disabled (register is bypassed). '1': Enabled."]
pub type PIPELINE_EN_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 25>;
#[doc = "Field `ENABLED` reader - Enable for programmable IO. Should only be set to '1' when the programmable IO is completely configured: '0': Disabled (signals are bypassed; behavior as if BYPASS is 0xFF). When disabled, the fabric (data unit and LUTs) reset is activated. If the IP is disabled: - The PIPELINE_EN register field should be set to '1', to ensure low power consumption by preventing combinatorial loops. - The CLOCK_SRC register field should be set to '20'-'30' (clock is constant '0'), to ensure low power consumption. '1': Enabled. Once enabled, it takes 3 'clk_fabric' clock cycles till the fabric reset is de-activated and the fabric becomes fully functional. This ensures that the IO pins' input synchronizer states are flushed when the fabric is fully functional."]
pub type ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ENABLED` writer - Enable for programmable IO. Should only be set to '1' when the programmable IO is completely configured: '0': Disabled (signals are bypassed; behavior as if BYPASS is 0xFF). When disabled, the fabric (data unit and LUTs) reset is activated. If the IP is disabled: - The PIPELINE_EN register field should be set to '1', to ensure low power consumption by preventing combinatorial loops. - The CLOCK_SRC register field should be set to '20'-'30' (clock is constant '0'), to ensure low power consumption. '1': Enabled. Once enabled, it takes 3 'clk_fabric' clock cycles till the fabric reset is de-activated and the fabric becomes fully functional. This ensures that the IO pins' input synchronizer states are flushed when the fabric is fully functional."]
pub type ENABLED_W<'a> = crate::BitWriter<'a, u32, CTL_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:7 - Bypass of the programmable IO, one bit for each IO pin: BYPASS\\[i\\]
is for IO pin i. When ENABLED is '1', this field is used. When ENABLED is '0', this field is NOT used and PRGIO is always bypassed. '0': No bypass (programmable IO fabric is exposed). '1': Bypass (programmable IO fabric is hidden)."]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - Clock ('clk_fabric') and reset ('rst_fabric_n') source selection: '0': io_data_in\\[0\\]/'1'. ... '7': io_data_in\\[7\\]/'1'. '8': chip_data\\[0\\]/'1'. ... '15': chip_data\\[7\\]/'1'. '16': clk_prgio/rst_sys_act_n. Used for both Active functionality synchronous logic on 'clk_prgio'. This selection is intended for synchronous operation on a PCLK specified clock frequency ('clock_prgio_en'). Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to 'clk_sys'. '17': clk_prgio/rst_sys_dpslp_n. Used for both DeepSleep functionality synchronous logic on 'clk_prgio' (note that 'clk_prgio' is NOT available in DeepSleep and Hibernate power modes). This selection is intended for synchronous operation on a PCLK specified clock frequency ('clock_prgio_en'). Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to 'clk_sys'. '18': clk_prgio/rst_sys_hib_n. Used for both Hibernate functionality synchronous logic on 'clk_prgio' (note that 'clk_prgio' is NOT available in DeepSleep and Hibernate power modes). This selection is intended for synchronous operation on a PCLK specified clock frequency ('clock_prgio_en'). Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to 'clk_sys'. '19': clk_lf/rst_lf_dpslp_n (note that 'clk_lf' is only available in DeepSleep power mode). This selection is intended for synchronous operation on'clk_lf'. Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to other 'clk_lf' clocked elements. '20'-'30': Clock source is constant '0'. Any of these clock sources should be selected when the IP is disabled to ensure low power consumption. '31': clk_sys/'1'. This selection is NOT intended for 'clk_sys' operation, but for asynchronous operation: three 'clk_sys' cycles after enabling the IP, the IP is fully functional (reset is de-activated). To be used for asynchronous (clockless) fabric functionality."]
    #[inline(always)]
    pub fn clock_src(&self) -> CLOCK_SRC_R {
        CLOCK_SRC_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - IO cell hold override functionality. In DeepSleep and Hibernate power modes, the HSIOM holds the IO cell output and output enable signals if Active functionality is connected to the IO pads. This is undesirable if the PRGIO is supposed to deliver DeepSleep or Hibernate output functionality on these IO pads. This field is used to control the hold override functionality from the PRGIO: '0': The HSIOM controls the IO cell hold override functionality ('hsiom_hld_ovr'). '1': The PRGIO controls the IO cel hold override functionality: - In bypass mode (ENABLED is '0' or BYPASS\\[i\\]
is '1'), the HSIOM control is used. - In NON bypass mode (ENABLED is '1' and BYPASS\\[i\\]
is '0'), the PRGIO sets hold override to 'pwr_hld_ovr_hib' to enable PRGIO functionality in DeepSleep and Hibernate power modes (but disables it in Stop power mode). Note that in Hibernate power mode, the PRGIO should not rely on the state of Active or DeepSleep functionality signals from the HSIOM: these signals are clamped to '0' in Hibernate'"]
    #[inline(always)]
    pub fn hld_ovr(&self) -> HLD_OVR_R {
        HLD_OVR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable for pipeline register: '0': Disabled (register is bypassed). '1': Enabled."]
    #[inline(always)]
    pub fn pipeline_en(&self) -> PIPELINE_EN_R {
        PIPELINE_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable for programmable IO. Should only be set to '1' when the programmable IO is completely configured: '0': Disabled (signals are bypassed; behavior as if BYPASS is 0xFF). When disabled, the fabric (data unit and LUTs) reset is activated. If the IP is disabled: - The PIPELINE_EN register field should be set to '1', to ensure low power consumption by preventing combinatorial loops. - The CLOCK_SRC register field should be set to '20'-'30' (clock is constant '0'), to ensure low power consumption. '1': Enabled. Once enabled, it takes 3 'clk_fabric' clock cycles till the fabric reset is de-activated and the fabric becomes fully functional. This ensures that the IO pins' input synchronizer states are flushed when the fabric is fully functional."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bypass of the programmable IO, one bit for each IO pin: BYPASS\\[i\\]
is for IO pin i. When ENABLED is '1', this field is used. When ENABLED is '0', this field is NOT used and PRGIO is always bypassed. '0': No bypass (programmable IO fabric is exposed). '1': Bypass (programmable IO fabric is hidden)."]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W::new(self)
    }
    #[doc = "Bits 8:12 - Clock ('clk_fabric') and reset ('rst_fabric_n') source selection: '0': io_data_in\\[0\\]/'1'. ... '7': io_data_in\\[7\\]/'1'. '8': chip_data\\[0\\]/'1'. ... '15': chip_data\\[7\\]/'1'. '16': clk_prgio/rst_sys_act_n. Used for both Active functionality synchronous logic on 'clk_prgio'. This selection is intended for synchronous operation on a PCLK specified clock frequency ('clock_prgio_en'). Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to 'clk_sys'. '17': clk_prgio/rst_sys_dpslp_n. Used for both DeepSleep functionality synchronous logic on 'clk_prgio' (note that 'clk_prgio' is NOT available in DeepSleep and Hibernate power modes). This selection is intended for synchronous operation on a PCLK specified clock frequency ('clock_prgio_en'). Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to 'clk_sys'. '18': clk_prgio/rst_sys_hib_n. Used for both Hibernate functionality synchronous logic on 'clk_prgio' (note that 'clk_prgio' is NOT available in DeepSleep and Hibernate power modes). This selection is intended for synchronous operation on a PCLK specified clock frequency ('clock_prgio_en'). Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to 'clk_sys'. '19': clk_lf/rst_lf_dpslp_n (note that 'clk_lf' is only available in DeepSleep power mode). This selection is intended for synchronous operation on'clk_lf'. Note that the fabric's clocked elements are frequency aligned, but NOT phase aligned to other 'clk_lf' clocked elements. '20'-'30': Clock source is constant '0'. Any of these clock sources should be selected when the IP is disabled to ensure low power consumption. '31': clk_sys/'1'. This selection is NOT intended for 'clk_sys' operation, but for asynchronous operation: three 'clk_sys' cycles after enabling the IP, the IP is fully functional (reset is de-activated). To be used for asynchronous (clockless) fabric functionality."]
    #[inline(always)]
    pub fn clock_src(&mut self) -> CLOCK_SRC_W {
        CLOCK_SRC_W::new(self)
    }
    #[doc = "Bit 24 - IO cell hold override functionality. In DeepSleep and Hibernate power modes, the HSIOM holds the IO cell output and output enable signals if Active functionality is connected to the IO pads. This is undesirable if the PRGIO is supposed to deliver DeepSleep or Hibernate output functionality on these IO pads. This field is used to control the hold override functionality from the PRGIO: '0': The HSIOM controls the IO cell hold override functionality ('hsiom_hld_ovr'). '1': The PRGIO controls the IO cel hold override functionality: - In bypass mode (ENABLED is '0' or BYPASS\\[i\\]
is '1'), the HSIOM control is used. - In NON bypass mode (ENABLED is '1' and BYPASS\\[i\\]
is '0'), the PRGIO sets hold override to 'pwr_hld_ovr_hib' to enable PRGIO functionality in DeepSleep and Hibernate power modes (but disables it in Stop power mode). Note that in Hibernate power mode, the PRGIO should not rely on the state of Active or DeepSleep functionality signals from the HSIOM: these signals are clamped to '0' in Hibernate'"]
    #[inline(always)]
    pub fn hld_ovr(&mut self) -> HLD_OVR_W {
        HLD_OVR_W::new(self)
    }
    #[doc = "Bit 25 - Enable for pipeline register: '0': Disabled (register is bypassed). '1': Enabled."]
    #[inline(always)]
    pub fn pipeline_en(&mut self) -> PIPELINE_EN_W {
        PIPELINE_EN_W::new(self)
    }
    #[doc = "Bit 31 - Enable for programmable IO. Should only be set to '1' when the programmable IO is completely configured: '0': Disabled (signals are bypassed; behavior as if BYPASS is 0xFF). When disabled, the fabric (data unit and LUTs) reset is activated. If the IP is disabled: - The PIPELINE_EN register field should be set to '1', to ensure low power consumption by preventing combinatorial loops. - The CLOCK_SRC register field should be set to '20'-'30' (clock is constant '0'), to ensure low power consumption. '1': Enabled. Once enabled, it takes 3 'clk_fabric' clock cycles till the fabric reset is de-activated and the fabric becomes fully functional. This ensures that the IO pins' input synchronizer states are flushed when the fabric is fully functional."]
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
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTL to value 0x0200_1400"]
impl crate::Resettable for CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_1400
    }
}
