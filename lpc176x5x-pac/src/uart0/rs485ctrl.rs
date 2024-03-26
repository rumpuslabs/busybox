#[doc = "Register `RS485CTRL` reader"]
pub type R = crate::R<RS485CTRL_SPEC>;
#[doc = "Register `RS485CTRL` writer"]
pub type W = crate::W<RS485CTRL_SPEC>;
#[doc = "Field `NMMEN` reader - NMM enable."]
pub type NMMEN_R = crate::BitReader<NMMEN_A>;
#[doc = "NMM enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NMMEN_A {
    #[doc = "0: RS-485/EIA-485 Normal Multidrop Mode (NMM) is disabled."]
    DISABLED = 0,
    #[doc = "1: RS-485/EIA-485 Normal Multidrop Mode (NMM) is enabled. In this mode, an address is detected when a received byte has the parity bit = 1, generating a received data interrupt. See Section 18.6.16 RS-485/EIA-485 modes of operation."]
    ENABLED = 1,
}
impl From<NMMEN_A> for bool {
    #[inline(always)]
    fn from(variant: NMMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl NMMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NMMEN_A {
        match self.bits {
            false => NMMEN_A::DISABLED,
            true => NMMEN_A::ENABLED,
        }
    }
    #[doc = "RS-485/EIA-485 Normal Multidrop Mode (NMM) is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NMMEN_A::DISABLED
    }
    #[doc = "RS-485/EIA-485 Normal Multidrop Mode (NMM) is enabled. In this mode, an address is detected when a received byte has the parity bit = 1, generating a received data interrupt. See Section 18.6.16 RS-485/EIA-485 modes of operation."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NMMEN_A::ENABLED
    }
}
#[doc = "Field `NMMEN` writer - NMM enable."]
pub type NMMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, NMMEN_A>;
impl<'a, REG, const O: u8> NMMEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RS-485/EIA-485 Normal Multidrop Mode (NMM) is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(NMMEN_A::DISABLED)
    }
    #[doc = "RS-485/EIA-485 Normal Multidrop Mode (NMM) is enabled. In this mode, an address is detected when a received byte has the parity bit = 1, generating a received data interrupt. See Section 18.6.16 RS-485/EIA-485 modes of operation."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(NMMEN_A::ENABLED)
    }
}
#[doc = "Field `RXDIS` reader - Receiver enable."]
pub type RXDIS_R = crate::BitReader<RXDIS_A>;
#[doc = "Receiver enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDIS_A {
    #[doc = "0: The receiver is enabled."]
    ENABLED = 0,
    #[doc = "1: The receiver is disabled."]
    DISABLED = 1,
}
impl From<RXDIS_A> for bool {
    #[inline(always)]
    fn from(variant: RXDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl RXDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXDIS_A {
        match self.bits {
            false => RXDIS_A::ENABLED,
            true => RXDIS_A::DISABLED,
        }
    }
    #[doc = "The receiver is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXDIS_A::ENABLED
    }
    #[doc = "The receiver is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXDIS_A::DISABLED
    }
}
#[doc = "Field `RXDIS` writer - Receiver enable."]
pub type RXDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RXDIS_A>;
impl<'a, REG, const O: u8> RXDIS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The receiver is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXDIS_A::ENABLED)
    }
    #[doc = "The receiver is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXDIS_A::DISABLED)
    }
}
#[doc = "Field `AADEN` reader - AAD enable."]
pub type AADEN_R = crate::BitReader<AADEN_A>;
#[doc = "AAD enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AADEN_A {
    #[doc = "0: Auto Address Detect (AAD) is disabled."]
    DISABLED = 0,
    #[doc = "1: Auto Address Detect (AAD) is enabled."]
    ENABLED = 1,
}
impl From<AADEN_A> for bool {
    #[inline(always)]
    fn from(variant: AADEN_A) -> Self {
        variant as u8 != 0
    }
}
impl AADEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AADEN_A {
        match self.bits {
            false => AADEN_A::DISABLED,
            true => AADEN_A::ENABLED,
        }
    }
    #[doc = "Auto Address Detect (AAD) is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AADEN_A::DISABLED
    }
    #[doc = "Auto Address Detect (AAD) is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AADEN_A::ENABLED
    }
}
#[doc = "Field `AADEN` writer - AAD enable."]
pub type AADEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, AADEN_A>;
impl<'a, REG, const O: u8> AADEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Auto Address Detect (AAD) is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AADEN_A::DISABLED)
    }
    #[doc = "Auto Address Detect (AAD) is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AADEN_A::ENABLED)
    }
}
#[doc = "Field `DCTRL` reader - Direction control enable."]
pub type DCTRL_R = crate::BitReader<DCTRL_A>;
#[doc = "Direction control enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCTRL_A {
    #[doc = "0: Disable Auto Direction Control."]
    DISABLE = 0,
    #[doc = "1: Enable Auto Direction Control."]
    ENABLE = 1,
}
impl From<DCTRL_A> for bool {
    #[inline(always)]
    fn from(variant: DCTRL_A) -> Self {
        variant as u8 != 0
    }
}
impl DCTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DCTRL_A {
        match self.bits {
            false => DCTRL_A::DISABLE,
            true => DCTRL_A::ENABLE,
        }
    }
    #[doc = "Disable Auto Direction Control."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DCTRL_A::DISABLE
    }
    #[doc = "Enable Auto Direction Control."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DCTRL_A::ENABLE
    }
}
#[doc = "Field `DCTRL` writer - Direction control enable."]
pub type DCTRL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DCTRL_A>;
impl<'a, REG, const O: u8> DCTRL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Auto Direction Control."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DCTRL_A::DISABLE)
    }
    #[doc = "Enable Auto Direction Control."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DCTRL_A::ENABLE)
    }
}
#[doc = "Field `OINV` reader - Direction control pin polarity. This bit reverses the polarity of the direction control signal on the Un_OE pin."]
pub type OINV_R = crate::BitReader<OINV_A>;
#[doc = "Direction control pin polarity. This bit reverses the polarity of the direction control signal on the Un_OE pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OINV_A {
    #[doc = "0: The direction control pin will be driven to logic 0 when the transmitter has data to be sent. It will be driven to logic 1 after the last bit of data has been transmitted."]
    DIRLOW = 0,
    #[doc = "1: The direction control pin will be driven to logic 1 when the transmitter has data to be sent. It will be driven to logic 0 after the last bit of data has been transmitted."]
    DIRHIGH = 1,
}
impl From<OINV_A> for bool {
    #[inline(always)]
    fn from(variant: OINV_A) -> Self {
        variant as u8 != 0
    }
}
impl OINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OINV_A {
        match self.bits {
            false => OINV_A::DIRLOW,
            true => OINV_A::DIRHIGH,
        }
    }
    #[doc = "The direction control pin will be driven to logic 0 when the transmitter has data to be sent. It will be driven to logic 1 after the last bit of data has been transmitted."]
    #[inline(always)]
    pub fn is_dirlow(&self) -> bool {
        *self == OINV_A::DIRLOW
    }
    #[doc = "The direction control pin will be driven to logic 1 when the transmitter has data to be sent. It will be driven to logic 0 after the last bit of data has been transmitted."]
    #[inline(always)]
    pub fn is_dirhigh(&self) -> bool {
        *self == OINV_A::DIRHIGH
    }
}
#[doc = "Field `OINV` writer - Direction control pin polarity. This bit reverses the polarity of the direction control signal on the Un_OE pin."]
pub type OINV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OINV_A>;
impl<'a, REG, const O: u8> OINV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The direction control pin will be driven to logic 0 when the transmitter has data to be sent. It will be driven to logic 1 after the last bit of data has been transmitted."]
    #[inline(always)]
    pub fn dirlow(self) -> &'a mut crate::W<REG> {
        self.variant(OINV_A::DIRLOW)
    }
    #[doc = "The direction control pin will be driven to logic 1 when the transmitter has data to be sent. It will be driven to logic 0 after the last bit of data has been transmitted."]
    #[inline(always)]
    pub fn dirhigh(self) -> &'a mut crate::W<REG> {
        self.variant(OINV_A::DIRHIGH)
    }
}
impl R {
    #[doc = "Bit 0 - NMM enable."]
    #[inline(always)]
    pub fn nmmen(&self) -> NMMEN_R {
        NMMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receiver enable."]
    #[inline(always)]
    pub fn rxdis(&self) -> RXDIS_R {
        RXDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AAD enable."]
    #[inline(always)]
    pub fn aaden(&self) -> AADEN_R {
        AADEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Direction control enable."]
    #[inline(always)]
    pub fn dctrl(&self) -> DCTRL_R {
        DCTRL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Direction control pin polarity. This bit reverses the polarity of the direction control signal on the Un_OE pin."]
    #[inline(always)]
    pub fn oinv(&self) -> OINV_R {
        OINV_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RS485CTRL")
            .field("nmmen", &format_args!("{}", self.nmmen().bit()))
            .field("rxdis", &format_args!("{}", self.rxdis().bit()))
            .field("aaden", &format_args!("{}", self.aaden().bit()))
            .field("dctrl", &format_args!("{}", self.dctrl().bit()))
            .field("oinv", &format_args!("{}", self.oinv().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<RS485CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - NMM enable."]
    #[inline(always)]
    #[must_use]
    pub fn nmmen(&mut self) -> NMMEN_W<RS485CTRL_SPEC, 0> {
        NMMEN_W::new(self)
    }
    #[doc = "Bit 1 - Receiver enable."]
    #[inline(always)]
    #[must_use]
    pub fn rxdis(&mut self) -> RXDIS_W<RS485CTRL_SPEC, 1> {
        RXDIS_W::new(self)
    }
    #[doc = "Bit 2 - AAD enable."]
    #[inline(always)]
    #[must_use]
    pub fn aaden(&mut self) -> AADEN_W<RS485CTRL_SPEC, 2> {
        AADEN_W::new(self)
    }
    #[doc = "Bit 4 - Direction control enable."]
    #[inline(always)]
    #[must_use]
    pub fn dctrl(&mut self) -> DCTRL_W<RS485CTRL_SPEC, 4> {
        DCTRL_W::new(self)
    }
    #[doc = "Bit 5 - Direction control pin polarity. This bit reverses the polarity of the direction control signal on the Un_OE pin."]
    #[inline(always)]
    #[must_use]
    pub fn oinv(&mut self) -> OINV_W<RS485CTRL_SPEC, 5> {
        OINV_W::new(self)
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
#[doc = "RS-485/EIA-485 Control. Contains controls to configure various aspects of RS-485/EIA-485 modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rs485ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rs485ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RS485CTRL_SPEC;
impl crate::RegisterSpec for RS485CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rs485ctrl::R`](R) reader structure"]
impl crate::Readable for RS485CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rs485ctrl::W`](W) writer structure"]
impl crate::Writable for RS485CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RS485CTRL to value 0"]
impl crate::Resettable for RS485CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
