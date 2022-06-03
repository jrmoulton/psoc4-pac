#[doc = "Register `ANA_TRIM` reader"]
pub struct R(crate::R<ANA_TRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANA_TRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANA_TRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANA_TRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANA_TRIM` writer"]
pub struct W(crate::W<ANA_TRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANA_TRIM_SPEC>;
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
impl From<crate::W<ANA_TRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANA_TRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAP_TRIM` reader - Attenuation cap trimming"]
pub type CAP_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAP_TRIM` writer - Attenuation cap trimming"]
pub type CAP_TRIM_W<'a> = crate::FieldWriter<'a, u32, ANA_TRIM_SPEC, u8, u8, 3, 0>;
#[doc = "Field `TRIMUNIT` reader - Attenuation cap trimming"]
pub type TRIMUNIT_R = crate::BitReader<bool>;
#[doc = "Field `TRIMUNIT` writer - Attenuation cap trimming"]
pub type TRIMUNIT_W<'a> = crate::BitWriter<'a, u32, ANA_TRIM_SPEC, bool, 3>;
impl R {
    #[doc = "Bits 0:2 - Attenuation cap trimming"]
    #[inline(always)]
    pub fn cap_trim(&self) -> CAP_TRIM_R {
        CAP_TRIM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Attenuation cap trimming"]
    #[inline(always)]
    pub fn trimunit(&self) -> TRIMUNIT_R {
        TRIMUNIT_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Attenuation cap trimming"]
    #[inline(always)]
    pub fn cap_trim(&mut self) -> CAP_TRIM_W {
        CAP_TRIM_W::new(self)
    }
    #[doc = "Bit 3 - Attenuation cap trimming"]
    #[inline(always)]
    pub fn trimunit(&mut self) -> TRIMUNIT_W {
        TRIMUNIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog trim register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana_trim](index.html) module"]
pub struct ANA_TRIM_SPEC;
impl crate::RegisterSpec for ANA_TRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ana_trim::R](R) reader structure"]
impl crate::Readable for ANA_TRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ana_trim::W](W) writer structure"]
impl crate::Writable for ANA_TRIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ANA_TRIM to value 0"]
impl crate::Resettable for ANA_TRIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
