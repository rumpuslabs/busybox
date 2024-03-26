#[doc = "Register `RXFILTERCTRL` reader"]
pub type R = crate::R<RXFILTERCTRL_SPEC>;
#[doc = "Register `RXFILTERCTRL` writer"]
pub type W = crate::W<RXFILTERCTRL_SPEC>;
#[doc = "Field `AUE` reader - AcceptUnicastEn. When set to 1, all unicast frames are accepted."]
pub type AUE_R = crate::BitReader;
#[doc = "Field `AUE` writer - AcceptUnicastEn. When set to 1, all unicast frames are accepted."]
pub type AUE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ABE` reader - AcceptBroadcastEn. When set to 1, all broadcast frames are accepted."]
pub type ABE_R = crate::BitReader;
#[doc = "Field `ABE` writer - AcceptBroadcastEn. When set to 1, all broadcast frames are accepted."]
pub type ABE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AME` reader - AcceptMulticastEn. When set to 1, all multicast frames are accepted."]
pub type AME_R = crate::BitReader;
#[doc = "Field `AME` writer - AcceptMulticastEn. When set to 1, all multicast frames are accepted."]
pub type AME_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AUHE` reader - AcceptUnicastHashEn. When set to 1, unicast frames that pass the imperfect hash filter are accepted."]
pub type AUHE_R = crate::BitReader;
#[doc = "Field `AUHE` writer - AcceptUnicastHashEn. When set to 1, unicast frames that pass the imperfect hash filter are accepted."]
pub type AUHE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AMHE` reader - AcceptMulticastHashEn. When set to 1, multicast frames that pass the imperfect hash filter are accepted."]
pub type AMHE_R = crate::BitReader;
#[doc = "Field `AMHE` writer - AcceptMulticastHashEn. When set to 1, multicast frames that pass the imperfect hash filter are accepted."]
pub type AMHE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `APE` reader - AcceptPerfectEn. When set to 1, the frames with a destination address identical to the station address are accepted."]
pub type APE_R = crate::BitReader;
#[doc = "Field `APE` writer - AcceptPerfectEn. When set to 1, the frames with a destination address identical to the station address are accepted."]
pub type APE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MPEW` reader - MagicPacketEnWoL. When set to 1, the result of the magic packet filter will generate a WoL interrupt when there is a match."]
pub type MPEW_R = crate::BitReader;
#[doc = "Field `MPEW` writer - MagicPacketEnWoL. When set to 1, the result of the magic packet filter will generate a WoL interrupt when there is a match."]
pub type MPEW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RFEW` reader - RxFilterEnWoL. When set to 1, the result of the perfect address matching filter and the imperfect hash filter will generate a WoL interrupt when there is a match."]
pub type RFEW_R = crate::BitReader;
#[doc = "Field `RFEW` writer - RxFilterEnWoL. When set to 1, the result of the perfect address matching filter and the imperfect hash filter will generate a WoL interrupt when there is a match."]
pub type RFEW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - AcceptUnicastEn. When set to 1, all unicast frames are accepted."]
    #[inline(always)]
    pub fn aue(&self) -> AUE_R {
        AUE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AcceptBroadcastEn. When set to 1, all broadcast frames are accepted."]
    #[inline(always)]
    pub fn abe(&self) -> ABE_R {
        ABE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AcceptMulticastEn. When set to 1, all multicast frames are accepted."]
    #[inline(always)]
    pub fn ame(&self) -> AME_R {
        AME_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AcceptUnicastHashEn. When set to 1, unicast frames that pass the imperfect hash filter are accepted."]
    #[inline(always)]
    pub fn auhe(&self) -> AUHE_R {
        AUHE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AcceptMulticastHashEn. When set to 1, multicast frames that pass the imperfect hash filter are accepted."]
    #[inline(always)]
    pub fn amhe(&self) -> AMHE_R {
        AMHE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AcceptPerfectEn. When set to 1, the frames with a destination address identical to the station address are accepted."]
    #[inline(always)]
    pub fn ape(&self) -> APE_R {
        APE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 12 - MagicPacketEnWoL. When set to 1, the result of the magic packet filter will generate a WoL interrupt when there is a match."]
    #[inline(always)]
    pub fn mpew(&self) -> MPEW_R {
        MPEW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RxFilterEnWoL. When set to 1, the result of the perfect address matching filter and the imperfect hash filter will generate a WoL interrupt when there is a match."]
    #[inline(always)]
    pub fn rfew(&self) -> RFEW_R {
        RFEW_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXFILTERCTRL")
            .field("aue", &format_args!("{}", self.aue().bit()))
            .field("abe", &format_args!("{}", self.abe().bit()))
            .field("ame", &format_args!("{}", self.ame().bit()))
            .field("auhe", &format_args!("{}", self.auhe().bit()))
            .field("amhe", &format_args!("{}", self.amhe().bit()))
            .field("ape", &format_args!("{}", self.ape().bit()))
            .field("mpew", &format_args!("{}", self.mpew().bit()))
            .field("rfew", &format_args!("{}", self.rfew().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<RXFILTERCTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - AcceptUnicastEn. When set to 1, all unicast frames are accepted."]
    #[inline(always)]
    #[must_use]
    pub fn aue(&mut self) -> AUE_W<RXFILTERCTRL_SPEC, 0> {
        AUE_W::new(self)
    }
    #[doc = "Bit 1 - AcceptBroadcastEn. When set to 1, all broadcast frames are accepted."]
    #[inline(always)]
    #[must_use]
    pub fn abe(&mut self) -> ABE_W<RXFILTERCTRL_SPEC, 1> {
        ABE_W::new(self)
    }
    #[doc = "Bit 2 - AcceptMulticastEn. When set to 1, all multicast frames are accepted."]
    #[inline(always)]
    #[must_use]
    pub fn ame(&mut self) -> AME_W<RXFILTERCTRL_SPEC, 2> {
        AME_W::new(self)
    }
    #[doc = "Bit 3 - AcceptUnicastHashEn. When set to 1, unicast frames that pass the imperfect hash filter are accepted."]
    #[inline(always)]
    #[must_use]
    pub fn auhe(&mut self) -> AUHE_W<RXFILTERCTRL_SPEC, 3> {
        AUHE_W::new(self)
    }
    #[doc = "Bit 4 - AcceptMulticastHashEn. When set to 1, multicast frames that pass the imperfect hash filter are accepted."]
    #[inline(always)]
    #[must_use]
    pub fn amhe(&mut self) -> AMHE_W<RXFILTERCTRL_SPEC, 4> {
        AMHE_W::new(self)
    }
    #[doc = "Bit 5 - AcceptPerfectEn. When set to 1, the frames with a destination address identical to the station address are accepted."]
    #[inline(always)]
    #[must_use]
    pub fn ape(&mut self) -> APE_W<RXFILTERCTRL_SPEC, 5> {
        APE_W::new(self)
    }
    #[doc = "Bit 12 - MagicPacketEnWoL. When set to 1, the result of the magic packet filter will generate a WoL interrupt when there is a match."]
    #[inline(always)]
    #[must_use]
    pub fn mpew(&mut self) -> MPEW_W<RXFILTERCTRL_SPEC, 12> {
        MPEW_W::new(self)
    }
    #[doc = "Bit 13 - RxFilterEnWoL. When set to 1, the result of the perfect address matching filter and the imperfect hash filter will generate a WoL interrupt when there is a match."]
    #[inline(always)]
    #[must_use]
    pub fn rfew(&mut self) -> RFEW_W<RXFILTERCTRL_SPEC, 13> {
        RFEW_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Receive filter control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxfilterctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxfilterctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXFILTERCTRL_SPEC;
impl crate::RegisterSpec for RXFILTERCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxfilterctrl::R`](R) reader structure"]
impl crate::Readable for RXFILTERCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxfilterctrl::W`](W) writer structure"]
impl crate::Writable for RXFILTERCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXFILTERCTRL to value 0"]
impl crate::Resettable for RXFILTERCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
