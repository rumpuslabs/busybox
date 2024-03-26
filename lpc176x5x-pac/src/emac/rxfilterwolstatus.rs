#[doc = "Register `RXFILTERWOLSTATUS` reader"]
pub type R = crate::R<RXFILTERWOLSTATUS_SPEC>;
#[doc = "Field `AUW` reader - AcceptUnicastWoL. When the value is 1, a unicast frames caused WoL."]
pub type AUW_R = crate::BitReader;
#[doc = "Field `ABW` reader - AcceptBroadcastWoL. When the value is 1, a broadcast frame caused WoL."]
pub type ABW_R = crate::BitReader;
#[doc = "Field `AMW` reader - AcceptMulticastWoL. When the value is 1, a multicast frame caused WoL."]
pub type AMW_R = crate::BitReader;
#[doc = "Field `AUHW` reader - AcceptUnicastHashWoL. When the value is 1, a unicast frame that passes the imperfect hash filter caused WoL."]
pub type AUHW_R = crate::BitReader;
#[doc = "Field `AMHW` reader - AcceptMulticastHashWoL. When the value is 1, a multicast frame that passes the imperfect hash filter caused WoL."]
pub type AMHW_R = crate::BitReader;
#[doc = "Field `APW` reader - AcceptPerfectWoL. When the value is 1, the perfect address matching filter caused WoL."]
pub type APW_R = crate::BitReader;
#[doc = "Field `RFW` reader - RxFilterWoL. When the value is 1, the receive filter caused WoL."]
pub type RFW_R = crate::BitReader;
#[doc = "Field `MPW` reader - MagicPacketWoL. When the value is 1, the magic packet filter caused WoL."]
pub type MPW_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - AcceptUnicastWoL. When the value is 1, a unicast frames caused WoL."]
    #[inline(always)]
    pub fn auw(&self) -> AUW_R {
        AUW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AcceptBroadcastWoL. When the value is 1, a broadcast frame caused WoL."]
    #[inline(always)]
    pub fn abw(&self) -> ABW_R {
        ABW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AcceptMulticastWoL. When the value is 1, a multicast frame caused WoL."]
    #[inline(always)]
    pub fn amw(&self) -> AMW_R {
        AMW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AcceptUnicastHashWoL. When the value is 1, a unicast frame that passes the imperfect hash filter caused WoL."]
    #[inline(always)]
    pub fn auhw(&self) -> AUHW_R {
        AUHW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AcceptMulticastHashWoL. When the value is 1, a multicast frame that passes the imperfect hash filter caused WoL."]
    #[inline(always)]
    pub fn amhw(&self) -> AMHW_R {
        AMHW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AcceptPerfectWoL. When the value is 1, the perfect address matching filter caused WoL."]
    #[inline(always)]
    pub fn apw(&self) -> APW_R {
        APW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - RxFilterWoL. When the value is 1, the receive filter caused WoL."]
    #[inline(always)]
    pub fn rfw(&self) -> RFW_R {
        RFW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MagicPacketWoL. When the value is 1, the magic packet filter caused WoL."]
    #[inline(always)]
    pub fn mpw(&self) -> MPW_R {
        MPW_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXFILTERWOLSTATUS")
            .field("auw", &format_args!("{}", self.auw().bit()))
            .field("abw", &format_args!("{}", self.abw().bit()))
            .field("amw", &format_args!("{}", self.amw().bit()))
            .field("auhw", &format_args!("{}", self.auhw().bit()))
            .field("amhw", &format_args!("{}", self.amhw().bit()))
            .field("apw", &format_args!("{}", self.apw().bit()))
            .field("rfw", &format_args!("{}", self.rfw().bit()))
            .field("mpw", &format_args!("{}", self.mpw().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<RXFILTERWOLSTATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Receive filter WoL status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxfilterwolstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXFILTERWOLSTATUS_SPEC;
impl crate::RegisterSpec for RXFILTERWOLSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxfilterwolstatus::R`](R) reader structure"]
impl crate::Readable for RXFILTERWOLSTATUS_SPEC {}
#[doc = "`reset()` method sets RXFILTERWOLSTATUS to value 0"]
impl crate::Resettable for RXFILTERWOLSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
