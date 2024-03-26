#[doc = "Register `RAWINTTCSTAT` reader"]
pub type R = crate::R<RAWINTTCSTAT_SPEC>;
#[doc = "Field `RAWINTTCSTAT0` reader - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub type RAWINTTCSTAT0_R = crate::BitReader;
#[doc = "Field `RAWINTTCSTAT1` reader - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub type RAWINTTCSTAT1_R = crate::BitReader;
#[doc = "Field `RAWINTTCSTAT2` reader - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub type RAWINTTCSTAT2_R = crate::BitReader;
#[doc = "Field `RAWINTTCSTAT3` reader - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub type RAWINTTCSTAT3_R = crate::BitReader;
#[doc = "Field `RAWINTTCSTAT4` reader - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub type RAWINTTCSTAT4_R = crate::BitReader;
#[doc = "Field `RAWINTTCSTAT5` reader - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub type RAWINTTCSTAT5_R = crate::BitReader;
#[doc = "Field `RAWINTTCSTAT6` reader - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub type RAWINTTCSTAT6_R = crate::BitReader;
#[doc = "Field `RAWINTTCSTAT7` reader - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
pub type RAWINTTCSTAT7_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn rawinttcstat0(&self) -> RAWINTTCSTAT0_R {
        RAWINTTCSTAT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn rawinttcstat1(&self) -> RAWINTTCSTAT1_R {
        RAWINTTCSTAT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn rawinttcstat2(&self) -> RAWINTTCSTAT2_R {
        RAWINTTCSTAT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn rawinttcstat3(&self) -> RAWINTTCSTAT3_R {
        RAWINTTCSTAT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn rawinttcstat4(&self) -> RAWINTTCSTAT4_R {
        RAWINTTCSTAT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn rawinttcstat5(&self) -> RAWINTTCSTAT5_R {
        RAWINTTCSTAT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn rawinttcstat6(&self) -> RAWINTTCSTAT6_R {
        RAWINTTCSTAT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Status of the terminal count interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active terminal count interrupt request. 1 - the corresponding channel does have an active terminal count interrupt request."]
    #[inline(always)]
    pub fn rawinttcstat7(&self) -> RAWINTTCSTAT7_R {
        RAWINTTCSTAT7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAWINTTCSTAT")
            .field(
                "rawinttcstat0",
                &format_args!("{}", self.rawinttcstat0().bit()),
            )
            .field(
                "rawinttcstat1",
                &format_args!("{}", self.rawinttcstat1().bit()),
            )
            .field(
                "rawinttcstat2",
                &format_args!("{}", self.rawinttcstat2().bit()),
            )
            .field(
                "rawinttcstat3",
                &format_args!("{}", self.rawinttcstat3().bit()),
            )
            .field(
                "rawinttcstat4",
                &format_args!("{}", self.rawinttcstat4().bit()),
            )
            .field(
                "rawinttcstat5",
                &format_args!("{}", self.rawinttcstat5().bit()),
            )
            .field(
                "rawinttcstat6",
                &format_args!("{}", self.rawinttcstat6().bit()),
            )
            .field(
                "rawinttcstat7",
                &format_args!("{}", self.rawinttcstat7().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<RAWINTTCSTAT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "DMA Raw Interrupt Terminal Count Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rawinttcstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAWINTTCSTAT_SPEC;
impl crate::RegisterSpec for RAWINTTCSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rawinttcstat::R`](R) reader structure"]
impl crate::Readable for RAWINTTCSTAT_SPEC {}
#[doc = "`reset()` method sets RAWINTTCSTAT to value 0"]
impl crate::Resettable for RAWINTTCSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
