#[doc = "Register `SRSS_INTR_SET` reader"]
pub struct R(crate::R<SRSS_INTR_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRSS_INTR_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRSS_INTR_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRSS_INTR_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRSS_INTR_SET` writer"]
pub struct W(crate::W<SRSS_INTR_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRSS_INTR_SET_SPEC>;
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
impl From<crate::W<SRSS_INTR_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRSS_INTR_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEMP_HIGH` reader - Writing 1 to this bit internally sets the overtemp interrupt. This can be observed by reading SRSS_INTR.TEMP_HIGH. This bit always reads back as zero."]
pub type TEMP_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `TEMP_HIGH` writer - Writing 1 to this bit internally sets the overtemp interrupt. This can be observed by reading SRSS_INTR.TEMP_HIGH. This bit always reads back as zero."]
pub type TEMP_HIGH_W<'a> = crate::BitWriter<'a, u32, SRSS_INTR_SET_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 1 - Writing 1 to this bit internally sets the overtemp interrupt. This can be observed by reading SRSS_INTR.TEMP_HIGH. This bit always reads back as zero."]
    #[inline(always)]
    pub fn temp_high(&self) -> TEMP_HIGH_R {
        TEMP_HIGH_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Writing 1 to this bit internally sets the overtemp interrupt. This can be observed by reading SRSS_INTR.TEMP_HIGH. This bit always reads back as zero."]
    #[inline(always)]
    pub fn temp_high(&mut self) -> TEMP_HIGH_W {
        TEMP_HIGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRSS Interrupt Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srss_intr_set](index.html) module"]
pub struct SRSS_INTR_SET_SPEC;
impl crate::RegisterSpec for SRSS_INTR_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srss_intr_set::R](R) reader structure"]
impl crate::Readable for SRSS_INTR_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srss_intr_set::W](W) writer structure"]
impl crate::Writable for SRSS_INTR_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRSS_INTR_SET to value 0"]
impl crate::Resettable for SRSS_INTR_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
