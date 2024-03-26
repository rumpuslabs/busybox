#[doc = "Register `MOD` reader"]
pub type R = crate::R<MOD_SPEC>;
#[doc = "Register `MOD` writer"]
pub type W = crate::W<MOD_SPEC>;
#[doc = "Field `WDEN` reader - Watchdog enable bit. This bit is Set Only."]
pub type WDEN_R = crate::BitReader<WDEN_A>;
#[doc = "Watchdog enable bit. This bit is Set Only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDEN_A {
    #[doc = "0: The watchdog timer is stopped."]
    STOP = 0,
    #[doc = "1: The watchdog timer is running."]
    RUN = 1,
}
impl From<WDEN_A> for bool {
    #[inline(always)]
    fn from(variant: WDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl WDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WDEN_A {
        match self.bits {
            false => WDEN_A::STOP,
            true => WDEN_A::RUN,
        }
    }
    #[doc = "The watchdog timer is stopped."]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == WDEN_A::STOP
    }
    #[doc = "The watchdog timer is running."]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == WDEN_A::RUN
    }
}
#[doc = "Field `WDEN` writer - Watchdog enable bit. This bit is Set Only."]
pub type WDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WDEN_A>;
impl<'a, REG, const O: u8> WDEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The watchdog timer is stopped."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(WDEN_A::STOP)
    }
    #[doc = "The watchdog timer is running."]
    #[inline(always)]
    pub fn run(self) -> &'a mut crate::W<REG> {
        self.variant(WDEN_A::RUN)
    }
}
#[doc = "Field `WDRESET` reader - Watchdog reset enable bit. This bit is Set Only. See Table 652."]
pub type WDRESET_R = crate::BitReader<WDRESET_A>;
#[doc = "Watchdog reset enable bit. This bit is Set Only. See Table 652.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDRESET_A {
    #[doc = "0: A watchdog timeout will not cause a chip reset."]
    NORESET = 0,
    #[doc = "1: A watchdog timeout will cause a chip reset."]
    RESET = 1,
}
impl From<WDRESET_A> for bool {
    #[inline(always)]
    fn from(variant: WDRESET_A) -> Self {
        variant as u8 != 0
    }
}
impl WDRESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WDRESET_A {
        match self.bits {
            false => WDRESET_A::NORESET,
            true => WDRESET_A::RESET,
        }
    }
    #[doc = "A watchdog timeout will not cause a chip reset."]
    #[inline(always)]
    pub fn is_noreset(&self) -> bool {
        *self == WDRESET_A::NORESET
    }
    #[doc = "A watchdog timeout will cause a chip reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == WDRESET_A::RESET
    }
}
#[doc = "Field `WDRESET` writer - Watchdog reset enable bit. This bit is Set Only. See Table 652."]
pub type WDRESET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WDRESET_A>;
impl<'a, REG, const O: u8> WDRESET_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A watchdog timeout will not cause a chip reset."]
    #[inline(always)]
    pub fn noreset(self) -> &'a mut crate::W<REG> {
        self.variant(WDRESET_A::NORESET)
    }
    #[doc = "A watchdog timeout will cause a chip reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(WDRESET_A::RESET)
    }
}
#[doc = "Field `WDTOF` reader - Watchdog time-out flag. Set when the watchdog timer times out, cleared by software."]
pub type WDTOF_R = crate::BitReader;
#[doc = "Field `WDTOF` writer - Watchdog time-out flag. Set when the watchdog timer times out, cleared by software."]
pub type WDTOF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WDINT` reader - Watchdog interrupt flag. Cleared by software."]
pub type WDINT_R = crate::BitReader;
#[doc = "Field `WDINT` writer - Watchdog interrupt flag. Cleared by software."]
pub type WDINT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Watchdog enable bit. This bit is Set Only."]
    #[inline(always)]
    pub fn wden(&self) -> WDEN_R {
        WDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog reset enable bit. This bit is Set Only. See Table 652."]
    #[inline(always)]
    pub fn wdreset(&self) -> WDRESET_R {
        WDRESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Watchdog time-out flag. Set when the watchdog timer times out, cleared by software."]
    #[inline(always)]
    pub fn wdtof(&self) -> WDTOF_R {
        WDTOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Watchdog interrupt flag. Cleared by software."]
    #[inline(always)]
    pub fn wdint(&self) -> WDINT_R {
        WDINT_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MOD")
            .field("wden", &format_args!("{}", self.wden().bit()))
            .field("wdreset", &format_args!("{}", self.wdreset().bit()))
            .field("wdtof", &format_args!("{}", self.wdtof().bit()))
            .field("wdint", &format_args!("{}", self.wdint().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<MOD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog enable bit. This bit is Set Only."]
    #[inline(always)]
    #[must_use]
    pub fn wden(&mut self) -> WDEN_W<MOD_SPEC, 0> {
        WDEN_W::new(self)
    }
    #[doc = "Bit 1 - Watchdog reset enable bit. This bit is Set Only. See Table 652."]
    #[inline(always)]
    #[must_use]
    pub fn wdreset(&mut self) -> WDRESET_W<MOD_SPEC, 1> {
        WDRESET_W::new(self)
    }
    #[doc = "Bit 2 - Watchdog time-out flag. Set when the watchdog timer times out, cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn wdtof(&mut self) -> WDTOF_W<MOD_SPEC, 2> {
        WDTOF_W::new(self)
    }
    #[doc = "Bit 3 - Watchdog interrupt flag. Cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn wdint(&mut self) -> WDINT_W<MOD_SPEC, 3> {
        WDINT_W::new(self)
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
#[doc = "Watchdog mode register. This register determines the basic mode and status of the Watchdog Timer.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mod_::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mod_::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MOD_SPEC;
impl crate::RegisterSpec for MOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mod_::R`](R) reader structure"]
impl crate::Readable for MOD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mod_::W`](W) writer structure"]
impl crate::Writable for MOD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MOD to value 0"]
impl crate::Resettable for MOD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
