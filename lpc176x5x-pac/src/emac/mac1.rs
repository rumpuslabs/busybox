#[doc = "Register `MAC1` reader"]
pub type R = crate::R<MAC1_SPEC>;
#[doc = "Register `MAC1` writer"]
pub type W = crate::W<MAC1_SPEC>;
#[doc = "Field `RXENABLE` reader - RECEIVE ENABLE. Set this to allow receive frames to be received. Internally the MAC synchronizes this control bit to the incoming receive stream."]
pub type RXENABLE_R = crate::BitReader;
#[doc = "Field `RXENABLE` writer - RECEIVE ENABLE. Set this to allow receive frames to be received. Internally the MAC synchronizes this control bit to the incoming receive stream."]
pub type RXENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PARF` reader - PASS ALL RECEIVE FRAMES. When enabled (set to 1), the MAC will pass all frames regardless of type (normal vs. Control). When disabled, the MAC does not pass valid Control frames."]
pub type PARF_R = crate::BitReader;
#[doc = "Field `PARF` writer - PASS ALL RECEIVE FRAMES. When enabled (set to 1), the MAC will pass all frames regardless of type (normal vs. Control). When disabled, the MAC does not pass valid Control frames."]
pub type PARF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXFLOWCTRL` reader - RX FLOW CONTROL. When enabled (set to 1), the MAC acts upon received PAUSE Flow Control frames. When disabled, received PAUSE Flow Control frames are ignored."]
pub type RXFLOWCTRL_R = crate::BitReader;
#[doc = "Field `RXFLOWCTRL` writer - RX FLOW CONTROL. When enabled (set to 1), the MAC acts upon received PAUSE Flow Control frames. When disabled, received PAUSE Flow Control frames are ignored."]
pub type RXFLOWCTRL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXFLOWCTRL` reader - TX FLOW CONTROL. When enabled (set to 1), PAUSE Flow Control frames are allowed to be transmitted. When disabled, Flow Control frames are blocked."]
pub type TXFLOWCTRL_R = crate::BitReader;
#[doc = "Field `TXFLOWCTRL` writer - TX FLOW CONTROL. When enabled (set to 1), PAUSE Flow Control frames are allowed to be transmitted. When disabled, Flow Control frames are blocked."]
pub type TXFLOWCTRL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LOOPBACK` reader - Setting this bit will cause the MAC Transmit interface to be looped back to the MAC Receive interface. Clearing this bit results in normal operation."]
pub type LOOPBACK_R = crate::BitReader;
#[doc = "Field `LOOPBACK` writer - Setting this bit will cause the MAC Transmit interface to be looped back to the MAC Receive interface. Clearing this bit results in normal operation."]
pub type LOOPBACK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESETTX` reader - Setting this bit will put the Transmit Function logic in reset."]
pub type RESETTX_R = crate::BitReader;
#[doc = "Field `RESETTX` writer - Setting this bit will put the Transmit Function logic in reset."]
pub type RESETTX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESETMCSTX` reader - Setting this bit resets the MAC Control Sublayer / Transmit logic. The MCS logic implements flow control."]
pub type RESETMCSTX_R = crate::BitReader;
#[doc = "Field `RESETMCSTX` writer - Setting this bit resets the MAC Control Sublayer / Transmit logic. The MCS logic implements flow control."]
pub type RESETMCSTX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESETRX` reader - Setting this bit will put the Ethernet receive logic in reset."]
pub type RESETRX_R = crate::BitReader;
#[doc = "Field `RESETRX` writer - Setting this bit will put the Ethernet receive logic in reset."]
pub type RESETRX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESETMCSRX` reader - Setting this bit resets the MAC Control Sublayer / Receive logic. The MCS logic implements flow control."]
pub type RESETMCSRX_R = crate::BitReader;
#[doc = "Field `RESETMCSRX` writer - Setting this bit resets the MAC Control Sublayer / Receive logic. The MCS logic implements flow control."]
pub type RESETMCSRX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SIMRESET` reader - SIMULATION RESET. Setting this bit will cause a reset to the random number generator within the Transmit Function."]
pub type SIMRESET_R = crate::BitReader;
#[doc = "Field `SIMRESET` writer - SIMULATION RESET. Setting this bit will cause a reset to the random number generator within the Transmit Function."]
pub type SIMRESET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOFTRESET` reader - SOFT RESET. Setting this bit will put all modules within the MAC in reset except the Host Interface."]
pub type SOFTRESET_R = crate::BitReader;
#[doc = "Field `SOFTRESET` writer - SOFT RESET. Setting this bit will put all modules within the MAC in reset except the Host Interface."]
pub type SOFTRESET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - RECEIVE ENABLE. Set this to allow receive frames to be received. Internally the MAC synchronizes this control bit to the incoming receive stream."]
    #[inline(always)]
    pub fn rxenable(&self) -> RXENABLE_R {
        RXENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PASS ALL RECEIVE FRAMES. When enabled (set to 1), the MAC will pass all frames regardless of type (normal vs. Control). When disabled, the MAC does not pass valid Control frames."]
    #[inline(always)]
    pub fn parf(&self) -> PARF_R {
        PARF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX FLOW CONTROL. When enabled (set to 1), the MAC acts upon received PAUSE Flow Control frames. When disabled, received PAUSE Flow Control frames are ignored."]
    #[inline(always)]
    pub fn rxflowctrl(&self) -> RXFLOWCTRL_R {
        RXFLOWCTRL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TX FLOW CONTROL. When enabled (set to 1), PAUSE Flow Control frames are allowed to be transmitted. When disabled, Flow Control frames are blocked."]
    #[inline(always)]
    pub fn txflowctrl(&self) -> TXFLOWCTRL_R {
        TXFLOWCTRL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Setting this bit will cause the MAC Transmit interface to be looped back to the MAC Receive interface. Clearing this bit results in normal operation."]
    #[inline(always)]
    pub fn loopback(&self) -> LOOPBACK_R {
        LOOPBACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Setting this bit will put the Transmit Function logic in reset."]
    #[inline(always)]
    pub fn resettx(&self) -> RESETTX_R {
        RESETTX_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Setting this bit resets the MAC Control Sublayer / Transmit logic. The MCS logic implements flow control."]
    #[inline(always)]
    pub fn resetmcstx(&self) -> RESETMCSTX_R {
        RESETMCSTX_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Setting this bit will put the Ethernet receive logic in reset."]
    #[inline(always)]
    pub fn resetrx(&self) -> RESETRX_R {
        RESETRX_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Setting this bit resets the MAC Control Sublayer / Receive logic. The MCS logic implements flow control."]
    #[inline(always)]
    pub fn resetmcsrx(&self) -> RESETMCSRX_R {
        RESETMCSRX_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SIMULATION RESET. Setting this bit will cause a reset to the random number generator within the Transmit Function."]
    #[inline(always)]
    pub fn simreset(&self) -> SIMRESET_R {
        SIMRESET_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SOFT RESET. Setting this bit will put all modules within the MAC in reset except the Host Interface."]
    #[inline(always)]
    pub fn softreset(&self) -> SOFTRESET_R {
        SOFTRESET_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAC1")
            .field("rxenable", &format_args!("{}", self.rxenable().bit()))
            .field("parf", &format_args!("{}", self.parf().bit()))
            .field("rxflowctrl", &format_args!("{}", self.rxflowctrl().bit()))
            .field("txflowctrl", &format_args!("{}", self.txflowctrl().bit()))
            .field("loopback", &format_args!("{}", self.loopback().bit()))
            .field("resettx", &format_args!("{}", self.resettx().bit()))
            .field("resetmcstx", &format_args!("{}", self.resetmcstx().bit()))
            .field("resetrx", &format_args!("{}", self.resetrx().bit()))
            .field("resetmcsrx", &format_args!("{}", self.resetmcsrx().bit()))
            .field("simreset", &format_args!("{}", self.simreset().bit()))
            .field("softreset", &format_args!("{}", self.softreset().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<MAC1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - RECEIVE ENABLE. Set this to allow receive frames to be received. Internally the MAC synchronizes this control bit to the incoming receive stream."]
    #[inline(always)]
    #[must_use]
    pub fn rxenable(&mut self) -> RXENABLE_W<MAC1_SPEC, 0> {
        RXENABLE_W::new(self)
    }
    #[doc = "Bit 1 - PASS ALL RECEIVE FRAMES. When enabled (set to 1), the MAC will pass all frames regardless of type (normal vs. Control). When disabled, the MAC does not pass valid Control frames."]
    #[inline(always)]
    #[must_use]
    pub fn parf(&mut self) -> PARF_W<MAC1_SPEC, 1> {
        PARF_W::new(self)
    }
    #[doc = "Bit 2 - RX FLOW CONTROL. When enabled (set to 1), the MAC acts upon received PAUSE Flow Control frames. When disabled, received PAUSE Flow Control frames are ignored."]
    #[inline(always)]
    #[must_use]
    pub fn rxflowctrl(&mut self) -> RXFLOWCTRL_W<MAC1_SPEC, 2> {
        RXFLOWCTRL_W::new(self)
    }
    #[doc = "Bit 3 - TX FLOW CONTROL. When enabled (set to 1), PAUSE Flow Control frames are allowed to be transmitted. When disabled, Flow Control frames are blocked."]
    #[inline(always)]
    #[must_use]
    pub fn txflowctrl(&mut self) -> TXFLOWCTRL_W<MAC1_SPEC, 3> {
        TXFLOWCTRL_W::new(self)
    }
    #[doc = "Bit 4 - Setting this bit will cause the MAC Transmit interface to be looped back to the MAC Receive interface. Clearing this bit results in normal operation."]
    #[inline(always)]
    #[must_use]
    pub fn loopback(&mut self) -> LOOPBACK_W<MAC1_SPEC, 4> {
        LOOPBACK_W::new(self)
    }
    #[doc = "Bit 8 - Setting this bit will put the Transmit Function logic in reset."]
    #[inline(always)]
    #[must_use]
    pub fn resettx(&mut self) -> RESETTX_W<MAC1_SPEC, 8> {
        RESETTX_W::new(self)
    }
    #[doc = "Bit 9 - Setting this bit resets the MAC Control Sublayer / Transmit logic. The MCS logic implements flow control."]
    #[inline(always)]
    #[must_use]
    pub fn resetmcstx(&mut self) -> RESETMCSTX_W<MAC1_SPEC, 9> {
        RESETMCSTX_W::new(self)
    }
    #[doc = "Bit 10 - Setting this bit will put the Ethernet receive logic in reset."]
    #[inline(always)]
    #[must_use]
    pub fn resetrx(&mut self) -> RESETRX_W<MAC1_SPEC, 10> {
        RESETRX_W::new(self)
    }
    #[doc = "Bit 11 - Setting this bit resets the MAC Control Sublayer / Receive logic. The MCS logic implements flow control."]
    #[inline(always)]
    #[must_use]
    pub fn resetmcsrx(&mut self) -> RESETMCSRX_W<MAC1_SPEC, 11> {
        RESETMCSRX_W::new(self)
    }
    #[doc = "Bit 14 - SIMULATION RESET. Setting this bit will cause a reset to the random number generator within the Transmit Function."]
    #[inline(always)]
    #[must_use]
    pub fn simreset(&mut self) -> SIMRESET_W<MAC1_SPEC, 14> {
        SIMRESET_W::new(self)
    }
    #[doc = "Bit 15 - SOFT RESET. Setting this bit will put all modules within the MAC in reset except the Host Interface."]
    #[inline(always)]
    #[must_use]
    pub fn softreset(&mut self) -> SOFTRESET_W<MAC1_SPEC, 15> {
        SOFTRESET_W::new(self)
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
#[doc = "MAC configuration register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAC1_SPEC;
impl crate::RegisterSpec for MAC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac1::R`](R) reader structure"]
impl crate::Readable for MAC1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mac1::W`](W) writer structure"]
impl crate::Writable for MAC1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAC1 to value 0x8000"]
impl crate::Resettable for MAC1_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
