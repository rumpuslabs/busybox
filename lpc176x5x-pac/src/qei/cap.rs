#[doc = "Register `CAP` reader"]
pub type R = crate::R<CAP_SPEC>;
#[doc = "Field `VELCAP` reader - Last velocity capture."]
pub type VELCAP_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Last velocity capture."]
    #[inline(always)]
    pub fn velcap(&self) -> VELCAP_R {
        VELCAP_R::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP")
            .field("velcap", &format_args!("{}", self.velcap().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Velocity capture register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAP_SPEC;
impl crate::RegisterSpec for CAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap::R`](R) reader structure"]
impl crate::Readable for CAP_SPEC {}
#[doc = "`reset()` method sets CAP to value 0xffff_ffff"]
impl crate::Resettable for CAP_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
