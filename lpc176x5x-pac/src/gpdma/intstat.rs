#[doc = "Register `INTSTAT` reader"]
pub type R = crate::R<INTSTAT_SPEC>;
#[doc = "Field `INTSTAT0` reader - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
pub type INTSTAT0_R = crate::BitReader;
#[doc = "Field `INTSTAT1` reader - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
pub type INTSTAT1_R = crate::BitReader;
#[doc = "Field `INTSTAT2` reader - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
pub type INTSTAT2_R = crate::BitReader;
#[doc = "Field `INTSTAT3` reader - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
pub type INTSTAT3_R = crate::BitReader;
#[doc = "Field `INTSTAT4` reader - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
pub type INTSTAT4_R = crate::BitReader;
#[doc = "Field `INTSTAT5` reader - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
pub type INTSTAT5_R = crate::BitReader;
#[doc = "Field `INTSTAT6` reader - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
pub type INTSTAT6_R = crate::BitReader;
#[doc = "Field `INTSTAT7` reader - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
pub type INTSTAT7_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
    #[inline(always)]
    pub fn intstat0(&self) -> INTSTAT0_R {
        INTSTAT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
    #[inline(always)]
    pub fn intstat1(&self) -> INTSTAT1_R {
        INTSTAT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
    #[inline(always)]
    pub fn intstat2(&self) -> INTSTAT2_R {
        INTSTAT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
    #[inline(always)]
    pub fn intstat3(&self) -> INTSTAT3_R {
        INTSTAT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
    #[inline(always)]
    pub fn intstat4(&self) -> INTSTAT4_R {
        INTSTAT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
    #[inline(always)]
    pub fn intstat5(&self) -> INTSTAT5_R {
        INTSTAT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
    #[inline(always)]
    pub fn intstat6(&self) -> INTSTAT6_R {
        INTSTAT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Status of DMA channel interrupts after masking. Each bit represents one channel: 0 - the corresponding channel has no active interrupt request. 1 - the corresponding channel does have an active interrupt request."]
    #[inline(always)]
    pub fn intstat7(&self) -> INTSTAT7_R {
        INTSTAT7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTSTAT")
            .field("intstat0", &format_args!("{}", self.intstat0().bit()))
            .field("intstat1", &format_args!("{}", self.intstat1().bit()))
            .field("intstat2", &format_args!("{}", self.intstat2().bit()))
            .field("intstat3", &format_args!("{}", self.intstat3().bit()))
            .field("intstat4", &format_args!("{}", self.intstat4().bit()))
            .field("intstat5", &format_args!("{}", self.intstat5().bit()))
            .field("intstat6", &format_args!("{}", self.intstat6().bit()))
            .field("intstat7", &format_args!("{}", self.intstat7().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<INTSTAT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "DMA Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTSTAT_SPEC;
impl crate::RegisterSpec for INTSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstat::R`](R) reader structure"]
impl crate::Readable for INTSTAT_SPEC {}
#[doc = "`reset()` method sets INTSTAT to value 0"]
impl crate::Resettable for INTSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
