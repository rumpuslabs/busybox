#[doc = "Register `PCR` reader"]
pub type R = crate::R<PCR_SPEC>;
#[doc = "Register `PCR` writer"]
pub type W = crate::W<PCR_SPEC>;
#[doc = "Field `PWMSEL2` reader - PWM\\[2\\]
output single/double edge mode control."]
pub type PWMSEL2_R = crate::BitReader<PWMSEL2_A>;
#[doc = "PWM\\[2\\]
output single/double edge mode control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWMSEL2_A {
    #[doc = "0: Single edge controlled mode is selected."]
    SINGLE_EDGE = 0,
    #[doc = "1: Double edge controlled mode is selected."]
    DOUBLE_EDGE = 1,
}
impl From<PWMSEL2_A> for bool {
    #[inline(always)]
    fn from(variant: PWMSEL2_A) -> Self {
        variant as u8 != 0
    }
}
impl PWMSEL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWMSEL2_A {
        match self.bits {
            false => PWMSEL2_A::SINGLE_EDGE,
            true => PWMSEL2_A::DOUBLE_EDGE,
        }
    }
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn is_single_edge(&self) -> bool {
        *self == PWMSEL2_A::SINGLE_EDGE
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn is_double_edge(&self) -> bool {
        *self == PWMSEL2_A::DOUBLE_EDGE
    }
}
#[doc = "Field `PWMSEL2` writer - PWM\\[2\\]
output single/double edge mode control."]
pub type PWMSEL2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PWMSEL2_A>;
impl<'a, REG, const O: u8> PWMSEL2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn single_edge(self) -> &'a mut crate::W<REG> {
        self.variant(PWMSEL2_A::SINGLE_EDGE)
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn double_edge(self) -> &'a mut crate::W<REG> {
        self.variant(PWMSEL2_A::DOUBLE_EDGE)
    }
}
#[doc = "Field `PWMSEL3` reader - PWM\\[3\\]
output edge control."]
pub type PWMSEL3_R = crate::BitReader<PWMSEL3_A>;
#[doc = "PWM\\[3\\]
output edge control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWMSEL3_A {
    #[doc = "0: Single edge controlled mode is selected."]
    SINGLE_EDGE = 0,
    #[doc = "1: Double edge controlled mode is selected."]
    DOUBLE_EDGE = 1,
}
impl From<PWMSEL3_A> for bool {
    #[inline(always)]
    fn from(variant: PWMSEL3_A) -> Self {
        variant as u8 != 0
    }
}
impl PWMSEL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWMSEL3_A {
        match self.bits {
            false => PWMSEL3_A::SINGLE_EDGE,
            true => PWMSEL3_A::DOUBLE_EDGE,
        }
    }
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn is_single_edge(&self) -> bool {
        *self == PWMSEL3_A::SINGLE_EDGE
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn is_double_edge(&self) -> bool {
        *self == PWMSEL3_A::DOUBLE_EDGE
    }
}
#[doc = "Field `PWMSEL3` writer - PWM\\[3\\]
output edge control."]
pub type PWMSEL3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PWMSEL3_A>;
impl<'a, REG, const O: u8> PWMSEL3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn single_edge(self) -> &'a mut crate::W<REG> {
        self.variant(PWMSEL3_A::SINGLE_EDGE)
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn double_edge(self) -> &'a mut crate::W<REG> {
        self.variant(PWMSEL3_A::DOUBLE_EDGE)
    }
}
#[doc = "Field `PWMSEL4` reader - PWM\\[4\\]
output edge control."]
pub type PWMSEL4_R = crate::BitReader<PWMSEL4_A>;
#[doc = "PWM\\[4\\]
output edge control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWMSEL4_A {
    #[doc = "0: Single edge controlled mode is selected."]
    SINGLE_EDGE = 0,
    #[doc = "1: Double edge controlled mode is selected."]
    DOUBLE_EDGE = 1,
}
impl From<PWMSEL4_A> for bool {
    #[inline(always)]
    fn from(variant: PWMSEL4_A) -> Self {
        variant as u8 != 0
    }
}
impl PWMSEL4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWMSEL4_A {
        match self.bits {
            false => PWMSEL4_A::SINGLE_EDGE,
            true => PWMSEL4_A::DOUBLE_EDGE,
        }
    }
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn is_single_edge(&self) -> bool {
        *self == PWMSEL4_A::SINGLE_EDGE
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn is_double_edge(&self) -> bool {
        *self == PWMSEL4_A::DOUBLE_EDGE
    }
}
#[doc = "Field `PWMSEL4` writer - PWM\\[4\\]
output edge control."]
pub type PWMSEL4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PWMSEL4_A>;
impl<'a, REG, const O: u8> PWMSEL4_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn single_edge(self) -> &'a mut crate::W<REG> {
        self.variant(PWMSEL4_A::SINGLE_EDGE)
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn double_edge(self) -> &'a mut crate::W<REG> {
        self.variant(PWMSEL4_A::DOUBLE_EDGE)
    }
}
#[doc = "Field `PWMSEL5` reader - PWM\\[5\\]
output edge control."]
pub type PWMSEL5_R = crate::BitReader<PWMSEL5_A>;
#[doc = "PWM\\[5\\]
output edge control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWMSEL5_A {
    #[doc = "0: Single edge controlled mode is selected."]
    SINGLE_EDGE = 0,
    #[doc = "1: Double edge controlled mode is selected."]
    DOUBLE_EDGE = 1,
}
impl From<PWMSEL5_A> for bool {
    #[inline(always)]
    fn from(variant: PWMSEL5_A) -> Self {
        variant as u8 != 0
    }
}
impl PWMSEL5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWMSEL5_A {
        match self.bits {
            false => PWMSEL5_A::SINGLE_EDGE,
            true => PWMSEL5_A::DOUBLE_EDGE,
        }
    }
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn is_single_edge(&self) -> bool {
        *self == PWMSEL5_A::SINGLE_EDGE
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn is_double_edge(&self) -> bool {
        *self == PWMSEL5_A::DOUBLE_EDGE
    }
}
#[doc = "Field `PWMSEL5` writer - PWM\\[5\\]
output edge control."]
pub type PWMSEL5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PWMSEL5_A>;
impl<'a, REG, const O: u8> PWMSEL5_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn single_edge(self) -> &'a mut crate::W<REG> {
        self.variant(PWMSEL5_A::SINGLE_EDGE)
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn double_edge(self) -> &'a mut crate::W<REG> {
        self.variant(PWMSEL5_A::DOUBLE_EDGE)
    }
}
#[doc = "Field `PWMSEL6` reader - PWM\\[6\\]
output edge control."]
pub type PWMSEL6_R = crate::BitReader<PWMSEL6_A>;
#[doc = "PWM\\[6\\]
output edge control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWMSEL6_A {
    #[doc = "0: Single edge controlled mode is selected."]
    SINGLE_EDGE = 0,
    #[doc = "1: Double edge controlled mode is selected."]
    DOUBLE_EDGE = 1,
}
impl From<PWMSEL6_A> for bool {
    #[inline(always)]
    fn from(variant: PWMSEL6_A) -> Self {
        variant as u8 != 0
    }
}
impl PWMSEL6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWMSEL6_A {
        match self.bits {
            false => PWMSEL6_A::SINGLE_EDGE,
            true => PWMSEL6_A::DOUBLE_EDGE,
        }
    }
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn is_single_edge(&self) -> bool {
        *self == PWMSEL6_A::SINGLE_EDGE
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn is_double_edge(&self) -> bool {
        *self == PWMSEL6_A::DOUBLE_EDGE
    }
}
#[doc = "Field `PWMSEL6` writer - PWM\\[6\\]
output edge control."]
pub type PWMSEL6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PWMSEL6_A>;
impl<'a, REG, const O: u8> PWMSEL6_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn single_edge(self) -> &'a mut crate::W<REG> {
        self.variant(PWMSEL6_A::SINGLE_EDGE)
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn double_edge(self) -> &'a mut crate::W<REG> {
        self.variant(PWMSEL6_A::DOUBLE_EDGE)
    }
}
#[doc = "Field `PWMENA1` reader - PWM\\[1\\]
output enable control."]
pub type PWMENA1_R = crate::BitReader<PWMENA1_A>;
#[doc = "PWM\\[1\\]
output enable control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWMENA1_A {
    #[doc = "0: The PWM output is disabled."]
    DISABLED = 0,
    #[doc = "1: The PWM output is enabled."]
    ENABLED = 1,
}
impl From<PWMENA1_A> for bool {
    #[inline(always)]
    fn from(variant: PWMENA1_A) -> Self {
        variant as u8 != 0
    }
}
impl PWMENA1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWMENA1_A {
        match self.bits {
            false => PWMENA1_A::DISABLED,
            true => PWMENA1_A::ENABLED,
        }
    }
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMENA1_A::DISABLED
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PWMENA1_A::ENABLED
    }
}
#[doc = "Field `PWMENA1` writer - PWM\\[1\\]
output enable control."]
pub type PWMENA1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PWMENA1_A>;
impl<'a, REG, const O: u8> PWMENA1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWMENA1_A::DISABLED)
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWMENA1_A::ENABLED)
    }
}
#[doc = "Field `PWMENA2` reader - PWM\\[2\\]
output enable control."]
pub type PWMENA2_R = crate::BitReader<PWMENA2_A>;
#[doc = "PWM\\[2\\]
output enable control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWMENA2_A {
    #[doc = "0: The PWM output is disabled."]
    DISABLED = 0,
    #[doc = "1: The PWM output is enabled."]
    ENABLED = 1,
}
impl From<PWMENA2_A> for bool {
    #[inline(always)]
    fn from(variant: PWMENA2_A) -> Self {
        variant as u8 != 0
    }
}
impl PWMENA2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWMENA2_A {
        match self.bits {
            false => PWMENA2_A::DISABLED,
            true => PWMENA2_A::ENABLED,
        }
    }
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMENA2_A::DISABLED
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PWMENA2_A::ENABLED
    }
}
#[doc = "Field `PWMENA2` writer - PWM\\[2\\]
output enable control."]
pub type PWMENA2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PWMENA2_A>;
impl<'a, REG, const O: u8> PWMENA2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWMENA2_A::DISABLED)
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWMENA2_A::ENABLED)
    }
}
#[doc = "Field `PWMENA3` reader - PWM\\[3\\]
output enable control."]
pub type PWMENA3_R = crate::BitReader<PWMENA3_A>;
#[doc = "PWM\\[3\\]
output enable control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWMENA3_A {
    #[doc = "0: The PWM output is disabled."]
    DISABLED = 0,
    #[doc = "1: The PWM output is enabled."]
    ENABLED = 1,
}
impl From<PWMENA3_A> for bool {
    #[inline(always)]
    fn from(variant: PWMENA3_A) -> Self {
        variant as u8 != 0
    }
}
impl PWMENA3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWMENA3_A {
        match self.bits {
            false => PWMENA3_A::DISABLED,
            true => PWMENA3_A::ENABLED,
        }
    }
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMENA3_A::DISABLED
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PWMENA3_A::ENABLED
    }
}
#[doc = "Field `PWMENA3` writer - PWM\\[3\\]
output enable control."]
pub type PWMENA3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PWMENA3_A>;
impl<'a, REG, const O: u8> PWMENA3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWMENA3_A::DISABLED)
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWMENA3_A::ENABLED)
    }
}
#[doc = "Field `PWMENA4` reader - PWM\\[4\\]
output enable control."]
pub type PWMENA4_R = crate::BitReader<PWMENA4_A>;
#[doc = "PWM\\[4\\]
output enable control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWMENA4_A {
    #[doc = "0: The PWM output is disabled."]
    DISABLED = 0,
    #[doc = "1: The PWM output is enabled."]
    ENABLED = 1,
}
impl From<PWMENA4_A> for bool {
    #[inline(always)]
    fn from(variant: PWMENA4_A) -> Self {
        variant as u8 != 0
    }
}
impl PWMENA4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWMENA4_A {
        match self.bits {
            false => PWMENA4_A::DISABLED,
            true => PWMENA4_A::ENABLED,
        }
    }
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMENA4_A::DISABLED
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PWMENA4_A::ENABLED
    }
}
#[doc = "Field `PWMENA4` writer - PWM\\[4\\]
output enable control."]
pub type PWMENA4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PWMENA4_A>;
impl<'a, REG, const O: u8> PWMENA4_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWMENA4_A::DISABLED)
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWMENA4_A::ENABLED)
    }
}
#[doc = "Field `PWMENA5` reader - PWM\\[5\\]
output enable control."]
pub type PWMENA5_R = crate::BitReader<PWMENA5_A>;
#[doc = "PWM\\[5\\]
output enable control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWMENA5_A {
    #[doc = "0: The PWM output is disabled."]
    DISABLED = 0,
    #[doc = "1: The PWM output is enabled."]
    ENABLED = 1,
}
impl From<PWMENA5_A> for bool {
    #[inline(always)]
    fn from(variant: PWMENA5_A) -> Self {
        variant as u8 != 0
    }
}
impl PWMENA5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWMENA5_A {
        match self.bits {
            false => PWMENA5_A::DISABLED,
            true => PWMENA5_A::ENABLED,
        }
    }
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMENA5_A::DISABLED
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PWMENA5_A::ENABLED
    }
}
#[doc = "Field `PWMENA5` writer - PWM\\[5\\]
output enable control."]
pub type PWMENA5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PWMENA5_A>;
impl<'a, REG, const O: u8> PWMENA5_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWMENA5_A::DISABLED)
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWMENA5_A::ENABLED)
    }
}
#[doc = "Field `PWMENA6` reader - PWM\\[6\\]
output enable control. See PWMENA1 for details."]
pub type PWMENA6_R = crate::BitReader<PWMENA6_A>;
#[doc = "PWM\\[6\\]
output enable control. See PWMENA1 for details.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWMENA6_A {
    #[doc = "0: The PWM output is disabled."]
    DISABLED = 0,
    #[doc = "1: The PWM output is enabled."]
    ENABLED = 1,
}
impl From<PWMENA6_A> for bool {
    #[inline(always)]
    fn from(variant: PWMENA6_A) -> Self {
        variant as u8 != 0
    }
}
impl PWMENA6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWMENA6_A {
        match self.bits {
            false => PWMENA6_A::DISABLED,
            true => PWMENA6_A::ENABLED,
        }
    }
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMENA6_A::DISABLED
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PWMENA6_A::ENABLED
    }
}
#[doc = "Field `PWMENA6` writer - PWM\\[6\\]
output enable control. See PWMENA1 for details."]
pub type PWMENA6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PWMENA6_A>;
impl<'a, REG, const O: u8> PWMENA6_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWMENA6_A::DISABLED)
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWMENA6_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 2 - PWM\\[2\\]
output single/double edge mode control."]
    #[inline(always)]
    pub fn pwmsel2(&self) -> PWMSEL2_R {
        PWMSEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PWM\\[3\\]
output edge control."]
    #[inline(always)]
    pub fn pwmsel3(&self) -> PWMSEL3_R {
        PWMSEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PWM\\[4\\]
output edge control."]
    #[inline(always)]
    pub fn pwmsel4(&self) -> PWMSEL4_R {
        PWMSEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PWM\\[5\\]
output edge control."]
    #[inline(always)]
    pub fn pwmsel5(&self) -> PWMSEL5_R {
        PWMSEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PWM\\[6\\]
output edge control."]
    #[inline(always)]
    pub fn pwmsel6(&self) -> PWMSEL6_R {
        PWMSEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - PWM\\[1\\]
output enable control."]
    #[inline(always)]
    pub fn pwmena1(&self) -> PWMENA1_R {
        PWMENA1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PWM\\[2\\]
output enable control."]
    #[inline(always)]
    pub fn pwmena2(&self) -> PWMENA2_R {
        PWMENA2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PWM\\[3\\]
output enable control."]
    #[inline(always)]
    pub fn pwmena3(&self) -> PWMENA3_R {
        PWMENA3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PWM\\[4\\]
output enable control."]
    #[inline(always)]
    pub fn pwmena4(&self) -> PWMENA4_R {
        PWMENA4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PWM\\[5\\]
output enable control."]
    #[inline(always)]
    pub fn pwmena5(&self) -> PWMENA5_R {
        PWMENA5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PWM\\[6\\]
output enable control. See PWMENA1 for details."]
    #[inline(always)]
    pub fn pwmena6(&self) -> PWMENA6_R {
        PWMENA6_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCR")
            .field("pwmsel2", &format_args!("{}", self.pwmsel2().bit()))
            .field("pwmsel3", &format_args!("{}", self.pwmsel3().bit()))
            .field("pwmsel4", &format_args!("{}", self.pwmsel4().bit()))
            .field("pwmsel5", &format_args!("{}", self.pwmsel5().bit()))
            .field("pwmsel6", &format_args!("{}", self.pwmsel6().bit()))
            .field("pwmena1", &format_args!("{}", self.pwmena1().bit()))
            .field("pwmena2", &format_args!("{}", self.pwmena2().bit()))
            .field("pwmena3", &format_args!("{}", self.pwmena3().bit()))
            .field("pwmena4", &format_args!("{}", self.pwmena4().bit()))
            .field("pwmena5", &format_args!("{}", self.pwmena5().bit()))
            .field("pwmena6", &format_args!("{}", self.pwmena6().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<PCR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 2 - PWM\\[2\\]
output single/double edge mode control."]
    #[inline(always)]
    #[must_use]
    pub fn pwmsel2(&mut self) -> PWMSEL2_W<PCR_SPEC, 2> {
        PWMSEL2_W::new(self)
    }
    #[doc = "Bit 3 - PWM\\[3\\]
output edge control."]
    #[inline(always)]
    #[must_use]
    pub fn pwmsel3(&mut self) -> PWMSEL3_W<PCR_SPEC, 3> {
        PWMSEL3_W::new(self)
    }
    #[doc = "Bit 4 - PWM\\[4\\]
output edge control."]
    #[inline(always)]
    #[must_use]
    pub fn pwmsel4(&mut self) -> PWMSEL4_W<PCR_SPEC, 4> {
        PWMSEL4_W::new(self)
    }
    #[doc = "Bit 5 - PWM\\[5\\]
output edge control."]
    #[inline(always)]
    #[must_use]
    pub fn pwmsel5(&mut self) -> PWMSEL5_W<PCR_SPEC, 5> {
        PWMSEL5_W::new(self)
    }
    #[doc = "Bit 6 - PWM\\[6\\]
output edge control."]
    #[inline(always)]
    #[must_use]
    pub fn pwmsel6(&mut self) -> PWMSEL6_W<PCR_SPEC, 6> {
        PWMSEL6_W::new(self)
    }
    #[doc = "Bit 9 - PWM\\[1\\]
output enable control."]
    #[inline(always)]
    #[must_use]
    pub fn pwmena1(&mut self) -> PWMENA1_W<PCR_SPEC, 9> {
        PWMENA1_W::new(self)
    }
    #[doc = "Bit 10 - PWM\\[2\\]
output enable control."]
    #[inline(always)]
    #[must_use]
    pub fn pwmena2(&mut self) -> PWMENA2_W<PCR_SPEC, 10> {
        PWMENA2_W::new(self)
    }
    #[doc = "Bit 11 - PWM\\[3\\]
output enable control."]
    #[inline(always)]
    #[must_use]
    pub fn pwmena3(&mut self) -> PWMENA3_W<PCR_SPEC, 11> {
        PWMENA3_W::new(self)
    }
    #[doc = "Bit 12 - PWM\\[4\\]
output enable control."]
    #[inline(always)]
    #[must_use]
    pub fn pwmena4(&mut self) -> PWMENA4_W<PCR_SPEC, 12> {
        PWMENA4_W::new(self)
    }
    #[doc = "Bit 13 - PWM\\[5\\]
output enable control."]
    #[inline(always)]
    #[must_use]
    pub fn pwmena5(&mut self) -> PWMENA5_W<PCR_SPEC, 13> {
        PWMENA5_W::new(self)
    }
    #[doc = "Bit 14 - PWM\\[6\\]
output enable control. See PWMENA1 for details."]
    #[inline(always)]
    #[must_use]
    pub fn pwmena6(&mut self) -> PWMENA6_W<PCR_SPEC, 14> {
        PWMENA6_W::new(self)
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
#[doc = "PWM Control Register. Enables PWM outputs and selects either single edge or double edge controlled PWM outputs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCR_SPEC;
impl crate::RegisterSpec for PCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcr::R`](R) reader structure"]
impl crate::Readable for PCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcr::W`](W) writer structure"]
impl crate::Writable for PCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCR to value 0"]
impl crate::Resettable for PCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
