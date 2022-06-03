#[doc = "Register `MUX_SWITCH1` reader"]
pub struct R(crate::R<MUX_SWITCH1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MUX_SWITCH1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MUX_SWITCH1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MUX_SWITCH1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MUX_SWITCH1` writer"]
pub struct W(crate::W<MUX_SWITCH1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MUX_SWITCH1_SPEC>;
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
impl From<crate::W<MUX_SWITCH1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MUX_SWITCH1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MUX_FW_P4_DFT_INP` reader - Firmware control: 0=open, 1=close switch between P4 pin and dft_inp signal. Write with '1' to set bit."]
pub type MUX_FW_P4_DFT_INP_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_P4_DFT_INP` writer - Firmware control: 0=open, 1=close switch between P4 pin and dft_inp signal. Write with '1' to set bit."]
pub type MUX_FW_P4_DFT_INP_W<'a> = crate::BitWriter<'a, u32, MUX_SWITCH1_SPEC, bool, 0>;
#[doc = "Field `MUX_FW_P5_DFT_INM` reader - Firmware control: 0=open, 1=close switch between P5 pin and dft_inm signal. Write with '1' to set bit."]
pub type MUX_FW_P5_DFT_INM_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_P5_DFT_INM` writer - Firmware control: 0=open, 1=close switch between P5 pin and dft_inm signal. Write with '1' to set bit."]
pub type MUX_FW_P5_DFT_INM_W<'a> = crate::BitWriter<'a, u32, MUX_SWITCH1_SPEC, bool, 1>;
#[doc = "Field `MUX_FW_ADFT0_SARBUS0` reader - Firmware control: 0=open, 1=close switch between adft0 signal and sarbus0 signal. Write with '1' to set bit."]
pub type MUX_FW_ADFT0_SARBUS0_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_ADFT0_SARBUS0` writer - Firmware control: 0=open, 1=close switch between adft0 signal and sarbus0 signal. Write with '1' to set bit."]
pub type MUX_FW_ADFT0_SARBUS0_W<'a> = crate::BitWriter<'a, u32, MUX_SWITCH1_SPEC, bool, 2>;
#[doc = "Field `MUX_FW_ADFT1_SARBUS1` reader - Firmware control: 0=open, 1=close switch between adft1 signal and sarbus1 signal. Write with '1' to set bit."]
pub type MUX_FW_ADFT1_SARBUS1_R = crate::BitReader<bool>;
#[doc = "Field `MUX_FW_ADFT1_SARBUS1` writer - Firmware control: 0=open, 1=close switch between adft1 signal and sarbus1 signal. Write with '1' to set bit."]
pub type MUX_FW_ADFT1_SARBUS1_W<'a> = crate::BitWriter<'a, u32, MUX_SWITCH1_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - Firmware control: 0=open, 1=close switch between P4 pin and dft_inp signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p4_dft_inp(&self) -> MUX_FW_P4_DFT_INP_R {
        MUX_FW_P4_DFT_INP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Firmware control: 0=open, 1=close switch between P5 pin and dft_inm signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p5_dft_inm(&self) -> MUX_FW_P5_DFT_INM_R {
        MUX_FW_P5_DFT_INM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Firmware control: 0=open, 1=close switch between adft0 signal and sarbus0 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_adft0_sarbus0(&self) -> MUX_FW_ADFT0_SARBUS0_R {
        MUX_FW_ADFT0_SARBUS0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Firmware control: 0=open, 1=close switch between adft1 signal and sarbus1 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_adft1_sarbus1(&self) -> MUX_FW_ADFT1_SARBUS1_R {
        MUX_FW_ADFT1_SARBUS1_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Firmware control: 0=open, 1=close switch between P4 pin and dft_inp signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p4_dft_inp(&mut self) -> MUX_FW_P4_DFT_INP_W {
        MUX_FW_P4_DFT_INP_W::new(self)
    }
    #[doc = "Bit 1 - Firmware control: 0=open, 1=close switch between P5 pin and dft_inm signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_p5_dft_inm(&mut self) -> MUX_FW_P5_DFT_INM_W {
        MUX_FW_P5_DFT_INM_W::new(self)
    }
    #[doc = "Bit 2 - Firmware control: 0=open, 1=close switch between adft0 signal and sarbus0 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_adft0_sarbus0(&mut self) -> MUX_FW_ADFT0_SARBUS0_W {
        MUX_FW_ADFT0_SARBUS0_W::new(self)
    }
    #[doc = "Bit 3 - Firmware control: 0=open, 1=close switch between adft1 signal and sarbus1 signal. Write with '1' to set bit."]
    #[inline(always)]
    pub fn mux_fw_adft1_sarbus1(&mut self) -> MUX_FW_ADFT1_SARBUS1_W {
        MUX_FW_ADFT1_SARBUS1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SARMUX Firmware switch controls\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mux_switch1](index.html) module"]
pub struct MUX_SWITCH1_SPEC;
impl crate::RegisterSpec for MUX_SWITCH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mux_switch1::R](R) reader structure"]
impl crate::Readable for MUX_SWITCH1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mux_switch1::W](W) writer structure"]
impl crate::Writable for MUX_SWITCH1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MUX_SWITCH1 to value 0"]
impl crate::Resettable for MUX_SWITCH1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
