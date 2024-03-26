#[doc = "Register `PINSEL0` reader"]
pub type R = crate::R<PINSEL0_SPEC>;
#[doc = "Register `PINSEL0` writer"]
pub type W = crate::W<PINSEL0_SPEC>;
#[doc = "Field `P0_00` reader - Pin function select P0.0."]
pub type P0_00_R = crate::FieldReader<P0_00_A>;
#[doc = "Pin function select P0.0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_00_A {
    #[doc = "0: GPIO P0.0"]
    GPIO_P0 = 0,
    #[doc = "1: RD1"]
    RD1 = 1,
    #[doc = "2: TXD3"]
    TXD3 = 2,
    #[doc = "3: SDA1"]
    SDA1 = 3,
}
impl From<P0_00_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_00_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_00_A {
    type Ux = u8;
}
impl P0_00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_00_A {
        match self.bits {
            0 => P0_00_A::GPIO_P0,
            1 => P0_00_A::RD1,
            2 => P0_00_A::TXD3,
            3 => P0_00_A::SDA1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.0"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_00_A::GPIO_P0
    }
    #[doc = "RD1"]
    #[inline(always)]
    pub fn is_rd1(&self) -> bool {
        *self == P0_00_A::RD1
    }
    #[doc = "TXD3"]
    #[inline(always)]
    pub fn is_txd3(&self) -> bool {
        *self == P0_00_A::TXD3
    }
    #[doc = "SDA1"]
    #[inline(always)]
    pub fn is_sda1(&self) -> bool {
        *self == P0_00_A::SDA1
    }
}
#[doc = "Field `P0_00` writer - Pin function select P0.0."]
pub type P0_00_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, P0_00_A>;
impl<'a, REG, const O: u8> P0_00_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.0"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(P0_00_A::GPIO_P0)
    }
    #[doc = "RD1"]
    #[inline(always)]
    pub fn rd1(self) -> &'a mut crate::W<REG> {
        self.variant(P0_00_A::RD1)
    }
    #[doc = "TXD3"]
    #[inline(always)]
    pub fn txd3(self) -> &'a mut crate::W<REG> {
        self.variant(P0_00_A::TXD3)
    }
    #[doc = "SDA1"]
    #[inline(always)]
    pub fn sda1(self) -> &'a mut crate::W<REG> {
        self.variant(P0_00_A::SDA1)
    }
}
#[doc = "Field `P0_01` reader - Pin function select P0.1."]
pub type P0_01_R = crate::FieldReader<P0_01_A>;
#[doc = "Pin function select P0.1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_01_A {
    #[doc = "0: GPIO P0.1"]
    GPIO_P0 = 0,
    #[doc = "1: TD1"]
    TD1 = 1,
    #[doc = "2: RXD3"]
    RXD3 = 2,
    #[doc = "3: SCL1"]
    SCL1 = 3,
}
impl From<P0_01_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_01_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_01_A {
    type Ux = u8;
}
impl P0_01_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_01_A {
        match self.bits {
            0 => P0_01_A::GPIO_P0,
            1 => P0_01_A::TD1,
            2 => P0_01_A::RXD3,
            3 => P0_01_A::SCL1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.1"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_01_A::GPIO_P0
    }
    #[doc = "TD1"]
    #[inline(always)]
    pub fn is_td1(&self) -> bool {
        *self == P0_01_A::TD1
    }
    #[doc = "RXD3"]
    #[inline(always)]
    pub fn is_rxd3(&self) -> bool {
        *self == P0_01_A::RXD3
    }
    #[doc = "SCL1"]
    #[inline(always)]
    pub fn is_scl1(&self) -> bool {
        *self == P0_01_A::SCL1
    }
}
#[doc = "Field `P0_01` writer - Pin function select P0.1."]
pub type P0_01_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, P0_01_A>;
impl<'a, REG, const O: u8> P0_01_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.1"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(P0_01_A::GPIO_P0)
    }
    #[doc = "TD1"]
    #[inline(always)]
    pub fn td1(self) -> &'a mut crate::W<REG> {
        self.variant(P0_01_A::TD1)
    }
    #[doc = "RXD3"]
    #[inline(always)]
    pub fn rxd3(self) -> &'a mut crate::W<REG> {
        self.variant(P0_01_A::RXD3)
    }
    #[doc = "SCL1"]
    #[inline(always)]
    pub fn scl1(self) -> &'a mut crate::W<REG> {
        self.variant(P0_01_A::SCL1)
    }
}
#[doc = "Field `P0_02` reader - Pin function select P0.2."]
pub type P0_02_R = crate::FieldReader<P0_02_A>;
#[doc = "Pin function select P0.2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_02_A {
    #[doc = "0: GPIO P0.2"]
    GPIO_P0 = 0,
    #[doc = "1: TXD0"]
    TXD0 = 1,
    #[doc = "2: AD0.7"]
    AD0 = 2,
}
impl From<P0_02_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_02_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_02_A {
    type Ux = u8;
}
impl P0_02_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_02_A {
        match self.bits {
            0 => P0_02_A::GPIO_P0,
            1 => P0_02_A::TXD0,
            2 => P0_02_A::AD0,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.2"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_02_A::GPIO_P0
    }
    #[doc = "TXD0"]
    #[inline(always)]
    pub fn is_txd0(&self) -> bool {
        *self == P0_02_A::TXD0
    }
    #[doc = "AD0.7"]
    #[inline(always)]
    pub fn is_ad0(&self) -> bool {
        *self == P0_02_A::AD0
    }
}
#[doc = "Field `P0_02` writer - Pin function select P0.2."]
pub type P0_02_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, P0_02_A>;
impl<'a, REG, const O: u8> P0_02_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.2"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(P0_02_A::GPIO_P0)
    }
    #[doc = "TXD0"]
    #[inline(always)]
    pub fn txd0(self) -> &'a mut crate::W<REG> {
        self.variant(P0_02_A::TXD0)
    }
    #[doc = "AD0.7"]
    #[inline(always)]
    pub fn ad0(self) -> &'a mut crate::W<REG> {
        self.variant(P0_02_A::AD0)
    }
}
#[doc = "Field `P0_03` reader - Pin function select P0.3."]
pub type P0_03_R = crate::FieldReader<P0_03_A>;
#[doc = "Pin function select P0.3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_03_A {
    #[doc = "0: GPIO P0.3."]
    GPIO_P0 = 0,
    #[doc = "1: RXD0"]
    RXD0 = 1,
    #[doc = "2: AD0.6"]
    AD0 = 2,
}
impl From<P0_03_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_03_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_03_A {
    type Ux = u8;
}
impl P0_03_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_03_A {
        match self.bits {
            0 => P0_03_A::GPIO_P0,
            1 => P0_03_A::RXD0,
            2 => P0_03_A::AD0,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.3."]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_03_A::GPIO_P0
    }
    #[doc = "RXD0"]
    #[inline(always)]
    pub fn is_rxd0(&self) -> bool {
        *self == P0_03_A::RXD0
    }
    #[doc = "AD0.6"]
    #[inline(always)]
    pub fn is_ad0(&self) -> bool {
        *self == P0_03_A::AD0
    }
}
#[doc = "Field `P0_03` writer - Pin function select P0.3."]
pub type P0_03_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, P0_03_A>;
impl<'a, REG, const O: u8> P0_03_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.3."]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(P0_03_A::GPIO_P0)
    }
    #[doc = "RXD0"]
    #[inline(always)]
    pub fn rxd0(self) -> &'a mut crate::W<REG> {
        self.variant(P0_03_A::RXD0)
    }
    #[doc = "AD0.6"]
    #[inline(always)]
    pub fn ad0(self) -> &'a mut crate::W<REG> {
        self.variant(P0_03_A::AD0)
    }
}
#[doc = "Field `P0_04` reader - Pin function select P0.4."]
pub type P0_04_R = crate::FieldReader<P0_04_A>;
#[doc = "Pin function select P0.4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_04_A {
    #[doc = "0: GPIO P0.4."]
    GPIO_P0 = 0,
    #[doc = "1: I2SRX_CLK"]
    I2SRX_CLK = 1,
    #[doc = "2: RD2"]
    RD2 = 2,
    #[doc = "3: CAP2.0"]
    CAP2 = 3,
}
impl From<P0_04_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_04_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_04_A {
    type Ux = u8;
}
impl P0_04_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_04_A {
        match self.bits {
            0 => P0_04_A::GPIO_P0,
            1 => P0_04_A::I2SRX_CLK,
            2 => P0_04_A::RD2,
            3 => P0_04_A::CAP2,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.4."]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_04_A::GPIO_P0
    }
    #[doc = "I2SRX_CLK"]
    #[inline(always)]
    pub fn is_i2srx_clk(&self) -> bool {
        *self == P0_04_A::I2SRX_CLK
    }
    #[doc = "RD2"]
    #[inline(always)]
    pub fn is_rd2(&self) -> bool {
        *self == P0_04_A::RD2
    }
    #[doc = "CAP2.0"]
    #[inline(always)]
    pub fn is_cap2(&self) -> bool {
        *self == P0_04_A::CAP2
    }
}
#[doc = "Field `P0_04` writer - Pin function select P0.4."]
pub type P0_04_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, P0_04_A>;
impl<'a, REG, const O: u8> P0_04_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.4."]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(P0_04_A::GPIO_P0)
    }
    #[doc = "I2SRX_CLK"]
    #[inline(always)]
    pub fn i2srx_clk(self) -> &'a mut crate::W<REG> {
        self.variant(P0_04_A::I2SRX_CLK)
    }
    #[doc = "RD2"]
    #[inline(always)]
    pub fn rd2(self) -> &'a mut crate::W<REG> {
        self.variant(P0_04_A::RD2)
    }
    #[doc = "CAP2.0"]
    #[inline(always)]
    pub fn cap2(self) -> &'a mut crate::W<REG> {
        self.variant(P0_04_A::CAP2)
    }
}
#[doc = "Field `P0_05` reader - Pin function select P0.5."]
pub type P0_05_R = crate::FieldReader<P0_05_A>;
#[doc = "Pin function select P0.5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_05_A {
    #[doc = "0: GPIO P0.5."]
    GPIO_P0 = 0,
    #[doc = "1: I2SRX_WS"]
    I2SRX_WS = 1,
    #[doc = "2: TD2"]
    TD2 = 2,
    #[doc = "3: CAP2.1"]
    CAP2 = 3,
}
impl From<P0_05_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_05_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_05_A {
    type Ux = u8;
}
impl P0_05_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_05_A {
        match self.bits {
            0 => P0_05_A::GPIO_P0,
            1 => P0_05_A::I2SRX_WS,
            2 => P0_05_A::TD2,
            3 => P0_05_A::CAP2,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.5."]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_05_A::GPIO_P0
    }
    #[doc = "I2SRX_WS"]
    #[inline(always)]
    pub fn is_i2srx_ws(&self) -> bool {
        *self == P0_05_A::I2SRX_WS
    }
    #[doc = "TD2"]
    #[inline(always)]
    pub fn is_td2(&self) -> bool {
        *self == P0_05_A::TD2
    }
    #[doc = "CAP2.1"]
    #[inline(always)]
    pub fn is_cap2(&self) -> bool {
        *self == P0_05_A::CAP2
    }
}
#[doc = "Field `P0_05` writer - Pin function select P0.5."]
pub type P0_05_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, P0_05_A>;
impl<'a, REG, const O: u8> P0_05_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.5."]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(P0_05_A::GPIO_P0)
    }
    #[doc = "I2SRX_WS"]
    #[inline(always)]
    pub fn i2srx_ws(self) -> &'a mut crate::W<REG> {
        self.variant(P0_05_A::I2SRX_WS)
    }
    #[doc = "TD2"]
    #[inline(always)]
    pub fn td2(self) -> &'a mut crate::W<REG> {
        self.variant(P0_05_A::TD2)
    }
    #[doc = "CAP2.1"]
    #[inline(always)]
    pub fn cap2(self) -> &'a mut crate::W<REG> {
        self.variant(P0_05_A::CAP2)
    }
}
#[doc = "Field `P0_06` reader - Pin function select P0.6."]
pub type P0_06_R = crate::FieldReader<P0_06_A>;
#[doc = "Pin function select P0.6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_06_A {
    #[doc = "0: GPIO P0.6."]
    GPIO_P0 = 0,
    #[doc = "1: I2SRX_SDA"]
    I2SRX_SDA = 1,
    #[doc = "2: SSEL1"]
    SSEL1 = 2,
    #[doc = "3: MAT2.0"]
    MAT2 = 3,
}
impl From<P0_06_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_06_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_06_A {
    type Ux = u8;
}
impl P0_06_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_06_A {
        match self.bits {
            0 => P0_06_A::GPIO_P0,
            1 => P0_06_A::I2SRX_SDA,
            2 => P0_06_A::SSEL1,
            3 => P0_06_A::MAT2,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.6."]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_06_A::GPIO_P0
    }
    #[doc = "I2SRX_SDA"]
    #[inline(always)]
    pub fn is_i2srx_sda(&self) -> bool {
        *self == P0_06_A::I2SRX_SDA
    }
    #[doc = "SSEL1"]
    #[inline(always)]
    pub fn is_ssel1(&self) -> bool {
        *self == P0_06_A::SSEL1
    }
    #[doc = "MAT2.0"]
    #[inline(always)]
    pub fn is_mat2(&self) -> bool {
        *self == P0_06_A::MAT2
    }
}
#[doc = "Field `P0_06` writer - Pin function select P0.6."]
pub type P0_06_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, P0_06_A>;
impl<'a, REG, const O: u8> P0_06_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.6."]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(P0_06_A::GPIO_P0)
    }
    #[doc = "I2SRX_SDA"]
    #[inline(always)]
    pub fn i2srx_sda(self) -> &'a mut crate::W<REG> {
        self.variant(P0_06_A::I2SRX_SDA)
    }
    #[doc = "SSEL1"]
    #[inline(always)]
    pub fn ssel1(self) -> &'a mut crate::W<REG> {
        self.variant(P0_06_A::SSEL1)
    }
    #[doc = "MAT2.0"]
    #[inline(always)]
    pub fn mat2(self) -> &'a mut crate::W<REG> {
        self.variant(P0_06_A::MAT2)
    }
}
#[doc = "Field `P0_07` reader - Pin function select P0.7."]
pub type P0_07_R = crate::FieldReader<P0_07_A>;
#[doc = "Pin function select P0.7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_07_A {
    #[doc = "0: GPIO P0.7."]
    GPIO_P0 = 0,
    #[doc = "1: I2STX_CLK"]
    I2STX_CLK = 1,
    #[doc = "2: SCK1"]
    SCK1 = 2,
    #[doc = "3: MAT2.1"]
    MAT2 = 3,
}
impl From<P0_07_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_07_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_07_A {
    type Ux = u8;
}
impl P0_07_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_07_A {
        match self.bits {
            0 => P0_07_A::GPIO_P0,
            1 => P0_07_A::I2STX_CLK,
            2 => P0_07_A::SCK1,
            3 => P0_07_A::MAT2,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.7."]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_07_A::GPIO_P0
    }
    #[doc = "I2STX_CLK"]
    #[inline(always)]
    pub fn is_i2stx_clk(&self) -> bool {
        *self == P0_07_A::I2STX_CLK
    }
    #[doc = "SCK1"]
    #[inline(always)]
    pub fn is_sck1(&self) -> bool {
        *self == P0_07_A::SCK1
    }
    #[doc = "MAT2.1"]
    #[inline(always)]
    pub fn is_mat2(&self) -> bool {
        *self == P0_07_A::MAT2
    }
}
#[doc = "Field `P0_07` writer - Pin function select P0.7."]
pub type P0_07_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, P0_07_A>;
impl<'a, REG, const O: u8> P0_07_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.7."]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(P0_07_A::GPIO_P0)
    }
    #[doc = "I2STX_CLK"]
    #[inline(always)]
    pub fn i2stx_clk(self) -> &'a mut crate::W<REG> {
        self.variant(P0_07_A::I2STX_CLK)
    }
    #[doc = "SCK1"]
    #[inline(always)]
    pub fn sck1(self) -> &'a mut crate::W<REG> {
        self.variant(P0_07_A::SCK1)
    }
    #[doc = "MAT2.1"]
    #[inline(always)]
    pub fn mat2(self) -> &'a mut crate::W<REG> {
        self.variant(P0_07_A::MAT2)
    }
}
#[doc = "Field `P0_08` reader - Pin function select P0.8."]
pub type P0_08_R = crate::FieldReader<P0_08_A>;
#[doc = "Pin function select P0.8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_08_A {
    #[doc = "0: GPIO P0.8."]
    GPIO_P0 = 0,
    #[doc = "1: I2STX_WS"]
    I2STX_WS = 1,
    #[doc = "2: MISO1"]
    MISO1 = 2,
    #[doc = "3: MAT2.2"]
    MAT2 = 3,
}
impl From<P0_08_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_08_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_08_A {
    type Ux = u8;
}
impl P0_08_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_08_A {
        match self.bits {
            0 => P0_08_A::GPIO_P0,
            1 => P0_08_A::I2STX_WS,
            2 => P0_08_A::MISO1,
            3 => P0_08_A::MAT2,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.8."]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_08_A::GPIO_P0
    }
    #[doc = "I2STX_WS"]
    #[inline(always)]
    pub fn is_i2stx_ws(&self) -> bool {
        *self == P0_08_A::I2STX_WS
    }
    #[doc = "MISO1"]
    #[inline(always)]
    pub fn is_miso1(&self) -> bool {
        *self == P0_08_A::MISO1
    }
    #[doc = "MAT2.2"]
    #[inline(always)]
    pub fn is_mat2(&self) -> bool {
        *self == P0_08_A::MAT2
    }
}
#[doc = "Field `P0_08` writer - Pin function select P0.8."]
pub type P0_08_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, P0_08_A>;
impl<'a, REG, const O: u8> P0_08_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.8."]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(P0_08_A::GPIO_P0)
    }
    #[doc = "I2STX_WS"]
    #[inline(always)]
    pub fn i2stx_ws(self) -> &'a mut crate::W<REG> {
        self.variant(P0_08_A::I2STX_WS)
    }
    #[doc = "MISO1"]
    #[inline(always)]
    pub fn miso1(self) -> &'a mut crate::W<REG> {
        self.variant(P0_08_A::MISO1)
    }
    #[doc = "MAT2.2"]
    #[inline(always)]
    pub fn mat2(self) -> &'a mut crate::W<REG> {
        self.variant(P0_08_A::MAT2)
    }
}
#[doc = "Field `P0_09` reader - Pin function select P0.9."]
pub type P0_09_R = crate::FieldReader<P0_09_A>;
#[doc = "Pin function select P0.9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_09_A {
    #[doc = "0: GPIO P0.9"]
    GPIO_P0 = 0,
    #[doc = "1: I2STX_SDA"]
    I2STX_SDA = 1,
    #[doc = "2: MOSI1"]
    MOSI1 = 2,
    #[doc = "3: MAT2.3"]
    MAT2 = 3,
}
impl From<P0_09_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_09_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_09_A {
    type Ux = u8;
}
impl P0_09_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_09_A {
        match self.bits {
            0 => P0_09_A::GPIO_P0,
            1 => P0_09_A::I2STX_SDA,
            2 => P0_09_A::MOSI1,
            3 => P0_09_A::MAT2,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.9"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_09_A::GPIO_P0
    }
    #[doc = "I2STX_SDA"]
    #[inline(always)]
    pub fn is_i2stx_sda(&self) -> bool {
        *self == P0_09_A::I2STX_SDA
    }
    #[doc = "MOSI1"]
    #[inline(always)]
    pub fn is_mosi1(&self) -> bool {
        *self == P0_09_A::MOSI1
    }
    #[doc = "MAT2.3"]
    #[inline(always)]
    pub fn is_mat2(&self) -> bool {
        *self == P0_09_A::MAT2
    }
}
#[doc = "Field `P0_09` writer - Pin function select P0.9."]
pub type P0_09_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, P0_09_A>;
impl<'a, REG, const O: u8> P0_09_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.9"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(P0_09_A::GPIO_P0)
    }
    #[doc = "I2STX_SDA"]
    #[inline(always)]
    pub fn i2stx_sda(self) -> &'a mut crate::W<REG> {
        self.variant(P0_09_A::I2STX_SDA)
    }
    #[doc = "MOSI1"]
    #[inline(always)]
    pub fn mosi1(self) -> &'a mut crate::W<REG> {
        self.variant(P0_09_A::MOSI1)
    }
    #[doc = "MAT2.3"]
    #[inline(always)]
    pub fn mat2(self) -> &'a mut crate::W<REG> {
        self.variant(P0_09_A::MAT2)
    }
}
#[doc = "Field `P0_10` reader - Pin function select P0.10."]
pub type P0_10_R = crate::FieldReader<P0_10_A>;
#[doc = "Pin function select P0.10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_10_A {
    #[doc = "0: GPIO P0.10"]
    GPIO_P0 = 0,
    #[doc = "1: TXD2"]
    TXD2 = 1,
    #[doc = "2: SDA2"]
    SDA2 = 2,
    #[doc = "3: MAT3.0"]
    MAT3 = 3,
}
impl From<P0_10_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_10_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_10_A {
    type Ux = u8;
}
impl P0_10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_10_A {
        match self.bits {
            0 => P0_10_A::GPIO_P0,
            1 => P0_10_A::TXD2,
            2 => P0_10_A::SDA2,
            3 => P0_10_A::MAT3,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.10"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_10_A::GPIO_P0
    }
    #[doc = "TXD2"]
    #[inline(always)]
    pub fn is_txd2(&self) -> bool {
        *self == P0_10_A::TXD2
    }
    #[doc = "SDA2"]
    #[inline(always)]
    pub fn is_sda2(&self) -> bool {
        *self == P0_10_A::SDA2
    }
    #[doc = "MAT3.0"]
    #[inline(always)]
    pub fn is_mat3(&self) -> bool {
        *self == P0_10_A::MAT3
    }
}
#[doc = "Field `P0_10` writer - Pin function select P0.10."]
pub type P0_10_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, P0_10_A>;
impl<'a, REG, const O: u8> P0_10_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.10"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(P0_10_A::GPIO_P0)
    }
    #[doc = "TXD2"]
    #[inline(always)]
    pub fn txd2(self) -> &'a mut crate::W<REG> {
        self.variant(P0_10_A::TXD2)
    }
    #[doc = "SDA2"]
    #[inline(always)]
    pub fn sda2(self) -> &'a mut crate::W<REG> {
        self.variant(P0_10_A::SDA2)
    }
    #[doc = "MAT3.0"]
    #[inline(always)]
    pub fn mat3(self) -> &'a mut crate::W<REG> {
        self.variant(P0_10_A::MAT3)
    }
}
#[doc = "Field `P0_11` reader - Pin function select P0.11."]
pub type P0_11_R = crate::FieldReader<P0_11_A>;
#[doc = "Pin function select P0.11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_11_A {
    #[doc = "0: GPIO P0.11"]
    GPIO_P0 = 0,
    #[doc = "1: RXD2"]
    RXD2 = 1,
    #[doc = "2: SCL2"]
    SCL2 = 2,
    #[doc = "3: MAT3.1"]
    MAT3 = 3,
}
impl From<P0_11_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_11_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_11_A {
    type Ux = u8;
}
impl P0_11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_11_A {
        match self.bits {
            0 => P0_11_A::GPIO_P0,
            1 => P0_11_A::RXD2,
            2 => P0_11_A::SCL2,
            3 => P0_11_A::MAT3,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.11"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_11_A::GPIO_P0
    }
    #[doc = "RXD2"]
    #[inline(always)]
    pub fn is_rxd2(&self) -> bool {
        *self == P0_11_A::RXD2
    }
    #[doc = "SCL2"]
    #[inline(always)]
    pub fn is_scl2(&self) -> bool {
        *self == P0_11_A::SCL2
    }
    #[doc = "MAT3.1"]
    #[inline(always)]
    pub fn is_mat3(&self) -> bool {
        *self == P0_11_A::MAT3
    }
}
#[doc = "Field `P0_11` writer - Pin function select P0.11."]
pub type P0_11_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, P0_11_A>;
impl<'a, REG, const O: u8> P0_11_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.11"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(P0_11_A::GPIO_P0)
    }
    #[doc = "RXD2"]
    #[inline(always)]
    pub fn rxd2(self) -> &'a mut crate::W<REG> {
        self.variant(P0_11_A::RXD2)
    }
    #[doc = "SCL2"]
    #[inline(always)]
    pub fn scl2(self) -> &'a mut crate::W<REG> {
        self.variant(P0_11_A::SCL2)
    }
    #[doc = "MAT3.1"]
    #[inline(always)]
    pub fn mat3(self) -> &'a mut crate::W<REG> {
        self.variant(P0_11_A::MAT3)
    }
}
#[doc = "Field `P0_15` reader - Pin function select P0.15."]
pub type P0_15_R = crate::FieldReader<P0_15_A>;
#[doc = "Pin function select P0.15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P0_15_A {
    #[doc = "0: GPIO P0.15"]
    GPIO_P0 = 0,
    #[doc = "1: TXD1"]
    TXD1 = 1,
    #[doc = "2: SCK0"]
    SCK0 = 2,
    #[doc = "3: SCK"]
    SCK = 3,
}
impl From<P0_15_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_15_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P0_15_A {
    type Ux = u8;
}
impl P0_15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0_15_A {
        match self.bits {
            0 => P0_15_A::GPIO_P0,
            1 => P0_15_A::TXD1,
            2 => P0_15_A::SCK0,
            3 => P0_15_A::SCK,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P0.15"]
    #[inline(always)]
    pub fn is_gpio_p0(&self) -> bool {
        *self == P0_15_A::GPIO_P0
    }
    #[doc = "TXD1"]
    #[inline(always)]
    pub fn is_txd1(&self) -> bool {
        *self == P0_15_A::TXD1
    }
    #[doc = "SCK0"]
    #[inline(always)]
    pub fn is_sck0(&self) -> bool {
        *self == P0_15_A::SCK0
    }
    #[doc = "SCK"]
    #[inline(always)]
    pub fn is_sck(&self) -> bool {
        *self == P0_15_A::SCK
    }
}
#[doc = "Field `P0_15` writer - Pin function select P0.15."]
pub type P0_15_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, P0_15_A>;
impl<'a, REG, const O: u8> P0_15_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P0.15"]
    #[inline(always)]
    pub fn gpio_p0(self) -> &'a mut crate::W<REG> {
        self.variant(P0_15_A::GPIO_P0)
    }
    #[doc = "TXD1"]
    #[inline(always)]
    pub fn txd1(self) -> &'a mut crate::W<REG> {
        self.variant(P0_15_A::TXD1)
    }
    #[doc = "SCK0"]
    #[inline(always)]
    pub fn sck0(self) -> &'a mut crate::W<REG> {
        self.variant(P0_15_A::SCK0)
    }
    #[doc = "SCK"]
    #[inline(always)]
    pub fn sck(self) -> &'a mut crate::W<REG> {
        self.variant(P0_15_A::SCK)
    }
}
impl R {
    #[doc = "Bits 0:1 - Pin function select P0.0."]
    #[inline(always)]
    pub fn p0_00(&self) -> P0_00_R {
        P0_00_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Pin function select P0.1."]
    #[inline(always)]
    pub fn p0_01(&self) -> P0_01_R {
        P0_01_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Pin function select P0.2."]
    #[inline(always)]
    pub fn p0_02(&self) -> P0_02_R {
        P0_02_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Pin function select P0.3."]
    #[inline(always)]
    pub fn p0_03(&self) -> P0_03_R {
        P0_03_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin function select P0.4."]
    #[inline(always)]
    pub fn p0_04(&self) -> P0_04_R {
        P0_04_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Pin function select P0.5."]
    #[inline(always)]
    pub fn p0_05(&self) -> P0_05_R {
        P0_05_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Pin function select P0.6."]
    #[inline(always)]
    pub fn p0_06(&self) -> P0_06_R {
        P0_06_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Pin function select P0.7."]
    #[inline(always)]
    pub fn p0_07(&self) -> P0_07_R {
        P0_07_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Pin function select P0.8."]
    #[inline(always)]
    pub fn p0_08(&self) -> P0_08_R {
        P0_08_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Pin function select P0.9."]
    #[inline(always)]
    pub fn p0_09(&self) -> P0_09_R {
        P0_09_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Pin function select P0.10."]
    #[inline(always)]
    pub fn p0_10(&self) -> P0_10_R {
        P0_10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Pin function select P0.11."]
    #[inline(always)]
    pub fn p0_11(&self) -> P0_11_R {
        P0_11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Pin function select P0.15."]
    #[inline(always)]
    pub fn p0_15(&self) -> P0_15_R {
        P0_15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PINSEL0")
            .field("p0_00", &format_args!("{}", self.p0_00().bits()))
            .field("p0_01", &format_args!("{}", self.p0_01().bits()))
            .field("p0_02", &format_args!("{}", self.p0_02().bits()))
            .field("p0_03", &format_args!("{}", self.p0_03().bits()))
            .field("p0_04", &format_args!("{}", self.p0_04().bits()))
            .field("p0_05", &format_args!("{}", self.p0_05().bits()))
            .field("p0_06", &format_args!("{}", self.p0_06().bits()))
            .field("p0_07", &format_args!("{}", self.p0_07().bits()))
            .field("p0_08", &format_args!("{}", self.p0_08().bits()))
            .field("p0_09", &format_args!("{}", self.p0_09().bits()))
            .field("p0_10", &format_args!("{}", self.p0_10().bits()))
            .field("p0_11", &format_args!("{}", self.p0_11().bits()))
            .field("p0_15", &format_args!("{}", self.p0_15().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<PINSEL0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Pin function select P0.0."]
    #[inline(always)]
    #[must_use]
    pub fn p0_00(&mut self) -> P0_00_W<PINSEL0_SPEC, 0> {
        P0_00_W::new(self)
    }
    #[doc = "Bits 2:3 - Pin function select P0.1."]
    #[inline(always)]
    #[must_use]
    pub fn p0_01(&mut self) -> P0_01_W<PINSEL0_SPEC, 2> {
        P0_01_W::new(self)
    }
    #[doc = "Bits 4:5 - Pin function select P0.2."]
    #[inline(always)]
    #[must_use]
    pub fn p0_02(&mut self) -> P0_02_W<PINSEL0_SPEC, 4> {
        P0_02_W::new(self)
    }
    #[doc = "Bits 6:7 - Pin function select P0.3."]
    #[inline(always)]
    #[must_use]
    pub fn p0_03(&mut self) -> P0_03_W<PINSEL0_SPEC, 6> {
        P0_03_W::new(self)
    }
    #[doc = "Bits 8:9 - Pin function select P0.4."]
    #[inline(always)]
    #[must_use]
    pub fn p0_04(&mut self) -> P0_04_W<PINSEL0_SPEC, 8> {
        P0_04_W::new(self)
    }
    #[doc = "Bits 10:11 - Pin function select P0.5."]
    #[inline(always)]
    #[must_use]
    pub fn p0_05(&mut self) -> P0_05_W<PINSEL0_SPEC, 10> {
        P0_05_W::new(self)
    }
    #[doc = "Bits 12:13 - Pin function select P0.6."]
    #[inline(always)]
    #[must_use]
    pub fn p0_06(&mut self) -> P0_06_W<PINSEL0_SPEC, 12> {
        P0_06_W::new(self)
    }
    #[doc = "Bits 14:15 - Pin function select P0.7."]
    #[inline(always)]
    #[must_use]
    pub fn p0_07(&mut self) -> P0_07_W<PINSEL0_SPEC, 14> {
        P0_07_W::new(self)
    }
    #[doc = "Bits 16:17 - Pin function select P0.8."]
    #[inline(always)]
    #[must_use]
    pub fn p0_08(&mut self) -> P0_08_W<PINSEL0_SPEC, 16> {
        P0_08_W::new(self)
    }
    #[doc = "Bits 18:19 - Pin function select P0.9."]
    #[inline(always)]
    #[must_use]
    pub fn p0_09(&mut self) -> P0_09_W<PINSEL0_SPEC, 18> {
        P0_09_W::new(self)
    }
    #[doc = "Bits 20:21 - Pin function select P0.10."]
    #[inline(always)]
    #[must_use]
    pub fn p0_10(&mut self) -> P0_10_W<PINSEL0_SPEC, 20> {
        P0_10_W::new(self)
    }
    #[doc = "Bits 22:23 - Pin function select P0.11."]
    #[inline(always)]
    #[must_use]
    pub fn p0_11(&mut self) -> P0_11_W<PINSEL0_SPEC, 22> {
        P0_11_W::new(self)
    }
    #[doc = "Bits 30:31 - Pin function select P0.15."]
    #[inline(always)]
    #[must_use]
    pub fn p0_15(&mut self) -> P0_15_W<PINSEL0_SPEC, 30> {
        P0_15_W::new(self)
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
#[doc = "Pin function select register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinsel0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinsel0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PINSEL0_SPEC;
impl crate::RegisterSpec for PINSEL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinsel0::R`](R) reader structure"]
impl crate::Readable for PINSEL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pinsel0::W`](W) writer structure"]
impl crate::Writable for PINSEL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINSEL0 to value 0"]
impl crate::Resettable for PINSEL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
