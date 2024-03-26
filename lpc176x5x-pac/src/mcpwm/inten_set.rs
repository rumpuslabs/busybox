#[doc = "Register `INTEN_SET` writer"]
pub type W = crate::W<INTEN_SET_SPEC>;
#[doc = "Field `ILIM0_SET` writer - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
pub type ILIM0_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IMAT0_SET` writer - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
pub type IMAT0_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ICAP0_SET` writer - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
pub type ICAP0_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ILIM1_SET` writer - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
pub type ILIM1_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IMAT1_SET` writer - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
pub type IMAT1_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ICAP1_SET` writer - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
pub type ICAP1_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ILIM2_SET` writer - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
pub type ILIM2_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IMAT2_SET` writer - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
pub type IMAT2_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ICAP2_SET` writer - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
pub type ICAP2_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ABORT_SET` writer - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
pub type ABORT_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<INTEN_SET_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ilim0_set(&mut self) -> ILIM0_SET_W<INTEN_SET_SPEC, 0> {
        ILIM0_SET_W::new(self)
    }
    #[doc = "Bit 1 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn imat0_set(&mut self) -> IMAT0_SET_W<INTEN_SET_SPEC, 1> {
        IMAT0_SET_W::new(self)
    }
    #[doc = "Bit 2 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn icap0_set(&mut self) -> ICAP0_SET_W<INTEN_SET_SPEC, 2> {
        ICAP0_SET_W::new(self)
    }
    #[doc = "Bit 4 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ilim1_set(&mut self) -> ILIM1_SET_W<INTEN_SET_SPEC, 4> {
        ILIM1_SET_W::new(self)
    }
    #[doc = "Bit 5 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn imat1_set(&mut self) -> IMAT1_SET_W<INTEN_SET_SPEC, 5> {
        IMAT1_SET_W::new(self)
    }
    #[doc = "Bit 6 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn icap1_set(&mut self) -> ICAP1_SET_W<INTEN_SET_SPEC, 6> {
        ICAP1_SET_W::new(self)
    }
    #[doc = "Bit 9 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ilim2_set(&mut self) -> ILIM2_SET_W<INTEN_SET_SPEC, 9> {
        ILIM2_SET_W::new(self)
    }
    #[doc = "Bit 10 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn imat2_set(&mut self) -> IMAT2_SET_W<INTEN_SET_SPEC, 10> {
        IMAT2_SET_W::new(self)
    }
    #[doc = "Bit 11 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn icap2_set(&mut self) -> ICAP2_SET_W<INTEN_SET_SPEC, 11> {
        ICAP2_SET_W::new(self)
    }
    #[doc = "Bit 15 - Writing a one sets the corresponding bit in INTEN, thus enabling the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn abort_set(&mut self) -> ABORT_SET_W<INTEN_SET_SPEC, 15> {
        ABORT_SET_W::new(self)
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
#[doc = "Interrupt Enable set address\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTEN_SET_SPEC;
impl crate::RegisterSpec for INTEN_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`inten_set::W`](W) writer structure"]
impl crate::Writable for INTEN_SET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTEN_SET to value 0"]
impl crate::Resettable for INTEN_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
