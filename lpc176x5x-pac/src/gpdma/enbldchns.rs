#[doc = "Register `ENBLDCHNS` reader"]
pub type R = crate::R<ENBLDCHNS_SPEC>;
#[doc = "Field `ENABLEDCHANNELS0` reader - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
pub type ENABLEDCHANNELS0_R = crate::BitReader;
#[doc = "Field `ENABLEDCHANNELS1` reader - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
pub type ENABLEDCHANNELS1_R = crate::BitReader;
#[doc = "Field `ENABLEDCHANNELS2` reader - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
pub type ENABLEDCHANNELS2_R = crate::BitReader;
#[doc = "Field `ENABLEDCHANNELS3` reader - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
pub type ENABLEDCHANNELS3_R = crate::BitReader;
#[doc = "Field `ENABLEDCHANNELS4` reader - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
pub type ENABLEDCHANNELS4_R = crate::BitReader;
#[doc = "Field `ENABLEDCHANNELS5` reader - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
pub type ENABLEDCHANNELS5_R = crate::BitReader;
#[doc = "Field `ENABLEDCHANNELS6` reader - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
pub type ENABLEDCHANNELS6_R = crate::BitReader;
#[doc = "Field `ENABLEDCHANNELS7` reader - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
pub type ENABLEDCHANNELS7_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
    #[inline(always)]
    pub fn enabledchannels0(&self) -> ENABLEDCHANNELS0_R {
        ENABLEDCHANNELS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
    #[inline(always)]
    pub fn enabledchannels1(&self) -> ENABLEDCHANNELS1_R {
        ENABLEDCHANNELS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
    #[inline(always)]
    pub fn enabledchannels2(&self) -> ENABLEDCHANNELS2_R {
        ENABLEDCHANNELS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
    #[inline(always)]
    pub fn enabledchannels3(&self) -> ENABLEDCHANNELS3_R {
        ENABLEDCHANNELS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
    #[inline(always)]
    pub fn enabledchannels4(&self) -> ENABLEDCHANNELS4_R {
        ENABLEDCHANNELS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
    #[inline(always)]
    pub fn enabledchannels5(&self) -> ENABLEDCHANNELS5_R {
        ENABLEDCHANNELS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
    #[inline(always)]
    pub fn enabledchannels6(&self) -> ENABLEDCHANNELS6_R {
        ENABLEDCHANNELS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
    #[inline(always)]
    pub fn enabledchannels7(&self) -> ENABLEDCHANNELS7_R {
        ENABLEDCHANNELS7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENBLDCHNS")
            .field(
                "enabledchannels0",
                &format_args!("{}", self.enabledchannels0().bit()),
            )
            .field(
                "enabledchannels1",
                &format_args!("{}", self.enabledchannels1().bit()),
            )
            .field(
                "enabledchannels2",
                &format_args!("{}", self.enabledchannels2().bit()),
            )
            .field(
                "enabledchannels3",
                &format_args!("{}", self.enabledchannels3().bit()),
            )
            .field(
                "enabledchannels4",
                &format_args!("{}", self.enabledchannels4().bit()),
            )
            .field(
                "enabledchannels5",
                &format_args!("{}", self.enabledchannels5().bit()),
            )
            .field(
                "enabledchannels6",
                &format_args!("{}", self.enabledchannels6().bit()),
            )
            .field(
                "enabledchannels7",
                &format_args!("{}", self.enabledchannels7().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<ENBLDCHNS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "DMA Enabled Channel Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enbldchns::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENBLDCHNS_SPEC;
impl crate::RegisterSpec for ENBLDCHNS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enbldchns::R`](R) reader structure"]
impl crate::Readable for ENBLDCHNS_SPEC {}
#[doc = "`reset()` method sets ENBLDCHNS to value 0"]
impl crate::Resettable for ENBLDCHNS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
