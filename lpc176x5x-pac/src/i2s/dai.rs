#[doc = "Register `DAI` reader"]
pub type R = crate::R<DAI_SPEC>;
#[doc = "Register `DAI` writer"]
pub type W = crate::W<DAI_SPEC>;
#[doc = "Field `WORDWIDTH` reader - Selects the number of bytes in data as follows:"]
pub type WORDWIDTH_R = crate::FieldReader<WORDWIDTH_A>;
#[doc = "Selects the number of bytes in data as follows:\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WORDWIDTH_A {
    #[doc = "0: 8-bit data"]
    _8_BITS = 0,
    #[doc = "1: 16-bit data"]
    _16_BITS = 1,
    #[doc = "3: 32-bit data"]
    _32_BITS = 3,
}
impl From<WORDWIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: WORDWIDTH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WORDWIDTH_A {
    type Ux = u8;
}
impl WORDWIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WORDWIDTH_A> {
        match self.bits {
            0 => Some(WORDWIDTH_A::_8_BITS),
            1 => Some(WORDWIDTH_A::_16_BITS),
            3 => Some(WORDWIDTH_A::_32_BITS),
            _ => None,
        }
    }
    #[doc = "8-bit data"]
    #[inline(always)]
    pub fn is_8_bits(&self) -> bool {
        *self == WORDWIDTH_A::_8_BITS
    }
    #[doc = "16-bit data"]
    #[inline(always)]
    pub fn is_16_bits(&self) -> bool {
        *self == WORDWIDTH_A::_16_BITS
    }
    #[doc = "32-bit data"]
    #[inline(always)]
    pub fn is_32_bits(&self) -> bool {
        *self == WORDWIDTH_A::_32_BITS
    }
}
#[doc = "Field `WORDWIDTH` writer - Selects the number of bytes in data as follows:"]
pub type WORDWIDTH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, WORDWIDTH_A>;
impl<'a, REG, const O: u8> WORDWIDTH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8-bit data"]
    #[inline(always)]
    pub fn _8_bits(self) -> &'a mut crate::W<REG> {
        self.variant(WORDWIDTH_A::_8_BITS)
    }
    #[doc = "16-bit data"]
    #[inline(always)]
    pub fn _16_bits(self) -> &'a mut crate::W<REG> {
        self.variant(WORDWIDTH_A::_16_BITS)
    }
    #[doc = "32-bit data"]
    #[inline(always)]
    pub fn _32_bits(self) -> &'a mut crate::W<REG> {
        self.variant(WORDWIDTH_A::_32_BITS)
    }
}
#[doc = "Field `MONO` reader - When 1, data is of monaural format. When 0, the data is in stereo format."]
pub type MONO_R = crate::BitReader;
#[doc = "Field `MONO` writer - When 1, data is of monaural format. When 0, the data is in stereo format."]
pub type MONO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STOP` reader - When 1, disables accesses on FIFOs, places the transmit channel in mute mode."]
pub type STOP_R = crate::BitReader;
#[doc = "Field `STOP` writer - When 1, disables accesses on FIFOs, places the transmit channel in mute mode."]
pub type STOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESET` reader - When 1, asynchronously reset the transmit channel and FIFO."]
pub type RESET_R = crate::BitReader;
#[doc = "Field `RESET` writer - When 1, asynchronously reset the transmit channel and FIFO."]
pub type RESET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WS_SEL` reader - When 0, the interface is in master mode. When 1, the interface is in slave mode. See Section 34.7.2 for a summary of useful combinations for this bit with RXMODE."]
pub type WS_SEL_R = crate::BitReader;
#[doc = "Field `WS_SEL` writer - When 0, the interface is in master mode. When 1, the interface is in slave mode. See Section 34.7.2 for a summary of useful combinations for this bit with RXMODE."]
pub type WS_SEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WS_HALFPERIOD` reader - Word select half period minus 1, i.e. WS 64clk period -> ws_halfperiod = 31."]
pub type WS_HALFPERIOD_R = crate::FieldReader<u16>;
#[doc = "Field `WS_HALFPERIOD` writer - Word select half period minus 1, i.e. WS 64clk period -> ws_halfperiod = 31."]
pub type WS_HALFPERIOD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
impl R {
    #[doc = "Bits 0:1 - Selects the number of bytes in data as follows:"]
    #[inline(always)]
    pub fn wordwidth(&self) -> WORDWIDTH_R {
        WORDWIDTH_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - When 1, data is of monaural format. When 0, the data is in stereo format."]
    #[inline(always)]
    pub fn mono(&self) -> MONO_R {
        MONO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When 1, disables accesses on FIFOs, places the transmit channel in mute mode."]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When 1, asynchronously reset the transmit channel and FIFO."]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When 0, the interface is in master mode. When 1, the interface is in slave mode. See Section 34.7.2 for a summary of useful combinations for this bit with RXMODE."]
    #[inline(always)]
    pub fn ws_sel(&self) -> WS_SEL_R {
        WS_SEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:14 - Word select half period minus 1, i.e. WS 64clk period -> ws_halfperiod = 31."]
    #[inline(always)]
    pub fn ws_halfperiod(&self) -> WS_HALFPERIOD_R {
        WS_HALFPERIOD_R::new(((self.bits >> 6) & 0x01ff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAI")
            .field("wordwidth", &format_args!("{}", self.wordwidth().bits()))
            .field("mono", &format_args!("{}", self.mono().bit()))
            .field("stop", &format_args!("{}", self.stop().bit()))
            .field("reset", &format_args!("{}", self.reset().bit()))
            .field("ws_sel", &format_args!("{}", self.ws_sel().bit()))
            .field(
                "ws_halfperiod",
                &format_args!("{}", self.ws_halfperiod().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<DAI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects the number of bytes in data as follows:"]
    #[inline(always)]
    #[must_use]
    pub fn wordwidth(&mut self) -> WORDWIDTH_W<DAI_SPEC, 0> {
        WORDWIDTH_W::new(self)
    }
    #[doc = "Bit 2 - When 1, data is of monaural format. When 0, the data is in stereo format."]
    #[inline(always)]
    #[must_use]
    pub fn mono(&mut self) -> MONO_W<DAI_SPEC, 2> {
        MONO_W::new(self)
    }
    #[doc = "Bit 3 - When 1, disables accesses on FIFOs, places the transmit channel in mute mode."]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<DAI_SPEC, 3> {
        STOP_W::new(self)
    }
    #[doc = "Bit 4 - When 1, asynchronously reset the transmit channel and FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<DAI_SPEC, 4> {
        RESET_W::new(self)
    }
    #[doc = "Bit 5 - When 0, the interface is in master mode. When 1, the interface is in slave mode. See Section 34.7.2 for a summary of useful combinations for this bit with RXMODE."]
    #[inline(always)]
    #[must_use]
    pub fn ws_sel(&mut self) -> WS_SEL_W<DAI_SPEC, 5> {
        WS_SEL_W::new(self)
    }
    #[doc = "Bits 6:14 - Word select half period minus 1, i.e. WS 64clk period -> ws_halfperiod = 31."]
    #[inline(always)]
    #[must_use]
    pub fn ws_halfperiod(&mut self) -> WS_HALFPERIOD_W<DAI_SPEC, 6> {
        WS_HALFPERIOD_W::new(self)
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
#[doc = "I2S Digital Audio Input Register. Contains control bits for the I2S receive channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dai::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dai::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAI_SPEC;
impl crate::RegisterSpec for DAI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dai::R`](R) reader structure"]
impl crate::Readable for DAI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dai::W`](W) writer structure"]
impl crate::Writable for DAI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAI to value 0x07e1"]
impl crate::Resettable for DAI_SPEC {
    const RESET_VALUE: Self::Ux = 0x07e1;
}
