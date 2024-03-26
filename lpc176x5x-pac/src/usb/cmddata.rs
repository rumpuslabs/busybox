#[doc = "Register `CMDDATA` reader"]
pub type R = crate::R<CMDDATA_SPEC>;
#[doc = "Field `CMD_RDATA` reader - Command Read Data."]
pub type CMD_RDATA_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Command Read Data."]
    #[inline(always)]
    pub fn cmd_rdata(&self) -> CMD_RDATA_R {
        CMD_RDATA_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDDATA")
            .field("cmd_rdata", &format_args!("{}", self.cmd_rdata().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CMDDATA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "USB Command Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmddata::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMDDATA_SPEC;
impl crate::RegisterSpec for CMDDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmddata::R`](R) reader structure"]
impl crate::Readable for CMDDATA_SPEC {}
#[doc = "`reset()` method sets CMDDATA to value 0"]
impl crate::Resettable for CMDDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
