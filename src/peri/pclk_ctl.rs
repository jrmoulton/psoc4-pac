#[doc = "Register `PCLK_CTL[%s]` reader"]
pub struct R(crate::R<PCLK_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCLK_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCLK_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCLK_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCLK_CTL[%s]` writer"]
pub struct W(crate::W<PCLK_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCLK_CTL_SPEC>;
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
impl From<crate::W<PCLK_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCLK_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL_DIV` reader - Specifies one of the dividers of the divider type specified by SEL_TYPE. If SEL_DIV is '63' and 'SEL_TYPE' is '3' (default/reset value), no divider is specified and no clock control signal(s) are generated. When transitioning a clock between two out of phase dividers, spurious clock control signals may be generated for one 'clk_hf' cycle during this transition. These clock control signals may cause a single clock period that is smaller than any of the two divider periods. To prevent these spurious clock signals, the clock multiplexer can be disconnected (SEL_DIV is '63' and 'SEL_TYPE' is '3') for a transition time that is larger than the smaller of the two divider periods."]
pub type SEL_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL_DIV` writer - Specifies one of the dividers of the divider type specified by SEL_TYPE. If SEL_DIV is '63' and 'SEL_TYPE' is '3' (default/reset value), no divider is specified and no clock control signal(s) are generated. When transitioning a clock between two out of phase dividers, spurious clock control signals may be generated for one 'clk_hf' cycle during this transition. These clock control signals may cause a single clock period that is smaller than any of the two divider periods. To prevent these spurious clock signals, the clock multiplexer can be disconnected (SEL_DIV is '63' and 'SEL_TYPE' is '3') for a transition time that is larger than the smaller of the two divider periods."]
pub type SEL_DIV_W<'a> = crate::FieldWriter<'a, u32, PCLK_CTL_SPEC, u8, u8, 6, 0>;
#[doc = "Field `SEL_TYPE` reader - Specifies divider type: 0: 8.0 (integer) clock dividers. 1: 16.0 (integer) clock dividers. 2: 16.5 (fractional) clock dividers. 3: 24.5 (fractional) clock dividers."]
pub type SEL_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL_TYPE` writer - Specifies divider type: 0: 8.0 (integer) clock dividers. 1: 16.0 (integer) clock dividers. 2: 16.5 (fractional) clock dividers. 3: 24.5 (fractional) clock dividers."]
pub type SEL_TYPE_W<'a> = crate::FieldWriter<'a, u32, PCLK_CTL_SPEC, u8, u8, 2, 6>;
impl R {
    #[doc = "Bits 0:5 - Specifies one of the dividers of the divider type specified by SEL_TYPE. If SEL_DIV is '63' and 'SEL_TYPE' is '3' (default/reset value), no divider is specified and no clock control signal(s) are generated. When transitioning a clock between two out of phase dividers, spurious clock control signals may be generated for one 'clk_hf' cycle during this transition. These clock control signals may cause a single clock period that is smaller than any of the two divider periods. To prevent these spurious clock signals, the clock multiplexer can be disconnected (SEL_DIV is '63' and 'SEL_TYPE' is '3') for a transition time that is larger than the smaller of the two divider periods."]
    #[inline(always)]
    pub fn sel_div(&self) -> SEL_DIV_R {
        SEL_DIV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Specifies divider type: 0: 8.0 (integer) clock dividers. 1: 16.0 (integer) clock dividers. 2: 16.5 (fractional) clock dividers. 3: 24.5 (fractional) clock dividers."]
    #[inline(always)]
    pub fn sel_type(&self) -> SEL_TYPE_R {
        SEL_TYPE_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Specifies one of the dividers of the divider type specified by SEL_TYPE. If SEL_DIV is '63' and 'SEL_TYPE' is '3' (default/reset value), no divider is specified and no clock control signal(s) are generated. When transitioning a clock between two out of phase dividers, spurious clock control signals may be generated for one 'clk_hf' cycle during this transition. These clock control signals may cause a single clock period that is smaller than any of the two divider periods. To prevent these spurious clock signals, the clock multiplexer can be disconnected (SEL_DIV is '63' and 'SEL_TYPE' is '3') for a transition time that is larger than the smaller of the two divider periods."]
    #[inline(always)]
    pub fn sel_div(&mut self) -> SEL_DIV_W {
        SEL_DIV_W::new(self)
    }
    #[doc = "Bits 6:7 - Specifies divider type: 0: 8.0 (integer) clock dividers. 1: 16.0 (integer) clock dividers. 2: 16.5 (fractional) clock dividers. 3: 24.5 (fractional) clock dividers."]
    #[inline(always)]
    pub fn sel_type(&mut self) -> SEL_TYPE_W {
        SEL_TYPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Programmable clock control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pclk_ctl](index.html) module"]
pub struct PCLK_CTL_SPEC;
impl crate::RegisterSpec for PCLK_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pclk_ctl::R](R) reader structure"]
impl crate::Readable for PCLK_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pclk_ctl::W](W) writer structure"]
impl crate::Writable for PCLK_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCLK_CTL[%s]
to value 0xff"]
impl crate::Resettable for PCLK_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
