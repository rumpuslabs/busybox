#[doc = "Register `CLKSRCSEL` reader"]
pub type R = crate::R<CLKSRCSEL_SPEC>;
#[doc = "Register `CLKSRCSEL` writer"]
pub type W = crate::W<CLKSRCSEL_SPEC>;
#[doc = "Field `CLKSRC` reader - Selects the clock source for PLL0 as follows. Warning: Improper setting of this value, or an incorrect sequence of changing this value may result in incorrect operation of the device."]
pub type CLKSRC_R = crate::FieldReader<CLKSRC_A>;
#[doc = "Selects the clock source for PLL0 as follows. Warning: Improper setting of this value, or an incorrect sequence of changing this value may result in incorrect operation of the device.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSRC_A {
    #[doc = "0: Selects the Internal RC oscillator as the PLL0 clock source (default)."]
    INTERNAL_RC_OSC = 0,
    #[doc = "1: Selects the main oscillator as the PLL0 clock source. Select the main oscillator as PLL0 clock source if the PLL0 clock output is used for USB or for CAN with baudrates > 100 kBit/s."]
    MAIN_OSC = 1,
    #[doc = "2: Selects the RTC oscillator as the PLL0 clock source."]
    RTC_OSC = 2,
}
impl From<CLKSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSRC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLKSRC_A {
    type Ux = u8;
}
impl CLKSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLKSRC_A {
        match self.bits {
            0 => CLKSRC_A::INTERNAL_RC_OSC,
            1 => CLKSRC_A::MAIN_OSC,
            2 => CLKSRC_A::RTC_OSC,
            _ => unreachable!(),
        }
    }
    #[doc = "Selects the Internal RC oscillator as the PLL0 clock source (default)."]
    #[inline(always)]
    pub fn is_internal_rc_osc(&self) -> bool {
        *self == CLKSRC_A::INTERNAL_RC_OSC
    }
    #[doc = "Selects the main oscillator as the PLL0 clock source. Select the main oscillator as PLL0 clock source if the PLL0 clock output is used for USB or for CAN with baudrates > 100 kBit/s."]
    #[inline(always)]
    pub fn is_main_osc(&self) -> bool {
        *self == CLKSRC_A::MAIN_OSC
    }
    #[doc = "Selects the RTC oscillator as the PLL0 clock source."]
    #[inline(always)]
    pub fn is_rtc_osc(&self) -> bool {
        *self == CLKSRC_A::RTC_OSC
    }
}
#[doc = "Field `CLKSRC` writer - Selects the clock source for PLL0 as follows. Warning: Improper setting of this value, or an incorrect sequence of changing this value may result in incorrect operation of the device."]
pub type CLKSRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, CLKSRC_A>;
impl<'a, REG, const O: u8> CLKSRC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Selects the Internal RC oscillator as the PLL0 clock source (default)."]
    #[inline(always)]
    pub fn internal_rc_osc(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSRC_A::INTERNAL_RC_OSC)
    }
    #[doc = "Selects the main oscillator as the PLL0 clock source. Select the main oscillator as PLL0 clock source if the PLL0 clock output is used for USB or for CAN with baudrates > 100 kBit/s."]
    #[inline(always)]
    pub fn main_osc(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSRC_A::MAIN_OSC)
    }
    #[doc = "Selects the RTC oscillator as the PLL0 clock source."]
    #[inline(always)]
    pub fn rtc_osc(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSRC_A::RTC_OSC)
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects the clock source for PLL0 as follows. Warning: Improper setting of this value, or an incorrect sequence of changing this value may result in incorrect operation of the device."]
    #[inline(always)]
    pub fn clksrc(&self) -> CLKSRC_R {
        CLKSRC_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKSRCSEL")
            .field("clksrc", &format_args!("{}", self.clksrc().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CLKSRCSEL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects the clock source for PLL0 as follows. Warning: Improper setting of this value, or an incorrect sequence of changing this value may result in incorrect operation of the device."]
    #[inline(always)]
    #[must_use]
    pub fn clksrc(&mut self) -> CLKSRC_W<CLKSRCSEL_SPEC, 0> {
        CLKSRC_W::new(self)
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
#[doc = "Clock Source Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksrcsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksrcsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKSRCSEL_SPEC;
impl crate::RegisterSpec for CLKSRCSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clksrcsel::R`](R) reader structure"]
impl crate::Readable for CLKSRCSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clksrcsel::W`](W) writer structure"]
impl crate::Writable for CLKSRCSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKSRCSEL to value 0"]
impl crate::Resettable for CLKSRCSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
