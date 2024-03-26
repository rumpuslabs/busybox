#[doc = "Register `RXDATA` reader"]
pub type R = crate::R<RXDATA_SPEC>;
#[doc = "Field `RX_DATA` reader - Data received."]
pub type RX_DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data received."]
    #[inline(always)]
    pub fn rx_data(&self) -> RX_DATA_R {
        RX_DATA_R::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXDATA")
            .field("rx_data", &format_args!("{}", self.rx_data().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<RXDATA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "USB Receive Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdata::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXDATA_SPEC;
impl crate::RegisterSpec for RXDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdata::R`](R) reader structure"]
impl crate::Readable for RXDATA_SPEC {}
#[doc = "`reset()` method sets RXDATA to value 0"]
impl crate::Resettable for RXDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
