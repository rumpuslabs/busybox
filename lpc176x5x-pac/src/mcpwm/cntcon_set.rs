#[doc = "Register `CNTCON_SET` writer"]
pub type W = crate::W<CNTCON_SET_SPEC>;
#[doc = "Field `TC0MCI0_RE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type TC0MCI0_RE_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TC0MCI0_FE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type TC0MCI0_FE_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TC0MCI1_RE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type TC0MCI1_RE_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TC0MCI1_FE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type TC0MCI1_FE_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TC0MCI2_RE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type TC0MCI2_RE_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TC0MCI2_FE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type TC0MCI2_FE_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TC1MCI0_RE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type TC1MCI0_RE_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TC1MCI0_FE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type TC1MCI0_FE_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TC1MCI1_RE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type TC1MCI1_RE_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TC1MCI1_FE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type TC1MCI1_FE_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TC1MCI2_RE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type TC1MCI2_RE_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TC1MCI2_FE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type TC1MCI2_FE_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TC2MCI0_RE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type TC2MCI0_RE_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TC2MCI0_FE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type TC2MCI0_FE_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TC2MCI1_RE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type TC2MCI1_RE_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TC2MCI1_FE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type TC2MCI1_FE_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TC2MCI2_RE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type TC2MCI2_RE_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TC2MCI2_FE_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type TC2MCI2_FE_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CNTR0_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type CNTR0_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CNTR1_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type CNTR1_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CNTR2_SET` writer - Writing a one sets the corresponding bit in the CNTCON register."]
pub type CNTR2_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CNTCON_SET_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn tc0mci0_re_set(&mut self) -> TC0MCI0_RE_SET_W<CNTCON_SET_SPEC, 0> {
        TC0MCI0_RE_SET_W::new(self)
    }
    #[doc = "Bit 1 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn tc0mci0_fe_set(&mut self) -> TC0MCI0_FE_SET_W<CNTCON_SET_SPEC, 1> {
        TC0MCI0_FE_SET_W::new(self)
    }
    #[doc = "Bit 2 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn tc0mci1_re_set(&mut self) -> TC0MCI1_RE_SET_W<CNTCON_SET_SPEC, 2> {
        TC0MCI1_RE_SET_W::new(self)
    }
    #[doc = "Bit 3 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn tc0mci1_fe_set(&mut self) -> TC0MCI1_FE_SET_W<CNTCON_SET_SPEC, 3> {
        TC0MCI1_FE_SET_W::new(self)
    }
    #[doc = "Bit 4 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn tc0mci2_re_set(&mut self) -> TC0MCI2_RE_SET_W<CNTCON_SET_SPEC, 4> {
        TC0MCI2_RE_SET_W::new(self)
    }
    #[doc = "Bit 5 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn tc0mci2_fe_set(&mut self) -> TC0MCI2_FE_SET_W<CNTCON_SET_SPEC, 5> {
        TC0MCI2_FE_SET_W::new(self)
    }
    #[doc = "Bit 6 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn tc1mci0_re_set(&mut self) -> TC1MCI0_RE_SET_W<CNTCON_SET_SPEC, 6> {
        TC1MCI0_RE_SET_W::new(self)
    }
    #[doc = "Bit 7 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn tc1mci0_fe_set(&mut self) -> TC1MCI0_FE_SET_W<CNTCON_SET_SPEC, 7> {
        TC1MCI0_FE_SET_W::new(self)
    }
    #[doc = "Bit 8 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn tc1mci1_re_set(&mut self) -> TC1MCI1_RE_SET_W<CNTCON_SET_SPEC, 8> {
        TC1MCI1_RE_SET_W::new(self)
    }
    #[doc = "Bit 9 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn tc1mci1_fe_set(&mut self) -> TC1MCI1_FE_SET_W<CNTCON_SET_SPEC, 9> {
        TC1MCI1_FE_SET_W::new(self)
    }
    #[doc = "Bit 10 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn tc1mci2_re_set(&mut self) -> TC1MCI2_RE_SET_W<CNTCON_SET_SPEC, 10> {
        TC1MCI2_RE_SET_W::new(self)
    }
    #[doc = "Bit 11 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn tc1mci2_fe_set(&mut self) -> TC1MCI2_FE_SET_W<CNTCON_SET_SPEC, 11> {
        TC1MCI2_FE_SET_W::new(self)
    }
    #[doc = "Bit 12 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn tc2mci0_re_set(&mut self) -> TC2MCI0_RE_SET_W<CNTCON_SET_SPEC, 12> {
        TC2MCI0_RE_SET_W::new(self)
    }
    #[doc = "Bit 13 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn tc2mci0_fe_set(&mut self) -> TC2MCI0_FE_SET_W<CNTCON_SET_SPEC, 13> {
        TC2MCI0_FE_SET_W::new(self)
    }
    #[doc = "Bit 14 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn tc2mci1_re_set(&mut self) -> TC2MCI1_RE_SET_W<CNTCON_SET_SPEC, 14> {
        TC2MCI1_RE_SET_W::new(self)
    }
    #[doc = "Bit 15 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn tc2mci1_fe_set(&mut self) -> TC2MCI1_FE_SET_W<CNTCON_SET_SPEC, 15> {
        TC2MCI1_FE_SET_W::new(self)
    }
    #[doc = "Bit 16 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn tc2mci2_re_set(&mut self) -> TC2MCI2_RE_SET_W<CNTCON_SET_SPEC, 16> {
        TC2MCI2_RE_SET_W::new(self)
    }
    #[doc = "Bit 17 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn tc2mci2_fe_set(&mut self) -> TC2MCI2_FE_SET_W<CNTCON_SET_SPEC, 17> {
        TC2MCI2_FE_SET_W::new(self)
    }
    #[doc = "Bit 29 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn cntr0_set(&mut self) -> CNTR0_SET_W<CNTCON_SET_SPEC, 29> {
        CNTR0_SET_W::new(self)
    }
    #[doc = "Bit 30 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn cntr1_set(&mut self) -> CNTR1_SET_W<CNTCON_SET_SPEC, 30> {
        CNTR1_SET_W::new(self)
    }
    #[doc = "Bit 31 - Writing a one sets the corresponding bit in the CNTCON register."]
    #[inline(always)]
    #[must_use]
    pub fn cntr2_set(&mut self) -> CNTR2_SET_W<CNTCON_SET_SPEC, 31> {
        CNTR2_SET_W::new(self)
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
#[doc = "Count Control set address\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntcon_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNTCON_SET_SPEC;
impl crate::RegisterSpec for CNTCON_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cntcon_set::W`](W) writer structure"]
impl crate::Writable for CNTCON_SET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNTCON_SET to value 0"]
impl crate::Resettable for CNTCON_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
