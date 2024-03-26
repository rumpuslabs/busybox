#[doc = "Register `DATA_BUFFER` reader"]
pub type R = crate::R<DATA_BUFFER_SPEC>;
#[doc = "Field `Data` reader - This register holds contents of the 8 MSBs of the DAT shift register."]
pub type DATA_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - This register holds contents of the 8 MSBs of the DAT shift register."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA_BUFFER")
            .field("data", &format_args!("{}", self.data().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<DATA_BUFFER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Data buffer register. The contents of the 8 MSBs of the DAT shift register will be transferred to the DATA_BUFFER automatically after every nine bits (8 bits of data plus ACK or NACK) has been received on the bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_buffer::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_BUFFER_SPEC;
impl crate::RegisterSpec for DATA_BUFFER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_buffer::R`](R) reader structure"]
impl crate::Readable for DATA_BUFFER_SPEC {}
#[doc = "`reset()` method sets DATA_BUFFER to value 0"]
impl crate::Resettable for DATA_BUFFER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
