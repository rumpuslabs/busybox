#[doc = "Register `INXCNT` reader"]
pub type R = crate::R<INXCNT_SPEC>;
#[doc = "Field `ENCPOS` reader - Current index counter value."]
pub type ENCPOS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current index counter value."]
    #[inline(always)]
    pub fn encpos(&self) -> ENCPOS_R {
        ENCPOS_R::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INXCNT")
            .field("encpos", &format_args!("{}", self.encpos().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<INXCNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Index count register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inxcnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INXCNT_SPEC;
impl crate::RegisterSpec for INXCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inxcnt::R`](R) reader structure"]
impl crate::Readable for INXCNT_SPEC {}
#[doc = "`reset()` method sets INXCNT to value 0"]
impl crate::Resettable for INXCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
