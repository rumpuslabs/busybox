#[doc = "Register `SR` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Field `RBS_1` reader - Receive Buffer Status. This bit is identical to the RBS bit in the CANxGSR."]
pub type RBS_1_R = crate::BitReader;
#[doc = "Field `DOS_1` reader - Data Overrun Status. This bit is identical to the DOS bit in the CANxGSR."]
pub type DOS_1_R = crate::BitReader;
#[doc = "Field `TBS1_1` reader - Transmit Buffer Status 1."]
pub type TBS1_1_R = crate::BitReader<TBS1_1_A>;
#[doc = "Transmit Buffer Status 1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBS1_1_A {
    #[doc = "0: Locked. Software cannot access the Tx Buffer 1 nor write to the corresponding CANxTFI, CANxTID, CANxTDA, and CANxTDB registers because a message is either waiting for transmission or is in transmitting process."]
    LOCKED = 0,
    #[doc = "1: Released. Software may write a message into the Transmit Buffer 1 and its CANxTFI, CANxTID, CANxTDA, and CANxTDB registers."]
    RELEASED = 1,
}
impl From<TBS1_1_A> for bool {
    #[inline(always)]
    fn from(variant: TBS1_1_A) -> Self {
        variant as u8 != 0
    }
}
impl TBS1_1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TBS1_1_A {
        match self.bits {
            false => TBS1_1_A::LOCKED,
            true => TBS1_1_A::RELEASED,
        }
    }
    #[doc = "Locked. Software cannot access the Tx Buffer 1 nor write to the corresponding CANxTFI, CANxTID, CANxTDA, and CANxTDB registers because a message is either waiting for transmission or is in transmitting process."]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == TBS1_1_A::LOCKED
    }
    #[doc = "Released. Software may write a message into the Transmit Buffer 1 and its CANxTFI, CANxTID, CANxTDA, and CANxTDB registers."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == TBS1_1_A::RELEASED
    }
}
#[doc = "Field `TCS1_1` reader - Transmission Complete Status."]
pub type TCS1_1_R = crate::BitReader<TCS1_1_A>;
#[doc = "Transmission Complete Status.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCS1_1_A {
    #[doc = "0: Incomplete. The previously requested transmission for Tx Buffer 1 is not complete."]
    INCOMPLETE = 0,
    #[doc = "1: Complete. The previously requested transmission for Tx Buffer 1 has been successfully completed."]
    COMPLETE = 1,
}
impl From<TCS1_1_A> for bool {
    #[inline(always)]
    fn from(variant: TCS1_1_A) -> Self {
        variant as u8 != 0
    }
}
impl TCS1_1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCS1_1_A {
        match self.bits {
            false => TCS1_1_A::INCOMPLETE,
            true => TCS1_1_A::COMPLETE,
        }
    }
    #[doc = "Incomplete. The previously requested transmission for Tx Buffer 1 is not complete."]
    #[inline(always)]
    pub fn is_incomplete(&self) -> bool {
        *self == TCS1_1_A::INCOMPLETE
    }
    #[doc = "Complete. The previously requested transmission for Tx Buffer 1 has been successfully completed."]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == TCS1_1_A::COMPLETE
    }
}
#[doc = "Field `RS_1` reader - Receive Status. This bit is identical to the RS bit in the GSR."]
pub type RS_1_R = crate::BitReader;
#[doc = "Field `TS1_1` reader - Transmit Status 1."]
pub type TS1_1_R = crate::BitReader<TS1_1_A>;
#[doc = "Transmit Status 1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TS1_1_A {
    #[doc = "0: Idle. There is no transmission from Tx Buffer 1."]
    IDLE = 0,
    #[doc = "1: Transmit. The CAN Controller is transmitting a message from Tx Buffer 1."]
    TRANSMIT = 1,
}
impl From<TS1_1_A> for bool {
    #[inline(always)]
    fn from(variant: TS1_1_A) -> Self {
        variant as u8 != 0
    }
}
impl TS1_1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TS1_1_A {
        match self.bits {
            false => TS1_1_A::IDLE,
            true => TS1_1_A::TRANSMIT,
        }
    }
    #[doc = "Idle. There is no transmission from Tx Buffer 1."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == TS1_1_A::IDLE
    }
    #[doc = "Transmit. The CAN Controller is transmitting a message from Tx Buffer 1."]
    #[inline(always)]
    pub fn is_transmit(&self) -> bool {
        *self == TS1_1_A::TRANSMIT
    }
}
#[doc = "Field `ES_1` reader - Error Status. This bit is identical to the ES bit in the CANxGSR."]
pub type ES_1_R = crate::BitReader;
#[doc = "Field `BS_1` reader - Bus Status. This bit is identical to the BS bit in the CANxGSR."]
pub type BS_1_R = crate::BitReader;
#[doc = "Field `RBS_2` reader - Receive Buffer Status. This bit is identical to the RBS bit in the CANxGSR."]
pub type RBS_2_R = crate::BitReader;
#[doc = "Field `DOS_2` reader - Data Overrun Status. This bit is identical to the DOS bit in the CANxGSR."]
pub type DOS_2_R = crate::BitReader;
#[doc = "Field `TBS2_2` reader - Transmit Buffer Status 2."]
pub type TBS2_2_R = crate::BitReader<TBS2_2_A>;
#[doc = "Transmit Buffer Status 2.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBS2_2_A {
    #[doc = "0: Locked. Software cannot access the Tx Buffer 2 nor write to the corresponding CANxTFI, CANxTID, CANxTDA, and CANxTDB registers because a message is either waiting for transmission or is in transmitting process."]
    LOCKED = 0,
    #[doc = "1: Released. Software may write a message into the Transmit Buffer 2 and its CANxTFI, CANxTID, CANxTDA, and CANxTDB registers."]
    RELEASED = 1,
}
impl From<TBS2_2_A> for bool {
    #[inline(always)]
    fn from(variant: TBS2_2_A) -> Self {
        variant as u8 != 0
    }
}
impl TBS2_2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TBS2_2_A {
        match self.bits {
            false => TBS2_2_A::LOCKED,
            true => TBS2_2_A::RELEASED,
        }
    }
    #[doc = "Locked. Software cannot access the Tx Buffer 2 nor write to the corresponding CANxTFI, CANxTID, CANxTDA, and CANxTDB registers because a message is either waiting for transmission or is in transmitting process."]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == TBS2_2_A::LOCKED
    }
    #[doc = "Released. Software may write a message into the Transmit Buffer 2 and its CANxTFI, CANxTID, CANxTDA, and CANxTDB registers."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == TBS2_2_A::RELEASED
    }
}
#[doc = "Field `TCS2_2` reader - Transmission Complete Status."]
pub type TCS2_2_R = crate::BitReader<TCS2_2_A>;
#[doc = "Transmission Complete Status.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCS2_2_A {
    #[doc = "0: Incomplete. The previously requested transmission for Tx Buffer 2 is not complete."]
    INCOMPLETE = 0,
    #[doc = "1: Complete. The previously requested transmission for Tx Buffer 2 has been successfully completed."]
    COMPLETE = 1,
}
impl From<TCS2_2_A> for bool {
    #[inline(always)]
    fn from(variant: TCS2_2_A) -> Self {
        variant as u8 != 0
    }
}
impl TCS2_2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCS2_2_A {
        match self.bits {
            false => TCS2_2_A::INCOMPLETE,
            true => TCS2_2_A::COMPLETE,
        }
    }
    #[doc = "Incomplete. The previously requested transmission for Tx Buffer 2 is not complete."]
    #[inline(always)]
    pub fn is_incomplete(&self) -> bool {
        *self == TCS2_2_A::INCOMPLETE
    }
    #[doc = "Complete. The previously requested transmission for Tx Buffer 2 has been successfully completed."]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == TCS2_2_A::COMPLETE
    }
}
#[doc = "Field `RS_2` reader - Receive Status. This bit is identical to the RS bit in the GSR."]
pub type RS_2_R = crate::BitReader;
#[doc = "Field `TS2_2` reader - Transmit Status 2."]
pub type TS2_2_R = crate::BitReader<TS2_2_A>;
#[doc = "Transmit Status 2.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TS2_2_A {
    #[doc = "0: Idle. There is no transmission from Tx Buffer 2."]
    IDLE = 0,
    #[doc = "1: Transmit. The CAN Controller is transmitting a message from Tx Buffer 2."]
    TRANSMIT = 1,
}
impl From<TS2_2_A> for bool {
    #[inline(always)]
    fn from(variant: TS2_2_A) -> Self {
        variant as u8 != 0
    }
}
impl TS2_2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TS2_2_A {
        match self.bits {
            false => TS2_2_A::IDLE,
            true => TS2_2_A::TRANSMIT,
        }
    }
    #[doc = "Idle. There is no transmission from Tx Buffer 2."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == TS2_2_A::IDLE
    }
    #[doc = "Transmit. The CAN Controller is transmitting a message from Tx Buffer 2."]
    #[inline(always)]
    pub fn is_transmit(&self) -> bool {
        *self == TS2_2_A::TRANSMIT
    }
}
#[doc = "Field `ES_2` reader - Error Status. This bit is identical to the ES bit in the CANxGSR."]
pub type ES_2_R = crate::BitReader;
#[doc = "Field `BS_2` reader - Bus Status. This bit is identical to the BS bit in the CANxGSR."]
pub type BS_2_R = crate::BitReader;
#[doc = "Field `RBS_3` reader - Receive Buffer Status. This bit is identical to the RBS bit in the CANxGSR."]
pub type RBS_3_R = crate::BitReader;
#[doc = "Field `DOS_3` reader - Data Overrun Status. This bit is identical to the DOS bit in the CANxGSR."]
pub type DOS_3_R = crate::BitReader;
#[doc = "Field `TBS3_3` reader - Transmit Buffer Status 3."]
pub type TBS3_3_R = crate::BitReader<TBS3_3_A>;
#[doc = "Transmit Buffer Status 3.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBS3_3_A {
    #[doc = "0: Locked. Software cannot access the Tx Buffer 3 nor write to the corresponding CANxTFI, CANxTID, CANxTDA, and CANxTDB registers because a message is either waiting for transmission or is in transmitting process."]
    LOCKED = 0,
    #[doc = "1: Released. Software may write a message into the Transmit Buffer 3 and its CANxTFI, CANxTID, CANxTDA, and CANxTDB registers."]
    RELEASED = 1,
}
impl From<TBS3_3_A> for bool {
    #[inline(always)]
    fn from(variant: TBS3_3_A) -> Self {
        variant as u8 != 0
    }
}
impl TBS3_3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TBS3_3_A {
        match self.bits {
            false => TBS3_3_A::LOCKED,
            true => TBS3_3_A::RELEASED,
        }
    }
    #[doc = "Locked. Software cannot access the Tx Buffer 3 nor write to the corresponding CANxTFI, CANxTID, CANxTDA, and CANxTDB registers because a message is either waiting for transmission or is in transmitting process."]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == TBS3_3_A::LOCKED
    }
    #[doc = "Released. Software may write a message into the Transmit Buffer 3 and its CANxTFI, CANxTID, CANxTDA, and CANxTDB registers."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == TBS3_3_A::RELEASED
    }
}
#[doc = "Field `TCS3_3` reader - Transmission Complete Status."]
pub type TCS3_3_R = crate::BitReader<TCS3_3_A>;
#[doc = "Transmission Complete Status.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCS3_3_A {
    #[doc = "0: Incomplete. The previously requested transmission for Tx Buffer 3 is not complete."]
    INCOMPLETE = 0,
    #[doc = "1: Complete. The previously requested transmission for Tx Buffer 3 has been successfully completed."]
    COMPLETE = 1,
}
impl From<TCS3_3_A> for bool {
    #[inline(always)]
    fn from(variant: TCS3_3_A) -> Self {
        variant as u8 != 0
    }
}
impl TCS3_3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCS3_3_A {
        match self.bits {
            false => TCS3_3_A::INCOMPLETE,
            true => TCS3_3_A::COMPLETE,
        }
    }
    #[doc = "Incomplete. The previously requested transmission for Tx Buffer 3 is not complete."]
    #[inline(always)]
    pub fn is_incomplete(&self) -> bool {
        *self == TCS3_3_A::INCOMPLETE
    }
    #[doc = "Complete. The previously requested transmission for Tx Buffer 3 has been successfully completed."]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == TCS3_3_A::COMPLETE
    }
}
#[doc = "Field `RS_3` reader - Receive Status. This bit is identical to the RS bit in the GSR."]
pub type RS_3_R = crate::BitReader;
#[doc = "Field `TS3_3` reader - Transmit Status 3."]
pub type TS3_3_R = crate::BitReader<TS3_3_A>;
#[doc = "Transmit Status 3.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TS3_3_A {
    #[doc = "0: Idle. There is no transmission from Tx Buffer 3."]
    IDLE = 0,
    #[doc = "1: Transmit. The CAN Controller is transmitting a message from Tx Buffer 3."]
    TRANSMIT = 1,
}
impl From<TS3_3_A> for bool {
    #[inline(always)]
    fn from(variant: TS3_3_A) -> Self {
        variant as u8 != 0
    }
}
impl TS3_3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TS3_3_A {
        match self.bits {
            false => TS3_3_A::IDLE,
            true => TS3_3_A::TRANSMIT,
        }
    }
    #[doc = "Idle. There is no transmission from Tx Buffer 3."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == TS3_3_A::IDLE
    }
    #[doc = "Transmit. The CAN Controller is transmitting a message from Tx Buffer 3."]
    #[inline(always)]
    pub fn is_transmit(&self) -> bool {
        *self == TS3_3_A::TRANSMIT
    }
}
#[doc = "Field `ES_3` reader - Error Status. This bit is identical to the ES bit in the CANxGSR."]
pub type ES_3_R = crate::BitReader;
#[doc = "Field `BS_3` reader - Bus Status. This bit is identical to the BS bit in the CANxGSR."]
pub type BS_3_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Receive Buffer Status. This bit is identical to the RBS bit in the CANxGSR."]
    #[inline(always)]
    pub fn rbs_1(&self) -> RBS_1_R {
        RBS_1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data Overrun Status. This bit is identical to the DOS bit in the CANxGSR."]
    #[inline(always)]
    pub fn dos_1(&self) -> DOS_1_R {
        DOS_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Status 1."]
    #[inline(always)]
    pub fn tbs1_1(&self) -> TBS1_1_R {
        TBS1_1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmission Complete Status."]
    #[inline(always)]
    pub fn tcs1_1(&self) -> TCS1_1_R {
        TCS1_1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Status. This bit is identical to the RS bit in the GSR."]
    #[inline(always)]
    pub fn rs_1(&self) -> RS_1_R {
        RS_1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Status 1."]
    #[inline(always)]
    pub fn ts1_1(&self) -> TS1_1_R {
        TS1_1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Error Status. This bit is identical to the ES bit in the CANxGSR."]
    #[inline(always)]
    pub fn es_1(&self) -> ES_1_R {
        ES_1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus Status. This bit is identical to the BS bit in the CANxGSR."]
    #[inline(always)]
    pub fn bs_1(&self) -> BS_1_R {
        BS_1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Buffer Status. This bit is identical to the RBS bit in the CANxGSR."]
    #[inline(always)]
    pub fn rbs_2(&self) -> RBS_2_R {
        RBS_2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Data Overrun Status. This bit is identical to the DOS bit in the CANxGSR."]
    #[inline(always)]
    pub fn dos_2(&self) -> DOS_2_R {
        DOS_2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmit Buffer Status 2."]
    #[inline(always)]
    pub fn tbs2_2(&self) -> TBS2_2_R {
        TBS2_2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmission Complete Status."]
    #[inline(always)]
    pub fn tcs2_2(&self) -> TCS2_2_R {
        TCS2_2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Receive Status. This bit is identical to the RS bit in the GSR."]
    #[inline(always)]
    pub fn rs_2(&self) -> RS_2_R {
        RS_2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmit Status 2."]
    #[inline(always)]
    pub fn ts2_2(&self) -> TS2_2_R {
        TS2_2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Error Status. This bit is identical to the ES bit in the CANxGSR."]
    #[inline(always)]
    pub fn es_2(&self) -> ES_2_R {
        ES_2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Bus Status. This bit is identical to the BS bit in the CANxGSR."]
    #[inline(always)]
    pub fn bs_2(&self) -> BS_2_R {
        BS_2_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Receive Buffer Status. This bit is identical to the RBS bit in the CANxGSR."]
    #[inline(always)]
    pub fn rbs_3(&self) -> RBS_3_R {
        RBS_3_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Data Overrun Status. This bit is identical to the DOS bit in the CANxGSR."]
    #[inline(always)]
    pub fn dos_3(&self) -> DOS_3_R {
        DOS_3_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Transmit Buffer Status 3."]
    #[inline(always)]
    pub fn tbs3_3(&self) -> TBS3_3_R {
        TBS3_3_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Transmission Complete Status."]
    #[inline(always)]
    pub fn tcs3_3(&self) -> TCS3_3_R {
        TCS3_3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Receive Status. This bit is identical to the RS bit in the GSR."]
    #[inline(always)]
    pub fn rs_3(&self) -> RS_3_R {
        RS_3_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmit Status 3."]
    #[inline(always)]
    pub fn ts3_3(&self) -> TS3_3_R {
        TS3_3_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Error Status. This bit is identical to the ES bit in the CANxGSR."]
    #[inline(always)]
    pub fn es_3(&self) -> ES_3_R {
        ES_3_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Bus Status. This bit is identical to the BS bit in the CANxGSR."]
    #[inline(always)]
    pub fn bs_3(&self) -> BS_3_R {
        BS_3_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("rbs_1", &format_args!("{}", self.rbs_1().bit()))
            .field("dos_1", &format_args!("{}", self.dos_1().bit()))
            .field("tbs1_1", &format_args!("{}", self.tbs1_1().bit()))
            .field("tcs1_1", &format_args!("{}", self.tcs1_1().bit()))
            .field("rs_1", &format_args!("{}", self.rs_1().bit()))
            .field("ts1_1", &format_args!("{}", self.ts1_1().bit()))
            .field("es_1", &format_args!("{}", self.es_1().bit()))
            .field("bs_1", &format_args!("{}", self.bs_1().bit()))
            .field("rbs_2", &format_args!("{}", self.rbs_2().bit()))
            .field("dos_2", &format_args!("{}", self.dos_2().bit()))
            .field("tbs2_2", &format_args!("{}", self.tbs2_2().bit()))
            .field("tcs2_2", &format_args!("{}", self.tcs2_2().bit()))
            .field("rs_2", &format_args!("{}", self.rs_2().bit()))
            .field("ts2_2", &format_args!("{}", self.ts2_2().bit()))
            .field("es_2", &format_args!("{}", self.es_2().bit()))
            .field("bs_2", &format_args!("{}", self.bs_2().bit()))
            .field("rbs_3", &format_args!("{}", self.rbs_3().bit()))
            .field("dos_3", &format_args!("{}", self.dos_3().bit()))
            .field("tbs3_3", &format_args!("{}", self.tbs3_3().bit()))
            .field("tcs3_3", &format_args!("{}", self.tcs3_3().bit()))
            .field("rs_3", &format_args!("{}", self.rs_3().bit()))
            .field("ts3_3", &format_args!("{}", self.ts3_3().bit()))
            .field("es_3", &format_args!("{}", self.es_3().bit()))
            .field("bs_3", &format_args!("{}", self.bs_3().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<SR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SR_SPEC {}
#[doc = "`reset()` method sets SR to value 0x003c_3c3c"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0x003c_3c3c;
}
