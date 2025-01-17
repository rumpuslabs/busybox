#[doc = "Register `MIS` reader"]
pub type R = crate::R<MIS_SPEC>;
#[doc = "Field `RORMIS` reader - This bit is 1 if another frame was completely received while the RxFIFO was full, and this interrupt is enabled."]
pub type RORMIS_R = crate::BitReader;
#[doc = "Field `RTMIS` reader - This bit is 1 if the Rx FIFO is not empty, has not been read for a time-out period, and this interrupt is enabled. The time-out period is the same for master and slave modes and is determined by the SSP bit rate: 32 bits at PCLK / (CPSDVSR X \\[SCR+1\\])."]
pub type RTMIS_R = crate::BitReader;
#[doc = "Field `RXMIS` reader - This bit is 1 if the Rx FIFO is at least half full, and this interrupt is enabled."]
pub type RXMIS_R = crate::BitReader;
#[doc = "Field `TXMIS` reader - This bit is 1 if the Tx FIFO is at least half empty, and this interrupt is enabled."]
pub type TXMIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This bit is 1 if another frame was completely received while the RxFIFO was full, and this interrupt is enabled."]
    #[inline(always)]
    pub fn rormis(&self) -> RORMIS_R {
        RORMIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit is 1 if the Rx FIFO is not empty, has not been read for a time-out period, and this interrupt is enabled. The time-out period is the same for master and slave modes and is determined by the SSP bit rate: 32 bits at PCLK / (CPSDVSR X \\[SCR+1\\])."]
    #[inline(always)]
    pub fn rtmis(&self) -> RTMIS_R {
        RTMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit is 1 if the Rx FIFO is at least half full, and this interrupt is enabled."]
    #[inline(always)]
    pub fn rxmis(&self) -> RXMIS_R {
        RXMIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit is 1 if the Tx FIFO is at least half empty, and this interrupt is enabled."]
    #[inline(always)]
    pub fn txmis(&self) -> TXMIS_R {
        TXMIS_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MIS")
            .field("rormis", &format_args!("{}", self.rormis().bit()))
            .field("rtmis", &format_args!("{}", self.rtmis().bit()))
            .field("rxmis", &format_args!("{}", self.rxmis().bit()))
            .field("txmis", &format_args!("{}", self.txmis().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<MIS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Masked Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MIS_SPEC;
impl crate::RegisterSpec for MIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MIS_SPEC {}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
