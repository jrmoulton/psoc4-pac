#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVS` reader - Serial interface bit period oversampling factor expressed in lP clock cycles. Used for SPI and UART functionality. OVS + 1 IP clock cycles constitute a single serial interface clock/bit cycle. The IP clock is provided by the programmable clock IP. This field is NOT used in externally clocked mode. If OVS is odd, the oversampling factor is even and the first and second phase of the interface clock period are the same. If OVS is even, the oversampling factor is odd and the first phase of the interface clock period is 1 peripheral clock cycle longer than the second phase of the interface clock period. In SPI master mode, the valid range is \\[3, 15\\]. At an IP frequency of 48 MHz, the maximum IP bit rate is 12 Mbps. The effective system bit rate is dependent on the external SPI slave that we communicate with. If the SPI output clock \"spi_clk_out\" to SPI MISO input \"spi_miso_in\" round trip delay is introducing significant delays (multiple \"spi_clk_out\" cycles), it may be necessary to increase OVS and/or to set SPI_CTRL.LATE_MISO_SAMPLE to '1' to achieve the maximum possible system bit rate. The max- imum IP bit rate of 12 Mbps provides an IP centric perspective, assuming ideal (0 ns) IO cell and routing (chip and board) delay. The required IP clock/IF clock ratio increases and the calculated maximum bit rate decreases, when realistic chip and routing delays are taken into account. In SPI slave mode, the OVS field is NOT used. However, there is a frequency requirement for the IP clock wrt. the interface (IF) clock to guarantee functional correct behavior. This requirement is expressed as a ratio: IP clock/IF clock. The ratio is dependent on the setting of RX_CTRL.MEDIAN and the external SPI master's capability to support \"late MISO sample\" functionality (similar to our SPI master functionality represented by SPI_CTRL.LATE_MISO_SAMPLE): - MEDIAN is '0' and external SPI master has NO \"late MISO sample functionality\": IP clock/IF clock >= 6. At a IP frequency of 48 MHz, the maximum bit rate is 8 Mbps. - MEDIAN is '0' and external SPI master has \"late MISO sample functionality\": IP clock/IF clock >= 3. At a IP frequency of 48 MHz, the maximum bit rate is 16 Mbps. - MEDIAN is '1' and external SPI master has NO \"late MISO sample functionality\": IP clock/IF clock >= 8. At a IP frequency of 48 MHz, the maximum bit rate is 6 Mbps. - MEDIAN is '1' and external SPI master has \"late MISO sample functionality\": IP clock/IF clock >= 4. At a IP frequency of 48 MHz, the maximum bit rate is 12 Mbps. The maximum bit rates provide an IP centric perspective, assuming ideal (0 ns) IO cell and routing (chip and board) delay. The required IP clock/IF clock ratio increases and the calculated maximum bit rate decreases, when realistic chip and routing delays are taken into account. In UART standard submode (including LIN), the valid range is \\[7, 15\\]. In UART SmartCard submode, the valid range is \\[7, 15\\]. In UART TX IrDA submode this field indirectly specifies the oversampling. The oversampling determines the interface clock/bit cycle and the width of the pulse. Only normal transmission mode is supported, the pulse is roughly 3/16 of the bit period (for all bit rates). There is only one valid OVS value: - 0: 16 times oversampling. IP clock frequency of 16*115.2 KHz for 115.2 Kbps. IP clock frequency of 16*57.6 KHz for 57.6 Kbps. IP clock frequency of 16*38.4 KHz for 38.4 Kbps. IP clock frequency of 16*19.2 KHz for 19.2 Kbps. IP clock frequency of 16*9.6 KHz for 9.6 Kbps. IP clock frequency of 16*2.4 KHz for 2.4 Kbps. IP clock frequency of 16*1.2 KHz for 1.2 Kbps. - all other values are not used in normal mode. In UART RX IrDA submode (1.2, 2.4, 9.6, 19.2, 38.4, 57.6 and 115.2 Kbps) this field indirectly specifies the oversampling. The oversampling determines the interface clock/bit cycle and the width of the pulse. In normal transmission mode, this pulse is roughly 3/16 of the bit period (for all bit rates). In low power transmission mode, this pulse is potentially smaller (down to 1.62 us typical and 1.41 us minimal) than 3/16 of the bit period (for less than 115.2 Kbps bitrates). Pulse widths greater or equal than two IP clock cycles are guaranteed to be detected by the receiver. Pulse widths less than two IP clock cycles and greater or equal than one IP clock cycle may be detected by the receiver. Pulse widths less than one IP clock cycle will not be detected by the receiver. RX_CTRL.MEDIAN should be set to '1' for IrDA receiver functionality. The IP clock (as provided by the programmable clock IP) and the oversampling together determine the IrDA bitrate. Normal mode, OVS field values (with the required IP clock frequency): - 0: 16 times oversampling. IP clock frequency of 16*115.2 KHz for 115.2 Kbps. IP clock frequency of 16*57.6 KHz for 57.6 Kbps. IP clock frequency of 16*38.4 KHz for 38.4 Kbps. IP clock frequency of 16*19.2 KHz for 19.2 Kbps. IP clock frequency of 16*9.6 KHz for 9.6 Kbps. IP clock frequency of 16*2.4 KHz for 2.4 Kbps. IP clock frequency of 16*1.2 KHz for 1.2 Kbps. - all other values are not used in normal mode. Low power mode, OVS field values (with the required IP clock frequency): - 0: 16 times oversampling. IP clock frequency of 16*115.2 KHz for 115.2 Kbps. - 1: 32 times oversampling. IP clock frequency of 32*57.6 KHz for 57.6 Kbps. - 2: 48 times oversampling. IP clock frequency of 48*38.4 KHz for 38.4 Kbps. - 3: 96 times oversampling. IP clock frequency of 96*19.2 KHz for 19.2 Kbps. - 4: 192 times oversampling. IP clock frequency of 192*9.6 KHz for 9.6 Kbps. - 5: 768 times oversampling. IP clock frequency of 768*2.4 KHz for 2.4 Kbps. - 6: 1536 times oversampling. IP clock frequency of 1536*1.2 KHz for 1.2 Kbps. - all other values are not used in low power mode. Default Value: 15"]
pub type OVS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OVS` writer - Serial interface bit period oversampling factor expressed in lP clock cycles. Used for SPI and UART functionality. OVS + 1 IP clock cycles constitute a single serial interface clock/bit cycle. The IP clock is provided by the programmable clock IP. This field is NOT used in externally clocked mode. If OVS is odd, the oversampling factor is even and the first and second phase of the interface clock period are the same. If OVS is even, the oversampling factor is odd and the first phase of the interface clock period is 1 peripheral clock cycle longer than the second phase of the interface clock period. In SPI master mode, the valid range is \\[3, 15\\]. At an IP frequency of 48 MHz, the maximum IP bit rate is 12 Mbps. The effective system bit rate is dependent on the external SPI slave that we communicate with. If the SPI output clock \"spi_clk_out\" to SPI MISO input \"spi_miso_in\" round trip delay is introducing significant delays (multiple \"spi_clk_out\" cycles), it may be necessary to increase OVS and/or to set SPI_CTRL.LATE_MISO_SAMPLE to '1' to achieve the maximum possible system bit rate. The max- imum IP bit rate of 12 Mbps provides an IP centric perspective, assuming ideal (0 ns) IO cell and routing (chip and board) delay. The required IP clock/IF clock ratio increases and the calculated maximum bit rate decreases, when realistic chip and routing delays are taken into account. In SPI slave mode, the OVS field is NOT used. However, there is a frequency requirement for the IP clock wrt. the interface (IF) clock to guarantee functional correct behavior. This requirement is expressed as a ratio: IP clock/IF clock. The ratio is dependent on the setting of RX_CTRL.MEDIAN and the external SPI master's capability to support \"late MISO sample\" functionality (similar to our SPI master functionality represented by SPI_CTRL.LATE_MISO_SAMPLE): - MEDIAN is '0' and external SPI master has NO \"late MISO sample functionality\": IP clock/IF clock >= 6. At a IP frequency of 48 MHz, the maximum bit rate is 8 Mbps. - MEDIAN is '0' and external SPI master has \"late MISO sample functionality\": IP clock/IF clock >= 3. At a IP frequency of 48 MHz, the maximum bit rate is 16 Mbps. - MEDIAN is '1' and external SPI master has NO \"late MISO sample functionality\": IP clock/IF clock >= 8. At a IP frequency of 48 MHz, the maximum bit rate is 6 Mbps. - MEDIAN is '1' and external SPI master has \"late MISO sample functionality\": IP clock/IF clock >= 4. At a IP frequency of 48 MHz, the maximum bit rate is 12 Mbps. The maximum bit rates provide an IP centric perspective, assuming ideal (0 ns) IO cell and routing (chip and board) delay. The required IP clock/IF clock ratio increases and the calculated maximum bit rate decreases, when realistic chip and routing delays are taken into account. In UART standard submode (including LIN), the valid range is \\[7, 15\\]. In UART SmartCard submode, the valid range is \\[7, 15\\]. In UART TX IrDA submode this field indirectly specifies the oversampling. The oversampling determines the interface clock/bit cycle and the width of the pulse. Only normal transmission mode is supported, the pulse is roughly 3/16 of the bit period (for all bit rates). There is only one valid OVS value: - 0: 16 times oversampling. IP clock frequency of 16*115.2 KHz for 115.2 Kbps. IP clock frequency of 16*57.6 KHz for 57.6 Kbps. IP clock frequency of 16*38.4 KHz for 38.4 Kbps. IP clock frequency of 16*19.2 KHz for 19.2 Kbps. IP clock frequency of 16*9.6 KHz for 9.6 Kbps. IP clock frequency of 16*2.4 KHz for 2.4 Kbps. IP clock frequency of 16*1.2 KHz for 1.2 Kbps. - all other values are not used in normal mode. In UART RX IrDA submode (1.2, 2.4, 9.6, 19.2, 38.4, 57.6 and 115.2 Kbps) this field indirectly specifies the oversampling. The oversampling determines the interface clock/bit cycle and the width of the pulse. In normal transmission mode, this pulse is roughly 3/16 of the bit period (for all bit rates). In low power transmission mode, this pulse is potentially smaller (down to 1.62 us typical and 1.41 us minimal) than 3/16 of the bit period (for less than 115.2 Kbps bitrates). Pulse widths greater or equal than two IP clock cycles are guaranteed to be detected by the receiver. Pulse widths less than two IP clock cycles and greater or equal than one IP clock cycle may be detected by the receiver. Pulse widths less than one IP clock cycle will not be detected by the receiver. RX_CTRL.MEDIAN should be set to '1' for IrDA receiver functionality. The IP clock (as provided by the programmable clock IP) and the oversampling together determine the IrDA bitrate. Normal mode, OVS field values (with the required IP clock frequency): - 0: 16 times oversampling. IP clock frequency of 16*115.2 KHz for 115.2 Kbps. IP clock frequency of 16*57.6 KHz for 57.6 Kbps. IP clock frequency of 16*38.4 KHz for 38.4 Kbps. IP clock frequency of 16*19.2 KHz for 19.2 Kbps. IP clock frequency of 16*9.6 KHz for 9.6 Kbps. IP clock frequency of 16*2.4 KHz for 2.4 Kbps. IP clock frequency of 16*1.2 KHz for 1.2 Kbps. - all other values are not used in normal mode. Low power mode, OVS field values (with the required IP clock frequency): - 0: 16 times oversampling. IP clock frequency of 16*115.2 KHz for 115.2 Kbps. - 1: 32 times oversampling. IP clock frequency of 32*57.6 KHz for 57.6 Kbps. - 2: 48 times oversampling. IP clock frequency of 48*38.4 KHz for 38.4 Kbps. - 3: 96 times oversampling. IP clock frequency of 96*19.2 KHz for 19.2 Kbps. - 4: 192 times oversampling. IP clock frequency of 192*9.6 KHz for 9.6 Kbps. - 5: 768 times oversampling. IP clock frequency of 768*2.4 KHz for 2.4 Kbps. - 6: 1536 times oversampling. IP clock frequency of 1536*1.2 KHz for 1.2 Kbps. - all other values are not used in low power mode. Default Value: 15"]
pub type OVS_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, 0>;
#[doc = "Field `EC_AM_MODE` reader - Internally clocked mode ('0') or externally clocked mode ('1') address matching (I2C) or selection (SPI). In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. In UART mode this field should be '0'."]
pub type EC_AM_MODE_R = crate::BitReader<bool>;
#[doc = "Field `EC_AM_MODE` writer - Internally clocked mode ('0') or externally clocked mode ('1') address matching (I2C) or selection (SPI). In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. In UART mode this field should be '0'."]
pub type EC_AM_MODE_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 8>;
#[doc = "Field `EC_OP_MODE` reader - Internally clocked mode ('0') or externally clocked mode ('1') operation. In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked operation mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode AND EZ mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. The maximum SPI slave, EZ mode bitrate is 48 Mbps (transmission and IO delays outside the IP will degrade the effective bitrate). In UART mode this field should be '0'."]
pub type EC_OP_MODE_R = crate::BitReader<bool>;
#[doc = "Field `EC_OP_MODE` writer - Internally clocked mode ('0') or externally clocked mode ('1') operation. In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked operation mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode AND EZ mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. The maximum SPI slave, EZ mode bitrate is 48 Mbps (transmission and IO delays outside the IP will degrade the effective bitrate). In UART mode this field should be '0'."]
pub type EC_OP_MODE_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 9>;
#[doc = "Field `EZ_MODE` reader - Non EZ mode ('0') or EZ mode ('1'). In EZ mode, a meta protocol is applied to the serial interface protocol. This meta protocol adds meaning to the data frames transferred by the serial interface protocol: a data frame can represent a memory address, a write memory data element or a read memory data element. EZ mode is only used for synchronous serial interface protocols: SPI and I2C. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported and the transmitter should use continuous data frames; i.e. data frames mot seperated by slave deselection. This mode is only applicable to slave functionality. In EZ mode, the slave can read from and write to an addressable memory structure of 32 bytes. In EZ mode, data frames should 8-bit in size and should be transmitted and received with the Most Significant Bit (MSB) first. In UART mode this field should be '0'."]
pub type EZ_MODE_R = crate::BitReader<bool>;
#[doc = "Field `EZ_MODE` writer - Non EZ mode ('0') or EZ mode ('1'). In EZ mode, a meta protocol is applied to the serial interface protocol. This meta protocol adds meaning to the data frames transferred by the serial interface protocol: a data frame can represent a memory address, a write memory data element or a read memory data element. EZ mode is only used for synchronous serial interface protocols: SPI and I2C. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported and the transmitter should use continuous data frames; i.e. data frames mot seperated by slave deselection. This mode is only applicable to slave functionality. In EZ mode, the slave can read from and write to an addressable memory structure of 32 bytes. In EZ mode, data frames should 8-bit in size and should be transmitted and received with the Most Significant Bit (MSB) first. In UART mode this field should be '0'."]
pub type EZ_MODE_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 10>;
#[doc = "Field `BYTE_MODE` reader - Determines the number of bits per FIFO data element: '0': 16-bit FIFO data elements. '1': 8-bit FIFO data elements. This mode doubles the amount of FIFO entries, but TX_CTRL.DATA_WIDTH and RX_CTRL.DATA_WIDTH are restricted to \\[0, 7\\]."]
pub type BYTE_MODE_R = crate::BitReader<bool>;
#[doc = "Field `BYTE_MODE` writer - Determines the number of bits per FIFO data element: '0': 16-bit FIFO data elements. '1': 8-bit FIFO data elements. This mode doubles the amount of FIFO entries, but TX_CTRL.DATA_WIDTH and RX_CTRL.DATA_WIDTH are restricted to \\[0, 7\\]."]
pub type BYTE_MODE_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 11>;
#[doc = "Field `CMD_RESP_MODE` reader - Determines CMD_RESP mode of operation: '0': CMD_RESP mode disabled. '1': CMD_RESP mode enabled (also requires EC_AM_MODE and EC_OP_MODE to be set to '1')."]
pub type CMD_RESP_MODE_R = crate::BitReader<bool>;
#[doc = "Field `CMD_RESP_MODE` writer - Determines CMD_RESP mode of operation: '0': CMD_RESP mode disabled. '1': CMD_RESP mode enabled (also requires EC_AM_MODE and EC_OP_MODE to be set to '1')."]
pub type CMD_RESP_MODE_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 12>;
#[doc = "Field `ADDR_ACCEPT` reader - Determines whether a received matching address is accepted in the RX FIFO ('1') or not ('0'). In I2C mode, this field is used to allow the slave to put the received slave address or general call address in the RX FIFO. Note that a received matching address is put in the RX FIFO when ADDR_ACCEPT is '1' for both I2C read and write transfers. In multi-processor UART receiver mode, this field is used to allow the receiver to put the received address in the RX FIFO. Note: non-matching addresses are never put in the RX FIFO."]
pub type ADDR_ACCEPT_R = crate::BitReader<bool>;
#[doc = "Field `ADDR_ACCEPT` writer - Determines whether a received matching address is accepted in the RX FIFO ('1') or not ('0'). In I2C mode, this field is used to allow the slave to put the received slave address or general call address in the RX FIFO. Note that a received matching address is put in the RX FIFO when ADDR_ACCEPT is '1' for both I2C read and write transfers. In multi-processor UART receiver mode, this field is used to allow the receiver to put the received address in the RX FIFO. Note: non-matching addresses are never put in the RX FIFO."]
pub type ADDR_ACCEPT_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 16>;
#[doc = "Field `BLOCK` reader - Only used in externally clocked mode. If the externally clocked logic and the MMIO SW accesses to EZ memory coincide/collide, this bit determines whether a SW access should block and result in bus wait states ('BLOCK is 1') or not (BLOCK is '0'). IF BLOCK is '0' and the accesses collide, MMIO read operations return 0xffff:ffff and MMIO write operations are ignored. Colliding accesses are registered as interrupt causes: field BLOCKED of MMIO registers INTR_TX and INTR_RX."]
pub type BLOCK_R = crate::BitReader<bool>;
#[doc = "Field `BLOCK` writer - Only used in externally clocked mode. If the externally clocked logic and the MMIO SW accesses to EZ memory coincide/collide, this bit determines whether a SW access should block and result in bus wait states ('BLOCK is 1') or not (BLOCK is '0'). IF BLOCK is '0' and the accesses collide, MMIO read operations return 0xffff:ffff and MMIO write operations are ignored. Colliding accesses are registered as interrupt causes: field BLOCKED of MMIO registers INTR_TX and INTR_RX."]
pub type BLOCK_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 17>;
#[doc = "Mode of operation (3: Reserved) Default Value: 3 0x0: I2C : Inter-Integrated Circuits (I2C) mode. 0x1: SPI : Serial Peripheral Interface (SPI) mode. 0x2: UART : Universal Asynchronous Receiver/Transmitter (UART) mode.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Inter-Integrated Circuits (I2C) mode."]
    I2C = 0,
    #[doc = "1: Serial Peripheral Interface (SPI) mode."]
    SPI = 1,
    #[doc = "2: Universal Asynchronous Receiver/Transmitter (UART) mode."]
    UART = 2,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - Mode of operation (3: Reserved) Default Value: 3 0x0: I2C : Inter-Integrated Circuits (I2C) mode. 0x1: SPI : Serial Peripheral Interface (SPI) mode. 0x2: UART : Universal Asynchronous Receiver/Transmitter (UART) mode."]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::I2C),
            1 => Some(MODE_A::SPI),
            2 => Some(MODE_A::UART),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `I2C`"]
    #[inline(always)]
    pub fn is_i2c(&self) -> bool {
        *self == MODE_A::I2C
    }
    #[doc = "Checks if the value of the field is `SPI`"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == MODE_A::SPI
    }
    #[doc = "Checks if the value of the field is `UART`"]
    #[inline(always)]
    pub fn is_uart(&self) -> bool {
        *self == MODE_A::UART
    }
}
#[doc = "Field `MODE` writer - Mode of operation (3: Reserved) Default Value: 3 0x0: I2C : Inter-Integrated Circuits (I2C) mode. 0x1: SPI : Serial Peripheral Interface (SPI) mode. 0x2: UART : Universal Asynchronous Receiver/Transmitter (UART) mode."]
pub type MODE_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, MODE_A, 2, 24>;
impl<'a> MODE_W<'a> {
    #[doc = "Inter-Integrated Circuits (I2C) mode."]
    #[inline(always)]
    pub fn i2c(self) -> &'a mut W {
        self.variant(MODE_A::I2C)
    }
    #[doc = "Serial Peripheral Interface (SPI) mode."]
    #[inline(always)]
    pub fn spi(self) -> &'a mut W {
        self.variant(MODE_A::SPI)
    }
    #[doc = "Universal Asynchronous Receiver/Transmitter (UART) mode."]
    #[inline(always)]
    pub fn uart(self) -> &'a mut W {
        self.variant(MODE_A::UART)
    }
}
#[doc = "Field `ENABLED` reader - IP enabled ('1') or not ('0'). The proper order in which to initialize the IP is as follows: - Program protocol specific information using SPI_CTRL, UART_CTRL (and UART_TX_CTRL and UART_RX_CTRL) or I2C_CTRL. This includes selection of a submode, master/slave functionality and transmitter/receiver functionality when applicable. - Program generic transmitter (TX_CTRL) and receiver (RX_CTRL) information. This includes enabling of the transmitter and receiver functionality. - Program transmitter FIFO (TX_FIFO_CTRL) and receiver FIFO (RX_FIFO_CTRL) information. - Program CTRL to enable IP, select the specific operation mode and oversampling factor. When the IP is enabled, no control information should be changed. Changes should be made AFTER disabling the IP, e.g. to modify the operation mode (from I2C to SPI) or to go from externally to internally clocked. The change takes effect after the IP is re-enabled. Note that disabling the IP will cause re-initialization of the design and associated state is lost (e.g. FIFO content)."]
pub type ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ENABLED` writer - IP enabled ('1') or not ('0'). The proper order in which to initialize the IP is as follows: - Program protocol specific information using SPI_CTRL, UART_CTRL (and UART_TX_CTRL and UART_RX_CTRL) or I2C_CTRL. This includes selection of a submode, master/slave functionality and transmitter/receiver functionality when applicable. - Program generic transmitter (TX_CTRL) and receiver (RX_CTRL) information. This includes enabling of the transmitter and receiver functionality. - Program transmitter FIFO (TX_FIFO_CTRL) and receiver FIFO (RX_FIFO_CTRL) information. - Program CTRL to enable IP, select the specific operation mode and oversampling factor. When the IP is enabled, no control information should be changed. Changes should be made AFTER disabling the IP, e.g. to modify the operation mode (from I2C to SPI) or to go from externally to internally clocked. The change takes effect after the IP is re-enabled. Note that disabling the IP will cause re-initialization of the design and associated state is lost (e.g. FIFO content)."]
pub type ENABLED_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:3 - Serial interface bit period oversampling factor expressed in lP clock cycles. Used for SPI and UART functionality. OVS + 1 IP clock cycles constitute a single serial interface clock/bit cycle. The IP clock is provided by the programmable clock IP. This field is NOT used in externally clocked mode. If OVS is odd, the oversampling factor is even and the first and second phase of the interface clock period are the same. If OVS is even, the oversampling factor is odd and the first phase of the interface clock period is 1 peripheral clock cycle longer than the second phase of the interface clock period. In SPI master mode, the valid range is \\[3, 15\\]. At an IP frequency of 48 MHz, the maximum IP bit rate is 12 Mbps. The effective system bit rate is dependent on the external SPI slave that we communicate with. If the SPI output clock \"spi_clk_out\" to SPI MISO input \"spi_miso_in\" round trip delay is introducing significant delays (multiple \"spi_clk_out\" cycles), it may be necessary to increase OVS and/or to set SPI_CTRL.LATE_MISO_SAMPLE to '1' to achieve the maximum possible system bit rate. The max- imum IP bit rate of 12 Mbps provides an IP centric perspective, assuming ideal (0 ns) IO cell and routing (chip and board) delay. The required IP clock/IF clock ratio increases and the calculated maximum bit rate decreases, when realistic chip and routing delays are taken into account. In SPI slave mode, the OVS field is NOT used. However, there is a frequency requirement for the IP clock wrt. the interface (IF) clock to guarantee functional correct behavior. This requirement is expressed as a ratio: IP clock/IF clock. The ratio is dependent on the setting of RX_CTRL.MEDIAN and the external SPI master's capability to support \"late MISO sample\" functionality (similar to our SPI master functionality represented by SPI_CTRL.LATE_MISO_SAMPLE): - MEDIAN is '0' and external SPI master has NO \"late MISO sample functionality\": IP clock/IF clock >= 6. At a IP frequency of 48 MHz, the maximum bit rate is 8 Mbps. - MEDIAN is '0' and external SPI master has \"late MISO sample functionality\": IP clock/IF clock >= 3. At a IP frequency of 48 MHz, the maximum bit rate is 16 Mbps. - MEDIAN is '1' and external SPI master has NO \"late MISO sample functionality\": IP clock/IF clock >= 8. At a IP frequency of 48 MHz, the maximum bit rate is 6 Mbps. - MEDIAN is '1' and external SPI master has \"late MISO sample functionality\": IP clock/IF clock >= 4. At a IP frequency of 48 MHz, the maximum bit rate is 12 Mbps. The maximum bit rates provide an IP centric perspective, assuming ideal (0 ns) IO cell and routing (chip and board) delay. The required IP clock/IF clock ratio increases and the calculated maximum bit rate decreases, when realistic chip and routing delays are taken into account. In UART standard submode (including LIN), the valid range is \\[7, 15\\]. In UART SmartCard submode, the valid range is \\[7, 15\\]. In UART TX IrDA submode this field indirectly specifies the oversampling. The oversampling determines the interface clock/bit cycle and the width of the pulse. Only normal transmission mode is supported, the pulse is roughly 3/16 of the bit period (for all bit rates). There is only one valid OVS value: - 0: 16 times oversampling. IP clock frequency of 16*115.2 KHz for 115.2 Kbps. IP clock frequency of 16*57.6 KHz for 57.6 Kbps. IP clock frequency of 16*38.4 KHz for 38.4 Kbps. IP clock frequency of 16*19.2 KHz for 19.2 Kbps. IP clock frequency of 16*9.6 KHz for 9.6 Kbps. IP clock frequency of 16*2.4 KHz for 2.4 Kbps. IP clock frequency of 16*1.2 KHz for 1.2 Kbps. - all other values are not used in normal mode. In UART RX IrDA submode (1.2, 2.4, 9.6, 19.2, 38.4, 57.6 and 115.2 Kbps) this field indirectly specifies the oversampling. The oversampling determines the interface clock/bit cycle and the width of the pulse. In normal transmission mode, this pulse is roughly 3/16 of the bit period (for all bit rates). In low power transmission mode, this pulse is potentially smaller (down to 1.62 us typical and 1.41 us minimal) than 3/16 of the bit period (for less than 115.2 Kbps bitrates). Pulse widths greater or equal than two IP clock cycles are guaranteed to be detected by the receiver. Pulse widths less than two IP clock cycles and greater or equal than one IP clock cycle may be detected by the receiver. Pulse widths less than one IP clock cycle will not be detected by the receiver. RX_CTRL.MEDIAN should be set to '1' for IrDA receiver functionality. The IP clock (as provided by the programmable clock IP) and the oversampling together determine the IrDA bitrate. Normal mode, OVS field values (with the required IP clock frequency): - 0: 16 times oversampling. IP clock frequency of 16*115.2 KHz for 115.2 Kbps. IP clock frequency of 16*57.6 KHz for 57.6 Kbps. IP clock frequency of 16*38.4 KHz for 38.4 Kbps. IP clock frequency of 16*19.2 KHz for 19.2 Kbps. IP clock frequency of 16*9.6 KHz for 9.6 Kbps. IP clock frequency of 16*2.4 KHz for 2.4 Kbps. IP clock frequency of 16*1.2 KHz for 1.2 Kbps. - all other values are not used in normal mode. Low power mode, OVS field values (with the required IP clock frequency): - 0: 16 times oversampling. IP clock frequency of 16*115.2 KHz for 115.2 Kbps. - 1: 32 times oversampling. IP clock frequency of 32*57.6 KHz for 57.6 Kbps. - 2: 48 times oversampling. IP clock frequency of 48*38.4 KHz for 38.4 Kbps. - 3: 96 times oversampling. IP clock frequency of 96*19.2 KHz for 19.2 Kbps. - 4: 192 times oversampling. IP clock frequency of 192*9.6 KHz for 9.6 Kbps. - 5: 768 times oversampling. IP clock frequency of 768*2.4 KHz for 2.4 Kbps. - 6: 1536 times oversampling. IP clock frequency of 1536*1.2 KHz for 1.2 Kbps. - all other values are not used in low power mode. Default Value: 15"]
    #[inline(always)]
    pub fn ovs(&self) -> OVS_R {
        OVS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Internally clocked mode ('0') or externally clocked mode ('1') address matching (I2C) or selection (SPI). In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. In UART mode this field should be '0'."]
    #[inline(always)]
    pub fn ec_am_mode(&self) -> EC_AM_MODE_R {
        EC_AM_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Internally clocked mode ('0') or externally clocked mode ('1') operation. In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked operation mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode AND EZ mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. The maximum SPI slave, EZ mode bitrate is 48 Mbps (transmission and IO delays outside the IP will degrade the effective bitrate). In UART mode this field should be '0'."]
    #[inline(always)]
    pub fn ec_op_mode(&self) -> EC_OP_MODE_R {
        EC_OP_MODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Non EZ mode ('0') or EZ mode ('1'). In EZ mode, a meta protocol is applied to the serial interface protocol. This meta protocol adds meaning to the data frames transferred by the serial interface protocol: a data frame can represent a memory address, a write memory data element or a read memory data element. EZ mode is only used for synchronous serial interface protocols: SPI and I2C. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported and the transmitter should use continuous data frames; i.e. data frames mot seperated by slave deselection. This mode is only applicable to slave functionality. In EZ mode, the slave can read from and write to an addressable memory structure of 32 bytes. In EZ mode, data frames should 8-bit in size and should be transmitted and received with the Most Significant Bit (MSB) first. In UART mode this field should be '0'."]
    #[inline(always)]
    pub fn ez_mode(&self) -> EZ_MODE_R {
        EZ_MODE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Determines the number of bits per FIFO data element: '0': 16-bit FIFO data elements. '1': 8-bit FIFO data elements. This mode doubles the amount of FIFO entries, but TX_CTRL.DATA_WIDTH and RX_CTRL.DATA_WIDTH are restricted to \\[0, 7\\]."]
    #[inline(always)]
    pub fn byte_mode(&self) -> BYTE_MODE_R {
        BYTE_MODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Determines CMD_RESP mode of operation: '0': CMD_RESP mode disabled. '1': CMD_RESP mode enabled (also requires EC_AM_MODE and EC_OP_MODE to be set to '1')."]
    #[inline(always)]
    pub fn cmd_resp_mode(&self) -> CMD_RESP_MODE_R {
        CMD_RESP_MODE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Determines whether a received matching address is accepted in the RX FIFO ('1') or not ('0'). In I2C mode, this field is used to allow the slave to put the received slave address or general call address in the RX FIFO. Note that a received matching address is put in the RX FIFO when ADDR_ACCEPT is '1' for both I2C read and write transfers. In multi-processor UART receiver mode, this field is used to allow the receiver to put the received address in the RX FIFO. Note: non-matching addresses are never put in the RX FIFO."]
    #[inline(always)]
    pub fn addr_accept(&self) -> ADDR_ACCEPT_R {
        ADDR_ACCEPT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Only used in externally clocked mode. If the externally clocked logic and the MMIO SW accesses to EZ memory coincide/collide, this bit determines whether a SW access should block and result in bus wait states ('BLOCK is 1') or not (BLOCK is '0'). IF BLOCK is '0' and the accesses collide, MMIO read operations return 0xffff:ffff and MMIO write operations are ignored. Colliding accesses are registered as interrupt causes: field BLOCKED of MMIO registers INTR_TX and INTR_RX."]
    #[inline(always)]
    pub fn block(&self) -> BLOCK_R {
        BLOCK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Mode of operation (3: Reserved) Default Value: 3 0x0: I2C : Inter-Integrated Circuits (I2C) mode. 0x1: SPI : Serial Peripheral Interface (SPI) mode. 0x2: UART : Universal Asynchronous Receiver/Transmitter (UART) mode."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 31 - IP enabled ('1') or not ('0'). The proper order in which to initialize the IP is as follows: - Program protocol specific information using SPI_CTRL, UART_CTRL (and UART_TX_CTRL and UART_RX_CTRL) or I2C_CTRL. This includes selection of a submode, master/slave functionality and transmitter/receiver functionality when applicable. - Program generic transmitter (TX_CTRL) and receiver (RX_CTRL) information. This includes enabling of the transmitter and receiver functionality. - Program transmitter FIFO (TX_FIFO_CTRL) and receiver FIFO (RX_FIFO_CTRL) information. - Program CTRL to enable IP, select the specific operation mode and oversampling factor. When the IP is enabled, no control information should be changed. Changes should be made AFTER disabling the IP, e.g. to modify the operation mode (from I2C to SPI) or to go from externally to internally clocked. The change takes effect after the IP is re-enabled. Note that disabling the IP will cause re-initialization of the design and associated state is lost (e.g. FIFO content)."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Serial interface bit period oversampling factor expressed in lP clock cycles. Used for SPI and UART functionality. OVS + 1 IP clock cycles constitute a single serial interface clock/bit cycle. The IP clock is provided by the programmable clock IP. This field is NOT used in externally clocked mode. If OVS is odd, the oversampling factor is even and the first and second phase of the interface clock period are the same. If OVS is even, the oversampling factor is odd and the first phase of the interface clock period is 1 peripheral clock cycle longer than the second phase of the interface clock period. In SPI master mode, the valid range is \\[3, 15\\]. At an IP frequency of 48 MHz, the maximum IP bit rate is 12 Mbps. The effective system bit rate is dependent on the external SPI slave that we communicate with. If the SPI output clock \"spi_clk_out\" to SPI MISO input \"spi_miso_in\" round trip delay is introducing significant delays (multiple \"spi_clk_out\" cycles), it may be necessary to increase OVS and/or to set SPI_CTRL.LATE_MISO_SAMPLE to '1' to achieve the maximum possible system bit rate. The max- imum IP bit rate of 12 Mbps provides an IP centric perspective, assuming ideal (0 ns) IO cell and routing (chip and board) delay. The required IP clock/IF clock ratio increases and the calculated maximum bit rate decreases, when realistic chip and routing delays are taken into account. In SPI slave mode, the OVS field is NOT used. However, there is a frequency requirement for the IP clock wrt. the interface (IF) clock to guarantee functional correct behavior. This requirement is expressed as a ratio: IP clock/IF clock. The ratio is dependent on the setting of RX_CTRL.MEDIAN and the external SPI master's capability to support \"late MISO sample\" functionality (similar to our SPI master functionality represented by SPI_CTRL.LATE_MISO_SAMPLE): - MEDIAN is '0' and external SPI master has NO \"late MISO sample functionality\": IP clock/IF clock >= 6. At a IP frequency of 48 MHz, the maximum bit rate is 8 Mbps. - MEDIAN is '0' and external SPI master has \"late MISO sample functionality\": IP clock/IF clock >= 3. At a IP frequency of 48 MHz, the maximum bit rate is 16 Mbps. - MEDIAN is '1' and external SPI master has NO \"late MISO sample functionality\": IP clock/IF clock >= 8. At a IP frequency of 48 MHz, the maximum bit rate is 6 Mbps. - MEDIAN is '1' and external SPI master has \"late MISO sample functionality\": IP clock/IF clock >= 4. At a IP frequency of 48 MHz, the maximum bit rate is 12 Mbps. The maximum bit rates provide an IP centric perspective, assuming ideal (0 ns) IO cell and routing (chip and board) delay. The required IP clock/IF clock ratio increases and the calculated maximum bit rate decreases, when realistic chip and routing delays are taken into account. In UART standard submode (including LIN), the valid range is \\[7, 15\\]. In UART SmartCard submode, the valid range is \\[7, 15\\]. In UART TX IrDA submode this field indirectly specifies the oversampling. The oversampling determines the interface clock/bit cycle and the width of the pulse. Only normal transmission mode is supported, the pulse is roughly 3/16 of the bit period (for all bit rates). There is only one valid OVS value: - 0: 16 times oversampling. IP clock frequency of 16*115.2 KHz for 115.2 Kbps. IP clock frequency of 16*57.6 KHz for 57.6 Kbps. IP clock frequency of 16*38.4 KHz for 38.4 Kbps. IP clock frequency of 16*19.2 KHz for 19.2 Kbps. IP clock frequency of 16*9.6 KHz for 9.6 Kbps. IP clock frequency of 16*2.4 KHz for 2.4 Kbps. IP clock frequency of 16*1.2 KHz for 1.2 Kbps. - all other values are not used in normal mode. In UART RX IrDA submode (1.2, 2.4, 9.6, 19.2, 38.4, 57.6 and 115.2 Kbps) this field indirectly specifies the oversampling. The oversampling determines the interface clock/bit cycle and the width of the pulse. In normal transmission mode, this pulse is roughly 3/16 of the bit period (for all bit rates). In low power transmission mode, this pulse is potentially smaller (down to 1.62 us typical and 1.41 us minimal) than 3/16 of the bit period (for less than 115.2 Kbps bitrates). Pulse widths greater or equal than two IP clock cycles are guaranteed to be detected by the receiver. Pulse widths less than two IP clock cycles and greater or equal than one IP clock cycle may be detected by the receiver. Pulse widths less than one IP clock cycle will not be detected by the receiver. RX_CTRL.MEDIAN should be set to '1' for IrDA receiver functionality. The IP clock (as provided by the programmable clock IP) and the oversampling together determine the IrDA bitrate. Normal mode, OVS field values (with the required IP clock frequency): - 0: 16 times oversampling. IP clock frequency of 16*115.2 KHz for 115.2 Kbps. IP clock frequency of 16*57.6 KHz for 57.6 Kbps. IP clock frequency of 16*38.4 KHz for 38.4 Kbps. IP clock frequency of 16*19.2 KHz for 19.2 Kbps. IP clock frequency of 16*9.6 KHz for 9.6 Kbps. IP clock frequency of 16*2.4 KHz for 2.4 Kbps. IP clock frequency of 16*1.2 KHz for 1.2 Kbps. - all other values are not used in normal mode. Low power mode, OVS field values (with the required IP clock frequency): - 0: 16 times oversampling. IP clock frequency of 16*115.2 KHz for 115.2 Kbps. - 1: 32 times oversampling. IP clock frequency of 32*57.6 KHz for 57.6 Kbps. - 2: 48 times oversampling. IP clock frequency of 48*38.4 KHz for 38.4 Kbps. - 3: 96 times oversampling. IP clock frequency of 96*19.2 KHz for 19.2 Kbps. - 4: 192 times oversampling. IP clock frequency of 192*9.6 KHz for 9.6 Kbps. - 5: 768 times oversampling. IP clock frequency of 768*2.4 KHz for 2.4 Kbps. - 6: 1536 times oversampling. IP clock frequency of 1536*1.2 KHz for 1.2 Kbps. - all other values are not used in low power mode. Default Value: 15"]
    #[inline(always)]
    pub fn ovs(&mut self) -> OVS_W {
        OVS_W::new(self)
    }
    #[doc = "Bit 8 - Internally clocked mode ('0') or externally clocked mode ('1') address matching (I2C) or selection (SPI). In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. In UART mode this field should be '0'."]
    #[inline(always)]
    pub fn ec_am_mode(&mut self) -> EC_AM_MODE_W {
        EC_AM_MODE_W::new(self)
    }
    #[doc = "Bit 9 - Internally clocked mode ('0') or externally clocked mode ('1') operation. In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked operation mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode AND EZ mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. The maximum SPI slave, EZ mode bitrate is 48 Mbps (transmission and IO delays outside the IP will degrade the effective bitrate). In UART mode this field should be '0'."]
    #[inline(always)]
    pub fn ec_op_mode(&mut self) -> EC_OP_MODE_W {
        EC_OP_MODE_W::new(self)
    }
    #[doc = "Bit 10 - Non EZ mode ('0') or EZ mode ('1'). In EZ mode, a meta protocol is applied to the serial interface protocol. This meta protocol adds meaning to the data frames transferred by the serial interface protocol: a data frame can represent a memory address, a write memory data element or a read memory data element. EZ mode is only used for synchronous serial interface protocols: SPI and I2C. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported and the transmitter should use continuous data frames; i.e. data frames mot seperated by slave deselection. This mode is only applicable to slave functionality. In EZ mode, the slave can read from and write to an addressable memory structure of 32 bytes. In EZ mode, data frames should 8-bit in size and should be transmitted and received with the Most Significant Bit (MSB) first. In UART mode this field should be '0'."]
    #[inline(always)]
    pub fn ez_mode(&mut self) -> EZ_MODE_W {
        EZ_MODE_W::new(self)
    }
    #[doc = "Bit 11 - Determines the number of bits per FIFO data element: '0': 16-bit FIFO data elements. '1': 8-bit FIFO data elements. This mode doubles the amount of FIFO entries, but TX_CTRL.DATA_WIDTH and RX_CTRL.DATA_WIDTH are restricted to \\[0, 7\\]."]
    #[inline(always)]
    pub fn byte_mode(&mut self) -> BYTE_MODE_W {
        BYTE_MODE_W::new(self)
    }
    #[doc = "Bit 12 - Determines CMD_RESP mode of operation: '0': CMD_RESP mode disabled. '1': CMD_RESP mode enabled (also requires EC_AM_MODE and EC_OP_MODE to be set to '1')."]
    #[inline(always)]
    pub fn cmd_resp_mode(&mut self) -> CMD_RESP_MODE_W {
        CMD_RESP_MODE_W::new(self)
    }
    #[doc = "Bit 16 - Determines whether a received matching address is accepted in the RX FIFO ('1') or not ('0'). In I2C mode, this field is used to allow the slave to put the received slave address or general call address in the RX FIFO. Note that a received matching address is put in the RX FIFO when ADDR_ACCEPT is '1' for both I2C read and write transfers. In multi-processor UART receiver mode, this field is used to allow the receiver to put the received address in the RX FIFO. Note: non-matching addresses are never put in the RX FIFO."]
    #[inline(always)]
    pub fn addr_accept(&mut self) -> ADDR_ACCEPT_W {
        ADDR_ACCEPT_W::new(self)
    }
    #[doc = "Bit 17 - Only used in externally clocked mode. If the externally clocked logic and the MMIO SW accesses to EZ memory coincide/collide, this bit determines whether a SW access should block and result in bus wait states ('BLOCK is 1') or not (BLOCK is '0'). IF BLOCK is '0' and the accesses collide, MMIO read operations return 0xffff:ffff and MMIO write operations are ignored. Colliding accesses are registered as interrupt causes: field BLOCKED of MMIO registers INTR_TX and INTR_RX."]
    #[inline(always)]
    pub fn block(&mut self) -> BLOCK_W {
        BLOCK_W::new(self)
    }
    #[doc = "Bits 24:25 - Mode of operation (3: Reserved) Default Value: 3 0x0: I2C : Inter-Integrated Circuits (I2C) mode. 0x1: SPI : Serial Peripheral Interface (SPI) mode. 0x2: UART : Universal Asynchronous Receiver/Transmitter (UART) mode."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W::new(self)
    }
    #[doc = "Bit 31 - IP enabled ('1') or not ('0'). The proper order in which to initialize the IP is as follows: - Program protocol specific information using SPI_CTRL, UART_CTRL (and UART_TX_CTRL and UART_RX_CTRL) or I2C_CTRL. This includes selection of a submode, master/slave functionality and transmitter/receiver functionality when applicable. - Program generic transmitter (TX_CTRL) and receiver (RX_CTRL) information. This includes enabling of the transmitter and receiver functionality. - Program transmitter FIFO (TX_FIFO_CTRL) and receiver FIFO (RX_FIFO_CTRL) information. - Program CTRL to enable IP, select the specific operation mode and oversampling factor. When the IP is enabled, no control information should be changed. Changes should be made AFTER disabling the IP, e.g. to modify the operation mode (from I2C to SPI) or to go from externally to internally clocked. The change takes effect after the IP is re-enabled. Note that disabling the IP will cause re-initialization of the design and associated state is lost (e.g. FIFO content)."]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W {
        ENABLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Generic control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0x0300_000f"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0300_000f
    }
}
