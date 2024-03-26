#[doc = "Register `FCR` writer"]
pub type W = crate::W<FCR_SPEC>;
#[doc = "FIFO Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOEN_AW {
    #[doc = "0: UARTn FIFOs are disabled. Must not be used in the application."]
    MUST_NOT_BE_USED = 0,
    #[doc = "1: Active high enable for both UARTn Rx and TX FIFOs and UnFCR\\[7:1\\]
access. This bit must be set for proper UART operation. Any transition on this bit will automatically clear the related UART FIFOs."]
    ACTIVE_HIGH_ENABLE = 1,
}
impl From<FIFOEN_AW> for bool {
    #[inline(always)]
    fn from(variant: FIFOEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOEN` writer - FIFO Enable."]
pub type FIFOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FIFOEN_AW>;
impl<'a, REG, const O: u8> FIFOEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "UARTn FIFOs are disabled. Must not be used in the application."]
    #[inline(always)]
    pub fn must_not_be_used(self) -> &'a mut crate::W<REG> {
        self.variant(FIFOEN_AW::MUST_NOT_BE_USED)
    }
    #[doc = "Active high enable for both UARTn Rx and TX FIFOs and UnFCR\\[7:1\\]
access. This bit must be set for proper UART operation. Any transition on this bit will automatically clear the related UART FIFOs."]
    #[inline(always)]
    pub fn active_high_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FIFOEN_AW::ACTIVE_HIGH_ENABLE)
    }
}
#[doc = "RX FIFO Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFIFORES_AW {
    #[doc = "0: No impact on either of UARTn FIFOs."]
    NO_ACTION = 0,
    #[doc = "1: Writing a logic 1 to UnFCR\\[1\\]
will clear all bytes in UARTn Rx FIFO, reset the pointer logic. This bit is self-clearing."]
    RESET = 1,
}
impl From<RXFIFORES_AW> for bool {
    #[inline(always)]
    fn from(variant: RXFIFORES_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFIFORES` writer - RX FIFO Reset."]
pub type RXFIFORES_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RXFIFORES_AW>;
impl<'a, REG, const O: u8> RXFIFORES_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No impact on either of UARTn FIFOs."]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(RXFIFORES_AW::NO_ACTION)
    }
    #[doc = "Writing a logic 1 to UnFCR\\[1\\]
will clear all bytes in UARTn Rx FIFO, reset the pointer logic. This bit is self-clearing."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(RXFIFORES_AW::RESET)
    }
}
#[doc = "TX FIFO Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFIFORES_AW {
    #[doc = "0: No impact on either of UARTn FIFOs."]
    NO_ACTION = 0,
    #[doc = "1: Writing a logic 1 to UnFCR\\[2\\]
will clear all bytes in UARTn TX FIFO, reset the pointer logic. This bit is self-clearing."]
    RESET = 1,
}
impl From<TXFIFORES_AW> for bool {
    #[inline(always)]
    fn from(variant: TXFIFORES_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFIFORES` writer - TX FIFO Reset."]
pub type TXFIFORES_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TXFIFORES_AW>;
impl<'a, REG, const O: u8> TXFIFORES_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No impact on either of UARTn FIFOs."]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(TXFIFORES_AW::NO_ACTION)
    }
    #[doc = "Writing a logic 1 to UnFCR\\[2\\]
will clear all bytes in UARTn TX FIFO, reset the pointer logic. This bit is self-clearing."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(TXFIFORES_AW::RESET)
    }
}
#[doc = "Field `DMAMODE` writer - DMA Mode Select. When the FIFO enable (bit 0 of this register) is set, this bit selects the DMA mode. See Section 18.6.6.1."]
pub type DMAMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "RX Trigger Level. These two bits determine how many receiver UARTn FIFO characters must be written before an interrupt or DMA request is activated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXTRIGLVL_AW {
    #[doc = "0: Trigger level 0 (1 character or 0x01)."]
    _1_CHARACTER = 0,
    #[doc = "1: Trigger level 1 (4 characters or 0x04)."]
    _4_CHARACTERS = 1,
    #[doc = "2: Trigger level 2 (8 characters or 0x08)."]
    _8_CHARACTERS = 2,
    #[doc = "3: Trigger level 3 (14 characters or 0x0E)."]
    _14_CHARACTERS = 3,
}
impl From<RXTRIGLVL_AW> for u8 {
    #[inline(always)]
    fn from(variant: RXTRIGLVL_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RXTRIGLVL_AW {
    type Ux = u8;
}
#[doc = "Field `RXTRIGLVL` writer - RX Trigger Level. These two bits determine how many receiver UARTn FIFO characters must be written before an interrupt or DMA request is activated."]
pub type RXTRIGLVL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, RXTRIGLVL_AW>;
impl<'a, REG, const O: u8> RXTRIGLVL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trigger level 0 (1 character or 0x01)."]
    #[inline(always)]
    pub fn _1_character(self) -> &'a mut crate::W<REG> {
        self.variant(RXTRIGLVL_AW::_1_CHARACTER)
    }
    #[doc = "Trigger level 1 (4 characters or 0x04)."]
    #[inline(always)]
    pub fn _4_characters(self) -> &'a mut crate::W<REG> {
        self.variant(RXTRIGLVL_AW::_4_CHARACTERS)
    }
    #[doc = "Trigger level 2 (8 characters or 0x08)."]
    #[inline(always)]
    pub fn _8_characters(self) -> &'a mut crate::W<REG> {
        self.variant(RXTRIGLVL_AW::_8_CHARACTERS)
    }
    #[doc = "Trigger level 3 (14 characters or 0x0E)."]
    #[inline(always)]
    pub fn _14_characters(self) -> &'a mut crate::W<REG> {
        self.variant(RXTRIGLVL_AW::_14_CHARACTERS)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<FCR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - FIFO Enable."]
    #[inline(always)]
    #[must_use]
    pub fn fifoen(&mut self) -> FIFOEN_W<FCR_SPEC, 0> {
        FIFOEN_W::new(self)
    }
    #[doc = "Bit 1 - RX FIFO Reset."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifores(&mut self) -> RXFIFORES_W<FCR_SPEC, 1> {
        RXFIFORES_W::new(self)
    }
    #[doc = "Bit 2 - TX FIFO Reset."]
    #[inline(always)]
    #[must_use]
    pub fn txfifores(&mut self) -> TXFIFORES_W<FCR_SPEC, 2> {
        TXFIFORES_W::new(self)
    }
    #[doc = "Bit 3 - DMA Mode Select. When the FIFO enable (bit 0 of this register) is set, this bit selects the DMA mode. See Section 18.6.6.1."]
    #[inline(always)]
    #[must_use]
    pub fn dmamode(&mut self) -> DMAMODE_W<FCR_SPEC, 3> {
        DMAMODE_W::new(self)
    }
    #[doc = "Bits 6:7 - RX Trigger Level. These two bits determine how many receiver UARTn FIFO characters must be written before an interrupt or DMA request is activated."]
    #[inline(always)]
    #[must_use]
    pub fn rxtriglvl(&mut self) -> RXTRIGLVL_W<FCR_SPEC, 6> {
        RXTRIGLVL_W::new(self)
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
#[doc = "FIFO Control Register. Controls UART FIFO usage and modes.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCR_SPEC;
impl crate::RegisterSpec for FCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fcr::W`](W) writer structure"]
impl crate::Writable for FCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCR to value 0"]
impl crate::Resettable for FCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
