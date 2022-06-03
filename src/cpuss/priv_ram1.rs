#[doc = "Register `PRIV_RAM1` reader"]
pub struct R(crate::R<PRIV_RAM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRIV_RAM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRIV_RAM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRIV_RAM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRIV_RAM1` writer"]
pub struct W(crate::W<PRIV_RAM1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRIV_RAM1_SPEC>;
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
impl From<crate::W<PRIV_RAM1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRIV_RAM1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAM_PROT_LIMIT` reader - See description of PRIV_RAM.RAM_PROT_LIMIT. Note that the reset value is 0x1ff, indicating that the complete RAM 1 memory capacity is User accessible."]
pub type RAM_PROT_LIMIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RAM_PROT_LIMIT` writer - See description of PRIV_RAM.RAM_PROT_LIMIT. Note that the reset value is 0x1ff, indicating that the complete RAM 1 memory capacity is User accessible."]
pub type RAM_PROT_LIMIT_W<'a> = crate::FieldWriter<'a, u32, PRIV_RAM1_SPEC, u16, u16, 9, 0>;
impl R {
    #[doc = "Bits 0:8 - See description of PRIV_RAM.RAM_PROT_LIMIT. Note that the reset value is 0x1ff, indicating that the complete RAM 1 memory capacity is User accessible."]
    #[inline(always)]
    pub fn ram_prot_limit(&self) -> RAM_PROT_LIMIT_R {
        RAM_PROT_LIMIT_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - See description of PRIV_RAM.RAM_PROT_LIMIT. Note that the reset value is 0x1ff, indicating that the complete RAM 1 memory capacity is User accessible."]
    #[inline(always)]
    pub fn ram_prot_limit(&mut self) -> RAM_PROT_LIMIT_W {
        RAM_PROT_LIMIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAM 1 privilege register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [priv_ram1](index.html) module"]
pub struct PRIV_RAM1_SPEC;
impl crate::RegisterSpec for PRIV_RAM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [priv_ram1::R](R) reader structure"]
impl crate::Readable for PRIV_RAM1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [priv_ram1::W](W) writer structure"]
impl crate::Writable for PRIV_RAM1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRIV_RAM1 to value 0x01ff"]
impl crate::Resettable for PRIV_RAM1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01ff
    }
}
