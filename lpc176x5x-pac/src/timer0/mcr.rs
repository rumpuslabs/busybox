#[doc = "Register `MCR` reader"]
pub type R = crate::R<MCR_SPEC>;
#[doc = "Register `MCR` writer"]
pub type W = crate::W<MCR_SPEC>;
#[doc = "Field `MR0I` reader - Interrupt on MR0"]
pub type MR0I_R = crate::BitReader<MR0I_A>;
#[doc = "Interrupt on MR0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR0I_A {
    #[doc = "1: Interrupt is generated when MR0 matches the value in the TC."]
    ENABLED = 1,
    #[doc = "0: Interrupt is disabled"]
    DISABLED = 0,
}
impl From<MR0I_A> for bool {
    #[inline(always)]
    fn from(variant: MR0I_A) -> Self {
        variant as u8 != 0
    }
}
impl MR0I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MR0I_A {
        match self.bits {
            true => MR0I_A::ENABLED,
            false => MR0I_A::DISABLED,
        }
    }
    #[doc = "Interrupt is generated when MR0 matches the value in the TC."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MR0I_A::ENABLED
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MR0I_A::DISABLED
    }
}
#[doc = "Field `MR0I` writer - Interrupt on MR0"]
pub type MR0I_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MR0I_A>;
impl<'a, REG, const O: u8> MR0I_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated when MR0 matches the value in the TC."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MR0I_A::ENABLED)
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MR0I_A::DISABLED)
    }
}
#[doc = "Field `MR0R` reader - Reset on MR0"]
pub type MR0R_R = crate::BitReader<MR0R_A>;
#[doc = "Reset on MR0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR0R_A {
    #[doc = "1: TC will be reset if MR0 matches it."]
    ENABLED = 1,
    #[doc = "0: Feature disabled."]
    DISABLED = 0,
}
impl From<MR0R_A> for bool {
    #[inline(always)]
    fn from(variant: MR0R_A) -> Self {
        variant as u8 != 0
    }
}
impl MR0R_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MR0R_A {
        match self.bits {
            true => MR0R_A::ENABLED,
            false => MR0R_A::DISABLED,
        }
    }
    #[doc = "TC will be reset if MR0 matches it."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MR0R_A::ENABLED
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MR0R_A::DISABLED
    }
}
#[doc = "Field `MR0R` writer - Reset on MR0"]
pub type MR0R_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MR0R_A>;
impl<'a, REG, const O: u8> MR0R_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TC will be reset if MR0 matches it."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MR0R_A::ENABLED)
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MR0R_A::DISABLED)
    }
}
#[doc = "Field `MR0S` reader - Stop on MR0"]
pub type MR0S_R = crate::BitReader<MR0S_A>;
#[doc = "Stop on MR0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR0S_A {
    #[doc = "1: TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR0 matches the TC."]
    ENABLED = 1,
    #[doc = "0: Feature disabled."]
    DISABLED = 0,
}
impl From<MR0S_A> for bool {
    #[inline(always)]
    fn from(variant: MR0S_A) -> Self {
        variant as u8 != 0
    }
}
impl MR0S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MR0S_A {
        match self.bits {
            true => MR0S_A::ENABLED,
            false => MR0S_A::DISABLED,
        }
    }
    #[doc = "TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR0 matches the TC."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MR0S_A::ENABLED
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MR0S_A::DISABLED
    }
}
#[doc = "Field `MR0S` writer - Stop on MR0"]
pub type MR0S_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MR0S_A>;
impl<'a, REG, const O: u8> MR0S_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR0 matches the TC."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MR0S_A::ENABLED)
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MR0S_A::DISABLED)
    }
}
#[doc = "Field `MR1I` reader - Interrupt on MR1"]
pub type MR1I_R = crate::BitReader<MR1I_A>;
#[doc = "Interrupt on MR1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR1I_A {
    #[doc = "1: Interrupt is generated when MR1 matches the value in the TC."]
    ENABLED = 1,
    #[doc = "0: Interrupt is disabled."]
    DISABLED = 0,
}
impl From<MR1I_A> for bool {
    #[inline(always)]
    fn from(variant: MR1I_A) -> Self {
        variant as u8 != 0
    }
}
impl MR1I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MR1I_A {
        match self.bits {
            true => MR1I_A::ENABLED,
            false => MR1I_A::DISABLED,
        }
    }
    #[doc = "Interrupt is generated when MR1 matches the value in the TC."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MR1I_A::ENABLED
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MR1I_A::DISABLED
    }
}
#[doc = "Field `MR1I` writer - Interrupt on MR1"]
pub type MR1I_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MR1I_A>;
impl<'a, REG, const O: u8> MR1I_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated when MR1 matches the value in the TC."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MR1I_A::ENABLED)
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MR1I_A::DISABLED)
    }
}
#[doc = "Field `MR1R` reader - Reset on MR1"]
pub type MR1R_R = crate::BitReader<MR1R_A>;
#[doc = "Reset on MR1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR1R_A {
    #[doc = "1: TC will be reset if MR1 matches it."]
    ENABLED = 1,
    #[doc = "0: Feature disabled."]
    DISABLED = 0,
}
impl From<MR1R_A> for bool {
    #[inline(always)]
    fn from(variant: MR1R_A) -> Self {
        variant as u8 != 0
    }
}
impl MR1R_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MR1R_A {
        match self.bits {
            true => MR1R_A::ENABLED,
            false => MR1R_A::DISABLED,
        }
    }
    #[doc = "TC will be reset if MR1 matches it."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MR1R_A::ENABLED
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MR1R_A::DISABLED
    }
}
#[doc = "Field `MR1R` writer - Reset on MR1"]
pub type MR1R_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MR1R_A>;
impl<'a, REG, const O: u8> MR1R_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TC will be reset if MR1 matches it."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MR1R_A::ENABLED)
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MR1R_A::DISABLED)
    }
}
#[doc = "Field `MR1S` reader - Stop on MR1"]
pub type MR1S_R = crate::BitReader<MR1S_A>;
#[doc = "Stop on MR1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR1S_A {
    #[doc = "1: TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR1 matches the TC."]
    ENABLED = 1,
    #[doc = "0: Feature disabled."]
    DISABLED = 0,
}
impl From<MR1S_A> for bool {
    #[inline(always)]
    fn from(variant: MR1S_A) -> Self {
        variant as u8 != 0
    }
}
impl MR1S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MR1S_A {
        match self.bits {
            true => MR1S_A::ENABLED,
            false => MR1S_A::DISABLED,
        }
    }
    #[doc = "TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR1 matches the TC."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MR1S_A::ENABLED
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MR1S_A::DISABLED
    }
}
#[doc = "Field `MR1S` writer - Stop on MR1"]
pub type MR1S_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MR1S_A>;
impl<'a, REG, const O: u8> MR1S_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR1 matches the TC."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MR1S_A::ENABLED)
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MR1S_A::DISABLED)
    }
}
#[doc = "Field `MR2I` reader - Interrupt on MR2"]
pub type MR2I_R = crate::BitReader<MR2I_A>;
#[doc = "Interrupt on MR2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR2I_A {
    #[doc = "1: Interrupt is generated when MR2 matches the value in the TC."]
    ENABLED = 1,
    #[doc = "0: Interrupt is disabled"]
    DISABLED = 0,
}
impl From<MR2I_A> for bool {
    #[inline(always)]
    fn from(variant: MR2I_A) -> Self {
        variant as u8 != 0
    }
}
impl MR2I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MR2I_A {
        match self.bits {
            true => MR2I_A::ENABLED,
            false => MR2I_A::DISABLED,
        }
    }
    #[doc = "Interrupt is generated when MR2 matches the value in the TC."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MR2I_A::ENABLED
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MR2I_A::DISABLED
    }
}
#[doc = "Field `MR2I` writer - Interrupt on MR2"]
pub type MR2I_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MR2I_A>;
impl<'a, REG, const O: u8> MR2I_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated when MR2 matches the value in the TC."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MR2I_A::ENABLED)
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MR2I_A::DISABLED)
    }
}
#[doc = "Field `MR2R` reader - Reset on MR2"]
pub type MR2R_R = crate::BitReader<MR2R_A>;
#[doc = "Reset on MR2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR2R_A {
    #[doc = "1: TC will be reset if MR2 matches it."]
    ENABLED = 1,
    #[doc = "0: Feature disabled."]
    DISABLED = 0,
}
impl From<MR2R_A> for bool {
    #[inline(always)]
    fn from(variant: MR2R_A) -> Self {
        variant as u8 != 0
    }
}
impl MR2R_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MR2R_A {
        match self.bits {
            true => MR2R_A::ENABLED,
            false => MR2R_A::DISABLED,
        }
    }
    #[doc = "TC will be reset if MR2 matches it."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MR2R_A::ENABLED
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MR2R_A::DISABLED
    }
}
#[doc = "Field `MR2R` writer - Reset on MR2"]
pub type MR2R_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MR2R_A>;
impl<'a, REG, const O: u8> MR2R_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TC will be reset if MR2 matches it."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MR2R_A::ENABLED)
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MR2R_A::DISABLED)
    }
}
#[doc = "Field `MR2S` reader - Stop on MR2."]
pub type MR2S_R = crate::BitReader<MR2S_A>;
#[doc = "Stop on MR2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR2S_A {
    #[doc = "1: TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR2 matches the TC"]
    ENABLED = 1,
    #[doc = "0: Feature disabled."]
    DISABLED = 0,
}
impl From<MR2S_A> for bool {
    #[inline(always)]
    fn from(variant: MR2S_A) -> Self {
        variant as u8 != 0
    }
}
impl MR2S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MR2S_A {
        match self.bits {
            true => MR2S_A::ENABLED,
            false => MR2S_A::DISABLED,
        }
    }
    #[doc = "TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR2 matches the TC"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MR2S_A::ENABLED
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MR2S_A::DISABLED
    }
}
#[doc = "Field `MR2S` writer - Stop on MR2."]
pub type MR2S_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MR2S_A>;
impl<'a, REG, const O: u8> MR2S_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR2 matches the TC"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MR2S_A::ENABLED)
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MR2S_A::DISABLED)
    }
}
#[doc = "Field `MR3I` reader - Interrupt on MR3"]
pub type MR3I_R = crate::BitReader<MR3I_A>;
#[doc = "Interrupt on MR3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR3I_A {
    #[doc = "1: Interrupt is generated when MR3 matches the value in the TC."]
    ENABLED = 1,
    #[doc = "0: This interrupt is disabled"]
    DISABLED = 0,
}
impl From<MR3I_A> for bool {
    #[inline(always)]
    fn from(variant: MR3I_A) -> Self {
        variant as u8 != 0
    }
}
impl MR3I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MR3I_A {
        match self.bits {
            true => MR3I_A::ENABLED,
            false => MR3I_A::DISABLED,
        }
    }
    #[doc = "Interrupt is generated when MR3 matches the value in the TC."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MR3I_A::ENABLED
    }
    #[doc = "This interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MR3I_A::DISABLED
    }
}
#[doc = "Field `MR3I` writer - Interrupt on MR3"]
pub type MR3I_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MR3I_A>;
impl<'a, REG, const O: u8> MR3I_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated when MR3 matches the value in the TC."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MR3I_A::ENABLED)
    }
    #[doc = "This interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MR3I_A::DISABLED)
    }
}
#[doc = "Field `MR3R` reader - Reset on MR3"]
pub type MR3R_R = crate::BitReader<MR3R_A>;
#[doc = "Reset on MR3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR3R_A {
    #[doc = "1: TC will be reset if MR3 matches it."]
    ENABLED = 1,
    #[doc = "0: Feature disabled."]
    DISABLED = 0,
}
impl From<MR3R_A> for bool {
    #[inline(always)]
    fn from(variant: MR3R_A) -> Self {
        variant as u8 != 0
    }
}
impl MR3R_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MR3R_A {
        match self.bits {
            true => MR3R_A::ENABLED,
            false => MR3R_A::DISABLED,
        }
    }
    #[doc = "TC will be reset if MR3 matches it."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MR3R_A::ENABLED
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MR3R_A::DISABLED
    }
}
#[doc = "Field `MR3R` writer - Reset on MR3"]
pub type MR3R_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MR3R_A>;
impl<'a, REG, const O: u8> MR3R_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TC will be reset if MR3 matches it."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MR3R_A::ENABLED)
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MR3R_A::DISABLED)
    }
}
#[doc = "Field `MR3S` reader - Stop on MR3"]
pub type MR3S_R = crate::BitReader<MR3S_A>;
#[doc = "Stop on MR3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR3S_A {
    #[doc = "1: TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR3 matches the TC."]
    ENABLED = 1,
    #[doc = "0: Feature disabled."]
    DISABLED = 0,
}
impl From<MR3S_A> for bool {
    #[inline(always)]
    fn from(variant: MR3S_A) -> Self {
        variant as u8 != 0
    }
}
impl MR3S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MR3S_A {
        match self.bits {
            true => MR3S_A::ENABLED,
            false => MR3S_A::DISABLED,
        }
    }
    #[doc = "TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR3 matches the TC."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MR3S_A::ENABLED
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MR3S_A::DISABLED
    }
}
#[doc = "Field `MR3S` writer - Stop on MR3"]
pub type MR3S_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MR3S_A>;
impl<'a, REG, const O: u8> MR3S_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR3 matches the TC."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MR3S_A::ENABLED)
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MR3S_A::DISABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt on MR0"]
    #[inline(always)]
    pub fn mr0i(&self) -> MR0I_R {
        MR0I_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset on MR0"]
    #[inline(always)]
    pub fn mr0r(&self) -> MR0R_R {
        MR0R_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stop on MR0"]
    #[inline(always)]
    pub fn mr0s(&self) -> MR0S_R {
        MR0S_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt on MR1"]
    #[inline(always)]
    pub fn mr1i(&self) -> MR1I_R {
        MR1I_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset on MR1"]
    #[inline(always)]
    pub fn mr1r(&self) -> MR1R_R {
        MR1R_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stop on MR1"]
    #[inline(always)]
    pub fn mr1s(&self) -> MR1S_R {
        MR1S_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt on MR2"]
    #[inline(always)]
    pub fn mr2i(&self) -> MR2I_R {
        MR2I_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reset on MR2"]
    #[inline(always)]
    pub fn mr2r(&self) -> MR2R_R {
        MR2R_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Stop on MR2."]
    #[inline(always)]
    pub fn mr2s(&self) -> MR2S_R {
        MR2S_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt on MR3"]
    #[inline(always)]
    pub fn mr3i(&self) -> MR3I_R {
        MR3I_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reset on MR3"]
    #[inline(always)]
    pub fn mr3r(&self) -> MR3R_R {
        MR3R_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Stop on MR3"]
    #[inline(always)]
    pub fn mr3s(&self) -> MR3S_R {
        MR3S_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCR")
            .field("mr0i", &format_args!("{}", self.mr0i().bit()))
            .field("mr0r", &format_args!("{}", self.mr0r().bit()))
            .field("mr0s", &format_args!("{}", self.mr0s().bit()))
            .field("mr1i", &format_args!("{}", self.mr1i().bit()))
            .field("mr1r", &format_args!("{}", self.mr1r().bit()))
            .field("mr1s", &format_args!("{}", self.mr1s().bit()))
            .field("mr2i", &format_args!("{}", self.mr2i().bit()))
            .field("mr2r", &format_args!("{}", self.mr2r().bit()))
            .field("mr2s", &format_args!("{}", self.mr2s().bit()))
            .field("mr3i", &format_args!("{}", self.mr3i().bit()))
            .field("mr3r", &format_args!("{}", self.mr3r().bit()))
            .field("mr3s", &format_args!("{}", self.mr3s().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<MCR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt on MR0"]
    #[inline(always)]
    #[must_use]
    pub fn mr0i(&mut self) -> MR0I_W<MCR_SPEC, 0> {
        MR0I_W::new(self)
    }
    #[doc = "Bit 1 - Reset on MR0"]
    #[inline(always)]
    #[must_use]
    pub fn mr0r(&mut self) -> MR0R_W<MCR_SPEC, 1> {
        MR0R_W::new(self)
    }
    #[doc = "Bit 2 - Stop on MR0"]
    #[inline(always)]
    #[must_use]
    pub fn mr0s(&mut self) -> MR0S_W<MCR_SPEC, 2> {
        MR0S_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt on MR1"]
    #[inline(always)]
    #[must_use]
    pub fn mr1i(&mut self) -> MR1I_W<MCR_SPEC, 3> {
        MR1I_W::new(self)
    }
    #[doc = "Bit 4 - Reset on MR1"]
    #[inline(always)]
    #[must_use]
    pub fn mr1r(&mut self) -> MR1R_W<MCR_SPEC, 4> {
        MR1R_W::new(self)
    }
    #[doc = "Bit 5 - Stop on MR1"]
    #[inline(always)]
    #[must_use]
    pub fn mr1s(&mut self) -> MR1S_W<MCR_SPEC, 5> {
        MR1S_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt on MR2"]
    #[inline(always)]
    #[must_use]
    pub fn mr2i(&mut self) -> MR2I_W<MCR_SPEC, 6> {
        MR2I_W::new(self)
    }
    #[doc = "Bit 7 - Reset on MR2"]
    #[inline(always)]
    #[must_use]
    pub fn mr2r(&mut self) -> MR2R_W<MCR_SPEC, 7> {
        MR2R_W::new(self)
    }
    #[doc = "Bit 8 - Stop on MR2."]
    #[inline(always)]
    #[must_use]
    pub fn mr2s(&mut self) -> MR2S_W<MCR_SPEC, 8> {
        MR2S_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt on MR3"]
    #[inline(always)]
    #[must_use]
    pub fn mr3i(&mut self) -> MR3I_W<MCR_SPEC, 9> {
        MR3I_W::new(self)
    }
    #[doc = "Bit 10 - Reset on MR3"]
    #[inline(always)]
    #[must_use]
    pub fn mr3r(&mut self) -> MR3R_W<MCR_SPEC, 10> {
        MR3R_W::new(self)
    }
    #[doc = "Bit 11 - Stop on MR3"]
    #[inline(always)]
    #[must_use]
    pub fn mr3s(&mut self) -> MR3S_W<MCR_SPEC, 11> {
        MR3S_W::new(self)
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
#[doc = "Match Control Register. The MCR is used to control if an interrupt is generated and if the TC is reset when a Match occurs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcr::R`](R) reader structure"]
impl crate::Readable for MCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcr::W`](W) writer structure"]
impl crate::Writable for MCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for MCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
