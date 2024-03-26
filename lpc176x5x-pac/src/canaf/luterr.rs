#[doc = "Register `LUTERR` reader"]
pub type R = crate::R<LUTERR_SPEC>;
#[doc = "Field `LUTERR` reader - This read-only bit is set to 1 if the Acceptance Filter encounters an error in the content of the tables in AF RAM. It is cleared when software reads the LUTerrAd register. This condition is ORed with the other CAN interrupts from the CAN controllers, to produce the request that is connected to the NVIC."]
pub type LUTERR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This read-only bit is set to 1 if the Acceptance Filter encounters an error in the content of the tables in AF RAM. It is cleared when software reads the LUTerrAd register. This condition is ORed with the other CAN interrupts from the CAN controllers, to produce the request that is connected to the NVIC."]
    #[inline(always)]
    pub fn luterr(&self) -> LUTERR_R {
        LUTERR_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LUTERR")
            .field("luterr", &format_args!("{}", self.luterr().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<LUTERR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "LUT Error Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`luterr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LUTERR_SPEC;
impl crate::RegisterSpec for LUTERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`luterr::R`](R) reader structure"]
impl crate::Readable for LUTERR_SPEC {}
#[doc = "`reset()` method sets LUTERR to value 0"]
impl crate::Resettable for LUTERR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
