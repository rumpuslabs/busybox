#[doc = "Register `GSR` reader"]
pub type R = crate::R<GSR_SPEC>;
#[doc = "Field `RBS` reader - Receive Buffer Status. After reading all messages and releasing their memory space with the command 'Release Receive Buffer,' this bit is cleared."]
pub type RBS_R = crate::BitReader<RBS_A>;
#[doc = "Receive Buffer Status. After reading all messages and releasing their memory space with the command 'Release Receive Buffer,' this bit is cleared.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RBS_A {
    #[doc = "0: Empty. No message is available."]
    EMPTY = 0,
    #[doc = "1: Full. At least one complete message is received by the Double Receive Buffer and available in the CANxRFS, CANxRID, and if applicable the CANxRDA and CANxRDB registers. This bit is cleared by the Release Receive Buffer command in CANxCMR, if no subsequent received message is available."]
    FULL = 1,
}
impl From<RBS_A> for bool {
    #[inline(always)]
    fn from(variant: RBS_A) -> Self {
        variant as u8 != 0
    }
}
impl RBS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RBS_A {
        match self.bits {
            false => RBS_A::EMPTY,
            true => RBS_A::FULL,
        }
    }
    #[doc = "Empty. No message is available."]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RBS_A::EMPTY
    }
    #[doc = "Full. At least one complete message is received by the Double Receive Buffer and available in the CANxRFS, CANxRID, and if applicable the CANxRDA and CANxRDB registers. This bit is cleared by the Release Receive Buffer command in CANxCMR, if no subsequent received message is available."]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == RBS_A::FULL
    }
}
#[doc = "Field `DOS` reader - Data Overrun Status. If there is not enough space to store the message within the Receive Buffer, that message is dropped and the Data Overrun condition is signalled to the CPU in the moment this message becomes valid. If this message is not completed successfully (e.g. because of an error), no overrun condition is signalled."]
pub type DOS_R = crate::BitReader<DOS_A>;
#[doc = "Data Overrun Status. If there is not enough space to store the message within the Receive Buffer, that message is dropped and the Data Overrun condition is signalled to the CPU in the moment this message becomes valid. If this message is not completed successfully (e.g. because of an error), no overrun condition is signalled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOS_A {
    #[doc = "0: Absent. No data overrun has occurred since the last Clear Data Overrun command was given/written to CANxCMR (or since Reset)."]
    ABSENT = 0,
    #[doc = "1: Overrun. A message was lost because the preceding message to this CAN controller was not read and released quickly enough (there was not enough space for a new message in the Double Receive Buffer)."]
    OVERRUN = 1,
}
impl From<DOS_A> for bool {
    #[inline(always)]
    fn from(variant: DOS_A) -> Self {
        variant as u8 != 0
    }
}
impl DOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DOS_A {
        match self.bits {
            false => DOS_A::ABSENT,
            true => DOS_A::OVERRUN,
        }
    }
    #[doc = "Absent. No data overrun has occurred since the last Clear Data Overrun command was given/written to CANxCMR (or since Reset)."]
    #[inline(always)]
    pub fn is_absent(&self) -> bool {
        *self == DOS_A::ABSENT
    }
    #[doc = "Overrun. A message was lost because the preceding message to this CAN controller was not read and released quickly enough (there was not enough space for a new message in the Double Receive Buffer)."]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == DOS_A::OVERRUN
    }
}
#[doc = "Field `TBS` reader - Transmit Buffer Status."]
pub type TBS_R = crate::BitReader<TBS_A>;
#[doc = "Transmit Buffer Status.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBS_A {
    #[doc = "0: Locked. At least one of the Transmit Buffers is not available for the CPU, i.e. at least one previously queued message for this CAN controller has not yet been sent, and therefore software should not write to the CANxTFI, CANxTID, CANxTDA, nor CANxTDB registers of that (those) Tx buffer(s)."]
    LOCKED = 0,
    #[doc = "1: Released. All three Transmit Buffers are available for the CPU. No transmit message is pending for this CAN controller (in any of the 3 Tx buffers), and software may write to any of the CANxTFI, CANxTID, CANxTDA, and CANxTDB registers."]
    RELEASED = 1,
}
impl From<TBS_A> for bool {
    #[inline(always)]
    fn from(variant: TBS_A) -> Self {
        variant as u8 != 0
    }
}
impl TBS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TBS_A {
        match self.bits {
            false => TBS_A::LOCKED,
            true => TBS_A::RELEASED,
        }
    }
    #[doc = "Locked. At least one of the Transmit Buffers is not available for the CPU, i.e. at least one previously queued message for this CAN controller has not yet been sent, and therefore software should not write to the CANxTFI, CANxTID, CANxTDA, nor CANxTDB registers of that (those) Tx buffer(s)."]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == TBS_A::LOCKED
    }
    #[doc = "Released. All three Transmit Buffers are available for the CPU. No transmit message is pending for this CAN controller (in any of the 3 Tx buffers), and software may write to any of the CANxTFI, CANxTID, CANxTDA, and CANxTDB registers."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == TBS_A::RELEASED
    }
}
#[doc = "Field `TCS` reader - Transmit Complete Status. The Transmission Complete Status bit is set '0' (incomplete) whenever the Transmission Request bit or the Self Reception Request bit is set '1' at least for one of the three Transmit Buffers. The Transmission Complete Status bit will remain '0' until all messages are transmitted successfully."]
pub type TCS_R = crate::BitReader<TCS_A>;
#[doc = "Transmit Complete Status. The Transmission Complete Status bit is set '0' (incomplete) whenever the Transmission Request bit or the Self Reception Request bit is set '1' at least for one of the three Transmit Buffers. The Transmission Complete Status bit will remain '0' until all messages are transmitted successfully.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCS_A {
    #[doc = "0: Incomplete. At least one requested transmission has not been successfully completed yet."]
    INCOMPLETE = 0,
    #[doc = "1: Complete. All requested transmission(s) has (have) been successfully completed."]
    COMPLETE = 1,
}
impl From<TCS_A> for bool {
    #[inline(always)]
    fn from(variant: TCS_A) -> Self {
        variant as u8 != 0
    }
}
impl TCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCS_A {
        match self.bits {
            false => TCS_A::INCOMPLETE,
            true => TCS_A::COMPLETE,
        }
    }
    #[doc = "Incomplete. At least one requested transmission has not been successfully completed yet."]
    #[inline(always)]
    pub fn is_incomplete(&self) -> bool {
        *self == TCS_A::INCOMPLETE
    }
    #[doc = "Complete. All requested transmission(s) has (have) been successfully completed."]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == TCS_A::COMPLETE
    }
}
#[doc = "Field `RS` reader - Receive Status. If both the Receive Status and the Transmit Status bits are '0' (idle), the CAN-Bus is idle. If both bits are set, the controller is waiting to become idle again. After hardware reset 11 consecutive recessive bits have to be detected until idle status is reached. After Bus-off this will take 128 times of 11 consecutive recessive bits."]
pub type RS_R = crate::BitReader<RS_A>;
#[doc = "Receive Status. If both the Receive Status and the Transmit Status bits are '0' (idle), the CAN-Bus is idle. If both bits are set, the controller is waiting to become idle again. After hardware reset 11 consecutive recessive bits have to be detected until idle status is reached. After Bus-off this will take 128 times of 11 consecutive recessive bits.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RS_A {
    #[doc = "0: Idle. The CAN controller is idle."]
    IDLE = 0,
    #[doc = "1: Receive. The CAN controller is receiving a message."]
    RECEIVE = 1,
}
impl From<RS_A> for bool {
    #[inline(always)]
    fn from(variant: RS_A) -> Self {
        variant as u8 != 0
    }
}
impl RS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RS_A {
        match self.bits {
            false => RS_A::IDLE,
            true => RS_A::RECEIVE,
        }
    }
    #[doc = "Idle. The CAN controller is idle."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == RS_A::IDLE
    }
    #[doc = "Receive. The CAN controller is receiving a message."]
    #[inline(always)]
    pub fn is_receive(&self) -> bool {
        *self == RS_A::RECEIVE
    }
}
#[doc = "Field `TS` reader - Transmit Status. If both the Receive Status and the Transmit Status bits are '0' (idle), the CAN-Bus is idle. If both bits are set, the controller is waiting to become idle again. After hardware reset 11 consecutive recessive bits have to be detected until idle status is reached. After Bus-off this will take 128 times of 11 consecutive recessive bits."]
pub type TS_R = crate::BitReader<TS_A>;
#[doc = "Transmit Status. If both the Receive Status and the Transmit Status bits are '0' (idle), the CAN-Bus is idle. If both bits are set, the controller is waiting to become idle again. After hardware reset 11 consecutive recessive bits have to be detected until idle status is reached. After Bus-off this will take 128 times of 11 consecutive recessive bits.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TS_A {
    #[doc = "0: Idle. The CAN controller is idle."]
    IDLE = 0,
    #[doc = "1: Transmit. The CAN controller is sending a message."]
    TRANSMIT = 1,
}
impl From<TS_A> for bool {
    #[inline(always)]
    fn from(variant: TS_A) -> Self {
        variant as u8 != 0
    }
}
impl TS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TS_A {
        match self.bits {
            false => TS_A::IDLE,
            true => TS_A::TRANSMIT,
        }
    }
    #[doc = "Idle. The CAN controller is idle."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == TS_A::IDLE
    }
    #[doc = "Transmit. The CAN controller is sending a message."]
    #[inline(always)]
    pub fn is_transmit(&self) -> bool {
        *self == TS_A::TRANSMIT
    }
}
#[doc = "Field `ES` reader - Error Status. Errors detected during reception or transmission will effect the error counters according to the CAN specification. The Error Status bit is set when at least one of the error counters has reached or exceeded the Error Warning Limit. An Error Warning Interrupt is generated, if enabled. The default value of the Error Warning Limit after hardware reset is 96 decimal, see also Section 21.7.7 CAN Error Warning Limit register (CAN1EWL - 0x4004 4018, CAN2EWL - 0x4004 8018)."]
pub type ES_R = crate::BitReader<ES_A>;
#[doc = "Error Status. Errors detected during reception or transmission will effect the error counters according to the CAN specification. The Error Status bit is set when at least one of the error counters has reached or exceeded the Error Warning Limit. An Error Warning Interrupt is generated, if enabled. The default value of the Error Warning Limit after hardware reset is 96 decimal, see also Section 21.7.7 CAN Error Warning Limit register (CAN1EWL - 0x4004 4018, CAN2EWL - 0x4004 8018).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ES_A {
    #[doc = "0: OK. Both error counters are below the Error Warning Limit."]
    OK = 0,
    #[doc = "1: Error. One or both of the Transmit and Receive Error Counters has reached the limit set in the Error Warning Limit register."]
    ERROR = 1,
}
impl From<ES_A> for bool {
    #[inline(always)]
    fn from(variant: ES_A) -> Self {
        variant as u8 != 0
    }
}
impl ES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ES_A {
        match self.bits {
            false => ES_A::OK,
            true => ES_A::ERROR,
        }
    }
    #[doc = "OK. Both error counters are below the Error Warning Limit."]
    #[inline(always)]
    pub fn is_ok(&self) -> bool {
        *self == ES_A::OK
    }
    #[doc = "Error. One or both of the Transmit and Receive Error Counters has reached the limit set in the Error Warning Limit register."]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == ES_A::ERROR
    }
}
#[doc = "Field `BS` reader - Bus Status. Mode bit '1' (present) and an Error Warning Interrupt is generated, if enabled. Afterwards the Transmit Error Counter is set to '127', and the Receive Error Counter is cleared. It will stay in this mode until the CPU clears the Reset Mode bit. Once this is completed the CAN Controller will wait the minimum protocol-defined time (128 occurrences of the Bus-Free signal) counting down the Transmit Error Counter. After that, the Bus Status bit is cleared (Bus-On), the Error Status bit is set '0' (ok), the Error Counters are reset, and an Error Warning Interrupt is generated, if enabled. Reading the TX Error Counter during this time gives information about the status of the Bus-Off recovery."]
pub type BS_R = crate::BitReader<BS_A>;
#[doc = "Bus Status. Mode bit '1' (present) and an Error Warning Interrupt is generated, if enabled. Afterwards the Transmit Error Counter is set to '127', and the Receive Error Counter is cleared. It will stay in this mode until the CPU clears the Reset Mode bit. Once this is completed the CAN Controller will wait the minimum protocol-defined time (128 occurrences of the Bus-Free signal) counting down the Transmit Error Counter. After that, the Bus Status bit is cleared (Bus-On), the Error Status bit is set '0' (ok), the Error Counters are reset, and an Error Warning Interrupt is generated, if enabled. Reading the TX Error Counter during this time gives information about the status of the Bus-Off recovery.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS_A {
    #[doc = "0: Bus-on. The CAN Controller is involved in bus activities"]
    BUS_ON = 0,
    #[doc = "1: Bus-off. The CAN controller is currently not involved/prohibited from bus activity because the Transmit Error Counter reached its limiting value of 255."]
    BUS_OFF = 1,
}
impl From<BS_A> for bool {
    #[inline(always)]
    fn from(variant: BS_A) -> Self {
        variant as u8 != 0
    }
}
impl BS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BS_A {
        match self.bits {
            false => BS_A::BUS_ON,
            true => BS_A::BUS_OFF,
        }
    }
    #[doc = "Bus-on. The CAN Controller is involved in bus activities"]
    #[inline(always)]
    pub fn is_bus_on(&self) -> bool {
        *self == BS_A::BUS_ON
    }
    #[doc = "Bus-off. The CAN controller is currently not involved/prohibited from bus activity because the Transmit Error Counter reached its limiting value of 255."]
    #[inline(always)]
    pub fn is_bus_off(&self) -> bool {
        *self == BS_A::BUS_OFF
    }
}
#[doc = "Field `RXERR` reader - The current value of the Rx Error Counter (an 8-bit value)."]
pub type RXERR_R = crate::FieldReader;
#[doc = "Field `TXERR` reader - The current value of the Tx Error Counter (an 8-bit value)."]
pub type TXERR_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Receive Buffer Status. After reading all messages and releasing their memory space with the command 'Release Receive Buffer,' this bit is cleared."]
    #[inline(always)]
    pub fn rbs(&self) -> RBS_R {
        RBS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data Overrun Status. If there is not enough space to store the message within the Receive Buffer, that message is dropped and the Data Overrun condition is signalled to the CPU in the moment this message becomes valid. If this message is not completed successfully (e.g. because of an error), no overrun condition is signalled."]
    #[inline(always)]
    pub fn dos(&self) -> DOS_R {
        DOS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Status."]
    #[inline(always)]
    pub fn tbs(&self) -> TBS_R {
        TBS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Complete Status. The Transmission Complete Status bit is set '0' (incomplete) whenever the Transmission Request bit or the Self Reception Request bit is set '1' at least for one of the three Transmit Buffers. The Transmission Complete Status bit will remain '0' until all messages are transmitted successfully."]
    #[inline(always)]
    pub fn tcs(&self) -> TCS_R {
        TCS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Status. If both the Receive Status and the Transmit Status bits are '0' (idle), the CAN-Bus is idle. If both bits are set, the controller is waiting to become idle again. After hardware reset 11 consecutive recessive bits have to be detected until idle status is reached. After Bus-off this will take 128 times of 11 consecutive recessive bits."]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Status. If both the Receive Status and the Transmit Status bits are '0' (idle), the CAN-Bus is idle. If both bits are set, the controller is waiting to become idle again. After hardware reset 11 consecutive recessive bits have to be detected until idle status is reached. After Bus-off this will take 128 times of 11 consecutive recessive bits."]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Error Status. Errors detected during reception or transmission will effect the error counters according to the CAN specification. The Error Status bit is set when at least one of the error counters has reached or exceeded the Error Warning Limit. An Error Warning Interrupt is generated, if enabled. The default value of the Error Warning Limit after hardware reset is 96 decimal, see also Section 21.7.7 CAN Error Warning Limit register (CAN1EWL - 0x4004 4018, CAN2EWL - 0x4004 8018)."]
    #[inline(always)]
    pub fn es(&self) -> ES_R {
        ES_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus Status. Mode bit '1' (present) and an Error Warning Interrupt is generated, if enabled. Afterwards the Transmit Error Counter is set to '127', and the Receive Error Counter is cleared. It will stay in this mode until the CPU clears the Reset Mode bit. Once this is completed the CAN Controller will wait the minimum protocol-defined time (128 occurrences of the Bus-Free signal) counting down the Transmit Error Counter. After that, the Bus Status bit is cleared (Bus-On), the Error Status bit is set '0' (ok), the Error Counters are reset, and an Error Warning Interrupt is generated, if enabled. Reading the TX Error Counter during this time gives information about the status of the Bus-Off recovery."]
    #[inline(always)]
    pub fn bs(&self) -> BS_R {
        BS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:23 - The current value of the Rx Error Counter (an 8-bit value)."]
    #[inline(always)]
    pub fn rxerr(&self) -> RXERR_R {
        RXERR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - The current value of the Tx Error Counter (an 8-bit value)."]
    #[inline(always)]
    pub fn txerr(&self) -> TXERR_R {
        TXERR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GSR")
            .field("rbs", &format_args!("{}", self.rbs().bit()))
            .field("dos", &format_args!("{}", self.dos().bit()))
            .field("tbs", &format_args!("{}", self.tbs().bit()))
            .field("tcs", &format_args!("{}", self.tcs().bit()))
            .field("rs", &format_args!("{}", self.rs().bit()))
            .field("ts", &format_args!("{}", self.ts().bit()))
            .field("es", &format_args!("{}", self.es().bit()))
            .field("bs", &format_args!("{}", self.bs().bit()))
            .field("rxerr", &format_args!("{}", self.rxerr().bits()))
            .field("txerr", &format_args!("{}", self.txerr().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<GSR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Global Controller Status and Error Counters. The error counters can only be written when RM in CANMOD is 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GSR_SPEC;
impl crate::RegisterSpec for GSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsr::R`](R) reader structure"]
impl crate::Readable for GSR_SPEC {}
#[doc = "`reset()` method sets GSR to value 0x3c"]
impl crate::Resettable for GSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x3c;
}
