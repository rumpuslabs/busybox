#[doc = "Register `CCR` reader"]
pub type R = crate::R<CCR_SPEC>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CCR_SPEC>;
#[doc = "Field `CAP0RE` reader - Capture on CAPn.0 rising edge"]
pub type CAP0RE_R = crate::BitReader<CAP0RE_A>;
#[doc = "Capture on CAPn.0 rising edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAP0RE_A {
    #[doc = "1: A sequence of 0 then 1 on CAPn.0 will cause CR0 to be loaded with the contents of TC."]
    ENABLE = 1,
    #[doc = "0: This feature is disabled."]
    DISABLE = 0,
}
impl From<CAP0RE_A> for bool {
    #[inline(always)]
    fn from(variant: CAP0RE_A) -> Self {
        variant as u8 != 0
    }
}
impl CAP0RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CAP0RE_A {
        match self.bits {
            true => CAP0RE_A::ENABLE,
            false => CAP0RE_A::DISABLE,
        }
    }
    #[doc = "A sequence of 0 then 1 on CAPn.0 will cause CR0 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CAP0RE_A::ENABLE
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CAP0RE_A::DISABLE
    }
}
#[doc = "Field `CAP0RE` writer - Capture on CAPn.0 rising edge"]
pub type CAP0RE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CAP0RE_A>;
impl<'a, REG, const O: u8> CAP0RE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A sequence of 0 then 1 on CAPn.0 will cause CR0 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CAP0RE_A::ENABLE)
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CAP0RE_A::DISABLE)
    }
}
#[doc = "Field `CAP0FE` reader - Capture on CAPn.0 falling edge"]
pub type CAP0FE_R = crate::BitReader<CAP0FE_A>;
#[doc = "Capture on CAPn.0 falling edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAP0FE_A {
    #[doc = "1: A sequence of 1 then 0 on CAPn.0 will cause CR0 to be loaded with the contents of TC."]
    ENABLE = 1,
    #[doc = "0: This feature is disabled."]
    DISABLE = 0,
}
impl From<CAP0FE_A> for bool {
    #[inline(always)]
    fn from(variant: CAP0FE_A) -> Self {
        variant as u8 != 0
    }
}
impl CAP0FE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CAP0FE_A {
        match self.bits {
            true => CAP0FE_A::ENABLE,
            false => CAP0FE_A::DISABLE,
        }
    }
    #[doc = "A sequence of 1 then 0 on CAPn.0 will cause CR0 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CAP0FE_A::ENABLE
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CAP0FE_A::DISABLE
    }
}
#[doc = "Field `CAP0FE` writer - Capture on CAPn.0 falling edge"]
pub type CAP0FE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CAP0FE_A>;
impl<'a, REG, const O: u8> CAP0FE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A sequence of 1 then 0 on CAPn.0 will cause CR0 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CAP0FE_A::ENABLE)
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CAP0FE_A::DISABLE)
    }
}
#[doc = "Field `CAP0I` reader - Interrupt on CAPn.0 event"]
pub type CAP0I_R = crate::BitReader<CAP0I_A>;
#[doc = "Interrupt on CAPn.0 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAP0I_A {
    #[doc = "1: A CR0 load due to a CAPn.0 event will generate an interrupt."]
    ENABLE = 1,
    #[doc = "0: This feature is disabled."]
    DISABLE = 0,
}
impl From<CAP0I_A> for bool {
    #[inline(always)]
    fn from(variant: CAP0I_A) -> Self {
        variant as u8 != 0
    }
}
impl CAP0I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CAP0I_A {
        match self.bits {
            true => CAP0I_A::ENABLE,
            false => CAP0I_A::DISABLE,
        }
    }
    #[doc = "A CR0 load due to a CAPn.0 event will generate an interrupt."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CAP0I_A::ENABLE
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CAP0I_A::DISABLE
    }
}
#[doc = "Field `CAP0I` writer - Interrupt on CAPn.0 event"]
pub type CAP0I_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CAP0I_A>;
impl<'a, REG, const O: u8> CAP0I_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A CR0 load due to a CAPn.0 event will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CAP0I_A::ENABLE)
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CAP0I_A::DISABLE)
    }
}
#[doc = "Field `CAP1RE` reader - Capture on CAPn.1 rising edge"]
pub type CAP1RE_R = crate::BitReader<CAP1RE_A>;
#[doc = "Capture on CAPn.1 rising edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAP1RE_A {
    #[doc = "1: A sequence of 0 then 1 on CAPn.1 will cause CR1 to be loaded with the contents of TC."]
    ENABLE = 1,
    #[doc = "0: This feature is disabled."]
    DISABLE = 0,
}
impl From<CAP1RE_A> for bool {
    #[inline(always)]
    fn from(variant: CAP1RE_A) -> Self {
        variant as u8 != 0
    }
}
impl CAP1RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CAP1RE_A {
        match self.bits {
            true => CAP1RE_A::ENABLE,
            false => CAP1RE_A::DISABLE,
        }
    }
    #[doc = "A sequence of 0 then 1 on CAPn.1 will cause CR1 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CAP1RE_A::ENABLE
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CAP1RE_A::DISABLE
    }
}
#[doc = "Field `CAP1RE` writer - Capture on CAPn.1 rising edge"]
pub type CAP1RE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CAP1RE_A>;
impl<'a, REG, const O: u8> CAP1RE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A sequence of 0 then 1 on CAPn.1 will cause CR1 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CAP1RE_A::ENABLE)
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CAP1RE_A::DISABLE)
    }
}
#[doc = "Field `CAP1FE` reader - Capture on CAPn.1 falling edge"]
pub type CAP1FE_R = crate::BitReader<CAP1FE_A>;
#[doc = "Capture on CAPn.1 falling edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAP1FE_A {
    #[doc = "1: A sequence of 1 then 0 on CAPn.1 will cause CR1 to be loaded with the contents of TC."]
    ENABLE = 1,
    #[doc = "0: This feature is disabled."]
    DISABLE = 0,
}
impl From<CAP1FE_A> for bool {
    #[inline(always)]
    fn from(variant: CAP1FE_A) -> Self {
        variant as u8 != 0
    }
}
impl CAP1FE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CAP1FE_A {
        match self.bits {
            true => CAP1FE_A::ENABLE,
            false => CAP1FE_A::DISABLE,
        }
    }
    #[doc = "A sequence of 1 then 0 on CAPn.1 will cause CR1 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CAP1FE_A::ENABLE
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CAP1FE_A::DISABLE
    }
}
#[doc = "Field `CAP1FE` writer - Capture on CAPn.1 falling edge"]
pub type CAP1FE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CAP1FE_A>;
impl<'a, REG, const O: u8> CAP1FE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A sequence of 1 then 0 on CAPn.1 will cause CR1 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CAP1FE_A::ENABLE)
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CAP1FE_A::DISABLE)
    }
}
#[doc = "Field `CAP1I` reader - Interrupt on CAPn.1 event"]
pub type CAP1I_R = crate::BitReader<CAP1I_A>;
#[doc = "Interrupt on CAPn.1 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAP1I_A {
    #[doc = "1: A CR1 load due to a CAPn.1 event will generate an interrupt."]
    ENABLE = 1,
    #[doc = "0: This feature is disabled."]
    DISABLE = 0,
}
impl From<CAP1I_A> for bool {
    #[inline(always)]
    fn from(variant: CAP1I_A) -> Self {
        variant as u8 != 0
    }
}
impl CAP1I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CAP1I_A {
        match self.bits {
            true => CAP1I_A::ENABLE,
            false => CAP1I_A::DISABLE,
        }
    }
    #[doc = "A CR1 load due to a CAPn.1 event will generate an interrupt."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CAP1I_A::ENABLE
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CAP1I_A::DISABLE
    }
}
#[doc = "Field `CAP1I` writer - Interrupt on CAPn.1 event"]
pub type CAP1I_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CAP1I_A>;
impl<'a, REG, const O: u8> CAP1I_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A CR1 load due to a CAPn.1 event will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CAP1I_A::ENABLE)
    }
    #[doc = "This feature is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CAP1I_A::DISABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Capture on CAPn.0 rising edge"]
    #[inline(always)]
    pub fn cap0re(&self) -> CAP0RE_R {
        CAP0RE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture on CAPn.0 falling edge"]
    #[inline(always)]
    pub fn cap0fe(&self) -> CAP0FE_R {
        CAP0FE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt on CAPn.0 event"]
    #[inline(always)]
    pub fn cap0i(&self) -> CAP0I_R {
        CAP0I_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture on CAPn.1 rising edge"]
    #[inline(always)]
    pub fn cap1re(&self) -> CAP1RE_R {
        CAP1RE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture on CAPn.1 falling edge"]
    #[inline(always)]
    pub fn cap1fe(&self) -> CAP1FE_R {
        CAP1FE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt on CAPn.1 event"]
    #[inline(always)]
    pub fn cap1i(&self) -> CAP1I_R {
        CAP1I_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR")
            .field("cap0re", &format_args!("{}", self.cap0re().bit()))
            .field("cap0fe", &format_args!("{}", self.cap0fe().bit()))
            .field("cap0i", &format_args!("{}", self.cap0i().bit()))
            .field("cap1re", &format_args!("{}", self.cap1re().bit()))
            .field("cap1fe", &format_args!("{}", self.cap1fe().bit()))
            .field("cap1i", &format_args!("{}", self.cap1i().bit()))
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
    #[doc = "Bit 0 - Capture on CAPn.0 rising edge"]
    #[inline(always)]
    #[must_use]
    pub fn cap0re(&mut self) -> CAP0RE_W<CCR_SPEC, 0> {
        CAP0RE_W::new(self)
    }
    #[doc = "Bit 1 - Capture on CAPn.0 falling edge"]
    #[inline(always)]
    #[must_use]
    pub fn cap0fe(&mut self) -> CAP0FE_W<CCR_SPEC, 1> {
        CAP0FE_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt on CAPn.0 event"]
    #[inline(always)]
    #[must_use]
    pub fn cap0i(&mut self) -> CAP0I_W<CCR_SPEC, 2> {
        CAP0I_W::new(self)
    }
    #[doc = "Bit 3 - Capture on CAPn.1 rising edge"]
    #[inline(always)]
    #[must_use]
    pub fn cap1re(&mut self) -> CAP1RE_W<CCR_SPEC, 3> {
        CAP1RE_W::new(self)
    }
    #[doc = "Bit 4 - Capture on CAPn.1 falling edge"]
    #[inline(always)]
    #[must_use]
    pub fn cap1fe(&mut self) -> CAP1FE_W<CCR_SPEC, 4> {
        CAP1FE_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt on CAPn.1 event"]
    #[inline(always)]
    #[must_use]
    pub fn cap1i(&mut self) -> CAP1I_W<CCR_SPEC, 5> {
        CAP1I_W::new(self)
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
#[doc = "Capture Control Register. The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated when a capture takes place.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
