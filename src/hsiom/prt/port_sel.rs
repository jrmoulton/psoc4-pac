#[doc = "Register `PORT_SEL` reader"]
pub struct R(crate::R<PORT_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORT_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORT_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORT_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PORT_SEL` writer"]
pub struct W(crate::W<PORT_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORT_SEL_SPEC>;
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
impl From<crate::W<PORT_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORT_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Selects connection for IO pad 0 route.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IO0_SEL_A {
    #[doc = "0: SW controlled GPIO."]
    GPIO = 0,
    #[doc = "1: SW controlled 'out', DSI controlled 'oe_n'."]
    GPIO_DSI = 1,
    #[doc = "2: DSI controlled 'out' and 'oe_n'."]
    DSI_DSI = 2,
    #[doc = "3: DSI controlled 'out', SW controlled 'oe_n'."]
    DSI_GPIO = 3,
    #[doc = "4: CSD sense connection (analog mode)"]
    CSD_SENSE = 4,
    #[doc = "5: CSD shield connection (analog mode)"]
    CSD_SHIELD = 5,
    #[doc = "6: AMUXBUS A connection."]
    AMUXA = 6,
    #[doc = "7: AMUXBUS B connection. This mode is also used for CSD GPIO charging. When CSD GPIO charging is enabled in CSD_CONTROL, 'oe_n' is connected to '!csd_charge' signal (and IO pad is also still connected to AMUXBUS B)."]
    AMUXB = 7,
    #[doc = "8: Chip specific Active source 0 connection."]
    ACT_0 = 8,
    #[doc = "9: Chip specific Active source 1 connection."]
    ACT_1 = 9,
    #[doc = "10: Chip specific Active source 2 connection."]
    ACT_2 = 10,
    #[doc = "11: Chip specific Active source 3 connection."]
    ACT_3 = 11,
    #[doc = "12: LCD common connection. This mode provides DeepSleep functionality (provided that the LCD block is enabled and properly configured)."]
    LCD_COM = 12,
    #[doc = "13: LCD segment connection. This mode provides DeepSleep functionality (provided that the LCD block is enabled and properly configured)."]
    LCD_SEG = 13,
    #[doc = "14: Chip specific DeepSleep source 2 connection."]
    DS_2 = 14,
    #[doc = "15: Chip specific DeepSleep source 3 connection."]
    DS_3 = 15,
}
impl From<IO0_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: IO0_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IO0_SEL` reader - Selects connection for IO pad 0 route."]
pub type IO0_SEL_R = crate::FieldReader<u8, IO0_SEL_A>;
impl IO0_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO0_SEL_A {
        match self.bits {
            0 => IO0_SEL_A::GPIO,
            1 => IO0_SEL_A::GPIO_DSI,
            2 => IO0_SEL_A::DSI_DSI,
            3 => IO0_SEL_A::DSI_GPIO,
            4 => IO0_SEL_A::CSD_SENSE,
            5 => IO0_SEL_A::CSD_SHIELD,
            6 => IO0_SEL_A::AMUXA,
            7 => IO0_SEL_A::AMUXB,
            8 => IO0_SEL_A::ACT_0,
            9 => IO0_SEL_A::ACT_1,
            10 => IO0_SEL_A::ACT_2,
            11 => IO0_SEL_A::ACT_3,
            12 => IO0_SEL_A::LCD_COM,
            13 => IO0_SEL_A::LCD_SEG,
            14 => IO0_SEL_A::DS_2,
            15 => IO0_SEL_A::DS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == IO0_SEL_A::GPIO
    }
    #[doc = "Checks if the value of the field is `GPIO_DSI`"]
    #[inline(always)]
    pub fn is_gpio_dsi(&self) -> bool {
        *self == IO0_SEL_A::GPIO_DSI
    }
    #[doc = "Checks if the value of the field is `DSI_DSI`"]
    #[inline(always)]
    pub fn is_dsi_dsi(&self) -> bool {
        *self == IO0_SEL_A::DSI_DSI
    }
    #[doc = "Checks if the value of the field is `DSI_GPIO`"]
    #[inline(always)]
    pub fn is_dsi_gpio(&self) -> bool {
        *self == IO0_SEL_A::DSI_GPIO
    }
    #[doc = "Checks if the value of the field is `CSD_SENSE`"]
    #[inline(always)]
    pub fn is_csd_sense(&self) -> bool {
        *self == IO0_SEL_A::CSD_SENSE
    }
    #[doc = "Checks if the value of the field is `CSD_SHIELD`"]
    #[inline(always)]
    pub fn is_csd_shield(&self) -> bool {
        *self == IO0_SEL_A::CSD_SHIELD
    }
    #[doc = "Checks if the value of the field is `AMUXA`"]
    #[inline(always)]
    pub fn is_amuxa(&self) -> bool {
        *self == IO0_SEL_A::AMUXA
    }
    #[doc = "Checks if the value of the field is `AMUXB`"]
    #[inline(always)]
    pub fn is_amuxb(&self) -> bool {
        *self == IO0_SEL_A::AMUXB
    }
    #[doc = "Checks if the value of the field is `ACT_0`"]
    #[inline(always)]
    pub fn is_act_0(&self) -> bool {
        *self == IO0_SEL_A::ACT_0
    }
    #[doc = "Checks if the value of the field is `ACT_1`"]
    #[inline(always)]
    pub fn is_act_1(&self) -> bool {
        *self == IO0_SEL_A::ACT_1
    }
    #[doc = "Checks if the value of the field is `ACT_2`"]
    #[inline(always)]
    pub fn is_act_2(&self) -> bool {
        *self == IO0_SEL_A::ACT_2
    }
    #[doc = "Checks if the value of the field is `ACT_3`"]
    #[inline(always)]
    pub fn is_act_3(&self) -> bool {
        *self == IO0_SEL_A::ACT_3
    }
    #[doc = "Checks if the value of the field is `LCD_COM`"]
    #[inline(always)]
    pub fn is_lcd_com(&self) -> bool {
        *self == IO0_SEL_A::LCD_COM
    }
    #[doc = "Checks if the value of the field is `LCD_SEG`"]
    #[inline(always)]
    pub fn is_lcd_seg(&self) -> bool {
        *self == IO0_SEL_A::LCD_SEG
    }
    #[doc = "Checks if the value of the field is `DS_2`"]
    #[inline(always)]
    pub fn is_ds_2(&self) -> bool {
        *self == IO0_SEL_A::DS_2
    }
    #[doc = "Checks if the value of the field is `DS_3`"]
    #[inline(always)]
    pub fn is_ds_3(&self) -> bool {
        *self == IO0_SEL_A::DS_3
    }
}
#[doc = "Field `IO0_SEL` writer - Selects connection for IO pad 0 route."]
pub type IO0_SEL_W<'a> = crate::FieldWriterSafe<'a, u32, PORT_SEL_SPEC, u8, IO0_SEL_A, 4, 0>;
impl<'a> IO0_SEL_W<'a> {
    #[doc = "SW controlled GPIO."]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(IO0_SEL_A::GPIO)
    }
    #[doc = "SW controlled 'out', DSI controlled 'oe_n'."]
    #[inline(always)]
    pub fn gpio_dsi(self) -> &'a mut W {
        self.variant(IO0_SEL_A::GPIO_DSI)
    }
    #[doc = "DSI controlled 'out' and 'oe_n'."]
    #[inline(always)]
    pub fn dsi_dsi(self) -> &'a mut W {
        self.variant(IO0_SEL_A::DSI_DSI)
    }
    #[doc = "DSI controlled 'out', SW controlled 'oe_n'."]
    #[inline(always)]
    pub fn dsi_gpio(self) -> &'a mut W {
        self.variant(IO0_SEL_A::DSI_GPIO)
    }
    #[doc = "CSD sense connection (analog mode)"]
    #[inline(always)]
    pub fn csd_sense(self) -> &'a mut W {
        self.variant(IO0_SEL_A::CSD_SENSE)
    }
    #[doc = "CSD shield connection (analog mode)"]
    #[inline(always)]
    pub fn csd_shield(self) -> &'a mut W {
        self.variant(IO0_SEL_A::CSD_SHIELD)
    }
    #[doc = "AMUXBUS A connection."]
    #[inline(always)]
    pub fn amuxa(self) -> &'a mut W {
        self.variant(IO0_SEL_A::AMUXA)
    }
    #[doc = "AMUXBUS B connection. This mode is also used for CSD GPIO charging. When CSD GPIO charging is enabled in CSD_CONTROL, 'oe_n' is connected to '!csd_charge' signal (and IO pad is also still connected to AMUXBUS B)."]
    #[inline(always)]
    pub fn amuxb(self) -> &'a mut W {
        self.variant(IO0_SEL_A::AMUXB)
    }
    #[doc = "Chip specific Active source 0 connection."]
    #[inline(always)]
    pub fn act_0(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_0)
    }
    #[doc = "Chip specific Active source 1 connection."]
    #[inline(always)]
    pub fn act_1(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_1)
    }
    #[doc = "Chip specific Active source 2 connection."]
    #[inline(always)]
    pub fn act_2(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_2)
    }
    #[doc = "Chip specific Active source 3 connection."]
    #[inline(always)]
    pub fn act_3(self) -> &'a mut W {
        self.variant(IO0_SEL_A::ACT_3)
    }
    #[doc = "LCD common connection. This mode provides DeepSleep functionality (provided that the LCD block is enabled and properly configured)."]
    #[inline(always)]
    pub fn lcd_com(self) -> &'a mut W {
        self.variant(IO0_SEL_A::LCD_COM)
    }
    #[doc = "LCD segment connection. This mode provides DeepSleep functionality (provided that the LCD block is enabled and properly configured)."]
    #[inline(always)]
    pub fn lcd_seg(self) -> &'a mut W {
        self.variant(IO0_SEL_A::LCD_SEG)
    }
    #[doc = "Chip specific DeepSleep source 2 connection."]
    #[inline(always)]
    pub fn ds_2(self) -> &'a mut W {
        self.variant(IO0_SEL_A::DS_2)
    }
    #[doc = "Chip specific DeepSleep source 3 connection."]
    #[inline(always)]
    pub fn ds_3(self) -> &'a mut W {
        self.variant(IO0_SEL_A::DS_3)
    }
}
#[doc = "Field `IO1_SEL` reader - Selects connection for IO pad 1 route."]
pub type IO1_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IO1_SEL` writer - Selects connection for IO pad 1 route."]
pub type IO1_SEL_W<'a> = crate::FieldWriter<'a, u32, PORT_SEL_SPEC, u8, u8, 4, 4>;
#[doc = "Field `IO2_SEL` reader - Selects connection for IO pad 2 route."]
pub type IO2_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IO2_SEL` writer - Selects connection for IO pad 2 route."]
pub type IO2_SEL_W<'a> = crate::FieldWriter<'a, u32, PORT_SEL_SPEC, u8, u8, 4, 8>;
#[doc = "Field `IO3_SEL` reader - Selects connection for IO pad 3 route."]
pub type IO3_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IO3_SEL` writer - Selects connection for IO pad 3 route."]
pub type IO3_SEL_W<'a> = crate::FieldWriter<'a, u32, PORT_SEL_SPEC, u8, u8, 4, 12>;
#[doc = "Field `IO4_SEL` reader - Selects connection for IO pad 4 route."]
pub type IO4_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IO4_SEL` writer - Selects connection for IO pad 4 route."]
pub type IO4_SEL_W<'a> = crate::FieldWriter<'a, u32, PORT_SEL_SPEC, u8, u8, 4, 16>;
#[doc = "Field `IO5_SEL` reader - Selects connection for IO pad 5 route."]
pub type IO5_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IO5_SEL` writer - Selects connection for IO pad 5 route."]
pub type IO5_SEL_W<'a> = crate::FieldWriter<'a, u32, PORT_SEL_SPEC, u8, u8, 4, 20>;
#[doc = "Field `IO6_SEL` reader - Selects connection for IO pad 6 route."]
pub type IO6_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IO6_SEL` writer - Selects connection for IO pad 6 route."]
pub type IO6_SEL_W<'a> = crate::FieldWriter<'a, u32, PORT_SEL_SPEC, u8, u8, 4, 24>;
#[doc = "Field `IO7_SEL` reader - Selects connection for IO pad 7 route."]
pub type IO7_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IO7_SEL` writer - Selects connection for IO pad 7 route."]
pub type IO7_SEL_W<'a> = crate::FieldWriter<'a, u32, PORT_SEL_SPEC, u8, u8, 4, 28>;
impl R {
    #[doc = "Bits 0:3 - Selects connection for IO pad 0 route."]
    #[inline(always)]
    pub fn io0_sel(&self) -> IO0_SEL_R {
        IO0_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Selects connection for IO pad 1 route."]
    #[inline(always)]
    pub fn io1_sel(&self) -> IO1_SEL_R {
        IO1_SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Selects connection for IO pad 2 route."]
    #[inline(always)]
    pub fn io2_sel(&self) -> IO2_SEL_R {
        IO2_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Selects connection for IO pad 3 route."]
    #[inline(always)]
    pub fn io3_sel(&self) -> IO3_SEL_R {
        IO3_SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Selects connection for IO pad 4 route."]
    #[inline(always)]
    pub fn io4_sel(&self) -> IO4_SEL_R {
        IO4_SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Selects connection for IO pad 5 route."]
    #[inline(always)]
    pub fn io5_sel(&self) -> IO5_SEL_R {
        IO5_SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Selects connection for IO pad 6 route."]
    #[inline(always)]
    pub fn io6_sel(&self) -> IO6_SEL_R {
        IO6_SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Selects connection for IO pad 7 route."]
    #[inline(always)]
    pub fn io7_sel(&self) -> IO7_SEL_R {
        IO7_SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Selects connection for IO pad 0 route."]
    #[inline(always)]
    pub fn io0_sel(&mut self) -> IO0_SEL_W {
        IO0_SEL_W::new(self)
    }
    #[doc = "Bits 4:7 - Selects connection for IO pad 1 route."]
    #[inline(always)]
    pub fn io1_sel(&mut self) -> IO1_SEL_W {
        IO1_SEL_W::new(self)
    }
    #[doc = "Bits 8:11 - Selects connection for IO pad 2 route."]
    #[inline(always)]
    pub fn io2_sel(&mut self) -> IO2_SEL_W {
        IO2_SEL_W::new(self)
    }
    #[doc = "Bits 12:15 - Selects connection for IO pad 3 route."]
    #[inline(always)]
    pub fn io3_sel(&mut self) -> IO3_SEL_W {
        IO3_SEL_W::new(self)
    }
    #[doc = "Bits 16:19 - Selects connection for IO pad 4 route."]
    #[inline(always)]
    pub fn io4_sel(&mut self) -> IO4_SEL_W {
        IO4_SEL_W::new(self)
    }
    #[doc = "Bits 20:23 - Selects connection for IO pad 5 route."]
    #[inline(always)]
    pub fn io5_sel(&mut self) -> IO5_SEL_W {
        IO5_SEL_W::new(self)
    }
    #[doc = "Bits 24:27 - Selects connection for IO pad 6 route."]
    #[inline(always)]
    pub fn io6_sel(&mut self) -> IO6_SEL_W {
        IO6_SEL_W::new(self)
    }
    #[doc = "Bits 28:31 - Selects connection for IO pad 7 route."]
    #[inline(always)]
    pub fn io7_sel(&mut self) -> IO7_SEL_W {
        IO7_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [port_sel](index.html) module"]
pub struct PORT_SEL_SPEC;
impl crate::RegisterSpec for PORT_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [port_sel::R](R) reader structure"]
impl crate::Readable for PORT_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [port_sel::W](W) writer structure"]
impl crate::Writable for PORT_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PORT_SEL to value 0"]
impl crate::Resettable for PORT_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
