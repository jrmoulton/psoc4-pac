#[doc = "Register `PC2` reader"]
pub struct R(crate::R<PC2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PC2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PC2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PC2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PC2` writer"]
pub struct W(crate::W<PC2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PC2_SPEC>;
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
impl From<crate::W<PC2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PC2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INP_DIS0` reader - Disables the input buffer for IO pad 0 independent of the port control drive mode (PC.DM). This bit should be set when analog signals are present on the pin and PC.DM != 0 is required to use the output driver."]
pub type INP_DIS0_R = crate::BitReader<bool>;
#[doc = "Field `INP_DIS0` writer - Disables the input buffer for IO pad 0 independent of the port control drive mode (PC.DM). This bit should be set when analog signals are present on the pin and PC.DM != 0 is required to use the output driver."]
pub type INP_DIS0_W<'a> = crate::BitWriter<'a, u32, PC2_SPEC, bool, 0>;
#[doc = "Field `INP_DIS1` reader - Disables the input buffer for IO pad 1."]
pub type INP_DIS1_R = crate::BitReader<bool>;
#[doc = "Field `INP_DIS1` writer - Disables the input buffer for IO pad 1."]
pub type INP_DIS1_W<'a> = crate::BitWriter<'a, u32, PC2_SPEC, bool, 1>;
#[doc = "Field `INP_DIS2` reader - Disables the input buffer for IO pad 2."]
pub type INP_DIS2_R = crate::BitReader<bool>;
#[doc = "Field `INP_DIS2` writer - Disables the input buffer for IO pad 2."]
pub type INP_DIS2_W<'a> = crate::BitWriter<'a, u32, PC2_SPEC, bool, 2>;
#[doc = "Field `INP_DIS3` reader - Disables the input buffer for IO pad 3."]
pub type INP_DIS3_R = crate::BitReader<bool>;
#[doc = "Field `INP_DIS3` writer - Disables the input buffer for IO pad 3."]
pub type INP_DIS3_W<'a> = crate::BitWriter<'a, u32, PC2_SPEC, bool, 3>;
#[doc = "Field `INP_DIS4` reader - Disables the input buffer for IO pad 4."]
pub type INP_DIS4_R = crate::BitReader<bool>;
#[doc = "Field `INP_DIS4` writer - Disables the input buffer for IO pad 4."]
pub type INP_DIS4_W<'a> = crate::BitWriter<'a, u32, PC2_SPEC, bool, 4>;
#[doc = "Field `INP_DIS5` reader - Disables the input buffer for IO pad 5."]
pub type INP_DIS5_R = crate::BitReader<bool>;
#[doc = "Field `INP_DIS5` writer - Disables the input buffer for IO pad 5."]
pub type INP_DIS5_W<'a> = crate::BitWriter<'a, u32, PC2_SPEC, bool, 5>;
#[doc = "Field `INP_DIS6` reader - Disables the input buffer for IO pad 6."]
pub type INP_DIS6_R = crate::BitReader<bool>;
#[doc = "Field `INP_DIS6` writer - Disables the input buffer for IO pad 6."]
pub type INP_DIS6_W<'a> = crate::BitWriter<'a, u32, PC2_SPEC, bool, 6>;
#[doc = "Field `INP_DIS7` reader - Disables the input buffer for IO pad 7."]
pub type INP_DIS7_R = crate::BitReader<bool>;
#[doc = "Field `INP_DIS7` writer - Disables the input buffer for IO pad 7."]
pub type INP_DIS7_W<'a> = crate::BitWriter<'a, u32, PC2_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - Disables the input buffer for IO pad 0 independent of the port control drive mode (PC.DM). This bit should be set when analog signals are present on the pin and PC.DM != 0 is required to use the output driver."]
    #[inline(always)]
    pub fn inp_dis0(&self) -> INP_DIS0_R {
        INP_DIS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disables the input buffer for IO pad 1."]
    #[inline(always)]
    pub fn inp_dis1(&self) -> INP_DIS1_R {
        INP_DIS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disables the input buffer for IO pad 2."]
    #[inline(always)]
    pub fn inp_dis2(&self) -> INP_DIS2_R {
        INP_DIS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Disables the input buffer for IO pad 3."]
    #[inline(always)]
    pub fn inp_dis3(&self) -> INP_DIS3_R {
        INP_DIS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Disables the input buffer for IO pad 4."]
    #[inline(always)]
    pub fn inp_dis4(&self) -> INP_DIS4_R {
        INP_DIS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Disables the input buffer for IO pad 5."]
    #[inline(always)]
    pub fn inp_dis5(&self) -> INP_DIS5_R {
        INP_DIS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Disables the input buffer for IO pad 6."]
    #[inline(always)]
    pub fn inp_dis6(&self) -> INP_DIS6_R {
        INP_DIS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Disables the input buffer for IO pad 7."]
    #[inline(always)]
    pub fn inp_dis7(&self) -> INP_DIS7_R {
        INP_DIS7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disables the input buffer for IO pad 0 independent of the port control drive mode (PC.DM). This bit should be set when analog signals are present on the pin and PC.DM != 0 is required to use the output driver."]
    #[inline(always)]
    pub fn inp_dis0(&mut self) -> INP_DIS0_W {
        INP_DIS0_W::new(self)
    }
    #[doc = "Bit 1 - Disables the input buffer for IO pad 1."]
    #[inline(always)]
    pub fn inp_dis1(&mut self) -> INP_DIS1_W {
        INP_DIS1_W::new(self)
    }
    #[doc = "Bit 2 - Disables the input buffer for IO pad 2."]
    #[inline(always)]
    pub fn inp_dis2(&mut self) -> INP_DIS2_W {
        INP_DIS2_W::new(self)
    }
    #[doc = "Bit 3 - Disables the input buffer for IO pad 3."]
    #[inline(always)]
    pub fn inp_dis3(&mut self) -> INP_DIS3_W {
        INP_DIS3_W::new(self)
    }
    #[doc = "Bit 4 - Disables the input buffer for IO pad 4."]
    #[inline(always)]
    pub fn inp_dis4(&mut self) -> INP_DIS4_W {
        INP_DIS4_W::new(self)
    }
    #[doc = "Bit 5 - Disables the input buffer for IO pad 5."]
    #[inline(always)]
    pub fn inp_dis5(&mut self) -> INP_DIS5_W {
        INP_DIS5_W::new(self)
    }
    #[doc = "Bit 6 - Disables the input buffer for IO pad 6."]
    #[inline(always)]
    pub fn inp_dis6(&mut self) -> INP_DIS6_W {
        INP_DIS6_W::new(self)
    }
    #[doc = "Bit 7 - Disables the input buffer for IO pad 7."]
    #[inline(always)]
    pub fn inp_dis7(&mut self) -> INP_DIS7_W {
        INP_DIS7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pc2](index.html) module"]
pub struct PC2_SPEC;
impl crate::RegisterSpec for PC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pc2::R](R) reader structure"]
impl crate::Readable for PC2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pc2::W](W) writer structure"]
impl crate::Writable for PC2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PC2 to value 0"]
impl crate::Resettable for PC2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
