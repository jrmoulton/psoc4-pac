#[doc = "Register `OA_COMP_TRIM` reader"]
pub struct R(crate::R<OA_COMP_TRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OA_COMP_TRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OA_COMP_TRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OA_COMP_TRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OA_COMP_TRIM` writer"]
pub struct W(crate::W<OA_COMP_TRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OA_COMP_TRIM_SPEC>;
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
impl From<crate::W<OA_COMP_TRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OA_COMP_TRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Opamp Compenation Capacitor Trim\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OA_COMP_TRIM_A {
    #[doc = "1: High Speed"]
    HIGH_SPEED = 1,
    #[doc = "2: Balanced"]
    BALANCED = 2,
    #[doc = "3: High Stability"]
    HIGH_STABILITY = 3,
}
impl From<OA_COMP_TRIM_A> for u8 {
    #[inline(always)]
    fn from(variant: OA_COMP_TRIM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OA_COMP_TRIM` reader - Opamp Compenation Capacitor Trim"]
pub type OA_COMP_TRIM_R = crate::FieldReader<u8, OA_COMP_TRIM_A>;
impl OA_COMP_TRIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OA_COMP_TRIM_A> {
        match self.bits {
            1 => Some(OA_COMP_TRIM_A::HIGH_SPEED),
            2 => Some(OA_COMP_TRIM_A::BALANCED),
            3 => Some(OA_COMP_TRIM_A::HIGH_STABILITY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH_SPEED`"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == OA_COMP_TRIM_A::HIGH_SPEED
    }
    #[doc = "Checks if the value of the field is `BALANCED`"]
    #[inline(always)]
    pub fn is_balanced(&self) -> bool {
        *self == OA_COMP_TRIM_A::BALANCED
    }
    #[doc = "Checks if the value of the field is `HIGH_STABILITY`"]
    #[inline(always)]
    pub fn is_high_stability(&self) -> bool {
        *self == OA_COMP_TRIM_A::HIGH_STABILITY
    }
}
#[doc = "Field `OA_COMP_TRIM` writer - Opamp Compenation Capacitor Trim"]
pub type OA_COMP_TRIM_W<'a> =
    crate::FieldWriter<'a, u32, OA_COMP_TRIM_SPEC, u8, OA_COMP_TRIM_A, 2, 0>;
impl<'a> OA_COMP_TRIM_W<'a> {
    #[doc = "High Speed"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OA_COMP_TRIM_A::HIGH_SPEED)
    }
    #[doc = "Balanced"]
    #[inline(always)]
    pub fn balanced(self) -> &'a mut W {
        self.variant(OA_COMP_TRIM_A::BALANCED)
    }
    #[doc = "High Stability"]
    #[inline(always)]
    pub fn high_stability(self) -> &'a mut W {
        self.variant(OA_COMP_TRIM_A::HIGH_STABILITY)
    }
}
impl R {
    #[doc = "Bits 0:1 - Opamp Compenation Capacitor Trim"]
    #[inline(always)]
    pub fn oa_comp_trim(&self) -> OA_COMP_TRIM_R {
        OA_COMP_TRIM_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Opamp Compenation Capacitor Trim"]
    #[inline(always)]
    pub fn oa_comp_trim(&mut self) -> OA_COMP_TRIM_W {
        OA_COMP_TRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Opamp Compenation Capacitor Trim\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oa_comp_trim](index.html) module"]
pub struct OA_COMP_TRIM_SPEC;
impl crate::RegisterSpec for OA_COMP_TRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oa_comp_trim::R](R) reader structure"]
impl crate::Readable for OA_COMP_TRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oa_comp_trim::W](W) writer structure"]
impl crate::Writable for OA_COMP_TRIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OA_COMP_TRIM to value 0"]
impl crate::Resettable for OA_COMP_TRIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
