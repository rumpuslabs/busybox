#[doc = "Register `TXMODE` reader"]
pub type R = crate::R<TXMODE_SPEC>;
#[doc = "Register `TXMODE` writer"]
pub type W = crate::W<TXMODE_SPEC>;
#[doc = "Field `TXCLKSEL` reader - Clock source selection for the transmit bit clock divider."]
pub type TXCLKSEL_R = crate::FieldReader<TXCLKSEL_A>;
#[doc = "Clock source selection for the transmit bit clock divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXCLKSEL_A {
    #[doc = "0: Select the TX fractional rate divider clock output as the source"]
    TX_DIVIDER = 0,
    #[doc = "2: Select the RX_MCLK signal as the TX_MCLK clock source"]
    RX_MCLK = 2,
}
impl From<TXCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TXCLKSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TXCLKSEL_A {
    type Ux = u8;
}
impl TXCLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TXCLKSEL_A> {
        match self.bits {
            0 => Some(TXCLKSEL_A::TX_DIVIDER),
            2 => Some(TXCLKSEL_A::RX_MCLK),
            _ => None,
        }
    }
    #[doc = "Select the TX fractional rate divider clock output as the source"]
    #[inline(always)]
    pub fn is_tx_divider(&self) -> bool {
        *self == TXCLKSEL_A::TX_DIVIDER
    }
    #[doc = "Select the RX_MCLK signal as the TX_MCLK clock source"]
    #[inline(always)]
    pub fn is_rx_mclk(&self) -> bool {
        *self == TXCLKSEL_A::RX_MCLK
    }
}
#[doc = "Field `TXCLKSEL` writer - Clock source selection for the transmit bit clock divider."]
pub type TXCLKSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, TXCLKSEL_A>;
impl<'a, REG, const O: u8> TXCLKSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select the TX fractional rate divider clock output as the source"]
    #[inline(always)]
    pub fn tx_divider(self) -> &'a mut crate::W<REG> {
        self.variant(TXCLKSEL_A::TX_DIVIDER)
    }
    #[doc = "Select the RX_MCLK signal as the TX_MCLK clock source"]
    #[inline(always)]
    pub fn rx_mclk(self) -> &'a mut crate::W<REG> {
        self.variant(TXCLKSEL_A::RX_MCLK)
    }
}
#[doc = "Field `TX4PIN` reader - Transmit 4-pin mode selection. When 1, enables 4-pin mode."]
pub type TX4PIN_R = crate::BitReader;
#[doc = "Field `TX4PIN` writer - Transmit 4-pin mode selection. When 1, enables 4-pin mode."]
pub type TX4PIN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXMCENA` reader - Enable for the TX_MCLK output. When 0, output of TX_MCLK is not enabled. When 1, output of TX_MCLK is enabled."]
pub type TXMCENA_R = crate::BitReader;
#[doc = "Field `TXMCENA` writer - Enable for the TX_MCLK output. When 0, output of TX_MCLK is not enabled. When 1, output of TX_MCLK is enabled."]
pub type TXMCENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - Clock source selection for the transmit bit clock divider."]
    #[inline(always)]
    pub fn txclksel(&self) -> TXCLKSEL_R {
        TXCLKSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Transmit 4-pin mode selection. When 1, enables 4-pin mode."]
    #[inline(always)]
    pub fn tx4pin(&self) -> TX4PIN_R {
        TX4PIN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable for the TX_MCLK output. When 0, output of TX_MCLK is not enabled. When 1, output of TX_MCLK is enabled."]
    #[inline(always)]
    pub fn txmcena(&self) -> TXMCENA_R {
        TXMCENA_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXMODE")
            .field("txclksel", &format_args!("{}", self.txclksel().bits()))
            .field("tx4pin", &format_args!("{}", self.tx4pin().bit()))
            .field("txmcena", &format_args!("{}", self.txmcena().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<TXMODE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock source selection for the transmit bit clock divider."]
    #[inline(always)]
    #[must_use]
    pub fn txclksel(&mut self) -> TXCLKSEL_W<TXMODE_SPEC, 0> {
        TXCLKSEL_W::new(self)
    }
    #[doc = "Bit 2 - Transmit 4-pin mode selection. When 1, enables 4-pin mode."]
    #[inline(always)]
    #[must_use]
    pub fn tx4pin(&mut self) -> TX4PIN_W<TXMODE_SPEC, 2> {
        TX4PIN_W::new(self)
    }
    #[doc = "Bit 3 - Enable for the TX_MCLK output. When 0, output of TX_MCLK is not enabled. When 1, output of TX_MCLK is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn txmcena(&mut self) -> TXMCENA_W<TXMODE_SPEC, 3> {
        TXMCENA_W::new(self)
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
#[doc = "I2S Transmit mode control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txmode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txmode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXMODE_SPEC;
impl crate::RegisterSpec for TXMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txmode::R`](R) reader structure"]
impl crate::Readable for TXMODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txmode::W`](W) writer structure"]
impl crate::Writable for TXMODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXMODE to value 0"]
impl crate::Resettable for TXMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
