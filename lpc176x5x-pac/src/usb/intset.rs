#[doc = "Register `INTSET` writer"]
pub type W = crate::W<INTSET_SPEC>;
#[doc = "Field `TMR_SET` writer - 0 = no effect. 1 = set the corresponding bit in the IntSt register."]
pub type TMR_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `REMOVE_PU_SET` writer - 0 = no effect. 1 = set the corresponding bit in the IntSt register."]
pub type REMOVE_PU_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HNP_FAILURE_SET` writer - 0 = no effect. 1 = set the corresponding bit in the IntSt register."]
pub type HNP_FAILURE_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HNP_SUCCES_SET` writer - 0 = no effect. 1 = set the corresponding bit in the IntSt register."]
pub type HNP_SUCCES_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<INTSET_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - 0 = no effect. 1 = set the corresponding bit in the IntSt register."]
    #[inline(always)]
    #[must_use]
    pub fn tmr_set(&mut self) -> TMR_SET_W<INTSET_SPEC, 0> {
        TMR_SET_W::new(self)
    }
    #[doc = "Bit 1 - 0 = no effect. 1 = set the corresponding bit in the IntSt register."]
    #[inline(always)]
    #[must_use]
    pub fn remove_pu_set(&mut self) -> REMOVE_PU_SET_W<INTSET_SPEC, 1> {
        REMOVE_PU_SET_W::new(self)
    }
    #[doc = "Bit 2 - 0 = no effect. 1 = set the corresponding bit in the IntSt register."]
    #[inline(always)]
    #[must_use]
    pub fn hnp_failure_set(&mut self) -> HNP_FAILURE_SET_W<INTSET_SPEC, 2> {
        HNP_FAILURE_SET_W::new(self)
    }
    #[doc = "Bit 3 - 0 = no effect. 1 = set the corresponding bit in the IntSt register."]
    #[inline(always)]
    #[must_use]
    pub fn hnp_succes_set(&mut self) -> HNP_SUCCES_SET_W<INTSET_SPEC, 3> {
        HNP_SUCCES_SET_W::new(self)
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
#[doc = "OTG Interrupt Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTSET_SPEC;
impl crate::RegisterSpec for INTSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intset::W`](W) writer structure"]
impl crate::Writable for INTSET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTSET to value 0"]
impl crate::Resettable for INTSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
