#[doc = "Register `TID%s` reader"]
pub type R = crate::R<TID_SPEC>;
#[doc = "Register `TID%s` writer"]
pub type W = crate::W<TID_SPEC>;
#[doc = "Field `ID` reader - The 11-bit Identifier to be sent in the next transmit message."]
pub type ID_R = crate::FieldReader<u16>;
#[doc = "Field `ID` writer - The 11-bit Identifier to be sent in the next transmit message."]
pub type ID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
impl R {
    #[doc = "Bits 0:10 - The 11-bit Identifier to be sent in the next transmit message."]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x07ff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TID")
            .field("id", &format_args!("{}", self.id().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<TID_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:10 - The 11-bit Identifier to be sent in the next transmit message."]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> ID_W<TID_SPEC, 0> {
        ID_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transmit Identifier (Tx Buffer)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TID_SPEC;
impl crate::RegisterSpec for TID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tid::R`](R) reader structure"]
impl crate::Readable for TID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tid::W`](W) writer structure"]
impl crate::Writable for TID_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TID%s to value 0"]
impl crate::Resettable for TID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
