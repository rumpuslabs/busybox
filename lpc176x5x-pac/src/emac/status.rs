#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `RXSTATUS` reader - If 1, the receive channel is active. If 0, the receive channel is inactive."]
pub type RXSTATUS_R = crate::BitReader;
#[doc = "Field `TXSTATUS` reader - If 1, the transmit channel is active. If 0, the transmit channel is inactive."]
pub type TXSTATUS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - If 1, the receive channel is active. If 0, the receive channel is inactive."]
    #[inline(always)]
    pub fn rxstatus(&self) -> RXSTATUS_R {
        RXSTATUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If 1, the transmit channel is active. If 0, the transmit channel is inactive."]
    #[inline(always)]
    pub fn txstatus(&self) -> TXSTATUS_R {
        TXSTATUS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("rxstatus", &format_args!("{}", self.rxstatus().bit()))
            .field("txstatus", &format_args!("{}", self.txstatus().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
