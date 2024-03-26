#[doc = "Register `INTST` reader"]
pub type R = crate::R<INTST_SPEC>;
#[doc = "Field `TMR` reader - Timer time-out."]
pub type TMR_R = crate::BitReader;
#[doc = "Field `REMOVE_PU` reader - Remove pull-up. This bit is set by hardware to indicate that software needs to disable the D+ pull-up resistor."]
pub type REMOVE_PU_R = crate::BitReader;
#[doc = "Field `HNP_FAILURE` reader - HNP failed. This bit is set by hardware to indicate that the HNP switching has failed."]
pub type HNP_FAILURE_R = crate::BitReader;
#[doc = "Field `HNP_SUCCESS` reader - HNP succeeded. This bit is set by hardware to indicate that the HNP switching has succeeded."]
pub type HNP_SUCCESS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Timer time-out."]
    #[inline(always)]
    pub fn tmr(&self) -> TMR_R {
        TMR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Remove pull-up. This bit is set by hardware to indicate that software needs to disable the D+ pull-up resistor."]
    #[inline(always)]
    pub fn remove_pu(&self) -> REMOVE_PU_R {
        REMOVE_PU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HNP failed. This bit is set by hardware to indicate that the HNP switching has failed."]
    #[inline(always)]
    pub fn hnp_failure(&self) -> HNP_FAILURE_R {
        HNP_FAILURE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HNP succeeded. This bit is set by hardware to indicate that the HNP switching has succeeded."]
    #[inline(always)]
    pub fn hnp_success(&self) -> HNP_SUCCESS_R {
        HNP_SUCCESS_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTST")
            .field("tmr", &format_args!("{}", self.tmr().bit()))
            .field("remove_pu", &format_args!("{}", self.remove_pu().bit()))
            .field("hnp_failure", &format_args!("{}", self.hnp_failure().bit()))
            .field("hnp_success", &format_args!("{}", self.hnp_success().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<INTST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "OTG Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intst::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTST_SPEC;
impl crate::RegisterSpec for INTST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intst::R`](R) reader structure"]
impl crate::Readable for INTST_SPEC {}
#[doc = "`reset()` method sets INTST to value 0"]
impl crate::Resettable for INTST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
