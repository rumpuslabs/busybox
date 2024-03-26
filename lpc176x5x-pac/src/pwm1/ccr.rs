#[doc = "Register `CCR` reader"]
pub type R = crate::R<CCR_SPEC>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CCR_SPEC>;
#[doc = "Field `CAP0_R` reader - Capture on PWMn_CAP0 rising edge"]
pub type CAP0_R_R = crate::BitReader<CAP0_R_A>;
#[doc = "Capture on PWMn_CAP0 rising edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAP0_R_A {
    #[doc = "0: Disabled. This feature is disabled."]
    DISABLED = 0,
    #[doc = "1: Rising edge. A synchronously sampled rising edge on PWMn_CAP0 will cause CR0 to be loaded with the contents of the TC."]
    RISING_EDGE = 1,
}
impl From<CAP0_R_A> for bool {
    #[inline(always)]
    fn from(variant: CAP0_R_A) -> Self {
        variant as u8 != 0
    }
}
impl CAP0_R_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CAP0_R_A {
        match self.bits {
            false => CAP0_R_A::DISABLED,
            true => CAP0_R_A::RISING_EDGE,
        }
    }
    #[doc = "Disabled. This feature is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CAP0_R_A::DISABLED
    }
    #[doc = "Rising edge. A synchronously sampled rising edge on PWMn_CAP0 will cause CR0 to be loaded with the contents of the TC."]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == CAP0_R_A::RISING_EDGE
    }
}
#[doc = "Field `CAP0_R` writer - Capture on PWMn_CAP0 rising edge"]
pub type CAP0_R_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CAP0_R_A>;
impl<'a, REG, const O: u8> CAP0_R_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. This feature is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CAP0_R_A::DISABLED)
    }
    #[doc = "Rising edge. A synchronously sampled rising edge on PWMn_CAP0 will cause CR0 to be loaded with the contents of the TC."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CAP0_R_A::RISING_EDGE)
    }
}
#[doc = "Field `CAP0_F` reader - Capture on PWMn_CAP0 falling edge"]
pub type CAP0_F_R = crate::BitReader<CAP0_F_A>;
#[doc = "Capture on PWMn_CAP0 falling edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAP0_F_A {
    #[doc = "0: Disabled. This feature is disabled."]
    DISABLED = 0,
    #[doc = "1: Falling edge. A synchronously sampled falling edge on PWMn_CAP0 will cause CR0 to be loaded with the contents of TC."]
    FALLING_EDGE = 1,
}
impl From<CAP0_F_A> for bool {
    #[inline(always)]
    fn from(variant: CAP0_F_A) -> Self {
        variant as u8 != 0
    }
}
impl CAP0_F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CAP0_F_A {
        match self.bits {
            false => CAP0_F_A::DISABLED,
            true => CAP0_F_A::FALLING_EDGE,
        }
    }
    #[doc = "Disabled. This feature is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CAP0_F_A::DISABLED
    }
    #[doc = "Falling edge. A synchronously sampled falling edge on PWMn_CAP0 will cause CR0 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == CAP0_F_A::FALLING_EDGE
    }
}
#[doc = "Field `CAP0_F` writer - Capture on PWMn_CAP0 falling edge"]
pub type CAP0_F_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CAP0_F_A>;
impl<'a, REG, const O: u8> CAP0_F_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. This feature is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CAP0_F_A::DISABLED)
    }
    #[doc = "Falling edge. A synchronously sampled falling edge on PWMn_CAP0 will cause CR0 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CAP0_F_A::FALLING_EDGE)
    }
}
#[doc = "Field `CAP0_I` reader - Interrupt on PWMn_CAP0 event"]
pub type CAP0_I_R = crate::BitReader<CAP0_I_A>;
#[doc = "Interrupt on PWMn_CAP0 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAP0_I_A {
    #[doc = "0: Disabled. This feature is disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt. A CR0 load due to a PWMn_CAP0 event will generate an interrupt."]
    INTERRUPT = 1,
}
impl From<CAP0_I_A> for bool {
    #[inline(always)]
    fn from(variant: CAP0_I_A) -> Self {
        variant as u8 != 0
    }
}
impl CAP0_I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CAP0_I_A {
        match self.bits {
            false => CAP0_I_A::DISABLED,
            true => CAP0_I_A::INTERRUPT,
        }
    }
    #[doc = "Disabled. This feature is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CAP0_I_A::DISABLED
    }
    #[doc = "Interrupt. A CR0 load due to a PWMn_CAP0 event will generate an interrupt."]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == CAP0_I_A::INTERRUPT
    }
}
#[doc = "Field `CAP0_I` writer - Interrupt on PWMn_CAP0 event"]
pub type CAP0_I_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CAP0_I_A>;
impl<'a, REG, const O: u8> CAP0_I_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. This feature is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CAP0_I_A::DISABLED)
    }
    #[doc = "Interrupt. A CR0 load due to a PWMn_CAP0 event will generate an interrupt."]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(CAP0_I_A::INTERRUPT)
    }
}
#[doc = "Field `CAP1_R` reader - Capture on PWMn_CAP1 rising edge. Reserved for PWM0."]
pub type CAP1_R_R = crate::BitReader<CAP1_R_A>;
#[doc = "Capture on PWMn_CAP1 rising edge. Reserved for PWM0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAP1_R_A {
    #[doc = "0: Disabled. This feature is disabled."]
    DISABLED = 0,
    #[doc = "1: Rising edge. A synchronously sampled rising edge on PWMn_CAP1 will cause CR1 to be loaded with the contents of the TC."]
    RISING_EDGE = 1,
}
impl From<CAP1_R_A> for bool {
    #[inline(always)]
    fn from(variant: CAP1_R_A) -> Self {
        variant as u8 != 0
    }
}
impl CAP1_R_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CAP1_R_A {
        match self.bits {
            false => CAP1_R_A::DISABLED,
            true => CAP1_R_A::RISING_EDGE,
        }
    }
    #[doc = "Disabled. This feature is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CAP1_R_A::DISABLED
    }
    #[doc = "Rising edge. A synchronously sampled rising edge on PWMn_CAP1 will cause CR1 to be loaded with the contents of the TC."]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == CAP1_R_A::RISING_EDGE
    }
}
#[doc = "Field `CAP1_R` writer - Capture on PWMn_CAP1 rising edge. Reserved for PWM0."]
pub type CAP1_R_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CAP1_R_A>;
impl<'a, REG, const O: u8> CAP1_R_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. This feature is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CAP1_R_A::DISABLED)
    }
    #[doc = "Rising edge. A synchronously sampled rising edge on PWMn_CAP1 will cause CR1 to be loaded with the contents of the TC."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CAP1_R_A::RISING_EDGE)
    }
}
#[doc = "Field `CAP1_F` reader - Capture on PWMn_CAP1 falling edge. Reserved for PWM0."]
pub type CAP1_F_R = crate::BitReader<CAP1_F_A>;
#[doc = "Capture on PWMn_CAP1 falling edge. Reserved for PWM0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAP1_F_A {
    #[doc = "0: Disabled. This feature is disabled."]
    DISABLED = 0,
    #[doc = "1: Falling edge. A synchronously sampled falling edge on PWMn_CAP1 will cause CR1 to be loaded with the contents of TC."]
    FALLING_EDGE = 1,
}
impl From<CAP1_F_A> for bool {
    #[inline(always)]
    fn from(variant: CAP1_F_A) -> Self {
        variant as u8 != 0
    }
}
impl CAP1_F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CAP1_F_A {
        match self.bits {
            false => CAP1_F_A::DISABLED,
            true => CAP1_F_A::FALLING_EDGE,
        }
    }
    #[doc = "Disabled. This feature is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CAP1_F_A::DISABLED
    }
    #[doc = "Falling edge. A synchronously sampled falling edge on PWMn_CAP1 will cause CR1 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == CAP1_F_A::FALLING_EDGE
    }
}
#[doc = "Field `CAP1_F` writer - Capture on PWMn_CAP1 falling edge. Reserved for PWM0."]
pub type CAP1_F_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CAP1_F_A>;
impl<'a, REG, const O: u8> CAP1_F_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. This feature is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CAP1_F_A::DISABLED)
    }
    #[doc = "Falling edge. A synchronously sampled falling edge on PWMn_CAP1 will cause CR1 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CAP1_F_A::FALLING_EDGE)
    }
}
#[doc = "Field `CAP1_I` reader - Interrupt on PWMn_CAP1 event. Reserved for PWM0."]
pub type CAP1_I_R = crate::BitReader<CAP1_I_A>;
#[doc = "Interrupt on PWMn_CAP1 event. Reserved for PWM0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAP1_I_A {
    #[doc = "0: Disabled. This feature is disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt. A CR1 load due to a PWMn_CAP1 event will generate an interrupt."]
    INTERRUPT = 1,
}
impl From<CAP1_I_A> for bool {
    #[inline(always)]
    fn from(variant: CAP1_I_A) -> Self {
        variant as u8 != 0
    }
}
impl CAP1_I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CAP1_I_A {
        match self.bits {
            false => CAP1_I_A::DISABLED,
            true => CAP1_I_A::INTERRUPT,
        }
    }
    #[doc = "Disabled. This feature is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CAP1_I_A::DISABLED
    }
    #[doc = "Interrupt. A CR1 load due to a PWMn_CAP1 event will generate an interrupt."]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == CAP1_I_A::INTERRUPT
    }
}
#[doc = "Field `CAP1_I` writer - Interrupt on PWMn_CAP1 event. Reserved for PWM0."]
pub type CAP1_I_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CAP1_I_A>;
impl<'a, REG, const O: u8> CAP1_I_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. This feature is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CAP1_I_A::DISABLED)
    }
    #[doc = "Interrupt. A CR1 load due to a PWMn_CAP1 event will generate an interrupt."]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(CAP1_I_A::INTERRUPT)
    }
}
impl R {
    #[doc = "Bit 0 - Capture on PWMn_CAP0 rising edge"]
    #[inline(always)]
    pub fn cap0_r(&self) -> CAP0_R_R {
        CAP0_R_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture on PWMn_CAP0 falling edge"]
    #[inline(always)]
    pub fn cap0_f(&self) -> CAP0_F_R {
        CAP0_F_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt on PWMn_CAP0 event"]
    #[inline(always)]
    pub fn cap0_i(&self) -> CAP0_I_R {
        CAP0_I_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture on PWMn_CAP1 rising edge. Reserved for PWM0."]
    #[inline(always)]
    pub fn cap1_r(&self) -> CAP1_R_R {
        CAP1_R_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture on PWMn_CAP1 falling edge. Reserved for PWM0."]
    #[inline(always)]
    pub fn cap1_f(&self) -> CAP1_F_R {
        CAP1_F_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt on PWMn_CAP1 event. Reserved for PWM0."]
    #[inline(always)]
    pub fn cap1_i(&self) -> CAP1_I_R {
        CAP1_I_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR")
            .field("cap0_r", &format_args!("{}", self.cap0_r().bit()))
            .field("cap0_f", &format_args!("{}", self.cap0_f().bit()))
            .field("cap0_i", &format_args!("{}", self.cap0_i().bit()))
            .field("cap1_r", &format_args!("{}", self.cap1_r().bit()))
            .field("cap1_f", &format_args!("{}", self.cap1_f().bit()))
            .field("cap1_i", &format_args!("{}", self.cap1_i().bit()))
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
    #[doc = "Bit 0 - Capture on PWMn_CAP0 rising edge"]
    #[inline(always)]
    #[must_use]
    pub fn cap0_r(&mut self) -> CAP0_R_W<CCR_SPEC, 0> {
        CAP0_R_W::new(self)
    }
    #[doc = "Bit 1 - Capture on PWMn_CAP0 falling edge"]
    #[inline(always)]
    #[must_use]
    pub fn cap0_f(&mut self) -> CAP0_F_W<CCR_SPEC, 1> {
        CAP0_F_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt on PWMn_CAP0 event"]
    #[inline(always)]
    #[must_use]
    pub fn cap0_i(&mut self) -> CAP0_I_W<CCR_SPEC, 2> {
        CAP0_I_W::new(self)
    }
    #[doc = "Bit 3 - Capture on PWMn_CAP1 rising edge. Reserved for PWM0."]
    #[inline(always)]
    #[must_use]
    pub fn cap1_r(&mut self) -> CAP1_R_W<CCR_SPEC, 3> {
        CAP1_R_W::new(self)
    }
    #[doc = "Bit 4 - Capture on PWMn_CAP1 falling edge. Reserved for PWM0."]
    #[inline(always)]
    #[must_use]
    pub fn cap1_f(&mut self) -> CAP1_F_W<CCR_SPEC, 4> {
        CAP1_F_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt on PWMn_CAP1 event. Reserved for PWM0."]
    #[inline(always)]
    #[must_use]
    pub fn cap1_i(&mut self) -> CAP1_I_W<CCR_SPEC, 5> {
        CAP1_I_W::new(self)
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
#[doc = "Capture Control Register. The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated for a capture event.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
