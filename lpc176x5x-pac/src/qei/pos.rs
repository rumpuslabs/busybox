#[doc = "Register `POS` reader"]
pub type R = crate::R<POS_SPEC>;
#[doc = "Field `POS` reader - Current position value."]
pub type POS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current position value."]
    #[inline(always)]
    pub fn pos(&self) -> POS_R {
        POS_R::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POS")
            .field("pos", &format_args!("{}", self.pos().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<POS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Position register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pos::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POS_SPEC;
impl crate::RegisterSpec for POS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pos::R`](R) reader structure"]
impl crate::Readable for POS_SPEC {}
#[doc = "`reset()` method sets POS to value 0"]
impl crate::Resettable for POS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
