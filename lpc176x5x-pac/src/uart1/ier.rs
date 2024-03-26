#[doc = "Register `IER` reader"]
pub type R = crate::R<IER_SPEC>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IER_SPEC>;
#[doc = "Field `RBRIE` reader - RBR Interrupt Enable. Enables the Receive Data Available interrupt for UART1. It also controls the Character Receive Time-out interrupt."]
pub type RBRIE_R = crate::BitReader<RBRIE_A>;
#[doc = "RBR Interrupt Enable. Enables the Receive Data Available interrupt for UART1. It also controls the Character Receive Time-out interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RBRIE_A {
    #[doc = "0: Disable the RDA interrupts."]
    DISABLE = 0,
    #[doc = "1: Enable the RDA interrupts."]
    ENABLE = 1,
}
impl From<RBRIE_A> for bool {
    #[inline(always)]
    fn from(variant: RBRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RBRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RBRIE_A {
        match self.bits {
            false => RBRIE_A::DISABLE,
            true => RBRIE_A::ENABLE,
        }
    }
    #[doc = "Disable the RDA interrupts."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RBRIE_A::DISABLE
    }
    #[doc = "Enable the RDA interrupts."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RBRIE_A::ENABLE
    }
}
#[doc = "Field `RBRIE` writer - RBR Interrupt Enable. Enables the Receive Data Available interrupt for UART1. It also controls the Character Receive Time-out interrupt."]
pub type RBRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RBRIE_A>;
impl<'a, REG, const O: u8> RBRIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the RDA interrupts."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RBRIE_A::DISABLE)
    }
    #[doc = "Enable the RDA interrupts."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RBRIE_A::ENABLE)
    }
}
#[doc = "Field `THREIE` reader - THRE Interrupt Enable. Enables the THRE interrupt for UART1. The status of this interrupt can be read from LSR\\[5\\]."]
pub type THREIE_R = crate::BitReader<THREIE_A>;
#[doc = "THRE Interrupt Enable. Enables the THRE interrupt for UART1. The status of this interrupt can be read from LSR\\[5\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum THREIE_A {
    #[doc = "0: Disable the THRE interrupts."]
    DISABLE = 0,
    #[doc = "1: Enable the THRE interrupts."]
    ENABLE = 1,
}
impl From<THREIE_A> for bool {
    #[inline(always)]
    fn from(variant: THREIE_A) -> Self {
        variant as u8 != 0
    }
}
impl THREIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> THREIE_A {
        match self.bits {
            false => THREIE_A::DISABLE,
            true => THREIE_A::ENABLE,
        }
    }
    #[doc = "Disable the THRE interrupts."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == THREIE_A::DISABLE
    }
    #[doc = "Enable the THRE interrupts."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == THREIE_A::ENABLE
    }
}
#[doc = "Field `THREIE` writer - THRE Interrupt Enable. Enables the THRE interrupt for UART1. The status of this interrupt can be read from LSR\\[5\\]."]
pub type THREIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, THREIE_A>;
impl<'a, REG, const O: u8> THREIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the THRE interrupts."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(THREIE_A::DISABLE)
    }
    #[doc = "Enable the THRE interrupts."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(THREIE_A::ENABLE)
    }
}
#[doc = "Field `RXIE` reader - RX Line Interrupt Enable. Enables the UART1 RX line status interrupts. The status of this interrupt can be read from LSR\\[4:1\\]."]
pub type RXIE_R = crate::BitReader<RXIE_A>;
#[doc = "RX Line Interrupt Enable. Enables the UART1 RX line status interrupts. The status of this interrupt can be read from LSR\\[4:1\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXIE_A {
    #[doc = "0: Disable the RX line status interrupts."]
    DISABLE = 0,
    #[doc = "1: Enable the RX line status interrupts."]
    ENABLE = 1,
}
impl From<RXIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXIE_A {
        match self.bits {
            false => RXIE_A::DISABLE,
            true => RXIE_A::ENABLE,
        }
    }
    #[doc = "Disable the RX line status interrupts."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RXIE_A::DISABLE
    }
    #[doc = "Enable the RX line status interrupts."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RXIE_A::ENABLE
    }
}
#[doc = "Field `RXIE` writer - RX Line Interrupt Enable. Enables the UART1 RX line status interrupts. The status of this interrupt can be read from LSR\\[4:1\\]."]
pub type RXIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RXIE_A>;
impl<'a, REG, const O: u8> RXIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the RX line status interrupts."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RXIE_A::DISABLE)
    }
    #[doc = "Enable the RX line status interrupts."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RXIE_A::ENABLE)
    }
}
#[doc = "Field `MSIE` reader - Modem Status Interrupt Enable. Enables the modem interrupt. The status of this interrupt can be read from MSR\\[3:0\\]."]
pub type MSIE_R = crate::BitReader<MSIE_A>;
#[doc = "Modem Status Interrupt Enable. Enables the modem interrupt. The status of this interrupt can be read from MSR\\[3:0\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIE_A {
    #[doc = "0: Disable the modem interrupt."]
    DISABLE = 0,
    #[doc = "1: Enable the modem interrupt."]
    ENABLE = 1,
}
impl From<MSIE_A> for bool {
    #[inline(always)]
    fn from(variant: MSIE_A) -> Self {
        variant as u8 != 0
    }
}
impl MSIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSIE_A {
        match self.bits {
            false => MSIE_A::DISABLE,
            true => MSIE_A::ENABLE,
        }
    }
    #[doc = "Disable the modem interrupt."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MSIE_A::DISABLE
    }
    #[doc = "Enable the modem interrupt."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MSIE_A::ENABLE
    }
}
#[doc = "Field `MSIE` writer - Modem Status Interrupt Enable. Enables the modem interrupt. The status of this interrupt can be read from MSR\\[3:0\\]."]
pub type MSIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MSIE_A>;
impl<'a, REG, const O: u8> MSIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the modem interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(MSIE_A::DISABLE)
    }
    #[doc = "Enable the modem interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(MSIE_A::ENABLE)
    }
}
#[doc = "Field `CTSIE` reader - CTS Interrupt Enable. If auto-cts mode is enabled this bit enables/disables the modem status interrupt generation on a CTS1 signal transition. If auto-cts mode is disabled a CTS1 transition will generate an interrupt if Modem Status Interrupt Enable (IER\\[3\\]) is set. In normal operation a CTS1 signal transition will generate a Modem Status Interrupt unless the interrupt has been disabled by clearing the IER\\[3\\]
bit in the IER register. In auto-cts mode a transition on the CTS1 bit will trigger an interrupt only if both the IER\\[3\\]
and IER\\[7\\]
bits are set."]
pub type CTSIE_R = crate::BitReader<CTSIE_A>;
#[doc = "CTS Interrupt Enable. If auto-cts mode is enabled this bit enables/disables the modem status interrupt generation on a CTS1 signal transition. If auto-cts mode is disabled a CTS1 transition will generate an interrupt if Modem Status Interrupt Enable (IER\\[3\\]) is set. In normal operation a CTS1 signal transition will generate a Modem Status Interrupt unless the interrupt has been disabled by clearing the IER\\[3\\]
bit in the IER register. In auto-cts mode a transition on the CTS1 bit will trigger an interrupt only if both the IER\\[3\\]
and IER\\[7\\]
bits are set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSIE_A {
    #[doc = "0: Disable the CTS interrupt."]
    DISABLE = 0,
    #[doc = "1: Enable the CTS interrupt."]
    ENABLE = 1,
}
impl From<CTSIE_A> for bool {
    #[inline(always)]
    fn from(variant: CTSIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CTSIE_A {
        match self.bits {
            false => CTSIE_A::DISABLE,
            true => CTSIE_A::ENABLE,
        }
    }
    #[doc = "Disable the CTS interrupt."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTSIE_A::DISABLE
    }
    #[doc = "Enable the CTS interrupt."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTSIE_A::ENABLE
    }
}
#[doc = "Field `CTSIE` writer - CTS Interrupt Enable. If auto-cts mode is enabled this bit enables/disables the modem status interrupt generation on a CTS1 signal transition. If auto-cts mode is disabled a CTS1 transition will generate an interrupt if Modem Status Interrupt Enable (IER\\[3\\]) is set. In normal operation a CTS1 signal transition will generate a Modem Status Interrupt unless the interrupt has been disabled by clearing the IER\\[3\\]
bit in the IER register. In auto-cts mode a transition on the CTS1 bit will trigger an interrupt only if both the IER\\[3\\]
and IER\\[7\\]
bits are set."]
pub type CTSIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CTSIE_A>;
impl<'a, REG, const O: u8> CTSIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the CTS interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CTSIE_A::DISABLE)
    }
    #[doc = "Enable the CTS interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CTSIE_A::ENABLE)
    }
}
#[doc = "Field `ABEOIE` reader - Enables the end of auto-baud interrupt."]
pub type ABEOIE_R = crate::BitReader<ABEOIE_A>;
#[doc = "Enables the end of auto-baud interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABEOIE_A {
    #[doc = "0: Disable end of auto-baud Interrupt."]
    DISABLE_END_OF_AUTO = 0,
    #[doc = "1: Enable end of auto-baud Interrupt."]
    ENABLE_END_OF_AUTO_B = 1,
}
impl From<ABEOIE_A> for bool {
    #[inline(always)]
    fn from(variant: ABEOIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ABEOIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ABEOIE_A {
        match self.bits {
            false => ABEOIE_A::DISABLE_END_OF_AUTO,
            true => ABEOIE_A::ENABLE_END_OF_AUTO_B,
        }
    }
    #[doc = "Disable end of auto-baud Interrupt."]
    #[inline(always)]
    pub fn is_disable_end_of_auto(&self) -> bool {
        *self == ABEOIE_A::DISABLE_END_OF_AUTO
    }
    #[doc = "Enable end of auto-baud Interrupt."]
    #[inline(always)]
    pub fn is_enable_end_of_auto_b(&self) -> bool {
        *self == ABEOIE_A::ENABLE_END_OF_AUTO_B
    }
}
#[doc = "Field `ABEOIE` writer - Enables the end of auto-baud interrupt."]
pub type ABEOIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ABEOIE_A>;
impl<'a, REG, const O: u8> ABEOIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable end of auto-baud Interrupt."]
    #[inline(always)]
    pub fn disable_end_of_auto(self) -> &'a mut crate::W<REG> {
        self.variant(ABEOIE_A::DISABLE_END_OF_AUTO)
    }
    #[doc = "Enable end of auto-baud Interrupt."]
    #[inline(always)]
    pub fn enable_end_of_auto_b(self) -> &'a mut crate::W<REG> {
        self.variant(ABEOIE_A::ENABLE_END_OF_AUTO_B)
    }
}
#[doc = "Field `ABTOIE` reader - Enables the auto-baud time-out interrupt."]
pub type ABTOIE_R = crate::BitReader<ABTOIE_A>;
#[doc = "Enables the auto-baud time-out interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABTOIE_A {
    #[doc = "0: Disable auto-baud time-out Interrupt."]
    DISABLE = 0,
    #[doc = "1: Enable auto-baud time-out Interrupt."]
    ENABLE = 1,
}
impl From<ABTOIE_A> for bool {
    #[inline(always)]
    fn from(variant: ABTOIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ABTOIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ABTOIE_A {
        match self.bits {
            false => ABTOIE_A::DISABLE,
            true => ABTOIE_A::ENABLE,
        }
    }
    #[doc = "Disable auto-baud time-out Interrupt."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ABTOIE_A::DISABLE
    }
    #[doc = "Enable auto-baud time-out Interrupt."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ABTOIE_A::ENABLE
    }
}
#[doc = "Field `ABTOIE` writer - Enables the auto-baud time-out interrupt."]
pub type ABTOIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ABTOIE_A>;
impl<'a, REG, const O: u8> ABTOIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable auto-baud time-out Interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ABTOIE_A::DISABLE)
    }
    #[doc = "Enable auto-baud time-out Interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ABTOIE_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - RBR Interrupt Enable. Enables the Receive Data Available interrupt for UART1. It also controls the Character Receive Time-out interrupt."]
    #[inline(always)]
    pub fn rbrie(&self) -> RBRIE_R {
        RBRIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - THRE Interrupt Enable. Enables the THRE interrupt for UART1. The status of this interrupt can be read from LSR\\[5\\]."]
    #[inline(always)]
    pub fn threie(&self) -> THREIE_R {
        THREIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Line Interrupt Enable. Enables the UART1 RX line status interrupts. The status of this interrupt can be read from LSR\\[4:1\\]."]
    #[inline(always)]
    pub fn rxie(&self) -> RXIE_R {
        RXIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Modem Status Interrupt Enable. Enables the modem interrupt. The status of this interrupt can be read from MSR\\[3:0\\]."]
    #[inline(always)]
    pub fn msie(&self) -> MSIE_R {
        MSIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - CTS Interrupt Enable. If auto-cts mode is enabled this bit enables/disables the modem status interrupt generation on a CTS1 signal transition. If auto-cts mode is disabled a CTS1 transition will generate an interrupt if Modem Status Interrupt Enable (IER\\[3\\]) is set. In normal operation a CTS1 signal transition will generate a Modem Status Interrupt unless the interrupt has been disabled by clearing the IER\\[3\\]
bit in the IER register. In auto-cts mode a transition on the CTS1 bit will trigger an interrupt only if both the IER\\[3\\]
and IER\\[7\\]
bits are set."]
    #[inline(always)]
    pub fn ctsie(&self) -> CTSIE_R {
        CTSIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enables the end of auto-baud interrupt."]
    #[inline(always)]
    pub fn abeoie(&self) -> ABEOIE_R {
        ABEOIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enables the auto-baud time-out interrupt."]
    #[inline(always)]
    pub fn abtoie(&self) -> ABTOIE_R {
        ABTOIE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("rbrie", &format_args!("{}", self.rbrie().bit()))
            .field("threie", &format_args!("{}", self.threie().bit()))
            .field("rxie", &format_args!("{}", self.rxie().bit()))
            .field("msie", &format_args!("{}", self.msie().bit()))
            .field("ctsie", &format_args!("{}", self.ctsie().bit()))
            .field("abeoie", &format_args!("{}", self.abeoie().bit()))
            .field("abtoie", &format_args!("{}", self.abtoie().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<IER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - RBR Interrupt Enable. Enables the Receive Data Available interrupt for UART1. It also controls the Character Receive Time-out interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rbrie(&mut self) -> RBRIE_W<IER_SPEC, 0> {
        RBRIE_W::new(self)
    }
    #[doc = "Bit 1 - THRE Interrupt Enable. Enables the THRE interrupt for UART1. The status of this interrupt can be read from LSR\\[5\\]."]
    #[inline(always)]
    #[must_use]
    pub fn threie(&mut self) -> THREIE_W<IER_SPEC, 1> {
        THREIE_W::new(self)
    }
    #[doc = "Bit 2 - RX Line Interrupt Enable. Enables the UART1 RX line status interrupts. The status of this interrupt can be read from LSR\\[4:1\\]."]
    #[inline(always)]
    #[must_use]
    pub fn rxie(&mut self) -> RXIE_W<IER_SPEC, 2> {
        RXIE_W::new(self)
    }
    #[doc = "Bit 3 - Modem Status Interrupt Enable. Enables the modem interrupt. The status of this interrupt can be read from MSR\\[3:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn msie(&mut self) -> MSIE_W<IER_SPEC, 3> {
        MSIE_W::new(self)
    }
    #[doc = "Bit 7 - CTS Interrupt Enable. If auto-cts mode is enabled this bit enables/disables the modem status interrupt generation on a CTS1 signal transition. If auto-cts mode is disabled a CTS1 transition will generate an interrupt if Modem Status Interrupt Enable (IER\\[3\\]) is set. In normal operation a CTS1 signal transition will generate a Modem Status Interrupt unless the interrupt has been disabled by clearing the IER\\[3\\]
bit in the IER register. In auto-cts mode a transition on the CTS1 bit will trigger an interrupt only if both the IER\\[3\\]
and IER\\[7\\]
bits are set."]
    #[inline(always)]
    #[must_use]
    pub fn ctsie(&mut self) -> CTSIE_W<IER_SPEC, 7> {
        CTSIE_W::new(self)
    }
    #[doc = "Bit 8 - Enables the end of auto-baud interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn abeoie(&mut self) -> ABEOIE_W<IER_SPEC, 8> {
        ABEOIE_W::new(self)
    }
    #[doc = "Bit 9 - Enables the auto-baud time-out interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn abtoie(&mut self) -> ABTOIE_W<IER_SPEC, 9> {
        ABTOIE_W::new(self)
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
#[doc = "DLAB =0. Interrupt Enable Register. Contains individual interrupt enable bits for the 7 potential UART1 interrupts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
