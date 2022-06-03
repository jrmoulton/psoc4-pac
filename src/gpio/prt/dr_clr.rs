#[doc = "Register `DR_CLR` reader"]
pub struct R(crate::R<DR_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DR_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DR_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DR_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DR_CLR` writer"]
pub struct W(crate::W<DR_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DR_CLR_SPEC>;
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
impl From<crate::W<DR_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DR_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - IO pad i: '0': Output state DR.DATA\\[i\\]
not affected. '1': Output state DR.DATA\\[i\\]
set to '0'."]
pub type DATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA` writer - IO pad i: '0': Output state DR.DATA\\[i\\]
not affected. '1': Output state DR.DATA\\[i\\]
set to '0'."]
pub type DATA_W<'a> = crate::FieldWriter<'a, u32, DR_CLR_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - IO pad i: '0': Output state DR.DATA\\[i\\]
not affected. '1': Output state DR.DATA\\[i\\]
set to '0'."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - IO pad i: '0': Output state DR.DATA\\[i\\]
not affected. '1': Output state DR.DATA\\[i\\]
set to '0'."]
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
#[doc = "Port output data clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr_clr](index.html) module"]
pub struct DR_CLR_SPEC;
impl crate::RegisterSpec for DR_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dr_clr::R](R) reader structure"]
impl crate::Readable for DR_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dr_clr::W](W) writer structure"]
impl crate::Writable for DR_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DR_CLR to value 0"]
impl crate::Resettable for DR_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
