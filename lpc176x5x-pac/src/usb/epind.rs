#[doc = "Register `EPIND` writer"]
pub type W = crate::W<EPIND_SPEC>;
#[doc = "Field `PHY_EP` writer - Physical endpoint number (0-31)"]
pub type PHY_EP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<EPIND_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:4 - Physical endpoint number (0-31)"]
    #[inline(always)]
    #[must_use]
    pub fn phy_ep(&mut self) -> PHY_EP_W<EPIND_SPEC, 0> {
        PHY_EP_W::new(self)
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
#[doc = "USB Endpoint Index\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epind::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPIND_SPEC;
impl crate::RegisterSpec for EPIND_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`epind::W`](W) writer structure"]
impl crate::Writable for EPIND_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EPIND to value 0"]
impl crate::Resettable for EPIND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
