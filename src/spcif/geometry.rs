#[doc = "Register `GEOMETRY` reader"]
pub struct R(crate::R<GEOMETRY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GEOMETRY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GEOMETRY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GEOMETRY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GEOMETRY` writer"]
pub struct W(crate::W<GEOMETRY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GEOMETRY_SPEC>;
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
impl From<crate::W<GEOMETRY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GEOMETRY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH` reader - Regular flash capacity in 256 Byte multiples (chip dependent). If multiple flash macros are present, this field provides the flash capacity of all flash macros together: '0': 256 Bytes. '1': 2*256 Bytes. ... '16383': 16384*256 Bytes."]
pub type FLASH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SFLASH` reader - Supervisory flash capacity in 256 Byte multiples (chip dependent). If multiple flash macros are present, this field provides the supervisory flash capacity of all flash macros together: '0': 256 Bytes. '1': 2*256 Bytes. ... '63': 64*256 Bytes."]
pub type SFLASH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NUM_FLASH` reader - Number of flash macros (chip dependent): '0': 1 flash macro '1': 2 flash macros '2': 3 flash macros '3': 4 flash macros"]
pub type NUM_FLASH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLASH_ROW` reader - Page size in 64 Byte multiples (chip dependent): '0': 64 byte '1': 128 byte '2': 192 byte '3': 256 byte The page size is used to detemine the number of Bytes in a page for Flash page based operations (e.g. PGM_PAGE). Note: the field name FLASH_ROW is misleading, as this field specifies the number of Bytes in a page, rather than the number of Bytes in a row. In a single plane flash macro architecture, a page consists of a single row. However, in a multi plane flash macro architecture, a page consists of multiple rows from different planes."]
pub type FLASH_ROW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NVL` reader - NVLatch size in Byte multiples (chip dependent): '0': 0 Bytes '1': 1 Byte ... '127': 127 Bytes"]
pub type NVL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DE_CPD_LP` reader - 0': SRAM busy wait loop has not been copied. '1': Busy wait loop has been written into SRAM."]
pub type DE_CPD_LP_R = crate::BitReader<bool>;
#[doc = "Field `DE_CPD_LP` writer - 0': SRAM busy wait loop has not been copied. '1': Busy wait loop has been written into SRAM."]
pub type DE_CPD_LP_W<'a> = crate::BitWriter<'a, u32, GEOMETRY_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:13 - Regular flash capacity in 256 Byte multiples (chip dependent). If multiple flash macros are present, this field provides the flash capacity of all flash macros together: '0': 256 Bytes. '1': 2*256 Bytes. ... '16383': 16384*256 Bytes."]
    #[inline(always)]
    pub fn flash(&self) -> FLASH_R {
        FLASH_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:19 - Supervisory flash capacity in 256 Byte multiples (chip dependent). If multiple flash macros are present, this field provides the supervisory flash capacity of all flash macros together: '0': 256 Bytes. '1': 2*256 Bytes. ... '63': 64*256 Bytes."]
    #[inline(always)]
    pub fn sflash(&self) -> SFLASH_R {
        SFLASH_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bits 20:21 - Number of flash macros (chip dependent): '0': 1 flash macro '1': 2 flash macros '2': 3 flash macros '3': 4 flash macros"]
    #[inline(always)]
    pub fn num_flash(&self) -> NUM_FLASH_R {
        NUM_FLASH_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Page size in 64 Byte multiples (chip dependent): '0': 64 byte '1': 128 byte '2': 192 byte '3': 256 byte The page size is used to detemine the number of Bytes in a page for Flash page based operations (e.g. PGM_PAGE). Note: the field name FLASH_ROW is misleading, as this field specifies the number of Bytes in a page, rather than the number of Bytes in a row. In a single plane flash macro architecture, a page consists of a single row. However, in a multi plane flash macro architecture, a page consists of multiple rows from different planes."]
    #[inline(always)]
    pub fn flash_row(&self) -> FLASH_ROW_R {
        FLASH_ROW_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:30 - NVLatch size in Byte multiples (chip dependent): '0': 0 Bytes '1': 1 Byte ... '127': 127 Bytes"]
    #[inline(always)]
    pub fn nvl(&self) -> NVL_R {
        NVL_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - 0': SRAM busy wait loop has not been copied. '1': Busy wait loop has been written into SRAM."]
    #[inline(always)]
    pub fn de_cpd_lp(&self) -> DE_CPD_LP_R {
        DE_CPD_LP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 0': SRAM busy wait loop has not been copied. '1': Busy wait loop has been written into SRAM."]
    #[inline(always)]
    pub fn de_cpd_lp(&mut self) -> DE_CPD_LP_W {
        DE_CPD_LP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash/NVL geometry information\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [geometry](index.html) module"]
pub struct GEOMETRY_SPEC;
impl crate::RegisterSpec for GEOMETRY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [geometry::R](R) reader structure"]
impl crate::Readable for GEOMETRY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [geometry::W](W) writer structure"]
impl crate::Writable for GEOMETRY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GEOMETRY to value 0"]
impl crate::Resettable for GEOMETRY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
