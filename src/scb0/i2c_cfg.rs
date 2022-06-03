#[doc = "Register `I2C_CFG` reader"]
pub struct R(crate::R<I2C_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_CFG` writer"]
pub struct W(crate::W<I2C_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_CFG_SPEC>;
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
impl From<crate::W<I2C_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDA_IN_FILT_TRIM` reader - Trim bits for 'i2c_sda_in' 50 ns filter. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
pub type SDA_IN_FILT_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDA_IN_FILT_TRIM` writer - Trim bits for 'i2c_sda_in' 50 ns filter. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
pub type SDA_IN_FILT_TRIM_W<'a> = crate::FieldWriter<'a, u32, I2C_CFG_SPEC, u8, u8, 2, 0>;
#[doc = "Field `SDA_IN_FILT_SEL` reader - Selection of 'i2c_sda_in' filter delay: '0': 0 ns. '1: 50 ns (filter enabled)."]
pub type SDA_IN_FILT_SEL_R = crate::BitReader<bool>;
#[doc = "Field `SDA_IN_FILT_SEL` writer - Selection of 'i2c_sda_in' filter delay: '0': 0 ns. '1: 50 ns (filter enabled)."]
pub type SDA_IN_FILT_SEL_W<'a> = crate::BitWriter<'a, u32, I2C_CFG_SPEC, bool, 4>;
#[doc = "Field `SCL_IN_FILT_TRIM` reader - Trim bits for 'i2c_scl_in' 50 ns filter. See s8i2cs BROS (001-59539) for more details on the trim bit values. With s8iom0s8v1p2 I/Os, trim bits should be programmed to 3 to suppress glitches below 50ns."]
pub type SCL_IN_FILT_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCL_IN_FILT_TRIM` writer - Trim bits for 'i2c_scl_in' 50 ns filter. See s8i2cs BROS (001-59539) for more details on the trim bit values. With s8iom0s8v1p2 I/Os, trim bits should be programmed to 3 to suppress glitches below 50ns."]
pub type SCL_IN_FILT_TRIM_W<'a> = crate::FieldWriter<'a, u32, I2C_CFG_SPEC, u8, u8, 2, 8>;
#[doc = "Field `SCL_IN_FILT_SEL` reader - Selection of 'i2c_scl_in' filter delay: '0': 0 ns. '1: 50 ns (filter enabled)."]
pub type SCL_IN_FILT_SEL_R = crate::BitReader<bool>;
#[doc = "Field `SCL_IN_FILT_SEL` writer - Selection of 'i2c_scl_in' filter delay: '0': 0 ns. '1: 50 ns (filter enabled)."]
pub type SCL_IN_FILT_SEL_W<'a> = crate::BitWriter<'a, u32, I2C_CFG_SPEC, bool, 12>;
#[doc = "Field `SDA_OUT_FILT0_TRIM` reader - Trim bits for 'i2c_sda_out' 50 ns filter 0. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
pub type SDA_OUT_FILT0_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDA_OUT_FILT0_TRIM` writer - Trim bits for 'i2c_sda_out' 50 ns filter 0. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
pub type SDA_OUT_FILT0_TRIM_W<'a> = crate::FieldWriter<'a, u32, I2C_CFG_SPEC, u8, u8, 2, 16>;
#[doc = "Field `SDA_OUT_FILT1_TRIM` reader - Trim bits for 'i2c_sda_out' 50 ns filter 1. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
pub type SDA_OUT_FILT1_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDA_OUT_FILT1_TRIM` writer - Trim bits for 'i2c_sda_out' 50 ns filter 1. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
pub type SDA_OUT_FILT1_TRIM_W<'a> = crate::FieldWriter<'a, u32, I2C_CFG_SPEC, u8, u8, 2, 18>;
#[doc = "Field `SDA_OUT_FILT2_TRIM` reader - Trim bits for 'i2c_sda_out' 50 ns filter 2. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
pub type SDA_OUT_FILT2_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDA_OUT_FILT2_TRIM` writer - Trim bits for 'i2c_sda_out' 50 ns filter 2. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
pub type SDA_OUT_FILT2_TRIM_W<'a> = crate::FieldWriter<'a, u32, I2C_CFG_SPEC, u8, u8, 2, 20>;
#[doc = "Field `SDA_OUT_FILT_SEL` reader - Selection of cumulative 'i2c_sda_out' filter delay: '0': 0 ns. '1': 50 ns (filter 0 enabled). '2': 100 ns (filters 0 and 1 enabled). '3': 150 ns (filters 0, 1 and 2 enabled)."]
pub type SDA_OUT_FILT_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDA_OUT_FILT_SEL` writer - Selection of cumulative 'i2c_sda_out' filter delay: '0': 0 ns. '1': 50 ns (filter 0 enabled). '2': 100 ns (filters 0 and 1 enabled). '3': 150 ns (filters 0, 1 and 2 enabled)."]
pub type SDA_OUT_FILT_SEL_W<'a> = crate::FieldWriter<'a, u32, I2C_CFG_SPEC, u8, u8, 2, 28>;
impl R {
    #[doc = "Bits 0:1 - Trim bits for 'i2c_sda_in' 50 ns filter. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
    #[inline(always)]
    pub fn sda_in_filt_trim(&self) -> SDA_IN_FILT_TRIM_R {
        SDA_IN_FILT_TRIM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Selection of 'i2c_sda_in' filter delay: '0': 0 ns. '1: 50 ns (filter enabled)."]
    #[inline(always)]
    pub fn sda_in_filt_sel(&self) -> SDA_IN_FILT_SEL_R {
        SDA_IN_FILT_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Trim bits for 'i2c_scl_in' 50 ns filter. See s8i2cs BROS (001-59539) for more details on the trim bit values. With s8iom0s8v1p2 I/Os, trim bits should be programmed to 3 to suppress glitches below 50ns."]
    #[inline(always)]
    pub fn scl_in_filt_trim(&self) -> SCL_IN_FILT_TRIM_R {
        SCL_IN_FILT_TRIM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - Selection of 'i2c_scl_in' filter delay: '0': 0 ns. '1: 50 ns (filter enabled)."]
    #[inline(always)]
    pub fn scl_in_filt_sel(&self) -> SCL_IN_FILT_SEL_R {
        SCL_IN_FILT_SEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Trim bits for 'i2c_sda_out' 50 ns filter 0. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
    #[inline(always)]
    pub fn sda_out_filt0_trim(&self) -> SDA_OUT_FILT0_TRIM_R {
        SDA_OUT_FILT0_TRIM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Trim bits for 'i2c_sda_out' 50 ns filter 1. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
    #[inline(always)]
    pub fn sda_out_filt1_trim(&self) -> SDA_OUT_FILT1_TRIM_R {
        SDA_OUT_FILT1_TRIM_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Trim bits for 'i2c_sda_out' 50 ns filter 2. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
    #[inline(always)]
    pub fn sda_out_filt2_trim(&self) -> SDA_OUT_FILT2_TRIM_R {
        SDA_OUT_FILT2_TRIM_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Selection of cumulative 'i2c_sda_out' filter delay: '0': 0 ns. '1': 50 ns (filter 0 enabled). '2': 100 ns (filters 0 and 1 enabled). '3': 150 ns (filters 0, 1 and 2 enabled)."]
    #[inline(always)]
    pub fn sda_out_filt_sel(&self) -> SDA_OUT_FILT_SEL_R {
        SDA_OUT_FILT_SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Trim bits for 'i2c_sda_in' 50 ns filter. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
    #[inline(always)]
    pub fn sda_in_filt_trim(&mut self) -> SDA_IN_FILT_TRIM_W {
        SDA_IN_FILT_TRIM_W::new(self)
    }
    #[doc = "Bit 4 - Selection of 'i2c_sda_in' filter delay: '0': 0 ns. '1: 50 ns (filter enabled)."]
    #[inline(always)]
    pub fn sda_in_filt_sel(&mut self) -> SDA_IN_FILT_SEL_W {
        SDA_IN_FILT_SEL_W::new(self)
    }
    #[doc = "Bits 8:9 - Trim bits for 'i2c_scl_in' 50 ns filter. See s8i2cs BROS (001-59539) for more details on the trim bit values. With s8iom0s8v1p2 I/Os, trim bits should be programmed to 3 to suppress glitches below 50ns."]
    #[inline(always)]
    pub fn scl_in_filt_trim(&mut self) -> SCL_IN_FILT_TRIM_W {
        SCL_IN_FILT_TRIM_W::new(self)
    }
    #[doc = "Bit 12 - Selection of 'i2c_scl_in' filter delay: '0': 0 ns. '1: 50 ns (filter enabled)."]
    #[inline(always)]
    pub fn scl_in_filt_sel(&mut self) -> SCL_IN_FILT_SEL_W {
        SCL_IN_FILT_SEL_W::new(self)
    }
    #[doc = "Bits 16:17 - Trim bits for 'i2c_sda_out' 50 ns filter 0. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
    #[inline(always)]
    pub fn sda_out_filt0_trim(&mut self) -> SDA_OUT_FILT0_TRIM_W {
        SDA_OUT_FILT0_TRIM_W::new(self)
    }
    #[doc = "Bits 18:19 - Trim bits for 'i2c_sda_out' 50 ns filter 1. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
    #[inline(always)]
    pub fn sda_out_filt1_trim(&mut self) -> SDA_OUT_FILT1_TRIM_W {
        SDA_OUT_FILT1_TRIM_W::new(self)
    }
    #[doc = "Bits 20:21 - Trim bits for 'i2c_sda_out' 50 ns filter 2. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
    #[inline(always)]
    pub fn sda_out_filt2_trim(&mut self) -> SDA_OUT_FILT2_TRIM_W {
        SDA_OUT_FILT2_TRIM_W::new(self)
    }
    #[doc = "Bits 28:29 - Selection of cumulative 'i2c_sda_out' filter delay: '0': 0 ns. '1': 50 ns (filter 0 enabled). '2': 100 ns (filters 0 and 1 enabled). '3': 150 ns (filters 0, 1 and 2 enabled)."]
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
#[doc = "I2C configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_cfg](index.html) module"]
pub struct I2C_CFG_SPEC;
impl crate::RegisterSpec for I2C_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_cfg::R](R) reader structure"]
impl crate::Readable for I2C_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_cfg::W](W) writer structure"]
impl crate::Writable for I2C_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_CFG to value 0x002a_1013"]
impl crate::Resettable for I2C_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x002a_1013
    }
}
