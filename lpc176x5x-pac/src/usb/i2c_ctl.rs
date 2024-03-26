#[doc = "Register `I2C_CTL` reader"]
pub type R = crate::R<I2C_CTL_SPEC>;
#[doc = "Register `I2C_CTL` writer"]
pub type W = crate::W<I2C_CTL_SPEC>;
#[doc = "Field `TDIE` reader - Transmit Done Interrupt Enable. This enables the TDI interrupt signalling that this I2C issued a STOP condition."]
pub type TDIE_R = crate::BitReader<TDIE_A>;
#[doc = "Transmit Done Interrupt Enable. This enables the TDI interrupt signalling that this I2C issued a STOP condition.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDIE_A {
    #[doc = "0: Disable the TDI interrupt."]
    DISABLE = 0,
    #[doc = "1: Enable the TDI interrupt."]
    ENABLE = 1,
}
impl From<TDIE_A> for bool {
    #[inline(always)]
    fn from(variant: TDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TDIE_A {
        match self.bits {
            false => TDIE_A::DISABLE,
            true => TDIE_A::ENABLE,
        }
    }
    #[doc = "Disable the TDI interrupt."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TDIE_A::DISABLE
    }
    #[doc = "Enable the TDI interrupt."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TDIE_A::ENABLE
    }
}
#[doc = "Field `TDIE` writer - Transmit Done Interrupt Enable. This enables the TDI interrupt signalling that this I2C issued a STOP condition."]
pub type TDIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TDIE_A>;
impl<'a, REG, const O: u8> TDIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the TDI interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TDIE_A::DISABLE)
    }
    #[doc = "Enable the TDI interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TDIE_A::ENABLE)
    }
}
#[doc = "Field `AFIE` reader - Transmitter Arbitration Failure Interrupt Enable. This enables the AFI interrupt which is asserted during transmission when trying to set SDA high, but the bus is driven low by another device."]
pub type AFIE_R = crate::BitReader<AFIE_A>;
#[doc = "Transmitter Arbitration Failure Interrupt Enable. This enables the AFI interrupt which is asserted during transmission when trying to set SDA high, but the bus is driven low by another device.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AFIE_A {
    #[doc = "0: Disable the AFI."]
    DISABLE = 0,
    #[doc = "1: Enable the AFI."]
    ENABLE = 1,
}
impl From<AFIE_A> for bool {
    #[inline(always)]
    fn from(variant: AFIE_A) -> Self {
        variant as u8 != 0
    }
}
impl AFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AFIE_A {
        match self.bits {
            false => AFIE_A::DISABLE,
            true => AFIE_A::ENABLE,
        }
    }
    #[doc = "Disable the AFI."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AFIE_A::DISABLE
    }
    #[doc = "Enable the AFI."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == AFIE_A::ENABLE
    }
}
#[doc = "Field `AFIE` writer - Transmitter Arbitration Failure Interrupt Enable. This enables the AFI interrupt which is asserted during transmission when trying to set SDA high, but the bus is driven low by another device."]
pub type AFIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, AFIE_A>;
impl<'a, REG, const O: u8> AFIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the AFI."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(AFIE_A::DISABLE)
    }
    #[doc = "Enable the AFI."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(AFIE_A::ENABLE)
    }
}
#[doc = "Field `NAIE` reader - Transmitter No Acknowledge Interrupt Enable. This enables the NAI interrupt signalling that transmitted byte was not acknowledged."]
pub type NAIE_R = crate::BitReader<NAIE_A>;
#[doc = "Transmitter No Acknowledge Interrupt Enable. This enables the NAI interrupt signalling that transmitted byte was not acknowledged.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NAIE_A {
    #[doc = "0: Disable the NAI."]
    DISABLE = 0,
    #[doc = "1: Enable the NAI."]
    ENABLE = 1,
}
impl From<NAIE_A> for bool {
    #[inline(always)]
    fn from(variant: NAIE_A) -> Self {
        variant as u8 != 0
    }
}
impl NAIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NAIE_A {
        match self.bits {
            false => NAIE_A::DISABLE,
            true => NAIE_A::ENABLE,
        }
    }
    #[doc = "Disable the NAI."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == NAIE_A::DISABLE
    }
    #[doc = "Enable the NAI."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == NAIE_A::ENABLE
    }
}
#[doc = "Field `NAIE` writer - Transmitter No Acknowledge Interrupt Enable. This enables the NAI interrupt signalling that transmitted byte was not acknowledged."]
pub type NAIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, NAIE_A>;
impl<'a, REG, const O: u8> NAIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the NAI."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(NAIE_A::DISABLE)
    }
    #[doc = "Enable the NAI."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(NAIE_A::ENABLE)
    }
}
#[doc = "Field `DRMIE` reader - Master Transmitter Data Request Interrupt Enable. This enables the DRMI interrupt which signals that the master transmitter has run out of data, has not issued a STOP, and is holding the SCL line low."]
pub type DRMIE_R = crate::BitReader<DRMIE_A>;
#[doc = "Master Transmitter Data Request Interrupt Enable. This enables the DRMI interrupt which signals that the master transmitter has run out of data, has not issued a STOP, and is holding the SCL line low.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRMIE_A {
    #[doc = "0: Disable the DRMI interrupt."]
    DISABLE = 0,
    #[doc = "1: Enable the DRMI interrupt."]
    ENABLE = 1,
}
impl From<DRMIE_A> for bool {
    #[inline(always)]
    fn from(variant: DRMIE_A) -> Self {
        variant as u8 != 0
    }
}
impl DRMIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DRMIE_A {
        match self.bits {
            false => DRMIE_A::DISABLE,
            true => DRMIE_A::ENABLE,
        }
    }
    #[doc = "Disable the DRMI interrupt."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DRMIE_A::DISABLE
    }
    #[doc = "Enable the DRMI interrupt."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DRMIE_A::ENABLE
    }
}
#[doc = "Field `DRMIE` writer - Master Transmitter Data Request Interrupt Enable. This enables the DRMI interrupt which signals that the master transmitter has run out of data, has not issued a STOP, and is holding the SCL line low."]
pub type DRMIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DRMIE_A>;
impl<'a, REG, const O: u8> DRMIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the DRMI interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DRMIE_A::DISABLE)
    }
    #[doc = "Enable the DRMI interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DRMIE_A::ENABLE)
    }
}
#[doc = "Field `DRSIE` reader - Slave Transmitter Data Request Interrupt Enable. This enables the DRSI interrupt which signals that the slave transmitter has run out of data and the last byte was acknowledged, so the SCL line is being held low."]
pub type DRSIE_R = crate::BitReader<DRSIE_A>;
#[doc = "Slave Transmitter Data Request Interrupt Enable. This enables the DRSI interrupt which signals that the slave transmitter has run out of data and the last byte was acknowledged, so the SCL line is being held low.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRSIE_A {
    #[doc = "0: Disable the DRSI interrupt."]
    DISABLE = 0,
    #[doc = "1: Enable the DRSI interrupt."]
    ENABLE = 1,
}
impl From<DRSIE_A> for bool {
    #[inline(always)]
    fn from(variant: DRSIE_A) -> Self {
        variant as u8 != 0
    }
}
impl DRSIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DRSIE_A {
        match self.bits {
            false => DRSIE_A::DISABLE,
            true => DRSIE_A::ENABLE,
        }
    }
    #[doc = "Disable the DRSI interrupt."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DRSIE_A::DISABLE
    }
    #[doc = "Enable the DRSI interrupt."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DRSIE_A::ENABLE
    }
}
#[doc = "Field `DRSIE` writer - Slave Transmitter Data Request Interrupt Enable. This enables the DRSI interrupt which signals that the slave transmitter has run out of data and the last byte was acknowledged, so the SCL line is being held low."]
pub type DRSIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DRSIE_A>;
impl<'a, REG, const O: u8> DRSIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the DRSI interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DRSIE_A::DISABLE)
    }
    #[doc = "Enable the DRSI interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DRSIE_A::ENABLE)
    }
}
#[doc = "Field `REFIE` reader - Receive FIFO Full Interrupt Enable. This enables the Receive FIFO Full interrupt to indicate that the receive FIFO cannot accept any more data."]
pub type REFIE_R = crate::BitReader<REFIE_A>;
#[doc = "Receive FIFO Full Interrupt Enable. This enables the Receive FIFO Full interrupt to indicate that the receive FIFO cannot accept any more data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFIE_A {
    #[doc = "0: Disable the RFFI."]
    DISABLE = 0,
    #[doc = "1: Enable the RFFI."]
    ENABLE = 1,
}
impl From<REFIE_A> for bool {
    #[inline(always)]
    fn from(variant: REFIE_A) -> Self {
        variant as u8 != 0
    }
}
impl REFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REFIE_A {
        match self.bits {
            false => REFIE_A::DISABLE,
            true => REFIE_A::ENABLE,
        }
    }
    #[doc = "Disable the RFFI."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == REFIE_A::DISABLE
    }
    #[doc = "Enable the RFFI."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == REFIE_A::ENABLE
    }
}
#[doc = "Field `REFIE` writer - Receive FIFO Full Interrupt Enable. This enables the Receive FIFO Full interrupt to indicate that the receive FIFO cannot accept any more data."]
pub type REFIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, REFIE_A>;
impl<'a, REG, const O: u8> REFIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the RFFI."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(REFIE_A::DISABLE)
    }
    #[doc = "Enable the RFFI."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(REFIE_A::ENABLE)
    }
}
#[doc = "Field `RFDAIE` reader - Receive Data Available Interrupt Enable. This enables the DAI interrupt to indicate that data is available in the receive FIFO (i.e. not empty)."]
pub type RFDAIE_R = crate::BitReader<RFDAIE_A>;
#[doc = "Receive Data Available Interrupt Enable. This enables the DAI interrupt to indicate that data is available in the receive FIFO (i.e. not empty).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFDAIE_A {
    #[doc = "0: Disable the DAI."]
    DISABLE = 0,
    #[doc = "1: Enable the DAI."]
    ENABLE = 1,
}
impl From<RFDAIE_A> for bool {
    #[inline(always)]
    fn from(variant: RFDAIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RFDAIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RFDAIE_A {
        match self.bits {
            false => RFDAIE_A::DISABLE,
            true => RFDAIE_A::ENABLE,
        }
    }
    #[doc = "Disable the DAI."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RFDAIE_A::DISABLE
    }
    #[doc = "Enable the DAI."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RFDAIE_A::ENABLE
    }
}
#[doc = "Field `RFDAIE` writer - Receive Data Available Interrupt Enable. This enables the DAI interrupt to indicate that data is available in the receive FIFO (i.e. not empty)."]
pub type RFDAIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RFDAIE_A>;
impl<'a, REG, const O: u8> RFDAIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the DAI."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RFDAIE_A::DISABLE)
    }
    #[doc = "Enable the DAI."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RFDAIE_A::ENABLE)
    }
}
#[doc = "Field `TFFIE` reader - Transmit FIFO Not Full Interrupt Enable. This enables the Transmit FIFO Not Full interrupt to indicate that the more data can be written to the transmit FIFO. Note that this is not full. It is intended help the CPU to write to the I2C block only when there is room in the FIFO and do this without polling the status register."]
pub type TFFIE_R = crate::BitReader<TFFIE_A>;
#[doc = "Transmit FIFO Not Full Interrupt Enable. This enables the Transmit FIFO Not Full interrupt to indicate that the more data can be written to the transmit FIFO. Note that this is not full. It is intended help the CPU to write to the I2C block only when there is room in the FIFO and do this without polling the status register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFFIE_A {
    #[doc = "0: Disable the TFFI."]
    DISABLE = 0,
    #[doc = "1: Enable the TFFI."]
    ENABLE = 1,
}
impl From<TFFIE_A> for bool {
    #[inline(always)]
    fn from(variant: TFFIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TFFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TFFIE_A {
        match self.bits {
            false => TFFIE_A::DISABLE,
            true => TFFIE_A::ENABLE,
        }
    }
    #[doc = "Disable the TFFI."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TFFIE_A::DISABLE
    }
    #[doc = "Enable the TFFI."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TFFIE_A::ENABLE
    }
}
#[doc = "Field `TFFIE` writer - Transmit FIFO Not Full Interrupt Enable. This enables the Transmit FIFO Not Full interrupt to indicate that the more data can be written to the transmit FIFO. Note that this is not full. It is intended help the CPU to write to the I2C block only when there is room in the FIFO and do this without polling the status register."]
pub type TFFIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TFFIE_A>;
impl<'a, REG, const O: u8> TFFIE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the TFFI."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TFFIE_A::DISABLE)
    }
    #[doc = "Enable the TFFI."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TFFIE_A::ENABLE)
    }
}
#[doc = "Field `SRST` reader - Soft reset. This is only needed in unusual circumstances. If a device issues a start condition without issuing a stop condition. A system timer may be used to reset the I2C if the bus remains busy longer than the time-out period. On a soft reset, the Tx and Rx FIFOs are flushed, I2C_STS register is cleared, and all internal state machines are reset to appear idle. The I2C_CLKHI, I2C_CLKLO and I2C_CTL (except Soft Reset Bit) are NOT modified by a soft reset."]
pub type SRST_R = crate::BitReader<SRST_A>;
#[doc = "Soft reset. This is only needed in unusual circumstances. If a device issues a start condition without issuing a stop condition. A system timer may be used to reset the I2C if the bus remains busy longer than the time-out period. On a soft reset, the Tx and Rx FIFOs are flushed, I2C_STS register is cleared, and all internal state machines are reset to appear idle. The I2C_CLKHI, I2C_CLKLO and I2C_CTL (except Soft Reset Bit) are NOT modified by a soft reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRST_A {
    #[doc = "0: No reset."]
    NO_RESET = 0,
    #[doc = "1: Reset the I2C to idle state. Self clearing."]
    RESET = 1,
}
impl From<SRST_A> for bool {
    #[inline(always)]
    fn from(variant: SRST_A) -> Self {
        variant as u8 != 0
    }
}
impl SRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRST_A {
        match self.bits {
            false => SRST_A::NO_RESET,
            true => SRST_A::RESET,
        }
    }
    #[doc = "No reset."]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == SRST_A::NO_RESET
    }
    #[doc = "Reset the I2C to idle state. Self clearing."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SRST_A::RESET
    }
}
#[doc = "Field `SRST` writer - Soft reset. This is only needed in unusual circumstances. If a device issues a start condition without issuing a stop condition. A system timer may be used to reset the I2C if the bus remains busy longer than the time-out period. On a soft reset, the Tx and Rx FIFOs are flushed, I2C_STS register is cleared, and all internal state machines are reset to appear idle. The I2C_CLKHI, I2C_CLKLO and I2C_CTL (except Soft Reset Bit) are NOT modified by a soft reset."]
pub type SRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SRST_A>;
impl<'a, REG, const O: u8> SRST_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No reset."]
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut crate::W<REG> {
        self.variant(SRST_A::NO_RESET)
    }
    #[doc = "Reset the I2C to idle state. Self clearing."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SRST_A::RESET)
    }
}
impl R {
    #[doc = "Bit 0 - Transmit Done Interrupt Enable. This enables the TDI interrupt signalling that this I2C issued a STOP condition."]
    #[inline(always)]
    pub fn tdie(&self) -> TDIE_R {
        TDIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmitter Arbitration Failure Interrupt Enable. This enables the AFI interrupt which is asserted during transmission when trying to set SDA high, but the bus is driven low by another device."]
    #[inline(always)]
    pub fn afie(&self) -> AFIE_R {
        AFIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmitter No Acknowledge Interrupt Enable. This enables the NAI interrupt signalling that transmitted byte was not acknowledged."]
    #[inline(always)]
    pub fn naie(&self) -> NAIE_R {
        NAIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Master Transmitter Data Request Interrupt Enable. This enables the DRMI interrupt which signals that the master transmitter has run out of data, has not issued a STOP, and is holding the SCL line low."]
    #[inline(always)]
    pub fn drmie(&self) -> DRMIE_R {
        DRMIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Slave Transmitter Data Request Interrupt Enable. This enables the DRSI interrupt which signals that the slave transmitter has run out of data and the last byte was acknowledged, so the SCL line is being held low."]
    #[inline(always)]
    pub fn drsie(&self) -> DRSIE_R {
        DRSIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO Full Interrupt Enable. This enables the Receive FIFO Full interrupt to indicate that the receive FIFO cannot accept any more data."]
    #[inline(always)]
    pub fn refie(&self) -> REFIE_R {
        REFIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Data Available Interrupt Enable. This enables the DAI interrupt to indicate that data is available in the receive FIFO (i.e. not empty)."]
    #[inline(always)]
    pub fn rfdaie(&self) -> RFDAIE_R {
        RFDAIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO Not Full Interrupt Enable. This enables the Transmit FIFO Not Full interrupt to indicate that the more data can be written to the transmit FIFO. Note that this is not full. It is intended help the CPU to write to the I2C block only when there is room in the FIFO and do this without polling the status register."]
    #[inline(always)]
    pub fn tffie(&self) -> TFFIE_R {
        TFFIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Soft reset. This is only needed in unusual circumstances. If a device issues a start condition without issuing a stop condition. A system timer may be used to reset the I2C if the bus remains busy longer than the time-out period. On a soft reset, the Tx and Rx FIFOs are flushed, I2C_STS register is cleared, and all internal state machines are reset to appear idle. The I2C_CLKHI, I2C_CLKLO and I2C_CTL (except Soft Reset Bit) are NOT modified by a soft reset."]
    #[inline(always)]
    pub fn srst(&self) -> SRST_R {
        SRST_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_CTL")
            .field("tdie", &format_args!("{}", self.tdie().bit()))
            .field("afie", &format_args!("{}", self.afie().bit()))
            .field("naie", &format_args!("{}", self.naie().bit()))
            .field("drmie", &format_args!("{}", self.drmie().bit()))
            .field("drsie", &format_args!("{}", self.drsie().bit()))
            .field("refie", &format_args!("{}", self.refie().bit()))
            .field("rfdaie", &format_args!("{}", self.rfdaie().bit()))
            .field("tffie", &format_args!("{}", self.tffie().bit()))
            .field("srst", &format_args!("{}", self.srst().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<I2C_CTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Done Interrupt Enable. This enables the TDI interrupt signalling that this I2C issued a STOP condition."]
    #[inline(always)]
    #[must_use]
    pub fn tdie(&mut self) -> TDIE_W<I2C_CTL_SPEC, 0> {
        TDIE_W::new(self)
    }
    #[doc = "Bit 1 - Transmitter Arbitration Failure Interrupt Enable. This enables the AFI interrupt which is asserted during transmission when trying to set SDA high, but the bus is driven low by another device."]
    #[inline(always)]
    #[must_use]
    pub fn afie(&mut self) -> AFIE_W<I2C_CTL_SPEC, 1> {
        AFIE_W::new(self)
    }
    #[doc = "Bit 2 - Transmitter No Acknowledge Interrupt Enable. This enables the NAI interrupt signalling that transmitted byte was not acknowledged."]
    #[inline(always)]
    #[must_use]
    pub fn naie(&mut self) -> NAIE_W<I2C_CTL_SPEC, 2> {
        NAIE_W::new(self)
    }
    #[doc = "Bit 3 - Master Transmitter Data Request Interrupt Enable. This enables the DRMI interrupt which signals that the master transmitter has run out of data, has not issued a STOP, and is holding the SCL line low."]
    #[inline(always)]
    #[must_use]
    pub fn drmie(&mut self) -> DRMIE_W<I2C_CTL_SPEC, 3> {
        DRMIE_W::new(self)
    }
    #[doc = "Bit 4 - Slave Transmitter Data Request Interrupt Enable. This enables the DRSI interrupt which signals that the slave transmitter has run out of data and the last byte was acknowledged, so the SCL line is being held low."]
    #[inline(always)]
    #[must_use]
    pub fn drsie(&mut self) -> DRSIE_W<I2C_CTL_SPEC, 4> {
        DRSIE_W::new(self)
    }
    #[doc = "Bit 5 - Receive FIFO Full Interrupt Enable. This enables the Receive FIFO Full interrupt to indicate that the receive FIFO cannot accept any more data."]
    #[inline(always)]
    #[must_use]
    pub fn refie(&mut self) -> REFIE_W<I2C_CTL_SPEC, 5> {
        REFIE_W::new(self)
    }
    #[doc = "Bit 6 - Receive Data Available Interrupt Enable. This enables the DAI interrupt to indicate that data is available in the receive FIFO (i.e. not empty)."]
    #[inline(always)]
    #[must_use]
    pub fn rfdaie(&mut self) -> RFDAIE_W<I2C_CTL_SPEC, 6> {
        RFDAIE_W::new(self)
    }
    #[doc = "Bit 7 - Transmit FIFO Not Full Interrupt Enable. This enables the Transmit FIFO Not Full interrupt to indicate that the more data can be written to the transmit FIFO. Note that this is not full. It is intended help the CPU to write to the I2C block only when there is room in the FIFO and do this without polling the status register."]
    #[inline(always)]
    #[must_use]
    pub fn tffie(&mut self) -> TFFIE_W<I2C_CTL_SPEC, 7> {
        TFFIE_W::new(self)
    }
    #[doc = "Bit 8 - Soft reset. This is only needed in unusual circumstances. If a device issues a start condition without issuing a stop condition. A system timer may be used to reset the I2C if the bus remains busy longer than the time-out period. On a soft reset, the Tx and Rx FIFOs are flushed, I2C_STS register is cleared, and all internal state machines are reset to appear idle. The I2C_CLKHI, I2C_CLKLO and I2C_CTL (except Soft Reset Bit) are NOT modified by a soft reset."]
    #[inline(always)]
    #[must_use]
    pub fn srst(&mut self) -> SRST_W<I2C_CTL_SPEC, 8> {
        SRST_W::new(self)
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
#[doc = "I2C Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_CTL_SPEC;
impl crate::RegisterSpec for I2C_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_ctl::R`](R) reader structure"]
impl crate::Readable for I2C_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c_ctl::W`](W) writer structure"]
impl crate::Writable for I2C_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2C_CTL to value 0"]
impl crate::Resettable for I2C_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
