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
#[doc = "Field `RAM_WOUND` reader - Indicates the amount of accessible RAM 0 memory capacitty in this part. The value in this field is effectively write-once (it is only possible to set bits, not clear them). The remainder portion of SRAM is not accessible and will return an AHB-Lite bus error. '0': entire memory accessible '1': first 1/2 of the memory accessible '2': first 1/4 of the memory accessible '3': first 1/8 of the memory accessible '4': first 1/16 of the memory accessible '5': first 1/32 of the memory accessible '6': first 1/64 of the memory accessible '7': first 1/128 of the memory accessible"]
pub type RAM_WOUND_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RAM_WOUND` writer - Indicates the amount of accessible RAM 0 memory capacitty in this part. The value in this field is effectively write-once (it is only possible to set bits, not clear them). The remainder portion of SRAM is not accessible and will return an AHB-Lite bus error. '0': entire memory accessible '1': first 1/2 of the memory accessible '2': first 1/4 of the memory accessible '3': first 1/8 of the memory accessible '4': first 1/16 of the memory accessible '5': first 1/32 of the memory accessible '6': first 1/64 of the memory accessible '7': first 1/128 of the memory accessible"]
pub type RAM_WOUND_W<'a> = crate::FieldWriter<'a, u32, WOUNDING_SPEC, u8, u8, 3, 16>;
#[doc = "Field `FLASH_WOUND` reader - Indicates the amount of accessible flash in this part. The value in this field is effectively write-once (it is only possible to set bits, not clear them). The remainder portion of flash is not accessible and will return an AHB-Lite bus error. '0': entire memory accessible '1': first 1/2 of the memory accessible '2': first 1/4 of the memory accessible '3': first 1/8 of the memory accessible '4': first 1/16 of the memory accessible '5': first 1/32 of the memory accessible '6': first 1/64 of the memory accessible '7': first 1/128 of the memory accessible (used for the DEAD protection mode)"]
pub type FLASH_WOUND_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLASH_WOUND` writer - Indicates the amount of accessible flash in this part. The value in this field is effectively write-once (it is only possible to set bits, not clear them). The remainder portion of flash is not accessible and will return an AHB-Lite bus error. '0': entire memory accessible '1': first 1/2 of the memory accessible '2': first 1/4 of the memory accessible '3': first 1/8 of the memory accessible '4': first 1/16 of the memory accessible '5': first 1/32 of the memory accessible '6': first 1/64 of the memory accessible '7': first 1/128 of the memory accessible (used for the DEAD protection mode)"]
pub type FLASH_WOUND_W<'a> = crate::FieldWriter<'a, u32, WOUNDING_SPEC, u8, u8, 3, 20>;
#[doc = "Field `RAM1_WOUND` reader - Wounding of RAM 1 (see description of RAM_WOUND)."]
pub type RAM1_WOUND_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RAM1_WOUND` writer - Wounding of RAM 1 (see description of RAM_WOUND)."]
pub type RAM1_WOUND_W<'a> = crate::FieldWriter<'a, u32, WOUNDING_SPEC, u8, u8, 3, 24>;
impl R {
    #[doc = "Bits 16:18 - Indicates the amount of accessible RAM 0 memory capacitty in this part. The value in this field is effectively write-once (it is only possible to set bits, not clear them). The remainder portion of SRAM is not accessible and will return an AHB-Lite bus error. '0': entire memory accessible '1': first 1/2 of the memory accessible '2': first 1/4 of the memory accessible '3': first 1/8 of the memory accessible '4': first 1/16 of the memory accessible '5': first 1/32 of the memory accessible '6': first 1/64 of the memory accessible '7': first 1/128 of the memory accessible"]
    #[inline(always)]
    pub fn ram_wound(&self) -> RAM_WOUND_R {
        RAM_WOUND_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Indicates the amount of accessible flash in this part. The value in this field is effectively write-once (it is only possible to set bits, not clear them). The remainder portion of flash is not accessible and will return an AHB-Lite bus error. '0': entire memory accessible '1': first 1/2 of the memory accessible '2': first 1/4 of the memory accessible '3': first 1/8 of the memory accessible '4': first 1/16 of the memory accessible '5': first 1/32 of the memory accessible '6': first 1/64 of the memory accessible '7': first 1/128 of the memory accessible (used for the DEAD protection mode)"]
    #[inline(always)]
    pub fn flash_wound(&self) -> FLASH_WOUND_R {
        FLASH_WOUND_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Wounding of RAM 1 (see description of RAM_WOUND)."]
    #[inline(always)]
    pub fn ram1_wound(&self) -> RAM1_WOUND_R {
        RAM1_WOUND_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 16:18 - Indicates the amount of accessible RAM 0 memory capacitty in this part. The value in this field is effectively write-once (it is only possible to set bits, not clear them). The remainder portion of SRAM is not accessible and will return an AHB-Lite bus error. '0': entire memory accessible '1': first 1/2 of the memory accessible '2': first 1/4 of the memory accessible '3': first 1/8 of the memory accessible '4': first 1/16 of the memory accessible '5': first 1/32 of the memory accessible '6': first 1/64 of the memory accessible '7': first 1/128 of the memory accessible"]
    #[inline(always)]
    pub fn ram_wound(&mut self) -> RAM_WOUND_W {
        RAM_WOUND_W::new(self)
    }
    #[doc = "Bits 20:22 - Indicates the amount of accessible flash in this part. The value in this field is effectively write-once (it is only possible to set bits, not clear them). The remainder portion of flash is not accessible and will return an AHB-Lite bus error. '0': entire memory accessible '1': first 1/2 of the memory accessible '2': first 1/4 of the memory accessible '3': first 1/8 of the memory accessible '4': first 1/16 of the memory accessible '5': first 1/32 of the memory accessible '6': first 1/64 of the memory accessible '7': first 1/128 of the memory accessible (used for the DEAD protection mode)"]
    #[inline(always)]
    pub fn flash_wound(&mut self) -> FLASH_WOUND_W {
        FLASH_WOUND_W::new(self)
    }
    #[doc = "Bits 24:26 - Wounding of RAM 1 (see description of RAM_WOUND)."]
    #[inline(always)]
    pub fn ram1_wound(&mut self) -> RAM1_WOUND_W {
        RAM1_WOUND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wounding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wounding](index.html) module"]
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
