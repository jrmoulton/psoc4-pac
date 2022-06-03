#[doc = "Register `SPI_CTRL` reader"]
pub struct R(crate::R<SPI_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_CTRL` writer"]
pub struct W(crate::W<SPI_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_CTRL_SPEC>;
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
impl From<crate::W<SPI_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONTINUOUS` reader - Continuous SPI data transfers enabled ('1') or not ('0'). This field is used in master mode. In slave mode, both continuous and non-continuous SPI data transfers are supported independent of this field. When continuous transfers are enabled individual data frame transfers are not necessarily seperated by slave deselection (as indicated by the level or pulse on the SELECT line): if the TX FIFO has multiple data frames, data frames are send out without slave deselection. When continuous transfers are not enabled individual data frame transfers are always seperated by slave deselection: independent of the availability of TX FIFO data frames, data frames are send out with slave deselection."]
pub type CONTINUOUS_R = crate::BitReader<bool>;
#[doc = "Field `CONTINUOUS` writer - Continuous SPI data transfers enabled ('1') or not ('0'). This field is used in master mode. In slave mode, both continuous and non-continuous SPI data transfers are supported independent of this field. When continuous transfers are enabled individual data frame transfers are not necessarily seperated by slave deselection (as indicated by the level or pulse on the SELECT line): if the TX FIFO has multiple data frames, data frames are send out without slave deselection. When continuous transfers are not enabled individual data frame transfers are always seperated by slave deselection: independent of the availability of TX FIFO data frames, data frames are send out with slave deselection."]
pub type CONTINUOUS_W<'a> = crate::BitWriter<'a, u32, SPI_CTRL_SPEC, bool, 0>;
#[doc = "Field `SELECT_PRECEDE` reader - Only used in SPI Texas Instruments' submode. When '1', the data frame start indication is a pulse on the SELECT line that precedes the transfer of the first data frame bit. When '0', the data frame start indication is a pulse on the SELECT line that coincides with the transfer of the first data frame bit."]
pub type SELECT_PRECEDE_R = crate::BitReader<bool>;
#[doc = "Field `SELECT_PRECEDE` writer - Only used in SPI Texas Instruments' submode. When '1', the data frame start indication is a pulse on the SELECT line that precedes the transfer of the first data frame bit. When '0', the data frame start indication is a pulse on the SELECT line that coincides with the transfer of the first data frame bit."]
pub type SELECT_PRECEDE_W<'a> = crate::BitWriter<'a, u32, SPI_CTRL_SPEC, bool, 1>;
#[doc = "Field `CPHA` reader - Only applicable in SPI Motorola submode. Indicates the clock phase. This field, together with the CPOL field, indicates when MOSI data is driven and MISO data is captured: - Motorola mode 0. CPOL is '0', CPHA is '0': MOSI is driven on a falling edge of SCLK. MISO is captured on a rising edge of SCLK. - Motorola mode 1. CPOL is '0', CPHA is '1': MOSI is driven on a rising edge of SCLK. MISO is captured on a falling edge of SCLK. - Motorola mode 2. CPOL is '1', CPHA is '0': MOSI is driven on a rising edge of SCLK. MISO is captured on a falling edge of SCLK. - Motorola mode 3. CPOL is '1', CPHA is '1': MOSI is driven on a falling edge of SCLK. MISO is captured on a rising edge of SCLK."]
pub type CPHA_R = crate::BitReader<bool>;
#[doc = "Field `CPHA` writer - Only applicable in SPI Motorola submode. Indicates the clock phase. This field, together with the CPOL field, indicates when MOSI data is driven and MISO data is captured: - Motorola mode 0. CPOL is '0', CPHA is '0': MOSI is driven on a falling edge of SCLK. MISO is captured on a rising edge of SCLK. - Motorola mode 1. CPOL is '0', CPHA is '1': MOSI is driven on a rising edge of SCLK. MISO is captured on a falling edge of SCLK. - Motorola mode 2. CPOL is '1', CPHA is '0': MOSI is driven on a rising edge of SCLK. MISO is captured on a falling edge of SCLK. - Motorola mode 3. CPOL is '1', CPHA is '1': MOSI is driven on a falling edge of SCLK. MISO is captured on a rising edge of SCLK."]
pub type CPHA_W<'a> = crate::BitWriter<'a, u32, SPI_CTRL_SPEC, bool, 2>;
#[doc = "Field `CPOL` reader - Indicates the clock polarity. Only used in SPI Motorola submode. This field, together with the CPHA field, indicates when MOSI data is driven and MISO data is captured: - CPOL is '0': SCLK is '0' when not transmitting data. - CPOL is '1': SCLK is '1' when not transmitting data."]
pub type CPOL_R = crate::BitReader<bool>;
#[doc = "Field `CPOL` writer - Indicates the clock polarity. Only used in SPI Motorola submode. This field, together with the CPHA field, indicates when MOSI data is driven and MISO data is captured: - CPOL is '0': SCLK is '0' when not transmitting data. - CPOL is '1': SCLK is '1' when not transmitting data."]
pub type CPOL_W<'a> = crate::BitWriter<'a, u32, SPI_CTRL_SPEC, bool, 3>;
#[doc = "Field `LATE_MISO_SAMPLE` reader - Changes the SCLK edge on which MISO is captured. Only used in master mode. When '0', the default applies (for Motorola as determined by CPOL and CPHA, for Texas Instruments on the falling edge of SCLK and for National Semiconductors on the rising edge of SCLK). When '1', the alternate clock edge is used (which comes half a SPI SCLK period later). Late sampling addresses the round trip delay associated with transmitting SCLK from the master to the slave and transmitting MISO from the slave to the master."]
pub type LATE_MISO_SAMPLE_R = crate::BitReader<bool>;
#[doc = "Field `LATE_MISO_SAMPLE` writer - Changes the SCLK edge on which MISO is captured. Only used in master mode. When '0', the default applies (for Motorola as determined by CPOL and CPHA, for Texas Instruments on the falling edge of SCLK and for National Semiconductors on the rising edge of SCLK). When '1', the alternate clock edge is used (which comes half a SPI SCLK period later). Late sampling addresses the round trip delay associated with transmitting SCLK from the master to the slave and transmitting MISO from the slave to the master."]
pub type LATE_MISO_SAMPLE_W<'a> = crate::BitWriter<'a, u32, SPI_CTRL_SPEC, bool, 4>;
#[doc = "Field `SCLK_CONTINUOUS` reader - Only applicable in master mode. '0': SCLK is generated, when the SPI master is enabled and data is transmitted. '1': SCLK is generated, when the SPI master is enabled. This mode is useful for slave devices that use SCLK for functional operation other than just SPI functionality."]
pub type SCLK_CONTINUOUS_R = crate::BitReader<bool>;
#[doc = "Field `SCLK_CONTINUOUS` writer - Only applicable in master mode. '0': SCLK is generated, when the SPI master is enabled and data is transmitted. '1': SCLK is generated, when the SPI master is enabled. This mode is useful for slave devices that use SCLK for functional operation other than just SPI functionality."]
pub type SCLK_CONTINUOUS_W<'a> = crate::BitWriter<'a, u32, SPI_CTRL_SPEC, bool, 5>;
#[doc = "Field `SSEL_POLARITY0` reader - Slave select polarity. SSEL_POLARITY0 applies to the outgoing SPI slave select signal 0 (master mode) and to the incoming SPI slave select signal (slave mode). only SPI_SELECT\\[0\\]
is used in slave mode. For Motorola and National Semiconductors submodes: '0': slave select is low/'0' active. '1': slave select is high/'1' active. For Texas Istruments submode: '0': high/'1' active precede/coincide pulse. '1': low/'0' active precede/coincide pulse."]
pub type SSEL_POLARITY0_R = crate::BitReader<bool>;
#[doc = "Field `SSEL_POLARITY0` writer - Slave select polarity. SSEL_POLARITY0 applies to the outgoing SPI slave select signal 0 (master mode) and to the incoming SPI slave select signal (slave mode). only SPI_SELECT\\[0\\]
is used in slave mode. For Motorola and National Semiconductors submodes: '0': slave select is low/'0' active. '1': slave select is high/'1' active. For Texas Istruments submode: '0': high/'1' active precede/coincide pulse. '1': low/'0' active precede/coincide pulse."]
pub type SSEL_POLARITY0_W<'a> = crate::BitWriter<'a, u32, SPI_CTRL_SPEC, bool, 8>;
#[doc = "Field `SSEL_POLARITY1` reader - Slave select polarity. SSEL_POLARITY1 applies to the outgoing SPI slave select signal 1 (master mode)."]
pub type SSEL_POLARITY1_R = crate::BitReader<bool>;
#[doc = "Field `SSEL_POLARITY1` writer - Slave select polarity. SSEL_POLARITY1 applies to the outgoing SPI slave select signal 1 (master mode)."]
pub type SSEL_POLARITY1_W<'a> = crate::BitWriter<'a, u32, SPI_CTRL_SPEC, bool, 9>;
#[doc = "Field `SSEL_POLARITY2` reader - Slave select polarity. SSEL_POLARITY2 applies to the outgoing SPI slave select signal 2 (master mode)."]
pub type SSEL_POLARITY2_R = crate::BitReader<bool>;
#[doc = "Field `SSEL_POLARITY2` writer - Slave select polarity. SSEL_POLARITY2 applies to the outgoing SPI slave select signal 2 (master mode)."]
pub type SSEL_POLARITY2_W<'a> = crate::BitWriter<'a, u32, SPI_CTRL_SPEC, bool, 10>;
#[doc = "Field `SSEL_POLARITY3` reader - Slave select polarity. SSEL_POLARITY3 applies to the outgoing SPI slave select signal 3 (master mode)."]
pub type SSEL_POLARITY3_R = crate::BitReader<bool>;
#[doc = "Field `SSEL_POLARITY3` writer - Slave select polarity. SSEL_POLARITY3 applies to the outgoing SPI slave select signal 3 (master mode)."]
pub type SSEL_POLARITY3_W<'a> = crate::BitWriter<'a, u32, SPI_CTRL_SPEC, bool, 11>;
#[doc = "Field `LOOPBACK` reader - Local loopback control (does NOT affect the information on the pins). Only used in master mode. Not used in National Semiconductors submode. '0': the SPI master MISO line 'spi_miso_in' is connected to the SPI MISO pin. '1': the SPI master MISO line 'spi_miso_in' is connected to the SPI master MOSI line 'spi_mosi_out'. In other words, in loopback mode the SPI master receives on MISO what it transmits on MOSI."]
pub type LOOPBACK_R = crate::BitReader<bool>;
#[doc = "Field `LOOPBACK` writer - Local loopback control (does NOT affect the information on the pins). Only used in master mode. Not used in National Semiconductors submode. '0': the SPI master MISO line 'spi_miso_in' is connected to the SPI MISO pin. '1': the SPI master MISO line 'spi_miso_in' is connected to the SPI master MOSI line 'spi_mosi_out'. In other words, in loopback mode the SPI master receives on MISO what it transmits on MOSI."]
pub type LOOPBACK_W<'a> = crate::BitWriter<'a, u32, SPI_CTRL_SPEC, bool, 16>;
#[doc = "N/A\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: SPI Motorola submode. In master mode, when not transmitting data (SELECT is inactive), SCLK is stable at CPOL. In slave mode, when not selected, SCLK is ignored; i.e. it can be either stable or clocking. In master mode, when there is no data to transmit (TX FIFO is empty), SELECT is inactive."]
    SPI_MOTOROLA = 0,
    #[doc = "1: SPI Texas Instruments submode. In master mode, when not transmitting data, SCLK is stable at '0'. In slave mode, when not selected, SCLK is ignored; i.e. it can be either stable or clocking. In master mode, when there is no data to transmit (TX FIFO is empty), SELECT is inactive; i.e. no pulse is generated."]
    SPI_TI = 1,
    #[doc = "2: SPI National Semiconducturs submode. In master mode, when not transmitting data, SCLK is stable at '0'. In slave mode, when not selected, SCLK is ignored; i.e. it can be either stable or clocking. In master mode, when there is no data to transmit (TX FIFO is empty), SELECT is inactive."]
    SPI_NS = 2,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - N/A"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::SPI_MOTOROLA),
            1 => Some(MODE_A::SPI_TI),
            2 => Some(MODE_A::SPI_NS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SPI_MOTOROLA`"]
    #[inline(always)]
    pub fn is_spi_motorola(&self) -> bool {
        *self == MODE_A::SPI_MOTOROLA
    }
    #[doc = "Checks if the value of the field is `SPI_TI`"]
    #[inline(always)]
    pub fn is_spi_ti(&self) -> bool {
        *self == MODE_A::SPI_TI
    }
    #[doc = "Checks if the value of the field is `SPI_NS`"]
    #[inline(always)]
    pub fn is_spi_ns(&self) -> bool {
        *self == MODE_A::SPI_NS
    }
}
#[doc = "Field `MODE` writer - N/A"]
pub type MODE_W<'a> = crate::FieldWriter<'a, u32, SPI_CTRL_SPEC, u8, MODE_A, 2, 24>;
impl<'a> MODE_W<'a> {
    #[doc = "SPI Motorola submode. In master mode, when not transmitting data (SELECT is inactive), SCLK is stable at CPOL. In slave mode, when not selected, SCLK is ignored; i.e. it can be either stable or clocking. In master mode, when there is no data to transmit (TX FIFO is empty), SELECT is inactive."]
    #[inline(always)]
    pub fn spi_motorola(self) -> &'a mut W {
        self.variant(MODE_A::SPI_MOTOROLA)
    }
    #[doc = "SPI Texas Instruments submode. In master mode, when not transmitting data, SCLK is stable at '0'. In slave mode, when not selected, SCLK is ignored; i.e. it can be either stable or clocking. In master mode, when there is no data to transmit (TX FIFO is empty), SELECT is inactive; i.e. no pulse is generated."]
    #[inline(always)]
    pub fn spi_ti(self) -> &'a mut W {
        self.variant(MODE_A::SPI_TI)
    }
    #[doc = "SPI National Semiconducturs submode. In master mode, when not transmitting data, SCLK is stable at '0'. In slave mode, when not selected, SCLK is ignored; i.e. it can be either stable or clocking. In master mode, when there is no data to transmit (TX FIFO is empty), SELECT is inactive."]
    #[inline(always)]
    pub fn spi_ns(self) -> &'a mut W {
        self.variant(MODE_A::SPI_NS)
    }
}
#[doc = "Field `SLAVE_SELECT` reader - Selects one of the four outgoing SPI slave select signals: - 0: Slave 0, SPI_SELECT\\[0\\]. - 1: Slave 1, SPI_SELECT\\[1\\]. - 2: Slave 2, SPI_SELECT\\[2\\]. - 3: Slave 3, SPI_SELECT\\[3\\]. Only used in master mode. The IP should be disabled when changes are made to this field. only SPI_SELECT\\[0\\]
is used in slave mode."]
pub type SLAVE_SELECT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLAVE_SELECT` writer - Selects one of the four outgoing SPI slave select signals: - 0: Slave 0, SPI_SELECT\\[0\\]. - 1: Slave 1, SPI_SELECT\\[1\\]. - 2: Slave 2, SPI_SELECT\\[2\\]. - 3: Slave 3, SPI_SELECT\\[3\\]. Only used in master mode. The IP should be disabled when changes are made to this field. only SPI_SELECT\\[0\\]
is used in slave mode."]
pub type SLAVE_SELECT_W<'a> = crate::FieldWriter<'a, u32, SPI_CTRL_SPEC, u8, u8, 2, 26>;
#[doc = "Field `MASTER_MODE` reader - Master ('1') or slave ('0') mode. In master mode, transmission will commence on availability of data frames in the TX FIFO. In slave mode, when selected and there is no data frame in the TX FIFO, the slave will transmit all '1's. In both master and slave modes, received data frames will be lost if the RX FIFO is full."]
pub type MASTER_MODE_R = crate::BitReader<bool>;
#[doc = "Field `MASTER_MODE` writer - Master ('1') or slave ('0') mode. In master mode, transmission will commence on availability of data frames in the TX FIFO. In slave mode, when selected and there is no data frame in the TX FIFO, the slave will transmit all '1's. In both master and slave modes, received data frames will be lost if the RX FIFO is full."]
pub type MASTER_MODE_W<'a> = crate::BitWriter<'a, u32, SPI_CTRL_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - Continuous SPI data transfers enabled ('1') or not ('0'). This field is used in master mode. In slave mode, both continuous and non-continuous SPI data transfers are supported independent of this field. When continuous transfers are enabled individual data frame transfers are not necessarily seperated by slave deselection (as indicated by the level or pulse on the SELECT line): if the TX FIFO has multiple data frames, data frames are send out without slave deselection. When continuous transfers are not enabled individual data frame transfers are always seperated by slave deselection: independent of the availability of TX FIFO data frames, data frames are send out with slave deselection."]
    #[inline(always)]
    pub fn continuous(&self) -> CONTINUOUS_R {
        CONTINUOUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Only used in SPI Texas Instruments' submode. When '1', the data frame start indication is a pulse on the SELECT line that precedes the transfer of the first data frame bit. When '0', the data frame start indication is a pulse on the SELECT line that coincides with the transfer of the first data frame bit."]
    #[inline(always)]
    pub fn select_precede(&self) -> SELECT_PRECEDE_R {
        SELECT_PRECEDE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Only applicable in SPI Motorola submode. Indicates the clock phase. This field, together with the CPOL field, indicates when MOSI data is driven and MISO data is captured: - Motorola mode 0. CPOL is '0', CPHA is '0': MOSI is driven on a falling edge of SCLK. MISO is captured on a rising edge of SCLK. - Motorola mode 1. CPOL is '0', CPHA is '1': MOSI is driven on a rising edge of SCLK. MISO is captured on a falling edge of SCLK. - Motorola mode 2. CPOL is '1', CPHA is '0': MOSI is driven on a rising edge of SCLK. MISO is captured on a falling edge of SCLK. - Motorola mode 3. CPOL is '1', CPHA is '1': MOSI is driven on a falling edge of SCLK. MISO is captured on a rising edge of SCLK."]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates the clock polarity. Only used in SPI Motorola submode. This field, together with the CPHA field, indicates when MOSI data is driven and MISO data is captured: - CPOL is '0': SCLK is '0' when not transmitting data. - CPOL is '1': SCLK is '1' when not transmitting data."]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Changes the SCLK edge on which MISO is captured. Only used in master mode. When '0', the default applies (for Motorola as determined by CPOL and CPHA, for Texas Instruments on the falling edge of SCLK and for National Semiconductors on the rising edge of SCLK). When '1', the alternate clock edge is used (which comes half a SPI SCLK period later). Late sampling addresses the round trip delay associated with transmitting SCLK from the master to the slave and transmitting MISO from the slave to the master."]
    #[inline(always)]
    pub fn late_miso_sample(&self) -> LATE_MISO_SAMPLE_R {
        LATE_MISO_SAMPLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Only applicable in master mode. '0': SCLK is generated, when the SPI master is enabled and data is transmitted. '1': SCLK is generated, when the SPI master is enabled. This mode is useful for slave devices that use SCLK for functional operation other than just SPI functionality."]
    #[inline(always)]
    pub fn sclk_continuous(&self) -> SCLK_CONTINUOUS_R {
        SCLK_CONTINUOUS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Slave select polarity. SSEL_POLARITY0 applies to the outgoing SPI slave select signal 0 (master mode) and to the incoming SPI slave select signal (slave mode). only SPI_SELECT\\[0\\]
is used in slave mode. For Motorola and National Semiconductors submodes: '0': slave select is low/'0' active. '1': slave select is high/'1' active. For Texas Istruments submode: '0': high/'1' active precede/coincide pulse. '1': low/'0' active precede/coincide pulse."]
    #[inline(always)]
    pub fn ssel_polarity0(&self) -> SSEL_POLARITY0_R {
        SSEL_POLARITY0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Slave select polarity. SSEL_POLARITY1 applies to the outgoing SPI slave select signal 1 (master mode)."]
    #[inline(always)]
    pub fn ssel_polarity1(&self) -> SSEL_POLARITY1_R {
        SSEL_POLARITY1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Slave select polarity. SSEL_POLARITY2 applies to the outgoing SPI slave select signal 2 (master mode)."]
    #[inline(always)]
    pub fn ssel_polarity2(&self) -> SSEL_POLARITY2_R {
        SSEL_POLARITY2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Slave select polarity. SSEL_POLARITY3 applies to the outgoing SPI slave select signal 3 (master mode)."]
    #[inline(always)]
    pub fn ssel_polarity3(&self) -> SSEL_POLARITY3_R {
        SSEL_POLARITY3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Local loopback control (does NOT affect the information on the pins). Only used in master mode. Not used in National Semiconductors submode. '0': the SPI master MISO line 'spi_miso_in' is connected to the SPI MISO pin. '1': the SPI master MISO line 'spi_miso_in' is connected to the SPI master MOSI line 'spi_mosi_out'. In other words, in loopback mode the SPI master receives on MISO what it transmits on MOSI."]
    #[inline(always)]
    pub fn loopback(&self) -> LOOPBACK_R {
        LOOPBACK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:25 - N/A"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Selects one of the four outgoing SPI slave select signals: - 0: Slave 0, SPI_SELECT\\[0\\]. - 1: Slave 1, SPI_SELECT\\[1\\]. - 2: Slave 2, SPI_SELECT\\[2\\]. - 3: Slave 3, SPI_SELECT\\[3\\]. Only used in master mode. The IP should be disabled when changes are made to this field. only SPI_SELECT\\[0\\]
is used in slave mode."]
    #[inline(always)]
    pub fn slave_select(&self) -> SLAVE_SELECT_R {
        SLAVE_SELECT_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 31 - Master ('1') or slave ('0') mode. In master mode, transmission will commence on availability of data frames in the TX FIFO. In slave mode, when selected and there is no data frame in the TX FIFO, the slave will transmit all '1's. In both master and slave modes, received data frames will be lost if the RX FIFO is full."]
    #[inline(always)]
    pub fn master_mode(&self) -> MASTER_MODE_R {
        MASTER_MODE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Continuous SPI data transfers enabled ('1') or not ('0'). This field is used in master mode. In slave mode, both continuous and non-continuous SPI data transfers are supported independent of this field. When continuous transfers are enabled individual data frame transfers are not necessarily seperated by slave deselection (as indicated by the level or pulse on the SELECT line): if the TX FIFO has multiple data frames, data frames are send out without slave deselection. When continuous transfers are not enabled individual data frame transfers are always seperated by slave deselection: independent of the availability of TX FIFO data frames, data frames are send out with slave deselection."]
    #[inline(always)]
    pub fn continuous(&mut self) -> CONTINUOUS_W {
        CONTINUOUS_W::new(self)
    }
    #[doc = "Bit 1 - Only used in SPI Texas Instruments' submode. When '1', the data frame start indication is a pulse on the SELECT line that precedes the transfer of the first data frame bit. When '0', the data frame start indication is a pulse on the SELECT line that coincides with the transfer of the first data frame bit."]
    #[inline(always)]
    pub fn select_precede(&mut self) -> SELECT_PRECEDE_W {
        SELECT_PRECEDE_W::new(self)
    }
    #[doc = "Bit 2 - Only applicable in SPI Motorola submode. Indicates the clock phase. This field, together with the CPOL field, indicates when MOSI data is driven and MISO data is captured: - Motorola mode 0. CPOL is '0', CPHA is '0': MOSI is driven on a falling edge of SCLK. MISO is captured on a rising edge of SCLK. - Motorola mode 1. CPOL is '0', CPHA is '1': MOSI is driven on a rising edge of SCLK. MISO is captured on a falling edge of SCLK. - Motorola mode 2. CPOL is '1', CPHA is '0': MOSI is driven on a rising edge of SCLK. MISO is captured on a falling edge of SCLK. - Motorola mode 3. CPOL is '1', CPHA is '1': MOSI is driven on a falling edge of SCLK. MISO is captured on a rising edge of SCLK."]
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W {
        CPHA_W::new(self)
    }
    #[doc = "Bit 3 - Indicates the clock polarity. Only used in SPI Motorola submode. This field, together with the CPHA field, indicates when MOSI data is driven and MISO data is captured: - CPOL is '0': SCLK is '0' when not transmitting data. - CPOL is '1': SCLK is '1' when not transmitting data."]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W {
        CPOL_W::new(self)
    }
    #[doc = "Bit 4 - Changes the SCLK edge on which MISO is captured. Only used in master mode. When '0', the default applies (for Motorola as determined by CPOL and CPHA, for Texas Instruments on the falling edge of SCLK and for National Semiconductors on the rising edge of SCLK). When '1', the alternate clock edge is used (which comes half a SPI SCLK period later). Late sampling addresses the round trip delay associated with transmitting SCLK from the master to the slave and transmitting MISO from the slave to the master."]
    #[inline(always)]
    pub fn late_miso_sample(&mut self) -> LATE_MISO_SAMPLE_W {
        LATE_MISO_SAMPLE_W::new(self)
    }
    #[doc = "Bit 5 - Only applicable in master mode. '0': SCLK is generated, when the SPI master is enabled and data is transmitted. '1': SCLK is generated, when the SPI master is enabled. This mode is useful for slave devices that use SCLK for functional operation other than just SPI functionality."]
    #[inline(always)]
    pub fn sclk_continuous(&mut self) -> SCLK_CONTINUOUS_W {
        SCLK_CONTINUOUS_W::new(self)
    }
    #[doc = "Bit 8 - Slave select polarity. SSEL_POLARITY0 applies to the outgoing SPI slave select signal 0 (master mode) and to the incoming SPI slave select signal (slave mode). only SPI_SELECT\\[0\\]
is used in slave mode. For Motorola and National Semiconductors submodes: '0': slave select is low/'0' active. '1': slave select is high/'1' active. For Texas Istruments submode: '0': high/'1' active precede/coincide pulse. '1': low/'0' active precede/coincide pulse."]
    #[inline(always)]
    pub fn ssel_polarity0(&mut self) -> SSEL_POLARITY0_W {
        SSEL_POLARITY0_W::new(self)
    }
    #[doc = "Bit 9 - Slave select polarity. SSEL_POLARITY1 applies to the outgoing SPI slave select signal 1 (master mode)."]
    #[inline(always)]
    pub fn ssel_polarity1(&mut self) -> SSEL_POLARITY1_W {
        SSEL_POLARITY1_W::new(self)
    }
    #[doc = "Bit 10 - Slave select polarity. SSEL_POLARITY2 applies to the outgoing SPI slave select signal 2 (master mode)."]
    #[inline(always)]
    pub fn ssel_polarity2(&mut self) -> SSEL_POLARITY2_W {
        SSEL_POLARITY2_W::new(self)
    }
    #[doc = "Bit 11 - Slave select polarity. SSEL_POLARITY3 applies to the outgoing SPI slave select signal 3 (master mode)."]
    #[inline(always)]
    pub fn ssel_polarity3(&mut self) -> SSEL_POLARITY3_W {
        SSEL_POLARITY3_W::new(self)
    }
    #[doc = "Bit 16 - Local loopback control (does NOT affect the information on the pins). Only used in master mode. Not used in National Semiconductors submode. '0': the SPI master MISO line 'spi_miso_in' is connected to the SPI MISO pin. '1': the SPI master MISO line 'spi_miso_in' is connected to the SPI master MOSI line 'spi_mosi_out'. In other words, in loopback mode the SPI master receives on MISO what it transmits on MOSI."]
    #[inline(always)]
    pub fn loopback(&mut self) -> LOOPBACK_W {
        LOOPBACK_W::new(self)
    }
    #[doc = "Bits 24:25 - N/A"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W::new(self)
    }
    #[doc = "Bits 26:27 - Selects one of the four outgoing SPI slave select signals: - 0: Slave 0, SPI_SELECT\\[0\\]. - 1: Slave 1, SPI_SELECT\\[1\\]. - 2: Slave 2, SPI_SELECT\\[2\\]. - 3: Slave 3, SPI_SELECT\\[3\\]. Only used in master mode. The IP should be disabled when changes are made to this field. only SPI_SELECT\\[0\\]
is used in slave mode."]
    #[inline(always)]
    pub fn slave_select(&mut self) -> SLAVE_SELECT_W {
        SLAVE_SELECT_W::new(self)
    }
    #[doc = "Bit 31 - Master ('1') or slave ('0') mode. In master mode, transmission will commence on availability of data frames in the TX FIFO. In slave mode, when selected and there is no data frame in the TX FIFO, the slave will transmit all '1's. In both master and slave modes, received data frames will be lost if the RX FIFO is full."]
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
#[doc = "SPI control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_ctrl](index.html) module"]
pub struct SPI_CTRL_SPEC;
impl crate::RegisterSpec for SPI_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_ctrl::R](R) reader structure"]
impl crate::Readable for SPI_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_ctrl::W](W) writer structure"]
impl crate::Writable for SPI_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_CTRL to value 0x0300_0000"]
impl crate::Resettable for SPI_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0300_0000
    }
}
