#[doc = "Register `INTTCCLEAR` writer"]
pub type W = crate::W<INTTCCLEAR_SPEC>;
#[doc = "Field `INTTCCLEAR0` writer - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
pub type INTTCCLEAR0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTTCCLEAR1` writer - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
pub type INTTCCLEAR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTTCCLEAR2` writer - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
pub type INTTCCLEAR2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTTCCLEAR3` writer - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
pub type INTTCCLEAR3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTTCCLEAR4` writer - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
pub type INTTCCLEAR4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTTCCLEAR5` writer - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
pub type INTTCCLEAR5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTTCCLEAR6` writer - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
pub type INTTCCLEAR6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTTCCLEAR7` writer - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
pub type INTTCCLEAR7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<INTTCCLEAR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn inttcclear0(&mut self) -> INTTCCLEAR0_W<INTTCCLEAR_SPEC, 0> {
        INTTCCLEAR0_W::new(self)
    }
    #[doc = "Bit 1 - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn inttcclear1(&mut self) -> INTTCCLEAR1_W<INTTCCLEAR_SPEC, 1> {
        INTTCCLEAR1_W::new(self)
    }
    #[doc = "Bit 2 - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn inttcclear2(&mut self) -> INTTCCLEAR2_W<INTTCCLEAR_SPEC, 2> {
        INTTCCLEAR2_W::new(self)
    }
    #[doc = "Bit 3 - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn inttcclear3(&mut self) -> INTTCCLEAR3_W<INTTCCLEAR_SPEC, 3> {
        INTTCCLEAR3_W::new(self)
    }
    #[doc = "Bit 4 - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn inttcclear4(&mut self) -> INTTCCLEAR4_W<INTTCCLEAR_SPEC, 4> {
        INTTCCLEAR4_W::new(self)
    }
    #[doc = "Bit 5 - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn inttcclear5(&mut self) -> INTTCCLEAR5_W<INTTCCLEAR_SPEC, 5> {
        INTTCCLEAR5_W::new(self)
    }
    #[doc = "Bit 6 - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn inttcclear6(&mut self) -> INTTCCLEAR6_W<INTTCCLEAR_SPEC, 6> {
        INTTCCLEAR6_W::new(self)
    }
    #[doc = "Bit 7 - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn inttcclear7(&mut self) -> INTTCCLEAR7_W<INTTCCLEAR_SPEC, 7> {
        INTTCCLEAR7_W::new(self)
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
#[doc = "DMA Interrupt Terminal Count Request Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inttcclear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTTCCLEAR_SPEC;
impl crate::RegisterSpec for INTTCCLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`inttcclear::W`](W) writer structure"]
impl crate::Writable for INTTCCLEAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTTCCLEAR to value 0"]
impl crate::Resettable for INTTCCLEAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
