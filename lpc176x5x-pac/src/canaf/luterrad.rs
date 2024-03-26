#[doc = "Register `LUTERRAD` reader"]
pub type R = crate::R<LUTERRAD_SPEC>;
#[doc = "Field `LUTERRAD` reader - It the LUT Error bit (below) is 1, this read-only field contains the address in AF Lookup Table RAM, at which the Acceptance Filter encountered an error in the content of the tables."]
pub type LUTERRAD_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 2:10 - It the LUT Error bit (below) is 1, this read-only field contains the address in AF Lookup Table RAM, at which the Acceptance Filter encountered an error in the content of the tables."]
    #[inline(always)]
    pub fn luterrad(&self) -> LUTERRAD_R {
        LUTERRAD_R::new(((self.bits >> 2) & 0x01ff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LUTERRAD")
            .field("luterrad", &format_args!("{}", self.luterrad().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<LUTERRAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "LUT Error Address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`luterrad::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LUTERRAD_SPEC;
impl crate::RegisterSpec for LUTERRAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`luterrad::R`](R) reader structure"]
impl crate::Readable for LUTERRAD_SPEC {}
#[doc = "`reset()` method sets LUTERRAD to value 0"]
impl crate::Resettable for LUTERRAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
