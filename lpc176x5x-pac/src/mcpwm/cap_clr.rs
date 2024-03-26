#[doc = "Register `CAP_CLR` writer"]
pub type W = crate::W<CAP_CLR_SPEC>;
#[doc = "Field `CAP_CLR0` writer - Writing a 1 to this bit clears the CAP0 register."]
pub type CAP_CLR0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAP_CLR1` writer - Writing a 1 to this bit clears the CAP1 register."]
pub type CAP_CLR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAP_CLR2` writer - Writing a 1 to this bit clears the CAP2 register."]
pub type CAP_CLR2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CAP_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Writing a 1 to this bit clears the CAP0 register."]
    #[inline(always)]
    #[must_use]
    pub fn cap_clr0(&mut self) -> CAP_CLR0_W<CAP_CLR_SPEC, 0> {
        CAP_CLR0_W::new(self)
    }
    #[doc = "Bit 1 - Writing a 1 to this bit clears the CAP1 register."]
    #[inline(always)]
    #[must_use]
    pub fn cap_clr1(&mut self) -> CAP_CLR1_W<CAP_CLR_SPEC, 1> {
        CAP_CLR1_W::new(self)
    }
    #[doc = "Bit 2 - Writing a 1 to this bit clears the CAP2 register."]
    #[inline(always)]
    #[must_use]
    pub fn cap_clr2(&mut self) -> CAP_CLR2_W<CAP_CLR_SPEC, 2> {
        CAP_CLR2_W::new(self)
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
#[doc = "Capture clear address\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAP_CLR_SPEC;
impl crate::RegisterSpec for CAP_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cap_clr::W`](W) writer structure"]
impl crate::Writable for CAP_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAP_CLR to value 0"]
impl crate::Resettable for CAP_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
