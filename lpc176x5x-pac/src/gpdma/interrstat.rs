#[doc = "Register `INTERRSTAT` reader"]
pub type R = crate::R<INTERRSTAT_SPEC>;
#[doc = "Field `INTERRSTAT0` reader - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub type INTERRSTAT0_R = crate::BitReader;
#[doc = "Field `INTERRSTAT1` reader - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub type INTERRSTAT1_R = crate::BitReader;
#[doc = "Field `INTERRSTAT2` reader - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub type INTERRSTAT2_R = crate::BitReader;
#[doc = "Field `INTERRSTAT3` reader - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub type INTERRSTAT3_R = crate::BitReader;
#[doc = "Field `INTERRSTAT4` reader - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub type INTERRSTAT4_R = crate::BitReader;
#[doc = "Field `INTERRSTAT5` reader - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub type INTERRSTAT5_R = crate::BitReader;
#[doc = "Field `INTERRSTAT6` reader - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub type INTERRSTAT6_R = crate::BitReader;
#[doc = "Field `INTERRSTAT7` reader - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub type INTERRSTAT7_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn interrstat0(&self) -> INTERRSTAT0_R {
        INTERRSTAT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn interrstat1(&self) -> INTERRSTAT1_R {
        INTERRSTAT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn interrstat2(&self) -> INTERRSTAT2_R {
        INTERRSTAT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn interrstat3(&self) -> INTERRSTAT3_R {
        INTERRSTAT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn interrstat4(&self) -> INTERRSTAT4_R {
        INTERRSTAT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn interrstat5(&self) -> INTERRSTAT5_R {
        INTERRSTAT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn interrstat6(&self) -> INTERRSTAT6_R {
        INTERRSTAT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt error status for DMA channels. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn interrstat7(&self) -> INTERRSTAT7_R {
        INTERRSTAT7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTERRSTAT")
            .field("interrstat0", &format_args!("{}", self.interrstat0().bit()))
            .field("interrstat1", &format_args!("{}", self.interrstat1().bit()))
            .field("interrstat2", &format_args!("{}", self.interrstat2().bit()))
            .field("interrstat3", &format_args!("{}", self.interrstat3().bit()))
            .field("interrstat4", &format_args!("{}", self.interrstat4().bit()))
            .field("interrstat5", &format_args!("{}", self.interrstat5().bit()))
            .field("interrstat6", &format_args!("{}", self.interrstat6().bit()))
            .field("interrstat7", &format_args!("{}", self.interrstat7().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<INTERRSTAT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "DMA Interrupt Error Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERRSTAT_SPEC;
impl crate::RegisterSpec for INTERRSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrstat::R`](R) reader structure"]
impl crate::Readable for INTERRSTAT_SPEC {}
#[doc = "`reset()` method sets INTERRSTAT to value 0"]
impl crate::Resettable for INTERRSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
