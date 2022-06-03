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
#[doc = "Field `PORT_INT` reader - Each IO port has an associated bit field in this register. The bit field reflects the IO port's interrupt line (bit field i reflects 'gpio_interrupts\\[i\\]' for IO port i). The register is used when the system uses a shared/combined interrupt line 'gpio_interrupt'. The SW ISR reads the register to deternine which IO port(s) is responsible for the shared/combined interrupt line 'gpio_interrupt'. Once, the IO port(s) is determined, the IO port's INTR register is read to determine the IO pad(s) in the IO port that caused the interrupt."]
pub type PORT_INT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Each IO port has an associated bit field in this register. The bit field reflects the IO port's interrupt line (bit field i reflects 'gpio_interrupts\\[i\\]' for IO port i). The register is used when the system uses a shared/combined interrupt line 'gpio_interrupt'. The SW ISR reads the register to deternine which IO port(s) is responsible for the shared/combined interrupt line 'gpio_interrupt'. Once, the IO port(s) is determined, the IO port's INTR register is read to determine the IO pad(s) in the IO port that caused the interrupt."]
    #[inline(always)]
    pub fn port_int(&self) -> PORT_INT_R {
        PORT_INT_R::new(self.bits)
    }
}
#[doc = "Interrupt port cause register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_cause](index.html) module"]
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
