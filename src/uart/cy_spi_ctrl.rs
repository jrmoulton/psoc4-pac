#[doc = "Register `Cy_SPI_CTRL` reader"]
pub struct R(crate::R<CY_SPI_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CY_SPI_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CY_SPI_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CY_SPI_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Cy_SPI_CTRL` writer"]
pub struct W(crate::W<CY_SPI_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CY_SPI_CTRL_SPEC>;
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
impl From<crate::W<CY_SPI_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CY_SPI_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONTINUOUS` reader - Continuous SPI data transfers enabled ('1') or not ('0')."]
pub type CONTINUOUS_R = crate::BitReader<bool>;
#[doc = "Field `CONTINUOUS` writer - Continuous SPI data transfers enabled ('1') or not ('0')."]
pub type CONTINUOUS_W<'a> = crate::BitWriter<'a, u32, CY_SPI_CTRL_SPEC, bool, 0>;
#[doc = "Field `SELECT_PRECEDE` reader - Only used in SPI Texas Instruments' submode. When '1', the data frame start indication is a pulse on the SELECT line that precedes the transfer of the first data frame bit. When '0', the data frame start indication is a pulse on the SELECT line that coincides with the transfer of the first data frame bit."]
pub type SELECT_PRECEDE_R = crate::BitReader<bool>;
#[doc = "Field `SELECT_PRECEDE` writer - Only used in SPI Texas Instruments' submode. When '1', the data frame start indication is a pulse on the SELECT line that precedes the transfer of the first data frame bit. When '0', the data frame start indication is a pulse on the SELECT line that coincides with the transfer of the first data frame bit."]
pub type SELECT_PRECEDE_W<'a> = crate::BitWriter<'a, u32, CY_SPI_CTRL_SPEC, bool, 1>;
#[doc = "Field `CPHA` reader - Only applicable in SPI Motorola submode. Indicates the clock phase."]
pub type CPHA_R = crate::BitReader<bool>;
#[doc = "Field `CPHA` writer - Only applicable in SPI Motorola submode. Indicates the clock phase."]
pub type CPHA_W<'a> = crate::BitWriter<'a, u32, CY_SPI_CTRL_SPEC, bool, 2>;
#[doc = "Field `CPOL` reader - Only applicable in SPI Motorola submode. Indicates the clock polarity."]
pub type CPOL_R = crate::BitReader<bool>;
#[doc = "Field `CPOL` writer - Only applicable in SPI Motorola submode. Indicates the clock polarity."]
pub type CPOL_W<'a> = crate::BitWriter<'a, u32, CY_SPI_CTRL_SPEC, bool, 3>;
#[doc = "Field `LATE_MISO_SAMPLE` reader - Only applicable in master mode. Changes the SCLK edge on which MISO is captured."]
pub type LATE_MISO_SAMPLE_R = crate::BitReader<bool>;
#[doc = "Field `LATE_MISO_SAMPLE` writer - Only applicable in master mode. Changes the SCLK edge on which MISO is captured."]
pub type LATE_MISO_SAMPLE_W<'a> = crate::BitWriter<'a, u32, CY_SPI_CTRL_SPEC, bool, 4>;
#[doc = "Field `SCLK_CONTINUOUS` reader - Enables SCLK generation continuiusly by master regars data is avaiable in TX FIFO or not."]
pub type SCLK_CONTINUOUS_R = crate::BitReader<bool>;
#[doc = "Field `SCLK_CONTINUOUS` writer - Enables SCLK generation continuiusly by master regars data is avaiable in TX FIFO or not."]
pub type SCLK_CONTINUOUS_W<'a> = crate::BitWriter<'a, u32, CY_SPI_CTRL_SPEC, bool, 5>;
#[doc = "Field `SSEL_POLARITY0` reader - Slave select polarity of SS0."]
pub type SSEL_POLARITY0_R = crate::BitReader<bool>;
#[doc = "Field `SSEL_POLARITY0` writer - Slave select polarity of SS0."]
pub type SSEL_POLARITY0_W<'a> = crate::BitWriter<'a, u32, CY_SPI_CTRL_SPEC, bool, 8>;
#[doc = "Field `SSEL_POLARITY1` reader - Slave select polarity of SS1."]
pub type SSEL_POLARITY1_R = crate::BitReader<bool>;
#[doc = "Field `SSEL_POLARITY1` writer - Slave select polarity of SS1."]
pub type SSEL_POLARITY1_W<'a> = crate::BitWriter<'a, u32, CY_SPI_CTRL_SPEC, bool, 9>;
#[doc = "Field `SSEL_POLARITY2` reader - Slave select polarity of SS2."]
pub type SSEL_POLARITY2_R = crate::BitReader<bool>;
#[doc = "Field `SSEL_POLARITY2` writer - Slave select polarity of SS2."]
pub type SSEL_POLARITY2_W<'a> = crate::BitWriter<'a, u32, CY_SPI_CTRL_SPEC, bool, 10>;
#[doc = "Field `SSEL_POLARITY3` reader - Slave select polarity of SS3."]
pub type SSEL_POLARITY3_R = crate::BitReader<bool>;
#[doc = "Field `SSEL_POLARITY3` writer - Slave select polarity of SS3."]
pub type SSEL_POLARITY3_W<'a> = crate::BitWriter<'a, u32, CY_SPI_CTRL_SPEC, bool, 11>;
#[doc = "Field `LOOPBACK` reader - Local loopback control."]
pub type LOOPBACK_R = crate::BitReader<bool>;
#[doc = "Field `LOOPBACK` writer - Local loopback control."]
pub type LOOPBACK_W<'a> = crate::BitWriter<'a, u32, CY_SPI_CTRL_SPEC, bool, 16>;
#[doc = "Field `MODE` reader - Submode of SPI operation: Motorola = 0, Texas Instruments = 1, National Semiconducturs = 2."]
pub type MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE` writer - Submode of SPI operation: Motorola = 0, Texas Instruments = 1, National Semiconducturs = 2."]
pub type MODE_W<'a> = crate::FieldWriter<'a, u32, CY_SPI_CTRL_SPEC, u8, u8, 2, 24>;
#[doc = "Field `SLAVE_SELECT` reader - Selects one of the four SPI slave select signals: SS0 = 0, SS1 = 1 , SS2 = 2, SS3 = 3."]
pub type SLAVE_SELECT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLAVE_SELECT` writer - Selects one of the four SPI slave select signals: SS0 = 0, SS1 = 1 , SS2 = 2, SS3 = 3."]
pub type SLAVE_SELECT_W<'a> = crate::FieldWriter<'a, u32, CY_SPI_CTRL_SPEC, u8, u8, 2, 26>;
#[doc = "Field `MASTER_MODE` reader - Master ('1') or slave ('0') mode."]
pub type MASTER_MODE_R = crate::BitReader<bool>;
#[doc = "Field `MASTER_MODE` writer - Master ('1') or slave ('0') mode."]
pub type MASTER_MODE_W<'a> = crate::BitWriter<'a, u32, CY_SPI_CTRL_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - Continuous SPI data transfers enabled ('1') or not ('0')."]
    #[inline(always)]
    pub fn continuous(&self) -> CONTINUOUS_R {
        CONTINUOUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Only used in SPI Texas Instruments' submode. When '1', the data frame start indication is a pulse on the SELECT line that precedes the transfer of the first data frame bit. When '0', the data frame start indication is a pulse on the SELECT line that coincides with the transfer of the first data frame bit."]
    #[inline(always)]
    pub fn select_precede(&self) -> SELECT_PRECEDE_R {
        SELECT_PRECEDE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Only applicable in SPI Motorola submode. Indicates the clock phase."]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Only applicable in SPI Motorola submode. Indicates the clock polarity."]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Only applicable in master mode. Changes the SCLK edge on which MISO is captured."]
    #[inline(always)]
    pub fn late_miso_sample(&self) -> LATE_MISO_SAMPLE_R {
        LATE_MISO_SAMPLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enables SCLK generation continuiusly by master regars data is avaiable in TX FIFO or not."]
    #[inline(always)]
    pub fn sclk_continuous(&self) -> SCLK_CONTINUOUS_R {
        SCLK_CONTINUOUS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Slave select polarity of SS0."]
    #[inline(always)]
    pub fn ssel_polarity0(&self) -> SSEL_POLARITY0_R {
        SSEL_POLARITY0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Slave select polarity of SS1."]
    #[inline(always)]
    pub fn ssel_polarity1(&self) -> SSEL_POLARITY1_R {
        SSEL_POLARITY1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Slave select polarity of SS2."]
    #[inline(always)]
    pub fn ssel_polarity2(&self) -> SSEL_POLARITY2_R {
        SSEL_POLARITY2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Slave select polarity of SS3."]
    #[inline(always)]
    pub fn ssel_polarity3(&self) -> SSEL_POLARITY3_R {
        SSEL_POLARITY3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Local loopback control."]
    #[inline(always)]
    pub fn loopback(&self) -> LOOPBACK_R {
        LOOPBACK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Submode of SPI operation: Motorola = 0, Texas Instruments = 1, National Semiconducturs = 2."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Selects one of the four SPI slave select signals: SS0 = 0, SS1 = 1 , SS2 = 2, SS3 = 3."]
    #[inline(always)]
    pub fn slave_select(&self) -> SLAVE_SELECT_R {
        SLAVE_SELECT_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 31 - Master ('1') or slave ('0') mode."]
    #[inline(always)]
    pub fn master_mode(&self) -> MASTER_MODE_R {
        MASTER_MODE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Continuous SPI data transfers enabled ('1') or not ('0')."]
    #[inline(always)]
    pub fn continuous(&mut self) -> CONTINUOUS_W {
        CONTINUOUS_W::new(self)
    }
    #[doc = "Bit 1 - Only used in SPI Texas Instruments' submode. When '1', the data frame start indication is a pulse on the SELECT line that precedes the transfer of the first data frame bit. When '0', the data frame start indication is a pulse on the SELECT line that coincides with the transfer of the first data frame bit."]
    #[inline(always)]
    pub fn select_precede(&mut self) -> SELECT_PRECEDE_W {
        SELECT_PRECEDE_W::new(self)
    }
    #[doc = "Bit 2 - Only applicable in SPI Motorola submode. Indicates the clock phase."]
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W {
        CPHA_W::new(self)
    }
    #[doc = "Bit 3 - Only applicable in SPI Motorola submode. Indicates the clock polarity."]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W {
        CPOL_W::new(self)
    }
    #[doc = "Bit 4 - Only applicable in master mode. Changes the SCLK edge on which MISO is captured."]
    #[inline(always)]
    pub fn late_miso_sample(&mut self) -> LATE_MISO_SAMPLE_W {
        LATE_MISO_SAMPLE_W::new(self)
    }
    #[doc = "Bit 5 - Enables SCLK generation continuiusly by master regars data is avaiable in TX FIFO or not."]
    #[inline(always)]
    pub fn sclk_continuous(&mut self) -> SCLK_CONTINUOUS_W {
        SCLK_CONTINUOUS_W::new(self)
    }
    #[doc = "Bit 8 - Slave select polarity of SS0."]
    #[inline(always)]
    pub fn ssel_polarity0(&mut self) -> SSEL_POLARITY0_W {
        SSEL_POLARITY0_W::new(self)
    }
    #[doc = "Bit 9 - Slave select polarity of SS1."]
    #[inline(always)]
    pub fn ssel_polarity1(&mut self) -> SSEL_POLARITY1_W {
        SSEL_POLARITY1_W::new(self)
    }
    #[doc = "Bit 10 - Slave select polarity of SS2."]
    #[inline(always)]
    pub fn ssel_polarity2(&mut self) -> SSEL_POLARITY2_W {
        SSEL_POLARITY2_W::new(self)
    }
    #[doc = "Bit 11 - Slave select polarity of SS3."]
    #[inline(always)]
    pub fn ssel_polarity3(&mut self) -> SSEL_POLARITY3_W {
        SSEL_POLARITY3_W::new(self)
    }
    #[doc = "Bit 16 - Local loopback control."]
    #[inline(always)]
    pub fn loopback(&mut self) -> LOOPBACK_W {
        LOOPBACK_W::new(self)
    }
    #[doc = "Bits 24:25 - Submode of SPI operation: Motorola = 0, Texas Instruments = 1, National Semiconducturs = 2."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W::new(self)
    }
    #[doc = "Bits 26:27 - Selects one of the four SPI slave select signals: SS0 = 0, SS1 = 1 , SS2 = 2, SS3 = 3."]
    #[inline(always)]
    pub fn slave_select(&mut self) -> SLAVE_SELECT_W {
        SLAVE_SELECT_W::new(self)
    }
    #[doc = "Bit 31 - Master ('1') or slave ('0') mode."]
    #[inline(always)]
    pub fn master_mode(&mut self) -> MASTER_MODE_W {
        MASTER_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cy_spi_ctrl](index.html) module"]
pub struct CY_SPI_CTRL_SPEC;
impl crate::RegisterSpec for CY_SPI_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cy_spi_ctrl::R](R) reader structure"]
impl crate::Readable for CY_SPI_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cy_spi_ctrl::W](W) writer structure"]
impl crate::Writable for CY_SPI_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Cy_SPI_CTRL to value 0"]
impl crate::Resettable for CY_SPI_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
