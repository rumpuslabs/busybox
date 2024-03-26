#[doc = "Register `FEED` writer"]
pub type W = crate::W<FEED_SPEC>;
#[doc = "Field `Feed` writer - Feed value should be 0xAA followed by 0x55."]
pub type FEED_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<FEED_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - Feed value should be 0xAA followed by 0x55."]
    #[inline(always)]
    #[must_use]
    pub fn feed(&mut self) -> FEED_W<FEED_SPEC, 0> {
        FEED_W::new(self)
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
#[doc = "Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register reloads the Watchdog timer with the value contained in WDTC.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`feed::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FEED_SPEC;
impl crate::RegisterSpec for FEED_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`feed::W`](W) writer structure"]
impl crate::Writable for FEED_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FEED to value 0"]
impl crate::Resettable for FEED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
