#[doc = "Register `PROTECTION` reader"]
pub struct R(crate::R<PROTECTION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PROTECTION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PROTECTION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PROTECTION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PROTECTION` writer"]
pub struct W(crate::W<PROTECTION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PROTECTION_SPEC>;
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
impl From<crate::W<PROTECTION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PROTECTION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROTECTION_MODE` reader - Current protection mode; this field is available as a global signal everywhere in the system. Writes to this field are ignored when PROTECTION_LOCK is '1': 0b1xxx: BOOT 0b01xx: KILL 0b001x: PROTECTED 0b0001: OPEN 0b0000: VIRGIN (also used for DEAD mode, but then FLASH_LOCK is also set)"]
pub type PROTECTION_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PROTECTION_MODE` writer - Current protection mode; this field is available as a global signal everywhere in the system. Writes to this field are ignored when PROTECTION_LOCK is '1': 0b1xxx: BOOT 0b01xx: KILL 0b001x: PROTECTED 0b0001: OPEN 0b0000: VIRGIN (also used for DEAD mode, but then FLASH_LOCK is also set)"]
pub type PROTECTION_MODE_W<'a> = crate::FieldWriter<'a, u32, PROTECTION_SPEC, u8, u8, 4, 0>;
#[doc = "Field `FLASH_LOCK` reader - Setting this bit will force SPCIF.ADDRESS.AXA to be ignored, which prevents SM Flash from being erased or overwritten. It is used to indicate the DEAD protection mode. Writes to this field are ignored when PROTECTION_LOCK is '1'"]
pub type FLASH_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_LOCK` writer - Setting this bit will force SPCIF.ADDRESS.AXA to be ignored, which prevents SM Flash from being erased or overwritten. It is used to indicate the DEAD protection mode. Writes to this field are ignored when PROTECTION_LOCK is '1'"]
pub type FLASH_LOCK_W<'a> = crate::BitWriter<'a, u32, PROTECTION_SPEC, bool, 30>;
#[doc = "Field `PROTECTION_LOCK` reader - Setting this field will block (ignore) any further writes to the PROTECTION_MODE field in this register. Once '1', this field cannot be cleared."]
pub type PROTECTION_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `PROTECTION_LOCK` writer - Setting this field will block (ignore) any further writes to the PROTECTION_MODE field in this register. Once '1', this field cannot be cleared."]
pub type PROTECTION_LOCK_W<'a> = crate::BitWriter<'a, u32, PROTECTION_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:3 - Current protection mode; this field is available as a global signal everywhere in the system. Writes to this field are ignored when PROTECTION_LOCK is '1': 0b1xxx: BOOT 0b01xx: KILL 0b001x: PROTECTED 0b0001: OPEN 0b0000: VIRGIN (also used for DEAD mode, but then FLASH_LOCK is also set)"]
    #[inline(always)]
    pub fn protection_mode(&self) -> PROTECTION_MODE_R {
        PROTECTION_MODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Setting this bit will force SPCIF.ADDRESS.AXA to be ignored, which prevents SM Flash from being erased or overwritten. It is used to indicate the DEAD protection mode. Writes to this field are ignored when PROTECTION_LOCK is '1'"]
    #[inline(always)]
    pub fn flash_lock(&self) -> FLASH_LOCK_R {
        FLASH_LOCK_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Setting this field will block (ignore) any further writes to the PROTECTION_MODE field in this register. Once '1', this field cannot be cleared."]
    #[inline(always)]
    pub fn protection_lock(&self) -> PROTECTION_LOCK_R {
        PROTECTION_LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Current protection mode; this field is available as a global signal everywhere in the system. Writes to this field are ignored when PROTECTION_LOCK is '1': 0b1xxx: BOOT 0b01xx: KILL 0b001x: PROTECTED 0b0001: OPEN 0b0000: VIRGIN (also used for DEAD mode, but then FLASH_LOCK is also set)"]
    #[inline(always)]
    pub fn protection_mode(&mut self) -> PROTECTION_MODE_W {
        PROTECTION_MODE_W::new(self)
    }
    #[doc = "Bit 30 - Setting this bit will force SPCIF.ADDRESS.AXA to be ignored, which prevents SM Flash from being erased or overwritten. It is used to indicate the DEAD protection mode. Writes to this field are ignored when PROTECTION_LOCK is '1'"]
    #[inline(always)]
    pub fn flash_lock(&mut self) -> FLASH_LOCK_W {
        FLASH_LOCK_W::new(self)
    }
    #[doc = "Bit 31 - Setting this field will block (ignore) any further writes to the PROTECTION_MODE field in this register. Once '1', this field cannot be cleared."]
    #[inline(always)]
    pub fn protection_lock(&mut self) -> PROTECTION_LOCK_W {
        PROTECTION_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Protection control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [protection](index.html) module"]
pub struct PROTECTION_SPEC;
impl crate::RegisterSpec for PROTECTION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [protection::R](R) reader structure"]
impl crate::Readable for PROTECTION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [protection::W](W) writer structure"]
impl crate::Writable for PROTECTION_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PROTECTION to value 0x0f"]
impl crate::Resettable for PROTECTION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
