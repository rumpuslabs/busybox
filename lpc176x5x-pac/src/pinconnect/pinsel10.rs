#[doc = "Register `PINSEL10` reader"]
pub type R = crate::R<PINSEL10_SPEC>;
#[doc = "Register `PINSEL10` writer"]
pub type W = crate::W<PINSEL10_SPEC>;
#[doc = "Field `TPIUCTRL` reader - TPIU interface pins control."]
pub type TPIUCTRL_R = crate::BitReader<TPIUCTRL_A>;
#[doc = "TPIU interface pins control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TPIUCTRL_A {
    #[doc = "0: Disabled. TPIU interface is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. TPIU interface is enabled. TPIU signals are available on the pins hosting them regardless of the PINSEL4 content."]
    ENABLED = 1,
}
impl From<TPIUCTRL_A> for bool {
    #[inline(always)]
    fn from(variant: TPIUCTRL_A) -> Self {
        variant as u8 != 0
    }
}
impl TPIUCTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TPIUCTRL_A {
        match self.bits {
            false => TPIUCTRL_A::DISABLED,
            true => TPIUCTRL_A::ENABLED,
        }
    }
    #[doc = "Disabled. TPIU interface is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TPIUCTRL_A::DISABLED
    }
    #[doc = "Enabled. TPIU interface is enabled. TPIU signals are available on the pins hosting them regardless of the PINSEL4 content."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TPIUCTRL_A::ENABLED
    }
}
#[doc = "Field `TPIUCTRL` writer - TPIU interface pins control."]
pub type TPIUCTRL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TPIUCTRL_A>;
impl<'a, REG, const O: u8> TPIUCTRL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. TPIU interface is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TPIUCTRL_A::DISABLED)
    }
    #[doc = "Enabled. TPIU interface is enabled. TPIU signals are available on the pins hosting them regardless of the PINSEL4 content."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TPIUCTRL_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 3 - TPIU interface pins control."]
    #[inline(always)]
    pub fn tpiuctrl(&self) -> TPIUCTRL_R {
        TPIUCTRL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PINSEL10")
            .field("tpiuctrl", &format_args!("{}", self.tpiuctrl().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<PINSEL10_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 3 - TPIU interface pins control."]
    #[inline(always)]
    #[must_use]
    pub fn tpiuctrl(&mut self) -> TPIUCTRL_W<PINSEL10_SPEC, 3> {
        TPIUCTRL_W::new(self)
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
#[doc = "Pin function select register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinsel10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinsel10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PINSEL10_SPEC;
impl crate::RegisterSpec for PINSEL10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinsel10::R`](R) reader structure"]
impl crate::Readable for PINSEL10_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pinsel10::W`](W) writer structure"]
impl crate::Writable for PINSEL10_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINSEL10 to value 0"]
impl crate::Resettable for PINSEL10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
