#[doc = "Register `PINSEL2` reader"]
pub type R = crate::R<PINSEL2_SPEC>;
#[doc = "Register `PINSEL2` writer"]
pub type W = crate::W<PINSEL2_SPEC>;
#[doc = "Field `P1_00` reader - Pin function select P1.0."]
pub type P1_00_R = crate::FieldReader<P1_00_A>;
#[doc = "Pin function select P1.0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_00_A {
    #[doc = "0: GPIO P1.0"]
    GPIO_P1 = 0,
    #[doc = "1: ENET_TXD0"]
    ENET_TXD0 = 1,
}
impl From<P1_00_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_00_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_00_A {
    type Ux = u8;
}
impl P1_00_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_00_A {
        match self.bits {
            0 => P1_00_A::GPIO_P1,
            1 => P1_00_A::ENET_TXD0,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.0"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_00_A::GPIO_P1
    }
    #[doc = "ENET_TXD0"]
    #[inline(always)]
    pub fn is_enet_txd0(&self) -> bool {
        *self == P1_00_A::ENET_TXD0
    }
}
#[doc = "Field `P1_00` writer - Pin function select P1.0."]
pub type P1_00_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, P1_00_A>;
impl<'a, REG, const O: u8> P1_00_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.0"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_00_A::GPIO_P1)
    }
    #[doc = "ENET_TXD0"]
    #[inline(always)]
    pub fn enet_txd0(self) -> &'a mut crate::W<REG> {
        self.variant(P1_00_A::ENET_TXD0)
    }
}
#[doc = "Field `P1_01` reader - Pin function select P1.1."]
pub type P1_01_R = crate::FieldReader<P1_01_A>;
#[doc = "Pin function select P1.1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_01_A {
    #[doc = "0: GPIO P1.1"]
    GPIO_P1 = 0,
    #[doc = "1: ENET_TXD1"]
    ENET_TXD1 = 1,
}
impl From<P1_01_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_01_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_01_A {
    type Ux = u8;
}
impl P1_01_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_01_A {
        match self.bits {
            0 => P1_01_A::GPIO_P1,
            1 => P1_01_A::ENET_TXD1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.1"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_01_A::GPIO_P1
    }
    #[doc = "ENET_TXD1"]
    #[inline(always)]
    pub fn is_enet_txd1(&self) -> bool {
        *self == P1_01_A::ENET_TXD1
    }
}
#[doc = "Field `P1_01` writer - Pin function select P1.1."]
pub type P1_01_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, P1_01_A>;
impl<'a, REG, const O: u8> P1_01_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.1"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_01_A::GPIO_P1)
    }
    #[doc = "ENET_TXD1"]
    #[inline(always)]
    pub fn enet_txd1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_01_A::ENET_TXD1)
    }
}
#[doc = "Field `P1_04` reader - Pin function select P1.4."]
pub type P1_04_R = crate::FieldReader<P1_04_A>;
#[doc = "Pin function select P1.4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_04_A {
    #[doc = "0: GPIO P1.4."]
    GPIO_P1 = 0,
    #[doc = "1: ENET_TX_EN"]
    ENET_TX_EN = 1,
}
impl From<P1_04_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_04_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_04_A {
    type Ux = u8;
}
impl P1_04_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_04_A {
        match self.bits {
            0 => P1_04_A::GPIO_P1,
            1 => P1_04_A::ENET_TX_EN,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.4."]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_04_A::GPIO_P1
    }
    #[doc = "ENET_TX_EN"]
    #[inline(always)]
    pub fn is_enet_tx_en(&self) -> bool {
        *self == P1_04_A::ENET_TX_EN
    }
}
#[doc = "Field `P1_04` writer - Pin function select P1.4."]
pub type P1_04_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, P1_04_A>;
impl<'a, REG, const O: u8> P1_04_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.4."]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_04_A::GPIO_P1)
    }
    #[doc = "ENET_TX_EN"]
    #[inline(always)]
    pub fn enet_tx_en(self) -> &'a mut crate::W<REG> {
        self.variant(P1_04_A::ENET_TX_EN)
    }
}
#[doc = "Field `P1_08` reader - Pin function select P1.8."]
pub type P1_08_R = crate::FieldReader<P1_08_A>;
#[doc = "Pin function select P1.8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_08_A {
    #[doc = "0: GPIO P1.8."]
    GPIO_P1 = 0,
    #[doc = "1: ENET_CRS"]
    ENET_CRS = 1,
}
impl From<P1_08_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_08_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_08_A {
    type Ux = u8;
}
impl P1_08_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_08_A {
        match self.bits {
            0 => P1_08_A::GPIO_P1,
            1 => P1_08_A::ENET_CRS,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.8."]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_08_A::GPIO_P1
    }
    #[doc = "ENET_CRS"]
    #[inline(always)]
    pub fn is_enet_crs(&self) -> bool {
        *self == P1_08_A::ENET_CRS
    }
}
#[doc = "Field `P1_08` writer - Pin function select P1.8."]
pub type P1_08_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, P1_08_A>;
impl<'a, REG, const O: u8> P1_08_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.8."]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_08_A::GPIO_P1)
    }
    #[doc = "ENET_CRS"]
    #[inline(always)]
    pub fn enet_crs(self) -> &'a mut crate::W<REG> {
        self.variant(P1_08_A::ENET_CRS)
    }
}
#[doc = "Field `P1_09` reader - Pin function select P1.9."]
pub type P1_09_R = crate::FieldReader<P1_09_A>;
#[doc = "Pin function select P1.9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_09_A {
    #[doc = "0: GPIO Port 1.9"]
    GPIO_P1 = 0,
    #[doc = "1: ENET_RXD0"]
    ENET_RXD0 = 1,
}
impl From<P1_09_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_09_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_09_A {
    type Ux = u8;
}
impl P1_09_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_09_A {
        match self.bits {
            0 => P1_09_A::GPIO_P1,
            1 => P1_09_A::ENET_RXD0,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO Port 1.9"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_09_A::GPIO_P1
    }
    #[doc = "ENET_RXD0"]
    #[inline(always)]
    pub fn is_enet_rxd0(&self) -> bool {
        *self == P1_09_A::ENET_RXD0
    }
}
#[doc = "Field `P1_09` writer - Pin function select P1.9."]
pub type P1_09_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, P1_09_A>;
impl<'a, REG, const O: u8> P1_09_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO Port 1.9"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_09_A::GPIO_P1)
    }
    #[doc = "ENET_RXD0"]
    #[inline(always)]
    pub fn enet_rxd0(self) -> &'a mut crate::W<REG> {
        self.variant(P1_09_A::ENET_RXD0)
    }
}
#[doc = "Field `P1_10` reader - Pin function select P1.10."]
pub type P1_10_R = crate::FieldReader<P1_10_A>;
#[doc = "Pin function select P1.10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_10_A {
    #[doc = "0: GPIO P1.10"]
    GPIO_P1 = 0,
    #[doc = "1: ENET_RXD1"]
    ENET_RXD1 = 1,
}
impl From<P1_10_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_10_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_10_A {
    type Ux = u8;
}
impl P1_10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_10_A {
        match self.bits {
            0 => P1_10_A::GPIO_P1,
            1 => P1_10_A::ENET_RXD1,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.10"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_10_A::GPIO_P1
    }
    #[doc = "ENET_RXD1"]
    #[inline(always)]
    pub fn is_enet_rxd1(&self) -> bool {
        *self == P1_10_A::ENET_RXD1
    }
}
#[doc = "Field `P1_10` writer - Pin function select P1.10."]
pub type P1_10_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, P1_10_A>;
impl<'a, REG, const O: u8> P1_10_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.10"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_10_A::GPIO_P1)
    }
    #[doc = "ENET_RXD1"]
    #[inline(always)]
    pub fn enet_rxd1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_10_A::ENET_RXD1)
    }
}
#[doc = "Field `P1_14` reader - Pin function select P1.14."]
pub type P1_14_R = crate::FieldReader<P1_14_A>;
#[doc = "Pin function select P1.14.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_14_A {
    #[doc = "0: GPIO P1.14"]
    GPIO_P1 = 0,
    #[doc = "1: ENET_RX_ER"]
    ENET_RX_ER = 1,
}
impl From<P1_14_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_14_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_14_A {
    type Ux = u8;
}
impl P1_14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_14_A {
        match self.bits {
            0 => P1_14_A::GPIO_P1,
            1 => P1_14_A::ENET_RX_ER,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.14"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_14_A::GPIO_P1
    }
    #[doc = "ENET_RX_ER"]
    #[inline(always)]
    pub fn is_enet_rx_er(&self) -> bool {
        *self == P1_14_A::ENET_RX_ER
    }
}
#[doc = "Field `P1_14` writer - Pin function select P1.14."]
pub type P1_14_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, P1_14_A>;
impl<'a, REG, const O: u8> P1_14_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.14"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_14_A::GPIO_P1)
    }
    #[doc = "ENET_RX_ER"]
    #[inline(always)]
    pub fn enet_rx_er(self) -> &'a mut crate::W<REG> {
        self.variant(P1_14_A::ENET_RX_ER)
    }
}
#[doc = "Field `P1_15` reader - Pin function select P1.15."]
pub type P1_15_R = crate::FieldReader<P1_15_A>;
#[doc = "Pin function select P1.15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1_15_A {
    #[doc = "0: GPIO P1.15"]
    GPIO_P1 = 0,
    #[doc = "1: ENET_REF_CLK"]
    ENET_REF_CLK = 1,
}
impl From<P1_15_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_15_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P1_15_A {
    type Ux = u8;
}
impl P1_15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1_15_A {
        match self.bits {
            0 => P1_15_A::GPIO_P1,
            1 => P1_15_A::ENET_REF_CLK,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P1.15"]
    #[inline(always)]
    pub fn is_gpio_p1(&self) -> bool {
        *self == P1_15_A::GPIO_P1
    }
    #[doc = "ENET_REF_CLK"]
    #[inline(always)]
    pub fn is_enet_ref_clk(&self) -> bool {
        *self == P1_15_A::ENET_REF_CLK
    }
}
#[doc = "Field `P1_15` writer - Pin function select P1.15."]
pub type P1_15_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, P1_15_A>;
impl<'a, REG, const O: u8> P1_15_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P1.15"]
    #[inline(always)]
    pub fn gpio_p1(self) -> &'a mut crate::W<REG> {
        self.variant(P1_15_A::GPIO_P1)
    }
    #[doc = "ENET_REF_CLK"]
    #[inline(always)]
    pub fn enet_ref_clk(self) -> &'a mut crate::W<REG> {
        self.variant(P1_15_A::ENET_REF_CLK)
    }
}
impl R {
    #[doc = "Bits 0:1 - Pin function select P1.0."]
    #[inline(always)]
    pub fn p1_00(&self) -> P1_00_R {
        P1_00_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Pin function select P1.1."]
    #[inline(always)]
    pub fn p1_01(&self) -> P1_01_R {
        P1_01_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pin function select P1.4."]
    #[inline(always)]
    pub fn p1_04(&self) -> P1_04_R {
        P1_04_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Pin function select P1.8."]
    #[inline(always)]
    pub fn p1_08(&self) -> P1_08_R {
        P1_08_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Pin function select P1.9."]
    #[inline(always)]
    pub fn p1_09(&self) -> P1_09_R {
        P1_09_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Pin function select P1.10."]
    #[inline(always)]
    pub fn p1_10(&self) -> P1_10_R {
        P1_10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Pin function select P1.14."]
    #[inline(always)]
    pub fn p1_14(&self) -> P1_14_R {
        P1_14_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Pin function select P1.15."]
    #[inline(always)]
    pub fn p1_15(&self) -> P1_15_R {
        P1_15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PINSEL2")
            .field("p1_00", &format_args!("{}", self.p1_00().bits()))
            .field("p1_01", &format_args!("{}", self.p1_01().bits()))
            .field("p1_04", &format_args!("{}", self.p1_04().bits()))
            .field("p1_08", &format_args!("{}", self.p1_08().bits()))
            .field("p1_09", &format_args!("{}", self.p1_09().bits()))
            .field("p1_10", &format_args!("{}", self.p1_10().bits()))
            .field("p1_14", &format_args!("{}", self.p1_14().bits()))
            .field("p1_15", &format_args!("{}", self.p1_15().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<PINSEL2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Pin function select P1.0."]
    #[inline(always)]
    #[must_use]
    pub fn p1_00(&mut self) -> P1_00_W<PINSEL2_SPEC, 0> {
        P1_00_W::new(self)
    }
    #[doc = "Bits 2:3 - Pin function select P1.1."]
    #[inline(always)]
    #[must_use]
    pub fn p1_01(&mut self) -> P1_01_W<PINSEL2_SPEC, 2> {
        P1_01_W::new(self)
    }
    #[doc = "Bits 8:9 - Pin function select P1.4."]
    #[inline(always)]
    #[must_use]
    pub fn p1_04(&mut self) -> P1_04_W<PINSEL2_SPEC, 8> {
        P1_04_W::new(self)
    }
    #[doc = "Bits 16:17 - Pin function select P1.8."]
    #[inline(always)]
    #[must_use]
    pub fn p1_08(&mut self) -> P1_08_W<PINSEL2_SPEC, 16> {
        P1_08_W::new(self)
    }
    #[doc = "Bits 18:19 - Pin function select P1.9."]
    #[inline(always)]
    #[must_use]
    pub fn p1_09(&mut self) -> P1_09_W<PINSEL2_SPEC, 18> {
        P1_09_W::new(self)
    }
    #[doc = "Bits 20:21 - Pin function select P1.10."]
    #[inline(always)]
    #[must_use]
    pub fn p1_10(&mut self) -> P1_10_W<PINSEL2_SPEC, 20> {
        P1_10_W::new(self)
    }
    #[doc = "Bits 22:23 - Pin function select P1.14."]
    #[inline(always)]
    #[must_use]
    pub fn p1_14(&mut self) -> P1_14_W<PINSEL2_SPEC, 22> {
        P1_14_W::new(self)
    }
    #[doc = "Bits 30:31 - Pin function select P1.15."]
    #[inline(always)]
    #[must_use]
    pub fn p1_15(&mut self) -> P1_15_W<PINSEL2_SPEC, 30> {
        P1_15_W::new(self)
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
#[doc = "Pin function select register 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinsel2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinsel2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PINSEL2_SPEC;
impl crate::RegisterSpec for PINSEL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinsel2::R`](R) reader structure"]
impl crate::Readable for PINSEL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pinsel2::W`](W) writer structure"]
impl crate::Writable for PINSEL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINSEL2 to value 0"]
impl crate::Resettable for PINSEL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
