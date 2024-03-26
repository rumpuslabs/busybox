#[doc = "Register `RIS` reader"]
pub type R = crate::R<RIS_SPEC>;
#[doc = "Field `RORRIS` reader - This bit is 1 if another frame was completely received while the RxFIFO was full. The ARM spec implies that the preceding frame data is overwritten by the new frame data when this occurs."]
pub type RORRIS_R = crate::BitReader;
#[doc = "Field `RTRIS` reader - This bit is 1 if the Rx FIFO is not empty, and has not been read for a time-out period. The time-out period is the same for master and slave modes and is determined by the SSP bit rate: 32 bits at PCLK / (CPSDVSR X \\[SCR+1\\])."]
pub type RTRIS_R = crate::BitReader;
#[doc = "Field `RXRIS` reader - This bit is 1 if the Rx FIFO is at least half full."]
pub type RXRIS_R = crate::BitReader;
#[doc = "Field `TXRIS` reader - This bit is 1 if the Tx FIFO is at least half empty."]
pub type TXRIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This bit is 1 if another frame was completely received while the RxFIFO was full. The ARM spec implies that the preceding frame data is overwritten by the new frame data when this occurs."]
    #[inline(always)]
    pub fn rorris(&self) -> RORRIS_R {
        RORRIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit is 1 if the Rx FIFO is not empty, and has not been read for a time-out period. The time-out period is the same for master and slave modes and is determined by the SSP bit rate: 32 bits at PCLK / (CPSDVSR X \\[SCR+1\\])."]
    #[inline(always)]
    pub fn rtris(&self) -> RTRIS_R {
        RTRIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit is 1 if the Rx FIFO is at least half full."]
    #[inline(always)]
    pub fn rxris(&self) -> RXRIS_R {
        RXRIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit is 1 if the Tx FIFO is at least half empty."]
    #[inline(always)]
    pub fn txris(&self) -> TXRIS_R {
        TXRIS_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RIS")
            .field("rorris", &format_args!("{}", self.rorris().bit()))
            .field("rtris", &format_args!("{}", self.rtris().bit()))
            .field("rxris", &format_args!("{}", self.rxris().bit()))
            .field("txris", &format_args!("{}", self.txris().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<RIS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Raw Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RIS_SPEC {}
#[doc = "`reset()` method sets RIS to value 0x08"]
impl crate::Resettable for RIS_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
