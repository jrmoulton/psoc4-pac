#[doc = "Register `OA_RES_CTRL` reader"]
pub struct R(crate::R<OA_RES_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OA_RES_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OA_RES_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OA_RES_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OA_RES_CTRL` writer"]
pub struct W(crate::W<OA_RES_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OA_RES_CTRL_SPEC>;
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
impl From<crate::W<OA_RES_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OA_RES_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Selects power for opamp\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OA_PWR_MODE_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: Low power"]
    LOW = 1,
    #[doc = "2: Medium power"]
    MED = 2,
    #[doc = "3: High power"]
    HIGH = 3,
}
impl From<OA_PWR_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: OA_PWR_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OA_PWR_MODE` reader - Selects power for opamp"]
pub type OA_PWR_MODE_R = crate::FieldReader<u8, OA_PWR_MODE_A>;
impl OA_PWR_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OA_PWR_MODE_A {
        match self.bits {
            0 => OA_PWR_MODE_A::OFF,
            1 => OA_PWR_MODE_A::LOW,
            2 => OA_PWR_MODE_A::MED,
            3 => OA_PWR_MODE_A::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == OA_PWR_MODE_A::OFF
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OA_PWR_MODE_A::LOW
    }
    #[doc = "Checks if the value of the field is `MED`"]
    #[inline(always)]
    pub fn is_med(&self) -> bool {
        *self == OA_PWR_MODE_A::MED
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OA_PWR_MODE_A::HIGH
    }
}
#[doc = "Field `OA_PWR_MODE` writer - Selects power for opamp"]
pub type OA_PWR_MODE_W<'a> =
    crate::FieldWriterSafe<'a, u32, OA_RES_CTRL_SPEC, u8, OA_PWR_MODE_A, 2, 0>;
impl<'a> OA_PWR_MODE_W<'a> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(OA_PWR_MODE_A::OFF)
    }
    #[doc = "Low power"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OA_PWR_MODE_A::LOW)
    }
    #[doc = "Medium power"]
    #[inline(always)]
    pub fn med(self) -> &'a mut W {
        self.variant(OA_PWR_MODE_A::MED)
    }
    #[doc = "High power"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OA_PWR_MODE_A::HIGH)
    }
}
#[doc = "Opamp output strenght select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OA0_DRIVE_STR_SEL_A {
    #[doc = "0: Internal only"]
    CY1X = 0,
    #[doc = "1: Output to pin"]
    CY10X = 1,
}
impl From<OA0_DRIVE_STR_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: OA0_DRIVE_STR_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OA0_DRIVE_STR_SEL` reader - Opamp output strenght select"]
pub type OA0_DRIVE_STR_SEL_R = crate::BitReader<OA0_DRIVE_STR_SEL_A>;
impl OA0_DRIVE_STR_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OA0_DRIVE_STR_SEL_A {
        match self.bits {
            false => OA0_DRIVE_STR_SEL_A::CY1X,
            true => OA0_DRIVE_STR_SEL_A::CY10X,
        }
    }
    #[doc = "Checks if the value of the field is `CY1X`"]
    #[inline(always)]
    pub fn is_cy1x(&self) -> bool {
        *self == OA0_DRIVE_STR_SEL_A::CY1X
    }
    #[doc = "Checks if the value of the field is `CY10X`"]
    #[inline(always)]
    pub fn is_cy10x(&self) -> bool {
        *self == OA0_DRIVE_STR_SEL_A::CY10X
    }
}
#[doc = "Field `OA0_DRIVE_STR_SEL` writer - Opamp output strenght select"]
pub type OA0_DRIVE_STR_SEL_W<'a> =
    crate::BitWriter<'a, u32, OA_RES_CTRL_SPEC, OA0_DRIVE_STR_SEL_A, 2>;
impl<'a> OA0_DRIVE_STR_SEL_W<'a> {
    #[doc = "Internal only"]
    #[inline(always)]
    pub fn cy1x(self) -> &'a mut W {
        self.variant(OA0_DRIVE_STR_SEL_A::CY1X)
    }
    #[doc = "Output to pin"]
    #[inline(always)]
    pub fn cy10x(self) -> &'a mut W {
        self.variant(OA0_DRIVE_STR_SEL_A::CY10X)
    }
}
#[doc = "Selects pump\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OA_PUMP_EN_A {
    #[doc = "0: Pump disabled"]
    DISABLE = 0,
    #[doc = "1: Pump enabled"]
    ENABLE = 1,
}
impl From<OA_PUMP_EN_A> for bool {
    #[inline(always)]
    fn from(variant: OA_PUMP_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OA_PUMP_EN` reader - Selects pump"]
pub type OA_PUMP_EN_R = crate::BitReader<OA_PUMP_EN_A>;
impl OA_PUMP_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OA_PUMP_EN_A {
        match self.bits {
            false => OA_PUMP_EN_A::DISABLE,
            true => OA_PUMP_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == OA_PUMP_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == OA_PUMP_EN_A::ENABLE
    }
}
#[doc = "Field `OA_PUMP_EN` writer - Selects pump"]
pub type OA_PUMP_EN_W<'a> = crate::BitWriter<'a, u32, OA_RES_CTRL_SPEC, OA_PUMP_EN_A, 11>;
impl<'a> OA_PUMP_EN_W<'a> {
    #[doc = "Pump disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(OA_PUMP_EN_A::DISABLE)
    }
    #[doc = "Pump enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(OA_PUMP_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects power for opamp"]
    #[inline(always)]
    pub fn oa_pwr_mode(&self) -> OA_PWR_MODE_R {
        OA_PWR_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Opamp output strenght select"]
    #[inline(always)]
    pub fn oa0_drive_str_sel(&self) -> OA0_DRIVE_STR_SEL_R {
        OA0_DRIVE_STR_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 11 - Selects pump"]
    #[inline(always)]
    pub fn oa_pump_en(&self) -> OA_PUMP_EN_R {
        OA_PUMP_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects power for opamp"]
    #[inline(always)]
    pub fn oa_pwr_mode(&mut self) -> OA_PWR_MODE_W {
        OA_PWR_MODE_W::new(self)
    }
    #[doc = "Bit 2 - Opamp output strenght select"]
    #[inline(always)]
    pub fn oa0_drive_str_sel(&mut self) -> OA0_DRIVE_STR_SEL_W {
        OA0_DRIVE_STR_SEL_W::new(self)
    }
    #[doc = "Bit 11 - Selects pump"]
    #[inline(always)]
    pub fn oa_pump_en(&mut self) -> OA_PUMP_EN_W {
        OA_PUMP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Opamp and resistor control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oa_res_ctrl](index.html) module"]
pub struct OA_RES_CTRL_SPEC;
impl crate::RegisterSpec for OA_RES_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oa_res_ctrl::R](R) reader structure"]
impl crate::Readable for OA_RES_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oa_res_ctrl::W](W) writer structure"]
impl crate::Writable for OA_RES_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OA_RES_CTRL to value 0"]
impl crate::Resettable for OA_RES_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
