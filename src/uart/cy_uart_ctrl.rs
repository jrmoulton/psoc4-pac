#[doc = "Register `Cy_UART_CTRL` reader"]
pub struct R(crate::R<CY_UART_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CY_UART_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CY_UART_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CY_UART_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Cy_UART_CTRL` writer"]
pub struct W(crate::W<CY_UART_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CY_UART_CTRL_SPEC>;
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
impl From<crate::W<CY_UART_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CY_UART_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOOPBACK` reader - Local loopback control."]
pub type LOOPBACK_R = crate::BitReader<bool>;
#[doc = "Field `LOOPBACK` writer - Local loopback control."]
pub type LOOPBACK_W<'a> = crate::BitWriter<'a, u32, CY_UART_CTRL_SPEC, bool, 16>;
#[doc = "Field `MODE` reader - Submode of UART operation: Standard = 0, Smart Card = 1, IrDA = 2."]
pub type MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE` writer - Submode of UART operation: Standard = 0, Smart Card = 1, IrDA = 2."]
pub type MODE_W<'a> = crate::FieldWriter<'a, u32, CY_UART_CTRL_SPEC, u8, u8, 2, 24>;
impl R {
    #[doc = "Bit 16 - Local loopback control."]
    #[inline(always)]
    pub fn loopback(&self) -> LOOPBACK_R {
        LOOPBACK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Submode of UART operation: Standard = 0, Smart Card = 1, IrDA = 2."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - Local loopback control."]
    #[inline(always)]
    pub fn loopback(&mut self) -> LOOPBACK_W {
        LOOPBACK_W::new(self)
    }
    #[doc = "Bits 24:25 - Submode of UART operation: Standard = 0, Smart Card = 1, IrDA = 2."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cy_uart_ctrl](index.html) module"]
pub struct CY_UART_CTRL_SPEC;
impl crate::RegisterSpec for CY_UART_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cy_uart_ctrl::R](R) reader structure"]
impl crate::Readable for CY_UART_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cy_uart_ctrl::W](W) writer structure"]
impl crate::Writable for CY_UART_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets Cy_UART_CTRL to value 0"]
impl crate::Resettable for CY_UART_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
