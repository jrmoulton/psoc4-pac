#[doc = "Register `INTR_CAUSE` reader"]
pub struct R(crate::R<INTR_CAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_CAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_CAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_CAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CTB0_INT` reader - CTB0 interrupt pending"]
pub type CTB0_INT_R = crate::BitReader<bool>;
#[doc = "Field `CTB1_INT` reader - CTB1 interrupt pending"]
pub type CTB1_INT_R = crate::BitReader<bool>;
#[doc = "Field `CTB2_INT` reader - CTB2 interrupt pending"]
pub type CTB2_INT_R = crate::BitReader<bool>;
#[doc = "Field `CTB3_INT` reader - CTB3 interrupt pending"]
pub type CTB3_INT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - CTB0 interrupt pending"]
    #[inline(always)]
    pub fn ctb0_int(&self) -> CTB0_INT_R {
        CTB0_INT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CTB1 interrupt pending"]
    #[inline(always)]
    pub fn ctb1_int(&self) -> CTB1_INT_R {
        CTB1_INT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CTB2 interrupt pending"]
    #[inline(always)]
    pub fn ctb2_int(&self) -> CTB2_INT_R {
        CTB2_INT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CTB3 interrupt pending"]
    #[inline(always)]
    pub fn ctb3_int(&self) -> CTB3_INT_R {
        CTB3_INT_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Interrupt cause register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_cause](index.html) module"]
pub struct INTR_CAUSE_SPEC;
impl crate::RegisterSpec for INTR_CAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_cause::R](R) reader structure"]
impl crate::Readable for INTR_CAUSE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_CAUSE to value 0"]
impl crate::Resettable for INTR_CAUSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
