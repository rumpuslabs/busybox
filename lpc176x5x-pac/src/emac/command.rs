#[doc = "Register `COMMAND` reader"]
pub type R = crate::R<COMMAND_SPEC>;
#[doc = "Register `COMMAND` writer"]
pub type W = crate::W<COMMAND_SPEC>;
#[doc = "Field `RXENABLE` reader - Enable receive."]
pub type RXENABLE_R = crate::BitReader;
#[doc = "Field `RXENABLE` writer - Enable receive."]
pub type RXENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXENABLE` reader - Enable transmit."]
pub type TXENABLE_R = crate::BitReader;
#[doc = "Field `TXENABLE` writer - Enable transmit."]
pub type TXENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `REGRESET` reader - When a 1 is written, all datapaths and the host registers are reset. The MAC needs to be reset separately."]
pub type REGRESET_R = crate::BitReader;
#[doc = "Field `REGRESET` writer - When a 1 is written, all datapaths and the host registers are reset. The MAC needs to be reset separately."]
pub type REGRESET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXRESET` reader - When a 1 is written, the transmit datapath is reset."]
pub type TXRESET_R = crate::BitReader;
#[doc = "Field `TXRESET` writer - When a 1 is written, the transmit datapath is reset."]
pub type TXRESET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXRESET` reader - When a 1 is written, the receive datapath is reset."]
pub type RXRESET_R = crate::BitReader;
#[doc = "Field `RXRESET` writer - When a 1 is written, the receive datapath is reset."]
pub type RXRESET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PASSRUNTFRAME` reader - When set to 1 , passes runt frames s1maller than 64 bytes to memory unless they have a CRC error. If 0 runt frames are filtered out."]
pub type PASSRUNTFRAME_R = crate::BitReader;
#[doc = "Field `PASSRUNTFRAME` writer - When set to 1 , passes runt frames s1maller than 64 bytes to memory unless they have a CRC error. If 0 runt frames are filtered out."]
pub type PASSRUNTFRAME_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PASSRXFILTER` reader - When set to 1 , disables receive filtering i.e. all frames received are written to memory."]
pub type PASSRXFILTER_R = crate::BitReader;
#[doc = "Field `PASSRXFILTER` writer - When set to 1 , disables receive filtering i.e. all frames received are written to memory."]
pub type PASSRXFILTER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXFLOWCONTROL` reader - Enable IEEE 802.3 / clause 31 flow control sending pause frames in full duplex and continuous preamble in half duplex."]
pub type TXFLOWCONTROL_R = crate::BitReader;
#[doc = "Field `TXFLOWCONTROL` writer - Enable IEEE 802.3 / clause 31 flow control sending pause frames in full duplex and continuous preamble in half duplex."]
pub type TXFLOWCONTROL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RMII` reader - When set to 1 , RMII mode is selected; if 0, MII mode is selected."]
pub type RMII_R = crate::BitReader;
#[doc = "Field `RMII` writer - When set to 1 , RMII mode is selected; if 0, MII mode is selected."]
pub type RMII_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FULLDUPLEX` reader - When set to 1 , indicates full duplex operation."]
pub type FULLDUPLEX_R = crate::BitReader;
#[doc = "Field `FULLDUPLEX` writer - When set to 1 , indicates full duplex operation."]
pub type FULLDUPLEX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Enable receive."]
    #[inline(always)]
    pub fn rxenable(&self) -> RXENABLE_R {
        RXENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable transmit."]
    #[inline(always)]
    pub fn txenable(&self) -> TXENABLE_R {
        TXENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - When a 1 is written, all datapaths and the host registers are reset. The MAC needs to be reset separately."]
    #[inline(always)]
    pub fn regreset(&self) -> REGRESET_R {
        REGRESET_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When a 1 is written, the transmit datapath is reset."]
    #[inline(always)]
    pub fn txreset(&self) -> TXRESET_R {
        TXRESET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When a 1 is written, the receive datapath is reset."]
    #[inline(always)]
    pub fn rxreset(&self) -> RXRESET_R {
        RXRESET_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When set to 1 , passes runt frames s1maller than 64 bytes to memory unless they have a CRC error. If 0 runt frames are filtered out."]
    #[inline(always)]
    pub fn passruntframe(&self) -> PASSRUNTFRAME_R {
        PASSRUNTFRAME_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When set to 1 , disables receive filtering i.e. all frames received are written to memory."]
    #[inline(always)]
    pub fn passrxfilter(&self) -> PASSRXFILTER_R {
        PASSRXFILTER_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable IEEE 802.3 / clause 31 flow control sending pause frames in full duplex and continuous preamble in half duplex."]
    #[inline(always)]
    pub fn txflowcontrol(&self) -> TXFLOWCONTROL_R {
        TXFLOWCONTROL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When set to 1 , RMII mode is selected; if 0, MII mode is selected."]
    #[inline(always)]
    pub fn rmii(&self) -> RMII_R {
        RMII_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - When set to 1 , indicates full duplex operation."]
    #[inline(always)]
    pub fn fullduplex(&self) -> FULLDUPLEX_R {
        FULLDUPLEX_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMMAND")
            .field("rxenable", &format_args!("{}", self.rxenable().bit()))
            .field("txenable", &format_args!("{}", self.txenable().bit()))
            .field("regreset", &format_args!("{}", self.regreset().bit()))
            .field("txreset", &format_args!("{}", self.txreset().bit()))
            .field("rxreset", &format_args!("{}", self.rxreset().bit()))
            .field(
                "passruntframe",
                &format_args!("{}", self.passruntframe().bit()),
            )
            .field(
                "passrxfilter",
                &format_args!("{}", self.passrxfilter().bit()),
            )
            .field(
                "txflowcontrol",
                &format_args!("{}", self.txflowcontrol().bit()),
            )
            .field("rmii", &format_args!("{}", self.rmii().bit()))
            .field("fullduplex", &format_args!("{}", self.fullduplex().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<COMMAND_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Enable receive."]
    #[inline(always)]
    #[must_use]
    pub fn rxenable(&mut self) -> RXENABLE_W<COMMAND_SPEC, 0> {
        RXENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Enable transmit."]
    #[inline(always)]
    #[must_use]
    pub fn txenable(&mut self) -> TXENABLE_W<COMMAND_SPEC, 1> {
        TXENABLE_W::new(self)
    }
    #[doc = "Bit 3 - When a 1 is written, all datapaths and the host registers are reset. The MAC needs to be reset separately."]
    #[inline(always)]
    #[must_use]
    pub fn regreset(&mut self) -> REGRESET_W<COMMAND_SPEC, 3> {
        REGRESET_W::new(self)
    }
    #[doc = "Bit 4 - When a 1 is written, the transmit datapath is reset."]
    #[inline(always)]
    #[must_use]
    pub fn txreset(&mut self) -> TXRESET_W<COMMAND_SPEC, 4> {
        TXRESET_W::new(self)
    }
    #[doc = "Bit 5 - When a 1 is written, the receive datapath is reset."]
    #[inline(always)]
    #[must_use]
    pub fn rxreset(&mut self) -> RXRESET_W<COMMAND_SPEC, 5> {
        RXRESET_W::new(self)
    }
    #[doc = "Bit 6 - When set to 1 , passes runt frames s1maller than 64 bytes to memory unless they have a CRC error. If 0 runt frames are filtered out."]
    #[inline(always)]
    #[must_use]
    pub fn passruntframe(&mut self) -> PASSRUNTFRAME_W<COMMAND_SPEC, 6> {
        PASSRUNTFRAME_W::new(self)
    }
    #[doc = "Bit 7 - When set to 1 , disables receive filtering i.e. all frames received are written to memory."]
    #[inline(always)]
    #[must_use]
    pub fn passrxfilter(&mut self) -> PASSRXFILTER_W<COMMAND_SPEC, 7> {
        PASSRXFILTER_W::new(self)
    }
    #[doc = "Bit 8 - Enable IEEE 802.3 / clause 31 flow control sending pause frames in full duplex and continuous preamble in half duplex."]
    #[inline(always)]
    #[must_use]
    pub fn txflowcontrol(&mut self) -> TXFLOWCONTROL_W<COMMAND_SPEC, 8> {
        TXFLOWCONTROL_W::new(self)
    }
    #[doc = "Bit 9 - When set to 1 , RMII mode is selected; if 0, MII mode is selected."]
    #[inline(always)]
    #[must_use]
    pub fn rmii(&mut self) -> RMII_W<COMMAND_SPEC, 9> {
        RMII_W::new(self)
    }
    #[doc = "Bit 10 - When set to 1 , indicates full duplex operation."]
    #[inline(always)]
    #[must_use]
    pub fn fullduplex(&mut self) -> FULLDUPLEX_W<COMMAND_SPEC, 10> {
        FULLDUPLEX_W::new(self)
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
#[doc = "Command register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`command::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`command::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMMAND_SPEC;
impl crate::RegisterSpec for COMMAND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`command::R`](R) reader structure"]
impl crate::Readable for COMMAND_SPEC {}
#[doc = "`write(|w| ..)` method takes [`command::W`](W) writer structure"]
impl crate::Writable for COMMAND_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMMAND to value 0"]
impl crate::Resettable for COMMAND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
