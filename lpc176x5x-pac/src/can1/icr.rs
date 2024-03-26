#[doc = "Register `ICR` reader"]
pub type R = crate::R<ICR_SPEC>;
#[doc = "Field `RI` reader - Receive Interrupt. This bit is set whenever the RBS bit in CANxSR and the RIE bit in CANxIER are both 1, indicating that a new message was received and stored in the Receive Buffer. The Receive Interrupt Bit is not cleared upon a read access to the Interrupt Register. Giving the Command Release Receive Buffer will clear RI temporarily. If there is another message available within the Receive Buffer after the release command, RI is set again. Otherwise RI remains cleared."]
pub type RI_R = crate::BitReader<RI_A>;
#[doc = "Receive Interrupt. This bit is set whenever the RBS bit in CANxSR and the RIE bit in CANxIER are both 1, indicating that a new message was received and stored in the Receive Buffer. The Receive Interrupt Bit is not cleared upon a read access to the Interrupt Register. Giving the Command Release Receive Buffer will clear RI temporarily. If there is another message available within the Receive Buffer after the release command, RI is set again. Otherwise RI remains cleared.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RI_A {
    #[doc = "0: Reset"]
    RESET = 0,
    #[doc = "1: Set"]
    SET = 1,
}
impl From<RI_A> for bool {
    #[inline(always)]
    fn from(variant: RI_A) -> Self {
        variant as u8 != 0
    }
}
impl RI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RI_A {
        match self.bits {
            false => RI_A::RESET,
            true => RI_A::SET,
        }
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RI_A::RESET
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == RI_A::SET
    }
}
#[doc = "Field `TI1` reader - Transmit Interrupt 1. This bit is set when the TBS1 bit in CANxSR goes from 0 to 1 (whenever a message out of TXB1 was successfully transmitted or aborted), indicating that Transmit buffer 1 is available, and the TIE1 bit in CANxIER is 1."]
pub type TI1_R = crate::BitReader<TI1_A>;
#[doc = "Transmit Interrupt 1. This bit is set when the TBS1 bit in CANxSR goes from 0 to 1 (whenever a message out of TXB1 was successfully transmitted or aborted), indicating that Transmit buffer 1 is available, and the TIE1 bit in CANxIER is 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TI1_A {
    #[doc = "0: Reset"]
    RESET = 0,
    #[doc = "1: Set"]
    SET = 1,
}
impl From<TI1_A> for bool {
    #[inline(always)]
    fn from(variant: TI1_A) -> Self {
        variant as u8 != 0
    }
}
impl TI1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TI1_A {
        match self.bits {
            false => TI1_A::RESET,
            true => TI1_A::SET,
        }
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TI1_A::RESET
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == TI1_A::SET
    }
}
#[doc = "Field `EI` reader - Error Warning Interrupt. This bit is set on every change (set or clear) of either the Error Status or Bus Status bit in CANxSR and the EIE bit bit is set within the Interrupt Enable Register at the time of the change."]
pub type EI_R = crate::BitReader<EI_A>;
#[doc = "Error Warning Interrupt. This bit is set on every change (set or clear) of either the Error Status or Bus Status bit in CANxSR and the EIE bit bit is set within the Interrupt Enable Register at the time of the change.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EI_A {
    #[doc = "0: Reset"]
    RESET = 0,
    #[doc = "1: Set"]
    SET = 1,
}
impl From<EI_A> for bool {
    #[inline(always)]
    fn from(variant: EI_A) -> Self {
        variant as u8 != 0
    }
}
impl EI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EI_A {
        match self.bits {
            false => EI_A::RESET,
            true => EI_A::SET,
        }
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == EI_A::RESET
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == EI_A::SET
    }
}
#[doc = "Field `DOI` reader - Data Overrun Interrupt. This bit is set when the DOS bit in CANxSR goes from 0 to 1 and the DOIE bit in CANxIER is 1."]
pub type DOI_R = crate::BitReader<DOI_A>;
#[doc = "Data Overrun Interrupt. This bit is set when the DOS bit in CANxSR goes from 0 to 1 and the DOIE bit in CANxIER is 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOI_A {
    #[doc = "0: Reset"]
    RESET = 0,
    #[doc = "1: Set"]
    SET = 1,
}
impl From<DOI_A> for bool {
    #[inline(always)]
    fn from(variant: DOI_A) -> Self {
        variant as u8 != 0
    }
}
impl DOI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DOI_A {
        match self.bits {
            false => DOI_A::RESET,
            true => DOI_A::SET,
        }
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DOI_A::RESET
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == DOI_A::SET
    }
}
#[doc = "Field `WUI` reader - Wake-Up Interrupt. This bit is set if the CAN controller is sleeping and bus activity is detected and the WUIE bit in CANxIER is 1. A Wake-Up Interrupt is also generated if the CPU tries to set the Sleep bit while the CAN controller is involved in bus activities or a CAN Interrupt is pending. The WUI flag can also get asserted when the according enable bit WUIE is not set. In this case a Wake-Up Interrupt does not get asserted."]
pub type WUI_R = crate::BitReader<WUI_A>;
#[doc = "Wake-Up Interrupt. This bit is set if the CAN controller is sleeping and bus activity is detected and the WUIE bit in CANxIER is 1. A Wake-Up Interrupt is also generated if the CPU tries to set the Sleep bit while the CAN controller is involved in bus activities or a CAN Interrupt is pending. The WUI flag can also get asserted when the according enable bit WUIE is not set. In this case a Wake-Up Interrupt does not get asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUI_A {
    #[doc = "0: Reset"]
    RESET = 0,
    #[doc = "1: Set"]
    SET = 1,
}
impl From<WUI_A> for bool {
    #[inline(always)]
    fn from(variant: WUI_A) -> Self {
        variant as u8 != 0
    }
}
impl WUI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUI_A {
        match self.bits {
            false => WUI_A::RESET,
            true => WUI_A::SET,
        }
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == WUI_A::RESET
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == WUI_A::SET
    }
}
#[doc = "Field `EPI` reader - Error Passive Interrupt. This bit is set if the EPIE bit in CANxIER is 1, and the CAN controller switches between Error Passive and Error Active mode in either direction. This is the case when the CAN Controller has reached the Error Passive Status (at least one error counter exceeds the CAN protocol defined level of 127) or if the CAN Controller is in Error Passive Status and enters the Error Active Status again."]
pub type EPI_R = crate::BitReader<EPI_A>;
#[doc = "Error Passive Interrupt. This bit is set if the EPIE bit in CANxIER is 1, and the CAN controller switches between Error Passive and Error Active mode in either direction. This is the case when the CAN Controller has reached the Error Passive Status (at least one error counter exceeds the CAN protocol defined level of 127) or if the CAN Controller is in Error Passive Status and enters the Error Active Status again.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EPI_A {
    #[doc = "0: Reset"]
    RESET = 0,
    #[doc = "1: Set"]
    SET = 1,
}
impl From<EPI_A> for bool {
    #[inline(always)]
    fn from(variant: EPI_A) -> Self {
        variant as u8 != 0
    }
}
impl EPI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EPI_A {
        match self.bits {
            false => EPI_A::RESET,
            true => EPI_A::SET,
        }
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == EPI_A::RESET
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == EPI_A::SET
    }
}
#[doc = "Field `ALI` reader - Arbitration Lost Interrupt. This bit is set if the ALIE bit in CANxIER is 1, and the CAN controller loses arbitration while attempting to transmit. In this case the CAN node becomes a receiver."]
pub type ALI_R = crate::BitReader<ALI_A>;
#[doc = "Arbitration Lost Interrupt. This bit is set if the ALIE bit in CANxIER is 1, and the CAN controller loses arbitration while attempting to transmit. In this case the CAN node becomes a receiver.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALI_A {
    #[doc = "0: Reset"]
    RESET = 0,
    #[doc = "1: Set"]
    SET = 1,
}
impl From<ALI_A> for bool {
    #[inline(always)]
    fn from(variant: ALI_A) -> Self {
        variant as u8 != 0
    }
}
impl ALI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALI_A {
        match self.bits {
            false => ALI_A::RESET,
            true => ALI_A::SET,
        }
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == ALI_A::RESET
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == ALI_A::SET
    }
}
#[doc = "Field `BEI` reader - Bus Error Interrupt -- this bit is set if the BEIE bit in CANxIER is 1, and the CAN controller detects an error on the bus."]
pub type BEI_R = crate::BitReader<BEI_A>;
#[doc = "Bus Error Interrupt -- this bit is set if the BEIE bit in CANxIER is 1, and the CAN controller detects an error on the bus.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BEI_A {
    #[doc = "0: Reset"]
    RESET = 0,
    #[doc = "1: Set"]
    SET = 1,
}
impl From<BEI_A> for bool {
    #[inline(always)]
    fn from(variant: BEI_A) -> Self {
        variant as u8 != 0
    }
}
impl BEI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BEI_A {
        match self.bits {
            false => BEI_A::RESET,
            true => BEI_A::SET,
        }
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == BEI_A::RESET
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == BEI_A::SET
    }
}
#[doc = "Field `IDI` reader - ID Ready Interrupt -- this bit is set if the IDIE bit in CANxIER is 1, and a CAN Identifier has been received (a message was successfully transmitted or aborted). This bit is set whenever a message was successfully transmitted or aborted and the IDIE bit is set in the IER register."]
pub type IDI_R = crate::BitReader<IDI_A>;
#[doc = "ID Ready Interrupt -- this bit is set if the IDIE bit in CANxIER is 1, and a CAN Identifier has been received (a message was successfully transmitted or aborted). This bit is set whenever a message was successfully transmitted or aborted and the IDIE bit is set in the IER register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDI_A {
    #[doc = "0: Reset"]
    RESET = 0,
    #[doc = "1: Set"]
    SET = 1,
}
impl From<IDI_A> for bool {
    #[inline(always)]
    fn from(variant: IDI_A) -> Self {
        variant as u8 != 0
    }
}
impl IDI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IDI_A {
        match self.bits {
            false => IDI_A::RESET,
            true => IDI_A::SET,
        }
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == IDI_A::RESET
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == IDI_A::SET
    }
}
#[doc = "Field `TI2` reader - Transmit Interrupt 2. This bit is set when the TBS2 bit in CANxSR goes from 0 to 1 (whenever a message out of TXB2 was successfully transmitted or aborted), indicating that Transmit buffer 2 is available, and the TIE2 bit in CANxIER is 1."]
pub type TI2_R = crate::BitReader<TI2_A>;
#[doc = "Transmit Interrupt 2. This bit is set when the TBS2 bit in CANxSR goes from 0 to 1 (whenever a message out of TXB2 was successfully transmitted or aborted), indicating that Transmit buffer 2 is available, and the TIE2 bit in CANxIER is 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TI2_A {
    #[doc = "0: Reset"]
    RESET = 0,
    #[doc = "1: Set"]
    SET = 1,
}
impl From<TI2_A> for bool {
    #[inline(always)]
    fn from(variant: TI2_A) -> Self {
        variant as u8 != 0
    }
}
impl TI2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TI2_A {
        match self.bits {
            false => TI2_A::RESET,
            true => TI2_A::SET,
        }
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TI2_A::RESET
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == TI2_A::SET
    }
}
#[doc = "Field `TI3` reader - Transmit Interrupt 3. This bit is set when the TBS3 bit in CANxSR goes from 0 to 1 (whenever a message out of TXB3 was successfully transmitted or aborted), indicating that Transmit buffer 3 is available, and the TIE3 bit in CANxIER is 1."]
pub type TI3_R = crate::BitReader<TI3_A>;
#[doc = "Transmit Interrupt 3. This bit is set when the TBS3 bit in CANxSR goes from 0 to 1 (whenever a message out of TXB3 was successfully transmitted or aborted), indicating that Transmit buffer 3 is available, and the TIE3 bit in CANxIER is 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TI3_A {
    #[doc = "0: Reset"]
    RESET = 0,
    #[doc = "1: Set"]
    SET = 1,
}
impl From<TI3_A> for bool {
    #[inline(always)]
    fn from(variant: TI3_A) -> Self {
        variant as u8 != 0
    }
}
impl TI3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TI3_A {
        match self.bits {
            false => TI3_A::RESET,
            true => TI3_A::SET,
        }
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TI3_A::RESET
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == TI3_A::SET
    }
}
#[doc = "Field `ERRBIT4_0` reader - Error Code Capture: when the CAN controller detects a bus error, the location of the error within the frame is captured in this field. The value reflects an internal state variable, and as a result is not very linear: 00011 = Start of Frame 00010 = ID28 ... ID21 00110 = ID20 ... ID18 00100 = SRTR Bit 00101 = IDE bit 00111 = ID17 ... 13 01111 = ID12 ... ID5 01110 = ID4 ... ID0 01100 = RTR Bit 01101 = Reserved Bit 1 01001 = Reserved Bit 0 01011 = Data Length Code 01010 = Data Field 01000 = CRC Sequence 11000 = CRC Delimiter 11001 = Acknowledge Slot 11011 = Acknowledge Delimiter 11010 = End of Frame 10010 = Intermission Whenever a bus error occurs, the corresponding bus error interrupt is forced, if enabled. At the same time, the current position of the Bit Stream Processor is captured into the Error Code Capture Register. The content within this register is fixed until the user software has read out its content once. From now on, the capture mechanism is activated again, i.e. reading the CANxICR enables another Bus Error Interrupt."]
pub type ERRBIT4_0_R = crate::FieldReader;
#[doc = "Field `ERRDIR` reader - When the CAN controller detects a bus error, the direction of the current bit is captured in this bit."]
pub type ERRDIR_R = crate::BitReader<ERRDIR_A>;
#[doc = "When the CAN controller detects a bus error, the direction of the current bit is captured in this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRDIR_A {
    #[doc = "0: Error occurred during transmitting."]
    TRANSMITTING = 0,
    #[doc = "1: Error occurred during receiving."]
    RECEIVING = 1,
}
impl From<ERRDIR_A> for bool {
    #[inline(always)]
    fn from(variant: ERRDIR_A) -> Self {
        variant as u8 != 0
    }
}
impl ERRDIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERRDIR_A {
        match self.bits {
            false => ERRDIR_A::TRANSMITTING,
            true => ERRDIR_A::RECEIVING,
        }
    }
    #[doc = "Error occurred during transmitting."]
    #[inline(always)]
    pub fn is_transmitting(&self) -> bool {
        *self == ERRDIR_A::TRANSMITTING
    }
    #[doc = "Error occurred during receiving."]
    #[inline(always)]
    pub fn is_receiving(&self) -> bool {
        *self == ERRDIR_A::RECEIVING
    }
}
#[doc = "Field `ERRC1_0` reader - When the CAN controller detects a bus error, the type of error is captured in this field:"]
pub type ERRC1_0_R = crate::FieldReader<ERRC1_0_A>;
#[doc = "When the CAN controller detects a bus error, the type of error is captured in this field:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ERRC1_0_A {
    #[doc = "0: Bit error"]
    BIT_ERROR = 0,
    #[doc = "1: Form error"]
    FORM_ERROR = 1,
    #[doc = "2: Stuff error"]
    STUFF_ERROR = 2,
    #[doc = "3: Other error"]
    OTHER_ERROR = 3,
}
impl From<ERRC1_0_A> for u8 {
    #[inline(always)]
    fn from(variant: ERRC1_0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ERRC1_0_A {
    type Ux = u8;
}
impl ERRC1_0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERRC1_0_A {
        match self.bits {
            0 => ERRC1_0_A::BIT_ERROR,
            1 => ERRC1_0_A::FORM_ERROR,
            2 => ERRC1_0_A::STUFF_ERROR,
            3 => ERRC1_0_A::OTHER_ERROR,
            _ => unreachable!(),
        }
    }
    #[doc = "Bit error"]
    #[inline(always)]
    pub fn is_bit_error(&self) -> bool {
        *self == ERRC1_0_A::BIT_ERROR
    }
    #[doc = "Form error"]
    #[inline(always)]
    pub fn is_form_error(&self) -> bool {
        *self == ERRC1_0_A::FORM_ERROR
    }
    #[doc = "Stuff error"]
    #[inline(always)]
    pub fn is_stuff_error(&self) -> bool {
        *self == ERRC1_0_A::STUFF_ERROR
    }
    #[doc = "Other error"]
    #[inline(always)]
    pub fn is_other_error(&self) -> bool {
        *self == ERRC1_0_A::OTHER_ERROR
    }
}
#[doc = "Field `ALCBIT` reader - Each time arbitration is lost while trying to send on the CAN, the bit number within the frame is captured into this field. After the content of ALCBIT is read, the ALI bit is cleared and a new Arbitration Lost interrupt can occur. 00 = arbitration lost in the first bit (MS) of identifier ... 11 = arbitration lost in SRTS bit (RTR bit for standard frame messages) 12 = arbitration lost in IDE bit 13 = arbitration lost in 12th bit of identifier (extended frame only) ... 30 = arbitration lost in last bit of identifier (extended frame only) 31 = arbitration lost in RTR bit (extended frame only) On arbitration lost, the corresponding arbitration lost interrupt is forced, if enabled. At that time, the current bit position of the Bit Stream Processor is captured into the Arbitration Lost Capture Register. The content within this register is fixed until the user application has read out its contents once. From now on, the capture mechanism is activated again."]
pub type ALCBIT_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Receive Interrupt. This bit is set whenever the RBS bit in CANxSR and the RIE bit in CANxIER are both 1, indicating that a new message was received and stored in the Receive Buffer. The Receive Interrupt Bit is not cleared upon a read access to the Interrupt Register. Giving the Command Release Receive Buffer will clear RI temporarily. If there is another message available within the Receive Buffer after the release command, RI is set again. Otherwise RI remains cleared."]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Interrupt 1. This bit is set when the TBS1 bit in CANxSR goes from 0 to 1 (whenever a message out of TXB1 was successfully transmitted or aborted), indicating that Transmit buffer 1 is available, and the TIE1 bit in CANxIER is 1."]
    #[inline(always)]
    pub fn ti1(&self) -> TI1_R {
        TI1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error Warning Interrupt. This bit is set on every change (set or clear) of either the Error Status or Bus Status bit in CANxSR and the EIE bit bit is set within the Interrupt Enable Register at the time of the change."]
    #[inline(always)]
    pub fn ei(&self) -> EI_R {
        EI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Overrun Interrupt. This bit is set when the DOS bit in CANxSR goes from 0 to 1 and the DOIE bit in CANxIER is 1."]
    #[inline(always)]
    pub fn doi(&self) -> DOI_R {
        DOI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wake-Up Interrupt. This bit is set if the CAN controller is sleeping and bus activity is detected and the WUIE bit in CANxIER is 1. A Wake-Up Interrupt is also generated if the CPU tries to set the Sleep bit while the CAN controller is involved in bus activities or a CAN Interrupt is pending. The WUI flag can also get asserted when the according enable bit WUIE is not set. In this case a Wake-Up Interrupt does not get asserted."]
    #[inline(always)]
    pub fn wui(&self) -> WUI_R {
        WUI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Error Passive Interrupt. This bit is set if the EPIE bit in CANxIER is 1, and the CAN controller switches between Error Passive and Error Active mode in either direction. This is the case when the CAN Controller has reached the Error Passive Status (at least one error counter exceeds the CAN protocol defined level of 127) or if the CAN Controller is in Error Passive Status and enters the Error Active Status again."]
    #[inline(always)]
    pub fn epi(&self) -> EPI_R {
        EPI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Arbitration Lost Interrupt. This bit is set if the ALIE bit in CANxIER is 1, and the CAN controller loses arbitration while attempting to transmit. In this case the CAN node becomes a receiver."]
    #[inline(always)]
    pub fn ali(&self) -> ALI_R {
        ALI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus Error Interrupt -- this bit is set if the BEIE bit in CANxIER is 1, and the CAN controller detects an error on the bus."]
    #[inline(always)]
    pub fn bei(&self) -> BEI_R {
        BEI_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ID Ready Interrupt -- this bit is set if the IDIE bit in CANxIER is 1, and a CAN Identifier has been received (a message was successfully transmitted or aborted). This bit is set whenever a message was successfully transmitted or aborted and the IDIE bit is set in the IER register."]
    #[inline(always)]
    pub fn idi(&self) -> IDI_R {
        IDI_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmit Interrupt 2. This bit is set when the TBS2 bit in CANxSR goes from 0 to 1 (whenever a message out of TXB2 was successfully transmitted or aborted), indicating that Transmit buffer 2 is available, and the TIE2 bit in CANxIER is 1."]
    #[inline(always)]
    pub fn ti2(&self) -> TI2_R {
        TI2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmit Interrupt 3. This bit is set when the TBS3 bit in CANxSR goes from 0 to 1 (whenever a message out of TXB3 was successfully transmitted or aborted), indicating that Transmit buffer 3 is available, and the TIE3 bit in CANxIER is 1."]
    #[inline(always)]
    pub fn ti3(&self) -> TI3_R {
        TI3_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Error Code Capture: when the CAN controller detects a bus error, the location of the error within the frame is captured in this field. The value reflects an internal state variable, and as a result is not very linear: 00011 = Start of Frame 00010 = ID28 ... ID21 00110 = ID20 ... ID18 00100 = SRTR Bit 00101 = IDE bit 00111 = ID17 ... 13 01111 = ID12 ... ID5 01110 = ID4 ... ID0 01100 = RTR Bit 01101 = Reserved Bit 1 01001 = Reserved Bit 0 01011 = Data Length Code 01010 = Data Field 01000 = CRC Sequence 11000 = CRC Delimiter 11001 = Acknowledge Slot 11011 = Acknowledge Delimiter 11010 = End of Frame 10010 = Intermission Whenever a bus error occurs, the corresponding bus error interrupt is forced, if enabled. At the same time, the current position of the Bit Stream Processor is captured into the Error Code Capture Register. The content within this register is fixed until the user software has read out its content once. From now on, the capture mechanism is activated again, i.e. reading the CANxICR enables another Bus Error Interrupt."]
    #[inline(always)]
    pub fn errbit4_0(&self) -> ERRBIT4_0_R {
        ERRBIT4_0_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - When the CAN controller detects a bus error, the direction of the current bit is captured in this bit."]
    #[inline(always)]
    pub fn errdir(&self) -> ERRDIR_R {
        ERRDIR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - When the CAN controller detects a bus error, the type of error is captured in this field:"]
    #[inline(always)]
    pub fn errc1_0(&self) -> ERRC1_0_R {
        ERRC1_0_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:31 - Each time arbitration is lost while trying to send on the CAN, the bit number within the frame is captured into this field. After the content of ALCBIT is read, the ALI bit is cleared and a new Arbitration Lost interrupt can occur. 00 = arbitration lost in the first bit (MS) of identifier ... 11 = arbitration lost in SRTS bit (RTR bit for standard frame messages) 12 = arbitration lost in IDE bit 13 = arbitration lost in 12th bit of identifier (extended frame only) ... 30 = arbitration lost in last bit of identifier (extended frame only) 31 = arbitration lost in RTR bit (extended frame only) On arbitration lost, the corresponding arbitration lost interrupt is forced, if enabled. At that time, the current bit position of the Bit Stream Processor is captured into the Arbitration Lost Capture Register. The content within this register is fixed until the user application has read out its contents once. From now on, the capture mechanism is activated again."]
    #[inline(always)]
    pub fn alcbit(&self) -> ALCBIT_R {
        ALCBIT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICR")
            .field("ri", &format_args!("{}", self.ri().bit()))
            .field("ti1", &format_args!("{}", self.ti1().bit()))
            .field("ei", &format_args!("{}", self.ei().bit()))
            .field("doi", &format_args!("{}", self.doi().bit()))
            .field("wui", &format_args!("{}", self.wui().bit()))
            .field("epi", &format_args!("{}", self.epi().bit()))
            .field("ali", &format_args!("{}", self.ali().bit()))
            .field("bei", &format_args!("{}", self.bei().bit()))
            .field("idi", &format_args!("{}", self.idi().bit()))
            .field("ti2", &format_args!("{}", self.ti2().bit()))
            .field("ti3", &format_args!("{}", self.ti3().bit()))
            .field("errbit4_0", &format_args!("{}", self.errbit4_0().bits()))
            .field("errdir", &format_args!("{}", self.errdir().bit()))
            .field("errc1_0", &format_args!("{}", self.errc1_0().bits()))
            .field("alcbit", &format_args!("{}", self.alcbit().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<ICR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Interrupt status, Arbitration Lost Capture, Error Code Capture\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for ICR_SPEC {}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
