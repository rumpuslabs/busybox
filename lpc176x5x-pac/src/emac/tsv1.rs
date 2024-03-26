#[doc = "Register `TSV1` reader"]
pub type R = crate::R<TSV1_SPEC>;
#[doc = "Field `TBC` reader - Transmit byte count. The total number of bytes in the frame, not counting the collided bytes."]
pub type TBC_R = crate::FieldReader<u16>;
#[doc = "Field `TCC` reader - Transmit collision count. Number of collisions the current packet incurred during transmission attempts. The maximum number of collisions (16) cannot be represented."]
pub type TCC_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Transmit byte count. The total number of bytes in the frame, not counting the collided bytes."]
    #[inline(always)]
    pub fn tbc(&self) -> TBC_R {
        TBC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Transmit collision count. Number of collisions the current packet incurred during transmission attempts. The maximum number of collisions (16) cannot be represented."]
    #[inline(always)]
    pub fn tcc(&self) -> TCC_R {
        TCC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSV1")
            .field("tbc", &format_args!("{}", self.tbc().bits()))
            .field("tcc", &format_args!("{}", self.tcc().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<TSV1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Transmit status vector 1 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsv1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSV1_SPEC;
impl crate::RegisterSpec for TSV1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsv1::R`](R) reader structure"]
impl crate::Readable for TSV1_SPEC {}
#[doc = "`reset()` method sets TSV1 to value 0"]
impl crate::Resettable for TSV1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
