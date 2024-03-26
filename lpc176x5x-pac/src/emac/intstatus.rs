#[doc = "Register `INTSTATUS` reader"]
pub type R = crate::R<INTSTATUS_SPEC>;
#[doc = "Field `RXOVERRUNINT` reader - Interrupt set on a fatal overrun error in the receive queue. The fatal interrupt should be resolved by a Rx soft-reset. The bit is not set when there is a nonfatal overrun error."]
pub type RXOVERRUNINT_R = crate::BitReader;
#[doc = "Field `RXERRORINT` reader - Interrupt trigger on receive errors: AlignmentError, RangeError, LengthError, SymbolError, CRCError or NoDescriptor or Overrun."]
pub type RXERRORINT_R = crate::BitReader;
#[doc = "Field `RXFINISHEDINT` reader - Interrupt triggered when all receive descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
pub type RXFINISHEDINT_R = crate::BitReader;
#[doc = "Field `RXDONEINT` reader - Interrupt triggered when a receive descriptor has been processed while the Interrupt bit in the Control field of the descriptor was set."]
pub type RXDONEINT_R = crate::BitReader;
#[doc = "Field `TXUNDERRUNINT` reader - Interrupt set on a fatal underrun error in the transmit queue. The fatal interrupt should be resolved by a Tx soft-reset. The bit is not set when there is a nonfatal underrun error."]
pub type TXUNDERRUNINT_R = crate::BitReader;
#[doc = "Field `TXERRORINT` reader - Interrupt trigger on transmit errors: LateCollision, ExcessiveCollision and ExcessiveDefer, NoDescriptor or Underrun."]
pub type TXERRORINT_R = crate::BitReader;
#[doc = "Field `TXFINISHEDINT` reader - Interrupt triggered when all transmit descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
pub type TXFINISHEDINT_R = crate::BitReader;
#[doc = "Field `TXDONEINT` reader - Interrupt triggered when a descriptor has been transmitted while the Interrupt bit in the Control field of the descriptor was set."]
pub type TXDONEINT_R = crate::BitReader;
#[doc = "Field `SOFTINT` reader - Interrupt triggered by software writing a 1 to the SoftIntSet bit in the IntSet register."]
pub type SOFTINT_R = crate::BitReader;
#[doc = "Field `WAKEUPINT` reader - Interrupt triggered by a Wake-up event detected by the receive filter."]
pub type WAKEUPINT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt set on a fatal overrun error in the receive queue. The fatal interrupt should be resolved by a Rx soft-reset. The bit is not set when there is a nonfatal overrun error."]
    #[inline(always)]
    pub fn rxoverrunint(&self) -> RXOVERRUNINT_R {
        RXOVERRUNINT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt trigger on receive errors: AlignmentError, RangeError, LengthError, SymbolError, CRCError or NoDescriptor or Overrun."]
    #[inline(always)]
    pub fn rxerrorint(&self) -> RXERRORINT_R {
        RXERRORINT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt triggered when all receive descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
    #[inline(always)]
    pub fn rxfinishedint(&self) -> RXFINISHEDINT_R {
        RXFINISHEDINT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt triggered when a receive descriptor has been processed while the Interrupt bit in the Control field of the descriptor was set."]
    #[inline(always)]
    pub fn rxdoneint(&self) -> RXDONEINT_R {
        RXDONEINT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt set on a fatal underrun error in the transmit queue. The fatal interrupt should be resolved by a Tx soft-reset. The bit is not set when there is a nonfatal underrun error."]
    #[inline(always)]
    pub fn txunderrunint(&self) -> TXUNDERRUNINT_R {
        TXUNDERRUNINT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt trigger on transmit errors: LateCollision, ExcessiveCollision and ExcessiveDefer, NoDescriptor or Underrun."]
    #[inline(always)]
    pub fn txerrorint(&self) -> TXERRORINT_R {
        TXERRORINT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt triggered when all transmit descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
    #[inline(always)]
    pub fn txfinishedint(&self) -> TXFINISHEDINT_R {
        TXFINISHEDINT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt triggered when a descriptor has been transmitted while the Interrupt bit in the Control field of the descriptor was set."]
    #[inline(always)]
    pub fn txdoneint(&self) -> TXDONEINT_R {
        TXDONEINT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt triggered by software writing a 1 to the SoftIntSet bit in the IntSet register."]
    #[inline(always)]
    pub fn softint(&self) -> SOFTINT_R {
        SOFTINT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt triggered by a Wake-up event detected by the receive filter."]
    #[inline(always)]
    pub fn wakeupint(&self) -> WAKEUPINT_R {
        WAKEUPINT_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTSTATUS")
            .field(
                "rxoverrunint",
                &format_args!("{}", self.rxoverrunint().bit()),
            )
            .field("rxerrorint", &format_args!("{}", self.rxerrorint().bit()))
            .field(
                "rxfinishedint",
                &format_args!("{}", self.rxfinishedint().bit()),
            )
            .field("rxdoneint", &format_args!("{}", self.rxdoneint().bit()))
            .field(
                "txunderrunint",
                &format_args!("{}", self.txunderrunint().bit()),
            )
            .field("txerrorint", &format_args!("{}", self.txerrorint().bit()))
            .field(
                "txfinishedint",
                &format_args!("{}", self.txfinishedint().bit()),
            )
            .field("txdoneint", &format_args!("{}", self.txdoneint().bit()))
            .field("softint", &format_args!("{}", self.softint().bit()))
            .field("wakeupint", &format_args!("{}", self.wakeupint().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<INTSTATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Interrupt status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTSTATUS_SPEC;
impl crate::RegisterSpec for INTSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstatus::R`](R) reader structure"]
impl crate::Readable for INTSTATUS_SPEC {}
#[doc = "`reset()` method sets INTSTATUS to value 0"]
impl crate::Resettable for INTSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
