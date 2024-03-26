#[doc = "Register `FLOWCONTROLSTATUS` reader"]
pub type R = crate::R<FLOWCONTROLSTATUS_SPEC>;
#[doc = "Field `MCC` reader - MirrorCounterCurrent. In full duplex mode this register represents the current value of the datapath's mirror counter which counts up to the value specified by the MirrorCounter field in the FlowControlCounter register. In half duplex mode the register counts until it reaches the value of the PauseTimer bits in the FlowControlCounter register."]
pub type MCC_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - MirrorCounterCurrent. In full duplex mode this register represents the current value of the datapath's mirror counter which counts up to the value specified by the MirrorCounter field in the FlowControlCounter register. In half duplex mode the register counts until it reaches the value of the PauseTimer bits in the FlowControlCounter register."]
    #[inline(always)]
    pub fn mcc(&self) -> MCC_R {
        MCC_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLOWCONTROLSTATUS")
            .field("mcc", &format_args!("{}", self.mcc().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<FLOWCONTROLSTATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Flow control status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flowcontrolstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLOWCONTROLSTATUS_SPEC;
impl crate::RegisterSpec for FLOWCONTROLSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flowcontrolstatus::R`](R) reader structure"]
impl crate::Readable for FLOWCONTROLSTATUS_SPEC {}
#[doc = "`reset()` method sets FLOWCONTROLSTATUS to value 0"]
impl crate::Resettable for FLOWCONTROLSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
