#[doc = "Register `CTB_SW_HW_CTRL` reader"]
pub struct R(crate::R<CTB_SW_HW_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTB_SW_HW_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTB_SW_HW_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTB_SW_HW_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTB_SW_HW_CTRL` writer"]
pub struct W(crate::W<CTB_SW_HW_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTB_SW_HW_CTRL_SPEC>;
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
impl From<crate::W<CTB_SW_HW_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTB_SW_HW_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P2_HW_CTRL` reader - for P22, D51 (dsi_out\\[2\\])"]
pub type P2_HW_CTRL_R = crate::BitReader<bool>;
#[doc = "Field `P2_HW_CTRL` writer - for P22, D51 (dsi_out\\[2\\])"]
pub type P2_HW_CTRL_W<'a> = crate::BitWriter<'a, u32, CTB_SW_HW_CTRL_SPEC, bool, 2>;
#[doc = "Field `P3_HW_CTRL` reader - for P33, D52, D62 (dsi_out\\[3\\])"]
pub type P3_HW_CTRL_R = crate::BitReader<bool>;
#[doc = "Field `P3_HW_CTRL` writer - for P33, D52, D62 (dsi_out\\[3\\])"]
pub type P3_HW_CTRL_W<'a> = crate::BitWriter<'a, u32, CTB_SW_HW_CTRL_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 2 - for P22, D51 (dsi_out\\[2\\])"]
    #[inline(always)]
    pub fn p2_hw_ctrl(&self) -> P2_HW_CTRL_R {
        P2_HW_CTRL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - for P33, D52, D62 (dsi_out\\[3\\])"]
    #[inline(always)]
    pub fn p3_hw_ctrl(&self) -> P3_HW_CTRL_R {
        P3_HW_CTRL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - for P22, D51 (dsi_out\\[2\\])"]
    #[inline(always)]
    pub fn p2_hw_ctrl(&mut self) -> P2_HW_CTRL_W {
        P2_HW_CTRL_W::new(self)
    }
    #[doc = "Bit 3 - for P33, D52, D62 (dsi_out\\[3\\])"]
    #[inline(always)]
    pub fn p3_hw_ctrl(&mut self) -> P3_HW_CTRL_W {
        P3_HW_CTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTB bus switch control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctb_sw_hw_ctrl](index.html) module"]
pub struct CTB_SW_HW_CTRL_SPEC;
impl crate::RegisterSpec for CTB_SW_HW_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctb_sw_hw_ctrl::R](R) reader structure"]
impl crate::Readable for CTB_SW_HW_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctb_sw_hw_ctrl::W](W) writer structure"]
impl crate::Writable for CTB_SW_HW_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTB_SW_HW_CTRL to value 0"]
impl crate::Resettable for CTB_SW_HW_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
