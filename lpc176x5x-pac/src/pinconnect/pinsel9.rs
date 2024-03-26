#[doc = "Register `PINSEL9` reader"]
pub type R = crate::R<PINSEL9_SPEC>;
#[doc = "Register `PINSEL9` writer"]
pub type W = crate::W<PINSEL9_SPEC>;
#[doc = "Field `P4_28` reader - Pin function select P4.28."]
pub type P4_28_R = crate::FieldReader<P4_28_A>;
#[doc = "Pin function select P4.28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P4_28_A {
    #[doc = "0: GPIO P4.28"]
    GPIO_P4 = 0,
    #[doc = "1: RX_MCLK"]
    RX_MCLK = 1,
    #[doc = "2: MAT2.0"]
    MAT2 = 2,
    #[doc = "3: TXD3"]
    TXD3 = 3,
}
impl From<P4_28_A> for u8 {
    #[inline(always)]
    fn from(variant: P4_28_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P4_28_A {
    type Ux = u8;
}
impl P4_28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P4_28_A {
        match self.bits {
            0 => P4_28_A::GPIO_P4,
            1 => P4_28_A::RX_MCLK,
            2 => P4_28_A::MAT2,
            3 => P4_28_A::TXD3,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P4.28"]
    #[inline(always)]
    pub fn is_gpio_p4(&self) -> bool {
        *self == P4_28_A::GPIO_P4
    }
    #[doc = "RX_MCLK"]
    #[inline(always)]
    pub fn is_rx_mclk(&self) -> bool {
        *self == P4_28_A::RX_MCLK
    }
    #[doc = "MAT2.0"]
    #[inline(always)]
    pub fn is_mat2(&self) -> bool {
        *self == P4_28_A::MAT2
    }
    #[doc = "TXD3"]
    #[inline(always)]
    pub fn is_txd3(&self) -> bool {
        *self == P4_28_A::TXD3
    }
}
#[doc = "Field `P4_28` writer - Pin function select P4.28."]
pub type P4_28_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, P4_28_A>;
impl<'a, REG, const O: u8> P4_28_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P4.28"]
    #[inline(always)]
    pub fn gpio_p4(self) -> &'a mut crate::W<REG> {
        self.variant(P4_28_A::GPIO_P4)
    }
    #[doc = "RX_MCLK"]
    #[inline(always)]
    pub fn rx_mclk(self) -> &'a mut crate::W<REG> {
        self.variant(P4_28_A::RX_MCLK)
    }
    #[doc = "MAT2.0"]
    #[inline(always)]
    pub fn mat2(self) -> &'a mut crate::W<REG> {
        self.variant(P4_28_A::MAT2)
    }
    #[doc = "TXD3"]
    #[inline(always)]
    pub fn txd3(self) -> &'a mut crate::W<REG> {
        self.variant(P4_28_A::TXD3)
    }
}
#[doc = "Field `P4_29` reader - Pin function select P4.29."]
pub type P4_29_R = crate::FieldReader<P4_29_A>;
#[doc = "Pin function select P4.29.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P4_29_A {
    #[doc = "0: GPIO P4.29"]
    GPIO_P4 = 0,
    #[doc = "1: TX_MCLK"]
    TX_MCLK = 1,
    #[doc = "2: MAT2.1"]
    MAT2 = 2,
    #[doc = "3: RXD3"]
    RXD3 = 3,
}
impl From<P4_29_A> for u8 {
    #[inline(always)]
    fn from(variant: P4_29_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P4_29_A {
    type Ux = u8;
}
impl P4_29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P4_29_A {
        match self.bits {
            0 => P4_29_A::GPIO_P4,
            1 => P4_29_A::TX_MCLK,
            2 => P4_29_A::MAT2,
            3 => P4_29_A::RXD3,
            _ => unreachable!(),
        }
    }
    #[doc = "GPIO P4.29"]
    #[inline(always)]
    pub fn is_gpio_p4(&self) -> bool {
        *self == P4_29_A::GPIO_P4
    }
    #[doc = "TX_MCLK"]
    #[inline(always)]
    pub fn is_tx_mclk(&self) -> bool {
        *self == P4_29_A::TX_MCLK
    }
    #[doc = "MAT2.1"]
    #[inline(always)]
    pub fn is_mat2(&self) -> bool {
        *self == P4_29_A::MAT2
    }
    #[doc = "RXD3"]
    #[inline(always)]
    pub fn is_rxd3(&self) -> bool {
        *self == P4_29_A::RXD3
    }
}
#[doc = "Field `P4_29` writer - Pin function select P4.29."]
pub type P4_29_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, P4_29_A>;
impl<'a, REG, const O: u8> P4_29_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO P4.29"]
    #[inline(always)]
    pub fn gpio_p4(self) -> &'a mut crate::W<REG> {
        self.variant(P4_29_A::GPIO_P4)
    }
    #[doc = "TX_MCLK"]
    #[inline(always)]
    pub fn tx_mclk(self) -> &'a mut crate::W<REG> {
        self.variant(P4_29_A::TX_MCLK)
    }
    #[doc = "MAT2.1"]
    #[inline(always)]
    pub fn mat2(self) -> &'a mut crate::W<REG> {
        self.variant(P4_29_A::MAT2)
    }
    #[doc = "RXD3"]
    #[inline(always)]
    pub fn rxd3(self) -> &'a mut crate::W<REG> {
        self.variant(P4_29_A::RXD3)
    }
}
impl R {
    #[doc = "Bits 24:25 - Pin function select P4.28."]
    #[inline(always)]
    pub fn p4_28(&self) -> P4_28_R {
        P4_28_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Pin function select P4.29."]
    #[inline(always)]
    pub fn p4_29(&self) -> P4_29_R {
        P4_29_R::new(((self.bits >> 26) & 3) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PINSEL9")
            .field("p4_28", &format_args!("{}", self.p4_28().bits()))
            .field("p4_29", &format_args!("{}", self.p4_29().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<PINSEL9_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 24:25 - Pin function select P4.28."]
    #[inline(always)]
    #[must_use]
    pub fn p4_28(&mut self) -> P4_28_W<PINSEL9_SPEC, 24> {
        P4_28_W::new(self)
    }
    #[doc = "Bits 26:27 - Pin function select P4.29."]
    #[inline(always)]
    #[must_use]
    pub fn p4_29(&mut self) -> P4_29_W<PINSEL9_SPEC, 26> {
        P4_29_W::new(self)
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
#[doc = "Pin function select register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pinsel9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pinsel9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PINSEL9_SPEC;
impl crate::RegisterSpec for PINSEL9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pinsel9::R`](R) reader structure"]
impl crate::Readable for PINSEL9_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pinsel9::W`](W) writer structure"]
impl crate::Writable for PINSEL9_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINSEL9 to value 0"]
impl crate::Resettable for PINSEL9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
