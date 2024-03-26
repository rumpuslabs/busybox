#[doc = "Register `EMR` reader"]
pub type R = crate::R<EMR_SPEC>;
#[doc = "Register `EMR` writer"]
pub type W = crate::W<EMR_SPEC>;
#[doc = "Field `EM0` reader - External Match 0. When a match occurs between the TC and MR0, this bit can either toggle, go low, go high, or do nothing, depending on bits 5:4 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
pub type EM0_R = crate::BitReader;
#[doc = "Field `EM0` writer - External Match 0. When a match occurs between the TC and MR0, this bit can either toggle, go low, go high, or do nothing, depending on bits 5:4 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
pub type EM0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EM1` reader - External Match 1. When a match occurs between the TC and MR1, this bit can either toggle, go low, go high, or do nothing, depending on bits 7:6 of this register. This bit can be driven onto a MATn.1 pin, in a positive-logic manner (0 = low, 1 = high)."]
pub type EM1_R = crate::BitReader;
#[doc = "Field `EM1` writer - External Match 1. When a match occurs between the TC and MR1, this bit can either toggle, go low, go high, or do nothing, depending on bits 7:6 of this register. This bit can be driven onto a MATn.1 pin, in a positive-logic manner (0 = low, 1 = high)."]
pub type EM1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EM2` reader - External Match 2. When a match occurs between the TC and MR2, this bit can either toggle, go low, go high, or do nothing, depending on bits 9:8 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
pub type EM2_R = crate::BitReader;
#[doc = "Field `EM2` writer - External Match 2. When a match occurs between the TC and MR2, this bit can either toggle, go low, go high, or do nothing, depending on bits 9:8 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
pub type EM2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EM3` reader - External Match 3. When a match occurs between the TC and MR3, this bit can either toggle, go low, go high, or do nothing, depending on bits 11:10 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
pub type EM3_R = crate::BitReader;
#[doc = "Field `EM3` writer - External Match 3. When a match occurs between the TC and MR3, this bit can either toggle, go low, go high, or do nothing, depending on bits 11:10 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
pub type EM3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EMC0` reader - External Match Control 0. Determines the functionality of External Match 0."]
pub type EMC0_R = crate::FieldReader<EMC0_A>;
#[doc = "External Match Control 0. Determines the functionality of External Match 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EMC0_A {
    #[doc = "0: Do Nothing."]
    DO_NOTHING = 0,
    #[doc = "1: Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    CLEAR = 1,
    #[doc = "2: Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    SET = 2,
    #[doc = "3: Toggle the corresponding External Match bit/output."]
    TOGGLE = 3,
}
impl From<EMC0_A> for u8 {
    #[inline(always)]
    fn from(variant: EMC0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EMC0_A {
    type Ux = u8;
}
impl EMC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EMC0_A {
        match self.bits {
            0 => EMC0_A::DO_NOTHING,
            1 => EMC0_A::CLEAR,
            2 => EMC0_A::SET,
            3 => EMC0_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn is_do_nothing(&self) -> bool {
        *self == EMC0_A::DO_NOTHING
    }
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == EMC0_A::CLEAR
    }
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == EMC0_A::SET
    }
    #[doc = "Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == EMC0_A::TOGGLE
    }
}
#[doc = "Field `EMC0` writer - External Match Control 0. Determines the functionality of External Match 0."]
pub type EMC0_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, EMC0_A>;
impl<'a, REG, const O: u8> EMC0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut crate::W<REG> {
        self.variant(EMC0_A::DO_NOTHING)
    }
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EMC0_A::CLEAR)
    }
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(EMC0_A::SET)
    }
    #[doc = "Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(EMC0_A::TOGGLE)
    }
}
#[doc = "Field `EMC1` reader - External Match Control 1. Determines the functionality of External Match 1."]
pub type EMC1_R = crate::FieldReader<EMC1_A>;
#[doc = "External Match Control 1. Determines the functionality of External Match 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EMC1_A {
    #[doc = "0: Do Nothing."]
    DO_NOTHING = 0,
    #[doc = "1: Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    CLEAR = 1,
    #[doc = "2: Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    SET = 2,
    #[doc = "3: Toggle the corresponding External Match bit/output."]
    TOGGLE = 3,
}
impl From<EMC1_A> for u8 {
    #[inline(always)]
    fn from(variant: EMC1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EMC1_A {
    type Ux = u8;
}
impl EMC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EMC1_A {
        match self.bits {
            0 => EMC1_A::DO_NOTHING,
            1 => EMC1_A::CLEAR,
            2 => EMC1_A::SET,
            3 => EMC1_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn is_do_nothing(&self) -> bool {
        *self == EMC1_A::DO_NOTHING
    }
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == EMC1_A::CLEAR
    }
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == EMC1_A::SET
    }
    #[doc = "Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == EMC1_A::TOGGLE
    }
}
#[doc = "Field `EMC1` writer - External Match Control 1. Determines the functionality of External Match 1."]
pub type EMC1_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, EMC1_A>;
impl<'a, REG, const O: u8> EMC1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut crate::W<REG> {
        self.variant(EMC1_A::DO_NOTHING)
    }
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EMC1_A::CLEAR)
    }
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(EMC1_A::SET)
    }
    #[doc = "Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(EMC1_A::TOGGLE)
    }
}
#[doc = "Field `EMC2` reader - External Match Control 2. Determines the functionality of External Match 2."]
pub type EMC2_R = crate::FieldReader<EMC2_A>;
#[doc = "External Match Control 2. Determines the functionality of External Match 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EMC2_A {
    #[doc = "0: Do Nothing."]
    DO_NOTHING = 0,
    #[doc = "1: Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    CLEAR = 1,
    #[doc = "2: Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    SET = 2,
    #[doc = "3: Toggle the corresponding External Match bit/output."]
    TOGGLE = 3,
}
impl From<EMC2_A> for u8 {
    #[inline(always)]
    fn from(variant: EMC2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EMC2_A {
    type Ux = u8;
}
impl EMC2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EMC2_A {
        match self.bits {
            0 => EMC2_A::DO_NOTHING,
            1 => EMC2_A::CLEAR,
            2 => EMC2_A::SET,
            3 => EMC2_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn is_do_nothing(&self) -> bool {
        *self == EMC2_A::DO_NOTHING
    }
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == EMC2_A::CLEAR
    }
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == EMC2_A::SET
    }
    #[doc = "Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == EMC2_A::TOGGLE
    }
}
#[doc = "Field `EMC2` writer - External Match Control 2. Determines the functionality of External Match 2."]
pub type EMC2_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, EMC2_A>;
impl<'a, REG, const O: u8> EMC2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut crate::W<REG> {
        self.variant(EMC2_A::DO_NOTHING)
    }
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EMC2_A::CLEAR)
    }
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(EMC2_A::SET)
    }
    #[doc = "Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(EMC2_A::TOGGLE)
    }
}
#[doc = "Field `EMC3` reader - External Match Control 3. Determines the functionality of External Match 3."]
pub type EMC3_R = crate::FieldReader<EMC3_A>;
#[doc = "External Match Control 3. Determines the functionality of External Match 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EMC3_A {
    #[doc = "0: Do Nothing."]
    DO_NOTHING = 0,
    #[doc = "1: Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    CLEAR = 1,
    #[doc = "2: Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    SET = 2,
    #[doc = "3: Toggle the corresponding External Match bit/output."]
    TOGGLE = 3,
}
impl From<EMC3_A> for u8 {
    #[inline(always)]
    fn from(variant: EMC3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EMC3_A {
    type Ux = u8;
}
impl EMC3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EMC3_A {
        match self.bits {
            0 => EMC3_A::DO_NOTHING,
            1 => EMC3_A::CLEAR,
            2 => EMC3_A::SET,
            3 => EMC3_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn is_do_nothing(&self) -> bool {
        *self == EMC3_A::DO_NOTHING
    }
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == EMC3_A::CLEAR
    }
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == EMC3_A::SET
    }
    #[doc = "Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == EMC3_A::TOGGLE
    }
}
#[doc = "Field `EMC3` writer - External Match Control 3. Determines the functionality of External Match 3."]
pub type EMC3_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, EMC3_A>;
impl<'a, REG, const O: u8> EMC3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut crate::W<REG> {
        self.variant(EMC3_A::DO_NOTHING)
    }
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EMC3_A::CLEAR)
    }
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(EMC3_A::SET)
    }
    #[doc = "Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(EMC3_A::TOGGLE)
    }
}
impl R {
    #[doc = "Bit 0 - External Match 0. When a match occurs between the TC and MR0, this bit can either toggle, go low, go high, or do nothing, depending on bits 5:4 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    pub fn em0(&self) -> EM0_R {
        EM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Match 1. When a match occurs between the TC and MR1, this bit can either toggle, go low, go high, or do nothing, depending on bits 7:6 of this register. This bit can be driven onto a MATn.1 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    pub fn em1(&self) -> EM1_R {
        EM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Match 2. When a match occurs between the TC and MR2, this bit can either toggle, go low, go high, or do nothing, depending on bits 9:8 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    pub fn em2(&self) -> EM2_R {
        EM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External Match 3. When a match occurs between the TC and MR3, this bit can either toggle, go low, go high, or do nothing, depending on bits 11:10 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    pub fn em3(&self) -> EM3_R {
        EM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - External Match Control 0. Determines the functionality of External Match 0."]
    #[inline(always)]
    pub fn emc0(&self) -> EMC0_R {
        EMC0_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - External Match Control 1. Determines the functionality of External Match 1."]
    #[inline(always)]
    pub fn emc1(&self) -> EMC1_R {
        EMC1_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - External Match Control 2. Determines the functionality of External Match 2."]
    #[inline(always)]
    pub fn emc2(&self) -> EMC2_R {
        EMC2_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - External Match Control 3. Determines the functionality of External Match 3."]
    #[inline(always)]
    pub fn emc3(&self) -> EMC3_R {
        EMC3_R::new(((self.bits >> 10) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMR")
            .field("em0", &format_args!("{}", self.em0().bit()))
            .field("em1", &format_args!("{}", self.em1().bit()))
            .field("em2", &format_args!("{}", self.em2().bit()))
            .field("em3", &format_args!("{}", self.em3().bit()))
            .field("emc0", &format_args!("{}", self.emc0().bits()))
            .field("emc1", &format_args!("{}", self.emc1().bits()))
            .field("emc2", &format_args!("{}", self.emc2().bits()))
            .field("emc3", &format_args!("{}", self.emc3().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<EMR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - External Match 0. When a match occurs between the TC and MR0, this bit can either toggle, go low, go high, or do nothing, depending on bits 5:4 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    #[must_use]
    pub fn em0(&mut self) -> EM0_W<EMR_SPEC, 0> {
        EM0_W::new(self)
    }
    #[doc = "Bit 1 - External Match 1. When a match occurs between the TC and MR1, this bit can either toggle, go low, go high, or do nothing, depending on bits 7:6 of this register. This bit can be driven onto a MATn.1 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    #[must_use]
    pub fn em1(&mut self) -> EM1_W<EMR_SPEC, 1> {
        EM1_W::new(self)
    }
    #[doc = "Bit 2 - External Match 2. When a match occurs between the TC and MR2, this bit can either toggle, go low, go high, or do nothing, depending on bits 9:8 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    #[must_use]
    pub fn em2(&mut self) -> EM2_W<EMR_SPEC, 2> {
        EM2_W::new(self)
    }
    #[doc = "Bit 3 - External Match 3. When a match occurs between the TC and MR3, this bit can either toggle, go low, go high, or do nothing, depending on bits 11:10 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    #[must_use]
    pub fn em3(&mut self) -> EM3_W<EMR_SPEC, 3> {
        EM3_W::new(self)
    }
    #[doc = "Bits 4:5 - External Match Control 0. Determines the functionality of External Match 0."]
    #[inline(always)]
    #[must_use]
    pub fn emc0(&mut self) -> EMC0_W<EMR_SPEC, 4> {
        EMC0_W::new(self)
    }
    #[doc = "Bits 6:7 - External Match Control 1. Determines the functionality of External Match 1."]
    #[inline(always)]
    #[must_use]
    pub fn emc1(&mut self) -> EMC1_W<EMR_SPEC, 6> {
        EMC1_W::new(self)
    }
    #[doc = "Bits 8:9 - External Match Control 2. Determines the functionality of External Match 2."]
    #[inline(always)]
    #[must_use]
    pub fn emc2(&mut self) -> EMC2_W<EMR_SPEC, 8> {
        EMC2_W::new(self)
    }
    #[doc = "Bits 10:11 - External Match Control 3. Determines the functionality of External Match 3."]
    #[inline(always)]
    #[must_use]
    pub fn emc3(&mut self) -> EMC3_W<EMR_SPEC, 10> {
        EMC3_W::new(self)
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
#[doc = "External Match Register. The EMR controls the external match pins.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMR_SPEC;
impl crate::RegisterSpec for EMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emr::R`](R) reader structure"]
impl crate::Readable for EMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emr::W`](W) writer structure"]
impl crate::Writable for EMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EMR to value 0"]
impl crate::Resettable for EMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
