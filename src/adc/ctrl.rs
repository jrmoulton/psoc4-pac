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
#[doc = "Field `VREF_SEL` reader - SARADC internal VREF selection"]
pub type VREF_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VREF_SEL` writer - SARADC internal VREF selection"]
pub type VREF_SEL_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 3, 4>;
#[doc = "Field `VREF_BYP_CAP_EN` reader - VREF bypass cap enable for when VREF buffer is on"]
pub type VREF_BYP_CAP_EN_R = crate::BitReader<bool>;
#[doc = "Field `VREF_BYP_CAP_EN` writer - VREF bypass cap enable for when VREF buffer is on"]
pub type VREF_BYP_CAP_EN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 7>;
#[doc = "Field `NEG_SEL` reader - SARADC internal NEG selection for Single ended conversion"]
pub type NEG_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NEG_SEL` writer - SARADC internal NEG selection for Single ended conversion"]
pub type NEG_SEL_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 3, 9>;
#[doc = "Field `SAR_HW_CTRL_NEGVREF` reader - Hardware control: 0=firmware, 1=hardware."]
pub type SAR_HW_CTRL_NEGVREF_R = crate::BitReader<bool>;
#[doc = "Field `SAR_HW_CTRL_NEGVREF` writer - Hardware control: 0=firmware, 1=hardware."]
pub type SAR_HW_CTRL_NEGVREF_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 13>;
#[doc = "Field `PWR_CTRL_VREF` reader - VREF buffer low power mode."]
pub type PWR_CTRL_VREF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWR_CTRL_VREF` writer - VREF buffer low power mode."]
pub type PWR_CTRL_VREF_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, 14>;
#[doc = "Field `SPARE` reader - Spare controls"]
pub type SPARE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPARE` writer - Spare controls"]
pub type SPARE_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, 16>;
#[doc = "Field `ICONT_LV` reader - SARADC low power mode"]
pub type ICONT_LV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ICONT_LV` writer - SARADC low power mode"]
pub type ICONT_LV_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, 24>;
#[doc = "Field `DSI_SYNC_CONFIG` reader - Synchronize the DSI config signals to peripheral clock domain"]
pub type DSI_SYNC_CONFIG_R = crate::BitReader<bool>;
#[doc = "Field `DSI_SYNC_CONFIG` writer - Synchronize the DSI config signals to peripheral clock domain"]
pub type DSI_SYNC_CONFIG_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 28>;
#[doc = "Field `DSI_MODE` reader - SAR sequencer takes configuration from DSI signals"]
pub type DSI_MODE_R = crate::BitReader<bool>;
#[doc = "Field `DSI_MODE` writer - SAR sequencer takes configuration from DSI signals"]
pub type DSI_MODE_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 29>;
#[doc = "Field `SWITCH_DISABLE` reader - Disable SAR sequencer from enabling routing switches"]
pub type SWITCH_DISABLE_R = crate::BitReader<bool>;
#[doc = "Field `SWITCH_DISABLE` writer - Disable SAR sequencer from enabling routing switches"]
pub type SWITCH_DISABLE_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 30>;
#[doc = "Field `ENABLED` reader - 0: SAR IP disabled, 1: SAR IP enabled."]
pub type ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ENABLED` writer - 0: SAR IP disabled, 1: SAR IP enabled."]
pub type ENABLED_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 4:6 - SARADC internal VREF selection"]
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
    #[doc = "Bit 13 - Hardware control: 0=firmware, 1=hardware."]
    #[inline(always)]
    pub fn sar_hw_ctrl_negvref(&self) -> SAR_HW_CTRL_NEGVREF_R {
        SAR_HW_CTRL_NEGVREF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - VREF buffer low power mode."]
    #[inline(always)]
    pub fn pwr_ctrl_vref(&self) -> PWR_CTRL_VREF_R {
        PWR_CTRL_VREF_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Spare controls"]
    #[inline(always)]
    pub fn spare(&self) -> SPARE_R {
        SPARE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25 - SARADC low power mode"]
    #[inline(always)]
    pub fn icont_lv(&self) -> ICONT_LV_R {
        ICONT_LV_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28 - Synchronize the DSI config signals to peripheral clock domain"]
    #[inline(always)]
    pub fn dsi_sync_config(&self) -> DSI_SYNC_CONFIG_R {
        DSI_SYNC_CONFIG_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SAR sequencer takes configuration from DSI signals"]
    #[inline(always)]
    pub fn dsi_mode(&self) -> DSI_MODE_R {
        DSI_MODE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Disable SAR sequencer from enabling routing switches"]
    #[inline(always)]
    pub fn switch_disable(&self) -> SWITCH_DISABLE_R {
        SWITCH_DISABLE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 0: SAR IP disabled, 1: SAR IP enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:6 - SARADC internal VREF selection"]
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
    #[doc = "Bit 13 - Hardware control: 0=firmware, 1=hardware."]
    #[inline(always)]
    pub fn sar_hw_ctrl_negvref(&mut self) -> SAR_HW_CTRL_NEGVREF_W {
        SAR_HW_CTRL_NEGVREF_W::new(self)
    }
    #[doc = "Bits 14:15 - VREF buffer low power mode."]
    #[inline(always)]
    pub fn pwr_ctrl_vref(&mut self) -> PWR_CTRL_VREF_W {
        PWR_CTRL_VREF_W::new(self)
    }
    #[doc = "Bits 16:19 - Spare controls"]
    #[inline(always)]
    pub fn spare(&mut self) -> SPARE_W {
        SPARE_W::new(self)
    }
    #[doc = "Bits 24:25 - SARADC low power mode"]
    #[inline(always)]
    pub fn icont_lv(&mut self) -> ICONT_LV_W {
        ICONT_LV_W::new(self)
    }
    #[doc = "Bit 28 - Synchronize the DSI config signals to peripheral clock domain"]
    #[inline(always)]
    pub fn dsi_sync_config(&mut self) -> DSI_SYNC_CONFIG_W {
        DSI_SYNC_CONFIG_W::new(self)
    }
    #[doc = "Bit 29 - SAR sequencer takes configuration from DSI signals"]
    #[inline(always)]
    pub fn dsi_mode(&mut self) -> DSI_MODE_W {
        DSI_MODE_W::new(self)
    }
    #[doc = "Bit 30 - Disable SAR sequencer from enabling routing switches"]
    #[inline(always)]
    pub fn switch_disable(&mut self) -> SWITCH_DISABLE_W {
        SWITCH_DISABLE_W::new(self)
    }
    #[doc = "Bit 31 - 0: SAR IP disabled, 1: SAR IP enabled."]
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
#[doc = "Analog control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
