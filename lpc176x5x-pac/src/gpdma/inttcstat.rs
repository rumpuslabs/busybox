#[doc = "Register `INTTCSTAT` reader"]
pub type R = crate::R<INTTCSTAT_SPEC>;
#[doc = "Field `INTTCSTAT0` reader - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub type INTTCSTAT0_R = crate::BitReader;
#[doc = "Field `INTTCSTAT1` reader - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub type INTTCSTAT1_R = crate::BitReader;
#[doc = "Field `INTTCSTAT2` reader - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub type INTTCSTAT2_R = crate::BitReader;
#[doc = "Field `INTTCSTAT3` reader - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub type INTTCSTAT3_R = crate::BitReader;
#[doc = "Field `INTTCSTAT4` reader - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub type INTTCSTAT4_R = crate::BitReader;
#[doc = "Field `INTTCSTAT5` reader - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub type INTTCSTAT5_R = crate::BitReader;
#[doc = "Field `INTTCSTAT6` reader - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub type INTTCSTAT6_R = crate::BitReader;
#[doc = "Field `INTTCSTAT7` reader - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub type INTTCSTAT7_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn inttcstat0(&self) -> INTTCSTAT0_R {
        INTTCSTAT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn inttcstat1(&self) -> INTTCSTAT1_R {
        INTTCSTAT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn inttcstat2(&self) -> INTTCSTAT2_R {
        INTTCSTAT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn inttcstat3(&self) -> INTTCSTAT3_R {
        INTTCSTAT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn inttcstat4(&self) -> INTTCSTAT4_R {
        INTTCSTAT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn inttcstat5(&self) -> INTTCSTAT5_R {
        INTTCSTAT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn inttcstat6(&self) -> INTTCSTAT6_R {
        INTTCSTAT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Terminal count interrupt request status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn inttcstat7(&self) -> INTTCSTAT7_R {
        INTTCSTAT7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTTCSTAT")
            .field("inttcstat0", &format_args!("{}", self.inttcstat0().bit()))
            .field("inttcstat1", &format_args!("{}", self.inttcstat1().bit()))
            .field("inttcstat2", &format_args!("{}", self.inttcstat2().bit()))
            .field("inttcstat3", &format_args!("{}", self.inttcstat3().bit()))
            .field("inttcstat4", &format_args!("{}", self.inttcstat4().bit()))
            .field("inttcstat5", &format_args!("{}", self.inttcstat5().bit()))
            .field("inttcstat6", &format_args!("{}", self.inttcstat6().bit()))
            .field("inttcstat7", &format_args!("{}", self.inttcstat7().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<INTTCSTAT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "DMA Interrupt Terminal Count Request Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inttcstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTTCSTAT_SPEC;
impl crate::RegisterSpec for INTTCSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inttcstat::R`](R) reader structure"]
impl crate::Readable for INTTCSTAT_SPEC {}
#[doc = "`reset()` method sets INTTCSTAT to value 0"]
impl crate::Resettable for INTTCSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
