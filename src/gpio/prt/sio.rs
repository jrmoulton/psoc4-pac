#[doc = "Register `SIO` reader"]
pub struct R(crate::R<SIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIO` writer"]
pub struct W(crate::W<SIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIO_SPEC>;
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
impl From<crate::W<SIO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAIR_VREG01_EN` reader - Selects output buffer mode: 0: unregulated output buffer 1: regulated output buffer"]
pub type PAIR_VREG01_EN_R = crate::BitReader<bool>;
#[doc = "Field `PAIR_VREG01_EN` writer - Selects output buffer mode: 0: unregulated output buffer 1: regulated output buffer"]
pub type PAIR_VREG01_EN_W<'a> = crate::BitWriter<'a, u32, SIO_SPEC, bool, 0>;
#[doc = "Field `PAIR_IBUF01_SEL` reader - Selects input buffer mode: 0: singled ended input buffer 1: differential input buffer"]
pub type PAIR_IBUF01_SEL_R = crate::BitReader<bool>;
#[doc = "Field `PAIR_IBUF01_SEL` writer - Selects input buffer mode: 0: singled ended input buffer 1: differential input buffer"]
pub type PAIR_IBUF01_SEL_W<'a> = crate::BitWriter<'a, u32, SIO_SPEC, bool, 1>;
#[doc = "Field `PAIR_VTRIP01_SEL` reader - Selects trip-point of input buffer. In single ended input buffer mode (IBUF01_SEL = '0'): 0: input buffer functions as a CMOS input buffer. 1: input buffer functions as a LVTTL input buffer. In differential input buffer mode (IBUF01_SEL = '1') '0': trip-point is 0.5*Vddio or 0.5*Voh (depends on VREF_SEL/VOH_SEL) '1': trip-point is 0.4*Vddio or 1.0*Vref (depends on VREF_SEL) Please refer to s8iom0s8 BROS 001-70428, section 4.2.7 for more details."]
pub type PAIR_VTRIP01_SEL_R = crate::BitReader<bool>;
#[doc = "Field `PAIR_VTRIP01_SEL` writer - Selects trip-point of input buffer. In single ended input buffer mode (IBUF01_SEL = '0'): 0: input buffer functions as a CMOS input buffer. 1: input buffer functions as a LVTTL input buffer. In differential input buffer mode (IBUF01_SEL = '1') '0': trip-point is 0.5*Vddio or 0.5*Voh (depends on VREF_SEL/VOH_SEL) '1': trip-point is 0.4*Vddio or 1.0*Vref (depends on VREF_SEL) Please refer to s8iom0s8 BROS 001-70428, section 4.2.7 for more details."]
pub type PAIR_VTRIP01_SEL_W<'a> = crate::BitWriter<'a, u32, SIO_SPEC, bool, 2>;
#[doc = "Field `PAIR_VREF01_SEL` reader - Selects reference voltage Vref for trip-point of input buffer: 0: trip-point reference of SRSS internal referece Vref (1.2V) 1: trip-point reference of SRSS internal referece Vref (1.2V) 2: trip-point reference of AMUXBUS_A 3: trip-point reference of AMUXBUS_B Please refer to s8iom0s8 BROS 001-70428, section 4.2.7 for more details."]
pub type PAIR_VREF01_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAIR_VREF01_SEL` writer - Selects reference voltage Vref for trip-point of input buffer: 0: trip-point reference of SRSS internal referece Vref (1.2V) 1: trip-point reference of SRSS internal referece Vref (1.2V) 2: trip-point reference of AMUXBUS_A 3: trip-point reference of AMUXBUS_B Please refer to s8iom0s8 BROS 001-70428, section 4.2.7 for more details."]
pub type PAIR_VREF01_SEL_W<'a> = crate::FieldWriter<'a, u32, SIO_SPEC, u8, u8, 2, 3>;
#[doc = "Field `PAIR_VOH01_SEL` reader - Selects regulated Voh output level and trip point of input buffer for a specific SIO pin pair. Voh depends on the selected reference voltage (VREF_SEL). 0: Voh = 1*reference; e.g. reference at 1.2V -> Voh = 1.2V 1: Voh = 1.25*reference; e.g. reference at 1.2V -> Voh = 1.5V 2: Voh = 1.49*reference; e.g. reference at 1.2V -> Voh = ~1.8V 3: Voh = 1.67*reference; e.g. reference at 1.2V -> Voh = 2V 4: Voh = 2.08*reference; e.g. reference at 1.2V -> Voh = 2.5V 5: Voh = 2.5*reference; e.g. reference at 1.2V -> Voh = 3V 6: Voh = 2.78*reference; e.g. reference at 1.2V -> Voh = ~3.3V 7: Voh = 4.16*reference; e.g. reference at 1.2V -> Voh = 5.0V Note: The upper value on VOH is limited to Vddio - 400mV"]
pub type PAIR_VOH01_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAIR_VOH01_SEL` writer - Selects regulated Voh output level and trip point of input buffer for a specific SIO pin pair. Voh depends on the selected reference voltage (VREF_SEL). 0: Voh = 1*reference; e.g. reference at 1.2V -> Voh = 1.2V 1: Voh = 1.25*reference; e.g. reference at 1.2V -> Voh = 1.5V 2: Voh = 1.49*reference; e.g. reference at 1.2V -> Voh = ~1.8V 3: Voh = 1.67*reference; e.g. reference at 1.2V -> Voh = 2V 4: Voh = 2.08*reference; e.g. reference at 1.2V -> Voh = 2.5V 5: Voh = 2.5*reference; e.g. reference at 1.2V -> Voh = 3V 6: Voh = 2.78*reference; e.g. reference at 1.2V -> Voh = ~3.3V 7: Voh = 4.16*reference; e.g. reference at 1.2V -> Voh = 5.0V Note: The upper value on VOH is limited to Vddio - 400mV"]
pub type PAIR_VOH01_SEL_W<'a> = crate::FieldWriter<'a, u32, SIO_SPEC, u8, u8, 3, 5>;
#[doc = "Field `PAIR_VREG23_EN` reader - See corresponding definition for IO pads 2 and 3."]
pub type PAIR_VREG23_EN_R = crate::BitReader<bool>;
#[doc = "Field `PAIR_VREG23_EN` writer - See corresponding definition for IO pads 2 and 3."]
pub type PAIR_VREG23_EN_W<'a> = crate::BitWriter<'a, u32, SIO_SPEC, bool, 8>;
#[doc = "Field `PAIR_IBUF23_SEL` reader - See corresponding definition for IO pads 2 and 3."]
pub type PAIR_IBUF23_SEL_R = crate::BitReader<bool>;
#[doc = "Field `PAIR_IBUF23_SEL` writer - See corresponding definition for IO pads 2 and 3."]
pub type PAIR_IBUF23_SEL_W<'a> = crate::BitWriter<'a, u32, SIO_SPEC, bool, 9>;
#[doc = "Field `PAIR_VTRIP23_SEL` reader - See corresponding definition for IO pads 2 and 3."]
pub type PAIR_VTRIP23_SEL_R = crate::BitReader<bool>;
#[doc = "Field `PAIR_VTRIP23_SEL` writer - See corresponding definition for IO pads 2 and 3."]
pub type PAIR_VTRIP23_SEL_W<'a> = crate::BitWriter<'a, u32, SIO_SPEC, bool, 10>;
#[doc = "Field `PAIR_VREF23_SEL` reader - See corresponding definition for IO pads 2 and 3."]
pub type PAIR_VREF23_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAIR_VREF23_SEL` writer - See corresponding definition for IO pads 2 and 3."]
pub type PAIR_VREF23_SEL_W<'a> = crate::FieldWriter<'a, u32, SIO_SPEC, u8, u8, 2, 11>;
#[doc = "Field `PAIR_VOH23_SEL` reader - See corresponding definition for IO pads 2 and 3."]
pub type PAIR_VOH23_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAIR_VOH23_SEL` writer - See corresponding definition for IO pads 2 and 3."]
pub type PAIR_VOH23_SEL_W<'a> = crate::FieldWriter<'a, u32, SIO_SPEC, u8, u8, 3, 13>;
#[doc = "Field `PAIR_VREG45_EN` reader - See corresponding definition for IO pads 4 and 5."]
pub type PAIR_VREG45_EN_R = crate::BitReader<bool>;
#[doc = "Field `PAIR_VREG45_EN` writer - See corresponding definition for IO pads 4 and 5."]
pub type PAIR_VREG45_EN_W<'a> = crate::BitWriter<'a, u32, SIO_SPEC, bool, 16>;
#[doc = "Field `PAIR_IBUF45_SEL` reader - See corresponding definition for IO pads 4 and 5."]
pub type PAIR_IBUF45_SEL_R = crate::BitReader<bool>;
#[doc = "Field `PAIR_IBUF45_SEL` writer - See corresponding definition for IO pads 4 and 5."]
pub type PAIR_IBUF45_SEL_W<'a> = crate::BitWriter<'a, u32, SIO_SPEC, bool, 17>;
#[doc = "Field `PAIR_VTRIP45_SEL` reader - See corresponding definition for IO pads 4 and 5."]
pub type PAIR_VTRIP45_SEL_R = crate::BitReader<bool>;
#[doc = "Field `PAIR_VTRIP45_SEL` writer - See corresponding definition for IO pads 4 and 5."]
pub type PAIR_VTRIP45_SEL_W<'a> = crate::BitWriter<'a, u32, SIO_SPEC, bool, 18>;
#[doc = "Field `PAIR_VREF45_SEL` reader - See corresponding definition for IO pads 4 and 5."]
pub type PAIR_VREF45_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAIR_VREF45_SEL` writer - See corresponding definition for IO pads 4 and 5."]
pub type PAIR_VREF45_SEL_W<'a> = crate::FieldWriter<'a, u32, SIO_SPEC, u8, u8, 2, 19>;
#[doc = "Field `PAIR_VOH45_SEL` reader - See corresponding definition for IO pads 4 and 5."]
pub type PAIR_VOH45_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAIR_VOH45_SEL` writer - See corresponding definition for IO pads 4 and 5."]
pub type PAIR_VOH45_SEL_W<'a> = crate::FieldWriter<'a, u32, SIO_SPEC, u8, u8, 3, 21>;
#[doc = "Field `PAIR_VREG67_EN` reader - See corresponding definition for IO pads 6 and 7."]
pub type PAIR_VREG67_EN_R = crate::BitReader<bool>;
#[doc = "Field `PAIR_VREG67_EN` writer - See corresponding definition for IO pads 6 and 7."]
pub type PAIR_VREG67_EN_W<'a> = crate::BitWriter<'a, u32, SIO_SPEC, bool, 24>;
#[doc = "Field `PAIR_IBUF67_SEL` reader - See corresponding definition for IO pads 6 and 7."]
pub type PAIR_IBUF67_SEL_R = crate::BitReader<bool>;
#[doc = "Field `PAIR_IBUF67_SEL` writer - See corresponding definition for IO pads 6 and 7."]
pub type PAIR_IBUF67_SEL_W<'a> = crate::BitWriter<'a, u32, SIO_SPEC, bool, 25>;
#[doc = "Field `PAIR_VTRIP67_SEL` reader - See corresponding definition for IO pads 6 and 7."]
pub type PAIR_VTRIP67_SEL_R = crate::BitReader<bool>;
#[doc = "Field `PAIR_VTRIP67_SEL` writer - See corresponding definition for IO pads 6 and 7."]
pub type PAIR_VTRIP67_SEL_W<'a> = crate::BitWriter<'a, u32, SIO_SPEC, bool, 26>;
#[doc = "Field `PAIR_VREF67_SEL` reader - See corresponding definition for IO pads 6 and 7."]
pub type PAIR_VREF67_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAIR_VREF67_SEL` writer - See corresponding definition for IO pads 6 and 7."]
pub type PAIR_VREF67_SEL_W<'a> = crate::FieldWriter<'a, u32, SIO_SPEC, u8, u8, 2, 27>;
#[doc = "Field `PAIR_VOH67_SEL` reader - See corresponding definition for IO pads 6 and 7."]
pub type PAIR_VOH67_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAIR_VOH67_SEL` writer - See corresponding definition for IO pads 6 and 7."]
pub type PAIR_VOH67_SEL_W<'a> = crate::FieldWriter<'a, u32, SIO_SPEC, u8, u8, 3, 29>;
impl R {
    #[doc = "Bit 0 - Selects output buffer mode: 0: unregulated output buffer 1: regulated output buffer"]
    #[inline(always)]
    pub fn pair_vreg01_en(&self) -> PAIR_VREG01_EN_R {
        PAIR_VREG01_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selects input buffer mode: 0: singled ended input buffer 1: differential input buffer"]
    #[inline(always)]
    pub fn pair_ibuf01_sel(&self) -> PAIR_IBUF01_SEL_R {
        PAIR_IBUF01_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Selects trip-point of input buffer. In single ended input buffer mode (IBUF01_SEL = '0'): 0: input buffer functions as a CMOS input buffer. 1: input buffer functions as a LVTTL input buffer. In differential input buffer mode (IBUF01_SEL = '1') '0': trip-point is 0.5*Vddio or 0.5*Voh (depends on VREF_SEL/VOH_SEL) '1': trip-point is 0.4*Vddio or 1.0*Vref (depends on VREF_SEL) Please refer to s8iom0s8 BROS 001-70428, section 4.2.7 for more details."]
    #[inline(always)]
    pub fn pair_vtrip01_sel(&self) -> PAIR_VTRIP01_SEL_R {
        PAIR_VTRIP01_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Selects reference voltage Vref for trip-point of input buffer: 0: trip-point reference of SRSS internal referece Vref (1.2V) 1: trip-point reference of SRSS internal referece Vref (1.2V) 2: trip-point reference of AMUXBUS_A 3: trip-point reference of AMUXBUS_B Please refer to s8iom0s8 BROS 001-70428, section 4.2.7 for more details."]
    #[inline(always)]
    pub fn pair_vref01_sel(&self) -> PAIR_VREF01_SEL_R {
        PAIR_VREF01_SEL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - Selects regulated Voh output level and trip point of input buffer for a specific SIO pin pair. Voh depends on the selected reference voltage (VREF_SEL). 0: Voh = 1*reference; e.g. reference at 1.2V -> Voh = 1.2V 1: Voh = 1.25*reference; e.g. reference at 1.2V -> Voh = 1.5V 2: Voh = 1.49*reference; e.g. reference at 1.2V -> Voh = ~1.8V 3: Voh = 1.67*reference; e.g. reference at 1.2V -> Voh = 2V 4: Voh = 2.08*reference; e.g. reference at 1.2V -> Voh = 2.5V 5: Voh = 2.5*reference; e.g. reference at 1.2V -> Voh = 3V 6: Voh = 2.78*reference; e.g. reference at 1.2V -> Voh = ~3.3V 7: Voh = 4.16*reference; e.g. reference at 1.2V -> Voh = 5.0V Note: The upper value on VOH is limited to Vddio - 400mV"]
    #[inline(always)]
    pub fn pair_voh01_sel(&self) -> PAIR_VOH01_SEL_R {
        PAIR_VOH01_SEL_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - See corresponding definition for IO pads 2 and 3."]
    #[inline(always)]
    pub fn pair_vreg23_en(&self) -> PAIR_VREG23_EN_R {
        PAIR_VREG23_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - See corresponding definition for IO pads 2 and 3."]
    #[inline(always)]
    pub fn pair_ibuf23_sel(&self) -> PAIR_IBUF23_SEL_R {
        PAIR_IBUF23_SEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - See corresponding definition for IO pads 2 and 3."]
    #[inline(always)]
    pub fn pair_vtrip23_sel(&self) -> PAIR_VTRIP23_SEL_R {
        PAIR_VTRIP23_SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - See corresponding definition for IO pads 2 and 3."]
    #[inline(always)]
    pub fn pair_vref23_sel(&self) -> PAIR_VREF23_SEL_R {
        PAIR_VREF23_SEL_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:15 - See corresponding definition for IO pads 2 and 3."]
    #[inline(always)]
    pub fn pair_voh23_sel(&self) -> PAIR_VOH23_SEL_R {
        PAIR_VOH23_SEL_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - See corresponding definition for IO pads 4 and 5."]
    #[inline(always)]
    pub fn pair_vreg45_en(&self) -> PAIR_VREG45_EN_R {
        PAIR_VREG45_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - See corresponding definition for IO pads 4 and 5."]
    #[inline(always)]
    pub fn pair_ibuf45_sel(&self) -> PAIR_IBUF45_SEL_R {
        PAIR_IBUF45_SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - See corresponding definition for IO pads 4 and 5."]
    #[inline(always)]
    pub fn pair_vtrip45_sel(&self) -> PAIR_VTRIP45_SEL_R {
        PAIR_VTRIP45_SEL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - See corresponding definition for IO pads 4 and 5."]
    #[inline(always)]
    pub fn pair_vref45_sel(&self) -> PAIR_VREF45_SEL_R {
        PAIR_VREF45_SEL_R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bits 21:23 - See corresponding definition for IO pads 4 and 5."]
    #[inline(always)]
    pub fn pair_voh45_sel(&self) -> PAIR_VOH45_SEL_R {
        PAIR_VOH45_SEL_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - See corresponding definition for IO pads 6 and 7."]
    #[inline(always)]
    pub fn pair_vreg67_en(&self) -> PAIR_VREG67_EN_R {
        PAIR_VREG67_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - See corresponding definition for IO pads 6 and 7."]
    #[inline(always)]
    pub fn pair_ibuf67_sel(&self) -> PAIR_IBUF67_SEL_R {
        PAIR_IBUF67_SEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - See corresponding definition for IO pads 6 and 7."]
    #[inline(always)]
    pub fn pair_vtrip67_sel(&self) -> PAIR_VTRIP67_SEL_R {
        PAIR_VTRIP67_SEL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - See corresponding definition for IO pads 6 and 7."]
    #[inline(always)]
    pub fn pair_vref67_sel(&self) -> PAIR_VREF67_SEL_R {
        PAIR_VREF67_SEL_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bits 29:31 - See corresponding definition for IO pads 6 and 7."]
    #[inline(always)]
    pub fn pair_voh67_sel(&self) -> PAIR_VOH67_SEL_R {
        PAIR_VOH67_SEL_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Selects output buffer mode: 0: unregulated output buffer 1: regulated output buffer"]
    #[inline(always)]
    pub fn pair_vreg01_en(&mut self) -> PAIR_VREG01_EN_W {
        PAIR_VREG01_EN_W::new(self)
    }
    #[doc = "Bit 1 - Selects input buffer mode: 0: singled ended input buffer 1: differential input buffer"]
    #[inline(always)]
    pub fn pair_ibuf01_sel(&mut self) -> PAIR_IBUF01_SEL_W {
        PAIR_IBUF01_SEL_W::new(self)
    }
    #[doc = "Bit 2 - Selects trip-point of input buffer. In single ended input buffer mode (IBUF01_SEL = '0'): 0: input buffer functions as a CMOS input buffer. 1: input buffer functions as a LVTTL input buffer. In differential input buffer mode (IBUF01_SEL = '1') '0': trip-point is 0.5*Vddio or 0.5*Voh (depends on VREF_SEL/VOH_SEL) '1': trip-point is 0.4*Vddio or 1.0*Vref (depends on VREF_SEL) Please refer to s8iom0s8 BROS 001-70428, section 4.2.7 for more details."]
    #[inline(always)]
    pub fn pair_vtrip01_sel(&mut self) -> PAIR_VTRIP01_SEL_W {
        PAIR_VTRIP01_SEL_W::new(self)
    }
    #[doc = "Bits 3:4 - Selects reference voltage Vref for trip-point of input buffer: 0: trip-point reference of SRSS internal referece Vref (1.2V) 1: trip-point reference of SRSS internal referece Vref (1.2V) 2: trip-point reference of AMUXBUS_A 3: trip-point reference of AMUXBUS_B Please refer to s8iom0s8 BROS 001-70428, section 4.2.7 for more details."]
    #[inline(always)]
    pub fn pair_vref01_sel(&mut self) -> PAIR_VREF01_SEL_W {
        PAIR_VREF01_SEL_W::new(self)
    }
    #[doc = "Bits 5:7 - Selects regulated Voh output level and trip point of input buffer for a specific SIO pin pair. Voh depends on the selected reference voltage (VREF_SEL). 0: Voh = 1*reference; e.g. reference at 1.2V -> Voh = 1.2V 1: Voh = 1.25*reference; e.g. reference at 1.2V -> Voh = 1.5V 2: Voh = 1.49*reference; e.g. reference at 1.2V -> Voh = ~1.8V 3: Voh = 1.67*reference; e.g. reference at 1.2V -> Voh = 2V 4: Voh = 2.08*reference; e.g. reference at 1.2V -> Voh = 2.5V 5: Voh = 2.5*reference; e.g. reference at 1.2V -> Voh = 3V 6: Voh = 2.78*reference; e.g. reference at 1.2V -> Voh = ~3.3V 7: Voh = 4.16*reference; e.g. reference at 1.2V -> Voh = 5.0V Note: The upper value on VOH is limited to Vddio - 400mV"]
    #[inline(always)]
    pub fn pair_voh01_sel(&mut self) -> PAIR_VOH01_SEL_W {
        PAIR_VOH01_SEL_W::new(self)
    }
    #[doc = "Bit 8 - See corresponding definition for IO pads 2 and 3."]
    #[inline(always)]
    pub fn pair_vreg23_en(&mut self) -> PAIR_VREG23_EN_W {
        PAIR_VREG23_EN_W::new(self)
    }
    #[doc = "Bit 9 - See corresponding definition for IO pads 2 and 3."]
    #[inline(always)]
    pub fn pair_ibuf23_sel(&mut self) -> PAIR_IBUF23_SEL_W {
        PAIR_IBUF23_SEL_W::new(self)
    }
    #[doc = "Bit 10 - See corresponding definition for IO pads 2 and 3."]
    #[inline(always)]
    pub fn pair_vtrip23_sel(&mut self) -> PAIR_VTRIP23_SEL_W {
        PAIR_VTRIP23_SEL_W::new(self)
    }
    #[doc = "Bits 11:12 - See corresponding definition for IO pads 2 and 3."]
    #[inline(always)]
    pub fn pair_vref23_sel(&mut self) -> PAIR_VREF23_SEL_W {
        PAIR_VREF23_SEL_W::new(self)
    }
    #[doc = "Bits 13:15 - See corresponding definition for IO pads 2 and 3."]
    #[inline(always)]
    pub fn pair_voh23_sel(&mut self) -> PAIR_VOH23_SEL_W {
        PAIR_VOH23_SEL_W::new(self)
    }
    #[doc = "Bit 16 - See corresponding definition for IO pads 4 and 5."]
    #[inline(always)]
    pub fn pair_vreg45_en(&mut self) -> PAIR_VREG45_EN_W {
        PAIR_VREG45_EN_W::new(self)
    }
    #[doc = "Bit 17 - See corresponding definition for IO pads 4 and 5."]
    #[inline(always)]
    pub fn pair_ibuf45_sel(&mut self) -> PAIR_IBUF45_SEL_W {
        PAIR_IBUF45_SEL_W::new(self)
    }
    #[doc = "Bit 18 - See corresponding definition for IO pads 4 and 5."]
    #[inline(always)]
    pub fn pair_vtrip45_sel(&mut self) -> PAIR_VTRIP45_SEL_W {
        PAIR_VTRIP45_SEL_W::new(self)
    }
    #[doc = "Bits 19:20 - See corresponding definition for IO pads 4 and 5."]
    #[inline(always)]
    pub fn pair_vref45_sel(&mut self) -> PAIR_VREF45_SEL_W {
        PAIR_VREF45_SEL_W::new(self)
    }
    #[doc = "Bits 21:23 - See corresponding definition for IO pads 4 and 5."]
    #[inline(always)]
    pub fn pair_voh45_sel(&mut self) -> PAIR_VOH45_SEL_W {
        PAIR_VOH45_SEL_W::new(self)
    }
    #[doc = "Bit 24 - See corresponding definition for IO pads 6 and 7."]
    #[inline(always)]
    pub fn pair_vreg67_en(&mut self) -> PAIR_VREG67_EN_W {
        PAIR_VREG67_EN_W::new(self)
    }
    #[doc = "Bit 25 - See corresponding definition for IO pads 6 and 7."]
    #[inline(always)]
    pub fn pair_ibuf67_sel(&mut self) -> PAIR_IBUF67_SEL_W {
        PAIR_IBUF67_SEL_W::new(self)
    }
    #[doc = "Bit 26 - See corresponding definition for IO pads 6 and 7."]
    #[inline(always)]
    pub fn pair_vtrip67_sel(&mut self) -> PAIR_VTRIP67_SEL_W {
        PAIR_VTRIP67_SEL_W::new(self)
    }
    #[doc = "Bits 27:28 - See corresponding definition for IO pads 6 and 7."]
    #[inline(always)]
    pub fn pair_vref67_sel(&mut self) -> PAIR_VREF67_SEL_W {
        PAIR_VREF67_SEL_W::new(self)
    }
    #[doc = "Bits 29:31 - See corresponding definition for IO pads 6 and 7."]
    #[inline(always)]
    pub fn pair_voh67_sel(&mut self) -> PAIR_VOH67_SEL_W {
        PAIR_VOH67_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port SIO configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sio](index.html) module"]
pub struct SIO_SPEC;
impl crate::RegisterSpec for SIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sio::R](R) reader structure"]
impl crate::Readable for SIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sio::W](W) writer structure"]
impl crate::Writable for SIO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SIO to value 0"]
impl crate::Resettable for SIO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
