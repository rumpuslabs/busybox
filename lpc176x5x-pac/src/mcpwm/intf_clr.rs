#[doc = "Register `INTF_CLR` writer"]
pub type W = crate::W<INTF_CLR_SPEC>;
#[doc = "Field `ILIM0_F_CLR` writer - Writing a one clears the corresponding bit in the INTF register, thus clearing the corresponding interrupt request."]
pub type ILIM0_F_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IMAT0_F_CLR` writer - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
pub type IMAT0_F_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ICAP0_F_CLR` writer - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
pub type ICAP0_F_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ILIM1_F_CLR` writer - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
pub type ILIM1_F_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IMAT1_F_CLR` writer - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
pub type IMAT1_F_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ICAP1_F_CLR` writer - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
pub type ICAP1_F_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ILIM2_F_CLR` writer - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
pub type ILIM2_F_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IMAT2_F_CLR` writer - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
pub type IMAT2_F_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ICAP2_F_CLR` writer - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
pub type ICAP2_F_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ABORT_F_CLR` writer - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
pub type ABORT_F_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<INTF_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Writing a one clears the corresponding bit in the INTF register, thus clearing the corresponding interrupt request."]
    #[inline(always)]
    #[must_use]
    pub fn ilim0_f_clr(&mut self) -> ILIM0_F_CLR_W<INTF_CLR_SPEC, 0> {
        ILIM0_F_CLR_W::new(self)
    }
    #[doc = "Bit 1 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn imat0_f_clr(&mut self) -> IMAT0_F_CLR_W<INTF_CLR_SPEC, 1> {
        IMAT0_F_CLR_W::new(self)
    }
    #[doc = "Bit 2 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn icap0_f_clr(&mut self) -> ICAP0_F_CLR_W<INTF_CLR_SPEC, 2> {
        ICAP0_F_CLR_W::new(self)
    }
    #[doc = "Bit 4 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ilim1_f_clr(&mut self) -> ILIM1_F_CLR_W<INTF_CLR_SPEC, 4> {
        ILIM1_F_CLR_W::new(self)
    }
    #[doc = "Bit 5 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn imat1_f_clr(&mut self) -> IMAT1_F_CLR_W<INTF_CLR_SPEC, 5> {
        IMAT1_F_CLR_W::new(self)
    }
    #[doc = "Bit 6 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn icap1_f_clr(&mut self) -> ICAP1_F_CLR_W<INTF_CLR_SPEC, 6> {
        ICAP1_F_CLR_W::new(self)
    }
    #[doc = "Bit 8 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ilim2_f_clr(&mut self) -> ILIM2_F_CLR_W<INTF_CLR_SPEC, 8> {
        ILIM2_F_CLR_W::new(self)
    }
    #[doc = "Bit 9 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn imat2_f_clr(&mut self) -> IMAT2_F_CLR_W<INTF_CLR_SPEC, 9> {
        IMAT2_F_CLR_W::new(self)
    }
    #[doc = "Bit 10 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn icap2_f_clr(&mut self) -> ICAP2_F_CLR_W<INTF_CLR_SPEC, 10> {
        ICAP2_F_CLR_W::new(self)
    }
    #[doc = "Bit 15 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn abort_f_clr(&mut self) -> ABORT_F_CLR_W<INTF_CLR_SPEC, 15> {
        ABORT_F_CLR_W::new(self)
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
#[doc = "Interrupt flags clear address\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intf_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTF_CLR_SPEC;
impl crate::RegisterSpec for INTF_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intf_clr::W`](W) writer structure"]
impl crate::Writable for INTF_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTF_CLR to value 0"]
impl crate::Resettable for INTF_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
