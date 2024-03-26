#[doc = "Register `PINSEL3` reader"]
pub type R = crate::R<PINSEL3_SPEC>;
#[doc = "Register `PINSEL3` writer"]
pub type W = crate::W<PINSEL3_SPEC>;
#[doc = "Field `P1_16` reader - Pin function select P1.16."]
pub type P1_16_R = crate::FieldReader<P1_16_A>;
#[doc = "Pin function select P1.16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_16_A {
    #[doc = "0: GPIO P1.16"]
    GPIO_P1 = 0,
    #[doc = "1: ENET_MDC"]
    ENET_MDC = 1,
}
impl From<P1_16_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_16_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_16_A {
    type Ux = u8;
}
impl P1_16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_16_A {
        match self.bits {
            0 => P1_16_A::GPIO_P1,
            1 => P1_16_A::ENET_MDC,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.16"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_16_A::GPIO_P1
    }
    #[doc = "ENET_MDC"]
    #[inline(always)]
    pub fn is_enet_mdc(&self) -> bool {
        *self == P1_16_A::ENET_MDC
    }
}
#[doc = "Field `P1_16` writer - Pin function select P1.16."]
pub type P1_16_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, P1_16_A>;
impl<'a, REG, const O: u8> P1_16_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.16"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_16_A::GPIO_P1)
    }
    #[doc = "ENET_MDC"]
    #[inline(always)]
    pub fn enet_mdc(self) -> &'a mut crate::W<REG> {
        self.variant(P1_16_A::ENET_MDC)
    }
}
#[doc = "Field `P1_17` reader - Pin function select P1.17."]
pub type P1_17_R = crate::FieldReader<P1_17_A>;
#[doc = "Pin function select P1.17.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_17_A {
    #[doc = "0: GPIO P1.17"]
    GPIO_P1 = 0,
    #[doc = "1: ENET_MDIO"]
    ENET_MDIO = 1,
}
impl From<P1_17_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_17_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_17_A {
    type Ux = u8;
}
impl P1_17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_17_A {
        match self.bits {
            0 => P1_17_A::GPIO_P1,
            1 => P1_17_A::ENET_MDIO,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.17"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_17_A::GPIO_P1
    }
    #[doc = "ENET_MDIO"]
    #[inline(always)]
    pub fn is_enet_mdio(&self) -> bool {
        *self == P1_17_A::ENET_MDIO
    }
}
#[doc = "Field `P1_17` writer - Pin function select P1.17."]
pub type P1_17_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, P1_17_A>;
impl<'a, REG, const O: u8> P1_17_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.17"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_17_A::GPIO_P1)
    }
    #[doc = "ENET_MDIO"]
    #[inline(always)]
    pub fn enet_mdio(self) -> &'a mut crate::W<REG> {
        self.variant(P1_17_A::ENET_MDIO)
    }
}
#[doc = "Field `P1_18` reader - Pin function select P1.18."]
pub type P1_18_R = crate::FieldReader<P1_18_A>;
#[doc = "Pin function select P1.18.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_18_A {
    #[doc = "0: GPIO P1.18"]
    GPIO_P1 = 0,
    #[doc = "1: USB_UP_LED"]
    USB_UP_LED = 1,
    #[doc = "2: PWM1.1"]
    PWM1 = 2,
    #[doc = "3: CAP1.0"]
    CAP1 = 3,
}
impl From<P1_18_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_18_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_18_A {
    type Ux = u8;
}
impl P1_18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_18_A {
        match self.bits {
            0 => P1_18_A::GPIO_P1,
            1 => P1_18_A::USB_UP_LED,
            2 => P1_18_A::PWM1,
            3 => P1_18_A::CAP1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.18"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_18_A::GPIO_P1
    }
    #[doc = "USB_UP_LED"]
    #[inline(always)]
    pub fn is_usb_up_led(&self) -> bool {
        *self == P1_18_A::USB_UP_LED
    }
    #[doc = "PWM1.1"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P1_18_A::PWM1
    }
    #[doc = "CAP1.0"]
    #[inline(always)]
    pub fn is_cap1(&self) -> bool {
        *self == P1_18_A::CAP1
    }
}
#[doc = "Field `P1_18` writer - Pin function select P1.18."]
pub type P1_18_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, P1_18_A>;
impl<'a, REG, const O: u8> P1_18_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.18"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_18_A::GPIO_P1)
    }
    #[doc = "USB_UP_LED"]
    #[inline(always)]
    pub fn usb_up_led(self) -> &'a mut crate::W<REG> {
        self.variant(P1_18_A::USB_UP_LED)
    }
    #[doc = "PWM1.1"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_18_A::PWM1)
    }
    #[doc = "CAP1.0"]
    #[inline(always)]
    pub fn cap1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_18_A::CAP1)
    }
}
#[doc = "Field `P1_19` reader - Pin function select P1.19."]
pub type P1_19_R = crate::FieldReader<P1_19_A>;
#[doc = "Pin function select P1.19.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_19_A {
    #[doc = "0: GPIO P1.19."]
    GPIO_P1 = 0,
    #[doc = "1: MCOA0"]
    MCOA0 = 1,
    #[doc = "2: USB_PPWR"]
    USB_PPWR = 2,
    #[doc = "3: CAP1.1"]
    CAP1 = 3,
}
impl From<P1_19_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_19_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_19_A {
    type Ux = u8;
}
impl P1_19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_19_A {
        match self.bits {
            0 => P1_19_A::GPIO_P1,
            1 => P1_19_A::MCOA0,
            2 => P1_19_A::USB_PPWR,
            3 => P1_19_A::CAP1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.19."]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_19_A::GPIO_P1
    }
    #[doc = "MCOA0"]
    #[inline(always)]
    pub fn is_mcoa0(&self) -> bool {
        *self == P1_19_A::MCOA0
    }
    #[doc = "USB_PPWR"]
    #[inline(always)]
    pub fn is_usb_ppwr(&self) -> bool {
        *self == P1_19_A::USB_PPWR
    }
    #[doc = "CAP1.1"]
    #[inline(always)]
    pub fn is_cap1(&self) -> bool {
        *self == P1_19_A::CAP1
    }
}
#[doc = "Field `P1_19` writer - Pin function select P1.19."]
pub type P1_19_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, P1_19_A>;
impl<'a, REG, const O: u8> P1_19_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.19."]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_19_A::GPIO_P1)
    }
    #[doc = "MCOA0"]
    #[inline(always)]
    pub fn mcoa0(self) -> &'a mut crate::W<REG> {
        self.variant(P1_19_A::MCOA0)
    }
    #[doc = "USB_PPWR"]
    #[inline(always)]
    pub fn usb_ppwr(self) -> &'a mut crate::W<REG> {
        self.variant(P1_19_A::USB_PPWR)
    }
    #[doc = "CAP1.1"]
    #[inline(always)]
    pub fn cap1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_19_A::CAP1)
    }
}
#[doc = "Field `P1_20` reader - Pin function select P1.20."]
pub type P1_20_R = crate::FieldReader<P1_20_A>;
#[doc = "Pin function select P1.20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_20_A {
    #[doc = "0: GPIO P1.20."]
    GPIO_P1 = 0,
    #[doc = "1: MCI0"]
    MCI0 = 1,
    #[doc = "2: PWM1.2"]
    PWM1 = 2,
    #[doc = "3: SCK0"]
    SCK0 = 3,
}
impl From<P1_20_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_20_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_20_A {
    type Ux = u8;
}
impl P1_20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_20_A {
        match self.bits {
            0 => P1_20_A::GPIO_P1,
            1 => P1_20_A::MCI0,
            2 => P1_20_A::PWM1,
            3 => P1_20_A::SCK0,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.20."]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_20_A::GPIO_P1
    }
    #[doc = "MCI0"]
    #[inline(always)]
    pub fn is_mci0(&self) -> bool {
        *self == P1_20_A::MCI0
    }
    #[doc = "PWM1.2"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P1_20_A::PWM1
    }
    #[doc = "SCK0"]
    #[inline(always)]
    pub fn is_sck0(&self) -> bool {
        *self == P1_20_A::SCK0
    }
}
#[doc = "Field `P1_20` writer - Pin function select P1.20."]
pub type P1_20_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, P1_20_A>;
impl<'a, REG, const O: u8> P1_20_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.20."]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_20_A::GPIO_P1)
    }
    #[doc = "MCI0"]
    #[inline(always)]
    pub fn mci0(self) -> &'a mut crate::W<REG> {
        self.variant(P1_20_A::MCI0)
    }
    #[doc = "PWM1.2"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_20_A::PWM1)
    }
    #[doc = "SCK0"]
    #[inline(always)]
    pub fn sck0(self) -> &'a mut crate::W<REG> {
        self.variant(P1_20_A::SCK0)
    }
}
#[doc = "Field `P1_21` reader - Pin function select P1.21."]
pub type P1_21_R = crate::FieldReader<P1_21_A>;
#[doc = "Pin function select P1.21.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_21_A {
    #[doc = "0: GPIO P1.21."]
    GPIO_P1 = 0,
    #[doc = "1: MCABORT"]
    MCABORT = 1,
    #[doc = "2: PWM1.3"]
    PWM1 = 2,
    #[doc = "3: SSEL0"]
    SSEL0 = 3,
}
impl From<P1_21_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_21_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_21_A {
    type Ux = u8;
}
impl P1_21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_21_A {
        match self.bits {
            0 => P1_21_A::GPIO_P1,
            1 => P1_21_A::MCABORT,
            2 => P1_21_A::PWM1,
            3 => P1_21_A::SSEL0,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.21."]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_21_A::GPIO_P1
    }
    #[doc = "MCABORT"]
    #[inline(always)]
    pub fn is_mcabort(&self) -> bool {
        *self == P1_21_A::MCABORT
    }
    #[doc = "PWM1.3"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P1_21_A::PWM1
    }
    #[doc = "SSEL0"]
    #[inline(always)]
    pub fn is_ssel0(&self) -> bool {
        *self == P1_21_A::SSEL0
    }
}
#[doc = "Field `P1_21` writer - Pin function select P1.21."]
pub type P1_21_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, P1_21_A>;
impl<'a, REG, const O: u8> P1_21_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.21."]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_21_A::GPIO_P1)
    }
    #[doc = "MCABORT"]
    #[inline(always)]
    pub fn mcabort(self) -> &'a mut crate::W<REG> {
        self.variant(P1_21_A::MCABORT)
    }
    #[doc = "PWM1.3"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_21_A::PWM1)
    }
    #[doc = "SSEL0"]
    #[inline(always)]
    pub fn ssel0(self) -> &'a mut crate::W<REG> {
        self.variant(P1_21_A::SSEL0)
    }
}
#[doc = "Field `P1_22` reader - Pin function select P1.22"]
pub type P1_22_R = crate::FieldReader<P1_22_A>;
#[doc = "Pin function select P1.22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_22_A {
    #[doc = "0: GPIO P1.22."]
    GPIO_P1 = 0,
    #[doc = "1: MCOB0"]
    MCOB0 = 1,
    #[doc = "2: USB_PWRD"]
    USB_PWRD = 2,
    #[doc = "3: MAT1.0"]
    MAT1 = 3,
}
impl From<P1_22_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_22_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_22_A {
    type Ux = u8;
}
impl P1_22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_22_A {
        match self.bits {
            0 => P1_22_A::GPIO_P1,
            1 => P1_22_A::MCOB0,
            2 => P1_22_A::USB_PWRD,
            3 => P1_22_A::MAT1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.22."]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_22_A::GPIO_P1
    }
    #[doc = "MCOB0"]
    #[inline(always)]
    pub fn is_mcob0(&self) -> bool {
        *self == P1_22_A::MCOB0
    }
    #[doc = "USB_PWRD"]
    #[inline(always)]
    pub fn is_usb_pwrd(&self) -> bool {
        *self == P1_22_A::USB_PWRD
    }
    #[doc = "MAT1.0"]
    #[inline(always)]
    pub fn is_mat1(&self) -> bool {
        *self == P1_22_A::MAT1
    }
}
#[doc = "Field `P1_22` writer - Pin function select P1.22"]
pub type P1_22_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, P1_22_A>;
impl<'a, REG, const O: u8> P1_22_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.22."]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_22_A::GPIO_P1)
    }
    #[doc = "MCOB0"]
    #[inline(always)]
    pub fn mcob0(self) -> &'a mut crate::W<REG> {
        self.variant(P1_22_A::MCOB0)
    }
    #[doc = "USB_PWRD"]
    #[inline(always)]
    pub fn usb_pwrd(self) -> &'a mut crate::W<REG> {
        self.variant(P1_22_A::USB_PWRD)
    }
    #[doc = "MAT1.0"]
    #[inline(always)]
    pub fn mat1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_22_A::MAT1)
    }
}
#[doc = "Field `P1_23` reader - Pin function select P1.23."]
pub type P1_23_R = crate::FieldReader<P1_23_A>;
#[doc = "Pin function select P1.23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_23_A {
    #[doc = "0: GPIO P1.23."]
    GPIO_P1 = 0,
    #[doc = "1: MCI1"]
    MCI1 = 1,
    #[doc = "2: PWM1.4"]
    PWM1 = 2,
    #[doc = "3: MISO0"]
    MISO0 = 3,
}
impl From<P1_23_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_23_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_23_A {
    type Ux = u8;
}
impl P1_23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_23_A {
        match self.bits {
            0 => P1_23_A::GPIO_P1,
            1 => P1_23_A::MCI1,
            2 => P1_23_A::PWM1,
            3 => P1_23_A::MISO0,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.23."]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_23_A::GPIO_P1
    }
    #[doc = "MCI1"]
    #[inline(always)]
    pub fn is_mci1(&self) -> bool {
        *self == P1_23_A::MCI1
    }
    #[doc = "PWM1.4"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P1_23_A::PWM1
    }
    #[doc = "MISO0"]
    #[inline(always)]
    pub fn is_miso0(&self) -> bool {
        *self == P1_23_A::MISO0
    }
}
#[doc = "Field `P1_23` writer - Pin function select P1.23."]
pub type P1_23_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, P1_23_A>;
impl<'a, REG, const O: u8> P1_23_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.23."]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_23_A::GPIO_P1)
    }
    #[doc = "MCI1"]
    #[inline(always)]
    pub fn mci1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_23_A::MCI1)
    }
    #[doc = "PWM1.4"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_23_A::PWM1)
    }
    #[doc = "MISO0"]
    #[inline(always)]
    pub fn miso0(self) -> &'a mut crate::W<REG> {
        self.variant(P1_23_A::MISO0)
    }
}
#[doc = "Field `P1_24` reader - Pin function select P1.24."]
pub type P1_24_R = crate::FieldReader<P1_24_A>;
#[doc = "Pin function select P1.24.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_24_A {
    #[doc = "0: GPIO P1.24."]
    GPIO_P1 = 0,
    #[doc = "1: MCI2"]
    MCI2 = 1,
    #[doc = "2: PWM1.5"]
    PWM1 = 2,
    #[doc = "3: MOSI0"]
    MOSI0 = 3,
}
impl From<P1_24_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_24_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_24_A {
    type Ux = u8;
}
impl P1_24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_24_A {
        match self.bits {
            0 => P1_24_A::GPIO_P1,
            1 => P1_24_A::MCI2,
            2 => P1_24_A::PWM1,
            3 => P1_24_A::MOSI0,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.24."]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_24_A::GPIO_P1
    }
    #[doc = "MCI2"]
    #[inline(always)]
    pub fn is_mci2(&self) -> bool {
        *self == P1_24_A::MCI2
    }
    #[doc = "PWM1.5"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P1_24_A::PWM1
    }
    #[doc = "MOSI0"]
    #[inline(always)]
    pub fn is_mosi0(&self) -> bool {
        *self == P1_24_A::MOSI0
    }
}
#[doc = "Field `P1_24` writer - Pin function select P1.24."]
pub type P1_24_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, P1_24_A>;
impl<'a, REG, const O: u8> P1_24_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.24."]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_24_A::GPIO_P1)
    }
    #[doc = "MCI2"]
    #[inline(always)]
    pub fn mci2(self) -> &'a mut crate::W<REG> {
        self.variant(P1_24_A::MCI2)
    }
    #[doc = "PWM1.5"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_24_A::PWM1)
    }
    #[doc = "MOSI0"]
    #[inline(always)]
    pub fn mosi0(self) -> &'a mut crate::W<REG> {
        self.variant(P1_24_A::MOSI0)
    }
}
#[doc = "Field `P1_25` reader - Pin function select P1.25."]
pub type P1_25_R = crate::FieldReader<P1_25_A>;
#[doc = "Pin function select P1.25.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_25_A {
    #[doc = "0: GPIO P1.25"]
    GPIO_P1 = 0,
    #[doc = "1: MCOA1"]
    MCOA1 = 1,
    #[doc = "3: MAT1.1"]
    MAT1 = 3,
}
impl From<P1_25_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_25_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_25_A {
    type Ux = u8;
}
impl P1_25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_25_A {
        match self.bits {
            0 => P1_25_A::GPIO_P1,
            1 => P1_25_A::MCOA1,
            3 => P1_25_A::MAT1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.25"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_25_A::GPIO_P1
    }
    #[doc = "MCOA1"]
    #[inline(always)]
    pub fn is_mcoa1(&self) -> bool {
        *self == P1_25_A::MCOA1
    }
    #[doc = "MAT1.1"]
    #[inline(always)]
    pub fn is_mat1(&self) -> bool {
        *self == P1_25_A::MAT1
    }
}
#[doc = "Field `P1_25` writer - Pin function select P1.25."]
pub type P1_25_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, P1_25_A>;
impl<'a, REG, const O: u8> P1_25_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.25"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_25_A::GPIO_P1)
    }
    #[doc = "MCOA1"]
    #[inline(always)]
    pub fn mcoa1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_25_A::MCOA1)
    }
    #[doc = "MAT1.1"]
    #[inline(always)]
    pub fn mat1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_25_A::MAT1)
    }
}
#[doc = "Field `P1_26` reader - Pin function select P1.26."]
pub type P1_26_R = crate::FieldReader<P1_26_A>;
#[doc = "Pin function select P1.26.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_26_A {
    #[doc = "0: GPIO P1.26"]
    GPIO_P1 = 0,
    #[doc = "1: MCOB1"]
    MCOB1 = 1,
    #[doc = "2: PWM1.6"]
    PWM1 = 2,
    #[doc = "3: CAP0.0"]
    CAP0 = 3,
}
impl From<P1_26_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_26_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_26_A {
    type Ux = u8;
}
impl P1_26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_26_A {
        match self.bits {
            0 => P1_26_A::GPIO_P1,
            1 => P1_26_A::MCOB1,
            2 => P1_26_A::PWM1,
            3 => P1_26_A::CAP0,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.26"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_26_A::GPIO_P1
    }
    #[doc = "MCOB1"]
    #[inline(always)]
    pub fn is_mcob1(&self) -> bool {
        *self == P1_26_A::MCOB1
    }
    #[doc = "PWM1.6"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P1_26_A::PWM1
    }
    #[doc = "CAP0.0"]
    #[inline(always)]
    pub fn is_cap0(&self) -> bool {
        *self == P1_26_A::CAP0
    }
}
#[doc = "Field `P1_26` writer - Pin function select P1.26."]
pub type P1_26_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, P1_26_A>;
impl<'a, REG, const O: u8> P1_26_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.26"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_26_A::GPIO_P1)
    }
    #[doc = "MCOB1"]
    #[inline(always)]
    pub fn mcob1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_26_A::MCOB1)
    }
    #[doc = "PWM1.6"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_26_A::PWM1)
    }
    #[doc = "CAP0.0"]
    #[inline(always)]
    pub fn cap0(self) -> &'a mut crate::W<REG> {
        self.variant(P1_26_A::CAP0)
    }
}
#[doc = "Field `P1_27` reader - Pin function select P1.27."]
pub type P1_27_R = crate::FieldReader<P1_27_A>;
#[doc = "Pin function select P1.27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_27_A {
    #[doc = "0: GPIO P1.27"]
    GPIO_P1 = 0,
    #[doc = "1: CLKOUT"]
    CLKOUT = 1,
    #[doc = "2: USB_OVRCR"]
    USB_OVRCR = 2,
    #[doc = "3: CAP0.1"]
    CAP0 = 3,
}
impl From<P1_27_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_27_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_27_A {
    type Ux = u8;
}
impl P1_27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_27_A {
        match self.bits {
            0 => P1_27_A::GPIO_P1,
            1 => P1_27_A::CLKOUT,
            2 => P1_27_A::USB_OVRCR,
            3 => P1_27_A::CAP0,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.27"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_27_A::GPIO_P1
    }
    #[doc = "CLKOUT"]
    #[inline(always)]
    pub fn is_clkout(&self) -> bool {
        *self == P1_27_A::CLKOUT
    }
    #[doc = "USB_OVRCR"]
    #[inline(always)]
    pub fn is_usb_ovrcr(&self) -> bool {
        *self == P1_27_A::USB_OVRCR
    }
    #[doc = "CAP0.1"]
    #[inline(always)]
    pub fn is_cap0(&self) -> bool {
        *self == P1_27_A::CAP0
    }
}
#[doc = "Field `P1_27` writer - Pin function select P1.27."]
pub type P1_27_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, P1_27_A>;
impl<'a, REG, const O: u8> P1_27_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.27"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_27_A::GPIO_P1)
    }
    #[doc = "CLKOUT"]
    #[inline(always)]
    pub fn clkout(self) -> &'a mut crate::W<REG> {
        self.variant(P1_27_A::CLKOUT)
    }
    #[doc = "USB_OVRCR"]
    #[inline(always)]
    pub fn usb_ovrcr(self) -> &'a mut crate::W<REG> {
        self.variant(P1_27_A::USB_OVRCR)
    }
    #[doc = "CAP0.1"]
    #[inline(always)]
    pub fn cap0(self) -> &'a mut crate::W<REG> {
        self.variant(P1_27_A::CAP0)
    }
}
#[doc = "Field `P1_28` reader - Pin function select P1.28."]
pub type P1_28_R = crate::FieldReader<P1_28_A>;
#[doc = "Pin function select P1.28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_28_A {
    #[doc = "0: GPIO P1.28"]
    GPIO_P1 = 0,
    #[doc = "1: MCOA2"]
    MCOA2 = 1,
    #[doc = "2: PCAP1.0"]
    PCAP1 = 2,
    #[doc = "3: MAT0.0"]
    MAT0 = 3,
}
impl From<P1_28_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_28_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_28_A {
    type Ux = u8;
}
impl P1_28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_28_A {
        match self.bits {
            0 => P1_28_A::GPIO_P1,
            1 => P1_28_A::MCOA2,
            2 => P1_28_A::PCAP1,
            3 => P1_28_A::MAT0,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.28"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_28_A::GPIO_P1
    }
    #[doc = "MCOA2"]
    #[inline(always)]
    pub fn is_mcoa2(&self) -> bool {
        *self == P1_28_A::MCOA2
    }
    #[doc = "PCAP1.0"]
    #[inline(always)]
    pub fn is_pcap1(&self) -> bool {
        *self == P1_28_A::PCAP1
    }
    #[doc = "MAT0.0"]
    #[inline(always)]
    pub fn is_mat0(&self) -> bool {
        *self == P1_28_A::MAT0
    }
}
#[doc = "Field `P1_28` writer - Pin function select P1.28."]
pub type P1_28_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, P1_28_A>;
impl<'a, REG, const O: u8> P1_28_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.28"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_28_A::GPIO_P1)
    }
    #[doc = "MCOA2"]
    #[inline(always)]
    pub fn mcoa2(self) -> &'a mut crate::W<REG> {
        self.variant(P1_28_A::MCOA2)
    }
    #[doc = "PCAP1.0"]
    #[inline(always)]
    pub fn pcap1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_28_A::PCAP1)
    }
    #[doc = "MAT0.0"]
    #[inline(always)]
    pub fn mat0(self) -> &'a mut crate::W<REG> {
        self.variant(P1_28_A::MAT0)
    }
}
#[doc = "Field `P1_29` reader - Pin function select P1.29"]
pub type P1_29_R = crate::FieldReader<P1_29_A>;
#[doc = "Pin function select P1.29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_29_A {
    #[doc = "0: GPIO P1.29"]
    GPIO_P1 = 0,
    #[doc = "1: MCOB2"]
    MCOB2 = 1,
    #[doc = "2: PCAP1.1"]
    PCAP1 = 2,
    #[doc = "3: MAT0.1"]
    MAT0 = 3,
}
impl From<P1_29_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_29_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_29_A {
    type Ux = u8;
}
impl P1_29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_29_A {
        match self.bits {
            0 => P1_29_A::GPIO_P1,
            1 => P1_29_A::MCOB2,
            2 => P1_29_A::PCAP1,
            3 => P1_29_A::MAT0,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.29"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_29_A::GPIO_P1
    }
    #[doc = "MCOB2"]
    #[inline(always)]
    pub fn is_mcob2(&self) -> bool {
        *self == P1_29_A::MCOB2
    }
    #[doc = "PCAP1.1"]
    #[inline(always)]
    pub fn is_pcap1(&self) -> bool {
        *self == P1_29_A::PCAP1
    }
    #[doc = "MAT0.1"]
    #[inline(always)]
    pub fn is_mat0(&self) -> bool {
        *self == P1_29_A::MAT0
    }
}
#[doc = "Field `P1_29` writer - Pin function select P1.29"]
pub type P1_29_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, P1_29_A>;
impl<'a, REG, const O: u8> P1_29_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.29"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_29_A::GPIO_P1)
    }
    #[doc = "MCOB2"]
    #[inline(always)]
    pub fn mcob2(self) -> &'a mut crate::W<REG> {
        self.variant(P1_29_A::MCOB2)
    }
    #[doc = "PCAP1.1"]
    #[inline(always)]
    pub fn pcap1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_29_A::PCAP1)
    }
    #[doc = "MAT0.1"]
    #[inline(always)]
    pub fn mat0(self) -> &'a mut crate::W<REG> {
        self.variant(P1_29_A::MAT0)
    }
}
#[doc = "Field `P1_30` reader - Pin function select P1.30."]
pub type P1_30_R = crate::FieldReader<P1_30_A>;
#[doc = "Pin function select P1.30.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_30_A {
    #[doc = "0: GPIO P1.30"]
    GPIO_P1 = 0,
    #[doc = "2: VBUS"]
    VBUS = 2,
    #[doc = "3: AD0.4"]
    AD0 = 3,
}
impl From<P1_30_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_30_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_30_A {
    type Ux = u8;
}
impl P1_30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_30_A {
        match self.bits {
            0 => P1_30_A::GPIO_P1,
            2 => P1_30_A::VBUS,
            3 => P1_30_A::AD0,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.30"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_30_A::GPIO_P1
    }
    #[doc = "VBUS"]
    #[inline(always)]
    pub fn is_vbus(&self) -> bool {
        *self == P1_30_A::VBUS
    }
    #[doc = "AD0.4"]
    #[inline(always)]
    pub fn is_ad0(&self) -> bool {
        *self == P1_30_A::AD0
    }
}
#[doc = "Field `P1_30` writer - Pin function select P1.30."]
pub type P1_30_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, P1_30_A>;
impl<'a, REG, const O: u8> P1_30_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.30"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_30_A::GPIO_P1)
    }
    #[doc = "VBUS"]
    #[inline(always)]
    pub fn vbus(self) -> &'a mut crate::W<REG> {
        self.variant(P1_30_A::VBUS)
    }
    #[doc = "AD0.4"]
    #[inline(always)]
    pub fn ad0(self) -> &'a mut crate::W<REG> {
        self.variant(P1_30_A::AD0)
    }
}
#[doc = "Field `P1_31` reader - Pin function select P1.31."]
pub type P1_31_R = crate::FieldReader<P1_31_A>;
#[doc = "Pin function select P1.31.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_31_A {
    #[doc = "0: GPIO Port 1.31"]
    GPIO_P1 = 0,
    #[doc = "2: SCK1"]
    SCK1 = 2,
    #[doc = "3: AD0.5"]
    AD0 = 3,
}
impl From<P1_31_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_31_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_31_A {
    type Ux = u8;
}
impl P1_31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_31_A {
        match self.bits {
            0 => P1_31_A::GPIO_P1,
            2 => P1_31_A::SCK1,
            3 => P1_31_A::AD0,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO Port 1.31"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_31_A::GPIO_P1
    }
    #[doc = "SCK1"]
    #[inline(always)]
    pub fn is_sck1(&self) -> bool {
        *self == P1_31_A::SCK1
    }
    #[doc = "AD0.5"]
    #[inline(always)]
    pub fn is_ad0(&self) -> bool {
        *self == P1_31_A::AD0
    }
}
#[doc = "Field `P1_31` writer - Pin function select P1.31."]
pub type P1_31_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, P1_31_A>;
impl<'a, REG, const O: u8> P1_31_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO Port 1.31"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_31_A::GPIO_P1)
    }
    #[doc = "SCK1"]
    #[inline(always)]
    pub fn sck1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_31_A::SCK1)
    }
    #[doc = "AD0.5"]
    #[inline(always)]
    pub fn ad0(self) -> &'a mut crate::W<REG> {
        self.variant(P1_31_A::AD0)
    }
}
impl R {
    #[doc = "Bits 0:1 - Pin function select P1.16."]
    #[inline(always)]
    pub fn p1_16(&self) -> P1_16_R {
        P1_16_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Pin function select P1.17."]
    #[inline(always)]
    pub fn p1_17(&self) -> P1_17_R {
        P1_17_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Pin function select P1.18."]
    #[inline(always)]
    pub fn p1_18(&self) -> P1_18_R {
        P1_18_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Pin function select P1.19."]
    #[inline(always)]
    pub fn p1_19(&self) -> P1_19_R {
        P1_19_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin function select P1.20."]
    #[inline(always)]
    pub fn p1_20(&self) -> P1_20_R {
        P1_20_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Pin function select P1.21."]
    #[inline(always)]
    pub fn p1_21(&self) -> P1_21_R {
        P1_21_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Pin function select P1.22"]
    #[inline(always)]
    pub fn p1_22(&self) -> P1_22_R {
        P1_22_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Pin function select P1.23."]
    #[inline(always)]
    pub fn p1_23(&self) -> P1_23_R {
        P1_23_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Pin function select P1.24."]
    #[inline(always)]
    pub fn p1_24(&self) -> P1_24_R {
        P1_24_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Pin function select P1.25."]
    #[inline(always)]
    pub fn p1_25(&self) -> P1_25_R {
        P1_25_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Pin function select P1.26."]
    #[inline(always)]
    pub fn p1_26(&self) -> P1_26_R {
        P1_26_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Pin function select P1.27."]
    #[inline(always)]
    pub fn p1_27(&self) -> P1_27_R {
        P1_27_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Pin function select P1.28."]
    #[inline(always)]
    pub fn p1_28(&self) -> P1_28_R {
        P1_28_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Pin function select P1.29"]
    #[inline(always)]
    pub fn p1_29(&self) -> P1_29_R {
        P1_29_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Pin function select P1.30."]
    #[inline(always)]
    pub fn p1_30(&self) -> P1_30_R {
        P1_30_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Pin function select P1.31."]
    #[inline(always)]
    pub fn p1_31(&self) -> P1_31_R {
        P1_31_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PINSEL3")
            .field("p1_16", &format_args!("{}", self.p1_16().bits()))
            .field("p1_17", &format_args!("{}", self.p1_17().bits()))
            .field("p1_18", &format_args!("{}", self.p1_18().bits()))
            .field("p1_19", &format_args!("{}", self.p1_19().bits()))
            .field("p1_20", &format_args!("{}", self.p1_20().bits()))
            .field("p1_21", &format_args!("{}", self.p1_21().bits()))
            .field("p1_22", &format_args!("{}", self.p1_22().bits()))
            .field("p1_23", &format_args!("{}", self.p1_23().bits()))
            .field("p1_24", &format_args!("{}", self.p1_24().bits()))
            .field("p1_25", &format_args!("{}", self.p1_25().bits()))
            .field("p1_26", &format_args!("{}", self.p1_26().bits()))
            .field("p1_27", &format_args!("{}", self.p1_27().bits()))
            .field("p1_28", &format_args!("{}", self.p1_28().bits()))
            .field("p1_29", &format_args!("{}", self.p1_29().bits()))
            .field("p1_30", &format_args!("{}", self.p1_30().bits()))
            .field("p1_31", &format_args!("{}", self.p1_31().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<PINSEL3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Pin function select P1.16."]
    #[inline(always)]
    #[must_use]
    pub fn p1_16(&mut self) -> P1_16_W<PINSEL3_SPEC, 0> {
        P1_16_W::new(self)
    }
    #[doc = "Bits 2:3 - Pin function select P1.17."]
    #[inline(always)]
    #[must_use]
    pub fn p1_17(&mut self) -> P1_17_W<PINSEL3_SPEC, 2> {
        P1_17_W::new(self)
    }
    #[doc = "Bits 4:5 - Pin function select P1.18."]
    #[inline(always)]
    #[must_use]
    pub fn p1_18(&mut self) -> P1_18_W<PINSEL3_SPEC, 4> {
        P1_18_W::new(self)
    }
    #[doc = "Bits 6:7 - Pin function select P1.19."]
    #[inline(always)]
    #[must_use]
    pub fn p1_19(&mut self) -> P1_19_W<PINSEL3_SPEC, 6> {
        P1_19_W::new(self)
    }
    #[doc = "Bits 8:9 - Pin function select P1.20."]
    #[inline(always)]
    #[must_use]
    pub fn p1_20(&mut self) -> P1_20_W<PINSEL3_SPEC, 8> {
        P1_20_W::new(self)
    }
    #[doc = "Bits 10:11 - Pin function select P1.21."]
    #[inline(always)]
    #[must_use]
    pub fn p1_21(&mut self) -> P1_21_W<PINSEL3_SPEC, 10> {
        P1_21_W::new(self)
    }
    #[doc = "Bits 12:13 - Pin function select P1.22"]
    #[inline(always)]
    #[must_use]
    pub fn p1_22(&mut self) -> P1_22_W<PINSEL3_SPEC, 12> {
        P1_22_W::new(self)
    }
    #[doc = "Bits 14:15 - Pin function select P1.23."]
    #[inline(always)]
    #[must_use]
    pub fn p1_23(&mut self) -> P1_23_W<PINSEL3_SPEC, 14> {
        P1_23_W::new(self)
    }
    #[doc = "Bits 16:17 - Pin function select P1.24."]
    #[inline(always)]
    #[must_use]
    pub fn p1_24(&mut self) -> P1_24_W<PINSEL3_SPEC, 16> {
        P1_24_W::new(self)
    }
    #[doc = "Bits 18:19 - Pin function select P1.25."]
    #[inline(always)]
    #[must_use]
    pub fn p1_25(&mut self) -> P1_25_W<PINSEL3_SPEC, 18> {
        P1_25_W::new(self)
    }
    #[doc = "Bits 20:21 - Pin function select P1.26."]
    #[inline(always)]
    #[must_use]
    pub fn p1_26(&mut self) -> P1_26_W<PINSEL3_SPEC, 20> {
        P1_26_W::new(self)
    }
    #[doc = "Bits 22:23 - Pin function select P1.27."]
    #[inline(always)]
    #[must_use]
    pub fn p1_27(&mut self) -> P1_27_W<PINSEL3_SPEC, 22> {
        P1_27_W::new(self)
    }
    #[doc = "Bits 24:25 - Pin function select P1.28."]
    #[inline(always)]
    #[must_use]
    pub fn p1_28(&mut self) -> P1_28_W<PINSEL3_SPEC, 24> {
        P1_28_W::new(self)
    }
    #[doc = "Bits 26:27 - Pin function select P1.29"]
    #[inline(always)]
    #[must_use]
    pub fn p1_29(&mut self) -> P1_29_W<PINSEL3_SPEC, 26> {
        P1_29_W::new(self)
    }
    #[doc = "Bits 28:29 - Pin function select P1.30."]
    #[inline(always)]
    #[must_use]
    pub fn p1_30(&mut self) -> P1_30_W<PINSEL3_SPEC, 28> {
        P1_30_W::new(self)
    }
    #[doc = "Bits 30:31 - Pin function select P1.31."]
    #[inline(always)]
    #[must_use]
    pub fn p1_31(&mut self) -> P1_31_W<PINSEL3_SPEC, 30> {
        P1_31_W::new(self)
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
#[doc = "Pin function select register 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinsel3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinsel3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PINSEL3_SPEC;
impl crate::RegisterSpec for PINSEL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinsel3::R`](R) reader structure"]
impl crate::Readable for PINSEL3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pinsel3::W`](W) writer structure"]
impl crate::Writable for PINSEL3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINSEL3 to value 0"]
impl crate::Resettable for PINSEL3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
