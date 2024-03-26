#[doc = "Register `TIME` reader"]
pub type R = crate::R<TIME_SPEC>;
#[doc = "Field `VELVAL` reader - Current velocity timer value."]
pub type VELVAL_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current velocity timer value."]
    #[inline(always)]
    pub fn velval(&self) -> VELVAL_R {
        VELVAL_R::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIME")
            .field("velval", &format_args!("{}", self.velval().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<TIME_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Velocity timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIME_SPEC;
impl crate::RegisterSpec for TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`time::R`](R) reader structure"]
impl crate::Readable for TIME_SPEC {}
#[doc = "`reset()` method sets TIME to value 0"]
impl crate::Resettable for TIME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
