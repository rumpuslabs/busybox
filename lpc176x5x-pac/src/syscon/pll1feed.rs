#[doc = "Register `PLL1FEED` writer"]
pub type W = crate::W<PLL1FEED_SPEC>;
#[doc = "Field `PLL1FEED` writer - The PLL1 feed sequence must be written to this register in order for PLL1 configuration and control register changes to take effect."]
pub type PLL1FEED_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<PLL1FEED_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - The PLL1 feed sequence must be written to this register in order for PLL1 configuration and control register changes to take effect."]
    #[inline(always)]
    #[must_use]
    pub fn pll1feed(&mut self) -> PLL1FEED_W<PLL1FEED_SPEC, 0> {
        PLL1FEED_W::new(self)
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
#[doc = "PLL1 Feed Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll1feed::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLL1FEED_SPEC;
impl crate::RegisterSpec for PLL1FEED_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pll1feed::W`](W) writer structure"]
impl crate::Writable for PLL1FEED_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL1FEED to value 0"]
impl crate::Resettable for PLL1FEED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
