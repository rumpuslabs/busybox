#[doc = "Register `SUPP` reader"]
pub type R = crate::R<SUPP_SPEC>;
#[doc = "Register `SUPP` writer"]
pub type W = crate::W<SUPP_SPEC>;
#[doc = "Field `SPEED` reader - This bit configures the Reduced MII logic for the current operating speed. When set, 100 Mbps mode is selected. When cleared, 10 Mbps mode is selected."]
pub type SPEED_R = crate::BitReader;
#[doc = "Field `SPEED` writer - This bit configures the Reduced MII logic for the current operating speed. When set, 100 Mbps mode is selected. When cleared, 10 Mbps mode is selected."]
pub type SPEED_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 8 - This bit configures the Reduced MII logic for the current operating speed. When set, 100 Mbps mode is selected. When cleared, 10 Mbps mode is selected."]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SUPP")
            .field("speed", &format_args!("{}", self.speed().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<SUPP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 8 - This bit configures the Reduced MII logic for the current operating speed. When set, 100 Mbps mode is selected. When cleared, 10 Mbps mode is selected."]
    #[inline(always)]
    #[must_use]
    pub fn speed(&mut self) -> SPEED_W<SUPP_SPEC, 8> {
        SPEED_W::new(self)
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
#[doc = "PHY Support register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`supp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`supp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SUPP_SPEC;
impl crate::RegisterSpec for SUPP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`supp::R`](R) reader structure"]
impl crate::Readable for SUPP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`supp::W`](W) writer structure"]
impl crate::Writable for SUPP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SUPP to value 0"]
impl crate::Resettable for SUPP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
