#[doc = "Register `DMAINTEN` reader"]
pub type R = crate::R<DMAINTEN_SPEC>;
#[doc = "Register `DMAINTEN` writer"]
pub type W = crate::W<DMAINTEN_SPEC>;
#[doc = "Field `EOT` reader - End of Transfer Interrupt enable bit."]
pub type EOT_R = crate::BitReader<EOT_A>;
#[doc = "End of Transfer Interrupt enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOT_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<EOT_A> for bool {
    #[inline(always)]
    fn from(variant: EOT_A) -> Self {
        variant as u8 != 0
    }
}
impl EOT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOT_A {
        match self.bits {
            false => EOT_A::DISABLED,
            true => EOT_A::ENABLED,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOT_A::DISABLED
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOT_A::ENABLED
    }
}
#[doc = "Field `EOT` writer - End of Transfer Interrupt enable bit."]
pub type EOT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, EOT_A>;
impl<'a, REG, const O: u8> EOT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOT_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOT_A::ENABLED)
    }
}
#[doc = "Field `NDDR` reader - New DD Request Interrupt enable bit."]
pub type NDDR_R = crate::BitReader<NDDR_A>;
#[doc = "New DD Request Interrupt enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NDDR_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<NDDR_A> for bool {
    #[inline(always)]
    fn from(variant: NDDR_A) -> Self {
        variant as u8 != 0
    }
}
impl NDDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NDDR_A {
        match self.bits {
            false => NDDR_A::DISABLED,
            true => NDDR_A::ENABLED,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NDDR_A::DISABLED
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NDDR_A::ENABLED
    }
}
#[doc = "Field `NDDR` writer - New DD Request Interrupt enable bit."]
pub type NDDR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, NDDR_A>;
impl<'a, REG, const O: u8> NDDR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(NDDR_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(NDDR_A::ENABLED)
    }
}
#[doc = "Field `ERR` reader - System Error Interrupt enable bit."]
pub type ERR_R = crate::BitReader<ERR_A>;
#[doc = "System Error Interrupt enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<ERR_A> for bool {
    #[inline(always)]
    fn from(variant: ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERR_A {
        match self.bits {
            false => ERR_A::DISABLED,
            true => ERR_A::ENABLED,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERR_A::DISABLED
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERR_A::ENABLED
    }
}
#[doc = "Field `ERR` writer - System Error Interrupt enable bit."]
pub type ERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ERR_A>;
impl<'a, REG, const O: u8> ERR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERR_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERR_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - End of Transfer Interrupt enable bit."]
    #[inline(always)]
    pub fn eot(&self) -> EOT_R {
        EOT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - New DD Request Interrupt enable bit."]
    #[inline(always)]
    pub fn nddr(&self) -> NDDR_R {
        NDDR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - System Error Interrupt enable bit."]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAINTEN")
            .field("eot", &format_args!("{}", self.eot().bit()))
            .field("nddr", &format_args!("{}", self.nddr().bit()))
            .field("err", &format_args!("{}", self.err().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<DMAINTEN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - End of Transfer Interrupt enable bit."]
    #[inline(always)]
    #[must_use]
    pub fn eot(&mut self) -> EOT_W<DMAINTEN_SPEC, 0> {
        EOT_W::new(self)
    }
    #[doc = "Bit 1 - New DD Request Interrupt enable bit."]
    #[inline(always)]
    #[must_use]
    pub fn nddr(&mut self) -> NDDR_W<DMAINTEN_SPEC, 1> {
        NDDR_W::new(self)
    }
    #[doc = "Bit 2 - System Error Interrupt enable bit."]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ERR_W<DMAINTEN_SPEC, 2> {
        ERR_W::new(self)
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
#[doc = "USB DMA Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmainten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmainten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAINTEN_SPEC;
impl crate::RegisterSpec for DMAINTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmainten::R`](R) reader structure"]
impl crate::Readable for DMAINTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmainten::W`](W) writer structure"]
impl crate::Writable for DMAINTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAINTEN to value 0"]
impl crate::Resettable for DMAINTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
