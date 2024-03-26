#[doc = "Register `ACR` reader"]
pub type R = crate::R<ACR_SPEC>;
#[doc = "Register `ACR` writer"]
pub type W = crate::W<ACR_SPEC>;
#[doc = "Field `START` reader - Start bit. This bit is automatically cleared after auto-baud completion."]
pub type START_R = crate::BitReader<START_A>;
#[doc = "Start bit. This bit is automatically cleared after auto-baud completion.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum START_A {
    #[doc = "0: Auto-baud stop (auto-baud is not running)."]
    STOP = 0,
    #[doc = "1: Auto-baud start (auto-baud is running). Auto-baud run bit. This bit is automatically cleared after auto-baud completion."]
    START = 1,
}
impl From<START_A> for bool {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as u8 != 0
    }
}
impl START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> START_A {
        match self.bits {
            false => START_A::STOP,
            true => START_A::START,
        }
    }
    #[doc = "Auto-baud stop (auto-baud is not running)."]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == START_A::STOP
    }
    #[doc = "Auto-baud start (auto-baud is running). Auto-baud run bit. This bit is automatically cleared after auto-baud completion."]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == START_A::START
    }
}
#[doc = "Field `START` writer - Start bit. This bit is automatically cleared after auto-baud completion."]
pub type START_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, START_A>;
impl<'a, REG, const O: u8> START_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Auto-baud stop (auto-baud is not running)."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(START_A::STOP)
    }
    #[doc = "Auto-baud start (auto-baud is running). Auto-baud run bit. This bit is automatically cleared after auto-baud completion."]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(START_A::START)
    }
}
#[doc = "Field `MODE` reader - Auto-baud mode select bit."]
pub type MODE_R = crate::BitReader<MODE_A>;
#[doc = "Auto-baud mode select bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODE_A {
    #[doc = "0: Mode 0."]
    MODE_0 = 0,
    #[doc = "1: Mode 1."]
    MODE_1 = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::MODE_0,
            true => MODE_A::MODE_1,
        }
    }
    #[doc = "Mode 0."]
    #[inline(always)]
    pub fn is_mode_0(&self) -> bool {
        *self == MODE_A::MODE_0
    }
    #[doc = "Mode 1."]
    #[inline(always)]
    pub fn is_mode_1(&self) -> bool {
        *self == MODE_A::MODE_1
    }
}
#[doc = "Field `MODE` writer - Auto-baud mode select bit."]
pub type MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MODE_A>;
impl<'a, REG, const O: u8> MODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Mode 0."]
    #[inline(always)]
    pub fn mode_0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::MODE_0)
    }
    #[doc = "Mode 1."]
    #[inline(always)]
    pub fn mode_1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::MODE_1)
    }
}
#[doc = "Field `AUTORESTART` reader - Restart bit."]
pub type AUTORESTART_R = crate::BitReader<AUTORESTART_A>;
#[doc = "Restart bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTORESTART_A {
    #[doc = "0: No restart."]
    NO_RESTART = 0,
    #[doc = "1: Restart in case of time-out (counter restarts at next UARTn Rx falling edge)"]
    RESTART = 1,
}
impl From<AUTORESTART_A> for bool {
    #[inline(always)]
    fn from(variant: AUTORESTART_A) -> Self {
        variant as u8 != 0
    }
}
impl AUTORESTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AUTORESTART_A {
        match self.bits {
            false => AUTORESTART_A::NO_RESTART,
            true => AUTORESTART_A::RESTART,
        }
    }
    #[doc = "No restart."]
    #[inline(always)]
    pub fn is_no_restart(&self) -> bool {
        *self == AUTORESTART_A::NO_RESTART
    }
    #[doc = "Restart in case of time-out (counter restarts at next UARTn Rx falling edge)"]
    #[inline(always)]
    pub fn is_restart(&self) -> bool {
        *self == AUTORESTART_A::RESTART
    }
}
#[doc = "Field `AUTORESTART` writer - Restart bit."]
pub type AUTORESTART_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, AUTORESTART_A>;
impl<'a, REG, const O: u8> AUTORESTART_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No restart."]
    #[inline(always)]
    pub fn no_restart(self) -> &'a mut crate::W<REG> {
        self.variant(AUTORESTART_A::NO_RESTART)
    }
    #[doc = "Restart in case of time-out (counter restarts at next UARTn Rx falling edge)"]
    #[inline(always)]
    pub fn restart(self) -> &'a mut crate::W<REG> {
        self.variant(AUTORESTART_A::RESTART)
    }
}
#[doc = "Field `ABEOINTCLR` reader - End of auto-baud interrupt clear bit (write-only accessible). Writing a 1 will clear the corresponding interrupt in the UnIIR. Writing a 0 has no impact."]
pub type ABEOINTCLR_R = crate::BitReader<ABEOINTCLR_A>;
#[doc = "End of auto-baud interrupt clear bit (write-only accessible). Writing a 1 will clear the corresponding interrupt in the UnIIR. Writing a 0 has no impact.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABEOINTCLR_A {
    #[doc = "0: No impact."]
    NO_ACTION = 0,
    #[doc = "1: Clear the corresponding interrupt in the IIR."]
    CLEAR = 1,
}
impl From<ABEOINTCLR_A> for bool {
    #[inline(always)]
    fn from(variant: ABEOINTCLR_A) -> Self {
        variant as u8 != 0
    }
}
impl ABEOINTCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ABEOINTCLR_A {
        match self.bits {
            false => ABEOINTCLR_A::NO_ACTION,
            true => ABEOINTCLR_A::CLEAR,
        }
    }
    #[doc = "No impact."]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == ABEOINTCLR_A::NO_ACTION
    }
    #[doc = "Clear the corresponding interrupt in the IIR."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == ABEOINTCLR_A::CLEAR
    }
}
#[doc = "Field `ABEOINTCLR` writer - End of auto-baud interrupt clear bit (write-only accessible). Writing a 1 will clear the corresponding interrupt in the UnIIR. Writing a 0 has no impact."]
pub type ABEOINTCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ABEOINTCLR_A>;
impl<'a, REG, const O: u8> ABEOINTCLR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No impact."]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(ABEOINTCLR_A::NO_ACTION)
    }
    #[doc = "Clear the corresponding interrupt in the IIR."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ABEOINTCLR_A::CLEAR)
    }
}
#[doc = "Field `ABTOINTCLR` reader - Auto-baud time-out interrupt clear bit (write-only accessible). Writing a 1 will clear the corresponding interrupt in the UnIIR. Writing a 0 has no impact."]
pub type ABTOINTCLR_R = crate::BitReader<ABTOINTCLR_A>;
#[doc = "Auto-baud time-out interrupt clear bit (write-only accessible). Writing a 1 will clear the corresponding interrupt in the UnIIR. Writing a 0 has no impact.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABTOINTCLR_A {
    #[doc = "0: No impact."]
    NO_ACTION = 0,
    #[doc = "1: Clear the corresponding interrupt in the IIR."]
    CLEAR = 1,
}
impl From<ABTOINTCLR_A> for bool {
    #[inline(always)]
    fn from(variant: ABTOINTCLR_A) -> Self {
        variant as u8 != 0
    }
}
impl ABTOINTCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ABTOINTCLR_A {
        match self.bits {
            false => ABTOINTCLR_A::NO_ACTION,
            true => ABTOINTCLR_A::CLEAR,
        }
    }
    #[doc = "No impact."]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == ABTOINTCLR_A::NO_ACTION
    }
    #[doc = "Clear the corresponding interrupt in the IIR."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == ABTOINTCLR_A::CLEAR
    }
}
#[doc = "Field `ABTOINTCLR` writer - Auto-baud time-out interrupt clear bit (write-only accessible). Writing a 1 will clear the corresponding interrupt in the UnIIR. Writing a 0 has no impact."]
pub type ABTOINTCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ABTOINTCLR_A>;
impl<'a, REG, const O: u8> ABTOINTCLR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No impact."]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(ABTOINTCLR_A::NO_ACTION)
    }
    #[doc = "Clear the corresponding interrupt in the IIR."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ABTOINTCLR_A::CLEAR)
    }
}
impl R {
    #[doc = "Bit 0 - Start bit. This bit is automatically cleared after auto-baud completion."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Auto-baud mode select bit."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Restart bit."]
    #[inline(always)]
    pub fn autorestart(&self) -> AUTORESTART_R {
        AUTORESTART_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - End of auto-baud interrupt clear bit (write-only accessible). Writing a 1 will clear the corresponding interrupt in the UnIIR. Writing a 0 has no impact."]
    #[inline(always)]
    pub fn abeointclr(&self) -> ABEOINTCLR_R {
        ABEOINTCLR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Auto-baud time-out interrupt clear bit (write-only accessible). Writing a 1 will clear the corresponding interrupt in the UnIIR. Writing a 0 has no impact."]
    #[inline(always)]
    pub fn abtointclr(&self) -> ABTOINTCLR_R {
        ABTOINTCLR_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACR")
            .field("start", &format_args!("{}", self.start().bit()))
            .field("mode", &format_args!("{}", self.mode().bit()))
            .field("autorestart", &format_args!("{}", self.autorestart().bit()))
            .field("abeointclr", &format_args!("{}", self.abeointclr().bit()))
            .field("abtointclr", &format_args!("{}", self.abtointclr().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<ACR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Start bit. This bit is automatically cleared after auto-baud completion."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<ACR_SPEC, 0> {
        START_W::new(self)
    }
    #[doc = "Bit 1 - Auto-baud mode select bit."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<ACR_SPEC, 1> {
        MODE_W::new(self)
    }
    #[doc = "Bit 2 - Restart bit."]
    #[inline(always)]
    #[must_use]
    pub fn autorestart(&mut self) -> AUTORESTART_W<ACR_SPEC, 2> {
        AUTORESTART_W::new(self)
    }
    #[doc = "Bit 8 - End of auto-baud interrupt clear bit (write-only accessible). Writing a 1 will clear the corresponding interrupt in the UnIIR. Writing a 0 has no impact."]
    #[inline(always)]
    #[must_use]
    pub fn abeointclr(&mut self) -> ABEOINTCLR_W<ACR_SPEC, 8> {
        ABEOINTCLR_W::new(self)
    }
    #[doc = "Bit 9 - Auto-baud time-out interrupt clear bit (write-only accessible). Writing a 1 will clear the corresponding interrupt in the UnIIR. Writing a 0 has no impact."]
    #[inline(always)]
    #[must_use]
    pub fn abtointclr(&mut self) -> ABTOINTCLR_W<ACR_SPEC, 9> {
        ABTOINTCLR_W::new(self)
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
#[doc = "Auto-baud Control Register. Contains controls for the auto-baud feature.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACR_SPEC;
impl crate::RegisterSpec for ACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acr::R`](R) reader structure"]
impl crate::Readable for ACR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`acr::W`](W) writer structure"]
impl crate::Writable for ACR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACR to value 0"]
impl crate::Resettable for ACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
