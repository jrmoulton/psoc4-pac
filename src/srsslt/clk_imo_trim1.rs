#[doc = "Register `CLK_IMO_TRIM1` reader"]
pub struct R(crate::R<CLK_IMO_TRIM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_IMO_TRIM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_IMO_TRIM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_IMO_TRIM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_IMO_TRIM1` writer"]
pub struct W(crate::W<CLK_IMO_TRIM1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_IMO_TRIM1_SPEC>;
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
impl From<crate::W<CLK_IMO_TRIM1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_IMO_TRIM1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFSET` reader - Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting in CLK_IMO_SELECT and stored in SFLASH. This field is hardware updated during USB osclock mode or when a WCO uses this mechanism for PLL locking the WCO. This is only available with devices that have either USB or a WCO. This field is mapped to the most significant bits of the IMO trim imo_clk_trim\\[10:3\\]. The step size of 1 LSB on this field is approximately 120 kHz."]
pub type OFFSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OFFSET` writer - Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting in CLK_IMO_SELECT and stored in SFLASH. This field is hardware updated during USB osclock mode or when a WCO uses this mechanism for PLL locking the WCO. This is only available with devices that have either USB or a WCO. This field is mapped to the most significant bits of the IMO trim imo_clk_trim\\[10:3\\]. The step size of 1 LSB on this field is approximately 120 kHz."]
pub type OFFSET_W<'a> = crate::FieldWriter<'a, u32, CLK_IMO_TRIM1_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting in CLK_IMO_SELECT and stored in SFLASH. This field is hardware updated during USB osclock mode or when a WCO uses this mechanism for PLL locking the WCO. This is only available with devices that have either USB or a WCO. This field is mapped to the most significant bits of the IMO trim imo_clk_trim\\[10:3\\]. The step size of 1 LSB on this field is approximately 120 kHz."]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frequency trim bits. These bits are determined at manufacturing time for each FREQ setting in CLK_IMO_SELECT and stored in SFLASH. This field is hardware updated during USB osclock mode or when a WCO uses this mechanism for PLL locking the WCO. This is only available with devices that have either USB or a WCO. This field is mapped to the most significant bits of the IMO trim imo_clk_trim\\[10:3\\]. The step size of 1 LSB on this field is approximately 120 kHz."]
    #[inline(always)]
    pub fn offset(&mut self) -> OFFSET_W {
        OFFSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IMO Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_imo_trim1](index.html) module"]
pub struct CLK_IMO_TRIM1_SPEC;
impl crate::RegisterSpec for CLK_IMO_TRIM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_imo_trim1::R](R) reader structure"]
impl crate::Readable for CLK_IMO_TRIM1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_imo_trim1::W](W) writer structure"]
impl crate::Writable for CLK_IMO_TRIM1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_IMO_TRIM1 to value 0x80"]
impl crate::Resettable for CLK_IMO_TRIM1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
