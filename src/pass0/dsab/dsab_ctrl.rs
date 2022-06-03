#[doc = "Register `DSAB_CTRL` reader"]
pub struct R(crate::R<DSAB_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSAB_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSAB_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSAB_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSAB_CTRL` writer"]
pub struct W(crate::W<DSAB_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSAB_CTRL_SPEC>;
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
impl From<crate::W<DSAB_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSAB_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CURRENT_SEL` reader - DSAB DAC control field Nominal DSAB Output Current = CURRENT_SEL * 0.075 uA In products with SRSS-LITE, this setting impacts the CTB(m) offset. A value of 0x20 is used during factory trim and is required to maintain low offsets across temperature variation. If a different setting is used then a periodic re-trim of CTB(m) offset should be performed."]
pub type CURRENT_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CURRENT_SEL` writer - DSAB DAC control field Nominal DSAB Output Current = CURRENT_SEL * 0.075 uA In products with SRSS-LITE, this setting impacts the CTB(m) offset. A value of 0x20 is used during factory trim and is required to maintain low offsets across temperature variation. If a different setting is used then a periodic re-trim of CTB(m) offset should be performed."]
pub type CURRENT_SEL_W<'a> = crate::FieldWriter<'a, u32, DSAB_CTRL_SPEC, u8, u8, 6, 0>;
#[doc = "Field `SEL_OUT` reader - N/A"]
pub type SEL_OUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL_OUT` writer - N/A"]
pub type SEL_OUT_W<'a> = crate::FieldWriter<'a, u32, DSAB_CTRL_SPEC, u8, u8, 4, 8>;
#[doc = "Field `REF_SWAP_EN` reader - This field (along with SEL_OUT and ENABLED) provides bitwise selection of the current sources that drive the DSAB ZTC and PTAT outputs. See SEL_OUT field for truth tables."]
pub type REF_SWAP_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REF_SWAP_EN` writer - This field (along with SEL_OUT and ENABLED) provides bitwise selection of the current sources that drive the DSAB ZTC and PTAT outputs. See SEL_OUT field for truth tables."]
pub type REF_SWAP_EN_W<'a> = crate::FieldWriter<'a, u32, DSAB_CTRL_SPEC, u8, u8, 4, 16>;
#[doc = "Field `BYPASS_MODE_EN` reader - 0 - DSAB PTAT generator is powered from DSAB regulator: VDDA must be at least 2.4V 1 - DSAB PTAT generator is pwoered directly from VDDA: VDDA cannot exceed 4.0V"]
pub type BYPASS_MODE_EN_R = crate::BitReader<bool>;
#[doc = "Field `BYPASS_MODE_EN` writer - 0 - DSAB PTAT generator is powered from DSAB regulator: VDDA must be at least 2.4V 1 - DSAB PTAT generator is pwoered directly from VDDA: VDDA cannot exceed 4.0V"]
pub type BYPASS_MODE_EN_W<'a> = crate::BitWriter<'a, u32, DSAB_CTRL_SPEC, bool, 24>;
#[doc = "Field `STARTUP_RM` reader - Risk mitigation control 1 - Force start the startup circuit"]
pub type STARTUP_RM_R = crate::BitReader<bool>;
#[doc = "Field `STARTUP_RM` writer - Risk mitigation control 1 - Force start the startup circuit"]
pub type STARTUP_RM_W<'a> = crate::BitWriter<'a, u32, DSAB_CTRL_SPEC, bool, 28>;
#[doc = "Field `ENABLED` reader - This field (along with SEL_OUT and REF_SWAP_EN) provides bitwise selection of the current sources that drive the DSAB ZTC and PTAT outputs. See SEL_OUT field for truth tables. In SRSSLT devices, in active mode, this bit is overridden to '1', that is - it is always enabled in active mode."]
pub type ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ENABLED` writer - This field (along with SEL_OUT and REF_SWAP_EN) provides bitwise selection of the current sources that drive the DSAB ZTC and PTAT outputs. See SEL_OUT field for truth tables. In SRSSLT devices, in active mode, this bit is overridden to '1', that is - it is always enabled in active mode."]
pub type ENABLED_W<'a> = crate::BitWriter<'a, u32, DSAB_CTRL_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:5 - DSAB DAC control field Nominal DSAB Output Current = CURRENT_SEL * 0.075 uA In products with SRSS-LITE, this setting impacts the CTB(m) offset. A value of 0x20 is used during factory trim and is required to maintain low offsets across temperature variation. If a different setting is used then a periodic re-trim of CTB(m) offset should be performed."]
    #[inline(always)]
    pub fn current_sel(&self) -> CURRENT_SEL_R {
        CURRENT_SEL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:11 - N/A"]
    #[inline(always)]
    pub fn sel_out(&self) -> SEL_OUT_R {
        SEL_OUT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - This field (along with SEL_OUT and ENABLED) provides bitwise selection of the current sources that drive the DSAB ZTC and PTAT outputs. See SEL_OUT field for truth tables."]
    #[inline(always)]
    pub fn ref_swap_en(&self) -> REF_SWAP_EN_R {
        REF_SWAP_EN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - 0 - DSAB PTAT generator is powered from DSAB regulator: VDDA must be at least 2.4V 1 - DSAB PTAT generator is pwoered directly from VDDA: VDDA cannot exceed 4.0V"]
    #[inline(always)]
    pub fn bypass_mode_en(&self) -> BYPASS_MODE_EN_R {
        BYPASS_MODE_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Risk mitigation control 1 - Force start the startup circuit"]
    #[inline(always)]
    pub fn startup_rm(&self) -> STARTUP_RM_R {
        STARTUP_RM_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - This field (along with SEL_OUT and REF_SWAP_EN) provides bitwise selection of the current sources that drive the DSAB ZTC and PTAT outputs. See SEL_OUT field for truth tables. In SRSSLT devices, in active mode, this bit is overridden to '1', that is - it is always enabled in active mode."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - DSAB DAC control field Nominal DSAB Output Current = CURRENT_SEL * 0.075 uA In products with SRSS-LITE, this setting impacts the CTB(m) offset. A value of 0x20 is used during factory trim and is required to maintain low offsets across temperature variation. If a different setting is used then a periodic re-trim of CTB(m) offset should be performed."]
    #[inline(always)]
    pub fn current_sel(&mut self) -> CURRENT_SEL_W {
        CURRENT_SEL_W::new(self)
    }
    #[doc = "Bits 8:11 - N/A"]
    #[inline(always)]
    pub fn sel_out(&mut self) -> SEL_OUT_W {
        SEL_OUT_W::new(self)
    }
    #[doc = "Bits 16:19 - This field (along with SEL_OUT and ENABLED) provides bitwise selection of the current sources that drive the DSAB ZTC and PTAT outputs. See SEL_OUT field for truth tables."]
    #[inline(always)]
    pub fn ref_swap_en(&mut self) -> REF_SWAP_EN_W {
        REF_SWAP_EN_W::new(self)
    }
    #[doc = "Bit 24 - 0 - DSAB PTAT generator is powered from DSAB regulator: VDDA must be at least 2.4V 1 - DSAB PTAT generator is pwoered directly from VDDA: VDDA cannot exceed 4.0V"]
    #[inline(always)]
    pub fn bypass_mode_en(&mut self) -> BYPASS_MODE_EN_W {
        BYPASS_MODE_EN_W::new(self)
    }
    #[doc = "Bit 28 - Risk mitigation control 1 - Force start the startup circuit"]
    #[inline(always)]
    pub fn startup_rm(&mut self) -> STARTUP_RM_W {
        STARTUP_RM_W::new(self)
    }
    #[doc = "Bit 31 - This field (along with SEL_OUT and REF_SWAP_EN) provides bitwise selection of the current sources that drive the DSAB ZTC and PTAT outputs. See SEL_OUT field for truth tables. In SRSSLT devices, in active mode, this bit is overridden to '1', that is - it is always enabled in active mode."]
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
#[doc = "global DSAB control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsab_ctrl](index.html) module"]
pub struct DSAB_CTRL_SPEC;
impl crate::RegisterSpec for DSAB_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsab_ctrl::R](R) reader structure"]
impl crate::Readable for DSAB_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsab_ctrl::W](W) writer structure"]
impl crate::Writable for DSAB_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DSAB_CTRL to value 0"]
impl crate::Resettable for DSAB_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
