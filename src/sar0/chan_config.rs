#[doc = "Register `CHAN_CONFIG[%s]` reader"]
pub struct R(crate::R<CHAN_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHAN_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHAN_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHAN_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHAN_CONFIG[%s]` writer"]
pub struct W(crate::W<CHAN_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHAN_CONFIG_SPEC>;
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
impl From<crate::W<CHAN_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHAN_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIN_ADDR` reader - Address of the pin to be sampled by this channel. If differential is enabled then PIN_ADDR\\[0\\]
is ignored and considered to be 0, i.e. PIN_ADDR points to the even pin of a pin pair. For differential the even pin of the pair is connected to Vplus and the odd pin of the pair is connected to Vminus."]
pub type PIN_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN_ADDR` writer - Address of the pin to be sampled by this channel. If differential is enabled then PIN_ADDR\\[0\\]
is ignored and considered to be 0, i.e. PIN_ADDR points to the even pin of a pin pair. For differential the even pin of the pair is connected to Vplus and the odd pin of the pair is connected to Vminus."]
pub type PIN_ADDR_W<'a> = crate::FieldWriter<'a, u32, CHAN_CONFIG_SPEC, u8, u8, 3, 0>;
#[doc = "Address of the port that contains the pin to be sampled by this channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PORT_ADDR_A {
    #[doc = "0: SARMUX pins."]
    SARMUX = 0,
    #[doc = "1: CTB0"]
    CTB0 = 1,
    #[doc = "2: CTB1"]
    CTB1 = 2,
    #[doc = "3: CTB2"]
    CTB2 = 3,
    #[doc = "4: CTB3"]
    CTB3 = 4,
    #[doc = "5: AROUTE virtual port2 (VPORT2)"]
    AROUTE_VIRT2 = 5,
    #[doc = "6: AROUTE virtual port1 (VPORT1)"]
    AROUTE_VIRT1 = 6,
    #[doc = "7: SARMUX virtual port (VPORT0)"]
    SARMUX_VIRT = 7,
}
impl From<PORT_ADDR_A> for u8 {
    #[inline(always)]
    fn from(variant: PORT_ADDR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PORT_ADDR` reader - Address of the port that contains the pin to be sampled by this channel."]
pub type PORT_ADDR_R = crate::FieldReader<u8, PORT_ADDR_A>;
impl PORT_ADDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT_ADDR_A {
        match self.bits {
            0 => PORT_ADDR_A::SARMUX,
            1 => PORT_ADDR_A::CTB0,
            2 => PORT_ADDR_A::CTB1,
            3 => PORT_ADDR_A::CTB2,
            4 => PORT_ADDR_A::CTB3,
            5 => PORT_ADDR_A::AROUTE_VIRT2,
            6 => PORT_ADDR_A::AROUTE_VIRT1,
            7 => PORT_ADDR_A::SARMUX_VIRT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SARMUX`"]
    #[inline(always)]
    pub fn is_sarmux(&self) -> bool {
        *self == PORT_ADDR_A::SARMUX
    }
    #[doc = "Checks if the value of the field is `CTB0`"]
    #[inline(always)]
    pub fn is_ctb0(&self) -> bool {
        *self == PORT_ADDR_A::CTB0
    }
    #[doc = "Checks if the value of the field is `CTB1`"]
    #[inline(always)]
    pub fn is_ctb1(&self) -> bool {
        *self == PORT_ADDR_A::CTB1
    }
    #[doc = "Checks if the value of the field is `CTB2`"]
    #[inline(always)]
    pub fn is_ctb2(&self) -> bool {
        *self == PORT_ADDR_A::CTB2
    }
    #[doc = "Checks if the value of the field is `CTB3`"]
    #[inline(always)]
    pub fn is_ctb3(&self) -> bool {
        *self == PORT_ADDR_A::CTB3
    }
    #[doc = "Checks if the value of the field is `AROUTE_VIRT2`"]
    #[inline(always)]
    pub fn is_aroute_virt2(&self) -> bool {
        *self == PORT_ADDR_A::AROUTE_VIRT2
    }
    #[doc = "Checks if the value of the field is `AROUTE_VIRT1`"]
    #[inline(always)]
    pub fn is_aroute_virt1(&self) -> bool {
        *self == PORT_ADDR_A::AROUTE_VIRT1
    }
    #[doc = "Checks if the value of the field is `SARMUX_VIRT`"]
    #[inline(always)]
    pub fn is_sarmux_virt(&self) -> bool {
        *self == PORT_ADDR_A::SARMUX_VIRT
    }
}
#[doc = "Field `PORT_ADDR` writer - Address of the port that contains the pin to be sampled by this channel."]
pub type PORT_ADDR_W<'a> = crate::FieldWriterSafe<'a, u32, CHAN_CONFIG_SPEC, u8, PORT_ADDR_A, 3, 4>;
impl<'a> PORT_ADDR_W<'a> {
    #[doc = "SARMUX pins."]
    #[inline(always)]
    pub fn sarmux(self) -> &'a mut W {
        self.variant(PORT_ADDR_A::SARMUX)
    }
    #[doc = "CTB0"]
    #[inline(always)]
    pub fn ctb0(self) -> &'a mut W {
        self.variant(PORT_ADDR_A::CTB0)
    }
    #[doc = "CTB1"]
    #[inline(always)]
    pub fn ctb1(self) -> &'a mut W {
        self.variant(PORT_ADDR_A::CTB1)
    }
    #[doc = "CTB2"]
    #[inline(always)]
    pub fn ctb2(self) -> &'a mut W {
        self.variant(PORT_ADDR_A::CTB2)
    }
    #[doc = "CTB3"]
    #[inline(always)]
    pub fn ctb3(self) -> &'a mut W {
        self.variant(PORT_ADDR_A::CTB3)
    }
    #[doc = "AROUTE virtual port2 (VPORT2)"]
    #[inline(always)]
    pub fn aroute_virt2(self) -> &'a mut W {
        self.variant(PORT_ADDR_A::AROUTE_VIRT2)
    }
    #[doc = "AROUTE virtual port1 (VPORT1)"]
    #[inline(always)]
    pub fn aroute_virt1(self) -> &'a mut W {
        self.variant(PORT_ADDR_A::AROUTE_VIRT1)
    }
    #[doc = "SARMUX virtual port (VPORT0)"]
    #[inline(always)]
    pub fn sarmux_virt(self) -> &'a mut W {
        self.variant(PORT_ADDR_A::SARMUX_VIRT)
    }
}
#[doc = "Field `DIFFERENTIAL_EN` reader - Differential enable for this channel. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (PIN_ADDR\\[0\\]
is ignored)."]
pub type DIFFERENTIAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `DIFFERENTIAL_EN` writer - Differential enable for this channel. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (PIN_ADDR\\[0\\]
is ignored)."]
pub type DIFFERENTIAL_EN_W<'a> = crate::BitWriter<'a, u32, CHAN_CONFIG_SPEC, bool, 8>;
#[doc = "Resolution for this channel. When AVG_EN is set this bit is ignored and always a 12-bit resolution (or highest resolution allowed by wounding) is used for this channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESOLUTION_A {
    #[doc = "0: The maximum resolution is used for this channel (maximum resolution depends on wounding)."]
    MAXRES = 0,
    #[doc = "1: The resolution specified by SUB_RESOLUTION in the SAR_SAMPLE_CTRL register is used for this channel."]
    SUBRES = 1,
}
impl From<RESOLUTION_A> for bool {
    #[inline(always)]
    fn from(variant: RESOLUTION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESOLUTION` reader - Resolution for this channel. When AVG_EN is set this bit is ignored and always a 12-bit resolution (or highest resolution allowed by wounding) is used for this channel."]
pub type RESOLUTION_R = crate::BitReader<RESOLUTION_A>;
impl RESOLUTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESOLUTION_A {
        match self.bits {
            false => RESOLUTION_A::MAXRES,
            true => RESOLUTION_A::SUBRES,
        }
    }
    #[doc = "Checks if the value of the field is `MAXRES`"]
    #[inline(always)]
    pub fn is_maxres(&self) -> bool {
        *self == RESOLUTION_A::MAXRES
    }
    #[doc = "Checks if the value of the field is `SUBRES`"]
    #[inline(always)]
    pub fn is_subres(&self) -> bool {
        *self == RESOLUTION_A::SUBRES
    }
}
#[doc = "Field `RESOLUTION` writer - Resolution for this channel. When AVG_EN is set this bit is ignored and always a 12-bit resolution (or highest resolution allowed by wounding) is used for this channel."]
pub type RESOLUTION_W<'a> = crate::BitWriter<'a, u32, CHAN_CONFIG_SPEC, RESOLUTION_A, 9>;
impl<'a> RESOLUTION_W<'a> {
    #[doc = "The maximum resolution is used for this channel (maximum resolution depends on wounding)."]
    #[inline(always)]
    pub fn maxres(self) -> &'a mut W {
        self.variant(RESOLUTION_A::MAXRES)
    }
    #[doc = "The resolution specified by SUB_RESOLUTION in the SAR_SAMPLE_CTRL register is used for this channel."]
    #[inline(always)]
    pub fn subres(self) -> &'a mut W {
        self.variant(RESOLUTION_A::SUBRES)
    }
}
#[doc = "Field `AVG_EN` reader - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
pub type AVG_EN_R = crate::BitReader<bool>;
#[doc = "Field `AVG_EN` writer - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
pub type AVG_EN_W<'a> = crate::BitWriter<'a, u32, CHAN_CONFIG_SPEC, bool, 10>;
#[doc = "Field `SAMPLE_TIME_SEL` reader - Sample time select: select which of the 4 global sample times to use for this channel"]
pub type SAMPLE_TIME_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMPLE_TIME_SEL` writer - Sample time select: select which of the 4 global sample times to use for this channel"]
pub type SAMPLE_TIME_SEL_W<'a> = crate::FieldWriter<'a, u32, CHAN_CONFIG_SPEC, u8, u8, 2, 12>;
#[doc = "Field `DSI_OUT_EN` reader - DSI data output enable for this channel. - 0: the conversion result for this channel is only stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. - 1: the conversion result for this channel is stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. The same data (same formating), together with the channel number, is sent out on the DSI communication channel for processing in UDBs."]
pub type DSI_OUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `DSI_OUT_EN` writer - DSI data output enable for this channel. - 0: the conversion result for this channel is only stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. - 1: the conversion result for this channel is stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. The same data (same formating), together with the channel number, is sent out on the DSI communication channel for processing in UDBs."]
pub type DSI_OUT_EN_W<'a> = crate::BitWriter<'a, u32, CHAN_CONFIG_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:2 - Address of the pin to be sampled by this channel. If differential is enabled then PIN_ADDR\\[0\\]
is ignored and considered to be 0, i.e. PIN_ADDR points to the even pin of a pin pair. For differential the even pin of the pair is connected to Vplus and the odd pin of the pair is connected to Vminus."]
    #[inline(always)]
    pub fn pin_addr(&self) -> PIN_ADDR_R {
        PIN_ADDR_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Address of the port that contains the pin to be sampled by this channel."]
    #[inline(always)]
    pub fn port_addr(&self) -> PORT_ADDR_R {
        PORT_ADDR_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Differential enable for this channel. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (PIN_ADDR\\[0\\]
is ignored)."]
    #[inline(always)]
    pub fn differential_en(&self) -> DIFFERENTIAL_EN_R {
        DIFFERENTIAL_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Resolution for this channel. When AVG_EN is set this bit is ignored and always a 12-bit resolution (or highest resolution allowed by wounding) is used for this channel."]
    #[inline(always)]
    pub fn resolution(&self) -> RESOLUTION_R {
        RESOLUTION_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
    #[inline(always)]
    pub fn avg_en(&self) -> AVG_EN_R {
        AVG_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Sample time select: select which of the 4 global sample times to use for this channel"]
    #[inline(always)]
    pub fn sample_time_sel(&self) -> SAMPLE_TIME_SEL_R {
        SAMPLE_TIME_SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 31 - DSI data output enable for this channel. - 0: the conversion result for this channel is only stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. - 1: the conversion result for this channel is stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. The same data (same formating), together with the channel number, is sent out on the DSI communication channel for processing in UDBs."]
    #[inline(always)]
    pub fn dsi_out_en(&self) -> DSI_OUT_EN_R {
        DSI_OUT_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Address of the pin to be sampled by this channel. If differential is enabled then PIN_ADDR\\[0\\]
is ignored and considered to be 0, i.e. PIN_ADDR points to the even pin of a pin pair. For differential the even pin of the pair is connected to Vplus and the odd pin of the pair is connected to Vminus."]
    #[inline(always)]
    pub fn pin_addr(&mut self) -> PIN_ADDR_W {
        PIN_ADDR_W::new(self)
    }
    #[doc = "Bits 4:6 - Address of the port that contains the pin to be sampled by this channel."]
    #[inline(always)]
    pub fn port_addr(&mut self) -> PORT_ADDR_W {
        PORT_ADDR_W::new(self)
    }
    #[doc = "Bit 8 - Differential enable for this channel. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (PIN_ADDR\\[0\\]
is ignored)."]
    #[inline(always)]
    pub fn differential_en(&mut self) -> DIFFERENTIAL_EN_W {
        DIFFERENTIAL_EN_W::new(self)
    }
    #[doc = "Bit 9 - Resolution for this channel. When AVG_EN is set this bit is ignored and always a 12-bit resolution (or highest resolution allowed by wounding) is used for this channel."]
    #[inline(always)]
    pub fn resolution(&mut self) -> RESOLUTION_W {
        RESOLUTION_W::new(self)
    }
    #[doc = "Bit 10 - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
    #[inline(always)]
    pub fn avg_en(&mut self) -> AVG_EN_W {
        AVG_EN_W::new(self)
    }
    #[doc = "Bits 12:13 - Sample time select: select which of the 4 global sample times to use for this channel"]
    #[inline(always)]
    pub fn sample_time_sel(&mut self) -> SAMPLE_TIME_SEL_W {
        SAMPLE_TIME_SEL_W::new(self)
    }
    #[doc = "Bit 31 - DSI data output enable for this channel. - 0: the conversion result for this channel is only stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. - 1: the conversion result for this channel is stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. The same data (same formating), together with the channel number, is sent out on the DSI communication channel for processing in UDBs."]
    #[inline(always)]
    pub fn dsi_out_en(&mut self) -> DSI_OUT_EN_W {
        DSI_OUT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chan_config](index.html) module"]
pub struct CHAN_CONFIG_SPEC;
impl crate::RegisterSpec for CHAN_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chan_config::R](R) reader structure"]
impl crate::Readable for CHAN_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chan_config::W](W) writer structure"]
impl crate::Writable for CHAN_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHAN_CONFIG[%s]
to value 0"]
impl crate::Resettable for CHAN_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
