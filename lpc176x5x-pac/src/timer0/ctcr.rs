#[doc = "Register `CTCR` reader"]
pub type R = crate::R<CTCR_SPEC>;
#[doc = "Register `CTCR` writer"]
pub type W = crate::W<CTCR_SPEC>;
#[doc = "Field `CTMODE` reader - Counter/Timer Mode This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register."]
pub type CTMODE_R = crate::FieldReader<CTMODE_A>;
#[doc = "Counter/Timer Mode This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTMODE_A {
    #[doc = "0: Timer Mode: every rising PCLK edge"]
    TIMER_MODE = 0,
    #[doc = "1: Counter Mode: TC is incremented on rising edges on the CAP input selected by bits 3:2."]
    RISING = 1,
    #[doc = "2: Counter Mode: TC is incremented on falling edges on the CAP input selected by bits 3:2."]
    FALLING = 2,
    #[doc = "3: Counter Mode: TC is incremented on both edges on the CAP input selected by bits 3:2."]
    DUALEDGE = 3,
}
impl From<CTMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CTMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CTMODE_A {
    type Ux = u8;
}
impl CTMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CTMODE_A {
        match self.bits {
            0 => CTMODE_A::TIMER_MODE,
            1 => CTMODE_A::RISING,
            2 => CTMODE_A::FALLING,
            3 => CTMODE_A::DUALEDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Timer Mode: every rising PCLK edge"]
    #[inline(always)]
    pub fn is_timer_mode(&self) -> bool {
        *self == CTMODE_A::TIMER_MODE
    }
    #[doc = "Counter Mode: TC is incremented on rising edges on the CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == CTMODE_A::RISING
    }
    #[doc = "Counter Mode: TC is incremented on falling edges on the CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == CTMODE_A::FALLING
    }
    #[doc = "Counter Mode: TC is incremented on both edges on the CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn is_dualedge(&self) -> bool {
        *self == CTMODE_A::DUALEDGE
    }
}
#[doc = "Field `CTMODE` writer - Counter/Timer Mode This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register."]
pub type CTMODE_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, CTMODE_A>;
impl<'a, REG, const O: u8> CTMODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer Mode: every rising PCLK edge"]
    #[inline(always)]
    pub fn timer_mode(self) -> &'a mut crate::W<REG> {
        self.variant(CTMODE_A::TIMER_MODE)
    }
    #[doc = "Counter Mode: TC is incremented on rising edges on the CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(CTMODE_A::RISING)
    }
    #[doc = "Counter Mode: TC is incremented on falling edges on the CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(CTMODE_A::FALLING)
    }
    #[doc = "Counter Mode: TC is incremented on both edges on the CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn dualedge(self) -> &'a mut crate::W<REG> {
        self.variant(CTMODE_A::DUALEDGE)
    }
}
#[doc = "Field `CINSEL` reader - Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the TnCTCR, the 3 bits for that input in the Capture Control Register (TnCCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer."]
pub type CINSEL_R = crate::FieldReader<CINSEL_A>;
#[doc = "Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the TnCTCR, the 3 bits for that input in the Capture Control Register (TnCCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CINSEL_A {
    #[doc = "0: CAPn.0 for TIMERn"]
    CAPN_0 = 0,
    #[doc = "1: CAPn.1 for TIMERn"]
    CAPN_1 = 1,
}
impl From<CINSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CINSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CINSEL_A {
    type Ux = u8;
}
impl CINSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CINSEL_A> {
        match self.bits {
            0 => Some(CINSEL_A::CAPN_0),
            1 => Some(CINSEL_A::CAPN_1),
            _ => None,
        }
    }
    #[doc = "CAPn.0 for TIMERn"]
    #[inline(always)]
    pub fn is_capn_0(&self) -> bool {
        *self == CINSEL_A::CAPN_0
    }
    #[doc = "CAPn.1 for TIMERn"]
    #[inline(always)]
    pub fn is_capn_1(&self) -> bool {
        *self == CINSEL_A::CAPN_1
    }
}
#[doc = "Field `CINSEL` writer - Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the TnCTCR, the 3 bits for that input in the Capture Control Register (TnCCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer."]
pub type CINSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, CINSEL_A>;
impl<'a, REG, const O: u8> CINSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CAPn.0 for TIMERn"]
    #[inline(always)]
    pub fn capn_0(self) -> &'a mut crate::W<REG> {
        self.variant(CINSEL_A::CAPN_0)
    }
    #[doc = "CAPn.1 for TIMERn"]
    #[inline(always)]
    pub fn capn_1(self) -> &'a mut crate::W<REG> {
        self.variant(CINSEL_A::CAPN_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Counter/Timer Mode This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register."]
    #[inline(always)]
    pub fn ctmode(&self) -> CTMODE_R {
        CTMODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the TnCTCR, the 3 bits for that input in the Capture Control Register (TnCCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer."]
    #[inline(always)]
    pub fn cinsel(&self) -> CINSEL_R {
        CINSEL_R::new(((self.bits >> 2) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTCR")
            .field("ctmode", &format_args!("{}", self.ctmode().bits()))
            .field("cinsel", &format_args!("{}", self.cinsel().bits()))
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
    #[doc = "Bits 0:1 - Counter/Timer Mode This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register."]
    #[inline(always)]
    #[must_use]
    pub fn ctmode(&mut self) -> CTMODE_W<CTCR_SPEC, 0> {
        CTMODE_W::new(self)
    }
    #[doc = "Bits 2:3 - Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the TnCTCR, the 3 bits for that input in the Capture Control Register (TnCCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer."]
    #[inline(always)]
    #[must_use]
    pub fn cinsel(&mut self) -> CINSEL_W<CTCR_SPEC, 2> {
        CINSEL_W::new(self)
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
