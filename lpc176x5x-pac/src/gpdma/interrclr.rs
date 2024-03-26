#[doc = "Register `INTERRCLR` writer"]
pub type W = crate::W<INTERRCLR_SPEC>;
#[doc = "Field `INTERRCLR0` writer - Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel error interrupt."]
pub type INTERRCLR0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTERRCLR1` writer - Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel error interrupt."]
pub type INTERRCLR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTERRCLR2` writer - Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel error interrupt."]
pub type INTERRCLR2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTERRCLR3` writer - Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel error interrupt."]
pub type INTERRCLR3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTERRCLR4` writer - Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel error interrupt."]
pub type INTERRCLR4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTERRCLR5` writer - Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel error interrupt."]
pub type INTERRCLR5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTERRCLR6` writer - Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel error interrupt."]
pub type INTERRCLR6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTERRCLR7` writer - Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel error interrupt."]
pub type INTERRCLR7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<INTERRCLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel error interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn interrclr0(&mut self) -> INTERRCLR0_W<INTERRCLR_SPEC, 0> {
        INTERRCLR0_W::new(self)
    }
    #[doc = "Bit 1 - Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel error interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn interrclr1(&mut self) -> INTERRCLR1_W<INTERRCLR_SPEC, 1> {
        INTERRCLR1_W::new(self)
    }
    #[doc = "Bit 2 - Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel error interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn interrclr2(&mut self) -> INTERRCLR2_W<INTERRCLR_SPEC, 2> {
        INTERRCLR2_W::new(self)
    }
    #[doc = "Bit 3 - Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel error interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn interrclr3(&mut self) -> INTERRCLR3_W<INTERRCLR_SPEC, 3> {
        INTERRCLR3_W::new(self)
    }
    #[doc = "Bit 4 - Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel error interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn interrclr4(&mut self) -> INTERRCLR4_W<INTERRCLR_SPEC, 4> {
        INTERRCLR4_W::new(self)
    }
    #[doc = "Bit 5 - Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel error interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn interrclr5(&mut self) -> INTERRCLR5_W<INTERRCLR_SPEC, 5> {
        INTERRCLR5_W::new(self)
    }
    #[doc = "Bit 6 - Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel error interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn interrclr6(&mut self) -> INTERRCLR6_W<INTERRCLR_SPEC, 6> {
        INTERRCLR6_W::new(self)
    }
    #[doc = "Bit 7 - Writing a 1 clears the error interrupt request (IntErrStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel error interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn interrclr7(&mut self) -> INTERRCLR7_W<INTERRCLR_SPEC, 7> {
        INTERRCLR7_W::new(self)
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
#[doc = "DMA Interrupt Error Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERRCLR_SPEC;
impl crate::RegisterSpec for INTERRCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`interrclr::W`](W) writer structure"]
impl crate::Writable for INTERRCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTERRCLR to value 0"]
impl crate::Resettable for INTERRCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
