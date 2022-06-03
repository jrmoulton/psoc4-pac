#[doc = "Register `SYSREQ` reader"]
pub struct R(crate::R<SYSREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSREQ` writer"]
pub struct W(crate::W<SYSREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSREQ_SPEC>;
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
impl From<crate::W<SYSREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCALL_COMMAND` reader - Opcode of the system call being requested."]
pub type SYSCALL_COMMAND_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SYSCALL_COMMAND` writer - Opcode of the system call being requested."]
pub type SYSCALL_COMMAND_W<'a> = crate::FieldWriter<'a, u32, SYSREQ_SPEC, u16, u16, 16, 0>;
#[doc = "Field `DIS_RESET_VECT_REL` reader - Disable Reset Vector fetch relocation: '0': CPU accesses to locations 0x0000:0000 - 0x0000:0007 are redirected to ROM. '1': CPU accesses to locations 0x0000:0000 - 0x0000:0007 are made to flash. Note that this field defaults to '0' on reset, ensuring actual reset vector fetches are always made to ROM. Note that this field does not affect DAP accesses. Flash DfT routines may set this bit to '1' to enable uninhibited read-back of programmed data in the first flash page."]
pub type DIS_RESET_VECT_REL_R = crate::BitReader<bool>;
#[doc = "Field `DIS_RESET_VECT_REL` writer - Disable Reset Vector fetch relocation: '0': CPU accesses to locations 0x0000:0000 - 0x0000:0007 are redirected to ROM. '1': CPU accesses to locations 0x0000:0000 - 0x0000:0007 are made to flash. Note that this field defaults to '0' on reset, ensuring actual reset vector fetches are always made to ROM. Note that this field does not affect DAP accesses. Flash DfT routines may set this bit to '1' to enable uninhibited read-back of programmed data in the first flash page."]
pub type DIS_RESET_VECT_REL_W<'a> = crate::BitWriter<'a, u32, SYSREQ_SPEC, bool, 27>;
#[doc = "Field `PRIVILEGED` reader - Indicates whether the system is in privileged ('1') or user mode ('0'). Only CPU SW executing from ROM can set this field to '1' when ROM_ACCESS_EN is '1' (the CPU is executing a SystemCall NMI interrupt handler). Any other write to this field sets is to '0'. This field is used as the AHB-Lite hprot\\[1\\]
signal to implement Cypress proprietary user/privileged modes. These modes are used to enable/disable access to specific MMIO registers and memory regions."]
pub type PRIVILEGED_R = crate::BitReader<bool>;
#[doc = "Field `PRIVILEGED` writer - Indicates whether the system is in privileged ('1') or user mode ('0'). Only CPU SW executing from ROM can set this field to '1' when ROM_ACCESS_EN is '1' (the CPU is executing a SystemCall NMI interrupt handler). Any other write to this field sets is to '0'. This field is used as the AHB-Lite hprot\\[1\\]
signal to implement Cypress proprietary user/privileged modes. These modes are used to enable/disable access to specific MMIO registers and memory regions."]
pub type PRIVILEGED_W<'a> = crate::BitWriter<'a, u32, SYSREQ_SPEC, bool, 28>;
#[doc = "Field `ROM_ACCESS_EN` reader - Indicates that executing from Boot ROM is enabled. HW sets this field to '1', on reset or when the SystemCall NMI vector is fetched from Boot ROM. HW sets this field to '0', when the CPU is NOT executing from either Boot or System ROM. This bit is used for debug purposes only."]
pub type ROM_ACCESS_EN_R = crate::BitReader<bool>;
#[doc = "Field `HMASTER_0` reader - Indicates the source of the write access to the SYSREQ register. '0': CPU write access. '1': DAP write access. HW sets this field when the SYSREQ register is written to and SYSCALL_REQ is '0' (the last time it is set is when SW sets SYSCALL_REQ from '0' to '1')."]
pub type HMASTER_0_R = crate::BitReader<bool>;
#[doc = "Field `SYSCALL_REQ` reader - CPU/DAP writes a '1' to this field to request a SystemCall. The HMASTER_0 field indicates the source of the write access. Setting this field to '1' immediate results in a NMI. The SystemCall NMI interrupt handler sets this field to '0' after servicing the request."]
pub type SYSCALL_REQ_R = crate::BitReader<bool>;
#[doc = "Field `SYSCALL_REQ` writer - CPU/DAP writes a '1' to this field to request a SystemCall. The HMASTER_0 field indicates the source of the write access. Setting this field to '1' immediate results in a NMI. The SystemCall NMI interrupt handler sets this field to '0' after servicing the request."]
pub type SYSCALL_REQ_W<'a> = crate::BitWriter<'a, u32, SYSREQ_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:15 - Opcode of the system call being requested."]
    #[inline(always)]
    pub fn syscall_command(&self) -> SYSCALL_COMMAND_R {
        SYSCALL_COMMAND_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 27 - Disable Reset Vector fetch relocation: '0': CPU accesses to locations 0x0000:0000 - 0x0000:0007 are redirected to ROM. '1': CPU accesses to locations 0x0000:0000 - 0x0000:0007 are made to flash. Note that this field defaults to '0' on reset, ensuring actual reset vector fetches are always made to ROM. Note that this field does not affect DAP accesses. Flash DfT routines may set this bit to '1' to enable uninhibited read-back of programmed data in the first flash page."]
    #[inline(always)]
    pub fn dis_reset_vect_rel(&self) -> DIS_RESET_VECT_REL_R {
        DIS_RESET_VECT_REL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Indicates whether the system is in privileged ('1') or user mode ('0'). Only CPU SW executing from ROM can set this field to '1' when ROM_ACCESS_EN is '1' (the CPU is executing a SystemCall NMI interrupt handler). Any other write to this field sets is to '0'. This field is used as the AHB-Lite hprot\\[1\\]
signal to implement Cypress proprietary user/privileged modes. These modes are used to enable/disable access to specific MMIO registers and memory regions."]
    #[inline(always)]
    pub fn privileged(&self) -> PRIVILEGED_R {
        PRIVILEGED_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Indicates that executing from Boot ROM is enabled. HW sets this field to '1', on reset or when the SystemCall NMI vector is fetched from Boot ROM. HW sets this field to '0', when the CPU is NOT executing from either Boot or System ROM. This bit is used for debug purposes only."]
    #[inline(always)]
    pub fn rom_access_en(&self) -> ROM_ACCESS_EN_R {
        ROM_ACCESS_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Indicates the source of the write access to the SYSREQ register. '0': CPU write access. '1': DAP write access. HW sets this field when the SYSREQ register is written to and SYSCALL_REQ is '0' (the last time it is set is when SW sets SYSCALL_REQ from '0' to '1')."]
    #[inline(always)]
    pub fn hmaster_0(&self) -> HMASTER_0_R {
        HMASTER_0_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - CPU/DAP writes a '1' to this field to request a SystemCall. The HMASTER_0 field indicates the source of the write access. Setting this field to '1' immediate results in a NMI. The SystemCall NMI interrupt handler sets this field to '0' after servicing the request."]
    #[inline(always)]
    pub fn syscall_req(&self) -> SYSCALL_REQ_R {
        SYSCALL_REQ_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Opcode of the system call being requested."]
    #[inline(always)]
    pub fn syscall_command(&mut self) -> SYSCALL_COMMAND_W {
        SYSCALL_COMMAND_W::new(self)
    }
    #[doc = "Bit 27 - Disable Reset Vector fetch relocation: '0': CPU accesses to locations 0x0000:0000 - 0x0000:0007 are redirected to ROM. '1': CPU accesses to locations 0x0000:0000 - 0x0000:0007 are made to flash. Note that this field defaults to '0' on reset, ensuring actual reset vector fetches are always made to ROM. Note that this field does not affect DAP accesses. Flash DfT routines may set this bit to '1' to enable uninhibited read-back of programmed data in the first flash page."]
    #[inline(always)]
    pub fn dis_reset_vect_rel(&mut self) -> DIS_RESET_VECT_REL_W {
        DIS_RESET_VECT_REL_W::new(self)
    }
    #[doc = "Bit 28 - Indicates whether the system is in privileged ('1') or user mode ('0'). Only CPU SW executing from ROM can set this field to '1' when ROM_ACCESS_EN is '1' (the CPU is executing a SystemCall NMI interrupt handler). Any other write to this field sets is to '0'. This field is used as the AHB-Lite hprot\\[1\\]
signal to implement Cypress proprietary user/privileged modes. These modes are used to enable/disable access to specific MMIO registers and memory regions."]
    #[inline(always)]
    pub fn privileged(&mut self) -> PRIVILEGED_W {
        PRIVILEGED_W::new(self)
    }
    #[doc = "Bit 31 - CPU/DAP writes a '1' to this field to request a SystemCall. The HMASTER_0 field indicates the source of the write access. Setting this field to '1' immediate results in a NMI. The SystemCall NMI interrupt handler sets this field to '0' after servicing the request."]
    #[inline(always)]
    pub fn syscall_req(&mut self) -> SYSCALL_REQ_W {
        SYSCALL_REQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCALL control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysreq](index.html) module"]
pub struct SYSREQ_SPEC;
impl crate::RegisterSpec for SYSREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysreq::R](R) reader structure"]
impl crate::Readable for SYSREQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysreq::W](W) writer structure"]
impl crate::Writable for SYSREQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSREQ to value 0x3000_0000"]
impl crate::Resettable for SYSREQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3000_0000
    }
}
