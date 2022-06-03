#[doc = "Register `CONFIG` reader"]
pub struct R(crate::R<CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG` writer"]
pub struct W(crate::W<CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_SPEC>;
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
impl From<crate::W<CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VECT_IN_RAM` reader - 0': Vector Table is located at 0x0000:0000 in flash '1': Vector Table is located at 0x2000:0000 in SRAM Note that vectors for RESET and FAULT are always fetched from ROM. Value in flash/RAM is ignored for these vectors."]
pub type VECT_IN_RAM_R = crate::BitReader<bool>;
#[doc = "Field `VECT_IN_RAM` writer - 0': Vector Table is located at 0x0000:0000 in flash '1': Vector Table is located at 0x2000:0000 in SRAM Note that vectors for RESET and FAULT are always fetched from ROM. Value in flash/RAM is ignored for these vectors."]
pub type VECT_IN_RAM_W<'a> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - 0': Vector Table is located at 0x0000:0000 in flash '1': Vector Table is located at 0x2000:0000 in SRAM Note that vectors for RESET and FAULT are always fetched from ROM. Value in flash/RAM is ignored for these vectors."]
    #[inline(always)]
    pub fn vect_in_ram(&self) -> VECT_IN_RAM_R {
        VECT_IN_RAM_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0': Vector Table is located at 0x0000:0000 in flash '1': Vector Table is located at 0x2000:0000 in SRAM Note that vectors for RESET and FAULT are always fetched from ROM. Value in flash/RAM is ignored for these vectors."]
    #[inline(always)]
    pub fn vect_in_ram(&mut self) -> VECT_IN_RAM_W {
        VECT_IN_RAM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](index.html) module"]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config::R](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config::W](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONFIG to value 0"]
impl crate::Resettable for CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
