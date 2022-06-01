#[doc = "Register `Cy_I2C_CFG` reader"]
pub struct R(crate::R<CY_I2C_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CY_I2C_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CY_I2C_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CY_I2C_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Cy_I2C_CFG` writer"]
pub struct W(crate::W<CY_I2C_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CY_I2C_CFG_SPEC>;
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
impl From<crate::W<CY_I2C_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CY_I2C_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDA_IN_FILT_TRIM` reader - No description available"]
pub type SDA_IN_FILT_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDA_IN_FILT_TRIM` writer - No description available"]
pub type SDA_IN_FILT_TRIM_W<'a> = crate::FieldWriter<'a, u32, CY_I2C_CFG_SPEC, u8, u8, 2, 0>;
#[doc = "Field `SDA_IN_FILT_SEL` reader - No description available"]
pub type SDA_IN_FILT_SEL_R = crate::BitReader<bool>;
#[doc = "Field `SDA_IN_FILT_SEL` writer - No description available"]
pub type SDA_IN_FILT_SEL_W<'a> = crate::BitWriter<'a, u32, CY_I2C_CFG_SPEC, bool, 4>;
#[doc = "Field `SCL_IN_FILT_TRIM` reader - No description available"]
pub type SCL_IN_FILT_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCL_IN_FILT_TRIM` writer - No description available"]
pub type SCL_IN_FILT_TRIM_W<'a> = crate::FieldWriter<'a, u32, CY_I2C_CFG_SPEC, u8, u8, 2, 8>;
#[doc = "Field `SCL_IN_FILT_SEL` reader - No description available"]
pub type SCL_IN_FILT_SEL_R = crate::BitReader<bool>;
#[doc = "Field `SCL_IN_FILT_SEL` writer - No description available"]
pub type SCL_IN_FILT_SEL_W<'a> = crate::BitWriter<'a, u32, CY_I2C_CFG_SPEC, bool, 12>;
#[doc = "Field `SDA_OUT_FILT0_TRIM` reader - No description available"]
pub type SDA_OUT_FILT0_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDA_OUT_FILT0_TRIM` writer - No description available"]
pub type SDA_OUT_FILT0_TRIM_W<'a> = crate::FieldWriter<'a, u32, CY_I2C_CFG_SPEC, u8, u8, 2, 16>;
#[doc = "Field `SDA_OUT_FILT1_TRIM` reader - No description available"]
pub type SDA_OUT_FILT1_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDA_OUT_FILT1_TRIM` writer - No description available"]
pub type SDA_OUT_FILT1_TRIM_W<'a> = crate::FieldWriter<'a, u32, CY_I2C_CFG_SPEC, u8, u8, 2, 18>;
#[doc = "Field `SDA_OUT_FILT2_TRIM` reader - No description available"]
pub type SDA_OUT_FILT2_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDA_OUT_FILT2_TRIM` writer - No description available"]
pub type SDA_OUT_FILT2_TRIM_W<'a> = crate::FieldWriter<'a, u32, CY_I2C_CFG_SPEC, u8, u8, 2, 20>;
#[doc = "Field `SDA_OUT_FILT_SEL` reader - No description available"]
pub type SDA_OUT_FILT_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDA_OUT_FILT_SEL` writer - No description available"]
pub type SDA_OUT_FILT_SEL_W<'a> = crate::FieldWriter<'a, u32, CY_I2C_CFG_SPEC, u8, u8, 2, 28>;
impl R {
    #[doc = "Bits 0:1 - No description available"]
    #[inline(always)]
    pub fn sda_in_filt_trim(&self) -> SDA_IN_FILT_TRIM_R {
        SDA_IN_FILT_TRIM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - No description available"]
    #[inline(always)]
    pub fn sda_in_filt_sel(&self) -> SDA_IN_FILT_SEL_R {
        SDA_IN_FILT_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - No description available"]
    #[inline(always)]
    pub fn scl_in_filt_trim(&self) -> SCL_IN_FILT_TRIM_R {
        SCL_IN_FILT_TRIM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - No description available"]
    #[inline(always)]
    pub fn scl_in_filt_sel(&self) -> SCL_IN_FILT_SEL_R {
        SCL_IN_FILT_SEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:17 - No description available"]
    #[inline(always)]
    pub fn sda_out_filt0_trim(&self) -> SDA_OUT_FILT0_TRIM_R {
        SDA_OUT_FILT0_TRIM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - No description available"]
    #[inline(always)]
    pub fn sda_out_filt1_trim(&self) -> SDA_OUT_FILT1_TRIM_R {
        SDA_OUT_FILT1_TRIM_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - No description available"]
    #[inline(always)]
    pub fn sda_out_filt2_trim(&self) -> SDA_OUT_FILT2_TRIM_R {
        SDA_OUT_FILT2_TRIM_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 28:29 - No description available"]
    #[inline(always)]
    pub fn sda_out_filt_sel(&self) -> SDA_OUT_FILT_SEL_R {
        SDA_OUT_FILT_SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - No description available"]
    #[inline(always)]
    pub fn sda_in_filt_trim(&mut self) -> SDA_IN_FILT_TRIM_W {
        SDA_IN_FILT_TRIM_W::new(self)
    }
    #[doc = "Bit 4 - No description available"]
    #[inline(always)]
    pub fn sda_in_filt_sel(&mut self) -> SDA_IN_FILT_SEL_W {
        SDA_IN_FILT_SEL_W::new(self)
    }
    #[doc = "Bits 8:9 - No description available"]
    #[inline(always)]
    pub fn scl_in_filt_trim(&mut self) -> SCL_IN_FILT_TRIM_W {
        SCL_IN_FILT_TRIM_W::new(self)
    }
    #[doc = "Bit 12 - No description available"]
    #[inline(always)]
    pub fn scl_in_filt_sel(&mut self) -> SCL_IN_FILT_SEL_W {
        SCL_IN_FILT_SEL_W::new(self)
    }
    #[doc = "Bits 16:17 - No description available"]
    #[inline(always)]
    pub fn sda_out_filt0_trim(&mut self) -> SDA_OUT_FILT0_TRIM_W {
        SDA_OUT_FILT0_TRIM_W::new(self)
    }
    #[doc = "Bits 18:19 - No description available"]
    #[inline(always)]
    pub fn sda_out_filt1_trim(&mut self) -> SDA_OUT_FILT1_TRIM_W {
        SDA_OUT_FILT1_TRIM_W::new(self)
    }
    #[doc = "Bits 20:21 - No description available"]
    #[inline(always)]
    pub fn sda_out_filt2_trim(&mut self) -> SDA_OUT_FILT2_TRIM_W {
        SDA_OUT_FILT2_TRIM_W::new(self)
    }
    #[doc = "Bits 28:29 - No description available"]
    #[inline(always)]
    pub fn sda_out_filt_sel(&mut self) -> SDA_OUT_FILT_SEL_W {
        SDA_OUT_FILT_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C fitler trimm register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cy_i2c_cfg](index.html) module"]
pub struct CY_I2C_CFG_SPEC;
impl crate::RegisterSpec for CY_I2C_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cy_i2c_cfg::R](R) reader structure"]
impl crate::Readable for CY_I2C_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cy_i2c_cfg::W](W) writer structure"]
impl crate::Writable for CY_I2C_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Cy_I2C_CFG to value 0"]
impl crate::Resettable for CY_I2C_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
