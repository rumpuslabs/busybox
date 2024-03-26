#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `RITINT` reader - Interrupt flag"]
pub type RITINT_R = crate::BitReader<RITINT_A>;
#[doc = "Interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RITINT_A {
    #[doc = "1: This bit is set to 1 by hardware whenever the counter value equals the masked compare value specified by the contents of RICOMPVAL and RIMASK registers. Writing a 1 to this bit will clear it to 0. Writing a 0 has no effect."]
    PENDING = 1,
    #[doc = "0: The counter value does not equal the masked compare value."]
    NOT_PENDING = 0,
}
impl From<RITINT_A> for bool {
    #[inline(always)]
    fn from(variant: RITINT_A) -> Self {
        variant as u8 != 0
    }
}
impl RITINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RITINT_A {
        match self.bits {
            true => RITINT_A::PENDING,
            false => RITINT_A::NOT_PENDING,
        }
    }
    #[doc = "This bit is set to 1 by hardware whenever the counter value equals the masked compare value specified by the contents of RICOMPVAL and RIMASK registers. Writing a 1 to this bit will clear it to 0. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RITINT_A::PENDING
    }
    #[doc = "The counter value does not equal the masked compare value."]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == RITINT_A::NOT_PENDING
    }
}
#[doc = "Field `RITINT` writer - Interrupt flag"]
pub type RITINT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RITINT_A>;
impl<'a, REG, const O: u8> RITINT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This bit is set to 1 by hardware whenever the counter value equals the masked compare value specified by the contents of RICOMPVAL and RIMASK registers. Writing a 1 to this bit will clear it to 0. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(RITINT_A::PENDING)
    }
    #[doc = "The counter value does not equal the masked compare value."]
    #[inline(always)]
    pub fn not_pending(self) -> &'a mut crate::W<REG> {
        self.variant(RITINT_A::NOT_PENDING)
    }
}
#[doc = "Field `RITENCLR` reader - Timer enable clear"]
pub type RITENCLR_R = crate::BitReader<RITENCLR_A>;
#[doc = "Timer enable clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RITENCLR_A {
    #[doc = "1: The timer will be cleared to 0 whenever the counter value equals the masked compare value specified by the contents of RICOMPVAL and RIMASK registers. This will occur on the same clock that sets the interrupt flag."]
    CLEAR = 1,
    #[doc = "0: The timer will not be cleared to 0."]
    NO_ACTION = 0,
}
impl From<RITENCLR_A> for bool {
    #[inline(always)]
    fn from(variant: RITENCLR_A) -> Self {
        variant as u8 != 0
    }
}
impl RITENCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RITENCLR_A {
        match self.bits {
            true => RITENCLR_A::CLEAR,
            false => RITENCLR_A::NO_ACTION,
        }
    }
    #[doc = "The timer will be cleared to 0 whenever the counter value equals the masked compare value specified by the contents of RICOMPVAL and RIMASK registers. This will occur on the same clock that sets the interrupt flag."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RITENCLR_A::CLEAR
    }
    #[doc = "The timer will not be cleared to 0."]
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == RITENCLR_A::NO_ACTION
    }
}
#[doc = "Field `RITENCLR` writer - Timer enable clear"]
pub type RITENCLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RITENCLR_A>;
impl<'a, REG, const O: u8> RITENCLR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The timer will be cleared to 0 whenever the counter value equals the masked compare value specified by the contents of RICOMPVAL and RIMASK registers. This will occur on the same clock that sets the interrupt flag."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RITENCLR_A::CLEAR)
    }
    #[doc = "The timer will not be cleared to 0."]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(RITENCLR_A::NO_ACTION)
    }
}
#[doc = "Field `RITENBR` reader - Timer enable for debug"]
pub type RITENBR_R = crate::BitReader<RITENBR_A>;
#[doc = "Timer enable for debug\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RITENBR_A {
    #[doc = "1: The timer is halted when the processor is halted for debugging."]
    HALTED = 1,
    #[doc = "0: Debug has no effect on the timer operation."]
    NO_EFFECT = 0,
}
impl From<RITENBR_A> for bool {
    #[inline(always)]
    fn from(variant: RITENBR_A) -> Self {
        variant as u8 != 0
    }
}
impl RITENBR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RITENBR_A {
        match self.bits {
            true => RITENBR_A::HALTED,
            false => RITENBR_A::NO_EFFECT,
        }
    }
    #[doc = "The timer is halted when the processor is halted for debugging."]
    #[inline(always)]
    pub fn is_halted(&self) -> bool {
        *self == RITENBR_A::HALTED
    }
    #[doc = "Debug has no effect on the timer operation."]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == RITENBR_A::NO_EFFECT
    }
}
#[doc = "Field `RITENBR` writer - Timer enable for debug"]
pub type RITENBR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RITENBR_A>;
impl<'a, REG, const O: u8> RITENBR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The timer is halted when the processor is halted for debugging."]
    #[inline(always)]
    pub fn halted(self) -> &'a mut crate::W<REG> {
        self.variant(RITENBR_A::HALTED)
    }
    #[doc = "Debug has no effect on the timer operation."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(RITENBR_A::NO_EFFECT)
    }
}
#[doc = "Field `RITEN` reader - Timer enable."]
pub type RITEN_R = crate::BitReader<RITEN_A>;
#[doc = "Timer enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RITEN_A {
    #[doc = "1: Timer enabled. This can be overruled by a debug halt if enabled in bit 2."]
    ENABLED = 1,
    #[doc = "0: Timer disabled."]
    DISABLED = 0,
}
impl From<RITEN_A> for bool {
    #[inline(always)]
    fn from(variant: RITEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RITEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RITEN_A {
        match self.bits {
            true => RITEN_A::ENABLED,
            false => RITEN_A::DISABLED,
        }
    }
    #[doc = "Timer enabled. This can be overruled by a debug halt if enabled in bit 2."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RITEN_A::ENABLED
    }
    #[doc = "Timer disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RITEN_A::DISABLED
    }
}
#[doc = "Field `RITEN` writer - Timer enable."]
pub type RITEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RITEN_A>;
impl<'a, REG, const O: u8> RITEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer enabled. This can be overruled by a debug halt if enabled in bit 2."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RITEN_A::ENABLED)
    }
    #[doc = "Timer disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RITEN_A::DISABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt flag"]
    #[inline(always)]
    pub fn ritint(&self) -> RITINT_R {
        RITINT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer enable clear"]
    #[inline(always)]
    pub fn ritenclr(&self) -> RITENCLR_R {
        RITENCLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer enable for debug"]
    #[inline(always)]
    pub fn ritenbr(&self) -> RITENBR_R {
        RITENBR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer enable."]
    #[inline(always)]
    pub fn riten(&self) -> RITEN_R {
        RITEN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("ritint", &format_args!("{}", self.ritint().bit()))
            .field("ritenclr", &format_args!("{}", self.ritenclr().bit()))
            .field("ritenbr", &format_args!("{}", self.ritenbr().bit()))
            .field("riten", &format_args!("{}", self.riten().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ritint(&mut self) -> RITINT_W<CTRL_SPEC, 0> {
        RITINT_W::new(self)
    }
    #[doc = "Bit 1 - Timer enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn ritenclr(&mut self) -> RITENCLR_W<CTRL_SPEC, 1> {
        RITENCLR_W::new(self)
    }
    #[doc = "Bit 2 - Timer enable for debug"]
    #[inline(always)]
    #[must_use]
    pub fn ritenbr(&mut self) -> RITENBR_W<CTRL_SPEC, 2> {
        RITENBR_W::new(self)
    }
    #[doc = "Bit 3 - Timer enable."]
    #[inline(always)]
    #[must_use]
    pub fn riten(&mut self) -> RITEN_W<CTRL_SPEC, 3> {
        RITEN_W::new(self)
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
#[doc = "Control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x0c"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0c;
}
