#[doc = "Register `MSR` reader"]
pub type R = crate::R<MSR_SPEC>;
#[doc = "Field `E1` reader - When 1, one or both of the CAN1 Tx and Rx Error Counters has reached the limit set in the CAN1EWL register (same as ES in CAN1GSR)"]
pub type E1_R = crate::BitReader;
#[doc = "Field `E2` reader - When 1, one or both of the CAN2 Tx and Rx Error Counters has reached the limit set in the CAN2EWL register (same as ES in CAN2GSR)"]
pub type E2_R = crate::BitReader;
#[doc = "Field `BS1` reader - When 1, the CAN1 controller is currently involved in bus activities (same as BS in CAN1GSR)."]
pub type BS1_R = crate::BitReader;
#[doc = "Field `BS2` reader - When 1, the CAN2 controller is currently involved in bus activities (same as BS in CAN2GSR)."]
pub type BS2_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - When 1, one or both of the CAN1 Tx and Rx Error Counters has reached the limit set in the CAN1EWL register (same as ES in CAN1GSR)"]
    #[inline(always)]
    pub fn e1(&self) -> E1_R {
        E1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When 1, one or both of the CAN2 Tx and Rx Error Counters has reached the limit set in the CAN2EWL register (same as ES in CAN2GSR)"]
    #[inline(always)]
    pub fn e2(&self) -> E2_R {
        E2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - When 1, the CAN1 controller is currently involved in bus activities (same as BS in CAN1GSR)."]
    #[inline(always)]
    pub fn bs1(&self) -> BS1_R {
        BS1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When 1, the CAN2 controller is currently involved in bus activities (same as BS in CAN2GSR)."]
    #[inline(always)]
    pub fn bs2(&self) -> BS2_R {
        BS2_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSR")
            .field("e1", &format_args!("{}", self.e1().bit()))
            .field("e2", &format_args!("{}", self.e2().bit()))
            .field("bs1", &format_args!("{}", self.bs1().bit()))
            .field("bs2", &format_args!("{}", self.bs2().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<MSR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "CAN Central Miscellaneous Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSR_SPEC;
impl crate::RegisterSpec for MSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msr::R`](R) reader structure"]
impl crate::Readable for MSR_SPEC {}
#[doc = "`reset()` method sets MSR to value 0"]
impl crate::Resettable for MSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
