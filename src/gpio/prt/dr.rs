#[doc = "Register `DR` reader"]
pub struct R(crate::R<DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DR` writer"]
pub struct W(crate::W<DR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DR_SPEC>;
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
impl From<crate::W<DR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA0` reader - IO pad 0 output data."]
pub type DATA0_R = crate::BitReader<bool>;
#[doc = "Field `DATA0` writer - IO pad 0 output data."]
pub type DATA0_W<'a> = crate::BitWriter<'a, u32, DR_SPEC, bool, 0>;
#[doc = "Field `DATA1` reader - IO pad 1 output data."]
pub type DATA1_R = crate::BitReader<bool>;
#[doc = "Field `DATA1` writer - IO pad 1 output data."]
pub type DATA1_W<'a> = crate::BitWriter<'a, u32, DR_SPEC, bool, 1>;
#[doc = "Field `DATA2` reader - IO pad 2 output data."]
pub type DATA2_R = crate::BitReader<bool>;
#[doc = "Field `DATA2` writer - IO pad 2 output data."]
pub type DATA2_W<'a> = crate::BitWriter<'a, u32, DR_SPEC, bool, 2>;
#[doc = "Field `DATA3` reader - IO pad 3 output data."]
pub type DATA3_R = crate::BitReader<bool>;
#[doc = "Field `DATA3` writer - IO pad 3 output data."]
pub type DATA3_W<'a> = crate::BitWriter<'a, u32, DR_SPEC, bool, 3>;
#[doc = "Field `DATA4` reader - IO pad 4 output data."]
pub type DATA4_R = crate::BitReader<bool>;
#[doc = "Field `DATA4` writer - IO pad 4 output data."]
pub type DATA4_W<'a> = crate::BitWriter<'a, u32, DR_SPEC, bool, 4>;
#[doc = "Field `DATA5` reader - IO pad 5 output data."]
pub type DATA5_R = crate::BitReader<bool>;
#[doc = "Field `DATA5` writer - IO pad 5 output data."]
pub type DATA5_W<'a> = crate::BitWriter<'a, u32, DR_SPEC, bool, 5>;
#[doc = "Field `DATA6` reader - IO pad 6 output data."]
pub type DATA6_R = crate::BitReader<bool>;
#[doc = "Field `DATA6` writer - IO pad 6 output data."]
pub type DATA6_W<'a> = crate::BitWriter<'a, u32, DR_SPEC, bool, 6>;
#[doc = "Field `DATA7` reader - IO pad 7 output data."]
pub type DATA7_R = crate::BitReader<bool>;
#[doc = "Field `DATA7` writer - IO pad 7 output data."]
pub type DATA7_W<'a> = crate::BitWriter<'a, u32, DR_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - IO pad 0 output data."]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO pad 1 output data."]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO pad 2 output data."]
    #[inline(always)]
    pub fn data2(&self) -> DATA2_R {
        DATA2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO pad 3 output data."]
    #[inline(always)]
    pub fn data3(&self) -> DATA3_R {
        DATA3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO pad 4 output data."]
    #[inline(always)]
    pub fn data4(&self) -> DATA4_R {
        DATA4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO pad 5 output data."]
    #[inline(always)]
    pub fn data5(&self) -> DATA5_R {
        DATA5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO pad 6 output data."]
    #[inline(always)]
    pub fn data6(&self) -> DATA6_R {
        DATA6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IO pad 7 output data."]
    #[inline(always)]
    pub fn data7(&self) -> DATA7_R {
        DATA7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO pad 0 output data."]
    #[inline(always)]
    pub fn data0(&mut self) -> DATA0_W {
        DATA0_W::new(self)
    }
    #[doc = "Bit 1 - IO pad 1 output data."]
    #[inline(always)]
    pub fn data1(&mut self) -> DATA1_W {
        DATA1_W::new(self)
    }
    #[doc = "Bit 2 - IO pad 2 output data."]
    #[inline(always)]
    pub fn data2(&mut self) -> DATA2_W {
        DATA2_W::new(self)
    }
    #[doc = "Bit 3 - IO pad 3 output data."]
    #[inline(always)]
    pub fn data3(&mut self) -> DATA3_W {
        DATA3_W::new(self)
    }
    #[doc = "Bit 4 - IO pad 4 output data."]
    #[inline(always)]
    pub fn data4(&mut self) -> DATA4_W {
        DATA4_W::new(self)
    }
    #[doc = "Bit 5 - IO pad 5 output data."]
    #[inline(always)]
    pub fn data5(&mut self) -> DATA5_W {
        DATA5_W::new(self)
    }
    #[doc = "Bit 6 - IO pad 6 output data."]
    #[inline(always)]
    pub fn data6(&mut self) -> DATA6_W {
        DATA6_W::new(self)
    }
    #[doc = "Bit 7 - IO pad 7 output data."]
    #[inline(always)]
    pub fn data7(&mut self) -> DATA7_W {
        DATA7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port output data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr](index.html) module"]
pub struct DR_SPEC;
impl crate::RegisterSpec for DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dr::R](R) reader structure"]
impl crate::Readable for DR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dr::W](W) writer structure"]
impl crate::Writable for DR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DR to value 0"]
impl crate::Resettable for DR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
