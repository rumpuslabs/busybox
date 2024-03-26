#[doc = "Register `RTC_AUX` reader"]
pub type R = crate::R<RTC_AUX_SPEC>;
#[doc = "Register `RTC_AUX` writer"]
pub type W = crate::W<RTC_AUX_SPEC>;
#[doc = "Field `RTC_OSCF` reader - RTC Oscillator Fail detect flag. Read: this bit is set if the RTC oscillator stops, and when RTC power is first turned on. An interrupt will occur when this bit is set, the RTC_OSCFEN bit in RTC_AUXEN is a 1, and the RTC interrupt is enabled in the NVIC. Write: writing a 1 to this bit clears the flag."]
pub type RTC_OSCF_R = crate::BitReader;
#[doc = "Field `RTC_OSCF` writer - RTC Oscillator Fail detect flag. Read: this bit is set if the RTC oscillator stops, and when RTC power is first turned on. An interrupt will occur when this bit is set, the RTC_OSCFEN bit in RTC_AUXEN is a 1, and the RTC interrupt is enabled in the NVIC. Write: writing a 1 to this bit clears the flag."]
pub type RTC_OSCF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTC_PDOUT` reader - When 0: the RTC_ALARM pin reflects the RTC alarm status. When 1: the RTC_ALARM pin indicates Deep Power-down mode."]
pub type RTC_PDOUT_R = crate::BitReader;
#[doc = "Field `RTC_PDOUT` writer - When 0: the RTC_ALARM pin reflects the RTC alarm status. When 1: the RTC_ALARM pin indicates Deep Power-down mode."]
pub type RTC_PDOUT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 4 - RTC Oscillator Fail detect flag. Read: this bit is set if the RTC oscillator stops, and when RTC power is first turned on. An interrupt will occur when this bit is set, the RTC_OSCFEN bit in RTC_AUXEN is a 1, and the RTC interrupt is enabled in the NVIC. Write: writing a 1 to this bit clears the flag."]
    #[inline(always)]
    pub fn rtc_oscf(&self) -> RTC_OSCF_R {
        RTC_OSCF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - When 0: the RTC_ALARM pin reflects the RTC alarm status. When 1: the RTC_ALARM pin indicates Deep Power-down mode."]
    #[inline(always)]
    pub fn rtc_pdout(&self) -> RTC_PDOUT_R {
        RTC_PDOUT_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_AUX")
            .field("rtc_oscf", &format_args!("{}", self.rtc_oscf().bit()))
            .field("rtc_pdout", &format_args!("{}", self.rtc_pdout().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<RTC_AUX_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 4 - RTC Oscillator Fail detect flag. Read: this bit is set if the RTC oscillator stops, and when RTC power is first turned on. An interrupt will occur when this bit is set, the RTC_OSCFEN bit in RTC_AUXEN is a 1, and the RTC interrupt is enabled in the NVIC. Write: writing a 1 to this bit clears the flag."]
    #[inline(always)]
    #[must_use]
    pub fn rtc_oscf(&mut self) -> RTC_OSCF_W<RTC_AUX_SPEC, 4> {
        RTC_OSCF_W::new(self)
    }
    #[doc = "Bit 6 - When 0: the RTC_ALARM pin reflects the RTC alarm status. When 1: the RTC_ALARM pin indicates Deep Power-down mode."]
    #[inline(always)]
    #[must_use]
    pub fn rtc_pdout(&mut self) -> RTC_PDOUT_W<RTC_AUX_SPEC, 6> {
        RTC_PDOUT_W::new(self)
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
#[doc = "RTC Auxiliary control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_aux::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_aux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_AUX_SPEC;
impl crate::RegisterSpec for RTC_AUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_aux::R`](R) reader structure"]
impl crate::Readable for RTC_AUX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_aux::W`](W) writer structure"]
impl crate::Writable for RTC_AUX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTC_AUX to value 0x10"]
impl crate::Resettable for RTC_AUX_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
