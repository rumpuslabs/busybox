#[doc = "Register `I2C_STS` reader"]
pub type R = crate::R<I2C_STS_SPEC>;
#[doc = "Field `TDI` reader - Transaction Done Interrupt. This flag is set if a transaction completes successfully. It is cleared by writing a one to bit 0 of the status register. It is unaffected by slave transactions."]
pub type TDI_R = crate::BitReader<TDI_A>;
#[doc = "Transaction Done Interrupt. This flag is set if a transaction completes successfully. It is cleared by writing a one to bit 0 of the status register. It is unaffected by slave transactions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDI_A {
    #[doc = "0: Transaction has not completed."]
    NOT_COMPLETE = 0,
    #[doc = "1: Transaction completed."]
    COMPLETE = 1,
}
impl From<TDI_A> for bool {
    #[inline(always)]
    fn from(variant: TDI_A) -> Self {
        variant as u8 != 0
    }
}
impl TDI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TDI_A {
        match self.bits {
            false => TDI_A::NOT_COMPLETE,
            true => TDI_A::COMPLETE,
        }
    }
    #[doc = "Transaction has not completed."]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == TDI_A::NOT_COMPLETE
    }
    #[doc = "Transaction completed."]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == TDI_A::COMPLETE
    }
}
#[doc = "Field `AFI` reader - Arbitration Failure Interrupt. When transmitting, if the SDA is low when SDAOUT is high, then this I2C has lost the arbitration to another device on the bus. The Arbitration Failure bit is set when this happens. It is cleared by writing a one to bit 1 of the status register."]
pub type AFI_R = crate::BitReader<AFI_A>;
#[doc = "Arbitration Failure Interrupt. When transmitting, if the SDA is low when SDAOUT is high, then this I2C has lost the arbitration to another device on the bus. The Arbitration Failure bit is set when this happens. It is cleared by writing a one to bit 1 of the status register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AFI_A {
    #[doc = "0: No arbitration failure on last transmission."]
    NO_ARBITRATION_FAILURE = 0,
    #[doc = "1: Arbitration failure occurred on last transmission."]
    ARBITRATION_FAILURE = 1,
}
impl From<AFI_A> for bool {
    #[inline(always)]
    fn from(variant: AFI_A) -> Self {
        variant as u8 != 0
    }
}
impl AFI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AFI_A {
        match self.bits {
            false => AFI_A::NO_ARBITRATION_FAILURE,
            true => AFI_A::ARBITRATION_FAILURE,
        }
    }
    #[doc = "No arbitration failure on last transmission."]
    #[inline(always)]
    pub fn is_no_arbitration_failure(&self) -> bool {
        *self == AFI_A::NO_ARBITRATION_FAILURE
    }
    #[doc = "Arbitration failure occurred on last transmission."]
    #[inline(always)]
    pub fn is_arbitration_failure(&self) -> bool {
        *self == AFI_A::ARBITRATION_FAILURE
    }
}
#[doc = "Field `NAI` reader - No Acknowledge Interrupt. After every byte of data is sent, the transmitter expects an acknowledge from the receiver. This bit is set if the acknowledge is not received. It is cleared when a byte is written to the master TX FIFO."]
pub type NAI_R = crate::BitReader<NAI_A>;
#[doc = "No Acknowledge Interrupt. After every byte of data is sent, the transmitter expects an acknowledge from the receiver. This bit is set if the acknowledge is not received. It is cleared when a byte is written to the master TX FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NAI_A {
    #[doc = "0: Last transmission received an acknowledge."]
    ACKNOWLEDGE_RCVD = 0,
    #[doc = "1: Last transmission did not receive an acknowledge."]
    NO_ACKNOWLEDGE_RCVD = 1,
}
impl From<NAI_A> for bool {
    #[inline(always)]
    fn from(variant: NAI_A) -> Self {
        variant as u8 != 0
    }
}
impl NAI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NAI_A {
        match self.bits {
            false => NAI_A::ACKNOWLEDGE_RCVD,
            true => NAI_A::NO_ACKNOWLEDGE_RCVD,
        }
    }
    #[doc = "Last transmission received an acknowledge."]
    #[inline(always)]
    pub fn is_acknowledge_rcvd(&self) -> bool {
        *self == NAI_A::ACKNOWLEDGE_RCVD
    }
    #[doc = "Last transmission did not receive an acknowledge."]
    #[inline(always)]
    pub fn is_no_acknowledge_rcvd(&self) -> bool {
        *self == NAI_A::NO_ACKNOWLEDGE_RCVD
    }
}
#[doc = "Field `DRMI` reader - Master Data Request Interrupt. Once a transmission is started, the transmitter must have data to transmit as long as it isn't followed by a stop condition or it will hold SCL low until more data is available. The Master Data Request bit is set when the master transmitter is data-starved. If the master TX FIFO is empty and the last byte did not have a STOP condition flag, then SCL is held low until the CPU writes another byte to transmit. This bit is cleared when a byte is written to the master TX FIFO."]
pub type DRMI_R = crate::BitReader<DRMI_A>;
#[doc = "Master Data Request Interrupt. Once a transmission is started, the transmitter must have data to transmit as long as it isn't followed by a stop condition or it will hold SCL low until more data is available. The Master Data Request bit is set when the master transmitter is data-starved. If the master TX FIFO is empty and the last byte did not have a STOP condition flag, then SCL is held low until the CPU writes another byte to transmit. This bit is cleared when a byte is written to the master TX FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRMI_A {
    #[doc = "0: Master transmitter does not need data."]
    BUSY = 0,
    #[doc = "1: Master transmitter needs data."]
    NEED_DATA = 1,
}
impl From<DRMI_A> for bool {
    #[inline(always)]
    fn from(variant: DRMI_A) -> Self {
        variant as u8 != 0
    }
}
impl DRMI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DRMI_A {
        match self.bits {
            false => DRMI_A::BUSY,
            true => DRMI_A::NEED_DATA,
        }
    }
    #[doc = "Master transmitter does not need data."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == DRMI_A::BUSY
    }
    #[doc = "Master transmitter needs data."]
    #[inline(always)]
    pub fn is_need_data(&self) -> bool {
        *self == DRMI_A::NEED_DATA
    }
}
#[doc = "Field `DRSI` reader - Slave Data Request Interrupt. Once a transmission is started, the transmitter must have data to transmit as long as it isn't followed by a STOP condition or it will hold SCL low until more data is available. The Slave Data Request bit is set when the slave transmitter is data-starved. If the slave TX FIFO is empty and the last byte transmitted was acknowledged, then SCL is held low until the CPU writes another byte to transmit. This bit is cleared when a byte is written to the slave Tx FIFO."]
pub type DRSI_R = crate::BitReader<DRSI_A>;
#[doc = "Slave Data Request Interrupt. Once a transmission is started, the transmitter must have data to transmit as long as it isn't followed by a STOP condition or it will hold SCL low until more data is available. The Slave Data Request bit is set when the slave transmitter is data-starved. If the slave TX FIFO is empty and the last byte transmitted was acknowledged, then SCL is held low until the CPU writes another byte to transmit. This bit is cleared when a byte is written to the slave Tx FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRSI_A {
    #[doc = "0: Slave transmitter does not need data."]
    BUSY = 0,
    #[doc = "1: Slave transmitter needs data."]
    NEED_DATA = 1,
}
impl From<DRSI_A> for bool {
    #[inline(always)]
    fn from(variant: DRSI_A) -> Self {
        variant as u8 != 0
    }
}
impl DRSI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DRSI_A {
        match self.bits {
            false => DRSI_A::BUSY,
            true => DRSI_A::NEED_DATA,
        }
    }
    #[doc = "Slave transmitter does not need data."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == DRSI_A::BUSY
    }
    #[doc = "Slave transmitter needs data."]
    #[inline(always)]
    pub fn is_need_data(&self) -> bool {
        *self == DRSI_A::NEED_DATA
    }
}
#[doc = "Field `Active` reader - Indicates whether the bus is busy. This bit is set when a START condition has been seen. It is cleared when a STOP condition is seen.."]
pub type ACTIVE_R = crate::BitReader;
#[doc = "Field `SCL` reader - The current value of the SCL signal."]
pub type SCL_R = crate::BitReader;
#[doc = "Field `SDA` reader - The current value of the SDA signal."]
pub type SDA_R = crate::BitReader;
#[doc = "Field `RFF` reader - Receive FIFO Full (RFF). This bit is set when the RX FIFO is full and cannot accept any more data. It is cleared when the RX FIFO is not full. If a byte arrives when the Receive FIFO is full, the SCL is held low until the CPU reads the RX FIFO and makes room for it."]
pub type RFF_R = crate::BitReader<RFF_A>;
#[doc = "Receive FIFO Full (RFF). This bit is set when the RX FIFO is full and cannot accept any more data. It is cleared when the RX FIFO is not full. If a byte arrives when the Receive FIFO is full, the SCL is held low until the CPU reads the RX FIFO and makes room for it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFF_A {
    #[doc = "0: RX FIFO is not full"]
    NOT_FULL = 0,
    #[doc = "1: RX FIFO is full"]
    FULL = 1,
}
impl From<RFF_A> for bool {
    #[inline(always)]
    fn from(variant: RFF_A) -> Self {
        variant as u8 != 0
    }
}
impl RFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RFF_A {
        match self.bits {
            false => RFF_A::NOT_FULL,
            true => RFF_A::FULL,
        }
    }
    #[doc = "RX FIFO is not full"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == RFF_A::NOT_FULL
    }
    #[doc = "RX FIFO is full"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == RFF_A::FULL
    }
}
#[doc = "Field `RFE` reader - Receive FIFO Empty. RFE is set when the RX FIFO is empty and is cleared when the RX FIFO contains valid data."]
pub type RFE_R = crate::BitReader<RFE_A>;
#[doc = "Receive FIFO Empty. RFE is set when the RX FIFO is empty and is cleared when the RX FIFO contains valid data.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFE_A {
    #[doc = "0: RX FIFO contains data."]
    DATA = 0,
    #[doc = "1: RX FIFO is empty"]
    EMPTY = 1,
}
impl From<RFE_A> for bool {
    #[inline(always)]
    fn from(variant: RFE_A) -> Self {
        variant as u8 != 0
    }
}
impl RFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RFE_A {
        match self.bits {
            false => RFE_A::DATA,
            true => RFE_A::EMPTY,
        }
    }
    #[doc = "RX FIFO contains data."]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == RFE_A::DATA
    }
    #[doc = "RX FIFO is empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RFE_A::EMPTY
    }
}
#[doc = "Field `TFF` reader - Transmit FIFO Full. TFF is set when the TX FIFO is full and is cleared when the TX FIFO is not full."]
pub type TFF_R = crate::BitReader<TFF_A>;
#[doc = "Transmit FIFO Full. TFF is set when the TX FIFO is full and is cleared when the TX FIFO is not full.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFF_A {
    #[doc = "0: TX FIFO is not full."]
    NOT_FULL = 0,
    #[doc = "1: TX FIFO is full"]
    FULL = 1,
}
impl From<TFF_A> for bool {
    #[inline(always)]
    fn from(variant: TFF_A) -> Self {
        variant as u8 != 0
    }
}
impl TFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TFF_A {
        match self.bits {
            false => TFF_A::NOT_FULL,
            true => TFF_A::FULL,
        }
    }
    #[doc = "TX FIFO is not full."]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == TFF_A::NOT_FULL
    }
    #[doc = "TX FIFO is full"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == TFF_A::FULL
    }
}
#[doc = "Field `TFE` reader - Transmit FIFO Empty. TFE is set when the TX FIFO is empty and is cleared when the TX FIFO contains valid data."]
pub type TFE_R = crate::BitReader<TFE_A>;
#[doc = "Transmit FIFO Empty. TFE is set when the TX FIFO is empty and is cleared when the TX FIFO contains valid data.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFE_A {
    #[doc = "0: TX FIFO contains valid data."]
    VALID_DATA = 0,
    #[doc = "1: TX FIFO is empty"]
    EMPTY = 1,
}
impl From<TFE_A> for bool {
    #[inline(always)]
    fn from(variant: TFE_A) -> Self {
        variant as u8 != 0
    }
}
impl TFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TFE_A {
        match self.bits {
            false => TFE_A::VALID_DATA,
            true => TFE_A::EMPTY,
        }
    }
    #[doc = "TX FIFO contains valid data."]
    #[inline(always)]
    pub fn is_valid_data(&self) -> bool {
        *self == TFE_A::VALID_DATA
    }
    #[doc = "TX FIFO is empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TFE_A::EMPTY
    }
}
impl R {
    #[doc = "Bit 0 - Transaction Done Interrupt. This flag is set if a transaction completes successfully. It is cleared by writing a one to bit 0 of the status register. It is unaffected by slave transactions."]
    #[inline(always)]
    pub fn tdi(&self) -> TDI_R {
        TDI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Arbitration Failure Interrupt. When transmitting, if the SDA is low when SDAOUT is high, then this I2C has lost the arbitration to another device on the bus. The Arbitration Failure bit is set when this happens. It is cleared by writing a one to bit 1 of the status register."]
    #[inline(always)]
    pub fn afi(&self) -> AFI_R {
        AFI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - No Acknowledge Interrupt. After every byte of data is sent, the transmitter expects an acknowledge from the receiver. This bit is set if the acknowledge is not received. It is cleared when a byte is written to the master TX FIFO."]
    #[inline(always)]
    pub fn nai(&self) -> NAI_R {
        NAI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Master Data Request Interrupt. Once a transmission is started, the transmitter must have data to transmit as long as it isn't followed by a stop condition or it will hold SCL low until more data is available. The Master Data Request bit is set when the master transmitter is data-starved. If the master TX FIFO is empty and the last byte did not have a STOP condition flag, then SCL is held low until the CPU writes another byte to transmit. This bit is cleared when a byte is written to the master TX FIFO."]
    #[inline(always)]
    pub fn drmi(&self) -> DRMI_R {
        DRMI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Slave Data Request Interrupt. Once a transmission is started, the transmitter must have data to transmit as long as it isn't followed by a STOP condition or it will hold SCL low until more data is available. The Slave Data Request bit is set when the slave transmitter is data-starved. If the slave TX FIFO is empty and the last byte transmitted was acknowledged, then SCL is held low until the CPU writes another byte to transmit. This bit is cleared when a byte is written to the slave Tx FIFO."]
    #[inline(always)]
    pub fn drsi(&self) -> DRSI_R {
        DRSI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates whether the bus is busy. This bit is set when a START condition has been seen. It is cleared when a STOP condition is seen.."]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The current value of the SCL signal."]
    #[inline(always)]
    pub fn scl(&self) -> SCL_R {
        SCL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The current value of the SDA signal."]
    #[inline(always)]
    pub fn sda(&self) -> SDA_R {
        SDA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive FIFO Full (RFF). This bit is set when the RX FIFO is full and cannot accept any more data. It is cleared when the RX FIFO is not full. If a byte arrives when the Receive FIFO is full, the SCL is held low until the CPU reads the RX FIFO and makes room for it."]
    #[inline(always)]
    pub fn rff(&self) -> RFF_R {
        RFF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive FIFO Empty. RFE is set when the RX FIFO is empty and is cleared when the RX FIFO contains valid data."]
    #[inline(always)]
    pub fn rfe(&self) -> RFE_R {
        RFE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmit FIFO Full. TFF is set when the TX FIFO is full and is cleared when the TX FIFO is not full."]
    #[inline(always)]
    pub fn tff(&self) -> TFF_R {
        TFF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmit FIFO Empty. TFE is set when the TX FIFO is empty and is cleared when the TX FIFO contains valid data."]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_STS")
            .field("tdi", &format_args!("{}", self.tdi().bit()))
            .field("afi", &format_args!("{}", self.afi().bit()))
            .field("nai", &format_args!("{}", self.nai().bit()))
            .field("drmi", &format_args!("{}", self.drmi().bit()))
            .field("drsi", &format_args!("{}", self.drsi().bit()))
            .field("active", &format_args!("{}", self.active().bit()))
            .field("scl", &format_args!("{}", self.scl().bit()))
            .field("sda", &format_args!("{}", self.sda().bit()))
            .field("rff", &format_args!("{}", self.rff().bit()))
            .field("rfe", &format_args!("{}", self.rfe().bit()))
            .field("tff", &format_args!("{}", self.tff().bit()))
            .field("tfe", &format_args!("{}", self.tfe().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<I2C_STS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "I2C Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_sts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_STS_SPEC;
impl crate::RegisterSpec for I2C_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_sts::R`](R) reader structure"]
impl crate::Readable for I2C_STS_SPEC {}
#[doc = "`reset()` method sets I2C_STS to value 0x0a00"]
impl crate::Resettable for I2C_STS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a00;
}
