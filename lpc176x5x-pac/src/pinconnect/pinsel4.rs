#[doc = "Register `PINSEL4` reader"]
pub type R = crate::R<PINSEL4_SPEC>;
#[doc = "Register `PINSEL4` writer"]
pub type W = crate::W<PINSEL4_SPEC>;
#[doc = "Field `P2_00` reader - Pin function select P2.0."]
pub type P2_00_R = crate::FieldReader<P2_00_A>;
#[doc = "Pin function select P2.0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_00_A {
    #[doc = "0: GPIO P2.0"]
    GPIO_P2 = 0,
    #[doc = "1: PWM1.1"]
    PWM1 = 1,
    #[doc = "2: TXD1"]
    TXD1 = 2,
}
impl From<P2_00_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_00_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_00_A {
    type Ux = u8;
}
impl P2_00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_00_A {
        match self.bits {
            0 => P2_00_A::GPIO_P2,
            1 => P2_00_A::PWM1,
            2 => P2_00_A::TXD1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P2.0"]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_00_A::GPIO_P2
    }
    #[doc = "PWM1.1"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P2_00_A::PWM1
    }
    #[doc = "TXD1"]
    #[inline(always)]
    pub fn is_txd1(&self) -> bool {
        *self == P2_00_A::TXD1
    }
}
#[doc = "Field `P2_00` writer - Pin function select P2.0."]
pub type P2_00_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, P2_00_A>;
impl<'a, REG, const O: u8> P2_00_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P2.0"]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut crate::W<REG> {
        self.variant(P2_00_A::GPIO_P2)
    }
    #[doc = "PWM1.1"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(P2_00_A::PWM1)
    }
    #[doc = "TXD1"]
    #[inline(always)]
    pub fn txd1(self) -> &'a mut crate::W<REG> {
        self.variant(P2_00_A::TXD1)
    }
}
#[doc = "Field `P2_01` reader - Pin function select P2.1."]
pub type P2_01_R = crate::FieldReader<P2_01_A>;
#[doc = "Pin function select P2.1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_01_A {
    #[doc = "0: GPIO P2.1"]
    GPIO_P2 = 0,
    #[doc = "1: PWM1.2"]
    PWM1 = 1,
    #[doc = "2: RXD1"]
    RXD1 = 2,
}
impl From<P2_01_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_01_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_01_A {
    type Ux = u8;
}
impl P2_01_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_01_A {
        match self.bits {
            0 => P2_01_A::GPIO_P2,
            1 => P2_01_A::PWM1,
            2 => P2_01_A::RXD1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P2.1"]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_01_A::GPIO_P2
    }
    #[doc = "PWM1.2"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P2_01_A::PWM1
    }
    #[doc = "RXD1"]
    #[inline(always)]
    pub fn is_rxd1(&self) -> bool {
        *self == P2_01_A::RXD1
    }
}
#[doc = "Field `P2_01` writer - Pin function select P2.1."]
pub type P2_01_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, P2_01_A>;
impl<'a, REG, const O: u8> P2_01_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P2.1"]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut crate::W<REG> {
        self.variant(P2_01_A::GPIO_P2)
    }
    #[doc = "PWM1.2"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(P2_01_A::PWM1)
    }
    #[doc = "RXD1"]
    #[inline(always)]
    pub fn rxd1(self) -> &'a mut crate::W<REG> {
        self.variant(P2_01_A::RXD1)
    }
}
#[doc = "Field `P2_02` reader - Pin function select P2.2."]
pub type P2_02_R = crate::FieldReader<P2_02_A>;
#[doc = "Pin function select P2.2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_02_A {
    #[doc = "0: GPIO P2.2"]
    GPIO_P2 = 0,
    #[doc = "1: PWM1.3"]
    PWM1 = 1,
    #[doc = "2: CTS1"]
    CTS1 = 2,
}
impl From<P2_02_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_02_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_02_A {
    type Ux = u8;
}
impl P2_02_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_02_A {
        match self.bits {
            0 => P2_02_A::GPIO_P2,
            1 => P2_02_A::PWM1,
            2 => P2_02_A::CTS1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P2.2"]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_02_A::GPIO_P2
    }
    #[doc = "PWM1.3"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P2_02_A::PWM1
    }
    #[doc = "CTS1"]
    #[inline(always)]
    pub fn is_cts1(&self) -> bool {
        *self == P2_02_A::CTS1
    }
}
#[doc = "Field `P2_02` writer - Pin function select P2.2."]
pub type P2_02_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, P2_02_A>;
impl<'a, REG, const O: u8> P2_02_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P2.2"]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut crate::W<REG> {
        self.variant(P2_02_A::GPIO_P2)
    }
    #[doc = "PWM1.3"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(P2_02_A::PWM1)
    }
    #[doc = "CTS1"]
    #[inline(always)]
    pub fn cts1(self) -> &'a mut crate::W<REG> {
        self.variant(P2_02_A::CTS1)
    }
}
#[doc = "Field `P2_03` reader - Pin function select P2.3."]
pub type P2_03_R = crate::FieldReader<P2_03_A>;
#[doc = "Pin function select P2.3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_03_A {
    #[doc = "0: GPIO P2.3."]
    GPIO_P2 = 0,
    #[doc = "1: PWM1.4"]
    PWM1 = 1,
    #[doc = "2: DCD1"]
    DCD1 = 2,
}
impl From<P2_03_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_03_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_03_A {
    type Ux = u8;
}
impl P2_03_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_03_A {
        match self.bits {
            0 => P2_03_A::GPIO_P2,
            1 => P2_03_A::PWM1,
            2 => P2_03_A::DCD1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P2.3."]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_03_A::GPIO_P2
    }
    #[doc = "PWM1.4"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P2_03_A::PWM1
    }
    #[doc = "DCD1"]
    #[inline(always)]
    pub fn is_dcd1(&self) -> bool {
        *self == P2_03_A::DCD1
    }
}
#[doc = "Field `P2_03` writer - Pin function select P2.3."]
pub type P2_03_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, P2_03_A>;
impl<'a, REG, const O: u8> P2_03_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P2.3."]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut crate::W<REG> {
        self.variant(P2_03_A::GPIO_P2)
    }
    #[doc = "PWM1.4"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(P2_03_A::PWM1)
    }
    #[doc = "DCD1"]
    #[inline(always)]
    pub fn dcd1(self) -> &'a mut crate::W<REG> {
        self.variant(P2_03_A::DCD1)
    }
}
#[doc = "Field `P2_04` reader - Pin function select P2.4."]
pub type P2_04_R = crate::FieldReader<P2_04_A>;
#[doc = "Pin function select P2.4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_04_A {
    #[doc = "0: GPIO P2.4."]
    GPIO_P2 = 0,
    #[doc = "1: PWM1.5"]
    PWM1 = 1,
    #[doc = "2: DSR1"]
    DSR1 = 2,
}
impl From<P2_04_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_04_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_04_A {
    type Ux = u8;
}
impl P2_04_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_04_A {
        match self.bits {
            0 => P2_04_A::GPIO_P2,
            1 => P2_04_A::PWM1,
            2 => P2_04_A::DSR1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P2.4."]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_04_A::GPIO_P2
    }
    #[doc = "PWM1.5"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P2_04_A::PWM1
    }
    #[doc = "DSR1"]
    #[inline(always)]
    pub fn is_dsr1(&self) -> bool {
        *self == P2_04_A::DSR1
    }
}
#[doc = "Field `P2_04` writer - Pin function select P2.4."]
pub type P2_04_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, P2_04_A>;
impl<'a, REG, const O: u8> P2_04_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P2.4."]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut crate::W<REG> {
        self.variant(P2_04_A::GPIO_P2)
    }
    #[doc = "PWM1.5"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(P2_04_A::PWM1)
    }
    #[doc = "DSR1"]
    #[inline(always)]
    pub fn dsr1(self) -> &'a mut crate::W<REG> {
        self.variant(P2_04_A::DSR1)
    }
}
#[doc = "Field `P2_05` reader - Pin function select P2.5."]
pub type P2_05_R = crate::FieldReader<P2_05_A>;
#[doc = "Pin function select P2.5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_05_A {
    #[doc = "0: GPIO P2.5."]
    GPIO_P2 = 0,
    #[doc = "1: PWM1.6"]
    PWM1 = 1,
    #[doc = "2: DTR1"]
    DTR1 = 2,
}
impl From<P2_05_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_05_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_05_A {
    type Ux = u8;
}
impl P2_05_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_05_A {
        match self.bits {
            0 => P2_05_A::GPIO_P2,
            1 => P2_05_A::PWM1,
            2 => P2_05_A::DTR1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P2.5."]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_05_A::GPIO_P2
    }
    #[doc = "PWM1.6"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == P2_05_A::PWM1
    }
    #[doc = "DTR1"]
    #[inline(always)]
    pub fn is_dtr1(&self) -> bool {
        *self == P2_05_A::DTR1
    }
}
#[doc = "Field `P2_05` writer - Pin function select P2.5."]
pub type P2_05_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, P2_05_A>;
impl<'a, REG, const O: u8> P2_05_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P2.5."]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut crate::W<REG> {
        self.variant(P2_05_A::GPIO_P2)
    }
    #[doc = "PWM1.6"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(P2_05_A::PWM1)
    }
    #[doc = "DTR1"]
    #[inline(always)]
    pub fn dtr1(self) -> &'a mut crate::W<REG> {
        self.variant(P2_05_A::DTR1)
    }
}
#[doc = "Field `P2_06` reader - Pin function select P2.6."]
pub type P2_06_R = crate::FieldReader<P2_06_A>;
#[doc = "Pin function select P2.6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_06_A {
    #[doc = "0: GPIO P2.6."]
    GPIO_P2 = 0,
    #[doc = "1: PCAP1.0"]
    PCAP1 = 1,
    #[doc = "2: RI1"]
    RI1 = 2,
}
impl From<P2_06_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_06_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_06_A {
    type Ux = u8;
}
impl P2_06_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_06_A {
        match self.bits {
            0 => P2_06_A::GPIO_P2,
            1 => P2_06_A::PCAP1,
            2 => P2_06_A::RI1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P2.6."]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_06_A::GPIO_P2
    }
    #[doc = "PCAP1.0"]
    #[inline(always)]
    pub fn is_pcap1(&self) -> bool {
        *self == P2_06_A::PCAP1
    }
    #[doc = "RI1"]
    #[inline(always)]
    pub fn is_ri1(&self) -> bool {
        *self == P2_06_A::RI1
    }
}
#[doc = "Field `P2_06` writer - Pin function select P2.6."]
pub type P2_06_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, P2_06_A>;
impl<'a, REG, const O: u8> P2_06_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P2.6."]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut crate::W<REG> {
        self.variant(P2_06_A::GPIO_P2)
    }
    #[doc = "PCAP1.0"]
    #[inline(always)]
    pub fn pcap1(self) -> &'a mut crate::W<REG> {
        self.variant(P2_06_A::PCAP1)
    }
    #[doc = "RI1"]
    #[inline(always)]
    pub fn ri1(self) -> &'a mut crate::W<REG> {
        self.variant(P2_06_A::RI1)
    }
}
#[doc = "Field `P2_07` reader - Pin function select P2.7."]
pub type P2_07_R = crate::FieldReader<P2_07_A>;
#[doc = "Pin function select P2.7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_07_A {
    #[doc = "0: GPIO P2.7."]
    GPIO_P2 = 0,
    #[doc = "1: RD2"]
    RD2 = 1,
    #[doc = "2: RTS1"]
    RTS1 = 2,
}
impl From<P2_07_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_07_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_07_A {
    type Ux = u8;
}
impl P2_07_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_07_A {
        match self.bits {
            0 => P2_07_A::GPIO_P2,
            1 => P2_07_A::RD2,
            2 => P2_07_A::RTS1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P2.7."]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_07_A::GPIO_P2
    }
    #[doc = "RD2"]
    #[inline(always)]
    pub fn is_rd2(&self) -> bool {
        *self == P2_07_A::RD2
    }
    #[doc = "RTS1"]
    #[inline(always)]
    pub fn is_rts1(&self) -> bool {
        *self == P2_07_A::RTS1
    }
}
#[doc = "Field `P2_07` writer - Pin function select P2.7."]
pub type P2_07_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, P2_07_A>;
impl<'a, REG, const O: u8> P2_07_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P2.7."]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut crate::W<REG> {
        self.variant(P2_07_A::GPIO_P2)
    }
    #[doc = "RD2"]
    #[inline(always)]
    pub fn rd2(self) -> &'a mut crate::W<REG> {
        self.variant(P2_07_A::RD2)
    }
    #[doc = "RTS1"]
    #[inline(always)]
    pub fn rts1(self) -> &'a mut crate::W<REG> {
        self.variant(P2_07_A::RTS1)
    }
}
#[doc = "Field `P2_08` reader - Pin function select P2.8."]
pub type P2_08_R = crate::FieldReader<P2_08_A>;
#[doc = "Pin function select P2.8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_08_A {
    #[doc = "0: GPIO P2.8."]
    GPIO_P2 = 0,
    #[doc = "1: TD2"]
    TD2 = 1,
    #[doc = "2: TXD2"]
    TXD2 = 2,
    #[doc = "3: ENET_MDC"]
    ENET_MDC = 3,
}
impl From<P2_08_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_08_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_08_A {
    type Ux = u8;
}
impl P2_08_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_08_A {
        match self.bits {
            0 => P2_08_A::GPIO_P2,
            1 => P2_08_A::TD2,
            2 => P2_08_A::TXD2,
            3 => P2_08_A::ENET_MDC,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P2.8."]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_08_A::GPIO_P2
    }
    #[doc = "TD2"]
    #[inline(always)]
    pub fn is_td2(&self) -> bool {
        *self == P2_08_A::TD2
    }
    #[doc = "TXD2"]
    #[inline(always)]
    pub fn is_txd2(&self) -> bool {
        *self == P2_08_A::TXD2
    }
    #[doc = "ENET_MDC"]
    #[inline(always)]
    pub fn is_enet_mdc(&self) -> bool {
        *self == P2_08_A::ENET_MDC
    }
}
#[doc = "Field `P2_08` writer - Pin function select P2.8."]
pub type P2_08_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, P2_08_A>;
impl<'a, REG, const O: u8> P2_08_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P2.8."]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut crate::W<REG> {
        self.variant(P2_08_A::GPIO_P2)
    }
    #[doc = "TD2"]
    #[inline(always)]
    pub fn td2(self) -> &'a mut crate::W<REG> {
        self.variant(P2_08_A::TD2)
    }
    #[doc = "TXD2"]
    #[inline(always)]
    pub fn txd2(self) -> &'a mut crate::W<REG> {
        self.variant(P2_08_A::TXD2)
    }
    #[doc = "ENET_MDC"]
    #[inline(always)]
    pub fn enet_mdc(self) -> &'a mut crate::W<REG> {
        self.variant(P2_08_A::ENET_MDC)
    }
}
#[doc = "Field `P2_09` reader - Pin function select P2.9."]
pub type P2_09_R = crate::FieldReader<P2_09_A>;
#[doc = "Pin function select P2.9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_09_A {
    #[doc = "0: GPIO P2.9"]
    GPIO_P2 = 0,
    #[doc = "1: USB_CONNECT"]
    USB_CONNECT = 1,
    #[doc = "2: RXD2"]
    RXD2 = 2,
    #[doc = "3: ENET_MDIO"]
    ENET_MDIO = 3,
}
impl From<P2_09_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_09_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_09_A {
    type Ux = u8;
}
impl P2_09_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_09_A {
        match self.bits {
            0 => P2_09_A::GPIO_P2,
            1 => P2_09_A::USB_CONNECT,
            2 => P2_09_A::RXD2,
            3 => P2_09_A::ENET_MDIO,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P2.9"]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_09_A::GPIO_P2
    }
    #[doc = "USB_CONNECT"]
    #[inline(always)]
    pub fn is_usb_connect(&self) -> bool {
        *self == P2_09_A::USB_CONNECT
    }
    #[doc = "RXD2"]
    #[inline(always)]
    pub fn is_rxd2(&self) -> bool {
        *self == P2_09_A::RXD2
    }
    #[doc = "ENET_MDIO"]
    #[inline(always)]
    pub fn is_enet_mdio(&self) -> bool {
        *self == P2_09_A::ENET_MDIO
    }
}
#[doc = "Field `P2_09` writer - Pin function select P2.9."]
pub type P2_09_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, P2_09_A>;
impl<'a, REG, const O: u8> P2_09_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P2.9"]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut crate::W<REG> {
        self.variant(P2_09_A::GPIO_P2)
    }
    #[doc = "USB_CONNECT"]
    #[inline(always)]
    pub fn usb_connect(self) -> &'a mut crate::W<REG> {
        self.variant(P2_09_A::USB_CONNECT)
    }
    #[doc = "RXD2"]
    #[inline(always)]
    pub fn rxd2(self) -> &'a mut crate::W<REG> {
        self.variant(P2_09_A::RXD2)
    }
    #[doc = "ENET_MDIO"]
    #[inline(always)]
    pub fn enet_mdio(self) -> &'a mut crate::W<REG> {
        self.variant(P2_09_A::ENET_MDIO)
    }
}
#[doc = "Field `P2_10` reader - Pin function select P2.10."]
pub type P2_10_R = crate::FieldReader<P2_10_A>;
#[doc = "Pin function select P2.10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_10_A {
    #[doc = "0: GPIO P2.10"]
    GPIO_P2 = 0,
    #[doc = "1: EINT0"]
    EINT0 = 1,
    #[doc = "2: NMI"]
    NMI = 2,
}
impl From<P2_10_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_10_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_10_A {
    type Ux = u8;
}
impl P2_10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_10_A {
        match self.bits {
            0 => P2_10_A::GPIO_P2,
            1 => P2_10_A::EINT0,
            2 => P2_10_A::NMI,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P2.10"]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_10_A::GPIO_P2
    }
    #[doc = "EINT0"]
    #[inline(always)]
    pub fn is_eint0(&self) -> bool {
        *self == P2_10_A::EINT0
    }
    #[doc = "NMI"]
    #[inline(always)]
    pub fn is_nmi(&self) -> bool {
        *self == P2_10_A::NMI
    }
}
#[doc = "Field `P2_10` writer - Pin function select P2.10."]
pub type P2_10_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, P2_10_A>;
impl<'a, REG, const O: u8> P2_10_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P2.10"]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut crate::W<REG> {
        self.variant(P2_10_A::GPIO_P2)
    }
    #[doc = "EINT0"]
    #[inline(always)]
    pub fn eint0(self) -> &'a mut crate::W<REG> {
        self.variant(P2_10_A::EINT0)
    }
    #[doc = "NMI"]
    #[inline(always)]
    pub fn nmi(self) -> &'a mut crate::W<REG> {
        self.variant(P2_10_A::NMI)
    }
}
#[doc = "Field `P2_11` reader - Pin function select P2.11."]
pub type P2_11_R = crate::FieldReader<P2_11_A>;
#[doc = "Pin function select P2.11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_11_A {
    #[doc = "0: GPIO P2.11"]
    GPIO_P2 = 0,
    #[doc = "1: EINT1"]
    EINT1 = 1,
    #[doc = "3: I2STX_CLK"]
    I2STX_CLK = 3,
}
impl From<P2_11_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_11_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_11_A {
    type Ux = u8;
}
impl P2_11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_11_A {
        match self.bits {
            0 => P2_11_A::GPIO_P2,
            1 => P2_11_A::EINT1,
            3 => P2_11_A::I2STX_CLK,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P2.11"]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_11_A::GPIO_P2
    }
    #[doc = "EINT1"]
    #[inline(always)]
    pub fn is_eint1(&self) -> bool {
        *self == P2_11_A::EINT1
    }
    #[doc = "I2STX_CLK"]
    #[inline(always)]
    pub fn is_i2stx_clk(&self) -> bool {
        *self == P2_11_A::I2STX_CLK
    }
}
#[doc = "Field `P2_11` writer - Pin function select P2.11."]
pub type P2_11_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, P2_11_A>;
impl<'a, REG, const O: u8> P2_11_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P2.11"]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut crate::W<REG> {
        self.variant(P2_11_A::GPIO_P2)
    }
    #[doc = "EINT1"]
    #[inline(always)]
    pub fn eint1(self) -> &'a mut crate::W<REG> {
        self.variant(P2_11_A::EINT1)
    }
    #[doc = "I2STX_CLK"]
    #[inline(always)]
    pub fn i2stx_clk(self) -> &'a mut crate::W<REG> {
        self.variant(P2_11_A::I2STX_CLK)
    }
}
#[doc = "Field `P2_12` reader - Pin function select P2.12."]
pub type P2_12_R = crate::FieldReader<P2_12_A>;
#[doc = "Pin function select P2.12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_12_A {
    #[doc = "0: GPIO P2.12"]
    GPIO_P2 = 0,
    #[doc = "1: EINT2"]
    EINT2 = 1,
    #[doc = "3: I2STX_WS"]
    I2STX_WS = 3,
}
impl From<P2_12_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_12_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_12_A {
    type Ux = u8;
}
impl P2_12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_12_A {
        match self.bits {
            0 => P2_12_A::GPIO_P2,
            1 => P2_12_A::EINT2,
            3 => P2_12_A::I2STX_WS,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P2.12"]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_12_A::GPIO_P2
    }
    #[doc = "EINT2"]
    #[inline(always)]
    pub fn is_eint2(&self) -> bool {
        *self == P2_12_A::EINT2
    }
    #[doc = "I2STX_WS"]
    #[inline(always)]
    pub fn is_i2stx_ws(&self) -> bool {
        *self == P2_12_A::I2STX_WS
    }
}
#[doc = "Field `P2_12` writer - Pin function select P2.12."]
pub type P2_12_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, P2_12_A>;
impl<'a, REG, const O: u8> P2_12_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P2.12"]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut crate::W<REG> {
        self.variant(P2_12_A::GPIO_P2)
    }
    #[doc = "EINT2"]
    #[inline(always)]
    pub fn eint2(self) -> &'a mut crate::W<REG> {
        self.variant(P2_12_A::EINT2)
    }
    #[doc = "I2STX_WS"]
    #[inline(always)]
    pub fn i2stx_ws(self) -> &'a mut crate::W<REG> {
        self.variant(P2_12_A::I2STX_WS)
    }
}
#[doc = "Field `P2_13` reader - Pin function select P2.13."]
pub type P2_13_R = crate::FieldReader<P2_13_A>;
#[doc = "Pin function select P2.13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2_13_A {
    #[doc = "0: GPIO P2.13"]
    GPIO_P2 = 0,
    #[doc = "1: EINT3"]
    EINT3 = 1,
    #[doc = "3: I2STX_SDA"]
    I2STX_SDA = 3,
}
impl From<P2_13_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_13_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2_13_A {
    type Ux = u8;
}
impl P2_13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2_13_A {
        match self.bits {
            0 => P2_13_A::GPIO_P2,
            1 => P2_13_A::EINT3,
            3 => P2_13_A::I2STX_SDA,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P2.13"]
    #[inline(always)]
    pub fn is_gpio_p2(&self) -> bool {
        *self == P2_13_A::GPIO_P2
    }
    #[doc = "EINT3"]
    #[inline(always)]
    pub fn is_eint3(&self) -> bool {
        *self == P2_13_A::EINT3
    }
    #[doc = "I2STX_SDA"]
    #[inline(always)]
    pub fn is_i2stx_sda(&self) -> bool {
        *self == P2_13_A::I2STX_SDA
    }
}
#[doc = "Field `P2_13` writer - Pin function select P2.13."]
pub type P2_13_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, P2_13_A>;
impl<'a, REG, const O: u8> P2_13_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P2.13"]
    #[inline(always)]
    pub fn gpio_p2(self) -> &'a mut crate::W<REG> {
        self.variant(P2_13_A::GPIO_P2)
    }
    #[doc = "EINT3"]
    #[inline(always)]
    pub fn eint3(self) -> &'a mut crate::W<REG> {
        self.variant(P2_13_A::EINT3)
    }
    #[doc = "I2STX_SDA"]
    #[inline(always)]
    pub fn i2stx_sda(self) -> &'a mut crate::W<REG> {
        self.variant(P2_13_A::I2STX_SDA)
    }
}
impl R {
    #[doc = "Bits 0:1 - Pin function select P2.0."]
    #[inline(always)]
    pub fn p2_00(&self) -> P2_00_R {
        P2_00_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Pin function select P2.1."]
    #[inline(always)]
    pub fn p2_01(&self) -> P2_01_R {
        P2_01_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Pin function select P2.2."]
    #[inline(always)]
    pub fn p2_02(&self) -> P2_02_R {
        P2_02_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Pin function select P2.3."]
    #[inline(always)]
    pub fn p2_03(&self) -> P2_03_R {
        P2_03_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin function select P2.4."]
    #[inline(always)]
    pub fn p2_04(&self) -> P2_04_R {
        P2_04_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Pin function select P2.5."]
    #[inline(always)]
    pub fn p2_05(&self) -> P2_05_R {
        P2_05_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Pin function select P2.6."]
    #[inline(always)]
    pub fn p2_06(&self) -> P2_06_R {
        P2_06_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Pin function select P2.7."]
    #[inline(always)]
    pub fn p2_07(&self) -> P2_07_R {
        P2_07_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Pin function select P2.8."]
    #[inline(always)]
    pub fn p2_08(&self) -> P2_08_R {
        P2_08_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Pin function select P2.9."]
    #[inline(always)]
    pub fn p2_09(&self) -> P2_09_R {
        P2_09_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Pin function select P2.10."]
    #[inline(always)]
    pub fn p2_10(&self) -> P2_10_R {
        P2_10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Pin function select P2.11."]
    #[inline(always)]
    pub fn p2_11(&self) -> P2_11_R {
        P2_11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Pin function select P2.12."]
    #[inline(always)]
    pub fn p2_12(&self) -> P2_12_R {
        P2_12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Pin function select P2.13."]
    #[inline(always)]
    pub fn p2_13(&self) -> P2_13_R {
        P2_13_R::new(((self.bits >> 26) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PINSEL4")
            .field("p2_00", &format_args!("{}", self.p2_00().bits()))
            .field("p2_01", &format_args!("{}", self.p2_01().bits()))
            .field("p2_02", &format_args!("{}", self.p2_02().bits()))
            .field("p2_03", &format_args!("{}", self.p2_03().bits()))
            .field("p2_04", &format_args!("{}", self.p2_04().bits()))
            .field("p2_05", &format_args!("{}", self.p2_05().bits()))
            .field("p2_06", &format_args!("{}", self.p2_06().bits()))
            .field("p2_07", &format_args!("{}", self.p2_07().bits()))
            .field("p2_08", &format_args!("{}", self.p2_08().bits()))
            .field("p2_09", &format_args!("{}", self.p2_09().bits()))
            .field("p2_10", &format_args!("{}", self.p2_10().bits()))
            .field("p2_11", &format_args!("{}", self.p2_11().bits()))
            .field("p2_12", &format_args!("{}", self.p2_12().bits()))
            .field("p2_13", &format_args!("{}", self.p2_13().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<PINSEL4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Pin function select P2.0."]
    #[inline(always)]
    #[must_use]
    pub fn p2_00(&mut self) -> P2_00_W<PINSEL4_SPEC, 0> {
        P2_00_W::new(self)
    }
    #[doc = "Bits 2:3 - Pin function select P2.1."]
    #[inline(always)]
    #[must_use]
    pub fn p2_01(&mut self) -> P2_01_W<PINSEL4_SPEC, 2> {
        P2_01_W::new(self)
    }
    #[doc = "Bits 4:5 - Pin function select P2.2."]
    #[inline(always)]
    #[must_use]
    pub fn p2_02(&mut self) -> P2_02_W<PINSEL4_SPEC, 4> {
        P2_02_W::new(self)
    }
    #[doc = "Bits 6:7 - Pin function select P2.3."]
    #[inline(always)]
    #[must_use]
    pub fn p2_03(&mut self) -> P2_03_W<PINSEL4_SPEC, 6> {
        P2_03_W::new(self)
    }
    #[doc = "Bits 8:9 - Pin function select P2.4."]
    #[inline(always)]
    #[must_use]
    pub fn p2_04(&mut self) -> P2_04_W<PINSEL4_SPEC, 8> {
        P2_04_W::new(self)
    }
    #[doc = "Bits 10:11 - Pin function select P2.5."]
    #[inline(always)]
    #[must_use]
    pub fn p2_05(&mut self) -> P2_05_W<PINSEL4_SPEC, 10> {
        P2_05_W::new(self)
    }
    #[doc = "Bits 12:13 - Pin function select P2.6."]
    #[inline(always)]
    #[must_use]
    pub fn p2_06(&mut self) -> P2_06_W<PINSEL4_SPEC, 12> {
        P2_06_W::new(self)
    }
    #[doc = "Bits 14:15 - Pin function select P2.7."]
    #[inline(always)]
    #[must_use]
    pub fn p2_07(&mut self) -> P2_07_W<PINSEL4_SPEC, 14> {
        P2_07_W::new(self)
    }
    #[doc = "Bits 16:17 - Pin function select P2.8."]
    #[inline(always)]
    #[must_use]
    pub fn p2_08(&mut self) -> P2_08_W<PINSEL4_SPEC, 16> {
        P2_08_W::new(self)
    }
    #[doc = "Bits 18:19 - Pin function select P2.9."]
    #[inline(always)]
    #[must_use]
    pub fn p2_09(&mut self) -> P2_09_W<PINSEL4_SPEC, 18> {
        P2_09_W::new(self)
    }
    #[doc = "Bits 20:21 - Pin function select P2.10."]
    #[inline(always)]
    #[must_use]
    pub fn p2_10(&mut self) -> P2_10_W<PINSEL4_SPEC, 20> {
        P2_10_W::new(self)
    }
    #[doc = "Bits 22:23 - Pin function select P2.11."]
    #[inline(always)]
    #[must_use]
    pub fn p2_11(&mut self) -> P2_11_W<PINSEL4_SPEC, 22> {
        P2_11_W::new(self)
    }
    #[doc = "Bits 24:25 - Pin function select P2.12."]
    #[inline(always)]
    #[must_use]
    pub fn p2_12(&mut self) -> P2_12_W<PINSEL4_SPEC, 24> {
        P2_12_W::new(self)
    }
    #[doc = "Bits 26:27 - Pin function select P2.13."]
    #[inline(always)]
    #[must_use]
    pub fn p2_13(&mut self) -> P2_13_W<PINSEL4_SPEC, 26> {
        P2_13_W::new(self)
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
#[doc = "Pin function select register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinsel4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinsel4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PINSEL4_SPEC;
impl crate::RegisterSpec for PINSEL4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinsel4::R`](R) reader structure"]
impl crate::Readable for PINSEL4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pinsel4::W`](W) writer structure"]
impl crate::Writable for PINSEL4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINSEL4 to value 0"]
impl crate::Resettable for PINSEL4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
