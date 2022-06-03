#[doc = "Register `PRIV_ROM` reader"]
pub struct R(crate::R<PRIV_ROM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRIV_ROM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRIV_ROM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRIV_ROM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRIV_ROM` writer"]
pub struct W(crate::W<PRIV_ROM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRIV_ROM_SPEC>;
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
impl From<crate::W<PRIV_ROM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRIV_ROM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BROM_PROT_LIMIT` reader - Indicates the limit where the privileged area of the Boot ROM partition starts in increments of 256 Bytes. '0': Entire Boot ROM is Privileged. '1': First 256 Bytes are User accessible. ... BROM_PROT_LIMIT >= 'Boot ROM partition capacity': Entire Boot ROM partition is user mode accessible."]
pub type BROM_PROT_LIMIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BROM_PROT_LIMIT` writer - Indicates the limit where the privileged area of the Boot ROM partition starts in increments of 256 Bytes. '0': Entire Boot ROM is Privileged. '1': First 256 Bytes are User accessible. ... BROM_PROT_LIMIT >= 'Boot ROM partition capacity': Entire Boot ROM partition is user mode accessible."]
pub type BROM_PROT_LIMIT_W<'a> = crate::FieldWriter<'a, u32, PRIV_ROM_SPEC, u8, u8, 8, 0>;
#[doc = "Field `SROM_PROT_LIMIT` reader - Indicates the limit where the privileged area of System ROM partition starts in increments of 256 Bytes. The limit is wrt. the start of the ROM memory (start of the Boot ROM partition). SROM_PROT_LIMIT * 256 Byte <= 'Boot ROM partition capacity': Entire System ROM is Privileged. SROM_PROT_LIMIT * 256 Byte > 'Boot ROM partition capacity': First SROM_PROT_LIMIT * 256 - 'Boot ROM partition capacity' Bytes are User accessible. ... SROM_PROT_LIMIT >= 'ROM capacity': Entire System ROM is user mode accessible."]
pub type SROM_PROT_LIMIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SROM_PROT_LIMIT` writer - Indicates the limit where the privileged area of System ROM partition starts in increments of 256 Bytes. The limit is wrt. the start of the ROM memory (start of the Boot ROM partition). SROM_PROT_LIMIT * 256 Byte <= 'Boot ROM partition capacity': Entire System ROM is Privileged. SROM_PROT_LIMIT * 256 Byte > 'Boot ROM partition capacity': First SROM_PROT_LIMIT * 256 - 'Boot ROM partition capacity' Bytes are User accessible. ... SROM_PROT_LIMIT >= 'ROM capacity': Entire System ROM is user mode accessible."]
pub type SROM_PROT_LIMIT_W<'a> = crate::FieldWriter<'a, u32, PRIV_ROM_SPEC, u16, u16, 10, 16>;
impl R {
    #[doc = "Bits 0:7 - Indicates the limit where the privileged area of the Boot ROM partition starts in increments of 256 Bytes. '0': Entire Boot ROM is Privileged. '1': First 256 Bytes are User accessible. ... BROM_PROT_LIMIT >= 'Boot ROM partition capacity': Entire Boot ROM partition is user mode accessible."]
    #[inline(always)]
    pub fn brom_prot_limit(&self) -> BROM_PROT_LIMIT_R {
        BROM_PROT_LIMIT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:25 - Indicates the limit where the privileged area of System ROM partition starts in increments of 256 Bytes. The limit is wrt. the start of the ROM memory (start of the Boot ROM partition). SROM_PROT_LIMIT * 256 Byte <= 'Boot ROM partition capacity': Entire System ROM is Privileged. SROM_PROT_LIMIT * 256 Byte > 'Boot ROM partition capacity': First SROM_PROT_LIMIT * 256 - 'Boot ROM partition capacity' Bytes are User accessible. ... SROM_PROT_LIMIT >= 'ROM capacity': Entire System ROM is user mode accessible."]
    #[inline(always)]
    pub fn srom_prot_limit(&self) -> SROM_PROT_LIMIT_R {
        SROM_PROT_LIMIT_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indicates the limit where the privileged area of the Boot ROM partition starts in increments of 256 Bytes. '0': Entire Boot ROM is Privileged. '1': First 256 Bytes are User accessible. ... BROM_PROT_LIMIT >= 'Boot ROM partition capacity': Entire Boot ROM partition is user mode accessible."]
    #[inline(always)]
    pub fn brom_prot_limit(&mut self) -> BROM_PROT_LIMIT_W {
        BROM_PROT_LIMIT_W::new(self)
    }
    #[doc = "Bits 16:25 - Indicates the limit where the privileged area of System ROM partition starts in increments of 256 Bytes. The limit is wrt. the start of the ROM memory (start of the Boot ROM partition). SROM_PROT_LIMIT * 256 Byte <= 'Boot ROM partition capacity': Entire System ROM is Privileged. SROM_PROT_LIMIT * 256 Byte > 'Boot ROM partition capacity': First SROM_PROT_LIMIT * 256 - 'Boot ROM partition capacity' Bytes are User accessible. ... SROM_PROT_LIMIT >= 'ROM capacity': Entire System ROM is user mode accessible."]
    #[inline(always)]
    pub fn srom_prot_limit(&mut self) -> SROM_PROT_LIMIT_W {
        SROM_PROT_LIMIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ROM privilege register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [priv_rom](index.html) module"]
pub struct PRIV_ROM_SPEC;
impl crate::RegisterSpec for PRIV_ROM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [priv_rom::R](R) reader structure"]
impl crate::Readable for PRIV_ROM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [priv_rom::W](W) writer structure"]
impl crate::Writable for PRIV_ROM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRIV_ROM to value 0"]
impl crate::Resettable for PRIV_ROM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
