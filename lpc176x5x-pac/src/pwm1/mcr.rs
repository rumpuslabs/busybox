#[doc = "Register `MCR` reader"]
pub type R = crate::R<MCR_SPEC>;
#[doc = "Register `MCR` writer"]
pub type W = crate::W<MCR_SPEC>;
#[doc = "Field `PWMMR0I` reader - Interrupt PWM0"]
pub type PWMMR0I_R = crate::BitReader<PWMMR0I_A>;
#[doc = "Interrupt PWM0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWMMR0I_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt on PWMMR0: an interrupt is generated when PWMMR0 matches the value in the PWMTC."]
    INTERRUPT_ON_PWMMR0 = 1,
}
impl From<PWMMR0I_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR0I_A) -> Self {
        variant as u8 != 0
    }
}
impl PWMMR0I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWMMR0I_A {
        match self.bits {
            false => PWMMR0I_A::DISABLED,
            true => PWMMR0I_A::INTERRUPT_ON_PWMMR0,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR0I_A::DISABLED
    }
    #[doc = "Interrupt on PWMMR0: an interrupt is generated when PWMMR0 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn is_interrupt_on_pwmmr0(&self) -> bool {
        *self == PWMMR0I_A::INTERRUPT_ON_PWMMR0
    }
}
#[doc = "Field `PWMMR0I` writer - Interrupt PWM0"]
pub type PWMMR0I_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PWMMR0I_A>;
impl<'a, REG, const O: u8> PWMMR0I_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR0I_A::DISABLED)
    }
    #[doc = "Interrupt on PWMMR0: an interrupt is generated when PWMMR0 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn interrupt_on_pwmmr0(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR0I_A::INTERRUPT_ON_PWMMR0)
    }
}
#[doc = "Field `PWMMR0R` reader - Reset PWM0"]
pub type PWMMR0R_R = crate::BitReader<PWMMR0R_A>;
#[doc = "Reset PWM0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWMMR0R_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Reset on PWMMR0: the PWMTC will be reset if PWMMR0 matches it."]
    RESET_ON_PWMMR0 = 1,
}
impl From<PWMMR0R_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR0R_A) -> Self {
        variant as u8 != 0
    }
}
impl PWMMR0R_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWMMR0R_A {
        match self.bits {
            false => PWMMR0R_A::DISABLED,
            true => PWMMR0R_A::RESET_ON_PWMMR0,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR0R_A::DISABLED
    }
    #[doc = "Reset on PWMMR0: the PWMTC will be reset if PWMMR0 matches it."]
    #[inline(always)]
    pub fn is_reset_on_pwmmr0(&self) -> bool {
        *self == PWMMR0R_A::RESET_ON_PWMMR0
    }
}
#[doc = "Field `PWMMR0R` writer - Reset PWM0"]
pub type PWMMR0R_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PWMMR0R_A>;
impl<'a, REG, const O: u8> PWMMR0R_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR0R_A::DISABLED)
    }
    #[doc = "Reset on PWMMR0: the PWMTC will be reset if PWMMR0 matches it."]
    #[inline(always)]
    pub fn reset_on_pwmmr0(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR0R_A::RESET_ON_PWMMR0)
    }
}
#[doc = "Field `PWMMR0S` reader - Stop PWM0"]
pub type PWMMR0S_R = crate::BitReader<PWMMR0S_A>;
#[doc = "Stop PWM0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWMMR0S_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Stop on PWMMR0: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    STOP_ON_PWMMR0 = 1,
}
impl From<PWMMR0S_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR0S_A) -> Self {
        variant as u8 != 0
    }
}
impl PWMMR0S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWMMR0S_A {
        match self.bits {
            false => PWMMR0S_A::DISABLED,
            true => PWMMR0S_A::STOP_ON_PWMMR0,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR0S_A::DISABLED
    }
    #[doc = "Stop on PWMMR0: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    #[inline(always)]
    pub fn is_stop_on_pwmmr0(&self) -> bool {
        *self == PWMMR0S_A::STOP_ON_PWMMR0
    }
}
#[doc = "Field `PWMMR0S` writer - Stop PWM0"]
pub type PWMMR0S_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PWMMR0S_A>;
impl<'a, REG, const O: u8> PWMMR0S_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR0S_A::DISABLED)
    }
    #[doc = "Stop on PWMMR0: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    #[inline(always)]
    pub fn stop_on_pwmmr0(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR0S_A::STOP_ON_PWMMR0)
    }
}
#[doc = "Field `PWMMR1I` reader - Interrupt PWM1"]
pub type PWMMR1I_R = crate::BitReader<PWMMR1I_A>;
#[doc = "Interrupt PWM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWMMR1I_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt on PWMMR1: an interrupt is generated when PWMMR1 matches the value in the PWMTC."]
    INTERRUPT_ON_PWMMR1 = 1,
}
impl From<PWMMR1I_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR1I_A) -> Self {
        variant as u8 != 0
    }
}
impl PWMMR1I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWMMR1I_A {
        match self.bits {
            false => PWMMR1I_A::DISABLED,
            true => PWMMR1I_A::INTERRUPT_ON_PWMMR1,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR1I_A::DISABLED
    }
    #[doc = "Interrupt on PWMMR1: an interrupt is generated when PWMMR1 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn is_interrupt_on_pwmmr1(&self) -> bool {
        *self == PWMMR1I_A::INTERRUPT_ON_PWMMR1
    }
}
#[doc = "Field `PWMMR1I` writer - Interrupt PWM1"]
pub type PWMMR1I_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PWMMR1I_A>;
impl<'a, REG, const O: u8> PWMMR1I_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR1I_A::DISABLED)
    }
    #[doc = "Interrupt on PWMMR1: an interrupt is generated when PWMMR1 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn interrupt_on_pwmmr1(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR1I_A::INTERRUPT_ON_PWMMR1)
    }
}
#[doc = "Field `PWMMR1R` reader - Reset PWM1"]
pub type PWMMR1R_R = crate::BitReader<PWMMR1R_A>;
#[doc = "Reset PWM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWMMR1R_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Reset on PWMMR1: the PWMTC will be reset if PWMMR1 matches it."]
    RESET_ON_PWMMR1 = 1,
}
impl From<PWMMR1R_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR1R_A) -> Self {
        variant as u8 != 0
    }
}
impl PWMMR1R_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWMMR1R_A {
        match self.bits {
            false => PWMMR1R_A::DISABLED,
            true => PWMMR1R_A::RESET_ON_PWMMR1,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR1R_A::DISABLED
    }
    #[doc = "Reset on PWMMR1: the PWMTC will be reset if PWMMR1 matches it."]
    #[inline(always)]
    pub fn is_reset_on_pwmmr1(&self) -> bool {
        *self == PWMMR1R_A::RESET_ON_PWMMR1
    }
}
#[doc = "Field `PWMMR1R` writer - Reset PWM1"]
pub type PWMMR1R_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PWMMR1R_A>;
impl<'a, REG, const O: u8> PWMMR1R_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR1R_A::DISABLED)
    }
    #[doc = "Reset on PWMMR1: the PWMTC will be reset if PWMMR1 matches it."]
    #[inline(always)]
    pub fn reset_on_pwmmr1(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR1R_A::RESET_ON_PWMMR1)
    }
}
#[doc = "Field `PWMMR1S` reader - Stop PWM1"]
pub type PWMMR1S_R = crate::BitReader<PWMMR1S_A>;
#[doc = "Stop PWM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWMMR1S_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Stop on PWMMR1: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR1 matches the PWMTC."]
    STOP_ON_PWMMR1 = 1,
}
impl From<PWMMR1S_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR1S_A) -> Self {
        variant as u8 != 0
    }
}
impl PWMMR1S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWMMR1S_A {
        match self.bits {
            false => PWMMR1S_A::DISABLED,
            true => PWMMR1S_A::STOP_ON_PWMMR1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR1S_A::DISABLED
    }
    #[doc = "Stop on PWMMR1: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR1 matches the PWMTC."]
    #[inline(always)]
    pub fn is_stop_on_pwmmr1(&self) -> bool {
        *self == PWMMR1S_A::STOP_ON_PWMMR1
    }
}
#[doc = "Field `PWMMR1S` writer - Stop PWM1"]
pub type PWMMR1S_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PWMMR1S_A>;
impl<'a, REG, const O: u8> PWMMR1S_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR1S_A::DISABLED)
    }
    #[doc = "Stop on PWMMR1: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR1 matches the PWMTC."]
    #[inline(always)]
    pub fn stop_on_pwmmr1(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR1S_A::STOP_ON_PWMMR1)
    }
}
#[doc = "Field `PWMMR2I` reader - Interrupt PWM0"]
pub type PWMMR2I_R = crate::BitReader<PWMMR2I_A>;
#[doc = "Interrupt PWM0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWMMR2I_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt on PWMMR2: an interrupt is generated when PWMMR2 matches the value in the PWMTC."]
    INTERRUPT_ON_PWMMR2 = 1,
}
impl From<PWMMR2I_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR2I_A) -> Self {
        variant as u8 != 0
    }
}
impl PWMMR2I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWMMR2I_A {
        match self.bits {
            false => PWMMR2I_A::DISABLED,
            true => PWMMR2I_A::INTERRUPT_ON_PWMMR2,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR2I_A::DISABLED
    }
    #[doc = "Interrupt on PWMMR2: an interrupt is generated when PWMMR2 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn is_interrupt_on_pwmmr2(&self) -> bool {
        *self == PWMMR2I_A::INTERRUPT_ON_PWMMR2
    }
}
#[doc = "Field `PWMMR2I` writer - Interrupt PWM0"]
pub type PWMMR2I_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PWMMR2I_A>;
impl<'a, REG, const O: u8> PWMMR2I_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR2I_A::DISABLED)
    }
    #[doc = "Interrupt on PWMMR2: an interrupt is generated when PWMMR2 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn interrupt_on_pwmmr2(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR2I_A::INTERRUPT_ON_PWMMR2)
    }
}
#[doc = "Field `PWMMR2R` reader - Reset PWM0"]
pub type PWMMR2R_R = crate::BitReader<PWMMR2R_A>;
#[doc = "Reset PWM0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWMMR2R_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Reset on PWMMR2: the PWMTC will be reset if PWMMR2 matches it."]
    RESET_ON_PWMMR2 = 1,
}
impl From<PWMMR2R_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR2R_A) -> Self {
        variant as u8 != 0
    }
}
impl PWMMR2R_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWMMR2R_A {
        match self.bits {
            false => PWMMR2R_A::DISABLED,
            true => PWMMR2R_A::RESET_ON_PWMMR2,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR2R_A::DISABLED
    }
    #[doc = "Reset on PWMMR2: the PWMTC will be reset if PWMMR2 matches it."]
    #[inline(always)]
    pub fn is_reset_on_pwmmr2(&self) -> bool {
        *self == PWMMR2R_A::RESET_ON_PWMMR2
    }
}
#[doc = "Field `PWMMR2R` writer - Reset PWM0"]
pub type PWMMR2R_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PWMMR2R_A>;
impl<'a, REG, const O: u8> PWMMR2R_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR2R_A::DISABLED)
    }
    #[doc = "Reset on PWMMR2: the PWMTC will be reset if PWMMR2 matches it."]
    #[inline(always)]
    pub fn reset_on_pwmmr2(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR2R_A::RESET_ON_PWMMR2)
    }
}
#[doc = "Field `PWMMR2S` reader - Stop PWM0"]
pub type PWMMR2S_R = crate::BitReader<PWMMR2S_A>;
#[doc = "Stop PWM0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWMMR2S_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Stop on PWMMR2: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    STOP_ON_PWMMR2 = 1,
}
impl From<PWMMR2S_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR2S_A) -> Self {
        variant as u8 != 0
    }
}
impl PWMMR2S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWMMR2S_A {
        match self.bits {
            false => PWMMR2S_A::DISABLED,
            true => PWMMR2S_A::STOP_ON_PWMMR2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR2S_A::DISABLED
    }
    #[doc = "Stop on PWMMR2: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    #[inline(always)]
    pub fn is_stop_on_pwmmr2(&self) -> bool {
        *self == PWMMR2S_A::STOP_ON_PWMMR2
    }
}
#[doc = "Field `PWMMR2S` writer - Stop PWM0"]
pub type PWMMR2S_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PWMMR2S_A>;
impl<'a, REG, const O: u8> PWMMR2S_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR2S_A::DISABLED)
    }
    #[doc = "Stop on PWMMR2: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    #[inline(always)]
    pub fn stop_on_pwmmr2(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR2S_A::STOP_ON_PWMMR2)
    }
}
#[doc = "Field `PWMMR3I` reader - Interrupt PWM3"]
pub type PWMMR3I_R = crate::BitReader<PWMMR3I_A>;
#[doc = "Interrupt PWM3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWMMR3I_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt on PWMMR3: an interrupt is generated when PWMMR3 matches the value in the PWMTC."]
    INTERRUPT_ON_PWMMR3 = 1,
}
impl From<PWMMR3I_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR3I_A) -> Self {
        variant as u8 != 0
    }
}
impl PWMMR3I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWMMR3I_A {
        match self.bits {
            false => PWMMR3I_A::DISABLED,
            true => PWMMR3I_A::INTERRUPT_ON_PWMMR3,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR3I_A::DISABLED
    }
    #[doc = "Interrupt on PWMMR3: an interrupt is generated when PWMMR3 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn is_interrupt_on_pwmmr3(&self) -> bool {
        *self == PWMMR3I_A::INTERRUPT_ON_PWMMR3
    }
}
#[doc = "Field `PWMMR3I` writer - Interrupt PWM3"]
pub type PWMMR3I_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PWMMR3I_A>;
impl<'a, REG, const O: u8> PWMMR3I_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR3I_A::DISABLED)
    }
    #[doc = "Interrupt on PWMMR3: an interrupt is generated when PWMMR3 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn interrupt_on_pwmmr3(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR3I_A::INTERRUPT_ON_PWMMR3)
    }
}
#[doc = "Field `PWMMR3R` reader - Reset PWM3"]
pub type PWMMR3R_R = crate::BitReader<PWMMR3R_A>;
#[doc = "Reset PWM3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWMMR3R_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Reset on PWMMR3: the PWMTC will be reset if PWMMR3 matches it."]
    RESET_ON_PWMMR3 = 1,
}
impl From<PWMMR3R_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR3R_A) -> Self {
        variant as u8 != 0
    }
}
impl PWMMR3R_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWMMR3R_A {
        match self.bits {
            false => PWMMR3R_A::DISABLED,
            true => PWMMR3R_A::RESET_ON_PWMMR3,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR3R_A::DISABLED
    }
    #[doc = "Reset on PWMMR3: the PWMTC will be reset if PWMMR3 matches it."]
    #[inline(always)]
    pub fn is_reset_on_pwmmr3(&self) -> bool {
        *self == PWMMR3R_A::RESET_ON_PWMMR3
    }
}
#[doc = "Field `PWMMR3R` writer - Reset PWM3"]
pub type PWMMR3R_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PWMMR3R_A>;
impl<'a, REG, const O: u8> PWMMR3R_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR3R_A::DISABLED)
    }
    #[doc = "Reset on PWMMR3: the PWMTC will be reset if PWMMR3 matches it."]
    #[inline(always)]
    pub fn reset_on_pwmmr3(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR3R_A::RESET_ON_PWMMR3)
    }
}
#[doc = "Field `PWMMR3S` reader - Stop PWM0"]
pub type PWMMR3S_R = crate::BitReader<PWMMR3S_A>;
#[doc = "Stop PWM0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWMMR3S_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Stop on PWMMR3: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    STOP_ON_PWMMR3 = 1,
}
impl From<PWMMR3S_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR3S_A) -> Self {
        variant as u8 != 0
    }
}
impl PWMMR3S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWMMR3S_A {
        match self.bits {
            false => PWMMR3S_A::DISABLED,
            true => PWMMR3S_A::STOP_ON_PWMMR3,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR3S_A::DISABLED
    }
    #[doc = "Stop on PWMMR3: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    #[inline(always)]
    pub fn is_stop_on_pwmmr3(&self) -> bool {
        *self == PWMMR3S_A::STOP_ON_PWMMR3
    }
}
#[doc = "Field `PWMMR3S` writer - Stop PWM0"]
pub type PWMMR3S_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PWMMR3S_A>;
impl<'a, REG, const O: u8> PWMMR3S_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR3S_A::DISABLED)
    }
    #[doc = "Stop on PWMMR3: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    #[inline(always)]
    pub fn stop_on_pwmmr3(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR3S_A::STOP_ON_PWMMR3)
    }
}
#[doc = "Field `PWMMR4I` reader - Interrupt PWM4"]
pub type PWMMR4I_R = crate::BitReader<PWMMR4I_A>;
#[doc = "Interrupt PWM4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWMMR4I_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt on PWMMR4: an interrupt is generated when PWMMR4 matches the value in the PWMTC."]
    INTERRUPT_ON_PWMMR4 = 1,
}
impl From<PWMMR4I_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR4I_A) -> Self {
        variant as u8 != 0
    }
}
impl PWMMR4I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWMMR4I_A {
        match self.bits {
            false => PWMMR4I_A::DISABLED,
            true => PWMMR4I_A::INTERRUPT_ON_PWMMR4,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR4I_A::DISABLED
    }
    #[doc = "Interrupt on PWMMR4: an interrupt is generated when PWMMR4 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn is_interrupt_on_pwmmr4(&self) -> bool {
        *self == PWMMR4I_A::INTERRUPT_ON_PWMMR4
    }
}
#[doc = "Field `PWMMR4I` writer - Interrupt PWM4"]
pub type PWMMR4I_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PWMMR4I_A>;
impl<'a, REG, const O: u8> PWMMR4I_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR4I_A::DISABLED)
    }
    #[doc = "Interrupt on PWMMR4: an interrupt is generated when PWMMR4 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn interrupt_on_pwmmr4(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR4I_A::INTERRUPT_ON_PWMMR4)
    }
}
#[doc = "Field `PWMMR4R` reader - Reset PWM4"]
pub type PWMMR4R_R = crate::BitReader<PWMMR4R_A>;
#[doc = "Reset PWM4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWMMR4R_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Reset on PWMMR4: the PWMTC will be reset if PWMMR4 matches it."]
    RESET_ON_PWMMR4 = 1,
}
impl From<PWMMR4R_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR4R_A) -> Self {
        variant as u8 != 0
    }
}
impl PWMMR4R_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWMMR4R_A {
        match self.bits {
            false => PWMMR4R_A::DISABLED,
            true => PWMMR4R_A::RESET_ON_PWMMR4,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR4R_A::DISABLED
    }
    #[doc = "Reset on PWMMR4: the PWMTC will be reset if PWMMR4 matches it."]
    #[inline(always)]
    pub fn is_reset_on_pwmmr4(&self) -> bool {
        *self == PWMMR4R_A::RESET_ON_PWMMR4
    }
}
#[doc = "Field `PWMMR4R` writer - Reset PWM4"]
pub type PWMMR4R_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PWMMR4R_A>;
impl<'a, REG, const O: u8> PWMMR4R_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR4R_A::DISABLED)
    }
    #[doc = "Reset on PWMMR4: the PWMTC will be reset if PWMMR4 matches it."]
    #[inline(always)]
    pub fn reset_on_pwmmr4(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR4R_A::RESET_ON_PWMMR4)
    }
}
#[doc = "Field `PWMMR4S` reader - Stop PWM4"]
pub type PWMMR4S_R = crate::BitReader<PWMMR4S_A>;
#[doc = "Stop PWM4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWMMR4S_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Stop on PWMMR4: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR4 matches the PWMTC."]
    STOP_ON_PWMMR4 = 1,
}
impl From<PWMMR4S_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR4S_A) -> Self {
        variant as u8 != 0
    }
}
impl PWMMR4S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWMMR4S_A {
        match self.bits {
            false => PWMMR4S_A::DISABLED,
            true => PWMMR4S_A::STOP_ON_PWMMR4,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR4S_A::DISABLED
    }
    #[doc = "Stop on PWMMR4: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR4 matches the PWMTC."]
    #[inline(always)]
    pub fn is_stop_on_pwmmr4(&self) -> bool {
        *self == PWMMR4S_A::STOP_ON_PWMMR4
    }
}
#[doc = "Field `PWMMR4S` writer - Stop PWM4"]
pub type PWMMR4S_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PWMMR4S_A>;
impl<'a, REG, const O: u8> PWMMR4S_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR4S_A::DISABLED)
    }
    #[doc = "Stop on PWMMR4: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR4 matches the PWMTC."]
    #[inline(always)]
    pub fn stop_on_pwmmr4(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR4S_A::STOP_ON_PWMMR4)
    }
}
#[doc = "Field `PWMMR5I` reader - Interrupt PWM5"]
pub type PWMMR5I_R = crate::BitReader<PWMMR5I_A>;
#[doc = "Interrupt PWM5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWMMR5I_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt on PWMMR5: an interrupt is generated when PWMMR5 matches the value in the PWMTC."]
    INTERRUPT_ON_PWMMR5 = 1,
}
impl From<PWMMR5I_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR5I_A) -> Self {
        variant as u8 != 0
    }
}
impl PWMMR5I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWMMR5I_A {
        match self.bits {
            false => PWMMR5I_A::DISABLED,
            true => PWMMR5I_A::INTERRUPT_ON_PWMMR5,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR5I_A::DISABLED
    }
    #[doc = "Interrupt on PWMMR5: an interrupt is generated when PWMMR5 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn is_interrupt_on_pwmmr5(&self) -> bool {
        *self == PWMMR5I_A::INTERRUPT_ON_PWMMR5
    }
}
#[doc = "Field `PWMMR5I` writer - Interrupt PWM5"]
pub type PWMMR5I_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PWMMR5I_A>;
impl<'a, REG, const O: u8> PWMMR5I_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR5I_A::DISABLED)
    }
    #[doc = "Interrupt on PWMMR5: an interrupt is generated when PWMMR5 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn interrupt_on_pwmmr5(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR5I_A::INTERRUPT_ON_PWMMR5)
    }
}
#[doc = "Field `PWMMR5R` reader - Reset PWM5"]
pub type PWMMR5R_R = crate::BitReader<PWMMR5R_A>;
#[doc = "Reset PWM5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWMMR5R_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Reset on PWMMR5: the PWMTC will be reset if PWMMR5 matches it."]
    RESET_ON_PWMMR5 = 1,
}
impl From<PWMMR5R_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR5R_A) -> Self {
        variant as u8 != 0
    }
}
impl PWMMR5R_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWMMR5R_A {
        match self.bits {
            false => PWMMR5R_A::DISABLED,
            true => PWMMR5R_A::RESET_ON_PWMMR5,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR5R_A::DISABLED
    }
    #[doc = "Reset on PWMMR5: the PWMTC will be reset if PWMMR5 matches it."]
    #[inline(always)]
    pub fn is_reset_on_pwmmr5(&self) -> bool {
        *self == PWMMR5R_A::RESET_ON_PWMMR5
    }
}
#[doc = "Field `PWMMR5R` writer - Reset PWM5"]
pub type PWMMR5R_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PWMMR5R_A>;
impl<'a, REG, const O: u8> PWMMR5R_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR5R_A::DISABLED)
    }
    #[doc = "Reset on PWMMR5: the PWMTC will be reset if PWMMR5 matches it."]
    #[inline(always)]
    pub fn reset_on_pwmmr5(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR5R_A::RESET_ON_PWMMR5)
    }
}
#[doc = "Field `PWMMR5S` reader - Stop PWM5"]
pub type PWMMR5S_R = crate::BitReader<PWMMR5S_A>;
#[doc = "Stop PWM5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWMMR5S_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Stop on PWMMR5: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR5 matches the PWMTC."]
    STOP_ON_PWMMR5 = 1,
}
impl From<PWMMR5S_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR5S_A) -> Self {
        variant as u8 != 0
    }
}
impl PWMMR5S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWMMR5S_A {
        match self.bits {
            false => PWMMR5S_A::DISABLED,
            true => PWMMR5S_A::STOP_ON_PWMMR5,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR5S_A::DISABLED
    }
    #[doc = "Stop on PWMMR5: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR5 matches the PWMTC."]
    #[inline(always)]
    pub fn is_stop_on_pwmmr5(&self) -> bool {
        *self == PWMMR5S_A::STOP_ON_PWMMR5
    }
}
#[doc = "Field `PWMMR5S` writer - Stop PWM5"]
pub type PWMMR5S_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PWMMR5S_A>;
impl<'a, REG, const O: u8> PWMMR5S_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR5S_A::DISABLED)
    }
    #[doc = "Stop on PWMMR5: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR5 matches the PWMTC."]
    #[inline(always)]
    pub fn stop_on_pwmmr5(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR5S_A::STOP_ON_PWMMR5)
    }
}
#[doc = "Field `PWMMR6I` reader - Interrupt PWM6"]
pub type PWMMR6I_R = crate::BitReader<PWMMR6I_A>;
#[doc = "Interrupt PWM6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWMMR6I_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt on PWMMR6: an interrupt is generated when PWMMR6 matches the value in the PWMTC."]
    INTERRUPT_ON_PWMMR6 = 1,
}
impl From<PWMMR6I_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR6I_A) -> Self {
        variant as u8 != 0
    }
}
impl PWMMR6I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWMMR6I_A {
        match self.bits {
            false => PWMMR6I_A::DISABLED,
            true => PWMMR6I_A::INTERRUPT_ON_PWMMR6,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR6I_A::DISABLED
    }
    #[doc = "Interrupt on PWMMR6: an interrupt is generated when PWMMR6 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn is_interrupt_on_pwmmr6(&self) -> bool {
        *self == PWMMR6I_A::INTERRUPT_ON_PWMMR6
    }
}
#[doc = "Field `PWMMR6I` writer - Interrupt PWM6"]
pub type PWMMR6I_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PWMMR6I_A>;
impl<'a, REG, const O: u8> PWMMR6I_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR6I_A::DISABLED)
    }
    #[doc = "Interrupt on PWMMR6: an interrupt is generated when PWMMR6 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn interrupt_on_pwmmr6(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR6I_A::INTERRUPT_ON_PWMMR6)
    }
}
#[doc = "Field `PWMMR6R` reader - Reset PWM6"]
pub type PWMMR6R_R = crate::BitReader<PWMMR6R_A>;
#[doc = "Reset PWM6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWMMR6R_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Reset on PWMMR6: the PWMTC will be reset if PWMMR6 matches it."]
    RESET_ON_PWMMR6 = 1,
}
impl From<PWMMR6R_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR6R_A) -> Self {
        variant as u8 != 0
    }
}
impl PWMMR6R_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWMMR6R_A {
        match self.bits {
            false => PWMMR6R_A::DISABLED,
            true => PWMMR6R_A::RESET_ON_PWMMR6,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR6R_A::DISABLED
    }
    #[doc = "Reset on PWMMR6: the PWMTC will be reset if PWMMR6 matches it."]
    #[inline(always)]
    pub fn is_reset_on_pwmmr6(&self) -> bool {
        *self == PWMMR6R_A::RESET_ON_PWMMR6
    }
}
#[doc = "Field `PWMMR6R` writer - Reset PWM6"]
pub type PWMMR6R_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PWMMR6R_A>;
impl<'a, REG, const O: u8> PWMMR6R_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR6R_A::DISABLED)
    }
    #[doc = "Reset on PWMMR6: the PWMTC will be reset if PWMMR6 matches it."]
    #[inline(always)]
    pub fn reset_on_pwmmr6(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR6R_A::RESET_ON_PWMMR6)
    }
}
#[doc = "Field `PWMMR6S` reader - Stop PWM6"]
pub type PWMMR6S_R = crate::BitReader<PWMMR6S_A>;
#[doc = "Stop PWM6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWMMR6S_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Stop on PWMMR6: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR6 matches the PWMTC."]
    STOP_ON_PWMMR6 = 1,
}
impl From<PWMMR6S_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR6S_A) -> Self {
        variant as u8 != 0
    }
}
impl PWMMR6S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PWMMR6S_A {
        match self.bits {
            false => PWMMR6S_A::DISABLED,
            true => PWMMR6S_A::STOP_ON_PWMMR6,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR6S_A::DISABLED
    }
    #[doc = "Stop on PWMMR6: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR6 matches the PWMTC."]
    #[inline(always)]
    pub fn is_stop_on_pwmmr6(&self) -> bool {
        *self == PWMMR6S_A::STOP_ON_PWMMR6
    }
}
#[doc = "Field `PWMMR6S` writer - Stop PWM6"]
pub type PWMMR6S_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PWMMR6S_A>;
impl<'a, REG, const O: u8> PWMMR6S_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR6S_A::DISABLED)
    }
    #[doc = "Stop on PWMMR6: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR6 matches the PWMTC."]
    #[inline(always)]
    pub fn stop_on_pwmmr6(self) -> &'a mut crate::W<REG> {
        self.variant(PWMMR6S_A::STOP_ON_PWMMR6)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt PWM0"]
    #[inline(always)]
    pub fn pwmmr0i(&self) -> PWMMR0I_R {
        PWMMR0I_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset PWM0"]
    #[inline(always)]
    pub fn pwmmr0r(&self) -> PWMMR0R_R {
        PWMMR0R_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stop PWM0"]
    #[inline(always)]
    pub fn pwmmr0s(&self) -> PWMMR0S_R {
        PWMMR0S_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt PWM1"]
    #[inline(always)]
    pub fn pwmmr1i(&self) -> PWMMR1I_R {
        PWMMR1I_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset PWM1"]
    #[inline(always)]
    pub fn pwmmr1r(&self) -> PWMMR1R_R {
        PWMMR1R_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stop PWM1"]
    #[inline(always)]
    pub fn pwmmr1s(&self) -> PWMMR1S_R {
        PWMMR1S_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt PWM0"]
    #[inline(always)]
    pub fn pwmmr2i(&self) -> PWMMR2I_R {
        PWMMR2I_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reset PWM0"]
    #[inline(always)]
    pub fn pwmmr2r(&self) -> PWMMR2R_R {
        PWMMR2R_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Stop PWM0"]
    #[inline(always)]
    pub fn pwmmr2s(&self) -> PWMMR2S_R {
        PWMMR2S_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt PWM3"]
    #[inline(always)]
    pub fn pwmmr3i(&self) -> PWMMR3I_R {
        PWMMR3I_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reset PWM3"]
    #[inline(always)]
    pub fn pwmmr3r(&self) -> PWMMR3R_R {
        PWMMR3R_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Stop PWM0"]
    #[inline(always)]
    pub fn pwmmr3s(&self) -> PWMMR3S_R {
        PWMMR3S_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt PWM4"]
    #[inline(always)]
    pub fn pwmmr4i(&self) -> PWMMR4I_R {
        PWMMR4I_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reset PWM4"]
    #[inline(always)]
    pub fn pwmmr4r(&self) -> PWMMR4R_R {
        PWMMR4R_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Stop PWM4"]
    #[inline(always)]
    pub fn pwmmr4s(&self) -> PWMMR4S_R {
        PWMMR4S_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt PWM5"]
    #[inline(always)]
    pub fn pwmmr5i(&self) -> PWMMR5I_R {
        PWMMR5I_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Reset PWM5"]
    #[inline(always)]
    pub fn pwmmr5r(&self) -> PWMMR5R_R {
        PWMMR5R_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Stop PWM5"]
    #[inline(always)]
    pub fn pwmmr5s(&self) -> PWMMR5S_R {
        PWMMR5S_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt PWM6"]
    #[inline(always)]
    pub fn pwmmr6i(&self) -> PWMMR6I_R {
        PWMMR6I_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Reset PWM6"]
    #[inline(always)]
    pub fn pwmmr6r(&self) -> PWMMR6R_R {
        PWMMR6R_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Stop PWM6"]
    #[inline(always)]
    pub fn pwmmr6s(&self) -> PWMMR6S_R {
        PWMMR6S_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCR")
            .field("pwmmr0i", &format_args!("{}", self.pwmmr0i().bit()))
            .field("pwmmr0r", &format_args!("{}", self.pwmmr0r().bit()))
            .field("pwmmr0s", &format_args!("{}", self.pwmmr0s().bit()))
            .field("pwmmr1i", &format_args!("{}", self.pwmmr1i().bit()))
            .field("pwmmr1r", &format_args!("{}", self.pwmmr1r().bit()))
            .field("pwmmr1s", &format_args!("{}", self.pwmmr1s().bit()))
            .field("pwmmr2i", &format_args!("{}", self.pwmmr2i().bit()))
            .field("pwmmr2r", &format_args!("{}", self.pwmmr2r().bit()))
            .field("pwmmr2s", &format_args!("{}", self.pwmmr2s().bit()))
            .field("pwmmr3i", &format_args!("{}", self.pwmmr3i().bit()))
            .field("pwmmr3r", &format_args!("{}", self.pwmmr3r().bit()))
            .field("pwmmr3s", &format_args!("{}", self.pwmmr3s().bit()))
            .field("pwmmr4i", &format_args!("{}", self.pwmmr4i().bit()))
            .field("pwmmr4r", &format_args!("{}", self.pwmmr4r().bit()))
            .field("pwmmr4s", &format_args!("{}", self.pwmmr4s().bit()))
            .field("pwmmr5i", &format_args!("{}", self.pwmmr5i().bit()))
            .field("pwmmr5r", &format_args!("{}", self.pwmmr5r().bit()))
            .field("pwmmr5s", &format_args!("{}", self.pwmmr5s().bit()))
            .field("pwmmr6i", &format_args!("{}", self.pwmmr6i().bit()))
            .field("pwmmr6r", &format_args!("{}", self.pwmmr6r().bit()))
            .field("pwmmr6s", &format_args!("{}", self.pwmmr6s().bit()))
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
    #[doc = "Bit 0 - Interrupt PWM0"]
    #[inline(always)]
    #[must_use]
    pub fn pwmmr0i(&mut self) -> PWMMR0I_W<MCR_SPEC, 0> {
        PWMMR0I_W::new(self)
    }
    #[doc = "Bit 1 - Reset PWM0"]
    #[inline(always)]
    #[must_use]
    pub fn pwmmr0r(&mut self) -> PWMMR0R_W<MCR_SPEC, 1> {
        PWMMR0R_W::new(self)
    }
    #[doc = "Bit 2 - Stop PWM0"]
    #[inline(always)]
    #[must_use]
    pub fn pwmmr0s(&mut self) -> PWMMR0S_W<MCR_SPEC, 2> {
        PWMMR0S_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt PWM1"]
    #[inline(always)]
    #[must_use]
    pub fn pwmmr1i(&mut self) -> PWMMR1I_W<MCR_SPEC, 3> {
        PWMMR1I_W::new(self)
    }
    #[doc = "Bit 4 - Reset PWM1"]
    #[inline(always)]
    #[must_use]
    pub fn pwmmr1r(&mut self) -> PWMMR1R_W<MCR_SPEC, 4> {
        PWMMR1R_W::new(self)
    }
    #[doc = "Bit 5 - Stop PWM1"]
    #[inline(always)]
    #[must_use]
    pub fn pwmmr1s(&mut self) -> PWMMR1S_W<MCR_SPEC, 5> {
        PWMMR1S_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt PWM0"]
    #[inline(always)]
    #[must_use]
    pub fn pwmmr2i(&mut self) -> PWMMR2I_W<MCR_SPEC, 6> {
        PWMMR2I_W::new(self)
    }
    #[doc = "Bit 7 - Reset PWM0"]
    #[inline(always)]
    #[must_use]
    pub fn pwmmr2r(&mut self) -> PWMMR2R_W<MCR_SPEC, 7> {
        PWMMR2R_W::new(self)
    }
    #[doc = "Bit 8 - Stop PWM0"]
    #[inline(always)]
    #[must_use]
    pub fn pwmmr2s(&mut self) -> PWMMR2S_W<MCR_SPEC, 8> {
        PWMMR2S_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt PWM3"]
    #[inline(always)]
    #[must_use]
    pub fn pwmmr3i(&mut self) -> PWMMR3I_W<MCR_SPEC, 9> {
        PWMMR3I_W::new(self)
    }
    #[doc = "Bit 10 - Reset PWM3"]
    #[inline(always)]
    #[must_use]
    pub fn pwmmr3r(&mut self) -> PWMMR3R_W<MCR_SPEC, 10> {
        PWMMR3R_W::new(self)
    }
    #[doc = "Bit 11 - Stop PWM0"]
    #[inline(always)]
    #[must_use]
    pub fn pwmmr3s(&mut self) -> PWMMR3S_W<MCR_SPEC, 11> {
        PWMMR3S_W::new(self)
    }
    #[doc = "Bit 12 - Interrupt PWM4"]
    #[inline(always)]
    #[must_use]
    pub fn pwmmr4i(&mut self) -> PWMMR4I_W<MCR_SPEC, 12> {
        PWMMR4I_W::new(self)
    }
    #[doc = "Bit 13 - Reset PWM4"]
    #[inline(always)]
    #[must_use]
    pub fn pwmmr4r(&mut self) -> PWMMR4R_W<MCR_SPEC, 13> {
        PWMMR4R_W::new(self)
    }
    #[doc = "Bit 14 - Stop PWM4"]
    #[inline(always)]
    #[must_use]
    pub fn pwmmr4s(&mut self) -> PWMMR4S_W<MCR_SPEC, 14> {
        PWMMR4S_W::new(self)
    }
    #[doc = "Bit 15 - Interrupt PWM5"]
    #[inline(always)]
    #[must_use]
    pub fn pwmmr5i(&mut self) -> PWMMR5I_W<MCR_SPEC, 15> {
        PWMMR5I_W::new(self)
    }
    #[doc = "Bit 16 - Reset PWM5"]
    #[inline(always)]
    #[must_use]
    pub fn pwmmr5r(&mut self) -> PWMMR5R_W<MCR_SPEC, 16> {
        PWMMR5R_W::new(self)
    }
    #[doc = "Bit 17 - Stop PWM5"]
    #[inline(always)]
    #[must_use]
    pub fn pwmmr5s(&mut self) -> PWMMR5S_W<MCR_SPEC, 17> {
        PWMMR5S_W::new(self)
    }
    #[doc = "Bit 18 - Interrupt PWM6"]
    #[inline(always)]
    #[must_use]
    pub fn pwmmr6i(&mut self) -> PWMMR6I_W<MCR_SPEC, 18> {
        PWMMR6I_W::new(self)
    }
    #[doc = "Bit 19 - Reset PWM6"]
    #[inline(always)]
    #[must_use]
    pub fn pwmmr6r(&mut self) -> PWMMR6R_W<MCR_SPEC, 19> {
        PWMMR6R_W::new(self)
    }
    #[doc = "Bit 20 - Stop PWM6"]
    #[inline(always)]
    #[must_use]
    pub fn pwmmr6s(&mut self) -> PWMMR6S_W<MCR_SPEC, 20> {
        PWMMR6S_W::new(self)
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
#[doc = "Match Control Register. The MCR is used to control whether an interrupt is generated and if the PWM counter is reset when a Match occurs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
