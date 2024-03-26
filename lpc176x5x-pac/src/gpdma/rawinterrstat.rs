#[doc = "Register `RAWINTERRSTAT` reader"]
pub type R = crate::R<RAWINTERRSTAT_SPEC>;
#[doc = "Field `RAWINTERRSTAT0` reader - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub type RAWINTERRSTAT0_R = crate::BitReader;
#[doc = "Field `RAWINTERRSTAT1` reader - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub type RAWINTERRSTAT1_R = crate::BitReader;
#[doc = "Field `RAWINTERRSTAT2` reader - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub type RAWINTERRSTAT2_R = crate::BitReader;
#[doc = "Field `RAWINTERRSTAT3` reader - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub type RAWINTERRSTAT3_R = crate::BitReader;
#[doc = "Field `RAWINTERRSTAT4` reader - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub type RAWINTERRSTAT4_R = crate::BitReader;
#[doc = "Field `RAWINTERRSTAT5` reader - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub type RAWINTERRSTAT5_R = crate::BitReader;
#[doc = "Field `RAWINTERRSTAT6` reader - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub type RAWINTERRSTAT6_R = crate::BitReader;
#[doc = "Field `RAWINTERRSTAT7` reader - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub type RAWINTERRSTAT7_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat0(&self) -> RAWINTERRSTAT0_R {
        RAWINTERRSTAT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat1(&self) -> RAWINTERRSTAT1_R {
        RAWINTERRSTAT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat2(&self) -> RAWINTERRSTAT2_R {
        RAWINTERRSTAT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat3(&self) -> RAWINTERRSTAT3_R {
        RAWINTERRSTAT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat4(&self) -> RAWINTERRSTAT4_R {
        RAWINTERRSTAT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat5(&self) -> RAWINTERRSTAT5_R {
        RAWINTERRSTAT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat6(&self) -> RAWINTERRSTAT6_R {
        RAWINTERRSTAT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat7(&self) -> RAWINTERRSTAT7_R {
        RAWINTERRSTAT7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAWINTERRSTAT")
            .field(
                "rawinterrstat0",
                &format_args!("{}", self.rawinterrstat0().bit()),
            )
            .field(
                "rawinterrstat1",
                &format_args!("{}", self.rawinterrstat1().bit()),
            )
            .field(
                "rawinterrstat2",
                &format_args!("{}", self.rawinterrstat2().bit()),
            )
            .field(
                "rawinterrstat3",
                &format_args!("{}", self.rawinterrstat3().bit()),
            )
            .field(
                "rawinterrstat4",
                &format_args!("{}", self.rawinterrstat4().bit()),
            )
            .field(
                "rawinterrstat5",
                &format_args!("{}", self.rawinterrstat5().bit()),
            )
            .field(
                "rawinterrstat6",
                &format_args!("{}", self.rawinterrstat6().bit()),
            )
            .field(
                "rawinterrstat7",
                &format_args!("{}", self.rawinterrstat7().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<RAWINTERRSTAT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "DMA Raw Error Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rawinterrstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAWINTERRSTAT_SPEC;
impl crate::RegisterSpec for RAWINTERRSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rawinterrstat::R`](R) reader structure"]
impl crate::Readable for RAWINTERRSTAT_SPEC {}
#[doc = "`reset()` method sets RAWINTERRSTAT to value 0"]
impl crate::Resettable for RAWINTERRSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
