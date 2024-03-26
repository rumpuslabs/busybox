#[doc = "Register `PCLKSEL0` reader"]
pub type R = crate::R<PCLKSEL0_SPEC>;
#[doc = "Register `PCLKSEL0` writer"]
pub type W = crate::W<PCLKSEL0_SPEC>;
#[doc = "Field `PCLK_WDT` reader - Peripheral clock selection for WDT."]
pub type PCLK_WDT_R = crate::FieldReader<PCLK_WDT_A>;
#[doc = "Peripheral clock selection for WDT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCLK_WDT_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_WDT_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_WDT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCLK_WDT_A {
    type Ux = u8;
}
impl PCLK_WDT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCLK_WDT_A {
        match self.bits {
            0 => PCLK_WDT_A::CCLK_DIV_4,
            1 => PCLK_WDT_A::CCLK,
            2 => PCLK_WDT_A::CCLK_DIV_2,
            3 => PCLK_WDT_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_WDT_A::CCLK_DIV_4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_WDT_A::CCLK
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_WDT_A::CCLK_DIV_2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_WDT_A::CCLK_DIV_8
    }
}
#[doc = "Field `PCLK_WDT` writer - Peripheral clock selection for WDT."]
pub type PCLK_WDT_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, PCLK_WDT_A>;
impl<'a, REG, const O: u8> PCLK_WDT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_WDT_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_WDT_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_WDT_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_WDT_A::CCLK_DIV_8)
    }
}
#[doc = "Field `PCLK_TIMER0` reader - Peripheral clock selection for TIMER0."]
pub type PCLK_TIMER0_R = crate::FieldReader<PCLK_TIMER0_A>;
#[doc = "Peripheral clock selection for TIMER0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCLK_TIMER0_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_TIMER0_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_TIMER0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCLK_TIMER0_A {
    type Ux = u8;
}
impl PCLK_TIMER0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCLK_TIMER0_A {
        match self.bits {
            0 => PCLK_TIMER0_A::CCLK_DIV_4,
            1 => PCLK_TIMER0_A::CCLK,
            2 => PCLK_TIMER0_A::CCLK_DIV_2,
            3 => PCLK_TIMER0_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_TIMER0_A::CCLK_DIV_4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_TIMER0_A::CCLK
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_TIMER0_A::CCLK_DIV_2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_TIMER0_A::CCLK_DIV_8
    }
}
#[doc = "Field `PCLK_TIMER0` writer - Peripheral clock selection for TIMER0."]
pub type PCLK_TIMER0_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, PCLK_TIMER0_A>;
impl<'a, REG, const O: u8> PCLK_TIMER0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_TIMER0_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_TIMER0_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_TIMER0_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_TIMER0_A::CCLK_DIV_8)
    }
}
#[doc = "Field `PCLK_TIMER1` reader - Peripheral clock selection for TIMER1."]
pub type PCLK_TIMER1_R = crate::FieldReader<PCLK_TIMER1_A>;
#[doc = "Peripheral clock selection for TIMER1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCLK_TIMER1_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_TIMER1_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_TIMER1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCLK_TIMER1_A {
    type Ux = u8;
}
impl PCLK_TIMER1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCLK_TIMER1_A {
        match self.bits {
            0 => PCLK_TIMER1_A::CCLK_DIV_4,
            1 => PCLK_TIMER1_A::CCLK,
            2 => PCLK_TIMER1_A::CCLK_DIV_2,
            3 => PCLK_TIMER1_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_TIMER1_A::CCLK_DIV_4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_TIMER1_A::CCLK
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_TIMER1_A::CCLK_DIV_2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_TIMER1_A::CCLK_DIV_8
    }
}
#[doc = "Field `PCLK_TIMER1` writer - Peripheral clock selection for TIMER1."]
pub type PCLK_TIMER1_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, PCLK_TIMER1_A>;
impl<'a, REG, const O: u8> PCLK_TIMER1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_TIMER1_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_TIMER1_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_TIMER1_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_TIMER1_A::CCLK_DIV_8)
    }
}
#[doc = "Field `PCLK_UART0` reader - Peripheral clock selection for UART0."]
pub type PCLK_UART0_R = crate::FieldReader<PCLK_UART0_A>;
#[doc = "Peripheral clock selection for UART0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCLK_UART0_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_UART0_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_UART0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCLK_UART0_A {
    type Ux = u8;
}
impl PCLK_UART0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCLK_UART0_A {
        match self.bits {
            0 => PCLK_UART0_A::CCLK_DIV_4,
            1 => PCLK_UART0_A::CCLK,
            2 => PCLK_UART0_A::CCLK_DIV_2,
            3 => PCLK_UART0_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_UART0_A::CCLK_DIV_4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_UART0_A::CCLK
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_UART0_A::CCLK_DIV_2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_UART0_A::CCLK_DIV_8
    }
}
#[doc = "Field `PCLK_UART0` writer - Peripheral clock selection for UART0."]
pub type PCLK_UART0_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, PCLK_UART0_A>;
impl<'a, REG, const O: u8> PCLK_UART0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_UART0_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_UART0_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_UART0_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_UART0_A::CCLK_DIV_8)
    }
}
#[doc = "Field `PCLK_UART1` reader - Peripheral clock selection for UART1."]
pub type PCLK_UART1_R = crate::FieldReader<PCLK_UART1_A>;
#[doc = "Peripheral clock selection for UART1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCLK_UART1_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_UART1_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_UART1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCLK_UART1_A {
    type Ux = u8;
}
impl PCLK_UART1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCLK_UART1_A {
        match self.bits {
            0 => PCLK_UART1_A::CCLK_DIV_4,
            1 => PCLK_UART1_A::CCLK,
            2 => PCLK_UART1_A::CCLK_DIV_2,
            3 => PCLK_UART1_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_UART1_A::CCLK_DIV_4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_UART1_A::CCLK
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_UART1_A::CCLK_DIV_2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_UART1_A::CCLK_DIV_8
    }
}
#[doc = "Field `PCLK_UART1` writer - Peripheral clock selection for UART1."]
pub type PCLK_UART1_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, PCLK_UART1_A>;
impl<'a, REG, const O: u8> PCLK_UART1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_UART1_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_UART1_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_UART1_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_UART1_A::CCLK_DIV_8)
    }
}
#[doc = "Field `PCLK_PWM1` reader - Peripheral clock selection for PWM1."]
pub type PCLK_PWM1_R = crate::FieldReader<PCLK_PWM1_A>;
#[doc = "Peripheral clock selection for PWM1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCLK_PWM1_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_PWM1_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_PWM1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCLK_PWM1_A {
    type Ux = u8;
}
impl PCLK_PWM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCLK_PWM1_A {
        match self.bits {
            0 => PCLK_PWM1_A::CCLK_DIV_4,
            1 => PCLK_PWM1_A::CCLK,
            2 => PCLK_PWM1_A::CCLK_DIV_2,
            3 => PCLK_PWM1_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_PWM1_A::CCLK_DIV_4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_PWM1_A::CCLK
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_PWM1_A::CCLK_DIV_2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_PWM1_A::CCLK_DIV_8
    }
}
#[doc = "Field `PCLK_PWM1` writer - Peripheral clock selection for PWM1."]
pub type PCLK_PWM1_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, PCLK_PWM1_A>;
impl<'a, REG, const O: u8> PCLK_PWM1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_PWM1_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_PWM1_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_PWM1_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_PWM1_A::CCLK_DIV_8)
    }
}
#[doc = "Field `PCLK_I2C0` reader - Peripheral clock selection for I2C0."]
pub type PCLK_I2C0_R = crate::FieldReader<PCLK_I2C0_A>;
#[doc = "Peripheral clock selection for I2C0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCLK_I2C0_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_I2C0_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_I2C0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCLK_I2C0_A {
    type Ux = u8;
}
impl PCLK_I2C0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCLK_I2C0_A {
        match self.bits {
            0 => PCLK_I2C0_A::CCLK_DIV_4,
            1 => PCLK_I2C0_A::CCLK,
            2 => PCLK_I2C0_A::CCLK_DIV_2,
            3 => PCLK_I2C0_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_I2C0_A::CCLK_DIV_4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_I2C0_A::CCLK
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_I2C0_A::CCLK_DIV_2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_I2C0_A::CCLK_DIV_8
    }
}
#[doc = "Field `PCLK_I2C0` writer - Peripheral clock selection for I2C0."]
pub type PCLK_I2C0_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, PCLK_I2C0_A>;
impl<'a, REG, const O: u8> PCLK_I2C0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_I2C0_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_I2C0_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_I2C0_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_I2C0_A::CCLK_DIV_8)
    }
}
#[doc = "Field `PCLK_SPI` reader - Peripheral clock selection for SPI."]
pub type PCLK_SPI_R = crate::FieldReader<PCLK_SPI_A>;
#[doc = "Peripheral clock selection for SPI.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCLK_SPI_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_SPI_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_SPI_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCLK_SPI_A {
    type Ux = u8;
}
impl PCLK_SPI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCLK_SPI_A {
        match self.bits {
            0 => PCLK_SPI_A::CCLK_DIV_4,
            1 => PCLK_SPI_A::CCLK,
            2 => PCLK_SPI_A::CCLK_DIV_2,
            3 => PCLK_SPI_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_SPI_A::CCLK_DIV_4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_SPI_A::CCLK
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_SPI_A::CCLK_DIV_2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_SPI_A::CCLK_DIV_8
    }
}
#[doc = "Field `PCLK_SPI` writer - Peripheral clock selection for SPI."]
pub type PCLK_SPI_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, PCLK_SPI_A>;
impl<'a, REG, const O: u8> PCLK_SPI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_SPI_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_SPI_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_SPI_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_SPI_A::CCLK_DIV_8)
    }
}
#[doc = "Field `PCLK_SSP1` reader - Peripheral clock selection for SSP1."]
pub type PCLK_SSP1_R = crate::FieldReader<PCLK_SSP1_A>;
#[doc = "Peripheral clock selection for SSP1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCLK_SSP1_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_SSP1_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_SSP1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCLK_SSP1_A {
    type Ux = u8;
}
impl PCLK_SSP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCLK_SSP1_A {
        match self.bits {
            0 => PCLK_SSP1_A::CCLK_DIV_4,
            1 => PCLK_SSP1_A::CCLK,
            2 => PCLK_SSP1_A::CCLK_DIV_2,
            3 => PCLK_SSP1_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_SSP1_A::CCLK_DIV_4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_SSP1_A::CCLK
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_SSP1_A::CCLK_DIV_2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_SSP1_A::CCLK_DIV_8
    }
}
#[doc = "Field `PCLK_SSP1` writer - Peripheral clock selection for SSP1."]
pub type PCLK_SSP1_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, PCLK_SSP1_A>;
impl<'a, REG, const O: u8> PCLK_SSP1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_SSP1_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_SSP1_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_SSP1_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_SSP1_A::CCLK_DIV_8)
    }
}
#[doc = "Field `PCLK_DAC` reader - Peripheral clock selection for DAC."]
pub type PCLK_DAC_R = crate::FieldReader<PCLK_DAC_A>;
#[doc = "Peripheral clock selection for DAC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCLK_DAC_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_DAC_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_DAC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCLK_DAC_A {
    type Ux = u8;
}
impl PCLK_DAC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCLK_DAC_A {
        match self.bits {
            0 => PCLK_DAC_A::CCLK_DIV_4,
            1 => PCLK_DAC_A::CCLK,
            2 => PCLK_DAC_A::CCLK_DIV_2,
            3 => PCLK_DAC_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_DAC_A::CCLK_DIV_4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_DAC_A::CCLK
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_DAC_A::CCLK_DIV_2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_DAC_A::CCLK_DIV_8
    }
}
#[doc = "Field `PCLK_DAC` writer - Peripheral clock selection for DAC."]
pub type PCLK_DAC_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, PCLK_DAC_A>;
impl<'a, REG, const O: u8> PCLK_DAC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_DAC_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_DAC_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_DAC_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_DAC_A::CCLK_DIV_8)
    }
}
#[doc = "Field `PCLK_ADC` reader - Peripheral clock selection for ADC."]
pub type PCLK_ADC_R = crate::FieldReader<PCLK_ADC_A>;
#[doc = "Peripheral clock selection for ADC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCLK_ADC_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_ADC_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_ADC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCLK_ADC_A {
    type Ux = u8;
}
impl PCLK_ADC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCLK_ADC_A {
        match self.bits {
            0 => PCLK_ADC_A::CCLK_DIV_4,
            1 => PCLK_ADC_A::CCLK,
            2 => PCLK_ADC_A::CCLK_DIV_2,
            3 => PCLK_ADC_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_ADC_A::CCLK_DIV_4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_ADC_A::CCLK
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_ADC_A::CCLK_DIV_2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_ADC_A::CCLK_DIV_8
    }
}
#[doc = "Field `PCLK_ADC` writer - Peripheral clock selection for ADC."]
pub type PCLK_ADC_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, PCLK_ADC_A>;
impl<'a, REG, const O: u8> PCLK_ADC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_ADC_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_ADC_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_ADC_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_ADC_A::CCLK_DIV_8)
    }
}
#[doc = "Field `PCLK_CAN1` reader - Peripheral clock selection for CAN1.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
pub type PCLK_CAN1_R = crate::FieldReader<PCLK_CAN1_A>;
#[doc = "Peripheral clock selection for CAN1.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCLK_CAN1_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 6. PCLK_peripheral = CCLK/6."]
    CCLK_DIV_6 = 3,
}
impl From<PCLK_CAN1_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_CAN1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCLK_CAN1_A {
    type Ux = u8;
}
impl PCLK_CAN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCLK_CAN1_A {
        match self.bits {
            0 => PCLK_CAN1_A::CCLK_DIV_4,
            1 => PCLK_CAN1_A::CCLK,
            2 => PCLK_CAN1_A::CCLK_DIV_2,
            3 => PCLK_CAN1_A::CCLK_DIV_6,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_CAN1_A::CCLK_DIV_4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_CAN1_A::CCLK
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_CAN1_A::CCLK_DIV_2
    }
    #[doc = "CCLK div 6. PCLK_peripheral = CCLK/6."]
    #[inline(always)]
    pub fn is_cclk_div_6(&self) -> bool {
        *self == PCLK_CAN1_A::CCLK_DIV_6
    }
}
#[doc = "Field `PCLK_CAN1` writer - Peripheral clock selection for CAN1.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
pub type PCLK_CAN1_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, PCLK_CAN1_A>;
impl<'a, REG, const O: u8> PCLK_CAN1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_CAN1_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_CAN1_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_CAN1_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 6. PCLK_peripheral = CCLK/6."]
    #[inline(always)]
    pub fn cclk_div_6(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_CAN1_A::CCLK_DIV_6)
    }
}
#[doc = "Field `PCLK_CAN2` reader - Peripheral clock selection for CAN2.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
pub type PCLK_CAN2_R = crate::FieldReader<PCLK_CAN2_A>;
#[doc = "Peripheral clock selection for CAN2.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCLK_CAN2_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 6. PCLK_peripheral = CCLK/6,"]
    CCLK_DIV_6 = 3,
}
impl From<PCLK_CAN2_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_CAN2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCLK_CAN2_A {
    type Ux = u8;
}
impl PCLK_CAN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCLK_CAN2_A {
        match self.bits {
            0 => PCLK_CAN2_A::CCLK_DIV_4,
            1 => PCLK_CAN2_A::CCLK,
            2 => PCLK_CAN2_A::CCLK_DIV_2,
            3 => PCLK_CAN2_A::CCLK_DIV_6,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_CAN2_A::CCLK_DIV_4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_CAN2_A::CCLK
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_CAN2_A::CCLK_DIV_2
    }
    #[doc = "CCLK div 6. PCLK_peripheral = CCLK/6,"]
    #[inline(always)]
    pub fn is_cclk_div_6(&self) -> bool {
        *self == PCLK_CAN2_A::CCLK_DIV_6
    }
}
#[doc = "Field `PCLK_CAN2` writer - Peripheral clock selection for CAN2.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
pub type PCLK_CAN2_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, PCLK_CAN2_A>;
impl<'a, REG, const O: u8> PCLK_CAN2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_CAN2_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_CAN2_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_CAN2_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 6. PCLK_peripheral = CCLK/6,"]
    #[inline(always)]
    pub fn cclk_div_6(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_CAN2_A::CCLK_DIV_6)
    }
}
#[doc = "Field `PCLK_ACF` reader - Peripheral clock selection for CAN acceptance filtering.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
pub type PCLK_ACF_R = crate::FieldReader<PCLK_ACF_A>;
#[doc = "Peripheral clock selection for CAN acceptance filtering.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCLK_ACF_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 6. PCLK_peripheral = CCLK/6"]
    CCLK_DIV_6 = 3,
}
impl From<PCLK_ACF_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_ACF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCLK_ACF_A {
    type Ux = u8;
}
impl PCLK_ACF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCLK_ACF_A {
        match self.bits {
            0 => PCLK_ACF_A::CCLK_DIV_4,
            1 => PCLK_ACF_A::CCLK,
            2 => PCLK_ACF_A::CCLK_DIV_2,
            3 => PCLK_ACF_A::CCLK_DIV_6,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_ACF_A::CCLK_DIV_4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_ACF_A::CCLK
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_ACF_A::CCLK_DIV_2
    }
    #[doc = "CCLK div 6. PCLK_peripheral = CCLK/6"]
    #[inline(always)]
    pub fn is_cclk_div_6(&self) -> bool {
        *self == PCLK_ACF_A::CCLK_DIV_6
    }
}
#[doc = "Field `PCLK_ACF` writer - Peripheral clock selection for CAN acceptance filtering.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
pub type PCLK_ACF_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, PCLK_ACF_A>;
impl<'a, REG, const O: u8> PCLK_ACF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_ACF_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_ACF_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_ACF_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 6. PCLK_peripheral = CCLK/6"]
    #[inline(always)]
    pub fn cclk_div_6(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_ACF_A::CCLK_DIV_6)
    }
}
impl R {
    #[doc = "Bits 0:1 - Peripheral clock selection for WDT."]
    #[inline(always)]
    pub fn pclk_wdt(&self) -> PCLK_WDT_R {
        PCLK_WDT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Peripheral clock selection for TIMER0."]
    #[inline(always)]
    pub fn pclk_timer0(&self) -> PCLK_TIMER0_R {
        PCLK_TIMER0_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Peripheral clock selection for TIMER1."]
    #[inline(always)]
    pub fn pclk_timer1(&self) -> PCLK_TIMER1_R {
        PCLK_TIMER1_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Peripheral clock selection for UART0."]
    #[inline(always)]
    pub fn pclk_uart0(&self) -> PCLK_UART0_R {
        PCLK_UART0_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Peripheral clock selection for UART1."]
    #[inline(always)]
    pub fn pclk_uart1(&self) -> PCLK_UART1_R {
        PCLK_UART1_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Peripheral clock selection for PWM1."]
    #[inline(always)]
    pub fn pclk_pwm1(&self) -> PCLK_PWM1_R {
        PCLK_PWM1_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Peripheral clock selection for I2C0."]
    #[inline(always)]
    pub fn pclk_i2c0(&self) -> PCLK_I2C0_R {
        PCLK_I2C0_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Peripheral clock selection for SPI."]
    #[inline(always)]
    pub fn pclk_spi(&self) -> PCLK_SPI_R {
        PCLK_SPI_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Peripheral clock selection for SSP1."]
    #[inline(always)]
    pub fn pclk_ssp1(&self) -> PCLK_SSP1_R {
        PCLK_SSP1_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Peripheral clock selection for DAC."]
    #[inline(always)]
    pub fn pclk_dac(&self) -> PCLK_DAC_R {
        PCLK_DAC_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Peripheral clock selection for ADC."]
    #[inline(always)]
    pub fn pclk_adc(&self) -> PCLK_ADC_R {
        PCLK_ADC_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Peripheral clock selection for CAN1.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
    #[inline(always)]
    pub fn pclk_can1(&self) -> PCLK_CAN1_R {
        PCLK_CAN1_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Peripheral clock selection for CAN2.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
    #[inline(always)]
    pub fn pclk_can2(&self) -> PCLK_CAN2_R {
        PCLK_CAN2_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Peripheral clock selection for CAN acceptance filtering.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
    #[inline(always)]
    pub fn pclk_acf(&self) -> PCLK_ACF_R {
        PCLK_ACF_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCLKSEL0")
            .field("pclk_wdt", &format_args!("{}", self.pclk_wdt().bits()))
            .field(
                "pclk_timer0",
                &format_args!("{}", self.pclk_timer0().bits()),
            )
            .field(
                "pclk_timer1",
                &format_args!("{}", self.pclk_timer1().bits()),
            )
            .field("pclk_uart0", &format_args!("{}", self.pclk_uart0().bits()))
            .field("pclk_uart1", &format_args!("{}", self.pclk_uart1().bits()))
            .field("pclk_pwm1", &format_args!("{}", self.pclk_pwm1().bits()))
            .field("pclk_i2c0", &format_args!("{}", self.pclk_i2c0().bits()))
            .field("pclk_spi", &format_args!("{}", self.pclk_spi().bits()))
            .field("pclk_ssp1", &format_args!("{}", self.pclk_ssp1().bits()))
            .field("pclk_dac", &format_args!("{}", self.pclk_dac().bits()))
            .field("pclk_adc", &format_args!("{}", self.pclk_adc().bits()))
            .field("pclk_can1", &format_args!("{}", self.pclk_can1().bits()))
            .field("pclk_can2", &format_args!("{}", self.pclk_can2().bits()))
            .field("pclk_acf", &format_args!("{}", self.pclk_acf().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<PCLKSEL0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Peripheral clock selection for WDT."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_wdt(&mut self) -> PCLK_WDT_W<PCLKSEL0_SPEC, 0> {
        PCLK_WDT_W::new(self)
    }
    #[doc = "Bits 2:3 - Peripheral clock selection for TIMER0."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_timer0(&mut self) -> PCLK_TIMER0_W<PCLKSEL0_SPEC, 2> {
        PCLK_TIMER0_W::new(self)
    }
    #[doc = "Bits 4:5 - Peripheral clock selection for TIMER1."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_timer1(&mut self) -> PCLK_TIMER1_W<PCLKSEL0_SPEC, 4> {
        PCLK_TIMER1_W::new(self)
    }
    #[doc = "Bits 6:7 - Peripheral clock selection for UART0."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_uart0(&mut self) -> PCLK_UART0_W<PCLKSEL0_SPEC, 6> {
        PCLK_UART0_W::new(self)
    }
    #[doc = "Bits 8:9 - Peripheral clock selection for UART1."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_uart1(&mut self) -> PCLK_UART1_W<PCLKSEL0_SPEC, 8> {
        PCLK_UART1_W::new(self)
    }
    #[doc = "Bits 12:13 - Peripheral clock selection for PWM1."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_pwm1(&mut self) -> PCLK_PWM1_W<PCLKSEL0_SPEC, 12> {
        PCLK_PWM1_W::new(self)
    }
    #[doc = "Bits 14:15 - Peripheral clock selection for I2C0."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_i2c0(&mut self) -> PCLK_I2C0_W<PCLKSEL0_SPEC, 14> {
        PCLK_I2C0_W::new(self)
    }
    #[doc = "Bits 16:17 - Peripheral clock selection for SPI."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_spi(&mut self) -> PCLK_SPI_W<PCLKSEL0_SPEC, 16> {
        PCLK_SPI_W::new(self)
    }
    #[doc = "Bits 20:21 - Peripheral clock selection for SSP1."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_ssp1(&mut self) -> PCLK_SSP1_W<PCLKSEL0_SPEC, 20> {
        PCLK_SSP1_W::new(self)
    }
    #[doc = "Bits 22:23 - Peripheral clock selection for DAC."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_dac(&mut self) -> PCLK_DAC_W<PCLKSEL0_SPEC, 22> {
        PCLK_DAC_W::new(self)
    }
    #[doc = "Bits 24:25 - Peripheral clock selection for ADC."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_adc(&mut self) -> PCLK_ADC_W<PCLKSEL0_SPEC, 24> {
        PCLK_ADC_W::new(self)
    }
    #[doc = "Bits 26:27 - Peripheral clock selection for CAN1.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_can1(&mut self) -> PCLK_CAN1_W<PCLKSEL0_SPEC, 26> {
        PCLK_CAN1_W::new(self)
    }
    #[doc = "Bits 28:29 - Peripheral clock selection for CAN2.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_can2(&mut self) -> PCLK_CAN2_W<PCLKSEL0_SPEC, 28> {
        PCLK_CAN2_W::new(self)
    }
    #[doc = "Bits 30:31 - Peripheral clock selection for CAN acceptance filtering.PCLK_CAN1 and PCLK_CAN2 must have the same PCLK divide value when the CAN function is used."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_acf(&mut self) -> PCLK_ACF_W<PCLKSEL0_SPEC, 30> {
        PCLK_ACF_W::new(self)
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
#[doc = "Peripheral Clock Selection register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pclksel0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pclksel0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCLKSEL0_SPEC;
impl crate::RegisterSpec for PCLKSEL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pclksel0::R`](R) reader structure"]
impl crate::Readable for PCLKSEL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pclksel0::W`](W) writer structure"]
impl crate::Writable for PCLKSEL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCLKSEL0 to value 0"]
impl crate::Resettable for PCLKSEL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
