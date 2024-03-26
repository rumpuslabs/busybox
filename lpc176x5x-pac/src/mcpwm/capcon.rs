#[doc = "Register `CAPCON` reader"]
pub type R = crate::R<CAPCON_SPEC>;
#[doc = "Field `CAP0MCI0_RE` reader - A 1 in this bit enables a channel 0 capture event on a rising edge on MCI0."]
pub type CAP0MCI0_RE_R = crate::BitReader;
#[doc = "Field `CAP0MCI0_FE` reader - A 1 in this bit enables a channel 0 capture event on a falling edge on MCI0."]
pub type CAP0MCI0_FE_R = crate::BitReader;
#[doc = "Field `CAP0MCI1_RE` reader - A 1 in this bit enables a channel 0 capture event on a rising edge on MCI1."]
pub type CAP0MCI1_RE_R = crate::BitReader;
#[doc = "Field `CAP0MCI1_FE` reader - A 1 in this bit enables a channel 0 capture event on a falling edge on MCI1."]
pub type CAP0MCI1_FE_R = crate::BitReader;
#[doc = "Field `CAP0MCI2_RE` reader - A 1 in this bit enables a channel 0 capture event on a rising edge on MCI2."]
pub type CAP0MCI2_RE_R = crate::BitReader;
#[doc = "Field `CAP0MCI2_FE` reader - A 1 in this bit enables a channel 0 capture event on a falling edge on MCI2."]
pub type CAP0MCI2_FE_R = crate::BitReader;
#[doc = "Field `CAP1MCI0_RE` reader - A 1 in this bit enables a channel 1 capture event on a rising edge on MCI0."]
pub type CAP1MCI0_RE_R = crate::BitReader;
#[doc = "Field `CAP1MCI0_FE` reader - A 1 in this bit enables a channel 1 capture event on a falling edge on MCI0."]
pub type CAP1MCI0_FE_R = crate::BitReader;
#[doc = "Field `CAP1MCI1_RE` reader - A 1 in this bit enables a channel 1 capture event on a rising edge on MCI1."]
pub type CAP1MCI1_RE_R = crate::BitReader;
#[doc = "Field `CAP1MCI1_FE` reader - A 1 in this bit enables a channel 1 capture event on a falling edge on MCI1."]
pub type CAP1MCI1_FE_R = crate::BitReader;
#[doc = "Field `CAP1MCI2_RE` reader - A 1 in this bit enables a channel 1 capture event on a rising edge on MCI2."]
pub type CAP1MCI2_RE_R = crate::BitReader;
#[doc = "Field `CAP1MCI2_FE` reader - A 1 in this bit enables a channel 1 capture event on a falling edge on MCI2."]
pub type CAP1MCI2_FE_R = crate::BitReader;
#[doc = "Field `CAP2MCI0_RE` reader - A 1 in this bit enables a channel 2 capture event on a rising edge on MCI0."]
pub type CAP2MCI0_RE_R = crate::BitReader;
#[doc = "Field `CAP2MCI0_FE` reader - A 1 in this bit enables a channel 2 capture event on a falling edge on MCI0."]
pub type CAP2MCI0_FE_R = crate::BitReader;
#[doc = "Field `CAP2MCI1_RE` reader - A 1 in this bit enables a channel 2 capture event on a rising edge on MCI1."]
pub type CAP2MCI1_RE_R = crate::BitReader;
#[doc = "Field `CAP2MCI1_FE` reader - A 1 in this bit enables a channel 2 capture event on a falling edge on MCI1."]
pub type CAP2MCI1_FE_R = crate::BitReader;
#[doc = "Field `CAP2MCI2_RE` reader - A 1 in this bit enables a channel 2 capture event on a rising edge on MCI2."]
pub type CAP2MCI2_RE_R = crate::BitReader;
#[doc = "Field `CAP2MCI2_FE` reader - A 1 in this bit enables a channel 2 capture event on a falling edge on MCI2."]
pub type CAP2MCI2_FE_R = crate::BitReader;
#[doc = "Field `RT0` reader - If this bit is 1, TC0 is reset by a channel 0 capture event."]
pub type RT0_R = crate::BitReader;
#[doc = "Field `RT1` reader - If this bit is 1, TC1 is reset by a channel 1 capture event."]
pub type RT1_R = crate::BitReader;
#[doc = "Field `RT2` reader - If this bit is 1, TC2 is reset by a channel 2 capture event."]
pub type RT2_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - A 1 in this bit enables a channel 0 capture event on a rising edge on MCI0."]
    #[inline(always)]
    pub fn cap0mci0_re(&self) -> CAP0MCI0_RE_R {
        CAP0MCI0_RE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - A 1 in this bit enables a channel 0 capture event on a falling edge on MCI0."]
    #[inline(always)]
    pub fn cap0mci0_fe(&self) -> CAP0MCI0_FE_R {
        CAP0MCI0_FE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - A 1 in this bit enables a channel 0 capture event on a rising edge on MCI1."]
    #[inline(always)]
    pub fn cap0mci1_re(&self) -> CAP0MCI1_RE_R {
        CAP0MCI1_RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - A 1 in this bit enables a channel 0 capture event on a falling edge on MCI1."]
    #[inline(always)]
    pub fn cap0mci1_fe(&self) -> CAP0MCI1_FE_R {
        CAP0MCI1_FE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - A 1 in this bit enables a channel 0 capture event on a rising edge on MCI2."]
    #[inline(always)]
    pub fn cap0mci2_re(&self) -> CAP0MCI2_RE_R {
        CAP0MCI2_RE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - A 1 in this bit enables a channel 0 capture event on a falling edge on MCI2."]
    #[inline(always)]
    pub fn cap0mci2_fe(&self) -> CAP0MCI2_FE_R {
        CAP0MCI2_FE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - A 1 in this bit enables a channel 1 capture event on a rising edge on MCI0."]
    #[inline(always)]
    pub fn cap1mci0_re(&self) -> CAP1MCI0_RE_R {
        CAP1MCI0_RE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - A 1 in this bit enables a channel 1 capture event on a falling edge on MCI0."]
    #[inline(always)]
    pub fn cap1mci0_fe(&self) -> CAP1MCI0_FE_R {
        CAP1MCI0_FE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - A 1 in this bit enables a channel 1 capture event on a rising edge on MCI1."]
    #[inline(always)]
    pub fn cap1mci1_re(&self) -> CAP1MCI1_RE_R {
        CAP1MCI1_RE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - A 1 in this bit enables a channel 1 capture event on a falling edge on MCI1."]
    #[inline(always)]
    pub fn cap1mci1_fe(&self) -> CAP1MCI1_FE_R {
        CAP1MCI1_FE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - A 1 in this bit enables a channel 1 capture event on a rising edge on MCI2."]
    #[inline(always)]
    pub fn cap1mci2_re(&self) -> CAP1MCI2_RE_R {
        CAP1MCI2_RE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - A 1 in this bit enables a channel 1 capture event on a falling edge on MCI2."]
    #[inline(always)]
    pub fn cap1mci2_fe(&self) -> CAP1MCI2_FE_R {
        CAP1MCI2_FE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - A 1 in this bit enables a channel 2 capture event on a rising edge on MCI0."]
    #[inline(always)]
    pub fn cap2mci0_re(&self) -> CAP2MCI0_RE_R {
        CAP2MCI0_RE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - A 1 in this bit enables a channel 2 capture event on a falling edge on MCI0."]
    #[inline(always)]
    pub fn cap2mci0_fe(&self) -> CAP2MCI0_FE_R {
        CAP2MCI0_FE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - A 1 in this bit enables a channel 2 capture event on a rising edge on MCI1."]
    #[inline(always)]
    pub fn cap2mci1_re(&self) -> CAP2MCI1_RE_R {
        CAP2MCI1_RE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - A 1 in this bit enables a channel 2 capture event on a falling edge on MCI1."]
    #[inline(always)]
    pub fn cap2mci1_fe(&self) -> CAP2MCI1_FE_R {
        CAP2MCI1_FE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - A 1 in this bit enables a channel 2 capture event on a rising edge on MCI2."]
    #[inline(always)]
    pub fn cap2mci2_re(&self) -> CAP2MCI2_RE_R {
        CAP2MCI2_RE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - A 1 in this bit enables a channel 2 capture event on a falling edge on MCI2."]
    #[inline(always)]
    pub fn cap2mci2_fe(&self) -> CAP2MCI2_FE_R {
        CAP2MCI2_FE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - If this bit is 1, TC0 is reset by a channel 0 capture event."]
    #[inline(always)]
    pub fn rt0(&self) -> RT0_R {
        RT0_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - If this bit is 1, TC1 is reset by a channel 1 capture event."]
    #[inline(always)]
    pub fn rt1(&self) -> RT1_R {
        RT1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - If this bit is 1, TC2 is reset by a channel 2 capture event."]
    #[inline(always)]
    pub fn rt2(&self) -> RT2_R {
        RT2_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAPCON")
            .field("cap0mci0_re", &format_args!("{}", self.cap0mci0_re().bit()))
            .field("cap0mci0_fe", &format_args!("{}", self.cap0mci0_fe().bit()))
            .field("cap0mci1_re", &format_args!("{}", self.cap0mci1_re().bit()))
            .field("cap0mci1_fe", &format_args!("{}", self.cap0mci1_fe().bit()))
            .field("cap0mci2_re", &format_args!("{}", self.cap0mci2_re().bit()))
            .field("cap0mci2_fe", &format_args!("{}", self.cap0mci2_fe().bit()))
            .field("cap1mci0_re", &format_args!("{}", self.cap1mci0_re().bit()))
            .field("cap1mci0_fe", &format_args!("{}", self.cap1mci0_fe().bit()))
            .field("cap1mci1_re", &format_args!("{}", self.cap1mci1_re().bit()))
            .field("cap1mci1_fe", &format_args!("{}", self.cap1mci1_fe().bit()))
            .field("cap1mci2_re", &format_args!("{}", self.cap1mci2_re().bit()))
            .field("cap1mci2_fe", &format_args!("{}", self.cap1mci2_fe().bit()))
            .field("cap2mci0_re", &format_args!("{}", self.cap2mci0_re().bit()))
            .field("cap2mci0_fe", &format_args!("{}", self.cap2mci0_fe().bit()))
            .field("cap2mci1_re", &format_args!("{}", self.cap2mci1_re().bit()))
            .field("cap2mci1_fe", &format_args!("{}", self.cap2mci1_fe().bit()))
            .field("cap2mci2_re", &format_args!("{}", self.cap2mci2_re().bit()))
            .field("cap2mci2_fe", &format_args!("{}", self.cap2mci2_fe().bit()))
            .field("rt0", &format_args!("{}", self.rt0().bit()))
            .field("rt1", &format_args!("{}", self.rt1().bit()))
            .field("rt2", &format_args!("{}", self.rt2().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CAPCON_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Capture Control read address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capcon::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAPCON_SPEC;
impl crate::RegisterSpec for CAPCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`capcon::R`](R) reader structure"]
impl crate::Readable for CAPCON_SPEC {}
#[doc = "`reset()` method sets CAPCON to value 0"]
impl crate::Resettable for CAPCON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
