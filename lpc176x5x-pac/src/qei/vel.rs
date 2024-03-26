#[doc = "Register `VEL` reader"]
pub type R = crate::R<VEL_SPEC>;
#[doc = "Field `VELPC` reader - Current velocity pulse count."]
pub type VELPC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current velocity pulse count."]
    #[inline(always)]
    pub fn velpc(&self) -> VELPC_R {
        VELPC_R::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VEL")
            .field("velpc", &format_args!("{}", self.velpc().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<VEL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Velocity counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vel::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VEL_SPEC;
impl crate::RegisterSpec for VEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vel::R`](R) reader structure"]
impl crate::Readable for VEL_SPEC {}
#[doc = "`reset()` method sets VEL to value 0"]
impl crate::Resettable for VEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
