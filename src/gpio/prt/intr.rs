#[doc = "Register `INTR` reader"]
pub struct R(crate::R<INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR` writer"]
pub struct W(crate::W<INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_SPEC>;
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
impl From<crate::W<INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA0` reader - Interrupt pending on IO pad 0. Firmware writes 1 to clear the interrupt."]
pub type DATA0_R = crate::BitReader<bool>;
#[doc = "Field `DATA0` writer - Interrupt pending on IO pad 0. Firmware writes 1 to clear the interrupt."]
pub type DATA0_W<'a> = crate::BitWriter<'a, u32, INTR_SPEC, bool, 0>;
#[doc = "Field `DATA1` reader - Interrupt pending on IO pad 1. Firmware writes 1 to clear the interrupt."]
pub type DATA1_R = crate::BitReader<bool>;
#[doc = "Field `DATA1` writer - Interrupt pending on IO pad 1. Firmware writes 1 to clear the interrupt."]
pub type DATA1_W<'a> = crate::BitWriter<'a, u32, INTR_SPEC, bool, 1>;
#[doc = "Field `DATA2` reader - Interrupt pending on IO pad 2. Firmware writes 1 to clear the interrupt."]
pub type DATA2_R = crate::BitReader<bool>;
#[doc = "Field `DATA2` writer - Interrupt pending on IO pad 2. Firmware writes 1 to clear the interrupt."]
pub type DATA2_W<'a> = crate::BitWriter<'a, u32, INTR_SPEC, bool, 2>;
#[doc = "Field `DATA3` reader - Interrupt pending on IO pad 3. Firmware writes 1 to clear the interrupt."]
pub type DATA3_R = crate::BitReader<bool>;
#[doc = "Field `DATA3` writer - Interrupt pending on IO pad 3. Firmware writes 1 to clear the interrupt."]
pub type DATA3_W<'a> = crate::BitWriter<'a, u32, INTR_SPEC, bool, 3>;
#[doc = "Field `DATA4` reader - Interrupt pending on IO pad 4. Firmware writes 1 to clear the interrupt."]
pub type DATA4_R = crate::BitReader<bool>;
#[doc = "Field `DATA4` writer - Interrupt pending on IO pad 4. Firmware writes 1 to clear the interrupt."]
pub type DATA4_W<'a> = crate::BitWriter<'a, u32, INTR_SPEC, bool, 4>;
#[doc = "Field `DATA5` reader - Interrupt pending on IO pad 5. Firmware writes 1 to clear the interrupt."]
pub type DATA5_R = crate::BitReader<bool>;
#[doc = "Field `DATA5` writer - Interrupt pending on IO pad 5. Firmware writes 1 to clear the interrupt."]
pub type DATA5_W<'a> = crate::BitWriter<'a, u32, INTR_SPEC, bool, 5>;
#[doc = "Field `DATA6` reader - Interrupt pending on IO pad 6. Firmware writes 1 to clear the interrupt."]
pub type DATA6_R = crate::BitReader<bool>;
#[doc = "Field `DATA6` writer - Interrupt pending on IO pad 6. Firmware writes 1 to clear the interrupt."]
pub type DATA6_W<'a> = crate::BitWriter<'a, u32, INTR_SPEC, bool, 6>;
#[doc = "Field `DATA7` reader - Interrupt pending on IO pad 7. Firmware writes 1 to clear the interrupt."]
pub type DATA7_R = crate::BitReader<bool>;
#[doc = "Field `DATA7` writer - Interrupt pending on IO pad 7. Firmware writes 1 to clear the interrupt."]
pub type DATA7_W<'a> = crate::BitWriter<'a, u32, INTR_SPEC, bool, 7>;
#[doc = "Field `FLT_DATA` reader - Deglitched interrupt pending (selected by FLT_SELECT)."]
pub type FLT_DATA_R = crate::BitReader<bool>;
#[doc = "Field `FLT_DATA` writer - Deglitched interrupt pending (selected by FLT_SELECT)."]
pub type FLT_DATA_W<'a> = crate::BitWriter<'a, u32, INTR_SPEC, bool, 8>;
#[doc = "Field `PS_DATA0` reader - `"]
pub type PS_DATA0_R = crate::BitReader<bool>;
#[doc = "Field `PS_DATA1` reader - N/A"]
pub type PS_DATA1_R = crate::BitReader<bool>;
#[doc = "Field `PS_DATA2` reader - N/A"]
pub type PS_DATA2_R = crate::BitReader<bool>;
#[doc = "Field `PS_DATA3` reader - N/A"]
pub type PS_DATA3_R = crate::BitReader<bool>;
#[doc = "Field `PS_DATA4` reader - N/A"]
pub type PS_DATA4_R = crate::BitReader<bool>;
#[doc = "Field `PS_DATA5` reader - N/A"]
pub type PS_DATA5_R = crate::BitReader<bool>;
#[doc = "Field `PS_DATA6` reader - N/A"]
pub type PS_DATA6_R = crate::BitReader<bool>;
#[doc = "Field `PS_DATA7` reader - N/A"]
pub type PS_DATA7_R = crate::BitReader<bool>;
#[doc = "Field `PS_FLT_DATA` reader - This is a duplicate of the contents of the PS register, provided here to allow reading of both pin state and interrupt state of the port in a single read operation."]
pub type PS_FLT_DATA_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Interrupt pending on IO pad 0. Firmware writes 1 to clear the interrupt."]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt pending on IO pad 1. Firmware writes 1 to clear the interrupt."]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt pending on IO pad 2. Firmware writes 1 to clear the interrupt."]
    #[inline(always)]
    pub fn data2(&self) -> DATA2_R {
        DATA2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt pending on IO pad 3. Firmware writes 1 to clear the interrupt."]
    #[inline(always)]
    pub fn data3(&self) -> DATA3_R {
        DATA3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt pending on IO pad 4. Firmware writes 1 to clear the interrupt."]
    #[inline(always)]
    pub fn data4(&self) -> DATA4_R {
        DATA4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt pending on IO pad 5. Firmware writes 1 to clear the interrupt."]
    #[inline(always)]
    pub fn data5(&self) -> DATA5_R {
        DATA5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt pending on IO pad 6. Firmware writes 1 to clear the interrupt."]
    #[inline(always)]
    pub fn data6(&self) -> DATA6_R {
        DATA6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt pending on IO pad 7. Firmware writes 1 to clear the interrupt."]
    #[inline(always)]
    pub fn data7(&self) -> DATA7_R {
        DATA7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Deglitched interrupt pending (selected by FLT_SELECT)."]
    #[inline(always)]
    pub fn flt_data(&self) -> FLT_DATA_R {
        FLT_DATA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - `"]
    #[inline(always)]
    pub fn ps_data0(&self) -> PS_DATA0_R {
        PS_DATA0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - N/A"]
    #[inline(always)]
    pub fn ps_data1(&self) -> PS_DATA1_R {
        PS_DATA1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - N/A"]
    #[inline(always)]
    pub fn ps_data2(&self) -> PS_DATA2_R {
        PS_DATA2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - N/A"]
    #[inline(always)]
    pub fn ps_data3(&self) -> PS_DATA3_R {
        PS_DATA3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - N/A"]
    #[inline(always)]
    pub fn ps_data4(&self) -> PS_DATA4_R {
        PS_DATA4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - N/A"]
    #[inline(always)]
    pub fn ps_data5(&self) -> PS_DATA5_R {
        PS_DATA5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - N/A"]
    #[inline(always)]
    pub fn ps_data6(&self) -> PS_DATA6_R {
        PS_DATA6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - N/A"]
    #[inline(always)]
    pub fn ps_data7(&self) -> PS_DATA7_R {
        PS_DATA7_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - This is a duplicate of the contents of the PS register, provided here to allow reading of both pin state and interrupt state of the port in a single read operation."]
    #[inline(always)]
    pub fn ps_flt_data(&self) -> PS_FLT_DATA_R {
        PS_FLT_DATA_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt pending on IO pad 0. Firmware writes 1 to clear the interrupt."]
    #[inline(always)]
    pub fn data0(&mut self) -> DATA0_W {
        DATA0_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt pending on IO pad 1. Firmware writes 1 to clear the interrupt."]
    #[inline(always)]
    pub fn data1(&mut self) -> DATA1_W {
        DATA1_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt pending on IO pad 2. Firmware writes 1 to clear the interrupt."]
    #[inline(always)]
    pub fn data2(&mut self) -> DATA2_W {
        DATA2_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt pending on IO pad 3. Firmware writes 1 to clear the interrupt."]
    #[inline(always)]
    pub fn data3(&mut self) -> DATA3_W {
        DATA3_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt pending on IO pad 4. Firmware writes 1 to clear the interrupt."]
    #[inline(always)]
    pub fn data4(&mut self) -> DATA4_W {
        DATA4_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt pending on IO pad 5. Firmware writes 1 to clear the interrupt."]
    #[inline(always)]
    pub fn data5(&mut self) -> DATA5_W {
        DATA5_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt pending on IO pad 6. Firmware writes 1 to clear the interrupt."]
    #[inline(always)]
    pub fn data6(&mut self) -> DATA6_W {
        DATA6_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt pending on IO pad 7. Firmware writes 1 to clear the interrupt."]
    #[inline(always)]
    pub fn data7(&mut self) -> DATA7_W {
        DATA7_W::new(self)
    }
    #[doc = "Bit 8 - Deglitched interrupt pending (selected by FLT_SELECT)."]
    #[inline(always)]
    pub fn flt_data(&mut self) -> FLT_DATA_W {
        FLT_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](index.html) module"]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr::R](R) reader structure"]
impl crate::Readable for INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr::W](W) writer structure"]
impl crate::Writable for INTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
