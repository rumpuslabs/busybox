#[doc = "Register `LSR` reader"]
pub type R = crate::R<LSR_SPEC>;
#[doc = "Field `RDR` reader - Receiver Data Ready. LSR\\[0\\]
is set when the RBR holds an unread character and is cleared when the UART1 RBR FIFO is empty."]
pub type RDR_R = crate::BitReader<RDR_A>;
#[doc = "Receiver Data Ready. LSR\\[0\\]
is set when the RBR holds an unread character and is cleared when the UART1 RBR FIFO is empty.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDR_A {
    #[doc = "0: The UART1 receiver FIFO is empty."]
    EMPTY = 0,
    #[doc = "1: The UART1 receiver FIFO is not empty."]
    NOTEMPTY = 1,
}
impl From<RDR_A> for bool {
    #[inline(always)]
    fn from(variant: RDR_A) -> Self {
        variant as u8 != 0
    }
}
impl RDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RDR_A {
        match self.bits {
            false => RDR_A::EMPTY,
            true => RDR_A::NOTEMPTY,
        }
    }
    #[doc = "The UART1 receiver FIFO is empty."]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RDR_A::EMPTY
    }
    #[doc = "The UART1 receiver FIFO is not empty."]
    #[inline(always)]
    pub fn is_notempty(&self) -> bool {
        *self == RDR_A::NOTEMPTY
    }
}
#[doc = "Field `OE` reader - Overrun Error. The overrun error condition is set as soon as it occurs. An LSR read clears LSR\\[1\\]. LSR\\[1\\]
is set when UART1 RSR has a new character assembled and the UART1 RBR FIFO is full. In this case, the UART1 RBR FIFO will not be overwritten and the character in the UART1 RSR will be lost."]
pub type OE_R = crate::BitReader<OE_A>;
#[doc = "Overrun Error. The overrun error condition is set as soon as it occurs. An LSR read clears LSR\\[1\\]. LSR\\[1\\]
is set when UART1 RSR has a new character assembled and the UART1 RBR FIFO is full. In this case, the UART1 RBR FIFO will not be overwritten and the character in the UART1 RSR will be lost.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OE_A {
    #[doc = "0: Overrun error status is inactive."]
    INACTIVE = 0,
    #[doc = "1: Overrun error status is active."]
    ACTIVE = 1,
}
impl From<OE_A> for bool {
    #[inline(always)]
    fn from(variant: OE_A) -> Self {
        variant as u8 != 0
    }
}
impl OE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OE_A {
        match self.bits {
            false => OE_A::INACTIVE,
            true => OE_A::ACTIVE,
        }
    }
    #[doc = "Overrun error status is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == OE_A::INACTIVE
    }
    #[doc = "Overrun error status is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == OE_A::ACTIVE
    }
}
#[doc = "Field `PE` reader - Parity Error. When the parity bit of a received character is in the wrong state, a parity error occurs. An LSR read clears LSR\\[2\\]. Time of parity error detection is dependent on FCR\\[0\\]. Note: A parity error is associated with the character at the top of the UART1 RBR FIFO."]
pub type PE_R = crate::BitReader<PE_A>;
#[doc = "Parity Error. When the parity bit of a received character is in the wrong state, a parity error occurs. An LSR read clears LSR\\[2\\]. Time of parity error detection is dependent on FCR\\[0\\]. Note: A parity error is associated with the character at the top of the UART1 RBR FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PE_A {
    #[doc = "0: Parity error status is inactive."]
    INACTIVE = 0,
    #[doc = "1: Parity error status is active."]
    ACTIVE = 1,
}
impl From<PE_A> for bool {
    #[inline(always)]
    fn from(variant: PE_A) -> Self {
        variant as u8 != 0
    }
}
impl PE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PE_A {
        match self.bits {
            false => PE_A::INACTIVE,
            true => PE_A::ACTIVE,
        }
    }
    #[doc = "Parity error status is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == PE_A::INACTIVE
    }
    #[doc = "Parity error status is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == PE_A::ACTIVE
    }
}
#[doc = "Field `FE` reader - Framing Error. When the stop bit of a received character is a logic 0, a framing error occurs. An LSR read clears LSR\\[3\\]. The time of the framing error detection is dependent on FCR0. Upon detection of a framing error, the RX will attempt to resynchronize to the data and assume that the bad stop bit is actually an early start bit. However, it cannot be assumed that the next received byte will be correct even if there is no Framing Error. Note: A framing error is associated with the character at the top of the UART1 RBR FIFO."]
pub type FE_R = crate::BitReader<FE_A>;
#[doc = "Framing Error. When the stop bit of a received character is a logic 0, a framing error occurs. An LSR read clears LSR\\[3\\]. The time of the framing error detection is dependent on FCR0. Upon detection of a framing error, the RX will attempt to resynchronize to the data and assume that the bad stop bit is actually an early start bit. However, it cannot be assumed that the next received byte will be correct even if there is no Framing Error. Note: A framing error is associated with the character at the top of the UART1 RBR FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FE_A {
    #[doc = "0: Framing error status is inactive."]
    INACTIVE = 0,
    #[doc = "1: Framing error status is active."]
    ACTIVE = 1,
}
impl From<FE_A> for bool {
    #[inline(always)]
    fn from(variant: FE_A) -> Self {
        variant as u8 != 0
    }
}
impl FE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FE_A {
        match self.bits {
            false => FE_A::INACTIVE,
            true => FE_A::ACTIVE,
        }
    }
    #[doc = "Framing error status is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == FE_A::INACTIVE
    }
    #[doc = "Framing error status is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == FE_A::ACTIVE
    }
}
#[doc = "Field `BI` reader - Break Interrupt. When RXD1 is held in the spacing state (all zeroes) for one full character transmission (start, data, parity, stop), a break interrupt occurs. Once the break condition has been detected, the receiver goes idle until RXD1 goes to marking state (all ones). An LSR read clears this status bit. The time of break detection is dependent on FCR\\[0\\]. Note: The break interrupt is associated with the character at the top of the UART1 RBR FIFO."]
pub type BI_R = crate::BitReader<BI_A>;
#[doc = "Break Interrupt. When RXD1 is held in the spacing state (all zeroes) for one full character transmission (start, data, parity, stop), a break interrupt occurs. Once the break condition has been detected, the receiver goes idle until RXD1 goes to marking state (all ones). An LSR read clears this status bit. The time of break detection is dependent on FCR\\[0\\]. Note: The break interrupt is associated with the character at the top of the UART1 RBR FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BI_A {
    #[doc = "0: Break interrupt status is inactive."]
    INACTIVE = 0,
    #[doc = "1: Break interrupt status is active."]
    ACTIVE = 1,
}
impl From<BI_A> for bool {
    #[inline(always)]
    fn from(variant: BI_A) -> Self {
        variant as u8 != 0
    }
}
impl BI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BI_A {
        match self.bits {
            false => BI_A::INACTIVE,
            true => BI_A::ACTIVE,
        }
    }
    #[doc = "Break interrupt status is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == BI_A::INACTIVE
    }
    #[doc = "Break interrupt status is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == BI_A::ACTIVE
    }
}
#[doc = "Field `THRE` reader - Transmitter Holding Register Empty. THRE is set immediately upon detection of an empty UART1 THR and is cleared on a THR write."]
pub type THRE_R = crate::BitReader<THRE_A>;
#[doc = "Transmitter Holding Register Empty. THRE is set immediately upon detection of an empty UART1 THR and is cleared on a THR write.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum THRE_A {
    #[doc = "0: THR contains valid data."]
    VALID = 0,
    #[doc = "1: THR is empty."]
    EMPTY = 1,
}
impl From<THRE_A> for bool {
    #[inline(always)]
    fn from(variant: THRE_A) -> Self {
        variant as u8 != 0
    }
}
impl THRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> THRE_A {
        match self.bits {
            false => THRE_A::VALID,
            true => THRE_A::EMPTY,
        }
    }
    #[doc = "THR contains valid data."]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == THRE_A::VALID
    }
    #[doc = "THR is empty."]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == THRE_A::EMPTY
    }
}
#[doc = "Field `TEMT` reader - Transmitter Empty. TEMT is set when both THR and TSR are empty; TEMT is cleared when either the TSR or the THR contain valid data."]
pub type TEMT_R = crate::BitReader<TEMT_A>;
#[doc = "Transmitter Empty. TEMT is set when both THR and TSR are empty; TEMT is cleared when either the TSR or the THR contain valid data.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEMT_A {
    #[doc = "0: THR and/or the TSR contains valid data."]
    VALID = 0,
    #[doc = "1: THR and the TSR are empty."]
    EMPTY = 1,
}
impl From<TEMT_A> for bool {
    #[inline(always)]
    fn from(variant: TEMT_A) -> Self {
        variant as u8 != 0
    }
}
impl TEMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TEMT_A {
        match self.bits {
            false => TEMT_A::VALID,
            true => TEMT_A::EMPTY,
        }
    }
    #[doc = "THR and/or the TSR contains valid data."]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == TEMT_A::VALID
    }
    #[doc = "THR and the TSR are empty."]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TEMT_A::EMPTY
    }
}
#[doc = "Field `RXFE` reader - Error in RX FIFO. LSR\\[7\\]
is set when a character with a RX error such as framing error, parity error or break interrupt, is loaded into the RBR. This bit is cleared when the LSR register is read and there are no subsequent errors in the UART1 FIFO."]
pub type RXFE_R = crate::BitReader<RXFE_A>;
#[doc = "Error in RX FIFO. LSR\\[7\\]
is set when a character with a RX error such as framing error, parity error or break interrupt, is loaded into the RBR. This bit is cleared when the LSR register is read and there are no subsequent errors in the UART1 FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFE_A {
    #[doc = "0: RBR contains no UART1 RX errors or FCR\\[0\\]=0."]
    NOERROR = 0,
    #[doc = "1: UART1 RBR contains at least one UART1 RX error."]
    ERRORS = 1,
}
impl From<RXFE_A> for bool {
    #[inline(always)]
    fn from(variant: RXFE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXFE_A {
        match self.bits {
            false => RXFE_A::NOERROR,
            true => RXFE_A::ERRORS,
        }
    }
    #[doc = "RBR contains no UART1 RX errors or FCR\\[0\\]=0."]
    #[inline(always)]
    pub fn is_noerror(&self) -> bool {
        *self == RXFE_A::NOERROR
    }
    #[doc = "UART1 RBR contains at least one UART1 RX error."]
    #[inline(always)]
    pub fn is_errors(&self) -> bool {
        *self == RXFE_A::ERRORS
    }
}
impl R {
    #[doc = "Bit 0 - Receiver Data Ready. LSR\\[0\\]
is set when the RBR holds an unread character and is cleared when the UART1 RBR FIFO is empty."]
    #[inline(always)]
    pub fn rdr(&self) -> RDR_R {
        RDR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun Error. The overrun error condition is set as soon as it occurs. An LSR read clears LSR\\[1\\]. LSR\\[1\\]
is set when UART1 RSR has a new character assembled and the UART1 RBR FIFO is full. In this case, the UART1 RBR FIFO will not be overwritten and the character in the UART1 RSR will be lost."]
    #[inline(always)]
    pub fn oe(&self) -> OE_R {
        OE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Parity Error. When the parity bit of a received character is in the wrong state, a parity error occurs. An LSR read clears LSR\\[2\\]. Time of parity error detection is dependent on FCR\\[0\\]. Note: A parity error is associated with the character at the top of the UART1 RBR FIFO."]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Framing Error. When the stop bit of a received character is a logic 0, a framing error occurs. An LSR read clears LSR\\[3\\]. The time of the framing error detection is dependent on FCR0. Upon detection of a framing error, the RX will attempt to resynchronize to the data and assume that the bad stop bit is actually an early start bit. However, it cannot be assumed that the next received byte will be correct even if there is no Framing Error. Note: A framing error is associated with the character at the top of the UART1 RBR FIFO."]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Break Interrupt. When RXD1 is held in the spacing state (all zeroes) for one full character transmission (start, data, parity, stop), a break interrupt occurs. Once the break condition has been detected, the receiver goes idle until RXD1 goes to marking state (all ones). An LSR read clears this status bit. The time of break detection is dependent on FCR\\[0\\]. Note: The break interrupt is associated with the character at the top of the UART1 RBR FIFO."]
    #[inline(always)]
    pub fn bi(&self) -> BI_R {
        BI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmitter Holding Register Empty. THRE is set immediately upon detection of an empty UART1 THR and is cleared on a THR write."]
    #[inline(always)]
    pub fn thre(&self) -> THRE_R {
        THRE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmitter Empty. TEMT is set when both THR and TSR are empty; TEMT is cleared when either the TSR or the THR contain valid data."]
    #[inline(always)]
    pub fn temt(&self) -> TEMT_R {
        TEMT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Error in RX FIFO. LSR\\[7\\]
is set when a character with a RX error such as framing error, parity error or break interrupt, is loaded into the RBR. This bit is cleared when the LSR register is read and there are no subsequent errors in the UART1 FIFO."]
    #[inline(always)]
    pub fn rxfe(&self) -> RXFE_R {
        RXFE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<LSR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
#[doc = "Line Status Register. Contains flags for transmit and receive status, including line errors.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsr::R`](R). WARN: The register is **modified** in some way after a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LSR_SPEC;
impl crate::RegisterSpec for LSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsr::R`](R) reader structure"]
impl crate::Readable for LSR_SPEC {}
#[doc = "`reset()` method sets LSR to value 0x60"]
impl crate::Resettable for LSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x60;
}
