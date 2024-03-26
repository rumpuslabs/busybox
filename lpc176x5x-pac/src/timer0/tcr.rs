#[doc = "Register `TCR` reader"]
pub type R = crate::R<TCR_SPEC>;
#[doc = "Register `TCR` writer"]
pub type W = crate::W<TCR_SPEC>;
#[doc = "Field `CEN` reader - When one, the Timer Counter and Prescale Counter are enabled for counting. When zero, the counters are disabled."]
pub type CEN_R = crate::BitReader;
#[doc = "Field `CEN` writer - When one, the Timer Counter and Prescale Counter are enabled for counting. When zero, the counters are disabled."]
pub type CEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRST` reader - When one, the Timer Counter and the Prescale Counter are synchronously reset on the next positive edge of PCLK. The counters remain reset until TCR\\[1\\]
is returned to zero."]
pub type CRST_R = crate::BitReader;
#[doc = "Field `CRST` writer - When one, the Timer Counter and the Prescale Counter are synchronously reset on the next positive edge of PCLK. The counters remain reset until TCR\\[1\\]
is returned to zero."]
pub type CRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - When one, the Timer Counter and Prescale Counter are enabled for counting. When zero, the counters are disabled."]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When one, the Timer Counter and the Prescale Counter are synchronously reset on the next positive edge of PCLK. The counters remain reset until TCR\\[1\\]
is returned to zero."]
    #[inline(always)]
    pub fn crst(&self) -> CRST_R {
        CRST_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCR")
            .field("cen", &format_args!("{}", self.cen().bit()))
            .field("crst", &format_args!("{}", self.crst().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<TCR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - When one, the Timer Counter and Prescale Counter are enabled for counting. When zero, the counters are disabled."]
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CEN_W<TCR_SPEC, 0> {
        CEN_W::new(self)
    }
    #[doc = "Bit 1 - When one, the Timer Counter and the Prescale Counter are synchronously reset on the next positive edge of PCLK. The counters remain reset until TCR\\[1\\]
is returned to zero."]
    #[inline(always)]
    #[must_use]
    pub fn crst(&mut self) -> CRST_W<TCR_SPEC, 1> {
        CRST_W::new(self)
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
#[doc = "Timer Control Register. The TCR is used to control the Timer Counter functions. The Timer Counter can be disabled or reset through the TCR.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCR_SPEC;
impl crate::RegisterSpec for TCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcr::R`](R) reader structure"]
impl crate::Readable for TCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tcr::W`](W) writer structure"]
impl crate::Writable for TCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCR to value 0"]
impl crate::Resettable for TCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
