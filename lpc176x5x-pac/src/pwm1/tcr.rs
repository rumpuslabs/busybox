#[doc = "Register `TCR` reader"]
pub type R = crate::R<TCR_SPEC>;
#[doc = "Register `TCR` writer"]
pub type W = crate::W<TCR_SPEC>;
#[doc = "Field `CE` reader - Counter Enable"]
pub type CE_R = crate::BitReader<CE_A>;
#[doc = "Counter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CE_A {
    #[doc = "1: The PWM Timer Counter and PWM Prescale Counter are enabled for counting."]
    ENABLED = 1,
    #[doc = "0: The counters are disabled."]
    DISABLED = 0,
}
impl From<CE_A> for bool {
    #[inline(always)]
    fn from(variant: CE_A) -> Self {
        variant as u8 != 0
    }
}
impl CE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CE_A {
        match self.bits {
            true => CE_A::ENABLED,
            false => CE_A::DISABLED,
        }
    }
    #[doc = "The PWM Timer Counter and PWM Prescale Counter are enabled for counting."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CE_A::ENABLED
    }
    #[doc = "The counters are disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CE_A::DISABLED
    }
}
#[doc = "Field `CE` writer - Counter Enable"]
pub type CE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CE_A>;
impl<'a, REG, const O: u8> CE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The PWM Timer Counter and PWM Prescale Counter are enabled for counting."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CE_A::ENABLED)
    }
    #[doc = "The counters are disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CE_A::DISABLED)
    }
}
#[doc = "Field `CR` reader - Counter Reset"]
pub type CR_R = crate::BitReader<CR_A>;
#[doc = "Counter Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CR_A {
    #[doc = "1: The PWM Timer Counter and the PWM Prescale Counter are synchronously reset on the next positive edge of PCLK. The counters remain reset until this bit is returned to zero."]
    RESET = 1,
    #[doc = "0: Clear reset."]
    CLEAR_RESET = 0,
}
impl From<CR_A> for bool {
    #[inline(always)]
    fn from(variant: CR_A) -> Self {
        variant as u8 != 0
    }
}
impl CR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CR_A {
        match self.bits {
            true => CR_A::RESET,
            false => CR_A::CLEAR_RESET,
        }
    }
    #[doc = "The PWM Timer Counter and the PWM Prescale Counter are synchronously reset on the next positive edge of PCLK. The counters remain reset until this bit is returned to zero."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == CR_A::RESET
    }
    #[doc = "Clear reset."]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == CR_A::CLEAR_RESET
    }
}
#[doc = "Field `CR` writer - Counter Reset"]
pub type CR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CR_A>;
impl<'a, REG, const O: u8> CR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The PWM Timer Counter and the PWM Prescale Counter are synchronously reset on the next positive edge of PCLK. The counters remain reset until this bit is returned to zero."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(CR_A::RESET)
    }
    #[doc = "Clear reset."]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut crate::W<REG> {
        self.variant(CR_A::CLEAR_RESET)
    }
}
#[doc = "Field `PWMEN` reader - PWM Enable"]
pub type PWMEN_R = crate::BitReader<PWMEN_A>;
#[doc = "PWM Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWMEN_A {
    #[doc = "1: PWM mode is enabled (counter resets to 1). PWM mode causes the shadow registers to operate in connection with the Match registers. A program write to a Match register will not have an effect on the Match result until the corresponding bit in PWMLER has been set, followed by the occurrence of a PWM Match 0 event. Note that the PWM Match register that determines the PWM rate (PWM Match Register 0 - MR0) must be set up prior to the PWM being enabled. Otherwise a Match event will not occur to cause shadow register contents to become effective."]
    PWM_MODE = 1,
    #[doc = "0: Timer mode is enabled (counter resets to 0)."]
    TIMER_MODE = 0,
}
impl From<PWMEN_A> for bool {
    #[inline(always)]
    fn from(variant: PWMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PWMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWMEN_A {
        match self.bits {
            true => PWMEN_A::PWM_MODE,
            false => PWMEN_A::TIMER_MODE,
        }
    }
    #[doc = "PWM mode is enabled (counter resets to 1). PWM mode causes the shadow registers to operate in connection with the Match registers. A program write to a Match register will not have an effect on the Match result until the corresponding bit in PWMLER has been set, followed by the occurrence of a PWM Match 0 event. Note that the PWM Match register that determines the PWM rate (PWM Match Register 0 - MR0) must be set up prior to the PWM being enabled. Otherwise a Match event will not occur to cause shadow register contents to become effective."]
    #[inline(always)]
    pub fn is_pwm_mode(&self) -> bool {
        *self == PWMEN_A::PWM_MODE
    }
    #[doc = "Timer mode is enabled (counter resets to 0)."]
    #[inline(always)]
    pub fn is_timer_mode(&self) -> bool {
        *self == PWMEN_A::TIMER_MODE
    }
}
#[doc = "Field `PWMEN` writer - PWM Enable"]
pub type PWMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PWMEN_A>;
impl<'a, REG, const O: u8> PWMEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PWM mode is enabled (counter resets to 1). PWM mode causes the shadow registers to operate in connection with the Match registers. A program write to a Match register will not have an effect on the Match result until the corresponding bit in PWMLER has been set, followed by the occurrence of a PWM Match 0 event. Note that the PWM Match register that determines the PWM rate (PWM Match Register 0 - MR0) must be set up prior to the PWM being enabled. Otherwise a Match event will not occur to cause shadow register contents to become effective."]
    #[inline(always)]
    pub fn pwm_mode(self) -> &'a mut crate::W<REG> {
        self.variant(PWMEN_A::PWM_MODE)
    }
    #[doc = "Timer mode is enabled (counter resets to 0)."]
    #[inline(always)]
    pub fn timer_mode(self) -> &'a mut crate::W<REG> {
        self.variant(PWMEN_A::TIMER_MODE)
    }
}
#[doc = "Field `MDIS` reader - Master Disable (PWM0 only). The two PWMs may be synchronized using the Master Disable control bit. The Master disable bit of the Master PWM (PWM0 module) controls a secondary enable input to both PWMs, as shown in Figure 141. This bit has no function in the Slave PWM (PWM1)."]
pub type MDIS_R = crate::BitReader<MDIS_A>;
#[doc = "Master Disable (PWM0 only). The two PWMs may be synchronized using the Master Disable control bit. The Master disable bit of the Master PWM (PWM0 module) controls a secondary enable input to both PWMs, as shown in Figure 141. This bit has no function in the Slave PWM (PWM1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MDIS_A {
    #[doc = "1: Master use. PWM0 is the master, and both PWMs are enabled for counting."]
    MASTER_USE = 1,
    #[doc = "0: Individual use. The PWMs are used independently, and the individual Counter Enable bits are used to control the PWMs."]
    INDIVIDUAL_USE = 0,
}
impl From<MDIS_A> for bool {
    #[inline(always)]
    fn from(variant: MDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl MDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MDIS_A {
        match self.bits {
            true => MDIS_A::MASTER_USE,
            false => MDIS_A::INDIVIDUAL_USE,
        }
    }
    #[doc = "Master use. PWM0 is the master, and both PWMs are enabled for counting."]
    #[inline(always)]
    pub fn is_master_use(&self) -> bool {
        *self == MDIS_A::MASTER_USE
    }
    #[doc = "Individual use. The PWMs are used independently, and the individual Counter Enable bits are used to control the PWMs."]
    #[inline(always)]
    pub fn is_individual_use(&self) -> bool {
        *self == MDIS_A::INDIVIDUAL_USE
    }
}
#[doc = "Field `MDIS` writer - Master Disable (PWM0 only). The two PWMs may be synchronized using the Master Disable control bit. The Master disable bit of the Master PWM (PWM0 module) controls a secondary enable input to both PWMs, as shown in Figure 141. This bit has no function in the Slave PWM (PWM1)."]
pub type MDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MDIS_A>;
impl<'a, REG, const O: u8> MDIS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master use. PWM0 is the master, and both PWMs are enabled for counting."]
    #[inline(always)]
    pub fn master_use(self) -> &'a mut crate::W<REG> {
        self.variant(MDIS_A::MASTER_USE)
    }
    #[doc = "Individual use. The PWMs are used independently, and the individual Counter Enable bits are used to control the PWMs."]
    #[inline(always)]
    pub fn individual_use(self) -> &'a mut crate::W<REG> {
        self.variant(MDIS_A::INDIVIDUAL_USE)
    }
}
impl R {
    #[doc = "Bit 0 - Counter Enable"]
    #[inline(always)]
    pub fn ce(&self) -> CE_R {
        CE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counter Reset"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - PWM Enable"]
    #[inline(always)]
    pub fn pwmen(&self) -> PWMEN_R {
        PWMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Master Disable (PWM0 only). The two PWMs may be synchronized using the Master Disable control bit. The Master disable bit of the Master PWM (PWM0 module) controls a secondary enable input to both PWMs, as shown in Figure 141. This bit has no function in the Slave PWM (PWM1)."]
    #[inline(always)]
    pub fn mdis(&self) -> MDIS_R {
        MDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCR")
            .field("ce", &format_args!("{}", self.ce().bit()))
            .field("cr", &format_args!("{}", self.cr().bit()))
            .field("pwmen", &format_args!("{}", self.pwmen().bit()))
            .field("mdis", &format_args!("{}", self.mdis().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<TCR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Counter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ce(&mut self) -> CE_W<TCR_SPEC, 0> {
        CE_W::new(self)
    }
    #[doc = "Bit 1 - Counter Reset"]
    #[inline(always)]
    #[must_use]
    pub fn cr(&mut self) -> CR_W<TCR_SPEC, 1> {
        CR_W::new(self)
    }
    #[doc = "Bit 3 - PWM Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwmen(&mut self) -> PWMEN_W<TCR_SPEC, 3> {
        PWMEN_W::new(self)
    }
    #[doc = "Bit 4 - Master Disable (PWM0 only). The two PWMs may be synchronized using the Master Disable control bit. The Master disable bit of the Master PWM (PWM0 module) controls a secondary enable input to both PWMs, as shown in Figure 141. This bit has no function in the Slave PWM (PWM1)."]
    #[inline(always)]
    #[must_use]
    pub fn mdis(&mut self) -> MDIS_W<TCR_SPEC, 4> {
        MDIS_W::new(self)
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
#[doc = "Timer Control Register. The TCR is used to control the Timer Counter functions.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCR_SPEC;
impl crate::RegisterSpec for TCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcr::R`](R) reader structure"]
impl crate::Readable for TCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tcr::W`](W) writer structure"]
impl crate::Writable for TCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCR to value 0"]
impl crate::Resettable for TCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
