#[doc = "Register `PASS_CTRL` reader"]
pub struct R(crate::R<PASS_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PASS_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PASS_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PASS_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PASS_CTRL` writer"]
pub struct W(crate::W<PASS_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PASS_CTRL_SPEC>;
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
impl From<crate::W<PASS_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PASS_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMPCLK_BYP` reader - - 0: Pump clk is clk_hf/2 - 1: Pump clk is selected from PMPCLK_SRC"]
pub type PMPCLK_BYP_R = crate::BitReader<bool>;
#[doc = "Field `PMPCLK_BYP` writer - - 0: Pump clk is clk_hf/2 - 1: Pump clk is selected from PMPCLK_SRC"]
pub type PMPCLK_BYP_W<'a> = crate::BitWriter<'a, u32, PASS_CTRL_SPEC, bool, 0>;
#[doc = "Field `PMPCLK_SRC` reader - - 0: Pump clk is clk_hf - 1: Pump clk is direct from SRSS"]
pub type PMPCLK_SRC_R = crate::BitReader<bool>;
#[doc = "Field `PMPCLK_SRC` writer - - 0: Pump clk is clk_hf - 1: Pump clk is direct from SRSS"]
pub type PMPCLK_SRC_W<'a> = crate::BitWriter<'a, u32, PASS_CTRL_SPEC, bool, 1>;
#[doc = "Field `RMB_BITS` reader - Risk mitigation bits"]
pub type RMB_BITS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RMB_BITS` writer - Risk mitigation bits"]
pub type RMB_BITS_W<'a> = crate::FieldWriter<'a, u32, PASS_CTRL_SPEC, u8, u8, 8, 8>;
impl R {
    #[doc = "Bit 0 - - 0: Pump clk is clk_hf/2 - 1: Pump clk is selected from PMPCLK_SRC"]
    #[inline(always)]
    pub fn pmpclk_byp(&self) -> PMPCLK_BYP_R {
        PMPCLK_BYP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - - 0: Pump clk is clk_hf - 1: Pump clk is direct from SRSS"]
    #[inline(always)]
    pub fn pmpclk_src(&self) -> PMPCLK_SRC_R {
        PMPCLK_SRC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Risk mitigation bits"]
    #[inline(always)]
    pub fn rmb_bits(&self) -> RMB_BITS_R {
        RMB_BITS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - - 0: Pump clk is clk_hf/2 - 1: Pump clk is selected from PMPCLK_SRC"]
    #[inline(always)]
    pub fn pmpclk_byp(&mut self) -> PMPCLK_BYP_W {
        PMPCLK_BYP_W::new(self)
    }
    #[doc = "Bit 1 - - 0: Pump clk is clk_hf - 1: Pump clk is direct from SRSS"]
    #[inline(always)]
    pub fn pmpclk_src(&mut self) -> PMPCLK_SRC_W {
        PMPCLK_SRC_W::new(self)
    }
    #[doc = "Bits 8:15 - Risk mitigation bits"]
    #[inline(always)]
    pub fn rmb_bits(&mut self) -> RMB_BITS_W {
        RMB_BITS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PASS Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pass_ctrl](index.html) module"]
pub struct PASS_CTRL_SPEC;
impl crate::RegisterSpec for PASS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pass_ctrl::R](R) reader structure"]
impl crate::Readable for PASS_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pass_ctrl::W](W) writer structure"]
impl crate::Writable for PASS_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PASS_CTRL to value 0"]
impl crate::Resettable for PASS_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
