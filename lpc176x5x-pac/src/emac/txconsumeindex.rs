#[doc = "Register `TXCONSUMEINDEX` reader"]
pub type R = crate::R<TXCONSUMEINDEX_SPEC>;
#[doc = "Field `TXCI` reader - TxConsumeIndex. Index of the descriptor that is going to be transmitted next by the transmit datapath."]
pub type TXCI_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - TxConsumeIndex. Index of the descriptor that is going to be transmitted next by the transmit datapath."]
    #[inline(always)]
    pub fn txci(&self) -> TXCI_R {
        TXCI_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXCONSUMEINDEX")
            .field("txci", &format_args!("{}", self.txci().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<TXCONSUMEINDEX_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Transmit consume index register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txconsumeindex::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXCONSUMEINDEX_SPEC;
impl crate::RegisterSpec for TXCONSUMEINDEX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txconsumeindex::R`](R) reader structure"]
impl crate::Readable for TXCONSUMEINDEX_SPEC {}
#[doc = "`reset()` method sets TXCONSUMEINDEX to value 0"]
impl crate::Resettable for TXCONSUMEINDEX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
