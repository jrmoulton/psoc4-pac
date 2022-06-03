#[doc = "Register `MUX_SWITCH_HW_CTRL` reader"]
pub struct R(crate::R<MUX_SWITCH_HW_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MUX_SWITCH_HW_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MUX_SWITCH_HW_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MUX_SWITCH_HW_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MUX_SWITCH_HW_CTRL` writer"]
pub struct W(crate::W<MUX_SWITCH_HW_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MUX_SWITCH_HW_CTRL_SPEC>;
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
impl From<crate::W<MUX_SWITCH_HW_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MUX_SWITCH_HW_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MUX_HW_CTRL_P0` reader - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P0 switches."]
pub type MUX_HW_CTRL_P0_R = crate::BitReader<bool>;
#[doc = "Field `MUX_HW_CTRL_P0` writer - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P0 switches."]
pub type MUX_HW_CTRL_P0_W<'a> = crate::BitWriter<'a, u32, MUX_SWITCH_HW_CTRL_SPEC, bool, 0>;
#[doc = "Field `MUX_HW_CTRL_P1` reader - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P1 switches."]
pub type MUX_HW_CTRL_P1_R = crate::BitReader<bool>;
#[doc = "Field `MUX_HW_CTRL_P1` writer - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P1 switches."]
pub type MUX_HW_CTRL_P1_W<'a> = crate::BitWriter<'a, u32, MUX_SWITCH_HW_CTRL_SPEC, bool, 1>;
#[doc = "Field `MUX_HW_CTRL_P2` reader - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P2 switches."]
pub type MUX_HW_CTRL_P2_R = crate::BitReader<bool>;
#[doc = "Field `MUX_HW_CTRL_P2` writer - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P2 switches."]
pub type MUX_HW_CTRL_P2_W<'a> = crate::BitWriter<'a, u32, MUX_SWITCH_HW_CTRL_SPEC, bool, 2>;
#[doc = "Field `MUX_HW_CTRL_P3` reader - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P3 switches."]
pub type MUX_HW_CTRL_P3_R = crate::BitReader<bool>;
#[doc = "Field `MUX_HW_CTRL_P3` writer - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P3 switches."]
pub type MUX_HW_CTRL_P3_W<'a> = crate::BitWriter<'a, u32, MUX_SWITCH_HW_CTRL_SPEC, bool, 3>;
#[doc = "Field `MUX_HW_CTRL_P4` reader - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P4 switches."]
pub type MUX_HW_CTRL_P4_R = crate::BitReader<bool>;
#[doc = "Field `MUX_HW_CTRL_P4` writer - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P4 switches."]
pub type MUX_HW_CTRL_P4_W<'a> = crate::BitWriter<'a, u32, MUX_SWITCH_HW_CTRL_SPEC, bool, 4>;
#[doc = "Field `MUX_HW_CTRL_P5` reader - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P5 switches."]
pub type MUX_HW_CTRL_P5_R = crate::BitReader<bool>;
#[doc = "Field `MUX_HW_CTRL_P5` writer - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P5 switches."]
pub type MUX_HW_CTRL_P5_W<'a> = crate::BitWriter<'a, u32, MUX_SWITCH_HW_CTRL_SPEC, bool, 5>;
#[doc = "Field `MUX_HW_CTRL_P6` reader - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P6 switches."]
pub type MUX_HW_CTRL_P6_R = crate::BitReader<bool>;
#[doc = "Field `MUX_HW_CTRL_P6` writer - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P6 switches."]
pub type MUX_HW_CTRL_P6_W<'a> = crate::BitWriter<'a, u32, MUX_SWITCH_HW_CTRL_SPEC, bool, 6>;
#[doc = "Field `MUX_HW_CTRL_P7` reader - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P7 switches."]
pub type MUX_HW_CTRL_P7_R = crate::BitReader<bool>;
#[doc = "Field `MUX_HW_CTRL_P7` writer - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P7 switches."]
pub type MUX_HW_CTRL_P7_W<'a> = crate::BitWriter<'a, u32, MUX_SWITCH_HW_CTRL_SPEC, bool, 7>;
#[doc = "Field `MUX_HW_CTRL_VSSA` reader - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for vssa switch."]
pub type MUX_HW_CTRL_VSSA_R = crate::BitReader<bool>;
#[doc = "Field `MUX_HW_CTRL_VSSA` writer - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for vssa switch."]
pub type MUX_HW_CTRL_VSSA_W<'a> = crate::BitWriter<'a, u32, MUX_SWITCH_HW_CTRL_SPEC, bool, 16>;
#[doc = "Field `MUX_HW_CTRL_TEMP` reader - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for temp switch."]
pub type MUX_HW_CTRL_TEMP_R = crate::BitReader<bool>;
#[doc = "Field `MUX_HW_CTRL_TEMP` writer - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for temp switch."]
pub type MUX_HW_CTRL_TEMP_W<'a> = crate::BitWriter<'a, u32, MUX_SWITCH_HW_CTRL_SPEC, bool, 17>;
#[doc = "Field `MUX_HW_CTRL_AMUXBUSA` reader - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for amuxbusa switches."]
pub type MUX_HW_CTRL_AMUXBUSA_R = crate::BitReader<bool>;
#[doc = "Field `MUX_HW_CTRL_AMUXBUSA` writer - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for amuxbusa switches."]
pub type MUX_HW_CTRL_AMUXBUSA_W<'a> = crate::BitWriter<'a, u32, MUX_SWITCH_HW_CTRL_SPEC, bool, 18>;
#[doc = "Field `MUX_HW_CTRL_AMUXBUSB` reader - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for amuxbusb switches."]
pub type MUX_HW_CTRL_AMUXBUSB_R = crate::BitReader<bool>;
#[doc = "Field `MUX_HW_CTRL_AMUXBUSB` writer - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for amuxbusb switches."]
pub type MUX_HW_CTRL_AMUXBUSB_W<'a> = crate::BitWriter<'a, u32, MUX_SWITCH_HW_CTRL_SPEC, bool, 19>;
#[doc = "Field `MUX_HW_CTRL_SARBUS0` reader - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for sarbus0 switches."]
pub type MUX_HW_CTRL_SARBUS0_R = crate::BitReader<bool>;
#[doc = "Field `MUX_HW_CTRL_SARBUS0` writer - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for sarbus0 switches."]
pub type MUX_HW_CTRL_SARBUS0_W<'a> = crate::BitWriter<'a, u32, MUX_SWITCH_HW_CTRL_SPEC, bool, 22>;
#[doc = "Field `MUX_HW_CTRL_SARBUS1` reader - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for sarbus1 switches."]
pub type MUX_HW_CTRL_SARBUS1_R = crate::BitReader<bool>;
#[doc = "Field `MUX_HW_CTRL_SARBUS1` writer - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for sarbus1 switches."]
pub type MUX_HW_CTRL_SARBUS1_W<'a> = crate::BitWriter<'a, u32, MUX_SWITCH_HW_CTRL_SPEC, bool, 23>;
impl R {
    #[doc = "Bit 0 - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P0 switches."]
    #[inline(always)]
    pub fn mux_hw_ctrl_p0(&self) -> MUX_HW_CTRL_P0_R {
        MUX_HW_CTRL_P0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P1 switches."]
    #[inline(always)]
    pub fn mux_hw_ctrl_p1(&self) -> MUX_HW_CTRL_P1_R {
        MUX_HW_CTRL_P1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P2 switches."]
    #[inline(always)]
    pub fn mux_hw_ctrl_p2(&self) -> MUX_HW_CTRL_P2_R {
        MUX_HW_CTRL_P2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P3 switches."]
    #[inline(always)]
    pub fn mux_hw_ctrl_p3(&self) -> MUX_HW_CTRL_P3_R {
        MUX_HW_CTRL_P3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P4 switches."]
    #[inline(always)]
    pub fn mux_hw_ctrl_p4(&self) -> MUX_HW_CTRL_P4_R {
        MUX_HW_CTRL_P4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P5 switches."]
    #[inline(always)]
    pub fn mux_hw_ctrl_p5(&self) -> MUX_HW_CTRL_P5_R {
        MUX_HW_CTRL_P5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P6 switches."]
    #[inline(always)]
    pub fn mux_hw_ctrl_p6(&self) -> MUX_HW_CTRL_P6_R {
        MUX_HW_CTRL_P6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P7 switches."]
    #[inline(always)]
    pub fn mux_hw_ctrl_p7(&self) -> MUX_HW_CTRL_P7_R {
        MUX_HW_CTRL_P7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for vssa switch."]
    #[inline(always)]
    pub fn mux_hw_ctrl_vssa(&self) -> MUX_HW_CTRL_VSSA_R {
        MUX_HW_CTRL_VSSA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for temp switch."]
    #[inline(always)]
    pub fn mux_hw_ctrl_temp(&self) -> MUX_HW_CTRL_TEMP_R {
        MUX_HW_CTRL_TEMP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for amuxbusa switches."]
    #[inline(always)]
    pub fn mux_hw_ctrl_amuxbusa(&self) -> MUX_HW_CTRL_AMUXBUSA_R {
        MUX_HW_CTRL_AMUXBUSA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for amuxbusb switches."]
    #[inline(always)]
    pub fn mux_hw_ctrl_amuxbusb(&self) -> MUX_HW_CTRL_AMUXBUSB_R {
        MUX_HW_CTRL_AMUXBUSB_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 22 - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for sarbus0 switches."]
    #[inline(always)]
    pub fn mux_hw_ctrl_sarbus0(&self) -> MUX_HW_CTRL_SARBUS0_R {
        MUX_HW_CTRL_SARBUS0_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for sarbus1 switches."]
    #[inline(always)]
    pub fn mux_hw_ctrl_sarbus1(&self) -> MUX_HW_CTRL_SARBUS1_R {
        MUX_HW_CTRL_SARBUS1_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P0 switches."]
    #[inline(always)]
    pub fn mux_hw_ctrl_p0(&mut self) -> MUX_HW_CTRL_P0_W {
        MUX_HW_CTRL_P0_W::new(self)
    }
    #[doc = "Bit 1 - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P1 switches."]
    #[inline(always)]
    pub fn mux_hw_ctrl_p1(&mut self) -> MUX_HW_CTRL_P1_W {
        MUX_HW_CTRL_P1_W::new(self)
    }
    #[doc = "Bit 2 - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P2 switches."]
    #[inline(always)]
    pub fn mux_hw_ctrl_p2(&mut self) -> MUX_HW_CTRL_P2_W {
        MUX_HW_CTRL_P2_W::new(self)
    }
    #[doc = "Bit 3 - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P3 switches."]
    #[inline(always)]
    pub fn mux_hw_ctrl_p3(&mut self) -> MUX_HW_CTRL_P3_W {
        MUX_HW_CTRL_P3_W::new(self)
    }
    #[doc = "Bit 4 - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P4 switches."]
    #[inline(always)]
    pub fn mux_hw_ctrl_p4(&mut self) -> MUX_HW_CTRL_P4_W {
        MUX_HW_CTRL_P4_W::new(self)
    }
    #[doc = "Bit 5 - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P5 switches."]
    #[inline(always)]
    pub fn mux_hw_ctrl_p5(&mut self) -> MUX_HW_CTRL_P5_W {
        MUX_HW_CTRL_P5_W::new(self)
    }
    #[doc = "Bit 6 - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P6 switches."]
    #[inline(always)]
    pub fn mux_hw_ctrl_p6(&mut self) -> MUX_HW_CTRL_P6_W {
        MUX_HW_CTRL_P6_W::new(self)
    }
    #[doc = "Bit 7 - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for pin P7 switches."]
    #[inline(always)]
    pub fn mux_hw_ctrl_p7(&mut self) -> MUX_HW_CTRL_P7_W {
        MUX_HW_CTRL_P7_W::new(self)
    }
    #[doc = "Bit 16 - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for vssa switch."]
    #[inline(always)]
    pub fn mux_hw_ctrl_vssa(&mut self) -> MUX_HW_CTRL_VSSA_W {
        MUX_HW_CTRL_VSSA_W::new(self)
    }
    #[doc = "Bit 17 - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for temp switch."]
    #[inline(always)]
    pub fn mux_hw_ctrl_temp(&mut self) -> MUX_HW_CTRL_TEMP_W {
        MUX_HW_CTRL_TEMP_W::new(self)
    }
    #[doc = "Bit 18 - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for amuxbusa switches."]
    #[inline(always)]
    pub fn mux_hw_ctrl_amuxbusa(&mut self) -> MUX_HW_CTRL_AMUXBUSA_W {
        MUX_HW_CTRL_AMUXBUSA_W::new(self)
    }
    #[doc = "Bit 19 - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for amuxbusb switches."]
    #[inline(always)]
    pub fn mux_hw_ctrl_amuxbusb(&mut self) -> MUX_HW_CTRL_AMUXBUSB_W {
        MUX_HW_CTRL_AMUXBUSB_W::new(self)
    }
    #[doc = "Bit 22 - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for sarbus0 switches."]
    #[inline(always)]
    pub fn mux_hw_ctrl_sarbus0(&mut self) -> MUX_HW_CTRL_SARBUS0_W {
        MUX_HW_CTRL_SARBUS0_W::new(self)
    }
    #[doc = "Bit 23 - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for sarbus1 switches."]
    #[inline(always)]
    pub fn mux_hw_ctrl_sarbus1(&mut self) -> MUX_HW_CTRL_SARBUS1_W {
        MUX_HW_CTRL_SARBUS1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SARMUX switch hardware control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mux_switch_hw_ctrl](index.html) module"]
pub struct MUX_SWITCH_HW_CTRL_SPEC;
impl crate::RegisterSpec for MUX_SWITCH_HW_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mux_switch_hw_ctrl::R](R) reader structure"]
impl crate::Readable for MUX_SWITCH_HW_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mux_switch_hw_ctrl::W](W) writer structure"]
impl crate::Writable for MUX_SWITCH_HW_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MUX_SWITCH_HW_CTRL to value 0"]
impl crate::Resettable for MUX_SWITCH_HW_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
