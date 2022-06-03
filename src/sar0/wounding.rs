#[doc = "Register `WOUNDING` reader"]
pub struct R(crate::R<WOUNDING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WOUNDING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WOUNDING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WOUNDING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WOUNDING` writer"]
pub struct W(crate::W<WOUNDING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WOUNDING_SPEC>;
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
impl From<crate::W<WOUNDING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WOUNDING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Maximum SAR resolution allowed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WOUND_RESOLUTION_A {
    #[doc = "0: unwounded: up to full 12-bit SAR resolution allowed"]
    _12BIT = 0,
    #[doc = "1: wounded: max resolution upto 10-bit SAR resolution allowed"]
    _10BIT = 1,
    #[doc = "2: wounded: only 8-bit SAR resolution allowed"]
    _8BIT = 2,
    #[doc = "3: wounded: only 8-bit SAR resolution allowed"]
    _8BIT_TOO = 3,
}
impl From<WOUND_RESOLUTION_A> for u8 {
    #[inline(always)]
    fn from(variant: WOUND_RESOLUTION_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WOUND_RESOLUTION` reader - Maximum SAR resolution allowed"]
pub type WOUND_RESOLUTION_R = crate::FieldReader<u8, WOUND_RESOLUTION_A>;
impl WOUND_RESOLUTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WOUND_RESOLUTION_A {
        match self.bits {
            0 => WOUND_RESOLUTION_A::_12BIT,
            1 => WOUND_RESOLUTION_A::_10BIT,
            2 => WOUND_RESOLUTION_A::_8BIT,
            3 => WOUND_RESOLUTION_A::_8BIT_TOO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_12BIT`"]
    #[inline(always)]
    pub fn is_12bit(&self) -> bool {
        *self == WOUND_RESOLUTION_A::_12BIT
    }
    #[doc = "Checks if the value of the field is `_10BIT`"]
    #[inline(always)]
    pub fn is_10bit(&self) -> bool {
        *self == WOUND_RESOLUTION_A::_10BIT
    }
    #[doc = "Checks if the value of the field is `_8BIT`"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == WOUND_RESOLUTION_A::_8BIT
    }
    #[doc = "Checks if the value of the field is `_8BIT_TOO`"]
    #[inline(always)]
    pub fn is_8bit_too(&self) -> bool {
        *self == WOUND_RESOLUTION_A::_8BIT_TOO
    }
}
#[doc = "Field `WOUND_RESOLUTION` writer - Maximum SAR resolution allowed"]
pub type WOUND_RESOLUTION_W<'a> =
    crate::FieldWriterSafe<'a, u32, WOUNDING_SPEC, u8, WOUND_RESOLUTION_A, 2, 0>;
impl<'a> WOUND_RESOLUTION_W<'a> {
    #[doc = "unwounded: up to full 12-bit SAR resolution allowed"]
    #[inline(always)]
    pub fn _12bit(self) -> &'a mut W {
        self.variant(WOUND_RESOLUTION_A::_12BIT)
    }
    #[doc = "wounded: max resolution upto 10-bit SAR resolution allowed"]
    #[inline(always)]
    pub fn _10bit(self) -> &'a mut W {
        self.variant(WOUND_RESOLUTION_A::_10BIT)
    }
    #[doc = "wounded: only 8-bit SAR resolution allowed"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut W {
        self.variant(WOUND_RESOLUTION_A::_8BIT)
    }
    #[doc = "wounded: only 8-bit SAR resolution allowed"]
    #[inline(always)]
    pub fn _8bit_too(self) -> &'a mut W {
        self.variant(WOUND_RESOLUTION_A::_8BIT_TOO)
    }
}
impl R {
    #[doc = "Bits 0:1 - Maximum SAR resolution allowed"]
    #[inline(always)]
    pub fn wound_resolution(&self) -> WOUND_RESOLUTION_R {
        WOUND_RESOLUTION_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Maximum SAR resolution allowed"]
    #[inline(always)]
    pub fn wound_resolution(&mut self) -> WOUND_RESOLUTION_W {
        WOUND_RESOLUTION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SAR wounding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wounding](index.html) module"]
pub struct WOUNDING_SPEC;
impl crate::RegisterSpec for WOUNDING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wounding::R](R) reader structure"]
impl crate::Readable for WOUNDING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wounding::W](W) writer structure"]
impl crate::Writable for WOUNDING_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WOUNDING to value 0"]
impl crate::Resettable for WOUNDING_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
