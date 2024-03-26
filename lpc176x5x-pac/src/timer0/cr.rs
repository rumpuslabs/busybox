#[doc = "Register `CR[%s]` reader"]
pub type R = crate::R<CR_SPEC>;
#[doc = "Field `CAP` reader - Timer counter capture value."]
pub type CAP_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Timer counter capture value."]
    #[inline(always)]
    pub fn cap(&self) -> CAP_R {
        CAP_R::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("cap", &format_args!("{}", self.cap().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Capture Register 0. CR0 is loaded with the value of TC when there is an event on the CAPn.0 input.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CR_SPEC {}
#[doc = "`reset()` method sets CR[%s]
to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
