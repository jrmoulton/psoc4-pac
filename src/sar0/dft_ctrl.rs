#[doc = "Register `DFT_CTRL` reader"]
pub struct R(crate::R<DFT_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFT_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFT_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFT_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFT_CTRL` writer"]
pub struct W(crate::W<DFT_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFT_CTRL_SPEC>;
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
impl From<crate::W<DFT_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFT_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLY_INC` reader - DFT control: Control for delay circuits on sampling phase, =1 doubes the non-overlap delay"]
pub type DLY_INC_R = crate::BitReader<bool>;
#[doc = "Field `DLY_INC` writer - DFT control: Control for delay circuits on sampling phase, =1 doubes the non-overlap delay"]
pub type DLY_INC_W<'a> = crate::BitWriter<'a, u32, DFT_CTRL_SPEC, bool, 0>;
#[doc = "Field `HIZ` reader - DFT control for getting higher input impedance, must be 1 (0 is deprecated)"]
pub type HIZ_R = crate::BitReader<bool>;
#[doc = "Field `HIZ` writer - DFT control for getting higher input impedance, must be 1 (0 is deprecated)"]
pub type HIZ_W<'a> = crate::BitWriter<'a, u32, DFT_CTRL_SPEC, bool, 1>;
#[doc = "Field `DFT_INC` reader - DFT control for preamp inputs"]
pub type DFT_INC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DFT_INC` writer - DFT control for preamp inputs"]
pub type DFT_INC_W<'a> = crate::FieldWriter<'a, u32, DFT_CTRL_SPEC, u8, u8, 4, 16>;
#[doc = "Field `DFT_OUTC` reader - DFT control for preamp outputs"]
pub type DFT_OUTC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DFT_OUTC` writer - DFT control for preamp outputs"]
pub type DFT_OUTC_W<'a> = crate::FieldWriter<'a, u32, DFT_CTRL_SPEC, u8, u8, 3, 20>;
#[doc = "Field `SEL_CSEL_DFT` reader - Usage 1: DFT bits for DAC array Usage 2: For \\[0\\]=1 (when dcen=0): Delay timing for latch enable increased by 20 percent \\[1\\]=1: comparator preamp power level increased by 25 percent"]
pub type SEL_CSEL_DFT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL_CSEL_DFT` writer - Usage 1: DFT bits for DAC array Usage 2: For \\[0\\]=1 (when dcen=0): Delay timing for latch enable increased by 20 percent \\[1\\]=1: comparator preamp power level increased by 25 percent"]
pub type SEL_CSEL_DFT_W<'a> = crate::FieldWriter<'a, u32, DFT_CTRL_SPEC, u8, u8, 4, 24>;
#[doc = "Field `EN_CSEL_DFT` reader - Mux select signal for DAC control"]
pub type EN_CSEL_DFT_R = crate::BitReader<bool>;
#[doc = "Field `EN_CSEL_DFT` writer - Mux select signal for DAC control"]
pub type EN_CSEL_DFT_W<'a> = crate::BitWriter<'a, u32, DFT_CTRL_SPEC, bool, 28>;
#[doc = "Field `DCEN` reader - Delay Control Enable for latch. - 0: doubles the latch enable time. - 1: normal latch enable time (default)."]
pub type DCEN_R = crate::BitReader<bool>;
#[doc = "Field `DCEN` writer - Delay Control Enable for latch. - 0: doubles the latch enable time. - 1: normal latch enable time (default)."]
pub type DCEN_W<'a> = crate::BitWriter<'a, u32, DFT_CTRL_SPEC, bool, 29>;
#[doc = "Field `ADFT_OVERRIDE` reader - During deepsleep/ hibernate mode keep SARMUX active, i.e. do not open all switches (disconnect), to be used for ADFT"]
pub type ADFT_OVERRIDE_R = crate::BitReader<bool>;
#[doc = "Field `ADFT_OVERRIDE` writer - During deepsleep/ hibernate mode keep SARMUX active, i.e. do not open all switches (disconnect), to be used for ADFT"]
pub type ADFT_OVERRIDE_W<'a> = crate::BitWriter<'a, u32, DFT_CTRL_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - DFT control: Control for delay circuits on sampling phase, =1 doubes the non-overlap delay"]
    #[inline(always)]
    pub fn dly_inc(&self) -> DLY_INC_R {
        DLY_INC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DFT control for getting higher input impedance, must be 1 (0 is deprecated)"]
    #[inline(always)]
    pub fn hiz(&self) -> HIZ_R {
        HIZ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:19 - DFT control for preamp inputs"]
    #[inline(always)]
    pub fn dft_inc(&self) -> DFT_INC_R {
        DFT_INC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - DFT control for preamp outputs"]
    #[inline(always)]
    pub fn dft_outc(&self) -> DFT_OUTC_R {
        DFT_OUTC_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:27 - Usage 1: DFT bits for DAC array Usage 2: For \\[0\\]=1 (when dcen=0): Delay timing for latch enable increased by 20 percent \\[1\\]=1: comparator preamp power level increased by 25 percent"]
    #[inline(always)]
    pub fn sel_csel_dft(&self) -> SEL_CSEL_DFT_R {
        SEL_CSEL_DFT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Mux select signal for DAC control"]
    #[inline(always)]
    pub fn en_csel_dft(&self) -> EN_CSEL_DFT_R {
        EN_CSEL_DFT_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Delay Control Enable for latch. - 0: doubles the latch enable time. - 1: normal latch enable time (default)."]
    #[inline(always)]
    pub fn dcen(&self) -> DCEN_R {
        DCEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - During deepsleep/ hibernate mode keep SARMUX active, i.e. do not open all switches (disconnect), to be used for ADFT"]
    #[inline(always)]
    pub fn adft_override(&self) -> ADFT_OVERRIDE_R {
        ADFT_OVERRIDE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DFT control: Control for delay circuits on sampling phase, =1 doubes the non-overlap delay"]
    #[inline(always)]
    pub fn dly_inc(&mut self) -> DLY_INC_W {
        DLY_INC_W::new(self)
    }
    #[doc = "Bit 1 - DFT control for getting higher input impedance, must be 1 (0 is deprecated)"]
    #[inline(always)]
    pub fn hiz(&mut self) -> HIZ_W {
        HIZ_W::new(self)
    }
    #[doc = "Bits 16:19 - DFT control for preamp inputs"]
    #[inline(always)]
    pub fn dft_inc(&mut self) -> DFT_INC_W {
        DFT_INC_W::new(self)
    }
    #[doc = "Bits 20:22 - DFT control for preamp outputs"]
    #[inline(always)]
    pub fn dft_outc(&mut self) -> DFT_OUTC_W {
        DFT_OUTC_W::new(self)
    }
    #[doc = "Bits 24:27 - Usage 1: DFT bits for DAC array Usage 2: For \\[0\\]=1 (when dcen=0): Delay timing for latch enable increased by 20 percent \\[1\\]=1: comparator preamp power level increased by 25 percent"]
    #[inline(always)]
    pub fn sel_csel_dft(&mut self) -> SEL_CSEL_DFT_W {
        SEL_CSEL_DFT_W::new(self)
    }
    #[doc = "Bit 28 - Mux select signal for DAC control"]
    #[inline(always)]
    pub fn en_csel_dft(&mut self) -> EN_CSEL_DFT_W {
        EN_CSEL_DFT_W::new(self)
    }
    #[doc = "Bit 29 - Delay Control Enable for latch. - 0: doubles the latch enable time. - 1: normal latch enable time (default)."]
    #[inline(always)]
    pub fn dcen(&mut self) -> DCEN_W {
        DCEN_W::new(self)
    }
    #[doc = "Bit 31 - During deepsleep/ hibernate mode keep SARMUX active, i.e. do not open all switches (disconnect), to be used for ADFT"]
    #[inline(always)]
    pub fn adft_override(&mut self) -> ADFT_OVERRIDE_W {
        ADFT_OVERRIDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFT control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dft_ctrl](index.html) module"]
pub struct DFT_CTRL_SPEC;
impl crate::RegisterSpec for DFT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dft_ctrl::R](R) reader structure"]
impl crate::Readable for DFT_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dft_ctrl::W](W) writer structure"]
impl crate::Writable for DFT_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DFT_CTRL to value 0x02"]
impl crate::Resettable for DFT_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
