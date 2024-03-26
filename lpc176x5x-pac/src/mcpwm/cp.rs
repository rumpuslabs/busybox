#[doc = "Register `CP` reader"]
pub type R = crate::R<CP_SPEC>;
#[doc = "Register `CP` writer"]
pub type W = crate::W<CP_SPEC>;
#[doc = "Field `CCPA0` reader - Communication pattern output A, channel 0."]
pub type CCPA0_R = crate::BitReader<CCPA0_A>;
#[doc = "Communication pattern output A, channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCPA0_A {
    #[doc = "0: MCOA0 passive."]
    PASSIVE = 0,
    #[doc = "1: internal MCOA0."]
    INTERNAL = 1,
}
impl From<CCPA0_A> for bool {
    #[inline(always)]
    fn from(variant: CCPA0_A) -> Self {
        variant as u8 != 0
    }
}
impl CCPA0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCPA0_A {
        match self.bits {
            false => CCPA0_A::PASSIVE,
            true => CCPA0_A::INTERNAL,
        }
    }
    #[doc = "MCOA0 passive."]
    #[inline(always)]
    pub fn is_passive(&self) -> bool {
        *self == CCPA0_A::PASSIVE
    }
    #[doc = "internal MCOA0."]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == CCPA0_A::INTERNAL
    }
}
#[doc = "Field `CCPA0` writer - Communication pattern output A, channel 0."]
pub type CCPA0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CCPA0_A>;
impl<'a, REG, const O: u8> CCPA0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MCOA0 passive."]
    #[inline(always)]
    pub fn passive(self) -> &'a mut crate::W<REG> {
        self.variant(CCPA0_A::PASSIVE)
    }
    #[doc = "internal MCOA0."]
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(CCPA0_A::INTERNAL)
    }
}
#[doc = "Field `CCPB0` reader - Communication pattern output B, channel 0."]
pub type CCPB0_R = crate::BitReader<CCPB0_A>;
#[doc = "Communication pattern output B, channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCPB0_A {
    #[doc = "0: MCOB0 passive."]
    PASSIVE = 0,
    #[doc = "1: MCOB0 tracks internal MCOA0."]
    INTERNAL_MCOA0 = 1,
}
impl From<CCPB0_A> for bool {
    #[inline(always)]
    fn from(variant: CCPB0_A) -> Self {
        variant as u8 != 0
    }
}
impl CCPB0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCPB0_A {
        match self.bits {
            false => CCPB0_A::PASSIVE,
            true => CCPB0_A::INTERNAL_MCOA0,
        }
    }
    #[doc = "MCOB0 passive."]
    #[inline(always)]
    pub fn is_passive(&self) -> bool {
        *self == CCPB0_A::PASSIVE
    }
    #[doc = "MCOB0 tracks internal MCOA0."]
    #[inline(always)]
    pub fn is_internal_mcoa0(&self) -> bool {
        *self == CCPB0_A::INTERNAL_MCOA0
    }
}
#[doc = "Field `CCPB0` writer - Communication pattern output B, channel 0."]
pub type CCPB0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CCPB0_A>;
impl<'a, REG, const O: u8> CCPB0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MCOB0 passive."]
    #[inline(always)]
    pub fn passive(self) -> &'a mut crate::W<REG> {
        self.variant(CCPB0_A::PASSIVE)
    }
    #[doc = "MCOB0 tracks internal MCOA0."]
    #[inline(always)]
    pub fn internal_mcoa0(self) -> &'a mut crate::W<REG> {
        self.variant(CCPB0_A::INTERNAL_MCOA0)
    }
}
#[doc = "Field `CCPA1` reader - Communication pattern output A, channel 1."]
pub type CCPA1_R = crate::BitReader<CCPA1_A>;
#[doc = "Communication pattern output A, channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCPA1_A {
    #[doc = "0: MCOA1 passive."]
    PASSIVE = 0,
    #[doc = "1: MCOA1 tracks internal MCOA0."]
    INTERNAL_MCOA0 = 1,
}
impl From<CCPA1_A> for bool {
    #[inline(always)]
    fn from(variant: CCPA1_A) -> Self {
        variant as u8 != 0
    }
}
impl CCPA1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCPA1_A {
        match self.bits {
            false => CCPA1_A::PASSIVE,
            true => CCPA1_A::INTERNAL_MCOA0,
        }
    }
    #[doc = "MCOA1 passive."]
    #[inline(always)]
    pub fn is_passive(&self) -> bool {
        *self == CCPA1_A::PASSIVE
    }
    #[doc = "MCOA1 tracks internal MCOA0."]
    #[inline(always)]
    pub fn is_internal_mcoa0(&self) -> bool {
        *self == CCPA1_A::INTERNAL_MCOA0
    }
}
#[doc = "Field `CCPA1` writer - Communication pattern output A, channel 1."]
pub type CCPA1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CCPA1_A>;
impl<'a, REG, const O: u8> CCPA1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MCOA1 passive."]
    #[inline(always)]
    pub fn passive(self) -> &'a mut crate::W<REG> {
        self.variant(CCPA1_A::PASSIVE)
    }
    #[doc = "MCOA1 tracks internal MCOA0."]
    #[inline(always)]
    pub fn internal_mcoa0(self) -> &'a mut crate::W<REG> {
        self.variant(CCPA1_A::INTERNAL_MCOA0)
    }
}
#[doc = "Field `CCPB1` reader - Communication pattern output B, channel 1."]
pub type CCPB1_R = crate::BitReader<CCPB1_A>;
#[doc = "Communication pattern output B, channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCPB1_A {
    #[doc = "0: MCOB1 passive."]
    PASSIVE = 0,
    #[doc = "1: MCOB1 tracks internal MCOA0."]
    INTERNAL_MCOA0 = 1,
}
impl From<CCPB1_A> for bool {
    #[inline(always)]
    fn from(variant: CCPB1_A) -> Self {
        variant as u8 != 0
    }
}
impl CCPB1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCPB1_A {
        match self.bits {
            false => CCPB1_A::PASSIVE,
            true => CCPB1_A::INTERNAL_MCOA0,
        }
    }
    #[doc = "MCOB1 passive."]
    #[inline(always)]
    pub fn is_passive(&self) -> bool {
        *self == CCPB1_A::PASSIVE
    }
    #[doc = "MCOB1 tracks internal MCOA0."]
    #[inline(always)]
    pub fn is_internal_mcoa0(&self) -> bool {
        *self == CCPB1_A::INTERNAL_MCOA0
    }
}
#[doc = "Field `CCPB1` writer - Communication pattern output B, channel 1."]
pub type CCPB1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CCPB1_A>;
impl<'a, REG, const O: u8> CCPB1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MCOB1 passive."]
    #[inline(always)]
    pub fn passive(self) -> &'a mut crate::W<REG> {
        self.variant(CCPB1_A::PASSIVE)
    }
    #[doc = "MCOB1 tracks internal MCOA0."]
    #[inline(always)]
    pub fn internal_mcoa0(self) -> &'a mut crate::W<REG> {
        self.variant(CCPB1_A::INTERNAL_MCOA0)
    }
}
#[doc = "Field `CCPA2` reader - Communication pattern output A, channel 2."]
pub type CCPA2_R = crate::BitReader<CCPA2_A>;
#[doc = "Communication pattern output A, channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCPA2_A {
    #[doc = "0: MCOA2 passive."]
    PASSIVE = 0,
    #[doc = "1: MCOA2 tracks internal MCOA0."]
    INTERNAL_MCOA0 = 1,
}
impl From<CCPA2_A> for bool {
    #[inline(always)]
    fn from(variant: CCPA2_A) -> Self {
        variant as u8 != 0
    }
}
impl CCPA2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCPA2_A {
        match self.bits {
            false => CCPA2_A::PASSIVE,
            true => CCPA2_A::INTERNAL_MCOA0,
        }
    }
    #[doc = "MCOA2 passive."]
    #[inline(always)]
    pub fn is_passive(&self) -> bool {
        *self == CCPA2_A::PASSIVE
    }
    #[doc = "MCOA2 tracks internal MCOA0."]
    #[inline(always)]
    pub fn is_internal_mcoa0(&self) -> bool {
        *self == CCPA2_A::INTERNAL_MCOA0
    }
}
#[doc = "Field `CCPA2` writer - Communication pattern output A, channel 2."]
pub type CCPA2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CCPA2_A>;
impl<'a, REG, const O: u8> CCPA2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MCOA2 passive."]
    #[inline(always)]
    pub fn passive(self) -> &'a mut crate::W<REG> {
        self.variant(CCPA2_A::PASSIVE)
    }
    #[doc = "MCOA2 tracks internal MCOA0."]
    #[inline(always)]
    pub fn internal_mcoa0(self) -> &'a mut crate::W<REG> {
        self.variant(CCPA2_A::INTERNAL_MCOA0)
    }
}
#[doc = "Field `CCPB2` reader - Communication pattern output B, channel 2."]
pub type CCPB2_R = crate::BitReader<CCPB2_A>;
#[doc = "Communication pattern output B, channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCPB2_A {
    #[doc = "0: MCOB2 passive."]
    PASSIVE = 0,
    #[doc = "1: MCOB2 tracks internal MCOA0."]
    INTERNAL_MCOA0 = 1,
}
impl From<CCPB2_A> for bool {
    #[inline(always)]
    fn from(variant: CCPB2_A) -> Self {
        variant as u8 != 0
    }
}
impl CCPB2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCPB2_A {
        match self.bits {
            false => CCPB2_A::PASSIVE,
            true => CCPB2_A::INTERNAL_MCOA0,
        }
    }
    #[doc = "MCOB2 passive."]
    #[inline(always)]
    pub fn is_passive(&self) -> bool {
        *self == CCPB2_A::PASSIVE
    }
    #[doc = "MCOB2 tracks internal MCOA0."]
    #[inline(always)]
    pub fn is_internal_mcoa0(&self) -> bool {
        *self == CCPB2_A::INTERNAL_MCOA0
    }
}
#[doc = "Field `CCPB2` writer - Communication pattern output B, channel 2."]
pub type CCPB2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CCPB2_A>;
impl<'a, REG, const O: u8> CCPB2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MCOB2 passive."]
    #[inline(always)]
    pub fn passive(self) -> &'a mut crate::W<REG> {
        self.variant(CCPB2_A::PASSIVE)
    }
    #[doc = "MCOB2 tracks internal MCOA0."]
    #[inline(always)]
    pub fn internal_mcoa0(self) -> &'a mut crate::W<REG> {
        self.variant(CCPB2_A::INTERNAL_MCOA0)
    }
}
impl R {
    #[doc = "Bit 0 - Communication pattern output A, channel 0."]
    #[inline(always)]
    pub fn ccpa0(&self) -> CCPA0_R {
        CCPA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Communication pattern output B, channel 0."]
    #[inline(always)]
    pub fn ccpb0(&self) -> CCPB0_R {
        CCPB0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Communication pattern output A, channel 1."]
    #[inline(always)]
    pub fn ccpa1(&self) -> CCPA1_R {
        CCPA1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Communication pattern output B, channel 1."]
    #[inline(always)]
    pub fn ccpb1(&self) -> CCPB1_R {
        CCPB1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Communication pattern output A, channel 2."]
    #[inline(always)]
    pub fn ccpa2(&self) -> CCPA2_R {
        CCPA2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Communication pattern output B, channel 2."]
    #[inline(always)]
    pub fn ccpb2(&self) -> CCPB2_R {
        CCPB2_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CP")
            .field("ccpa0", &format_args!("{}", self.ccpa0().bit()))
            .field("ccpb0", &format_args!("{}", self.ccpb0().bit()))
            .field("ccpa1", &format_args!("{}", self.ccpa1().bit()))
            .field("ccpb1", &format_args!("{}", self.ccpb1().bit()))
            .field("ccpa2", &format_args!("{}", self.ccpa2().bit()))
            .field("ccpb2", &format_args!("{}", self.ccpb2().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Communication pattern output A, channel 0."]
    #[inline(always)]
    #[must_use]
    pub fn ccpa0(&mut self) -> CCPA0_W<CP_SPEC, 0> {
        CCPA0_W::new(self)
    }
    #[doc = "Bit 1 - Communication pattern output B, channel 0."]
    #[inline(always)]
    #[must_use]
    pub fn ccpb0(&mut self) -> CCPB0_W<CP_SPEC, 1> {
        CCPB0_W::new(self)
    }
    #[doc = "Bit 2 - Communication pattern output A, channel 1."]
    #[inline(always)]
    #[must_use]
    pub fn ccpa1(&mut self) -> CCPA1_W<CP_SPEC, 2> {
        CCPA1_W::new(self)
    }
    #[doc = "Bit 3 - Communication pattern output B, channel 1."]
    #[inline(always)]
    #[must_use]
    pub fn ccpb1(&mut self) -> CCPB1_W<CP_SPEC, 3> {
        CCPB1_W::new(self)
    }
    #[doc = "Bit 4 - Communication pattern output A, channel 2."]
    #[inline(always)]
    #[must_use]
    pub fn ccpa2(&mut self) -> CCPA2_W<CP_SPEC, 4> {
        CCPA2_W::new(self)
    }
    #[doc = "Bit 5 - Communication pattern output B, channel 2."]
    #[inline(always)]
    #[must_use]
    pub fn ccpb2(&mut self) -> CCPB2_W<CP_SPEC, 5> {
        CCPB2_W::new(self)
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
#[doc = "Communication Pattern register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CP_SPEC;
impl crate::RegisterSpec for CP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cp::R`](R) reader structure"]
impl crate::Readable for CP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cp::W`](W) writer structure"]
impl crate::Writable for CP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CP to value 0"]
impl crate::Resettable for CP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
