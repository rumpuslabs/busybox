#[doc = "Register `RBR` reader"]
pub type R = crate::R<RBR_SPEC>;
#[doc = "Field `RBR` reader - The UART1 Receiver Buffer Register contains the oldest received byte in the UART1 RX FIFO."]
pub type RBR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - The UART1 Receiver Buffer Register contains the oldest received byte in the UART1 RX FIFO."]
    #[inline(always)]
    pub fn rbr(&self) -> RBR_R {
        RBR_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<RBR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
#[doc = "DLAB =0 Receiver Buffer Register. Contains the next received character to be read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbr::R`](R). WARN: The register is **modified** in some way after a read operation. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RBR_SPEC;
impl crate::RegisterSpec for RBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbr::R`](R) reader structure"]
impl crate::Readable for RBR_SPEC {}
#[doc = "`reset()` method sets RBR to value 0"]
impl crate::Resettable for RBR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
