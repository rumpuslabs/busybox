#[doc = "Register `INTCLR` writer"]
pub type W = crate::W<INTCLR_SPEC>;
#[doc = "Field `TMR_CLR` writer - 0 = no effect. 1 = clear the corresponding bit in the IntSt register."]
pub type TMR_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `REMOVE_PU_CLR` writer - 0 = no effect. 1 = clear the corresponding bit in the IntSt register."]
pub type REMOVE_PU_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HNP_FAILURE_CLR` writer - 0 = no effect. 1 = clear the corresponding bit in the IntSt register."]
pub type HNP_FAILURE_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HNP_SUCCES_CLR` writer - 0 = no effect. 1 = clear the corresponding bit in the IntSt register."]
pub type HNP_SUCCES_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<INTCLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - 0 = no effect. 1 = clear the corresponding bit in the IntSt register."]
    #[inline(always)]
    #[must_use]
    pub fn tmr_clr(&mut self) -> TMR_CLR_W<INTCLR_SPEC, 0> {
        TMR_CLR_W::new(self)
    }
    #[doc = "Bit 1 - 0 = no effect. 1 = clear the corresponding bit in the IntSt register."]
    #[inline(always)]
    #[must_use]
    pub fn remove_pu_clr(&mut self) -> REMOVE_PU_CLR_W<INTCLR_SPEC, 1> {
        REMOVE_PU_CLR_W::new(self)
    }
    #[doc = "Bit 2 - 0 = no effect. 1 = clear the corresponding bit in the IntSt register."]
    #[inline(always)]
    #[must_use]
    pub fn hnp_failure_clr(&mut self) -> HNP_FAILURE_CLR_W<INTCLR_SPEC, 2> {
        HNP_FAILURE_CLR_W::new(self)
    }
    #[doc = "Bit 3 - 0 = no effect. 1 = clear the corresponding bit in the IntSt register."]
    #[inline(always)]
    #[must_use]
    pub fn hnp_succes_clr(&mut self) -> HNP_SUCCES_CLR_W<INTCLR_SPEC, 3> {
        HNP_SUCCES_CLR_W::new(self)
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
#[doc = "OTG Interrupt Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTCLR_SPEC;
impl crate::RegisterSpec for INTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intclr::W`](W) writer structure"]
impl crate::Writable for INTCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTCLR to value 0"]
impl crate::Resettable for INTCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
