#[doc = "Register `CTCR` reader"]
pub type R = crate::R<CTCR_SPEC>;
#[doc = "Register `CTCR` writer"]
pub type W = crate::W<CTCR_SPEC>;
#[doc = "Field `MOD` reader - Counter/ Timer Mode"]
pub type MOD_R = crate::FieldReader<MOD_A>;
#[doc = "Counter/ Timer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MOD_A {
    #[doc = "0: Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale register."]
    TIMER_MODE = 0,
    #[doc = "1: Rising edge counter Mode: the TC is incremented on rising edges of the PWM_CAP input selected by bits 3:2."]
    RISING_EDGE_COUNTER = 1,
    #[doc = "2: Falling edge counter Mode: the TC is incremented on falling edges of the PWM_CAP input selected by bits 3:2."]
    FALLING_EDGE_COUNTER = 2,
    #[doc = "3: Dual edge counter Mode: the TC is incremented on both edges of the PWM_CAP input selected by bits 3:2."]
    DUAL_EDGE_COUNTER = 3,
}
impl From<MOD_A> for u8 {
    #[inline(always)]
    fn from(variant: MOD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MOD_A {
    type Ux = u8;
}
impl MOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MOD_A {
        match self.bits {
            0 => MOD_A::TIMER_MODE,
            1 => MOD_A::RISING_EDGE_COUNTER,
            2 => MOD_A::FALLING_EDGE_COUNTER,
            3 => MOD_A::DUAL_EDGE_COUNTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale register."]
    #[inline(always)]
    pub fn is_timer_mode(&self) -> bool {
        *self == MOD_A::TIMER_MODE
    }
    #[doc = "Rising edge counter Mode: the TC is incremented on rising edges of the PWM_CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn is_rising_edge_counter(&self) -> bool {
        *self == MOD_A::RISING_EDGE_COUNTER
    }
    #[doc = "Falling edge counter Mode: the TC is incremented on falling edges of the PWM_CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn is_falling_edge_counter(&self) -> bool {
        *self == MOD_A::FALLING_EDGE_COUNTER
    }
    #[doc = "Dual edge counter Mode: the TC is incremented on both edges of the PWM_CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn is_dual_edge_counter(&self) -> bool {
        *self == MOD_A::DUAL_EDGE_COUNTER
    }
}
#[doc = "Field `MOD` writer - Counter/ Timer Mode"]
pub type MOD_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, MOD_A>;
impl<'a, REG, const O: u8> MOD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale register."]
    #[inline(always)]
    pub fn timer_mode(self) -> &'a mut crate::W<REG> {
        self.variant(MOD_A::TIMER_MODE)
    }
    #[doc = "Rising edge counter Mode: the TC is incremented on rising edges of the PWM_CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn rising_edge_counter(self) -> &'a mut crate::W<REG> {
        self.variant(MOD_A::RISING_EDGE_COUNTER)
    }
    #[doc = "Falling edge counter Mode: the TC is incremented on falling edges of the PWM_CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn falling_edge_counter(self) -> &'a mut crate::W<REG> {
        self.variant(MOD_A::FALLING_EDGE_COUNTER)
    }
    #[doc = "Dual edge counter Mode: the TC is incremented on both edges of the PWM_CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn dual_edge_counter(self) -> &'a mut crate::W<REG> {
        self.variant(MOD_A::DUAL_EDGE_COUNTER)
    }
}
#[doc = "Field `CIS` reader - Count Input Select. When bits 1:0 are not 00, these bits select which PWM_CAP pin carries the signal used to increment the TC. Other combinations are reserved."]
pub type CIS_R = crate::FieldReader<CIS_A>;
#[doc = "Count Input Select. When bits 1:0 are not 00, these bits select which PWM_CAP pin carries the signal used to increment the TC. Other combinations are reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CIS_A {
    #[doc = "0: For PWM0: 00 = PWM0_CAP0 (Other combinations are reserved) For PWM1: 00 = PWM1_CAP0, 01 = PWM1_CAP1 (Other combinations are reserved)"]
    FOR_PWM0_00_EQ_PWM0 = 0,
}
impl From<CIS_A> for u8 {
    #[inline(always)]
    fn from(variant: CIS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CIS_A {
    type Ux = u8;
}
impl CIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CIS_A> {
        match self.bits {
            0 => Some(CIS_A::FOR_PWM0_00_EQ_PWM0),
            _ => None,
        }
    }
    #[doc = "For PWM0: 00 = PWM0_CAP0 (Other combinations are reserved) For PWM1: 00 = PWM1_CAP0, 01 = PWM1_CAP1 (Other combinations are reserved)"]
    #[inline(always)]
    pub fn is_for_pwm0_00_eq_pwm0(&self) -> bool {
        *self == CIS_A::FOR_PWM0_00_EQ_PWM0
    }
}
#[doc = "Field `CIS` writer - Count Input Select. When bits 1:0 are not 00, these bits select which PWM_CAP pin carries the signal used to increment the TC. Other combinations are reserved."]
pub type CIS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, CIS_A>;
impl<'a, REG, const O: u8> CIS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "For PWM0: 00 = PWM0_CAP0 (Other combinations are reserved) For PWM1: 00 = PWM1_CAP0, 01 = PWM1_CAP1 (Other combinations are reserved)"]
    #[inline(always)]
    pub fn for_pwm0_00_eq_pwm0(self) -> &'a mut crate::W<REG> {
        self.variant(CIS_A::FOR_PWM0_00_EQ_PWM0)
    }
}
impl R {
    #[doc = "Bits 0:1 - Counter/ Timer Mode"]
    #[inline(always)]
    pub fn mod_(&self) -> MOD_R {
        MOD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Count Input Select. When bits 1:0 are not 00, these bits select which PWM_CAP pin carries the signal used to increment the TC. Other combinations are reserved."]
    #[inline(always)]
    pub fn cis(&self) -> CIS_R {
        CIS_R::new(((self.bits >> 2) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTCR")
            .field("mod_", &format_args!("{}", self.mod_().bits()))
            .field("cis", &format_args!("{}", self.cis().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CTCR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Counter/ Timer Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mod_(&mut self) -> MOD_W<CTCR_SPEC, 0> {
        MOD_W::new(self)
    }
    #[doc = "Bits 2:3 - Count Input Select. When bits 1:0 are not 00, these bits select which PWM_CAP pin carries the signal used to increment the TC. Other combinations are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn cis(&mut self) -> CIS_W<CTCR_SPEC, 2> {
        CIS_W::new(self)
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
#[doc = "Count Control Register. The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTCR_SPEC;
impl crate::RegisterSpec for CTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctcr::R`](R) reader structure"]
impl crate::Readable for CTCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctcr::W`](W) writer structure"]
impl crate::Writable for CTCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTCR to value 0"]
impl crate::Resettable for CTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
