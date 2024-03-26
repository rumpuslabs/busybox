#[doc = "Register `MRDD` reader"]
pub type R = crate::R<MRDD_SPEC>;
#[doc = "Field `READDATA` reader - READ DATA. Following an MII Mgmt Read Cycle, the 16-bit data can be read from this location."]
pub type READDATA_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - READ DATA. Following an MII Mgmt Read Cycle, the 16-bit data can be read from this location."]
    #[inline(always)]
    pub fn readdata(&self) -> READDATA_R {
        READDATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MRDD")
            .field("readdata", &format_args!("{}", self.readdata().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<MRDD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "MII Mgmt Read Data register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrdd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MRDD_SPEC;
impl crate::RegisterSpec for MRDD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrdd::R`](R) reader structure"]
impl crate::Readable for MRDD_SPEC {}
#[doc = "`reset()` method sets MRDD to value 0"]
impl crate::Resettable for MRDD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
