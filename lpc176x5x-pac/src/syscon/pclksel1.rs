#[doc = "Register `PCLKSEL1` reader"]
pub type R = crate::R<PCLKSEL1_SPEC>;
#[doc = "Register `PCLKSEL1` writer"]
pub type W = crate::W<PCLKSEL1_SPEC>;
#[doc = "Field `PCLK_QEI` reader - Peripheral clock selection for the Quadrature Encoder Interface."]
pub type PCLK_QEI_R = crate::FieldReader<PCLK_QEI_A>;
#[doc = "Peripheral clock selection for the Quadrature Encoder Interface.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCLK_QEI_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_QEI_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_QEI_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCLK_QEI_A {
    type Ux = u8;
}
impl PCLK_QEI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCLK_QEI_A {
        match self.bits {
            0 => PCLK_QEI_A::CCLK_DIV_4,
            1 => PCLK_QEI_A::CCLK,
            2 => PCLK_QEI_A::CCLK_DIV_2,
            3 => PCLK_QEI_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_QEI_A::CCLK_DIV_4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_QEI_A::CCLK
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_QEI_A::CCLK_DIV_2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_QEI_A::CCLK_DIV_8
    }
}
#[doc = "Field `PCLK_QEI` writer - Peripheral clock selection for the Quadrature Encoder Interface."]
pub type PCLK_QEI_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, PCLK_QEI_A>;
impl<'a, REG, const O: u8> PCLK_QEI_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_QEI_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_QEI_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_QEI_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_QEI_A::CCLK_DIV_8)
    }
}
#[doc = "Field `PCLK_GPIOINT` reader - Peripheral clock selection for GPIO interrupts."]
pub type PCLK_GPIOINT_R = crate::FieldReader<PCLK_GPIOINT_A>;
#[doc = "Peripheral clock selection for GPIO interrupts.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCLK_GPIOINT_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_GPIOINT_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_GPIOINT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCLK_GPIOINT_A {
    type Ux = u8;
}
impl PCLK_GPIOINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCLK_GPIOINT_A {
        match self.bits {
            0 => PCLK_GPIOINT_A::CCLK_DIV_4,
            1 => PCLK_GPIOINT_A::CCLK,
            2 => PCLK_GPIOINT_A::CCLK_DIV_2,
            3 => PCLK_GPIOINT_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_GPIOINT_A::CCLK_DIV_4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_GPIOINT_A::CCLK
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_GPIOINT_A::CCLK_DIV_2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_GPIOINT_A::CCLK_DIV_8
    }
}
#[doc = "Field `PCLK_GPIOINT` writer - Peripheral clock selection for GPIO interrupts."]
pub type PCLK_GPIOINT_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 2, O, PCLK_GPIOINT_A>;
impl<'a, REG, const O: u8> PCLK_GPIOINT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_GPIOINT_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_GPIOINT_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_GPIOINT_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_GPIOINT_A::CCLK_DIV_8)
    }
}
#[doc = "Field `PCLK_PCB` reader - Peripheral clock selection for the Pin Connect block."]
pub type PCLK_PCB_R = crate::FieldReader<PCLK_PCB_A>;
#[doc = "Peripheral clock selection for the Pin Connect block.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCLK_PCB_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_PCB_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_PCB_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCLK_PCB_A {
    type Ux = u8;
}
impl PCLK_PCB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCLK_PCB_A {
        match self.bits {
            0 => PCLK_PCB_A::CCLK_DIV_4,
            1 => PCLK_PCB_A::CCLK,
            2 => PCLK_PCB_A::CCLK_DIV_2,
            3 => PCLK_PCB_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_PCB_A::CCLK_DIV_4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_PCB_A::CCLK
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_PCB_A::CCLK_DIV_2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_PCB_A::CCLK_DIV_8
    }
}
#[doc = "Field `PCLK_PCB` writer - Peripheral clock selection for the Pin Connect block."]
pub type PCLK_PCB_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, PCLK_PCB_A>;
impl<'a, REG, const O: u8> PCLK_PCB_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_PCB_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_PCB_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_PCB_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_PCB_A::CCLK_DIV_8)
    }
}
#[doc = "Field `PCLK_I2C1` reader - Peripheral clock selection for I2C1."]
pub type PCLK_I2C1_R = crate::FieldReader<PCLK_I2C1_A>;
#[doc = "Peripheral clock selection for I2C1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCLK_I2C1_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_I2C1_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_I2C1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCLK_I2C1_A {
    type Ux = u8;
}
impl PCLK_I2C1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCLK_I2C1_A {
        match self.bits {
            0 => PCLK_I2C1_A::CCLK_DIV_4,
            1 => PCLK_I2C1_A::CCLK,
            2 => PCLK_I2C1_A::CCLK_DIV_2,
            3 => PCLK_I2C1_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_I2C1_A::CCLK_DIV_4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_I2C1_A::CCLK
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_I2C1_A::CCLK_DIV_2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_I2C1_A::CCLK_DIV_8
    }
}
#[doc = "Field `PCLK_I2C1` writer - Peripheral clock selection for I2C1."]
pub type PCLK_I2C1_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, PCLK_I2C1_A>;
impl<'a, REG, const O: u8> PCLK_I2C1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_I2C1_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_I2C1_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_I2C1_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_I2C1_A::CCLK_DIV_8)
    }
}
#[doc = "Field `PCLK_SSP0` reader - Peripheral clock selection for SSP0."]
pub type PCLK_SSP0_R = crate::FieldReader<PCLK_SSP0_A>;
#[doc = "Peripheral clock selection for SSP0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCLK_SSP0_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_SSP0_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_SSP0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCLK_SSP0_A {
    type Ux = u8;
}
impl PCLK_SSP0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCLK_SSP0_A {
        match self.bits {
            0 => PCLK_SSP0_A::CCLK_DIV_4,
            1 => PCLK_SSP0_A::CCLK,
            2 => PCLK_SSP0_A::CCLK_DIV_2,
            3 => PCLK_SSP0_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_SSP0_A::CCLK_DIV_4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_SSP0_A::CCLK
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_SSP0_A::CCLK_DIV_2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_SSP0_A::CCLK_DIV_8
    }
}
#[doc = "Field `PCLK_SSP0` writer - Peripheral clock selection for SSP0."]
pub type PCLK_SSP0_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, PCLK_SSP0_A>;
impl<'a, REG, const O: u8> PCLK_SSP0_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_SSP0_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_SSP0_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_SSP0_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_SSP0_A::CCLK_DIV_8)
    }
}
#[doc = "Field `PCLK_TIMER2` reader - Peripheral clock selection for TIMER2."]
pub type PCLK_TIMER2_R = crate::FieldReader<PCLK_TIMER2_A>;
#[doc = "Peripheral clock selection for TIMER2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCLK_TIMER2_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_TIMER2_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_TIMER2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCLK_TIMER2_A {
    type Ux = u8;
}
impl PCLK_TIMER2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCLK_TIMER2_A {
        match self.bits {
            0 => PCLK_TIMER2_A::CCLK_DIV_4,
            1 => PCLK_TIMER2_A::CCLK,
            2 => PCLK_TIMER2_A::CCLK_DIV_2,
            3 => PCLK_TIMER2_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_TIMER2_A::CCLK_DIV_4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_TIMER2_A::CCLK
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_TIMER2_A::CCLK_DIV_2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_TIMER2_A::CCLK_DIV_8
    }
}
#[doc = "Field `PCLK_TIMER2` writer - Peripheral clock selection for TIMER2."]
pub type PCLK_TIMER2_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, PCLK_TIMER2_A>;
impl<'a, REG, const O: u8> PCLK_TIMER2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_TIMER2_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_TIMER2_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_TIMER2_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_TIMER2_A::CCLK_DIV_8)
    }
}
#[doc = "Field `PCLK_TIMER3` reader - Peripheral clock selection for TIMER3."]
pub type PCLK_TIMER3_R = crate::FieldReader<PCLK_TIMER3_A>;
#[doc = "Peripheral clock selection for TIMER3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCLK_TIMER3_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_TIMER3_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_TIMER3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCLK_TIMER3_A {
    type Ux = u8;
}
impl PCLK_TIMER3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCLK_TIMER3_A {
        match self.bits {
            0 => PCLK_TIMER3_A::CCLK_DIV_4,
            1 => PCLK_TIMER3_A::CCLK,
            2 => PCLK_TIMER3_A::CCLK_DIV_2,
            3 => PCLK_TIMER3_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_TIMER3_A::CCLK_DIV_4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_TIMER3_A::CCLK
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_TIMER3_A::CCLK_DIV_2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_TIMER3_A::CCLK_DIV_8
    }
}
#[doc = "Field `PCLK_TIMER3` writer - Peripheral clock selection for TIMER3."]
pub type PCLK_TIMER3_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, PCLK_TIMER3_A>;
impl<'a, REG, const O: u8> PCLK_TIMER3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_TIMER3_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_TIMER3_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_TIMER3_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_TIMER3_A::CCLK_DIV_8)
    }
}
#[doc = "Field `PCLK_UART2` reader - Peripheral clock selection for UART2."]
pub type PCLK_UART2_R = crate::FieldReader<PCLK_UART2_A>;
#[doc = "Peripheral clock selection for UART2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCLK_UART2_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_UART2_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_UART2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCLK_UART2_A {
    type Ux = u8;
}
impl PCLK_UART2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCLK_UART2_A {
        match self.bits {
            0 => PCLK_UART2_A::CCLK_DIV_4,
            1 => PCLK_UART2_A::CCLK,
            2 => PCLK_UART2_A::CCLK_DIV_2,
            3 => PCLK_UART2_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_UART2_A::CCLK_DIV_4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_UART2_A::CCLK
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_UART2_A::CCLK_DIV_2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_UART2_A::CCLK_DIV_8
    }
}
#[doc = "Field `PCLK_UART2` writer - Peripheral clock selection for UART2."]
pub type PCLK_UART2_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, PCLK_UART2_A>;
impl<'a, REG, const O: u8> PCLK_UART2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_UART2_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_UART2_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_UART2_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_UART2_A::CCLK_DIV_8)
    }
}
#[doc = "Field `PCLK_UART3` reader - Peripheral clock selection for UART3."]
pub type PCLK_UART3_R = crate::FieldReader<PCLK_UART3_A>;
#[doc = "Peripheral clock selection for UART3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCLK_UART3_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_UART3_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_UART3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCLK_UART3_A {
    type Ux = u8;
}
impl PCLK_UART3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCLK_UART3_A {
        match self.bits {
            0 => PCLK_UART3_A::CCLK_DIV_4,
            1 => PCLK_UART3_A::CCLK,
            2 => PCLK_UART3_A::CCLK_DIV_2,
            3 => PCLK_UART3_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_UART3_A::CCLK_DIV_4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_UART3_A::CCLK
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_UART3_A::CCLK_DIV_2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_UART3_A::CCLK_DIV_8
    }
}
#[doc = "Field `PCLK_UART3` writer - Peripheral clock selection for UART3."]
pub type PCLK_UART3_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, PCLK_UART3_A>;
impl<'a, REG, const O: u8> PCLK_UART3_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_UART3_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_UART3_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_UART3_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_UART3_A::CCLK_DIV_8)
    }
}
#[doc = "Field `PCLK_I2C2` reader - Peripheral clock selection for I2C2."]
pub type PCLK_I2C2_R = crate::FieldReader<PCLK_I2C2_A>;
#[doc = "Peripheral clock selection for I2C2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCLK_I2C2_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_I2C2_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_I2C2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCLK_I2C2_A {
    type Ux = u8;
}
impl PCLK_I2C2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCLK_I2C2_A {
        match self.bits {
            0 => PCLK_I2C2_A::CCLK_DIV_4,
            1 => PCLK_I2C2_A::CCLK,
            2 => PCLK_I2C2_A::CCLK_DIV_2,
            3 => PCLK_I2C2_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_I2C2_A::CCLK_DIV_4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_I2C2_A::CCLK
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_I2C2_A::CCLK_DIV_2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_I2C2_A::CCLK_DIV_8
    }
}
#[doc = "Field `PCLK_I2C2` writer - Peripheral clock selection for I2C2."]
pub type PCLK_I2C2_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, PCLK_I2C2_A>;
impl<'a, REG, const O: u8> PCLK_I2C2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_I2C2_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_I2C2_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_I2C2_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_I2C2_A::CCLK_DIV_8)
    }
}
#[doc = "Field `PCLK_I2S` reader - Peripheral clock selection for I2S."]
pub type PCLK_I2S_R = crate::FieldReader<PCLK_I2S_A>;
#[doc = "Peripheral clock selection for I2S.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCLK_I2S_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_I2S_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_I2S_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCLK_I2S_A {
    type Ux = u8;
}
impl PCLK_I2S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCLK_I2S_A {
        match self.bits {
            0 => PCLK_I2S_A::CCLK_DIV_4,
            1 => PCLK_I2S_A::CCLK,
            2 => PCLK_I2S_A::CCLK_DIV_2,
            3 => PCLK_I2S_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_I2S_A::CCLK_DIV_4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_I2S_A::CCLK
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_I2S_A::CCLK_DIV_2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_I2S_A::CCLK_DIV_8
    }
}
#[doc = "Field `PCLK_I2S` writer - Peripheral clock selection for I2S."]
pub type PCLK_I2S_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, PCLK_I2S_A>;
impl<'a, REG, const O: u8> PCLK_I2S_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_I2S_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_I2S_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_I2S_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_I2S_A::CCLK_DIV_8)
    }
}
#[doc = "Field `PCLK_RIT` reader - Peripheral clock selection for Repetitive Interrupt Timer."]
pub type PCLK_RIT_R = crate::FieldReader<PCLK_RIT_A>;
#[doc = "Peripheral clock selection for Repetitive Interrupt Timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCLK_RIT_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_RIT_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_RIT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCLK_RIT_A {
    type Ux = u8;
}
impl PCLK_RIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCLK_RIT_A {
        match self.bits {
            0 => PCLK_RIT_A::CCLK_DIV_4,
            1 => PCLK_RIT_A::CCLK,
            2 => PCLK_RIT_A::CCLK_DIV_2,
            3 => PCLK_RIT_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_RIT_A::CCLK_DIV_4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_RIT_A::CCLK
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_RIT_A::CCLK_DIV_2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_RIT_A::CCLK_DIV_8
    }
}
#[doc = "Field `PCLK_RIT` writer - Peripheral clock selection for Repetitive Interrupt Timer."]
pub type PCLK_RIT_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, PCLK_RIT_A>;
impl<'a, REG, const O: u8> PCLK_RIT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_RIT_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_RIT_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_RIT_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_RIT_A::CCLK_DIV_8)
    }
}
#[doc = "Field `PCLK_SYSCON` reader - Peripheral clock selection for the System Control block."]
pub type PCLK_SYSCON_R = crate::FieldReader<PCLK_SYSCON_A>;
#[doc = "Peripheral clock selection for the System Control block.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCLK_SYSCON_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_SYSCON_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_SYSCON_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCLK_SYSCON_A {
    type Ux = u8;
}
impl PCLK_SYSCON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCLK_SYSCON_A {
        match self.bits {
            0 => PCLK_SYSCON_A::CCLK_DIV_4,
            1 => PCLK_SYSCON_A::CCLK,
            2 => PCLK_SYSCON_A::CCLK_DIV_2,
            3 => PCLK_SYSCON_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_SYSCON_A::CCLK_DIV_4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_SYSCON_A::CCLK
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_SYSCON_A::CCLK_DIV_2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_SYSCON_A::CCLK_DIV_8
    }
}
#[doc = "Field `PCLK_SYSCON` writer - Peripheral clock selection for the System Control block."]
pub type PCLK_SYSCON_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, PCLK_SYSCON_A>;
impl<'a, REG, const O: u8> PCLK_SYSCON_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_SYSCON_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_SYSCON_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_SYSCON_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_SYSCON_A::CCLK_DIV_8)
    }
}
#[doc = "Field `PCLK_MC` reader - Peripheral clock selection for the Motor Control PWM."]
pub type PCLK_MC_R = crate::FieldReader<PCLK_MC_A>;
#[doc = "Peripheral clock selection for the Motor Control PWM.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCLK_MC_A {
    #[doc = "0: CCLK div 4. PCLK_peripheral = CCLK/4"]
    CCLK_DIV_4 = 0,
    #[doc = "1: CCLK. PCLK_peripheral = CCLK"]
    CCLK = 1,
    #[doc = "2: CCLK div 2. PCLK_peripheral = CCLK/2"]
    CCLK_DIV_2 = 2,
    #[doc = "3: CCLK div 8. PCLK_peripheral = CCLK/8"]
    CCLK_DIV_8 = 3,
}
impl From<PCLK_MC_A> for u8 {
    #[inline(always)]
    fn from(variant: PCLK_MC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCLK_MC_A {
    type Ux = u8;
}
impl PCLK_MC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCLK_MC_A {
        match self.bits {
            0 => PCLK_MC_A::CCLK_DIV_4,
            1 => PCLK_MC_A::CCLK,
            2 => PCLK_MC_A::CCLK_DIV_2,
            3 => PCLK_MC_A::CCLK_DIV_8,
            _ => unreachable!(),
        }
    }
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn is_cclk_div_4(&self) -> bool {
        *self == PCLK_MC_A::CCLK_DIV_4
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        *self == PCLK_MC_A::CCLK
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn is_cclk_div_2(&self) -> bool {
        *self == PCLK_MC_A::CCLK_DIV_2
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn is_cclk_div_8(&self) -> bool {
        *self == PCLK_MC_A::CCLK_DIV_8
    }
}
#[doc = "Field `PCLK_MC` writer - Peripheral clock selection for the Motor Control PWM."]
pub type PCLK_MC_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, PCLK_MC_A>;
impl<'a, REG, const O: u8> PCLK_MC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCLK div 4. PCLK_peripheral = CCLK/4"]
    #[inline(always)]
    pub fn cclk_div_4(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_MC_A::CCLK_DIV_4)
    }
    #[doc = "CCLK. PCLK_peripheral = CCLK"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_MC_A::CCLK)
    }
    #[doc = "CCLK div 2. PCLK_peripheral = CCLK/2"]
    #[inline(always)]
    pub fn cclk_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_MC_A::CCLK_DIV_2)
    }
    #[doc = "CCLK div 8. PCLK_peripheral = CCLK/8"]
    #[inline(always)]
    pub fn cclk_div_8(self) -> &'a mut crate::W<REG> {
        self.variant(PCLK_MC_A::CCLK_DIV_8)
    }
}
impl R {
    #[doc = "Bits 0:1 - Peripheral clock selection for the Quadrature Encoder Interface."]
    #[inline(always)]
    pub fn pclk_qei(&self) -> PCLK_QEI_R {
        PCLK_QEI_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Peripheral clock selection for GPIO interrupts."]
    #[inline(always)]
    pub fn pclk_gpioint(&self) -> PCLK_GPIOINT_R {
        PCLK_GPIOINT_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Peripheral clock selection for the Pin Connect block."]
    #[inline(always)]
    pub fn pclk_pcb(&self) -> PCLK_PCB_R {
        PCLK_PCB_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Peripheral clock selection for I2C1."]
    #[inline(always)]
    pub fn pclk_i2c1(&self) -> PCLK_I2C1_R {
        PCLK_I2C1_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Peripheral clock selection for SSP0."]
    #[inline(always)]
    pub fn pclk_ssp0(&self) -> PCLK_SSP0_R {
        PCLK_SSP0_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Peripheral clock selection for TIMER2."]
    #[inline(always)]
    pub fn pclk_timer2(&self) -> PCLK_TIMER2_R {
        PCLK_TIMER2_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Peripheral clock selection for TIMER3."]
    #[inline(always)]
    pub fn pclk_timer3(&self) -> PCLK_TIMER3_R {
        PCLK_TIMER3_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Peripheral clock selection for UART2."]
    #[inline(always)]
    pub fn pclk_uart2(&self) -> PCLK_UART2_R {
        PCLK_UART2_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Peripheral clock selection for UART3."]
    #[inline(always)]
    pub fn pclk_uart3(&self) -> PCLK_UART3_R {
        PCLK_UART3_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Peripheral clock selection for I2C2."]
    #[inline(always)]
    pub fn pclk_i2c2(&self) -> PCLK_I2C2_R {
        PCLK_I2C2_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Peripheral clock selection for I2S."]
    #[inline(always)]
    pub fn pclk_i2s(&self) -> PCLK_I2S_R {
        PCLK_I2S_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Peripheral clock selection for Repetitive Interrupt Timer."]
    #[inline(always)]
    pub fn pclk_rit(&self) -> PCLK_RIT_R {
        PCLK_RIT_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Peripheral clock selection for the System Control block."]
    #[inline(always)]
    pub fn pclk_syscon(&self) -> PCLK_SYSCON_R {
        PCLK_SYSCON_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Peripheral clock selection for the Motor Control PWM."]
    #[inline(always)]
    pub fn pclk_mc(&self) -> PCLK_MC_R {
        PCLK_MC_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCLKSEL1")
            .field("pclk_qei", &format_args!("{}", self.pclk_qei().bits()))
            .field(
                "pclk_gpioint",
                &format_args!("{}", self.pclk_gpioint().bits()),
            )
            .field("pclk_pcb", &format_args!("{}", self.pclk_pcb().bits()))
            .field("pclk_i2c1", &format_args!("{}", self.pclk_i2c1().bits()))
            .field("pclk_ssp0", &format_args!("{}", self.pclk_ssp0().bits()))
            .field(
                "pclk_timer2",
                &format_args!("{}", self.pclk_timer2().bits()),
            )
            .field(
                "pclk_timer3",
                &format_args!("{}", self.pclk_timer3().bits()),
            )
            .field("pclk_uart2", &format_args!("{}", self.pclk_uart2().bits()))
            .field("pclk_uart3", &format_args!("{}", self.pclk_uart3().bits()))
            .field("pclk_i2c2", &format_args!("{}", self.pclk_i2c2().bits()))
            .field("pclk_i2s", &format_args!("{}", self.pclk_i2s().bits()))
            .field("pclk_rit", &format_args!("{}", self.pclk_rit().bits()))
            .field(
                "pclk_syscon",
                &format_args!("{}", self.pclk_syscon().bits()),
            )
            .field("pclk_mc", &format_args!("{}", self.pclk_mc().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<PCLKSEL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Peripheral clock selection for the Quadrature Encoder Interface."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_qei(&mut self) -> PCLK_QEI_W<PCLKSEL1_SPEC, 0> {
        PCLK_QEI_W::new(self)
    }
    #[doc = "Bits 2:3 - Peripheral clock selection for GPIO interrupts."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_gpioint(&mut self) -> PCLK_GPIOINT_W<PCLKSEL1_SPEC, 2> {
        PCLK_GPIOINT_W::new(self)
    }
    #[doc = "Bits 4:5 - Peripheral clock selection for the Pin Connect block."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_pcb(&mut self) -> PCLK_PCB_W<PCLKSEL1_SPEC, 4> {
        PCLK_PCB_W::new(self)
    }
    #[doc = "Bits 6:7 - Peripheral clock selection for I2C1."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_i2c1(&mut self) -> PCLK_I2C1_W<PCLKSEL1_SPEC, 6> {
        PCLK_I2C1_W::new(self)
    }
    #[doc = "Bits 10:11 - Peripheral clock selection for SSP0."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_ssp0(&mut self) -> PCLK_SSP0_W<PCLKSEL1_SPEC, 10> {
        PCLK_SSP0_W::new(self)
    }
    #[doc = "Bits 12:13 - Peripheral clock selection for TIMER2."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_timer2(&mut self) -> PCLK_TIMER2_W<PCLKSEL1_SPEC, 12> {
        PCLK_TIMER2_W::new(self)
    }
    #[doc = "Bits 14:15 - Peripheral clock selection for TIMER3."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_timer3(&mut self) -> PCLK_TIMER3_W<PCLKSEL1_SPEC, 14> {
        PCLK_TIMER3_W::new(self)
    }
    #[doc = "Bits 16:17 - Peripheral clock selection for UART2."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_uart2(&mut self) -> PCLK_UART2_W<PCLKSEL1_SPEC, 16> {
        PCLK_UART2_W::new(self)
    }
    #[doc = "Bits 18:19 - Peripheral clock selection for UART3."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_uart3(&mut self) -> PCLK_UART3_W<PCLKSEL1_SPEC, 18> {
        PCLK_UART3_W::new(self)
    }
    #[doc = "Bits 20:21 - Peripheral clock selection for I2C2."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_i2c2(&mut self) -> PCLK_I2C2_W<PCLKSEL1_SPEC, 20> {
        PCLK_I2C2_W::new(self)
    }
    #[doc = "Bits 22:23 - Peripheral clock selection for I2S."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_i2s(&mut self) -> PCLK_I2S_W<PCLKSEL1_SPEC, 22> {
        PCLK_I2S_W::new(self)
    }
    #[doc = "Bits 26:27 - Peripheral clock selection for Repetitive Interrupt Timer."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_rit(&mut self) -> PCLK_RIT_W<PCLKSEL1_SPEC, 26> {
        PCLK_RIT_W::new(self)
    }
    #[doc = "Bits 28:29 - Peripheral clock selection for the System Control block."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_syscon(&mut self) -> PCLK_SYSCON_W<PCLKSEL1_SPEC, 28> {
        PCLK_SYSCON_W::new(self)
    }
    #[doc = "Bits 30:31 - Peripheral clock selection for the Motor Control PWM."]
    #[inline(always)]
    #[must_use]
    pub fn pclk_mc(&mut self) -> PCLK_MC_W<PCLKSEL1_SPEC, 30> {
        PCLK_MC_W::new(self)
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
#[doc = "Peripheral Clock Selection register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pclksel1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pclksel1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCLKSEL1_SPEC;
impl crate::RegisterSpec for PCLKSEL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pclksel1::R`](R) reader structure"]
impl crate::Readable for PCLKSEL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pclksel1::W`](W) writer structure"]
impl crate::Writable for PCLKSEL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCLKSEL1 to value 0"]
impl crate::Resettable for PCLKSEL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
