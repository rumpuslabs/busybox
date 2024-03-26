#[doc = "Register `EXTPOLAR` reader"]
pub type R = crate::R<EXTPOLAR_SPEC>;
#[doc = "Register `EXTPOLAR` writer"]
pub type W = crate::W<EXTPOLAR_SPEC>;
#[doc = "Field `EXTPOLAR0` reader - External interrupt 0 EINT0 polarity."]
pub type EXTPOLAR0_R = crate::BitReader<EXTPOLAR0_A>;
#[doc = "External interrupt 0 EINT0 polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTPOLAR0_A {
    #[doc = "0: Falling edge. EINT0 is low-active or falling-edge sensitive (depending on EXTMODE0)."]
    FALLING_EDGE = 0,
    #[doc = "1: Rising edge. EINT0 is high-active or rising-edge sensitive (depending on EXTMODE0)."]
    RISING_EDGE = 1,
}
impl From<EXTPOLAR0_A> for bool {
    #[inline(always)]
    fn from(variant: EXTPOLAR0_A) -> Self {
        variant as u8 != 0
    }
}
impl EXTPOLAR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTPOLAR0_A {
        match self.bits {
            false => EXTPOLAR0_A::FALLING_EDGE,
            true => EXTPOLAR0_A::RISING_EDGE,
        }
    }
    #[doc = "Falling edge. EINT0 is low-active or falling-edge sensitive (depending on EXTMODE0)."]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == EXTPOLAR0_A::FALLING_EDGE
    }
    #[doc = "Rising edge. EINT0 is high-active or rising-edge sensitive (depending on EXTMODE0)."]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == EXTPOLAR0_A::RISING_EDGE
    }
}
#[doc = "Field `EXTPOLAR0` writer - External interrupt 0 EINT0 polarity."]
pub type EXTPOLAR0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, EXTPOLAR0_A>;
impl<'a, REG, const O: u8> EXTPOLAR0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge. EINT0 is low-active or falling-edge sensitive (depending on EXTMODE0)."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(EXTPOLAR0_A::FALLING_EDGE)
    }
    #[doc = "Rising edge. EINT0 is high-active or rising-edge sensitive (depending on EXTMODE0)."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(EXTPOLAR0_A::RISING_EDGE)
    }
}
#[doc = "Field `EXTPOLAR1` reader - External interrupt 1 EINT1 polarity."]
pub type EXTPOLAR1_R = crate::BitReader<EXTPOLAR1_A>;
#[doc = "External interrupt 1 EINT1 polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTPOLAR1_A {
    #[doc = "0: Falling edge. EINT1 is low-active or falling-edge sensitive (depending on EXTMODE1)."]
    FALLING_EDGE = 0,
    #[doc = "1: Rising edge. EINT1 is high-active or rising-edge sensitive (depending on EXTMODE1)."]
    RISING_EDGE = 1,
}
impl From<EXTPOLAR1_A> for bool {
    #[inline(always)]
    fn from(variant: EXTPOLAR1_A) -> Self {
        variant as u8 != 0
    }
}
impl EXTPOLAR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTPOLAR1_A {
        match self.bits {
            false => EXTPOLAR1_A::FALLING_EDGE,
            true => EXTPOLAR1_A::RISING_EDGE,
        }
    }
    #[doc = "Falling edge. EINT1 is low-active or falling-edge sensitive (depending on EXTMODE1)."]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == EXTPOLAR1_A::FALLING_EDGE
    }
    #[doc = "Rising edge. EINT1 is high-active or rising-edge sensitive (depending on EXTMODE1)."]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == EXTPOLAR1_A::RISING_EDGE
    }
}
#[doc = "Field `EXTPOLAR1` writer - External interrupt 1 EINT1 polarity."]
pub type EXTPOLAR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, EXTPOLAR1_A>;
impl<'a, REG, const O: u8> EXTPOLAR1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge. EINT1 is low-active or falling-edge sensitive (depending on EXTMODE1)."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(EXTPOLAR1_A::FALLING_EDGE)
    }
    #[doc = "Rising edge. EINT1 is high-active or rising-edge sensitive (depending on EXTMODE1)."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(EXTPOLAR1_A::RISING_EDGE)
    }
}
#[doc = "Field `EXTPOLAR2` reader - External interrupt 2 EINT2 polarity."]
pub type EXTPOLAR2_R = crate::BitReader<EXTPOLAR2_A>;
#[doc = "External interrupt 2 EINT2 polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTPOLAR2_A {
    #[doc = "0: Falling edge. EINT2 is low-active or falling-edge sensitive (depending on EXTMODE2)."]
    FALLING_EDGE = 0,
    #[doc = "1: Rising edge. EINT2 is high-active or rising-edge sensitive (depending on EXTMODE2)."]
    RISING_EDGE = 1,
}
impl From<EXTPOLAR2_A> for bool {
    #[inline(always)]
    fn from(variant: EXTPOLAR2_A) -> Self {
        variant as u8 != 0
    }
}
impl EXTPOLAR2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTPOLAR2_A {
        match self.bits {
            false => EXTPOLAR2_A::FALLING_EDGE,
            true => EXTPOLAR2_A::RISING_EDGE,
        }
    }
    #[doc = "Falling edge. EINT2 is low-active or falling-edge sensitive (depending on EXTMODE2)."]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == EXTPOLAR2_A::FALLING_EDGE
    }
    #[doc = "Rising edge. EINT2 is high-active or rising-edge sensitive (depending on EXTMODE2)."]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == EXTPOLAR2_A::RISING_EDGE
    }
}
#[doc = "Field `EXTPOLAR2` writer - External interrupt 2 EINT2 polarity."]
pub type EXTPOLAR2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, EXTPOLAR2_A>;
impl<'a, REG, const O: u8> EXTPOLAR2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge. EINT2 is low-active or falling-edge sensitive (depending on EXTMODE2)."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(EXTPOLAR2_A::FALLING_EDGE)
    }
    #[doc = "Rising edge. EINT2 is high-active or rising-edge sensitive (depending on EXTMODE2)."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(EXTPOLAR2_A::RISING_EDGE)
    }
}
#[doc = "Field `EXTPOLAR3` reader - External interrupt 3 EINT3 polarity."]
pub type EXTPOLAR3_R = crate::BitReader<EXTPOLAR3_A>;
#[doc = "External interrupt 3 EINT3 polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTPOLAR3_A {
    #[doc = "0: Falling edge. EINT3 is low-active or falling-edge sensitive (depending on EXTMODE3)."]
    FALLING_EDGE = 0,
    #[doc = "1: Rising edge. EINT3 is high-active or rising-edge sensitive (depending on EXTMODE3)."]
    RISING_EDGE = 1,
}
impl From<EXTPOLAR3_A> for bool {
    #[inline(always)]
    fn from(variant: EXTPOLAR3_A) -> Self {
        variant as u8 != 0
    }
}
impl EXTPOLAR3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTPOLAR3_A {
        match self.bits {
            false => EXTPOLAR3_A::FALLING_EDGE,
            true => EXTPOLAR3_A::RISING_EDGE,
        }
    }
    #[doc = "Falling edge. EINT3 is low-active or falling-edge sensitive (depending on EXTMODE3)."]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == EXTPOLAR3_A::FALLING_EDGE
    }
    #[doc = "Rising edge. EINT3 is high-active or rising-edge sensitive (depending on EXTMODE3)."]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == EXTPOLAR3_A::RISING_EDGE
    }
}
#[doc = "Field `EXTPOLAR3` writer - External interrupt 3 EINT3 polarity."]
pub type EXTPOLAR3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, EXTPOLAR3_A>;
impl<'a, REG, const O: u8> EXTPOLAR3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge. EINT3 is low-active or falling-edge sensitive (depending on EXTMODE3)."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(EXTPOLAR3_A::FALLING_EDGE)
    }
    #[doc = "Rising edge. EINT3 is high-active or rising-edge sensitive (depending on EXTMODE3)."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(EXTPOLAR3_A::RISING_EDGE)
    }
}
impl R {
    #[doc = "Bit 0 - External interrupt 0 EINT0 polarity."]
    #[inline(always)]
    pub fn extpolar0(&self) -> EXTPOLAR0_R {
        EXTPOLAR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External interrupt 1 EINT1 polarity."]
    #[inline(always)]
    pub fn extpolar1(&self) -> EXTPOLAR1_R {
        EXTPOLAR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External interrupt 2 EINT2 polarity."]
    #[inline(always)]
    pub fn extpolar2(&self) -> EXTPOLAR2_R {
        EXTPOLAR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External interrupt 3 EINT3 polarity."]
    #[inline(always)]
    pub fn extpolar3(&self) -> EXTPOLAR3_R {
        EXTPOLAR3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTPOLAR")
            .field("extpolar0", &format_args!("{}", self.extpolar0().bit()))
            .field("extpolar1", &format_args!("{}", self.extpolar1().bit()))
            .field("extpolar2", &format_args!("{}", self.extpolar2().bit()))
            .field("extpolar3", &format_args!("{}", self.extpolar3().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<EXTPOLAR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - External interrupt 0 EINT0 polarity."]
    #[inline(always)]
    #[must_use]
    pub fn extpolar0(&mut self) -> EXTPOLAR0_W<EXTPOLAR_SPEC, 0> {
        EXTPOLAR0_W::new(self)
    }
    #[doc = "Bit 1 - External interrupt 1 EINT1 polarity."]
    #[inline(always)]
    #[must_use]
    pub fn extpolar1(&mut self) -> EXTPOLAR1_W<EXTPOLAR_SPEC, 1> {
        EXTPOLAR1_W::new(self)
    }
    #[doc = "Bit 2 - External interrupt 2 EINT2 polarity."]
    #[inline(always)]
    #[must_use]
    pub fn extpolar2(&mut self) -> EXTPOLAR2_W<EXTPOLAR_SPEC, 2> {
        EXTPOLAR2_W::new(self)
    }
    #[doc = "Bit 3 - External interrupt 3 EINT3 polarity."]
    #[inline(always)]
    #[must_use]
    pub fn extpolar3(&mut self) -> EXTPOLAR3_W<EXTPOLAR_SPEC, 3> {
        EXTPOLAR3_W::new(self)
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
#[doc = "External Interrupt Polarity Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extpolar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extpolar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTPOLAR_SPEC;
impl crate::RegisterSpec for EXTPOLAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extpolar::R`](R) reader structure"]
impl crate::Readable for EXTPOLAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`extpolar::W`](W) writer structure"]
impl crate::Writable for EXTPOLAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTPOLAR to value 0"]
impl crate::Resettable for EXTPOLAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
