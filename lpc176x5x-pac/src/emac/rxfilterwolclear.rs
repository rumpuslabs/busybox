#[doc = "Register `RXFILTERWOLCLEAR` writer"]
pub type W = crate::W<RXFILTERWOLCLEAR_SPEC>;
#[doc = "Field `AUWCLR` writer - AcceptUnicastWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
pub type AUWCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ABWCLR` writer - AcceptBroadcastWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
pub type ABWCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AMWCLR` writer - AcceptMulticastWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
pub type AMWCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AUHWCLR` writer - AcceptUnicastHashWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
pub type AUHWCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AMHWCLR` writer - AcceptMulticastHashWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
pub type AMHWCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `APWCLR` writer - AcceptPerfectWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
pub type APWCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RFWCLR` writer - RxFilterWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
pub type RFWCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MPWCLR` writer - MagicPacketWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
pub type MPWCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<RXFILTERWOLCLEAR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - AcceptUnicastWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn auwclr(&mut self) -> AUWCLR_W<RXFILTERWOLCLEAR_SPEC, 0> {
        AUWCLR_W::new(self)
    }
    #[doc = "Bit 1 - AcceptBroadcastWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn abwclr(&mut self) -> ABWCLR_W<RXFILTERWOLCLEAR_SPEC, 1> {
        ABWCLR_W::new(self)
    }
    #[doc = "Bit 2 - AcceptMulticastWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn amwclr(&mut self) -> AMWCLR_W<RXFILTERWOLCLEAR_SPEC, 2> {
        AMWCLR_W::new(self)
    }
    #[doc = "Bit 3 - AcceptUnicastHashWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn auhwclr(&mut self) -> AUHWCLR_W<RXFILTERWOLCLEAR_SPEC, 3> {
        AUHWCLR_W::new(self)
    }
    #[doc = "Bit 4 - AcceptMulticastHashWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn amhwclr(&mut self) -> AMHWCLR_W<RXFILTERWOLCLEAR_SPEC, 4> {
        AMHWCLR_W::new(self)
    }
    #[doc = "Bit 5 - AcceptPerfectWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn apwclr(&mut self) -> APWCLR_W<RXFILTERWOLCLEAR_SPEC, 5> {
        APWCLR_W::new(self)
    }
    #[doc = "Bit 7 - RxFilterWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn rfwclr(&mut self) -> RFWCLR_W<RXFILTERWOLCLEAR_SPEC, 7> {
        RFWCLR_W::new(self)
    }
    #[doc = "Bit 8 - MagicPacketWoLClr. When a 1 is written, the corresponding status bit in the RxFilterWoLStatus register is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn mpwclr(&mut self) -> MPWCLR_W<RXFILTERWOLCLEAR_SPEC, 8> {
        MPWCLR_W::new(self)
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
#[doc = "Receive filter WoL clear register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxfilterwolclear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXFILTERWOLCLEAR_SPEC;
impl crate::RegisterSpec for RXFILTERWOLCLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rxfilterwolclear::W`](W) writer structure"]
impl crate::Writable for RXFILTERWOLCLEAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXFILTERWOLCLEAR to value 0"]
impl crate::Resettable for RXFILTERWOLCLEAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
