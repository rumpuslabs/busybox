#[doc = "Register `CON_CLR` writer"]
pub type W = crate::W<CON_CLR_SPEC>;
#[doc = "Field `RUN0_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub type RUN0_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CENTER0_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub type CENTER0_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `POLA0_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub type POLA0_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTE0_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub type DTE0_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DISUP0_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub type DISUP0_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RUN1_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub type RUN1_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CENTER1_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub type CENTER1_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `POLA1_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub type POLA1_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTE1_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub type DTE1_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DISUP1_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub type DISUP1_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RUN2_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub type RUN2_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CENTER2_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub type CENTER2_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `POLA2_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub type POLA2_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTE2_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub type DTE2_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DISUP2_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub type DISUP2_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INVBDC_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub type INVBDC_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ACMOD_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub type ACMOD_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DCMODE_CLR` writer - Writing a one clears the corresponding bit in the CON register."]
pub type DCMODE_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CON_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn run0_clr(&mut self) -> RUN0_CLR_W<CON_CLR_SPEC, 0> {
        RUN0_CLR_W::new(self)
    }
    #[doc = "Bit 1 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn center0_clr(&mut self) -> CENTER0_CLR_W<CON_CLR_SPEC, 1> {
        CENTER0_CLR_W::new(self)
    }
    #[doc = "Bit 2 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn pola0_clr(&mut self) -> POLA0_CLR_W<CON_CLR_SPEC, 2> {
        POLA0_CLR_W::new(self)
    }
    #[doc = "Bit 3 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn dte0_clr(&mut self) -> DTE0_CLR_W<CON_CLR_SPEC, 3> {
        DTE0_CLR_W::new(self)
    }
    #[doc = "Bit 4 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn disup0_clr(&mut self) -> DISUP0_CLR_W<CON_CLR_SPEC, 4> {
        DISUP0_CLR_W::new(self)
    }
    #[doc = "Bit 8 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn run1_clr(&mut self) -> RUN1_CLR_W<CON_CLR_SPEC, 8> {
        RUN1_CLR_W::new(self)
    }
    #[doc = "Bit 9 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn center1_clr(&mut self) -> CENTER1_CLR_W<CON_CLR_SPEC, 9> {
        CENTER1_CLR_W::new(self)
    }
    #[doc = "Bit 10 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn pola1_clr(&mut self) -> POLA1_CLR_W<CON_CLR_SPEC, 10> {
        POLA1_CLR_W::new(self)
    }
    #[doc = "Bit 11 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn dte1_clr(&mut self) -> DTE1_CLR_W<CON_CLR_SPEC, 11> {
        DTE1_CLR_W::new(self)
    }
    #[doc = "Bit 12 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn disup1_clr(&mut self) -> DISUP1_CLR_W<CON_CLR_SPEC, 12> {
        DISUP1_CLR_W::new(self)
    }
    #[doc = "Bit 16 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn run2_clr(&mut self) -> RUN2_CLR_W<CON_CLR_SPEC, 16> {
        RUN2_CLR_W::new(self)
    }
    #[doc = "Bit 17 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn center2_clr(&mut self) -> CENTER2_CLR_W<CON_CLR_SPEC, 17> {
        CENTER2_CLR_W::new(self)
    }
    #[doc = "Bit 18 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn pola2_clr(&mut self) -> POLA2_CLR_W<CON_CLR_SPEC, 18> {
        POLA2_CLR_W::new(self)
    }
    #[doc = "Bit 19 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn dte2_clr(&mut self) -> DTE2_CLR_W<CON_CLR_SPEC, 19> {
        DTE2_CLR_W::new(self)
    }
    #[doc = "Bit 20 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn disup2_clr(&mut self) -> DISUP2_CLR_W<CON_CLR_SPEC, 20> {
        DISUP2_CLR_W::new(self)
    }
    #[doc = "Bit 29 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn invbdc_clr(&mut self) -> INVBDC_CLR_W<CON_CLR_SPEC, 29> {
        INVBDC_CLR_W::new(self)
    }
    #[doc = "Bit 30 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn acmod_clr(&mut self) -> ACMOD_CLR_W<CON_CLR_SPEC, 30> {
        ACMOD_CLR_W::new(self)
    }
    #[doc = "Bit 31 - Writing a one clears the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn dcmode_clr(&mut self) -> DCMODE_CLR_W<CON_CLR_SPEC, 31> {
        DCMODE_CLR_W::new(self)
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
#[doc = "PWM Control clear address\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`con_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CON_CLR_SPEC;
impl crate::RegisterSpec for CON_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`con_clr::W`](W) writer structure"]
impl crate::Writable for CON_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CON_CLR to value 0"]
impl crate::Resettable for CON_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
