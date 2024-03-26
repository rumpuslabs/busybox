#[doc = "Register `EXTMODE` reader"]
pub type R = crate::R<EXTMODE_SPEC>;
#[doc = "Register `EXTMODE` writer"]
pub type W = crate::W<EXTMODE_SPEC>;
#[doc = "Field `EXTMODE0` reader - External interrupt 0 EINT0 mode."]
pub type EXTMODE0_R = crate::BitReader<EXTMODE0_A>;
#[doc = "External interrupt 0 EINT0 mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTMODE0_A {
    #[doc = "0: Level-sensitive. Level-sensitivity is selected for EINT0."]
    LEVEL_SENSITIVE = 0,
    #[doc = "1: Edge-sensitive. EINT0 is edge sensitive."]
    EDGE_SENSITIVE = 1,
}
impl From<EXTMODE0_A> for bool {
    #[inline(always)]
    fn from(variant: EXTMODE0_A) -> Self {
        variant as u8 != 0
    }
}
impl EXTMODE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTMODE0_A {
        match self.bits {
            false => EXTMODE0_A::LEVEL_SENSITIVE,
            true => EXTMODE0_A::EDGE_SENSITIVE,
        }
    }
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT0."]
    #[inline(always)]
    pub fn is_level_sensitive(&self) -> bool {
        *self == EXTMODE0_A::LEVEL_SENSITIVE
    }
    #[doc = "Edge-sensitive. EINT0 is edge sensitive."]
    #[inline(always)]
    pub fn is_edge_sensitive(&self) -> bool {
        *self == EXTMODE0_A::EDGE_SENSITIVE
    }
}
#[doc = "Field `EXTMODE0` writer - External interrupt 0 EINT0 mode."]
pub type EXTMODE0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, EXTMODE0_A>;
impl<'a, REG, const O: u8> EXTMODE0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT0."]
    #[inline(always)]
    pub fn level_sensitive(self) -> &'a mut crate::W<REG> {
        self.variant(EXTMODE0_A::LEVEL_SENSITIVE)
    }
    #[doc = "Edge-sensitive. EINT0 is edge sensitive."]
    #[inline(always)]
    pub fn edge_sensitive(self) -> &'a mut crate::W<REG> {
        self.variant(EXTMODE0_A::EDGE_SENSITIVE)
    }
}
#[doc = "Field `EXTMODE1` reader - External interrupt 1 EINT1 mode."]
pub type EXTMODE1_R = crate::BitReader<EXTMODE1_A>;
#[doc = "External interrupt 1 EINT1 mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTMODE1_A {
    #[doc = "0: Level-sensitive. Level-sensitivity is selected for EINT1."]
    LEVEL_SENSITIVE = 0,
    #[doc = "1: Edge-sensitive. EINT1 is edge sensitive."]
    EDGE_SENSITIVE = 1,
}
impl From<EXTMODE1_A> for bool {
    #[inline(always)]
    fn from(variant: EXTMODE1_A) -> Self {
        variant as u8 != 0
    }
}
impl EXTMODE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTMODE1_A {
        match self.bits {
            false => EXTMODE1_A::LEVEL_SENSITIVE,
            true => EXTMODE1_A::EDGE_SENSITIVE,
        }
    }
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT1."]
    #[inline(always)]
    pub fn is_level_sensitive(&self) -> bool {
        *self == EXTMODE1_A::LEVEL_SENSITIVE
    }
    #[doc = "Edge-sensitive. EINT1 is edge sensitive."]
    #[inline(always)]
    pub fn is_edge_sensitive(&self) -> bool {
        *self == EXTMODE1_A::EDGE_SENSITIVE
    }
}
#[doc = "Field `EXTMODE1` writer - External interrupt 1 EINT1 mode."]
pub type EXTMODE1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, EXTMODE1_A>;
impl<'a, REG, const O: u8> EXTMODE1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT1."]
    #[inline(always)]
    pub fn level_sensitive(self) -> &'a mut crate::W<REG> {
        self.variant(EXTMODE1_A::LEVEL_SENSITIVE)
    }
    #[doc = "Edge-sensitive. EINT1 is edge sensitive."]
    #[inline(always)]
    pub fn edge_sensitive(self) -> &'a mut crate::W<REG> {
        self.variant(EXTMODE1_A::EDGE_SENSITIVE)
    }
}
#[doc = "Field `EXTMODE2` reader - External interrupt 2 EINT2 mode."]
pub type EXTMODE2_R = crate::BitReader<EXTMODE2_A>;
#[doc = "External interrupt 2 EINT2 mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTMODE2_A {
    #[doc = "0: Level-sensitive. Level-sensitivity is selected for EINT2."]
    LEVEL_SENSITIVE = 0,
    #[doc = "1: Edge-sensitive. EINT2 is edge sensitive."]
    EDGE_SENSITIVE = 1,
}
impl From<EXTMODE2_A> for bool {
    #[inline(always)]
    fn from(variant: EXTMODE2_A) -> Self {
        variant as u8 != 0
    }
}
impl EXTMODE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTMODE2_A {
        match self.bits {
            false => EXTMODE2_A::LEVEL_SENSITIVE,
            true => EXTMODE2_A::EDGE_SENSITIVE,
        }
    }
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT2."]
    #[inline(always)]
    pub fn is_level_sensitive(&self) -> bool {
        *self == EXTMODE2_A::LEVEL_SENSITIVE
    }
    #[doc = "Edge-sensitive. EINT2 is edge sensitive."]
    #[inline(always)]
    pub fn is_edge_sensitive(&self) -> bool {
        *self == EXTMODE2_A::EDGE_SENSITIVE
    }
}
#[doc = "Field `EXTMODE2` writer - External interrupt 2 EINT2 mode."]
pub type EXTMODE2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, EXTMODE2_A>;
impl<'a, REG, const O: u8> EXTMODE2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT2."]
    #[inline(always)]
    pub fn level_sensitive(self) -> &'a mut crate::W<REG> {
        self.variant(EXTMODE2_A::LEVEL_SENSITIVE)
    }
    #[doc = "Edge-sensitive. EINT2 is edge sensitive."]
    #[inline(always)]
    pub fn edge_sensitive(self) -> &'a mut crate::W<REG> {
        self.variant(EXTMODE2_A::EDGE_SENSITIVE)
    }
}
#[doc = "Field `EXTMODE3` reader - External interrupt 3 EINT3 mode."]
pub type EXTMODE3_R = crate::BitReader<EXTMODE3_A>;
#[doc = "External interrupt 3 EINT3 mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTMODE3_A {
    #[doc = "0: Level-sensitive. Level-sensitivity is selected for EINT3."]
    LEVEL_SENSITIVE = 0,
    #[doc = "1: Edge-sensitive. EINT3 is edge sensitive."]
    EDGE_SENSITIVE = 1,
}
impl From<EXTMODE3_A> for bool {
    #[inline(always)]
    fn from(variant: EXTMODE3_A) -> Self {
        variant as u8 != 0
    }
}
impl EXTMODE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXTMODE3_A {
        match self.bits {
            false => EXTMODE3_A::LEVEL_SENSITIVE,
            true => EXTMODE3_A::EDGE_SENSITIVE,
        }
    }
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT3."]
    #[inline(always)]
    pub fn is_level_sensitive(&self) -> bool {
        *self == EXTMODE3_A::LEVEL_SENSITIVE
    }
    #[doc = "Edge-sensitive. EINT3 is edge sensitive."]
    #[inline(always)]
    pub fn is_edge_sensitive(&self) -> bool {
        *self == EXTMODE3_A::EDGE_SENSITIVE
    }
}
#[doc = "Field `EXTMODE3` writer - External interrupt 3 EINT3 mode."]
pub type EXTMODE3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, EXTMODE3_A>;
impl<'a, REG, const O: u8> EXTMODE3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT3."]
    #[inline(always)]
    pub fn level_sensitive(self) -> &'a mut crate::W<REG> {
        self.variant(EXTMODE3_A::LEVEL_SENSITIVE)
    }
    #[doc = "Edge-sensitive. EINT3 is edge sensitive."]
    #[inline(always)]
    pub fn edge_sensitive(self) -> &'a mut crate::W<REG> {
        self.variant(EXTMODE3_A::EDGE_SENSITIVE)
    }
}
impl R {
    #[doc = "Bit 0 - External interrupt 0 EINT0 mode."]
    #[inline(always)]
    pub fn extmode0(&self) -> EXTMODE0_R {
        EXTMODE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External interrupt 1 EINT1 mode."]
    #[inline(always)]
    pub fn extmode1(&self) -> EXTMODE1_R {
        EXTMODE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External interrupt 2 EINT2 mode."]
    #[inline(always)]
    pub fn extmode2(&self) -> EXTMODE2_R {
        EXTMODE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External interrupt 3 EINT3 mode."]
    #[inline(always)]
    pub fn extmode3(&self) -> EXTMODE3_R {
        EXTMODE3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTMODE")
            .field("extmode0", &format_args!("{}", self.extmode0().bit()))
            .field("extmode1", &format_args!("{}", self.extmode1().bit()))
            .field("extmode2", &format_args!("{}", self.extmode2().bit()))
            .field("extmode3", &format_args!("{}", self.extmode3().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<EXTMODE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - External interrupt 0 EINT0 mode."]
    #[inline(always)]
    #[must_use]
    pub fn extmode0(&mut self) -> EXTMODE0_W<EXTMODE_SPEC, 0> {
        EXTMODE0_W::new(self)
    }
    #[doc = "Bit 1 - External interrupt 1 EINT1 mode."]
    #[inline(always)]
    #[must_use]
    pub fn extmode1(&mut self) -> EXTMODE1_W<EXTMODE_SPEC, 1> {
        EXTMODE1_W::new(self)
    }
    #[doc = "Bit 2 - External interrupt 2 EINT2 mode."]
    #[inline(always)]
    #[must_use]
    pub fn extmode2(&mut self) -> EXTMODE2_W<EXTMODE_SPEC, 2> {
        EXTMODE2_W::new(self)
    }
    #[doc = "Bit 3 - External interrupt 3 EINT3 mode."]
    #[inline(always)]
    #[must_use]
    pub fn extmode3(&mut self) -> EXTMODE3_W<EXTMODE_SPEC, 3> {
        EXTMODE3_W::new(self)
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
#[doc = "External Interrupt Mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extmode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extmode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTMODE_SPEC;
impl crate::RegisterSpec for EXTMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extmode::R`](R) reader structure"]
impl crate::Readable for EXTMODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`extmode::W`](W) writer structure"]
impl crate::Writable for EXTMODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTMODE to value 0"]
impl crate::Resettable for EXTMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
