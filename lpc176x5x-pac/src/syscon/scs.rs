#[doc = "Register `SCS` reader"]
pub type R = crate::R<SCS_SPEC>;
#[doc = "Register `SCS` writer"]
pub type W = crate::W<SCS_SPEC>;
#[doc = "Field `OSCRANGE` reader - Main oscillator range select."]
pub type OSCRANGE_R = crate::BitReader<OSCRANGE_A>;
#[doc = "Main oscillator range select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSCRANGE_A {
    #[doc = "0: Low. The frequency range of the main oscillator is 1 MHz to 20 MHz."]
    LOW = 0,
    #[doc = "1: High. The frequency range of the main oscillator is 15 MHz to 25 MHz."]
    HIGH = 1,
}
impl From<OSCRANGE_A> for bool {
    #[inline(always)]
    fn from(variant: OSCRANGE_A) -> Self {
        variant as u8 != 0
    }
}
impl OSCRANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSCRANGE_A {
        match self.bits {
            false => OSCRANGE_A::LOW,
            true => OSCRANGE_A::HIGH,
        }
    }
    #[doc = "Low. The frequency range of the main oscillator is 1 MHz to 20 MHz."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OSCRANGE_A::LOW
    }
    #[doc = "High. The frequency range of the main oscillator is 15 MHz to 25 MHz."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OSCRANGE_A::HIGH
    }
}
#[doc = "Field `OSCRANGE` writer - Main oscillator range select."]
pub type OSCRANGE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OSCRANGE_A>;
impl<'a, REG, const O: u8> OSCRANGE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low. The frequency range of the main oscillator is 1 MHz to 20 MHz."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(OSCRANGE_A::LOW)
    }
    #[doc = "High. The frequency range of the main oscillator is 15 MHz to 25 MHz."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(OSCRANGE_A::HIGH)
    }
}
#[doc = "Field `OSCEN` reader - Main oscillator enable."]
pub type OSCEN_R = crate::BitReader<OSCEN_A>;
#[doc = "Main oscillator enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSCEN_A {
    #[doc = "0: Disabled. The main oscillator is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled.The main oscillator is enabled, and will start up if the correct external circuitry is connected to the XTAL1 and XTAL2 pins."]
    ENABLED = 1,
}
impl From<OSCEN_A> for bool {
    #[inline(always)]
    fn from(variant: OSCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl OSCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSCEN_A {
        match self.bits {
            false => OSCEN_A::DISABLED,
            true => OSCEN_A::ENABLED,
        }
    }
    #[doc = "Disabled. The main oscillator is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSCEN_A::DISABLED
    }
    #[doc = "Enabled.The main oscillator is enabled, and will start up if the correct external circuitry is connected to the XTAL1 and XTAL2 pins."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OSCEN_A::ENABLED
    }
}
#[doc = "Field `OSCEN` writer - Main oscillator enable."]
pub type OSCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OSCEN_A>;
impl<'a, REG, const O: u8> OSCEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. The main oscillator is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OSCEN_A::DISABLED)
    }
    #[doc = "Enabled.The main oscillator is enabled, and will start up if the correct external circuitry is connected to the XTAL1 and XTAL2 pins."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OSCEN_A::ENABLED)
    }
}
#[doc = "Field `OSCSTAT` reader - Main oscillator status."]
pub type OSCSTAT_R = crate::BitReader<OSCSTAT_A>;
#[doc = "Main oscillator status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSCSTAT_A {
    #[doc = "0: Not ready. The main oscillator is not ready to be used as a clock source."]
    NOT_READY = 0,
    #[doc = "1: Ready. The main oscillator is ready to be used as a clock source. The main oscillator must be enabled via the OSCEN bit."]
    READY = 1,
}
impl From<OSCSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: OSCSTAT_A) -> Self {
        variant as u8 != 0
    }
}
impl OSCSTAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSCSTAT_A {
        match self.bits {
            false => OSCSTAT_A::NOT_READY,
            true => OSCSTAT_A::READY,
        }
    }
    #[doc = "Not ready. The main oscillator is not ready to be used as a clock source."]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == OSCSTAT_A::NOT_READY
    }
    #[doc = "Ready. The main oscillator is ready to be used as a clock source. The main oscillator must be enabled via the OSCEN bit."]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == OSCSTAT_A::READY
    }
}
#[doc = "Field `OSCSTAT` writer - Main oscillator status."]
pub type OSCSTAT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OSCSTAT_A>;
impl<'a, REG, const O: u8> OSCSTAT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not ready. The main oscillator is not ready to be used as a clock source."]
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut crate::W<REG> {
        self.variant(OSCSTAT_A::NOT_READY)
    }
    #[doc = "Ready. The main oscillator is ready to be used as a clock source. The main oscillator must be enabled via the OSCEN bit."]
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(OSCSTAT_A::READY)
    }
}
impl R {
    #[doc = "Bit 4 - Main oscillator range select."]
    #[inline(always)]
    pub fn oscrange(&self) -> OSCRANGE_R {
        OSCRANGE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Main oscillator enable."]
    #[inline(always)]
    pub fn oscen(&self) -> OSCEN_R {
        OSCEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Main oscillator status."]
    #[inline(always)]
    pub fn oscstat(&self) -> OSCSTAT_R {
        OSCSTAT_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCS")
            .field("oscrange", &format_args!("{}", self.oscrange().bit()))
            .field("oscen", &format_args!("{}", self.oscen().bit()))
            .field("oscstat", &format_args!("{}", self.oscstat().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<SCS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 4 - Main oscillator range select."]
    #[inline(always)]
    #[must_use]
    pub fn oscrange(&mut self) -> OSCRANGE_W<SCS_SPEC, 4> {
        OSCRANGE_W::new(self)
    }
    #[doc = "Bit 5 - Main oscillator enable."]
    #[inline(always)]
    #[must_use]
    pub fn oscen(&mut self) -> OSCEN_W<SCS_SPEC, 5> {
        OSCEN_W::new(self)
    }
    #[doc = "Bit 6 - Main oscillator status."]
    #[inline(always)]
    #[must_use]
    pub fn oscstat(&mut self) -> OSCSTAT_W<SCS_SPEC, 6> {
        OSCSTAT_W::new(self)
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
#[doc = "System control and status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCS_SPEC;
impl crate::RegisterSpec for SCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scs::R`](R) reader structure"]
impl crate::Readable for SCS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scs::W`](W) writer structure"]
impl crate::Writable for SCS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCS to value 0"]
impl crate::Resettable for SCS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
