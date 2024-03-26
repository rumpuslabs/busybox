#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1_SPEC>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1_SPEC>;
#[doc = "Field `LBM` reader - Loop Back Mode."]
pub type LBM_R = crate::BitReader<LBM_A>;
#[doc = "Loop Back Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBM_A {
    #[doc = "0: During normal operation."]
    NORMAL = 0,
    #[doc = "1: Serial input is taken from the serial output (MOSI or MISO) rather than the serial input pin (MISO or MOSI respectively)."]
    OUPTU = 1,
}
impl From<LBM_A> for bool {
    #[inline(always)]
    fn from(variant: LBM_A) -> Self {
        variant as u8 != 0
    }
}
impl LBM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LBM_A {
        match self.bits {
            false => LBM_A::NORMAL,
            true => LBM_A::OUPTU,
        }
    }
    #[doc = "During normal operation."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == LBM_A::NORMAL
    }
    #[doc = "Serial input is taken from the serial output (MOSI or MISO) rather than the serial input pin (MISO or MOSI respectively)."]
    #[inline(always)]
    pub fn is_ouptu(&self) -> bool {
        *self == LBM_A::OUPTU
    }
}
#[doc = "Field `LBM` writer - Loop Back Mode."]
pub type LBM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, LBM_A>;
impl<'a, REG, const O: u8> LBM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "During normal operation."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(LBM_A::NORMAL)
    }
    #[doc = "Serial input is taken from the serial output (MOSI or MISO) rather than the serial input pin (MISO or MOSI respectively)."]
    #[inline(always)]
    pub fn ouptu(self) -> &'a mut crate::W<REG> {
        self.variant(LBM_A::OUPTU)
    }
}
#[doc = "Field `SSE` reader - SSP Enable."]
pub type SSE_R = crate::BitReader<SSE_A>;
#[doc = "SSP Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSE_A {
    #[doc = "0: The SSP controller is disabled."]
    DISABLED = 0,
    #[doc = "1: The SSP controller will interact with other devices on the serial bus. Software should write the appropriate control information to the other SSP registers and interrupt controller registers, before setting this bit."]
    ENABLED = 1,
}
impl From<SSE_A> for bool {
    #[inline(always)]
    fn from(variant: SSE_A) -> Self {
        variant as u8 != 0
    }
}
impl SSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSE_A {
        match self.bits {
            false => SSE_A::DISABLED,
            true => SSE_A::ENABLED,
        }
    }
    #[doc = "The SSP controller is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSE_A::DISABLED
    }
    #[doc = "The SSP controller will interact with other devices on the serial bus. Software should write the appropriate control information to the other SSP registers and interrupt controller registers, before setting this bit."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSE_A::ENABLED
    }
}
#[doc = "Field `SSE` writer - SSP Enable."]
pub type SSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SSE_A>;
impl<'a, REG, const O: u8> SSE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The SSP controller is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SSE_A::DISABLED)
    }
    #[doc = "The SSP controller will interact with other devices on the serial bus. Software should write the appropriate control information to the other SSP registers and interrupt controller registers, before setting this bit."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SSE_A::ENABLED)
    }
}
#[doc = "Field `MS` reader - Master/Slave Mode.This bit can only be written when the SSE bit is 0."]
pub type MS_R = crate::BitReader<MS_A>;
#[doc = "Master/Slave Mode.This bit can only be written when the SSE bit is 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MS_A {
    #[doc = "0: The SSP controller acts as a master on the bus, driving the SCLK, MOSI, and SSEL lines and receiving the MISO line."]
    MASTER = 0,
    #[doc = "1: The SSP controller acts as a slave on the bus, driving MISO line and receiving SCLK, MOSI, and SSEL lines."]
    SLAVE = 1,
}
impl From<MS_A> for bool {
    #[inline(always)]
    fn from(variant: MS_A) -> Self {
        variant as u8 != 0
    }
}
impl MS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MS_A {
        match self.bits {
            false => MS_A::MASTER,
            true => MS_A::SLAVE,
        }
    }
    #[doc = "The SSP controller acts as a master on the bus, driving the SCLK, MOSI, and SSEL lines and receiving the MISO line."]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == MS_A::MASTER
    }
    #[doc = "The SSP controller acts as a slave on the bus, driving MISO line and receiving SCLK, MOSI, and SSEL lines."]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == MS_A::SLAVE
    }
}
#[doc = "Field `MS` writer - Master/Slave Mode.This bit can only be written when the SSE bit is 0."]
pub type MS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MS_A>;
impl<'a, REG, const O: u8> MS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The SSP controller acts as a master on the bus, driving the SCLK, MOSI, and SSEL lines and receiving the MISO line."]
    #[inline(always)]
    pub fn master(self) -> &'a mut crate::W<REG> {
        self.variant(MS_A::MASTER)
    }
    #[doc = "The SSP controller acts as a slave on the bus, driving MISO line and receiving SCLK, MOSI, and SSEL lines."]
    #[inline(always)]
    pub fn slave(self) -> &'a mut crate::W<REG> {
        self.variant(MS_A::SLAVE)
    }
}
#[doc = "Field `SOD` reader - Slave Output Disable. This bit is relevant only in slave mode (MS = 1). If it is 1, this blocks this SSP controller from driving the transmit data line (MISO)."]
pub type SOD_R = crate::BitReader;
#[doc = "Field `SOD` writer - Slave Output Disable. This bit is relevant only in slave mode (MS = 1). If it is 1, this blocks this SSP controller from driving the transmit data line (MISO)."]
pub type SOD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Loop Back Mode."]
    #[inline(always)]
    pub fn lbm(&self) -> LBM_R {
        LBM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SSP Enable."]
    #[inline(always)]
    pub fn sse(&self) -> SSE_R {
        SSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master/Slave Mode.This bit can only be written when the SSE bit is 0."]
    #[inline(always)]
    pub fn ms(&self) -> MS_R {
        MS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Slave Output Disable. This bit is relevant only in slave mode (MS = 1). If it is 1, this blocks this SSP controller from driving the transmit data line (MISO)."]
    #[inline(always)]
    pub fn sod(&self) -> SOD_R {
        SOD_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("lbm", &format_args!("{}", self.lbm().bit()))
            .field("sse", &format_args!("{}", self.sse().bit()))
            .field("ms", &format_args!("{}", self.ms().bit()))
            .field("sod", &format_args!("{}", self.sod().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CR1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Loop Back Mode."]
    #[inline(always)]
    #[must_use]
    pub fn lbm(&mut self) -> LBM_W<CR1_SPEC, 0> {
        LBM_W::new(self)
    }
    #[doc = "Bit 1 - SSP Enable."]
    #[inline(always)]
    #[must_use]
    pub fn sse(&mut self) -> SSE_W<CR1_SPEC, 1> {
        SSE_W::new(self)
    }
    #[doc = "Bit 2 - Master/Slave Mode.This bit can only be written when the SSE bit is 0."]
    #[inline(always)]
    #[must_use]
    pub fn ms(&mut self) -> MS_W<CR1_SPEC, 2> {
        MS_W::new(self)
    }
    #[doc = "Bit 3 - Slave Output Disable. This bit is relevant only in slave mode (MS = 1). If it is 1, this blocks this SSP controller from driving the transmit data line (MISO)."]
    #[inline(always)]
    #[must_use]
    pub fn sod(&mut self) -> SOD_W<CR1_SPEC, 3> {
        SOD_W::new(self)
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
#[doc = "Control Register 1. Selects master/slave and other modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
