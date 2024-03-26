#[doc = "Register `MCR` reader"]
pub type R = crate::R<MCR_SPEC>;
#[doc = "Register `MCR` writer"]
pub type W = crate::W<MCR_SPEC>;
#[doc = "Field `DTRCTRL` reader - DTR Control. Source for modem output pin, DTR. This bit reads as 0 when modem loopback mode is active."]
pub type DTRCTRL_R = crate::BitReader;
#[doc = "Field `DTRCTRL` writer - DTR Control. Source for modem output pin, DTR. This bit reads as 0 when modem loopback mode is active."]
pub type DTRCTRL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTSCTRL` reader - RTS Control. Source for modem output pin RTS. This bit reads as 0 when modem loopback mode is active."]
pub type RTSCTRL_R = crate::BitReader;
#[doc = "Field `RTSCTRL` writer - RTS Control. Source for modem output pin RTS. This bit reads as 0 when modem loopback mode is active."]
pub type RTSCTRL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LMS` reader - Loopback Mode Select. The modem loopback mode provides a mechanism to perform diagnostic loopback testing. Serial data from the transmitter is connected internally to serial input of the receiver. Input pin, RXD1, has no effect on loopback and output pin, TXD1 is held in marking state. The 4 modem inputs (CTS, DSR, RI and DCD) are disconnected externally. Externally, the modem outputs (RTS, DTR) are set inactive. Internally, the 4 modem outputs are connected to the 4 modem inputs. As a result of these connections, the upper 4 bits of the MSR will be driven by the lower 4 bits of the MCR rather than the 4 modem inputs in normal mode. This permits modem status interrupts to be generated in loopback mode by writing the lower 4 bits of MCR."]
pub type LMS_R = crate::BitReader<LMS_A>;
#[doc = "Loopback Mode Select. The modem loopback mode provides a mechanism to perform diagnostic loopback testing. Serial data from the transmitter is connected internally to serial input of the receiver. Input pin, RXD1, has no effect on loopback and output pin, TXD1 is held in marking state. The 4 modem inputs (CTS, DSR, RI and DCD) are disconnected externally. Externally, the modem outputs (RTS, DTR) are set inactive. Internally, the 4 modem outputs are connected to the 4 modem inputs. As a result of these connections, the upper 4 bits of the MSR will be driven by the lower 4 bits of the MCR rather than the 4 modem inputs in normal mode. This permits modem status interrupts to be generated in loopback mode by writing the lower 4 bits of MCR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LMS_A {
    #[doc = "0: Disable modem loopback mode."]
    DISABLE_MODEM_LOOPBA = 0,
    #[doc = "1: Enable modem loopback mode."]
    ENABLE_MODEM_LOOPBAC = 1,
}
impl From<LMS_A> for bool {
    #[inline(always)]
    fn from(variant: LMS_A) -> Self {
        variant as u8 != 0
    }
}
impl LMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LMS_A {
        match self.bits {
            false => LMS_A::DISABLE_MODEM_LOOPBA,
            true => LMS_A::ENABLE_MODEM_LOOPBAC,
        }
    }
    #[doc = "Disable modem loopback mode."]
    #[inline(always)]
    pub fn is_disable_modem_loopba(&self) -> bool {
        *self == LMS_A::DISABLE_MODEM_LOOPBA
    }
    #[doc = "Enable modem loopback mode."]
    #[inline(always)]
    pub fn is_enable_modem_loopbac(&self) -> bool {
        *self == LMS_A::ENABLE_MODEM_LOOPBAC
    }
}
#[doc = "Field `LMS` writer - Loopback Mode Select. The modem loopback mode provides a mechanism to perform diagnostic loopback testing. Serial data from the transmitter is connected internally to serial input of the receiver. Input pin, RXD1, has no effect on loopback and output pin, TXD1 is held in marking state. The 4 modem inputs (CTS, DSR, RI and DCD) are disconnected externally. Externally, the modem outputs (RTS, DTR) are set inactive. Internally, the 4 modem outputs are connected to the 4 modem inputs. As a result of these connections, the upper 4 bits of the MSR will be driven by the lower 4 bits of the MCR rather than the 4 modem inputs in normal mode. This permits modem status interrupts to be generated in loopback mode by writing the lower 4 bits of MCR."]
pub type LMS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LMS_A>;
impl<'a, REG, const O: u8> LMS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable modem loopback mode."]
    #[inline(always)]
    pub fn disable_modem_loopba(self) -> &'a mut crate::W<REG> {
        self.variant(LMS_A::DISABLE_MODEM_LOOPBA)
    }
    #[doc = "Enable modem loopback mode."]
    #[inline(always)]
    pub fn enable_modem_loopbac(self) -> &'a mut crate::W<REG> {
        self.variant(LMS_A::ENABLE_MODEM_LOOPBAC)
    }
}
#[doc = "Field `RTSEN` reader - RTS enable."]
pub type RTSEN_R = crate::BitReader<RTSEN_A>;
#[doc = "RTS enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTSEN_A {
    #[doc = "0: Disable auto-rts flow control."]
    DISABLE_AUTO_RTS_FLO = 0,
    #[doc = "1: Enable auto-rts flow control."]
    ENABLE_AUTO_RTS_FLOW = 1,
}
impl From<RTSEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RTSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTSEN_A {
        match self.bits {
            false => RTSEN_A::DISABLE_AUTO_RTS_FLO,
            true => RTSEN_A::ENABLE_AUTO_RTS_FLOW,
        }
    }
    #[doc = "Disable auto-rts flow control."]
    #[inline(always)]
    pub fn is_disable_auto_rts_flo(&self) -> bool {
        *self == RTSEN_A::DISABLE_AUTO_RTS_FLO
    }
    #[doc = "Enable auto-rts flow control."]
    #[inline(always)]
    pub fn is_enable_auto_rts_flow(&self) -> bool {
        *self == RTSEN_A::ENABLE_AUTO_RTS_FLOW
    }
}
#[doc = "Field `RTSEN` writer - RTS enable."]
pub type RTSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RTSEN_A>;
impl<'a, REG, const O: u8> RTSEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable auto-rts flow control."]
    #[inline(always)]
    pub fn disable_auto_rts_flo(self) -> &'a mut crate::W<REG> {
        self.variant(RTSEN_A::DISABLE_AUTO_RTS_FLO)
    }
    #[doc = "Enable auto-rts flow control."]
    #[inline(always)]
    pub fn enable_auto_rts_flow(self) -> &'a mut crate::W<REG> {
        self.variant(RTSEN_A::ENABLE_AUTO_RTS_FLOW)
    }
}
#[doc = "Field `CTSEN` reader - CTS enable."]
pub type CTSEN_R = crate::BitReader<CTSEN_A>;
#[doc = "CTS enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSEN_A {
    #[doc = "0: Disable auto-cts flow control."]
    DISABLE_AUTO_CTS_FLO = 0,
    #[doc = "1: Enable auto-cts flow control."]
    ENABLE_AUTO_CTS_FLOW = 1,
}
impl From<CTSEN_A> for bool {
    #[inline(always)]
    fn from(variant: CTSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CTSEN_A {
        match self.bits {
            false => CTSEN_A::DISABLE_AUTO_CTS_FLO,
            true => CTSEN_A::ENABLE_AUTO_CTS_FLOW,
        }
    }
    #[doc = "Disable auto-cts flow control."]
    #[inline(always)]
    pub fn is_disable_auto_cts_flo(&self) -> bool {
        *self == CTSEN_A::DISABLE_AUTO_CTS_FLO
    }
    #[doc = "Enable auto-cts flow control."]
    #[inline(always)]
    pub fn is_enable_auto_cts_flow(&self) -> bool {
        *self == CTSEN_A::ENABLE_AUTO_CTS_FLOW
    }
}
#[doc = "Field `CTSEN` writer - CTS enable."]
pub type CTSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CTSEN_A>;
impl<'a, REG, const O: u8> CTSEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable auto-cts flow control."]
    #[inline(always)]
    pub fn disable_auto_cts_flo(self) -> &'a mut crate::W<REG> {
        self.variant(CTSEN_A::DISABLE_AUTO_CTS_FLO)
    }
    #[doc = "Enable auto-cts flow control."]
    #[inline(always)]
    pub fn enable_auto_cts_flow(self) -> &'a mut crate::W<REG> {
        self.variant(CTSEN_A::ENABLE_AUTO_CTS_FLOW)
    }
}
impl R {
    #[doc = "Bit 0 - DTR Control. Source for modem output pin, DTR. This bit reads as 0 when modem loopback mode is active."]
    #[inline(always)]
    pub fn dtrctrl(&self) -> DTRCTRL_R {
        DTRCTRL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTS Control. Source for modem output pin RTS. This bit reads as 0 when modem loopback mode is active."]
    #[inline(always)]
    pub fn rtsctrl(&self) -> RTSCTRL_R {
        RTSCTRL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Loopback Mode Select. The modem loopback mode provides a mechanism to perform diagnostic loopback testing. Serial data from the transmitter is connected internally to serial input of the receiver. Input pin, RXD1, has no effect on loopback and output pin, TXD1 is held in marking state. The 4 modem inputs (CTS, DSR, RI and DCD) are disconnected externally. Externally, the modem outputs (RTS, DTR) are set inactive. Internally, the 4 modem outputs are connected to the 4 modem inputs. As a result of these connections, the upper 4 bits of the MSR will be driven by the lower 4 bits of the MCR rather than the 4 modem inputs in normal mode. This permits modem status interrupts to be generated in loopback mode by writing the lower 4 bits of MCR."]
    #[inline(always)]
    pub fn lms(&self) -> LMS_R {
        LMS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - RTS enable."]
    #[inline(always)]
    pub fn rtsen(&self) -> RTSEN_R {
        RTSEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CTS enable."]
    #[inline(always)]
    pub fn ctsen(&self) -> CTSEN_R {
        CTSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCR")
            .field("dtrctrl", &format_args!("{}", self.dtrctrl().bit()))
            .field("rtsctrl", &format_args!("{}", self.rtsctrl().bit()))
            .field("lms", &format_args!("{}", self.lms().bit()))
            .field("rtsen", &format_args!("{}", self.rtsen().bit()))
            .field("ctsen", &format_args!("{}", self.ctsen().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<MCR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - DTR Control. Source for modem output pin, DTR. This bit reads as 0 when modem loopback mode is active."]
    #[inline(always)]
    #[must_use]
    pub fn dtrctrl(&mut self) -> DTRCTRL_W<MCR_SPEC, 0> {
        DTRCTRL_W::new(self)
    }
    #[doc = "Bit 1 - RTS Control. Source for modem output pin RTS. This bit reads as 0 when modem loopback mode is active."]
    #[inline(always)]
    #[must_use]
    pub fn rtsctrl(&mut self) -> RTSCTRL_W<MCR_SPEC, 1> {
        RTSCTRL_W::new(self)
    }
    #[doc = "Bit 4 - Loopback Mode Select. The modem loopback mode provides a mechanism to perform diagnostic loopback testing. Serial data from the transmitter is connected internally to serial input of the receiver. Input pin, RXD1, has no effect on loopback and output pin, TXD1 is held in marking state. The 4 modem inputs (CTS, DSR, RI and DCD) are disconnected externally. Externally, the modem outputs (RTS, DTR) are set inactive. Internally, the 4 modem outputs are connected to the 4 modem inputs. As a result of these connections, the upper 4 bits of the MSR will be driven by the lower 4 bits of the MCR rather than the 4 modem inputs in normal mode. This permits modem status interrupts to be generated in loopback mode by writing the lower 4 bits of MCR."]
    #[inline(always)]
    #[must_use]
    pub fn lms(&mut self) -> LMS_W<MCR_SPEC, 4> {
        LMS_W::new(self)
    }
    #[doc = "Bit 6 - RTS enable."]
    #[inline(always)]
    #[must_use]
    pub fn rtsen(&mut self) -> RTSEN_W<MCR_SPEC, 6> {
        RTSEN_W::new(self)
    }
    #[doc = "Bit 7 - CTS enable."]
    #[inline(always)]
    #[must_use]
    pub fn ctsen(&mut self) -> CTSEN_W<MCR_SPEC, 7> {
        CTSEN_W::new(self)
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
#[doc = "Modem Control Register. Contains controls for flow control handshaking and loopback mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcr::R`](R) reader structure"]
impl crate::Readable for MCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcr::W`](W) writer structure"]
impl crate::Writable for MCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for MCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
