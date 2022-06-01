#[doc = "Register `Cy_TX_CTRL` reader"]
pub struct R(crate::R<CY_TX_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CY_TX_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CY_TX_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CY_TX_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Cy_TX_CTRL` writer"]
pub struct W(crate::W<CY_TX_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CY_TX_CTRL_SPEC>;
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
impl From<crate::W<CY_TX_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CY_TX_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_WIDTH` reader - Data frame width. DATA_WIDTH + 1 is the amount of bits in a transmitted data frame."]
pub type DATA_WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_WIDTH` writer - Data frame width. DATA_WIDTH + 1 is the amount of bits in a transmitted data frame."]
pub type DATA_WIDTH_W<'a> = crate::FieldWriter<'a, u32, CY_TX_CTRL_SPEC, u8, u8, 4, 0>;
#[doc = "Field `MSB_FIRST` reader - Least significant bit first ('0') or most significant bit first ('1'). For I2C, this field should be '1'."]
pub type MSB_FIRST_R = crate::BitReader<bool>;
#[doc = "Field `MSB_FIRST` writer - Least significant bit first ('0') or most significant bit first ('1'). For I2C, this field should be '1'."]
pub type MSB_FIRST_W<'a> = crate::BitWriter<'a, u32, CY_TX_CTRL_SPEC, bool, 8>;
impl R {
    #[doc = "Bits 0:3 - Data frame width. DATA_WIDTH + 1 is the amount of bits in a transmitted data frame."]
    #[inline(always)]
    pub fn data_width(&self) -> DATA_WIDTH_R {
        DATA_WIDTH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Least significant bit first ('0') or most significant bit first ('1'). For I2C, this field should be '1'."]
    #[inline(always)]
    pub fn msb_first(&self) -> MSB_FIRST_R {
        MSB_FIRST_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data frame width. DATA_WIDTH + 1 is the amount of bits in a transmitted data frame."]
    #[inline(always)]
    pub fn data_width(&mut self) -> DATA_WIDTH_W {
        DATA_WIDTH_W::new(self)
    }
    #[doc = "Bit 8 - Least significant bit first ('0') or most significant bit first ('1'). For I2C, this field should be '1'."]
    #[inline(always)]
    pub fn msb_first(&mut self) -> MSB_FIRST_W {
        MSB_FIRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmitter control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cy_tx_ctrl](index.html) module"]
pub struct CY_TX_CTRL_SPEC;
impl crate::RegisterSpec for CY_TX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cy_tx_ctrl::R](R) reader structure"]
impl crate::Readable for CY_TX_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cy_tx_ctrl::W](W) writer structure"]
impl crate::Writable for CY_TX_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Cy_TX_CTRL to value 0"]
impl crate::Resettable for CY_TX_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
