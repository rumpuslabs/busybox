#[doc = "Register `RTC_AUXEN` reader"]
pub type R = crate::R<RTC_AUXEN_SPEC>;
#[doc = "Register `RTC_AUXEN` writer"]
pub type W = crate::W<RTC_AUXEN_SPEC>;
#[doc = "Field `RTC_OSCFEN` reader - Oscillator Fail Detect interrupt enable. When 0: the RTC Oscillator Fail detect interrupt is disabled. When 1: the RTC Oscillator Fail detect interrupt is enabled. See Section 30.6.2.5."]
pub type RTC_OSCFEN_R = crate::BitReader;
#[doc = "Field `RTC_OSCFEN` writer - Oscillator Fail Detect interrupt enable. When 0: the RTC Oscillator Fail detect interrupt is disabled. When 1: the RTC Oscillator Fail detect interrupt is enabled. See Section 30.6.2.5."]
pub type RTC_OSCFEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 4 - Oscillator Fail Detect interrupt enable. When 0: the RTC Oscillator Fail detect interrupt is disabled. When 1: the RTC Oscillator Fail detect interrupt is enabled. See Section 30.6.2.5."]
    #[inline(always)]
    pub fn rtc_oscfen(&self) -> RTC_OSCFEN_R {
        RTC_OSCFEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_AUXEN")
            .field("rtc_oscfen", &format_args!("{}", self.rtc_oscfen().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<RTC_AUXEN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 4 - Oscillator Fail Detect interrupt enable. When 0: the RTC Oscillator Fail detect interrupt is disabled. When 1: the RTC Oscillator Fail detect interrupt is enabled. See Section 30.6.2.5."]
    #[inline(always)]
    #[must_use]
    pub fn rtc_oscfen(&mut self) -> RTC_OSCFEN_W<RTC_AUXEN_SPEC, 4> {
        RTC_OSCFEN_W::new(self)
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
#[doc = "RTC Auxiliary Enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_auxen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_auxen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_AUXEN_SPEC;
impl crate::RegisterSpec for RTC_AUXEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_auxen::R`](R) reader structure"]
impl crate::Readable for RTC_AUXEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_auxen::W`](W) writer structure"]
impl crate::Writable for RTC_AUXEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTC_AUXEN to value 0"]
impl crate::Resettable for RTC_AUXEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
