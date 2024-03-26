#[doc = "Register `CCR` reader"]
pub type R = crate::R<CCR_SPEC>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CCR_SPEC>;
#[doc = "Field `CLKEN` reader - Clock Enable."]
pub type CLKEN_R = crate::BitReader<CLKEN_A>;
#[doc = "Clock Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKEN_A {
    #[doc = "1: The time counters are enabled."]
    ENABLED = 1,
    #[doc = "0: The time counters are disabled so that they may be initialized."]
    DISABLED = 0,
}
impl From<CLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: CLKEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLKEN_A {
        match self.bits {
            true => CLKEN_A::ENABLED,
            false => CLKEN_A::DISABLED,
        }
    }
    #[doc = "The time counters are enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CLKEN_A::ENABLED
    }
    #[doc = "The time counters are disabled so that they may be initialized."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLKEN_A::DISABLED
    }
}
#[doc = "Field `CLKEN` writer - Clock Enable."]
pub type CLKEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CLKEN_A>;
impl<'a, REG, const O: u8> CLKEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The time counters are enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CLKEN_A::ENABLED)
    }
    #[doc = "The time counters are disabled so that they may be initialized."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CLKEN_A::DISABLED)
    }
}
#[doc = "Field `CTCRST` reader - CTC Reset."]
pub type CTCRST_R = crate::BitReader<CTCRST_A>;
#[doc = "CTC Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTCRST_A {
    #[doc = "1: When one, the elements in the internal oscillator divider are reset, and remain reset until CCR\\[1\\]
is changed to zero. This is the divider that generates the 1 Hz clock from the 32.768 kHz crystal. The state of the divider is not visible to software."]
    RESET = 1,
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
}
impl From<CTCRST_A> for bool {
    #[inline(always)]
    fn from(variant: CTCRST_A) -> Self {
        variant as u8 != 0
    }
}
impl CTCRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CTCRST_A {
        match self.bits {
            true => CTCRST_A::RESET,
            false => CTCRST_A::NO_EFFECT,
        }
    }
    #[doc = "When one, the elements in the internal oscillator divider are reset, and remain reset until CCR\\[1\\]
is changed to zero. This is the divider that generates the 1 Hz clock from the 32.768 kHz crystal. The state of the divider is not visible to software."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == CTCRST_A::RESET
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CTCRST_A::NO_EFFECT
    }
}
#[doc = "Field `CTCRST` writer - CTC Reset."]
pub type CTCRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CTCRST_A>;
impl<'a, REG, const O: u8> CTCRST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When one, the elements in the internal oscillator divider are reset, and remain reset until CCR\\[1\\]
is changed to zero. This is the divider that generates the 1 Hz clock from the 32.768 kHz crystal. The state of the divider is not visible to software."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(CTCRST_A::RESET)
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(CTCRST_A::NO_EFFECT)
    }
}
#[doc = "Field `CCALEN` reader - Calibration counter enable."]
pub type CCALEN_R = crate::BitReader<CCALEN_A>;
#[doc = "Calibration counter enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCALEN_A {
    #[doc = "1: The calibration counter is disabled and reset to zero."]
    DISABLED = 1,
    #[doc = "0: The calibration counter is enabled and counting, using the 1 Hz clock. When the calibration counter is equal to the value of the CALIBRATION register, the counter resets and repeats counting up to the value of the CALIBRATION register. See Section 30.6.4.2 and Section 30.6.5."]
    ENABLED = 0,
}
impl From<CCALEN_A> for bool {
    #[inline(always)]
    fn from(variant: CCALEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CCALEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCALEN_A {
        match self.bits {
            true => CCALEN_A::DISABLED,
            false => CCALEN_A::ENABLED,
        }
    }
    #[doc = "The calibration counter is disabled and reset to zero."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CCALEN_A::DISABLED
    }
    #[doc = "The calibration counter is enabled and counting, using the 1 Hz clock. When the calibration counter is equal to the value of the CALIBRATION register, the counter resets and repeats counting up to the value of the CALIBRATION register. See Section 30.6.4.2 and Section 30.6.5."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CCALEN_A::ENABLED
    }
}
#[doc = "Field `CCALEN` writer - Calibration counter enable."]
pub type CCALEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CCALEN_A>;
impl<'a, REG, const O: u8> CCALEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The calibration counter is disabled and reset to zero."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CCALEN_A::DISABLED)
    }
    #[doc = "The calibration counter is enabled and counting, using the 1 Hz clock. When the calibration counter is equal to the value of the CALIBRATION register, the counter resets and repeats counting up to the value of the CALIBRATION register. See Section 30.6.4.2 and Section 30.6.5."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CCALEN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Clock Enable."]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CTC Reset."]
    #[inline(always)]
    pub fn ctcrst(&self) -> CTCRST_R {
        CTCRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Calibration counter enable."]
    #[inline(always)]
    pub fn ccalen(&self) -> CCALEN_R {
        CCALEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR")
            .field("clken", &format_args!("{}", self.clken().bit()))
            .field("ctcrst", &format_args!("{}", self.ctcrst().bit()))
            .field("ccalen", &format_args!("{}", self.ccalen().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CCR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Enable."]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> CLKEN_W<CCR_SPEC, 0> {
        CLKEN_W::new(self)
    }
    #[doc = "Bit 1 - CTC Reset."]
    #[inline(always)]
    #[must_use]
    pub fn ctcrst(&mut self) -> CTCRST_W<CCR_SPEC, 1> {
        CTCRST_W::new(self)
    }
    #[doc = "Bit 4 - Calibration counter enable."]
    #[inline(always)]
    #[must_use]
    pub fn ccalen(&mut self) -> CCALEN_W<CCR_SPEC, 4> {
        CCALEN_W::new(self)
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
#[doc = "Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
