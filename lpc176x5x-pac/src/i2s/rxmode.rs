#[doc = "Register `RXMODE` reader"]
pub type R = crate::R<RXMODE_SPEC>;
#[doc = "Register `RXMODE` writer"]
pub type W = crate::W<RXMODE_SPEC>;
#[doc = "Field `RXCLKSEL` reader - Clock source selection for the receive bit clock divider."]
pub type RXCLKSEL_R = crate::FieldReader<RXCLKSEL_A>;
#[doc = "Clock source selection for the receive bit clock divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXCLKSEL_A {
    #[doc = "0: Select the RX fractional rate divider clock output as the source"]
    RX_DIVIDER = 0,
    #[doc = "2: Select the TX_MCLK signal as the RX_MCLK clock source"]
    TX_MCLK = 2,
}
impl From<RXCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RXCLKSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RXCLKSEL_A {
    type Ux = u8;
}
impl RXCLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RXCLKSEL_A> {
        match self.bits {
            0 => Some(RXCLKSEL_A::RX_DIVIDER),
            2 => Some(RXCLKSEL_A::TX_MCLK),
            _ => None,
        }
    }
    #[doc = "Select the RX fractional rate divider clock output as the source"]
    #[inline(always)]
    pub fn is_rx_divider(&self) -> bool {
        *self == RXCLKSEL_A::RX_DIVIDER
    }
    #[doc = "Select the TX_MCLK signal as the RX_MCLK clock source"]
    #[inline(always)]
    pub fn is_tx_mclk(&self) -> bool {
        *self == RXCLKSEL_A::TX_MCLK
    }
}
#[doc = "Field `RXCLKSEL` writer - Clock source selection for the receive bit clock divider."]
pub type RXCLKSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, RXCLKSEL_A>;
impl<'a, REG, const O: u8> RXCLKSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select the RX fractional rate divider clock output as the source"]
    #[inline(always)]
    pub fn rx_divider(self) -> &'a mut crate::W<REG> {
        self.variant(RXCLKSEL_A::RX_DIVIDER)
    }
    #[doc = "Select the TX_MCLK signal as the RX_MCLK clock source"]
    #[inline(always)]
    pub fn tx_mclk(self) -> &'a mut crate::W<REG> {
        self.variant(RXCLKSEL_A::TX_MCLK)
    }
}
#[doc = "Field `RX4PIN` reader - Receive 4-pin mode selection. When 1, enables 4-pin mode."]
pub type RX4PIN_R = crate::BitReader;
#[doc = "Field `RX4PIN` writer - Receive 4-pin mode selection. When 1, enables 4-pin mode."]
pub type RX4PIN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXMCENA` reader - Enable for the RX_MCLK output. When 0, output of RX_MCLK is not enabled. When 1, output of RX_MCLK is enabled."]
pub type RXMCENA_R = crate::BitReader;
#[doc = "Field `RXMCENA` writer - Enable for the RX_MCLK output. When 0, output of RX_MCLK is not enabled. When 1, output of RX_MCLK is enabled."]
pub type RXMCENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - Clock source selection for the receive bit clock divider."]
    #[inline(always)]
    pub fn rxclksel(&self) -> RXCLKSEL_R {
        RXCLKSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Receive 4-pin mode selection. When 1, enables 4-pin mode."]
    #[inline(always)]
    pub fn rx4pin(&self) -> RX4PIN_R {
        RX4PIN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable for the RX_MCLK output. When 0, output of RX_MCLK is not enabled. When 1, output of RX_MCLK is enabled."]
    #[inline(always)]
    pub fn rxmcena(&self) -> RXMCENA_R {
        RXMCENA_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXMODE")
            .field("rxclksel", &format_args!("{}", self.rxclksel().bits()))
            .field("rx4pin", &format_args!("{}", self.rx4pin().bit()))
            .field("rxmcena", &format_args!("{}", self.rxmcena().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<RXMODE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock source selection for the receive bit clock divider."]
    #[inline(always)]
    #[must_use]
    pub fn rxclksel(&mut self) -> RXCLKSEL_W<RXMODE_SPEC, 0> {
        RXCLKSEL_W::new(self)
    }
    #[doc = "Bit 2 - Receive 4-pin mode selection. When 1, enables 4-pin mode."]
    #[inline(always)]
    #[must_use]
    pub fn rx4pin(&mut self) -> RX4PIN_W<RXMODE_SPEC, 2> {
        RX4PIN_W::new(self)
    }
    #[doc = "Bit 3 - Enable for the RX_MCLK output. When 0, output of RX_MCLK is not enabled. When 1, output of RX_MCLK is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn rxmcena(&mut self) -> RXMCENA_W<RXMODE_SPEC, 3> {
        RXMCENA_W::new(self)
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
#[doc = "I2S Receive mode control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxmode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxmode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXMODE_SPEC;
impl crate::RegisterSpec for RXMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxmode::R`](R) reader structure"]
impl crate::Readable for RXMODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxmode::W`](W) writer structure"]
impl crate::Writable for RXMODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXMODE to value 0"]
impl crate::Resettable for RXMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
