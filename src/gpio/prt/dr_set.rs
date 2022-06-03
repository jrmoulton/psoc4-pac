#[doc = "Register `DR_SET` reader"]
pub struct R(crate::R<DR_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DR_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DR_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DR_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DR_SET` writer"]
pub struct W(crate::W<DR_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DR_SET_SPEC>;
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
impl From<crate::W<DR_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DR_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - IO pad i: '0': Output state DR.DATA\\[i\\]
not affected. '1': Output state DR.DATA\\[i\\]
set to '1'."]
pub type DATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA` writer - IO pad i: '0': Output state DR.DATA\\[i\\]
not affected. '1': Output state DR.DATA\\[i\\]
set to '1'."]
pub type DATA_W<'a> = crate::FieldWriter<'a, u32, DR_SET_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - IO pad i: '0': Output state DR.DATA\\[i\\]
not affected. '1': Output state DR.DATA\\[i\\]
set to '1'."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - IO pad i: '0': Output state DR.DATA\\[i\\]
not affected. '1': Output state DR.DATA\\[i\\]
set to '1'."]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port output data set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr_set](index.html) module"]
pub struct DR_SET_SPEC;
impl crate::RegisterSpec for DR_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dr_set::R](R) reader structure"]
impl crate::Readable for DR_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dr_set::W](W) writer structure"]
impl crate::Writable for DR_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DR_SET to value 0"]
impl crate::Resettable for DR_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
