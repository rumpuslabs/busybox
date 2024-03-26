#[doc = "Register `CMDCODE` writer"]
pub type W = crate::W<CMDCODE_SPEC>;
#[doc = "The command phase:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMD_PHASE_AW {
    #[doc = "2: Read"]
    READ = 2,
    #[doc = "1: Write"]
    WRITE = 1,
    #[doc = "5: Command"]
    COMMAND = 5,
}
impl From<CMD_PHASE_AW> for u8 {
    #[inline(always)]
    fn from(variant: CMD_PHASE_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMD_PHASE_AW {
    type Ux = u8;
}
#[doc = "Field `CMD_PHASE` writer - The command phase:"]
pub type CMD_PHASE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O, CMD_PHASE_AW>;
impl<'a, REG, const O: u8> CMD_PHASE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Read"]
    #[inline(always)]
    pub fn read(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_PHASE_AW::READ)
    }
    #[doc = "Write"]
    #[inline(always)]
    pub fn write(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_PHASE_AW::WRITE)
    }
    #[doc = "Command"]
    #[inline(always)]
    pub fn command(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_PHASE_AW::COMMAND)
    }
}
#[doc = "Field `CMD_CODE_WDATA` writer - This is a multi-purpose field. When CMD_PHASE is Command or Read, this field contains the code for the command (CMD_CODE). When CMD_PHASE is Write, this field contains the command write data (CMD_WDATA)."]
pub type CMD_CODE_WDATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CMDCODE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 8:15 - The command phase:"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_phase(&mut self) -> CMD_PHASE_W<CMDCODE_SPEC, 8> {
        CMD_PHASE_W::new(self)
    }
    #[doc = "Bits 16:23 - This is a multi-purpose field. When CMD_PHASE is Command or Read, this field contains the code for the command (CMD_CODE). When CMD_PHASE is Write, this field contains the command write data (CMD_WDATA)."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_code_wdata(&mut self) -> CMD_CODE_WDATA_W<CMDCODE_SPEC, 16> {
        CMD_CODE_WDATA_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USB Command Code\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdcode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMDCODE_SPEC;
impl crate::RegisterSpec for CMDCODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmdcode::W`](W) writer structure"]
impl crate::Writable for CMDCODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMDCODE to value 0"]
impl crate::Resettable for CMDCODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
