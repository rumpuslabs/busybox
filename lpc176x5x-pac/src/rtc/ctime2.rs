#[doc = "Register `CTIME2` reader"]
pub type R = crate::R<CTIME2_SPEC>;
#[doc = "Field `DOY` reader - Day of year value in the range of 1 to 365 (366 for leap years)."]
pub type DOY_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Day of year value in the range of 1 to 365 (366 for leap years)."]
    #[inline(always)]
    pub fn doy(&self) -> DOY_R {
        DOY_R::new((self.bits & 0x0fff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTIME2")
            .field("doy", &format_args!("{}", self.doy().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CTIME2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Consolidated Time Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctime2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTIME2_SPEC;
impl crate::RegisterSpec for CTIME2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctime2::R`](R) reader structure"]
impl crate::Readable for CTIME2_SPEC {}
#[doc = "`reset()` method sets CTIME2 to value 0"]
impl crate::Resettable for CTIME2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
