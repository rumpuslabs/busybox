#[doc = "Register `CLKOUTCFG` reader"]
pub type R = crate::R<CLKOUTCFG_SPEC>;
#[doc = "Register `CLKOUTCFG` writer"]
pub type W = crate::W<CLKOUTCFG_SPEC>;
#[doc = "Field `CLKOUTSEL` reader - Selects the clock source for the CLKOUT function. Other values are reserved. Do not use."]
pub type CLKOUTSEL_R = crate::FieldReader<CLKOUTSEL_A>;
#[doc = "Selects the clock source for the CLKOUT function. Other values are reserved. Do not use.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKOUTSEL_A {
    #[doc = "0: Selects the CPU clock as the CLKOUT source."]
    CPU_CLK = 0,
    #[doc = "1: Selects the main oscillator as the CLKOUT source."]
    MAIN_OSC = 1,
    #[doc = "2: Selects the Internal RC oscillator as the CLKOUT source."]
    INTERNAL_RC_OSC = 2,
    #[doc = "3: Selects the USB clock as the CLKOUT source."]
    USB_CLK = 3,
    #[doc = "4: Selects the RTC oscillator as the CLKOUT source."]
    RTC_OSC = 4,
}
impl From<CLKOUTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUTSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLKOUTSEL_A {
    type Ux = u8;
}
impl CLKOUTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLKOUTSEL_A> {
        match self.bits {
            0 => Some(CLKOUTSEL_A::CPU_CLK),
            1 => Some(CLKOUTSEL_A::MAIN_OSC),
            2 => Some(CLKOUTSEL_A::INTERNAL_RC_OSC),
            3 => Some(CLKOUTSEL_A::USB_CLK),
            4 => Some(CLKOUTSEL_A::RTC_OSC),
            _ => None,
        }
    }
    #[doc = "Selects the CPU clock as the CLKOUT source."]
    #[inline(always)]
    pub fn is_cpu_clk(&self) -> bool {
        *self == CLKOUTSEL_A::CPU_CLK
    }
    #[doc = "Selects the main oscillator as the CLKOUT source."]
    #[inline(always)]
    pub fn is_main_osc(&self) -> bool {
        *self == CLKOUTSEL_A::MAIN_OSC
    }
    #[doc = "Selects the Internal RC oscillator as the CLKOUT source."]
    #[inline(always)]
    pub fn is_internal_rc_osc(&self) -> bool {
        *self == CLKOUTSEL_A::INTERNAL_RC_OSC
    }
    #[doc = "Selects the USB clock as the CLKOUT source."]
    #[inline(always)]
    pub fn is_usb_clk(&self) -> bool {
        *self == CLKOUTSEL_A::USB_CLK
    }
    #[doc = "Selects the RTC oscillator as the CLKOUT source."]
    #[inline(always)]
    pub fn is_rtc_osc(&self) -> bool {
        *self == CLKOUTSEL_A::RTC_OSC
    }
}
#[doc = "Field `CLKOUTSEL` writer - Selects the clock source for the CLKOUT function. Other values are reserved. Do not use."]
pub type CLKOUTSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, CLKOUTSEL_A>;
impl<'a, REG, const O: u8> CLKOUTSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Selects the CPU clock as the CLKOUT source."]
    #[inline(always)]
    pub fn cpu_clk(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL_A::CPU_CLK)
    }
    #[doc = "Selects the main oscillator as the CLKOUT source."]
    #[inline(always)]
    pub fn main_osc(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL_A::MAIN_OSC)
    }
    #[doc = "Selects the Internal RC oscillator as the CLKOUT source."]
    #[inline(always)]
    pub fn internal_rc_osc(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL_A::INTERNAL_RC_OSC)
    }
    #[doc = "Selects the USB clock as the CLKOUT source."]
    #[inline(always)]
    pub fn usb_clk(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL_A::USB_CLK)
    }
    #[doc = "Selects the RTC oscillator as the CLKOUT source."]
    #[inline(always)]
    pub fn rtc_osc(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUTSEL_A::RTC_OSC)
    }
}
#[doc = "Field `CLKOUTDIV` reader - Integer value to divide the output clock by, minus one. 0 = Clock is divided by 1 1 = Clock is divided by 2. 2 = Clock is divided by 3. ... 15 = Clock is divided by 16."]
pub type CLKOUTDIV_R = crate::FieldReader;
#[doc = "Field `CLKOUTDIV` writer - Integer value to divide the output clock by, minus one. 0 = Clock is divided by 1 1 = Clock is divided by 2. 2 = Clock is divided by 3. ... 15 = Clock is divided by 16."]
pub type CLKOUTDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CLKOUT_EN` reader - CLKOUT enable control, allows switching the CLKOUT source without glitches. Clear to stop CLKOUT on the next falling edge. Set to enable CLKOUT."]
pub type CLKOUT_EN_R = crate::BitReader;
#[doc = "Field `CLKOUT_EN` writer - CLKOUT enable control, allows switching the CLKOUT source without glitches. Clear to stop CLKOUT on the next falling edge. Set to enable CLKOUT."]
pub type CLKOUT_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLKOUT_ACT` reader - CLKOUT activity indication. Reads as 1 when CLKOUT is enabled. Read as 0 when CLKOUT has been disabled via the CLKOUT_EN bit and the clock has completed being stopped."]
pub type CLKOUT_ACT_R = crate::BitReader;
#[doc = "Field `CLKOUT_ACT` writer - CLKOUT activity indication. Reads as 1 when CLKOUT is enabled. Read as 0 when CLKOUT has been disabled via the CLKOUT_EN bit and the clock has completed being stopped."]
pub type CLKOUT_ACT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3 - Selects the clock source for the CLKOUT function. Other values are reserved. Do not use."]
    #[inline(always)]
    pub fn clkoutsel(&self) -> CLKOUTSEL_R {
        CLKOUTSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Integer value to divide the output clock by, minus one. 0 = Clock is divided by 1 1 = Clock is divided by 2. 2 = Clock is divided by 3. ... 15 = Clock is divided by 16."]
    #[inline(always)]
    pub fn clkoutdiv(&self) -> CLKOUTDIV_R {
        CLKOUTDIV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - CLKOUT enable control, allows switching the CLKOUT source without glitches. Clear to stop CLKOUT on the next falling edge. Set to enable CLKOUT."]
    #[inline(always)]
    pub fn clkout_en(&self) -> CLKOUT_EN_R {
        CLKOUT_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CLKOUT activity indication. Reads as 1 when CLKOUT is enabled. Read as 0 when CLKOUT has been disabled via the CLKOUT_EN bit and the clock has completed being stopped."]
    #[inline(always)]
    pub fn clkout_act(&self) -> CLKOUT_ACT_R {
        CLKOUT_ACT_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKOUTCFG")
            .field("clkoutsel", &format_args!("{}", self.clkoutsel().bits()))
            .field("clkoutdiv", &format_args!("{}", self.clkoutdiv().bits()))
            .field("clkout_en", &format_args!("{}", self.clkout_en().bit()))
            .field("clkout_act", &format_args!("{}", self.clkout_act().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CLKOUTCFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Selects the clock source for the CLKOUT function. Other values are reserved. Do not use."]
    #[inline(always)]
    #[must_use]
    pub fn clkoutsel(&mut self) -> CLKOUTSEL_W<CLKOUTCFG_SPEC, 0> {
        CLKOUTSEL_W::new(self)
    }
    #[doc = "Bits 4:7 - Integer value to divide the output clock by, minus one. 0 = Clock is divided by 1 1 = Clock is divided by 2. 2 = Clock is divided by 3. ... 15 = Clock is divided by 16."]
    #[inline(always)]
    #[must_use]
    pub fn clkoutdiv(&mut self) -> CLKOUTDIV_W<CLKOUTCFG_SPEC, 4> {
        CLKOUTDIV_W::new(self)
    }
    #[doc = "Bit 8 - CLKOUT enable control, allows switching the CLKOUT source without glitches. Clear to stop CLKOUT on the next falling edge. Set to enable CLKOUT."]
    #[inline(always)]
    #[must_use]
    pub fn clkout_en(&mut self) -> CLKOUT_EN_W<CLKOUTCFG_SPEC, 8> {
        CLKOUT_EN_W::new(self)
    }
    #[doc = "Bit 9 - CLKOUT activity indication. Reads as 1 when CLKOUT is enabled. Read as 0 when CLKOUT has been disabled via the CLKOUT_EN bit and the clock has completed being stopped."]
    #[inline(always)]
    #[must_use]
    pub fn clkout_act(&mut self) -> CLKOUT_ACT_W<CLKOUTCFG_SPEC, 9> {
        CLKOUT_ACT_W::new(self)
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
#[doc = "Clock Output Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkoutcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkoutcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKOUTCFG_SPEC;
impl crate::RegisterSpec for CLKOUTCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkoutcfg::R`](R) reader structure"]
impl crate::Readable for CLKOUTCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clkoutcfg::W`](W) writer structure"]
impl crate::Writable for CLKOUTCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKOUTCFG to value 0"]
impl crate::Resettable for CLKOUTCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
