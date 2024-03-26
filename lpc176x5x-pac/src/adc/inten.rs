#[doc = "Register `INTEN` reader"]
pub type R = crate::R<INTEN_SPEC>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<INTEN_SPEC>;
#[doc = "Field `ADINTEN0` reader - Interrupt enable"]
pub type ADINTEN0_R = crate::BitReader<ADINTEN0_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADINTEN0_A {
    #[doc = "0: Completion of a conversion on ADC channel 0 will not generate an interrupt."]
    DISABLE = 0,
    #[doc = "1: Completion of a conversion on ADC channel 0 will generate an interrupt."]
    ENABLE = 1,
}
impl From<ADINTEN0_A> for bool {
    #[inline(always)]
    fn from(variant: ADINTEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl ADINTEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADINTEN0_A {
        match self.bits {
            false => ADINTEN0_A::DISABLE,
            true => ADINTEN0_A::ENABLE,
        }
    }
    #[doc = "Completion of a conversion on ADC channel 0 will not generate an interrupt."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADINTEN0_A::DISABLE
    }
    #[doc = "Completion of a conversion on ADC channel 0 will generate an interrupt."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADINTEN0_A::ENABLE
    }
}
#[doc = "Field `ADINTEN0` writer - Interrupt enable"]
pub type ADINTEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADINTEN0_A>;
impl<'a, REG, const O: u8> ADINTEN0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Completion of a conversion on ADC channel 0 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ADINTEN0_A::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 0 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ADINTEN0_A::ENABLE)
    }
}
#[doc = "Field `ADINTEN1` reader - Interrupt enable"]
pub type ADINTEN1_R = crate::BitReader<ADINTEN1_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADINTEN1_A {
    #[doc = "0: Completion of a conversion on ADC channel 1 will not generate an interrupt."]
    DISABLE = 0,
    #[doc = "1: Completion of a conversion on ADC channel 1 will generate an interrupt."]
    ENABLE = 1,
}
impl From<ADINTEN1_A> for bool {
    #[inline(always)]
    fn from(variant: ADINTEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl ADINTEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADINTEN1_A {
        match self.bits {
            false => ADINTEN1_A::DISABLE,
            true => ADINTEN1_A::ENABLE,
        }
    }
    #[doc = "Completion of a conversion on ADC channel 1 will not generate an interrupt."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADINTEN1_A::DISABLE
    }
    #[doc = "Completion of a conversion on ADC channel 1 will generate an interrupt."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADINTEN1_A::ENABLE
    }
}
#[doc = "Field `ADINTEN1` writer - Interrupt enable"]
pub type ADINTEN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADINTEN1_A>;
impl<'a, REG, const O: u8> ADINTEN1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Completion of a conversion on ADC channel 1 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ADINTEN1_A::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 1 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ADINTEN1_A::ENABLE)
    }
}
#[doc = "Field `ADINTEN2` reader - Interrupt enable"]
pub type ADINTEN2_R = crate::BitReader<ADINTEN2_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADINTEN2_A {
    #[doc = "0: Completion of a conversion on ADC channel 2 will not generate an interrupt."]
    DISABLE = 0,
    #[doc = "1: Completion of a conversion on ADC channel 2 will generate an interrupt."]
    ENABLE = 1,
}
impl From<ADINTEN2_A> for bool {
    #[inline(always)]
    fn from(variant: ADINTEN2_A) -> Self {
        variant as u8 != 0
    }
}
impl ADINTEN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADINTEN2_A {
        match self.bits {
            false => ADINTEN2_A::DISABLE,
            true => ADINTEN2_A::ENABLE,
        }
    }
    #[doc = "Completion of a conversion on ADC channel 2 will not generate an interrupt."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADINTEN2_A::DISABLE
    }
    #[doc = "Completion of a conversion on ADC channel 2 will generate an interrupt."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADINTEN2_A::ENABLE
    }
}
#[doc = "Field `ADINTEN2` writer - Interrupt enable"]
pub type ADINTEN2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADINTEN2_A>;
impl<'a, REG, const O: u8> ADINTEN2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Completion of a conversion on ADC channel 2 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ADINTEN2_A::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 2 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ADINTEN2_A::ENABLE)
    }
}
#[doc = "Field `ADINTEN3` reader - Interrupt enable"]
pub type ADINTEN3_R = crate::BitReader<ADINTEN3_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADINTEN3_A {
    #[doc = "0: Completion of a conversion on ADC channel 3 will not generate an interrupt."]
    DISABLE = 0,
    #[doc = "1: Completion of a conversion on ADC channel 3 will generate an interrupt."]
    ENABLE = 1,
}
impl From<ADINTEN3_A> for bool {
    #[inline(always)]
    fn from(variant: ADINTEN3_A) -> Self {
        variant as u8 != 0
    }
}
impl ADINTEN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADINTEN3_A {
        match self.bits {
            false => ADINTEN3_A::DISABLE,
            true => ADINTEN3_A::ENABLE,
        }
    }
    #[doc = "Completion of a conversion on ADC channel 3 will not generate an interrupt."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADINTEN3_A::DISABLE
    }
    #[doc = "Completion of a conversion on ADC channel 3 will generate an interrupt."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADINTEN3_A::ENABLE
    }
}
#[doc = "Field `ADINTEN3` writer - Interrupt enable"]
pub type ADINTEN3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADINTEN3_A>;
impl<'a, REG, const O: u8> ADINTEN3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Completion of a conversion on ADC channel 3 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ADINTEN3_A::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 3 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ADINTEN3_A::ENABLE)
    }
}
#[doc = "Field `ADINTEN4` reader - Interrupt enable"]
pub type ADINTEN4_R = crate::BitReader<ADINTEN4_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADINTEN4_A {
    #[doc = "0: Completion of a conversion on ADC channel 4 will not generate an interrupt."]
    DISABLE = 0,
    #[doc = "1: Completion of a conversion on ADC channel 4 will generate an interrupt."]
    ENABLE = 1,
}
impl From<ADINTEN4_A> for bool {
    #[inline(always)]
    fn from(variant: ADINTEN4_A) -> Self {
        variant as u8 != 0
    }
}
impl ADINTEN4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADINTEN4_A {
        match self.bits {
            false => ADINTEN4_A::DISABLE,
            true => ADINTEN4_A::ENABLE,
        }
    }
    #[doc = "Completion of a conversion on ADC channel 4 will not generate an interrupt."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADINTEN4_A::DISABLE
    }
    #[doc = "Completion of a conversion on ADC channel 4 will generate an interrupt."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADINTEN4_A::ENABLE
    }
}
#[doc = "Field `ADINTEN4` writer - Interrupt enable"]
pub type ADINTEN4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADINTEN4_A>;
impl<'a, REG, const O: u8> ADINTEN4_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Completion of a conversion on ADC channel 4 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ADINTEN4_A::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 4 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ADINTEN4_A::ENABLE)
    }
}
#[doc = "Field `ADINTEN5` reader - Interrupt enable"]
pub type ADINTEN5_R = crate::BitReader<ADINTEN5_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADINTEN5_A {
    #[doc = "0: Completion of a conversion on ADC channel 5 will not generate an interrupt."]
    DISABLE = 0,
    #[doc = "1: Completion of a conversion on ADC channel 5 will generate an interrupt."]
    ENABLE = 1,
}
impl From<ADINTEN5_A> for bool {
    #[inline(always)]
    fn from(variant: ADINTEN5_A) -> Self {
        variant as u8 != 0
    }
}
impl ADINTEN5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADINTEN5_A {
        match self.bits {
            false => ADINTEN5_A::DISABLE,
            true => ADINTEN5_A::ENABLE,
        }
    }
    #[doc = "Completion of a conversion on ADC channel 5 will not generate an interrupt."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADINTEN5_A::DISABLE
    }
    #[doc = "Completion of a conversion on ADC channel 5 will generate an interrupt."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADINTEN5_A::ENABLE
    }
}
#[doc = "Field `ADINTEN5` writer - Interrupt enable"]
pub type ADINTEN5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADINTEN5_A>;
impl<'a, REG, const O: u8> ADINTEN5_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Completion of a conversion on ADC channel 5 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ADINTEN5_A::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 5 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ADINTEN5_A::ENABLE)
    }
}
#[doc = "Field `ADINTEN6` reader - Interrupt enable"]
pub type ADINTEN6_R = crate::BitReader<ADINTEN6_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADINTEN6_A {
    #[doc = "0: Completion of a conversion on ADC channel 6 will not generate an interrupt."]
    DISABLE = 0,
    #[doc = "1: Completion of a conversion on ADC channel 6 will generate an interrupt."]
    ENABLE = 1,
}
impl From<ADINTEN6_A> for bool {
    #[inline(always)]
    fn from(variant: ADINTEN6_A) -> Self {
        variant as u8 != 0
    }
}
impl ADINTEN6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADINTEN6_A {
        match self.bits {
            false => ADINTEN6_A::DISABLE,
            true => ADINTEN6_A::ENABLE,
        }
    }
    #[doc = "Completion of a conversion on ADC channel 6 will not generate an interrupt."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADINTEN6_A::DISABLE
    }
    #[doc = "Completion of a conversion on ADC channel 6 will generate an interrupt."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADINTEN6_A::ENABLE
    }
}
#[doc = "Field `ADINTEN6` writer - Interrupt enable"]
pub type ADINTEN6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADINTEN6_A>;
impl<'a, REG, const O: u8> ADINTEN6_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Completion of a conversion on ADC channel 6 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ADINTEN6_A::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 6 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ADINTEN6_A::ENABLE)
    }
}
#[doc = "Field `ADINTEN7` reader - Interrupt enable"]
pub type ADINTEN7_R = crate::BitReader<ADINTEN7_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADINTEN7_A {
    #[doc = "0: Completion of a conversion on ADC channel 7 will not generate an interrupt."]
    DISABLE = 0,
    #[doc = "1: Completion of a conversion on ADC channel 7 will generate an interrupt."]
    ENABLE = 1,
}
impl From<ADINTEN7_A> for bool {
    #[inline(always)]
    fn from(variant: ADINTEN7_A) -> Self {
        variant as u8 != 0
    }
}
impl ADINTEN7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADINTEN7_A {
        match self.bits {
            false => ADINTEN7_A::DISABLE,
            true => ADINTEN7_A::ENABLE,
        }
    }
    #[doc = "Completion of a conversion on ADC channel 7 will not generate an interrupt."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADINTEN7_A::DISABLE
    }
    #[doc = "Completion of a conversion on ADC channel 7 will generate an interrupt."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADINTEN7_A::ENABLE
    }
}
#[doc = "Field `ADINTEN7` writer - Interrupt enable"]
pub type ADINTEN7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADINTEN7_A>;
impl<'a, REG, const O: u8> ADINTEN7_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Completion of a conversion on ADC channel 7 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ADINTEN7_A::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 7 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ADINTEN7_A::ENABLE)
    }
}
#[doc = "Field `ADGINTEN` reader - Interrupt enable"]
pub type ADGINTEN_R = crate::BitReader<ADGINTEN_A>;
#[doc = "Interrupt enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADGINTEN_A {
    #[doc = "0: Only the individual ADC channels enabled by ADINTEN7:0 will generate interrupts."]
    CHANNELS = 0,
    #[doc = "1: The global DONE flag in ADDR is enabled to generate an interrupt in addition to any individual ADC channels that are enabled to generate interrupts."]
    GLOBAL = 1,
}
impl From<ADGINTEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADGINTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADGINTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADGINTEN_A {
        match self.bits {
            false => ADGINTEN_A::CHANNELS,
            true => ADGINTEN_A::GLOBAL,
        }
    }
    #[doc = "Only the individual ADC channels enabled by ADINTEN7:0 will generate interrupts."]
    #[inline(always)]
    pub fn is_channels(&self) -> bool {
        *self == ADGINTEN_A::CHANNELS
    }
    #[doc = "The global DONE flag in ADDR is enabled to generate an interrupt in addition to any individual ADC channels that are enabled to generate interrupts."]
    #[inline(always)]
    pub fn is_global(&self) -> bool {
        *self == ADGINTEN_A::GLOBAL
    }
}
#[doc = "Field `ADGINTEN` writer - Interrupt enable"]
pub type ADGINTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADGINTEN_A>;
impl<'a, REG, const O: u8> ADGINTEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Only the individual ADC channels enabled by ADINTEN7:0 will generate interrupts."]
    #[inline(always)]
    pub fn channels(self) -> &'a mut crate::W<REG> {
        self.variant(ADGINTEN_A::CHANNELS)
    }
    #[doc = "The global DONE flag in ADDR is enabled to generate an interrupt in addition to any individual ADC channels that are enabled to generate interrupts."]
    #[inline(always)]
    pub fn global(self) -> &'a mut crate::W<REG> {
        self.variant(ADGINTEN_A::GLOBAL)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten0(&self) -> ADINTEN0_R {
        ADINTEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten1(&self) -> ADINTEN1_R {
        ADINTEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten2(&self) -> ADINTEN2_R {
        ADINTEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten3(&self) -> ADINTEN3_R {
        ADINTEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten4(&self) -> ADINTEN4_R {
        ADINTEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten5(&self) -> ADINTEN5_R {
        ADINTEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten6(&self) -> ADINTEN6_R {
        ADINTEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten7(&self) -> ADINTEN7_R {
        ADINTEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt enable"]
    #[inline(always)]
    pub fn adginten(&self) -> ADGINTEN_R {
        ADGINTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTEN")
            .field("adinten0", &format_args!("{}", self.adinten0().bit()))
            .field("adinten1", &format_args!("{}", self.adinten1().bit()))
            .field("adinten2", &format_args!("{}", self.adinten2().bit()))
            .field("adinten3", &format_args!("{}", self.adinten3().bit()))
            .field("adinten4", &format_args!("{}", self.adinten4().bit()))
            .field("adinten5", &format_args!("{}", self.adinten5().bit()))
            .field("adinten6", &format_args!("{}", self.adinten6().bit()))
            .field("adinten7", &format_args!("{}", self.adinten7().bit()))
            .field("adginten", &format_args!("{}", self.adginten().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<INTEN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn adinten0(&mut self) -> ADINTEN0_W<INTEN_SPEC, 0> {
        ADINTEN0_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn adinten1(&mut self) -> ADINTEN1_W<INTEN_SPEC, 1> {
        ADINTEN1_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn adinten2(&mut self) -> ADINTEN2_W<INTEN_SPEC, 2> {
        ADINTEN2_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn adinten3(&mut self) -> ADINTEN3_W<INTEN_SPEC, 3> {
        ADINTEN3_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn adinten4(&mut self) -> ADINTEN4_W<INTEN_SPEC, 4> {
        ADINTEN4_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn adinten5(&mut self) -> ADINTEN5_W<INTEN_SPEC, 5> {
        ADINTEN5_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn adinten6(&mut self) -> ADINTEN6_W<INTEN_SPEC, 6> {
        ADINTEN6_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn adinten7(&mut self) -> ADINTEN7_W<INTEN_SPEC, 7> {
        ADINTEN7_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn adginten(&mut self) -> ADGINTEN_W<INTEN_SPEC, 8> {
        ADGINTEN_W::new(self)
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
#[doc = "A/D Interrupt Enable Register. This register contains enable bits that allow the DONE flag of each A/D channel to be included or excluded from contributing to the generation of an A/D interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for INTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTEN to value 0x0100"]
impl crate::Resettable for INTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
