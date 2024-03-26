#[doc = "Register `CLKSEL` reader"]
pub type R = crate::R<CLKSEL_SPEC>;
#[doc = "Register `CLKSEL` writer"]
pub type W = crate::W<CLKSEL_SPEC>;
#[doc = "Field `CLKSEL` reader - Selects source of WDT clock"]
pub type CLKSEL_R = crate::FieldReader<CLKSEL_A>;
#[doc = "Selects source of WDT clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: IRC"]
    IRC = 0,
    #[doc = "1: Peripheral clock"]
    PCLK = 1,
    #[doc = "2: RTC oscillator"]
    RTCOSC = 2,
}
impl From<CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLKSEL_A {
    type Ux = u8;
}
impl CLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLKSEL_A {
        match self.bits {
            0 => CLKSEL_A::IRC,
            1 => CLKSEL_A::PCLK,
            2 => CLKSEL_A::RTCOSC,
            _ => unreachable!(),
        }
    }
    #[doc = "IRC"]
    #[inline(always)]
    pub fn is_irc(&self) -> bool {
        *self == CLKSEL_A::IRC
    }
    #[doc = "Peripheral clock"]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == CLKSEL_A::PCLK
    }
    #[doc = "RTC oscillator"]
    #[inline(always)]
    pub fn is_rtcosc(&self) -> bool {
        *self == CLKSEL_A::RTCOSC
    }
}
#[doc = "Field `CLKSEL` writer - Selects source of WDT clock"]
pub type CLKSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, CLKSEL_A>;
impl<'a, REG, const O: u8> CLKSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IRC"]
    #[inline(always)]
    pub fn irc(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::IRC)
    }
    #[doc = "Peripheral clock"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::PCLK)
    }
    #[doc = "RTC oscillator"]
    #[inline(always)]
    pub fn rtcosc(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL_A::RTCOSC)
    }
}
#[doc = "Field `LOCK` reader - If this bit is set to one writing to this register does not affect bit 0. The clock source can only be changed by first clearing this bit, then writing the new value of bit 0."]
pub type LOCK_R = crate::BitReader<LOCK_A>;
#[doc = "If this bit is set to one writing to this register does not affect bit 0. The clock source can only be changed by first clearing this bit, then writing the new value of bit 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK_A {
    #[doc = "0: This bit is set to 0 on any reset. It cannot be cleared by software."]
    UNLOCKED = 0,
    #[doc = "1: Software can set this bit to 1 at any time. Once WDLOCK is set, the bits of this register cannot be modified."]
    LOCKED = 1,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LOCK_A {
        match self.bits {
            false => LOCK_A::UNLOCKED,
            true => LOCK_A::LOCKED,
        }
    }
    #[doc = "This bit is set to 0 on any reset. It cannot be cleared by software."]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCK_A::UNLOCKED
    }
    #[doc = "Software can set this bit to 1 at any time. Once WDLOCK is set, the bits of this register cannot be modified."]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCK_A::LOCKED
    }
}
#[doc = "Field `LOCK` writer - If this bit is set to one writing to this register does not affect bit 0. The clock source can only be changed by first clearing this bit, then writing the new value of bit 0."]
pub type LOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LOCK_A>;
impl<'a, REG, const O: u8> LOCK_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This bit is set to 0 on any reset. It cannot be cleared by software."]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK_A::UNLOCKED)
    }
    #[doc = "Software can set this bit to 1 at any time. Once WDLOCK is set, the bits of this register cannot be modified."]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK_A::LOCKED)
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects source of WDT clock"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 31 - If this bit is set to one writing to this register does not affect bit 0. The clock source can only be changed by first clearing this bit, then writing the new value of bit 0."]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKSEL")
            .field("clksel", &format_args!("{}", self.clksel().bits()))
            .field("lock", &format_args!("{}", self.lock().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CLKSEL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects source of WDT clock"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<CLKSEL_SPEC, 0> {
        CLKSEL_W::new(self)
    }
    #[doc = "Bit 31 - If this bit is set to one writing to this register does not affect bit 0. The clock source can only be changed by first clearing this bit, then writing the new value of bit 0."]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<CLKSEL_SPEC, 31> {
        LOCK_W::new(self)
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
#[doc = "Watchdog clock select register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKSEL_SPEC;
impl crate::RegisterSpec for CLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clksel::R`](R) reader structure"]
impl crate::Readable for CLKSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clksel::W`](W) writer structure"]
impl crate::Writable for CLKSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKSEL to value 0"]
impl crate::Resettable for CLKSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
