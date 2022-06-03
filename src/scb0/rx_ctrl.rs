#[doc = "Register `RX_CTRL` reader"]
pub struct R(crate::R<RX_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_CTRL` writer"]
pub struct W(crate::W<RX_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_CTRL_SPEC>;
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
impl From<crate::W<RX_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_WIDTH` reader - Dataframe width. DATA_WIDTH + 1 is the expected amount of bits in received data frame. This number does not include start, parity and stop bits. For UART mode, the valid range is \\[3, 8\\]. For SPI, the valid range is \\[3, 15\\]. For I2C the only valid value is 7. In EZ mode (for both SPI and I2C), the only valid value is 7."]
pub type DATA_WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_WIDTH` writer - Dataframe width. DATA_WIDTH + 1 is the expected amount of bits in received data frame. This number does not include start, parity and stop bits. For UART mode, the valid range is \\[3, 8\\]. For SPI, the valid range is \\[3, 15\\]. For I2C the only valid value is 7. In EZ mode (for both SPI and I2C), the only valid value is 7."]
pub type DATA_WIDTH_W<'a> = crate::FieldWriter<'a, u32, RX_CTRL_SPEC, u8, u8, 4, 0>;
#[doc = "Field `MSB_FIRST` reader - Least significant bit first ('0') or most significant bit first ('1'). For I2C, this field should be '1'."]
pub type MSB_FIRST_R = crate::BitReader<bool>;
#[doc = "Field `MSB_FIRST` writer - Least significant bit first ('0') or most significant bit first ('1'). For I2C, this field should be '1'."]
pub type MSB_FIRST_W<'a> = crate::BitWriter<'a, u32, RX_CTRL_SPEC, bool, 8>;
#[doc = "Field `MEDIAN` reader - Median filter. When '1', a digital 3 taps median filter is performed on input interface lines. This filter should reduce the susceptability to errors. However, its requires higher oversampling values. For UART IrDA submode, this field should always be '1'."]
pub type MEDIAN_R = crate::BitReader<bool>;
#[doc = "Field `MEDIAN` writer - Median filter. When '1', a digital 3 taps median filter is performed on input interface lines. This filter should reduce the susceptability to errors. However, its requires higher oversampling values. For UART IrDA submode, this field should always be '1'."]
pub type MEDIAN_W<'a> = crate::BitWriter<'a, u32, RX_CTRL_SPEC, bool, 9>;
impl R {
    #[doc = "Bits 0:3 - Dataframe width. DATA_WIDTH + 1 is the expected amount of bits in received data frame. This number does not include start, parity and stop bits. For UART mode, the valid range is \\[3, 8\\]. For SPI, the valid range is \\[3, 15\\]. For I2C the only valid value is 7. In EZ mode (for both SPI and I2C), the only valid value is 7."]
    #[inline(always)]
    pub fn data_width(&self) -> DATA_WIDTH_R {
        DATA_WIDTH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Least significant bit first ('0') or most significant bit first ('1'). For I2C, this field should be '1'."]
    #[inline(always)]
    pub fn msb_first(&self) -> MSB_FIRST_R {
        MSB_FIRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Median filter. When '1', a digital 3 taps median filter is performed on input interface lines. This filter should reduce the susceptability to errors. However, its requires higher oversampling values. For UART IrDA submode, this field should always be '1'."]
    #[inline(always)]
    pub fn median(&self) -> MEDIAN_R {
        MEDIAN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Dataframe width. DATA_WIDTH + 1 is the expected amount of bits in received data frame. This number does not include start, parity and stop bits. For UART mode, the valid range is \\[3, 8\\]. For SPI, the valid range is \\[3, 15\\]. For I2C the only valid value is 7. In EZ mode (for both SPI and I2C), the only valid value is 7."]
    #[inline(always)]
    pub fn data_width(&mut self) -> DATA_WIDTH_W {
        DATA_WIDTH_W::new(self)
    }
    #[doc = "Bit 8 - Least significant bit first ('0') or most significant bit first ('1'). For I2C, this field should be '1'."]
    #[inline(always)]
    pub fn msb_first(&mut self) -> MSB_FIRST_W {
        MSB_FIRST_W::new(self)
    }
    #[doc = "Bit 9 - Median filter. When '1', a digital 3 taps median filter is performed on input interface lines. This filter should reduce the susceptability to errors. However, its requires higher oversampling values. For UART IrDA submode, this field should always be '1'."]
    #[inline(always)]
    pub fn median(&mut self) -> MEDIAN_W {
        MEDIAN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receiver control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_ctrl](index.html) module"]
pub struct RX_CTRL_SPEC;
impl crate::RegisterSpec for RX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_ctrl::R](R) reader structure"]
impl crate::Readable for RX_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_ctrl::W](W) writer structure"]
impl crate::Writable for RX_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RX_CTRL to value 0x0107"]
impl crate::Resettable for RX_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0107
    }
}
