#[doc = "Register `INTEN_CLR` writer"]
pub type W = crate::W<INTEN_CLR_SPEC>;
#[doc = "Field `ILIM0_CLR` writer - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
pub type ILIM0_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IMAT0_CLR` writer - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
pub type IMAT0_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ICAP0_CLR` writer - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
pub type ICAP0_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ILIM1_CLR` writer - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
pub type ILIM1_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IMAT1_CLR` writer - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
pub type IMAT1_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ICAP1_CLR` writer - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
pub type ICAP1_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ILIM2_CLR` writer - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
pub type ILIM2_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IMAT2_CLR` writer - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
pub type IMAT2_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ICAP2_CLR` writer - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
pub type ICAP2_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ABORT_CLR` writer - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
pub type ABORT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<INTEN_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ilim0_clr(&mut self) -> ILIM0_CLR_W<INTEN_CLR_SPEC, 0> {
        ILIM0_CLR_W::new(self)
    }
    #[doc = "Bit 1 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn imat0_clr(&mut self) -> IMAT0_CLR_W<INTEN_CLR_SPEC, 1> {
        IMAT0_CLR_W::new(self)
    }
    #[doc = "Bit 2 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn icap0_clr(&mut self) -> ICAP0_CLR_W<INTEN_CLR_SPEC, 2> {
        ICAP0_CLR_W::new(self)
    }
    #[doc = "Bit 4 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ilim1_clr(&mut self) -> ILIM1_CLR_W<INTEN_CLR_SPEC, 4> {
        ILIM1_CLR_W::new(self)
    }
    #[doc = "Bit 5 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn imat1_clr(&mut self) -> IMAT1_CLR_W<INTEN_CLR_SPEC, 5> {
        IMAT1_CLR_W::new(self)
    }
    #[doc = "Bit 6 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn icap1_clr(&mut self) -> ICAP1_CLR_W<INTEN_CLR_SPEC, 6> {
        ICAP1_CLR_W::new(self)
    }
    #[doc = "Bit 8 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ilim2_clr(&mut self) -> ILIM2_CLR_W<INTEN_CLR_SPEC, 8> {
        ILIM2_CLR_W::new(self)
    }
    #[doc = "Bit 9 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn imat2_clr(&mut self) -> IMAT2_CLR_W<INTEN_CLR_SPEC, 9> {
        IMAT2_CLR_W::new(self)
    }
    #[doc = "Bit 10 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn icap2_clr(&mut self) -> ICAP2_CLR_W<INTEN_CLR_SPEC, 10> {
        ICAP2_CLR_W::new(self)
    }
    #[doc = "Bit 15 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn abort_clr(&mut self) -> ABORT_CLR_W<INTEN_CLR_SPEC, 15> {
        ABORT_CLR_W::new(self)
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
#[doc = "Interrupt Enable clear address\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTEN_CLR_SPEC;
impl crate::RegisterSpec for INTEN_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`inten_clr::W`](W) writer structure"]
impl crate::Writable for INTEN_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTEN_CLR to value 0"]
impl crate::Resettable for INTEN_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
