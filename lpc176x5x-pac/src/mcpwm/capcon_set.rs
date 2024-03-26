#[doc = "Register `CAPCON_SET` writer"]
pub type W = crate::W<CAPCON_SET_SPEC>;
#[doc = "Field `CAP0MCI0_RE_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type CAP0MCI0_RE_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAP0MCI0_FE_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type CAP0MCI0_FE_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAP0MCI1_RE_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type CAP0MCI1_RE_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAP0MCI1_FE_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type CAP0MCI1_FE_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAP0MCI2_RE_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type CAP0MCI2_RE_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAP0MCI2_FE_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type CAP0MCI2_FE_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAP1MCI0_RE_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type CAP1MCI0_RE_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAP1MCI0_FE_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type CAP1MCI0_FE_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAP1MCI1_RE_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type CAP1MCI1_RE_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAP1MCI1_FE_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type CAP1MCI1_FE_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAP1MCI2_RE_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type CAP1MCI2_RE_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAP1MCI2_FE_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type CAP1MCI2_FE_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAP2MCI0_RE_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type CAP2MCI0_RE_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAP2MCI0_FE_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type CAP2MCI0_FE_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAP2MCI1_RE_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type CAP2MCI1_RE_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAP2MCI1_FE_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type CAP2MCI1_FE_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAP2MCI2_RE_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type CAP2MCI2_RE_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAP2MCI2_FE_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type CAP2MCI2_FE_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RT0_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type RT0_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RT1_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type RT1_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RT2_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type RT2_SET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CAPCON_SET_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    #[must_use]
    pub fn cap0mci0_re_set(&mut self) -> CAP0MCI0_RE_SET_W<CAPCON_SET_SPEC, 0> {
        CAP0MCI0_RE_SET_W::new(self)
    }
    #[doc = "Bit 1 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    #[must_use]
    pub fn cap0mci0_fe_set(&mut self) -> CAP0MCI0_FE_SET_W<CAPCON_SET_SPEC, 1> {
        CAP0MCI0_FE_SET_W::new(self)
    }
    #[doc = "Bit 2 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    #[must_use]
    pub fn cap0mci1_re_set(&mut self) -> CAP0MCI1_RE_SET_W<CAPCON_SET_SPEC, 2> {
        CAP0MCI1_RE_SET_W::new(self)
    }
    #[doc = "Bit 3 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    #[must_use]
    pub fn cap0mci1_fe_set(&mut self) -> CAP0MCI1_FE_SET_W<CAPCON_SET_SPEC, 3> {
        CAP0MCI1_FE_SET_W::new(self)
    }
    #[doc = "Bit 4 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    #[must_use]
    pub fn cap0mci2_re_set(&mut self) -> CAP0MCI2_RE_SET_W<CAPCON_SET_SPEC, 4> {
        CAP0MCI2_RE_SET_W::new(self)
    }
    #[doc = "Bit 5 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    #[must_use]
    pub fn cap0mci2_fe_set(&mut self) -> CAP0MCI2_FE_SET_W<CAPCON_SET_SPEC, 5> {
        CAP0MCI2_FE_SET_W::new(self)
    }
    #[doc = "Bit 6 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    #[must_use]
    pub fn cap1mci0_re_set(&mut self) -> CAP1MCI0_RE_SET_W<CAPCON_SET_SPEC, 6> {
        CAP1MCI0_RE_SET_W::new(self)
    }
    #[doc = "Bit 7 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    #[must_use]
    pub fn cap1mci0_fe_set(&mut self) -> CAP1MCI0_FE_SET_W<CAPCON_SET_SPEC, 7> {
        CAP1MCI0_FE_SET_W::new(self)
    }
    #[doc = "Bit 8 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    #[must_use]
    pub fn cap1mci1_re_set(&mut self) -> CAP1MCI1_RE_SET_W<CAPCON_SET_SPEC, 8> {
        CAP1MCI1_RE_SET_W::new(self)
    }
    #[doc = "Bit 9 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    #[must_use]
    pub fn cap1mci1_fe_set(&mut self) -> CAP1MCI1_FE_SET_W<CAPCON_SET_SPEC, 9> {
        CAP1MCI1_FE_SET_W::new(self)
    }
    #[doc = "Bit 10 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    #[must_use]
    pub fn cap1mci2_re_set(&mut self) -> CAP1MCI2_RE_SET_W<CAPCON_SET_SPEC, 10> {
        CAP1MCI2_RE_SET_W::new(self)
    }
    #[doc = "Bit 11 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    #[must_use]
    pub fn cap1mci2_fe_set(&mut self) -> CAP1MCI2_FE_SET_W<CAPCON_SET_SPEC, 11> {
        CAP1MCI2_FE_SET_W::new(self)
    }
    #[doc = "Bit 12 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    #[must_use]
    pub fn cap2mci0_re_set(&mut self) -> CAP2MCI0_RE_SET_W<CAPCON_SET_SPEC, 12> {
        CAP2MCI0_RE_SET_W::new(self)
    }
    #[doc = "Bit 13 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    #[must_use]
    pub fn cap2mci0_fe_set(&mut self) -> CAP2MCI0_FE_SET_W<CAPCON_SET_SPEC, 13> {
        CAP2MCI0_FE_SET_W::new(self)
    }
    #[doc = "Bit 14 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    #[must_use]
    pub fn cap2mci1_re_set(&mut self) -> CAP2MCI1_RE_SET_W<CAPCON_SET_SPEC, 14> {
        CAP2MCI1_RE_SET_W::new(self)
    }
    #[doc = "Bit 15 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    #[must_use]
    pub fn cap2mci1_fe_set(&mut self) -> CAP2MCI1_FE_SET_W<CAPCON_SET_SPEC, 15> {
        CAP2MCI1_FE_SET_W::new(self)
    }
    #[doc = "Bit 16 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    #[must_use]
    pub fn cap2mci2_re_set(&mut self) -> CAP2MCI2_RE_SET_W<CAPCON_SET_SPEC, 16> {
        CAP2MCI2_RE_SET_W::new(self)
    }
    #[doc = "Bit 17 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    #[must_use]
    pub fn cap2mci2_fe_set(&mut self) -> CAP2MCI2_FE_SET_W<CAPCON_SET_SPEC, 17> {
        CAP2MCI2_FE_SET_W::new(self)
    }
    #[doc = "Bit 18 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    #[must_use]
    pub fn rt0_set(&mut self) -> RT0_SET_W<CAPCON_SET_SPEC, 18> {
        RT0_SET_W::new(self)
    }
    #[doc = "Bit 19 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    #[must_use]
    pub fn rt1_set(&mut self) -> RT1_SET_W<CAPCON_SET_SPEC, 19> {
        RT1_SET_W::new(self)
    }
    #[doc = "Bit 20 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    #[must_use]
    pub fn rt2_set(&mut self) -> RT2_SET_W<CAPCON_SET_SPEC, 20> {
        RT2_SET_W::new(self)
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
#[doc = "Capture Control set address\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`capcon_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAPCON_SET_SPEC;
impl crate::RegisterSpec for CAPCON_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`capcon_set::W`](W) writer structure"]
impl crate::Writable for CAPCON_SET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAPCON_SET to value 0"]
impl crate::Resettable for CAPCON_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
