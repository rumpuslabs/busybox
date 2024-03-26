#[doc = "Register `CON_SET` writer"]
pub type W = crate::W<CON_SET_SPEC>;
#[doc = "Field `RUN0_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub type RUN0_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CENTER0_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub type CENTER0_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `POLA0_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub type POLA0_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTE0_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub type DTE0_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DISUP0_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub type DISUP0_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RUN1_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub type RUN1_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CENTER1_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub type CENTER1_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `POLA1_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub type POLA1_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTE1_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub type DTE1_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DISUP1_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub type DISUP1_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RUN2_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub type RUN2_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CENTER2_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub type CENTER2_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `POLA2_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub type POLA2_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTE2_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub type DTE2_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DISUP2_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub type DISUP2_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INVBDC_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub type INVBDC_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ACMODE_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub type ACMODE_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DCMODE_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub type DCMODE_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CON_SET_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn run0_set(&mut self) -> RUN0_SET_W<CON_SET_SPEC, 0> {
        RUN0_SET_W::new(self)
    }
    #[doc = "Bit 1 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn center0_set(&mut self) -> CENTER0_SET_W<CON_SET_SPEC, 1> {
        CENTER0_SET_W::new(self)
    }
    #[doc = "Bit 2 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn pola0_set(&mut self) -> POLA0_SET_W<CON_SET_SPEC, 2> {
        POLA0_SET_W::new(self)
    }
    #[doc = "Bit 3 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn dte0_set(&mut self) -> DTE0_SET_W<CON_SET_SPEC, 3> {
        DTE0_SET_W::new(self)
    }
    #[doc = "Bit 4 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn disup0_set(&mut self) -> DISUP0_SET_W<CON_SET_SPEC, 4> {
        DISUP0_SET_W::new(self)
    }
    #[doc = "Bit 8 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn run1_set(&mut self) -> RUN1_SET_W<CON_SET_SPEC, 8> {
        RUN1_SET_W::new(self)
    }
    #[doc = "Bit 9 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn center1_set(&mut self) -> CENTER1_SET_W<CON_SET_SPEC, 9> {
        CENTER1_SET_W::new(self)
    }
    #[doc = "Bit 10 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn pola1_set(&mut self) -> POLA1_SET_W<CON_SET_SPEC, 10> {
        POLA1_SET_W::new(self)
    }
    #[doc = "Bit 11 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn dte1_set(&mut self) -> DTE1_SET_W<CON_SET_SPEC, 11> {
        DTE1_SET_W::new(self)
    }
    #[doc = "Bit 12 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn disup1_set(&mut self) -> DISUP1_SET_W<CON_SET_SPEC, 12> {
        DISUP1_SET_W::new(self)
    }
    #[doc = "Bit 16 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn run2_set(&mut self) -> RUN2_SET_W<CON_SET_SPEC, 16> {
        RUN2_SET_W::new(self)
    }
    #[doc = "Bit 17 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn center2_set(&mut self) -> CENTER2_SET_W<CON_SET_SPEC, 17> {
        CENTER2_SET_W::new(self)
    }
    #[doc = "Bit 18 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn pola2_set(&mut self) -> POLA2_SET_W<CON_SET_SPEC, 18> {
        POLA2_SET_W::new(self)
    }
    #[doc = "Bit 19 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn dte2_set(&mut self) -> DTE2_SET_W<CON_SET_SPEC, 19> {
        DTE2_SET_W::new(self)
    }
    #[doc = "Bit 20 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn disup2_set(&mut self) -> DISUP2_SET_W<CON_SET_SPEC, 20> {
        DISUP2_SET_W::new(self)
    }
    #[doc = "Bit 29 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn invbdc_set(&mut self) -> INVBDC_SET_W<CON_SET_SPEC, 29> {
        INVBDC_SET_W::new(self)
    }
    #[doc = "Bit 30 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn acmode_set(&mut self) -> ACMODE_SET_W<CON_SET_SPEC, 30> {
        ACMODE_SET_W::new(self)
    }
    #[doc = "Bit 31 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    #[must_use]
    pub fn dcmode_set(&mut self) -> DCMODE_SET_W<CON_SET_SPEC, 31> {
        DCMODE_SET_W::new(self)
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
#[doc = "PWM Control set address\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`con_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CON_SET_SPEC;
impl crate::RegisterSpec for CON_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`con_set::W`](W) writer structure"]
impl crate::Writable for CON_SET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CON_SET to value 0"]
impl crate::Resettable for CON_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
