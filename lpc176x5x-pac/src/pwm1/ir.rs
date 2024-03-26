#[doc = "Register `IR` reader"]
pub type R = crate::R<IR_SPEC>;
#[doc = "Register `IR` writer"]
pub type W = crate::W<IR_SPEC>;
#[doc = "Field `PWMMR0INT` reader - Interrupt flag for PWM match channel 0."]
pub type PWMMR0INT_R = crate::BitReader;
#[doc = "Field `PWMMR0INT` writer - Interrupt flag for PWM match channel 0."]
pub type PWMMR0INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWMMR1INT` reader - Interrupt flag for PWM match channel 1."]
pub type PWMMR1INT_R = crate::BitReader;
#[doc = "Field `PWMMR1INT` writer - Interrupt flag for PWM match channel 1."]
pub type PWMMR1INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWMMR2INT` reader - Interrupt flag for PWM match channel 2."]
pub type PWMMR2INT_R = crate::BitReader;
#[doc = "Field `PWMMR2INT` writer - Interrupt flag for PWM match channel 2."]
pub type PWMMR2INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWMMR3INT` reader - Interrupt flag for PWM match channel 3."]
pub type PWMMR3INT_R = crate::BitReader;
#[doc = "Field `PWMMR3INT` writer - Interrupt flag for PWM match channel 3."]
pub type PWMMR3INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWMCAP0INT` reader - Interrupt flag for capture input 0"]
pub type PWMCAP0INT_R = crate::BitReader;
#[doc = "Field `PWMCAP0INT` writer - Interrupt flag for capture input 0"]
pub type PWMCAP0INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWMCAP1INT` reader - Interrupt flag for capture input 1 (available in PWM1IR only; this bit is reserved in PWM0IR)."]
pub type PWMCAP1INT_R = crate::BitReader;
#[doc = "Field `PWMCAP1INT` writer - Interrupt flag for capture input 1 (available in PWM1IR only; this bit is reserved in PWM0IR)."]
pub type PWMCAP1INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWMMR4INT` reader - Interrupt flag for PWM match channel 4."]
pub type PWMMR4INT_R = crate::BitReader;
#[doc = "Field `PWMMR4INT` writer - Interrupt flag for PWM match channel 4."]
pub type PWMMR4INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWMMR5INT` reader - Interrupt flag for PWM match channel 5."]
pub type PWMMR5INT_R = crate::BitReader;
#[doc = "Field `PWMMR5INT` writer - Interrupt flag for PWM match channel 5."]
pub type PWMMR5INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWMMR6INT` reader - Interrupt flag for PWM match channel 6."]
pub type PWMMR6INT_R = crate::BitReader;
#[doc = "Field `PWMMR6INT` writer - Interrupt flag for PWM match channel 6."]
pub type PWMMR6INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Interrupt flag for PWM match channel 0."]
    #[inline(always)]
    pub fn pwmmr0int(&self) -> PWMMR0INT_R {
        PWMMR0INT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt flag for PWM match channel 1."]
    #[inline(always)]
    pub fn pwmmr1int(&self) -> PWMMR1INT_R {
        PWMMR1INT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt flag for PWM match channel 2."]
    #[inline(always)]
    pub fn pwmmr2int(&self) -> PWMMR2INT_R {
        PWMMR2INT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt flag for PWM match channel 3."]
    #[inline(always)]
    pub fn pwmmr3int(&self) -> PWMMR3INT_R {
        PWMMR3INT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt flag for capture input 0"]
    #[inline(always)]
    pub fn pwmcap0int(&self) -> PWMCAP0INT_R {
        PWMCAP0INT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt flag for capture input 1 (available in PWM1IR only; this bit is reserved in PWM0IR)."]
    #[inline(always)]
    pub fn pwmcap1int(&self) -> PWMCAP1INT_R {
        PWMCAP1INT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt flag for PWM match channel 4."]
    #[inline(always)]
    pub fn pwmmr4int(&self) -> PWMMR4INT_R {
        PWMMR4INT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt flag for PWM match channel 5."]
    #[inline(always)]
    pub fn pwmmr5int(&self) -> PWMMR5INT_R {
        PWMMR5INT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt flag for PWM match channel 6."]
    #[inline(always)]
    pub fn pwmmr6int(&self) -> PWMMR6INT_R {
        PWMMR6INT_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IR")
            .field("pwmmr0int", &format_args!("{}", self.pwmmr0int().bit()))
            .field("pwmmr1int", &format_args!("{}", self.pwmmr1int().bit()))
            .field("pwmmr2int", &format_args!("{}", self.pwmmr2int().bit()))
            .field("pwmmr3int", &format_args!("{}", self.pwmmr3int().bit()))
            .field("pwmcap0int", &format_args!("{}", self.pwmcap0int().bit()))
            .field("pwmcap1int", &format_args!("{}", self.pwmcap1int().bit()))
            .field("pwmmr4int", &format_args!("{}", self.pwmmr4int().bit()))
            .field("pwmmr5int", &format_args!("{}", self.pwmmr5int().bit()))
            .field("pwmmr6int", &format_args!("{}", self.pwmmr6int().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<IR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt flag for PWM match channel 0."]
    #[inline(always)]
    #[must_use]
    pub fn pwmmr0int(&mut self) -> PWMMR0INT_W<IR_SPEC, 0> {
        PWMMR0INT_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt flag for PWM match channel 1."]
    #[inline(always)]
    #[must_use]
    pub fn pwmmr1int(&mut self) -> PWMMR1INT_W<IR_SPEC, 1> {
        PWMMR1INT_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt flag for PWM match channel 2."]
    #[inline(always)]
    #[must_use]
    pub fn pwmmr2int(&mut self) -> PWMMR2INT_W<IR_SPEC, 2> {
        PWMMR2INT_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt flag for PWM match channel 3."]
    #[inline(always)]
    #[must_use]
    pub fn pwmmr3int(&mut self) -> PWMMR3INT_W<IR_SPEC, 3> {
        PWMMR3INT_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt flag for capture input 0"]
    #[inline(always)]
    #[must_use]
    pub fn pwmcap0int(&mut self) -> PWMCAP0INT_W<IR_SPEC, 4> {
        PWMCAP0INT_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt flag for capture input 1 (available in PWM1IR only; this bit is reserved in PWM0IR)."]
    #[inline(always)]
    #[must_use]
    pub fn pwmcap1int(&mut self) -> PWMCAP1INT_W<IR_SPEC, 5> {
        PWMCAP1INT_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt flag for PWM match channel 4."]
    #[inline(always)]
    #[must_use]
    pub fn pwmmr4int(&mut self) -> PWMMR4INT_W<IR_SPEC, 8> {
        PWMMR4INT_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt flag for PWM match channel 5."]
    #[inline(always)]
    #[must_use]
    pub fn pwmmr5int(&mut self) -> PWMMR5INT_W<IR_SPEC, 9> {
        PWMMR5INT_W::new(self)
    }
    #[doc = "Bit 10 - Interrupt flag for PWM match channel 6."]
    #[inline(always)]
    #[must_use]
    pub fn pwmmr6int(&mut self) -> PWMMR6INT_W<IR_SPEC, 10> {
        PWMMR6INT_W::new(self)
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
#[doc = "Interrupt Register. The IR can be written to clear interrupts, or read to identify which PWM interrupt sources are pending.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IR_SPEC;
impl crate::RegisterSpec for IR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ir::R`](R) reader structure"]
impl crate::Readable for IR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ir::W`](W) writer structure"]
impl crate::Writable for IR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IR to value 0"]
impl crate::Resettable for IR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
