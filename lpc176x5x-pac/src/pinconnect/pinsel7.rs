#[doc = "Register `PINSEL7` reader"]
pub type R = crate::R<PINSEL7_SPEC>;
#[doc = "Register `PINSEL7` writer"]
pub type W = crate::W<PINSEL7_SPEC>;
#[doc = "Field `P3_25` reader - Pin function select P3.25."]
pub type P3_25_R = crate::FieldReader<P3_25_A>;
#[doc = "Pin function select P3.25.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P3_25_A {
    #[doc = "0: GPIO P3.25"]
    GPIO_P3 = 0,
    #[doc = "2: MAT0.0"]
    MAT0 = 2,
    #[doc = "3: PWM1.2"]
    PWM1 = 3,
}
impl From<P3_25_A> for u8 {
    #[inline(always)]
    fn from(variant: P3_25_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P3_25_A {
    type Ux = u8;
}
impl P3_25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P3_25_A {
        match self.bits {
            0 => P3_25_A::GPIO_P3,
            2 => P3_25_A::MAT0,
            3 => P3_25_A::PWM1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P3.25"]
    #[inline(always)]
    pub fn is_gpio_p3(&self) -> bool {
        *self == P3_25_A::GPIO_P3
    }
    #[doc = "MAT0.0"]
    #[inline(always)]
    pub fn is_mat0(&self) -> bool {
        *self == P3_25_A::MAT0
    }
    #[doc = "PWM1.2"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P3_25_A::PWM1
    }
}
#[doc = "Field `P3_25` writer - Pin function select P3.25."]
pub type P3_25_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, P3_25_A>;
impl<'a, REG, const O: u8> P3_25_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P3.25"]
    #[inline(always)]
    pub fn gpio_p3(self) -> &'a mut crate::W<REG> {
        self.variant(P3_25_A::GPIO_P3)
    }
    #[doc = "MAT0.0"]
    #[inline(always)]
    pub fn mat0(self) -> &'a mut crate::W<REG> {
        self.variant(P3_25_A::MAT0)
    }
    #[doc = "PWM1.2"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(P3_25_A::PWM1)
    }
}
#[doc = "Field `P3_26` reader - Pin function select P3.26."]
pub type P3_26_R = crate::FieldReader<P3_26_A>;
#[doc = "Pin function select P3.26.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P3_26_A {
    #[doc = "0: GPIO P3.26"]
    GPIO_P3 = 0,
    #[doc = "1: STCLK"]
    STCLK = 1,
    #[doc = "2: MAT0.1"]
    MAT0 = 2,
    #[doc = "3: PWM1.3"]
    PWM1 = 3,
}
impl From<P3_26_A> for u8 {
    #[inline(always)]
    fn from(variant: P3_26_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P3_26_A {
    type Ux = u8;
}
impl P3_26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P3_26_A {
        match self.bits {
            0 => P3_26_A::GPIO_P3,
            1 => P3_26_A::STCLK,
            2 => P3_26_A::MAT0,
            3 => P3_26_A::PWM1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P3.26"]
    #[inline(always)]
    pub fn is_gpio_p3(&self) -> bool {
        *self == P3_26_A::GPIO_P3
    }
    #[doc = "STCLK"]
    #[inline(always)]
    pub fn is_stclk(&self) -> bool {
        *self == P3_26_A::STCLK
    }
    #[doc = "MAT0.1"]
    #[inline(always)]
    pub fn is_mat0(&self) -> bool {
        *self == P3_26_A::MAT0
    }
    #[doc = "PWM1.3"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P3_26_A::PWM1
    }
}
#[doc = "Field `P3_26` writer - Pin function select P3.26."]
pub type P3_26_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, P3_26_A>;
impl<'a, REG, const O: u8> P3_26_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P3.26"]
    #[inline(always)]
    pub fn gpio_p3(self) -> &'a mut crate::W<REG> {
        self.variant(P3_26_A::GPIO_P3)
    }
    #[doc = "STCLK"]
    #[inline(always)]
    pub fn stclk(self) -> &'a mut crate::W<REG> {
        self.variant(P3_26_A::STCLK)
    }
    #[doc = "MAT0.1"]
    #[inline(always)]
    pub fn mat0(self) -> &'a mut crate::W<REG> {
        self.variant(P3_26_A::MAT0)
    }
    #[doc = "PWM1.3"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(P3_26_A::PWM1)
    }
}
impl R {
    #[doc = "Bits 18:19 - Pin function select P3.25."]
    #[inline(always)]
    pub fn p3_25(&self) -> P3_25_R {
        P3_25_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Pin function select P3.26."]
    #[inline(always)]
    pub fn p3_26(&self) -> P3_26_R {
        P3_26_R::new(((self.bits >> 20) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PINSEL7")
            .field("p3_25", &format_args!("{}", self.p3_25().bits()))
            .field("p3_26", &format_args!("{}", self.p3_26().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<PINSEL7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 18:19 - Pin function select P3.25."]
    #[inline(always)]
    #[must_use]
    pub fn p3_25(&mut self) -> P3_25_W<PINSEL7_SPEC, 18> {
        P3_25_W::new(self)
    }
    #[doc = "Bits 20:21 - Pin function select P3.26."]
    #[inline(always)]
    #[must_use]
    pub fn p3_26(&mut self) -> P3_26_W<PINSEL7_SPEC, 20> {
        P3_26_W::new(self)
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
#[doc = "Pin function select register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinsel7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinsel7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PINSEL7_SPEC;
impl crate::RegisterSpec for PINSEL7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinsel7::R`](R) reader structure"]
impl crate::Readable for PINSEL7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pinsel7::W`](W) writer structure"]
impl crate::Writable for PINSEL7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINSEL7 to value 0"]
impl crate::Resettable for PINSEL7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
