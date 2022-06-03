#[doc = "Register `PS` reader"]
pub struct R(crate::R<PS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA0` reader - IO pad 0 state: 1: Logic high, if the pin voltage is above the input buffer threshold, logic high. 0: Logic low, if the pin voltage is below that threshold, logic low. If the drive mode for the pin is set to high Z Analog, the pin state will read 0 independent of the voltage on the pin."]
pub type DATA0_R = crate::BitReader<bool>;
#[doc = "Field `DATA1` reader - IO pad 1 state."]
pub type DATA1_R = crate::BitReader<bool>;
#[doc = "Field `DATA2` reader - IO pad 2 state."]
pub type DATA2_R = crate::BitReader<bool>;
#[doc = "Field `DATA3` reader - IO pad 3 state."]
pub type DATA3_R = crate::BitReader<bool>;
#[doc = "Field `DATA4` reader - IO pad 4 state."]
pub type DATA4_R = crate::BitReader<bool>;
#[doc = "Field `DATA5` reader - IO pad 5 state."]
pub type DATA5_R = crate::BitReader<bool>;
#[doc = "Field `DATA6` reader - IO pad 6 state."]
pub type DATA6_R = crate::BitReader<bool>;
#[doc = "Field `DATA7` reader - IO pad 7 state."]
pub type DATA7_R = crate::BitReader<bool>;
#[doc = "Field `FLT_DATA` reader - Reads of this register return the logical state of the filtered pin."]
pub type FLT_DATA_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - IO pad 0 state: 1: Logic high, if the pin voltage is above the input buffer threshold, logic high. 0: Logic low, if the pin voltage is below that threshold, logic low. If the drive mode for the pin is set to high Z Analog, the pin state will read 0 independent of the voltage on the pin."]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO pad 1 state."]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO pad 2 state."]
    #[inline(always)]
    pub fn data2(&self) -> DATA2_R {
        DATA2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO pad 3 state."]
    #[inline(always)]
    pub fn data3(&self) -> DATA3_R {
        DATA3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO pad 4 state."]
    #[inline(always)]
    pub fn data4(&self) -> DATA4_R {
        DATA4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO pad 5 state."]
    #[inline(always)]
    pub fn data5(&self) -> DATA5_R {
        DATA5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO pad 6 state."]
    #[inline(always)]
    pub fn data6(&self) -> DATA6_R {
        DATA6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IO pad 7 state."]
    #[inline(always)]
    pub fn data7(&self) -> DATA7_R {
        DATA7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reads of this register return the logical state of the filtered pin."]
    #[inline(always)]
    pub fn flt_data(&self) -> FLT_DATA_R {
        FLT_DATA_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Port IO pad state register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ps](index.html) module"]
pub struct PS_SPEC;
impl crate::RegisterSpec for PS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ps::R](R) reader structure"]
impl crate::Readable for PS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PS to value 0"]
impl crate::Resettable for PS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
