#[doc = "Register `HASHFILTERH` reader"]
pub type R = crate::R<HASHFILTERH_SPEC>;
#[doc = "Register `HASHFILTERH` writer"]
pub type W = crate::W<HASHFILTERH_SPEC>;
#[doc = "Field `HFH` reader - Bits 63:32 of the imperfect filter hash table for receive filtering."]
pub type HFH_R = crate::FieldReader<u32>;
#[doc = "Field `HFH` writer - Bits 63:32 of the imperfect filter hash table for receive filtering."]
pub type HFH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Bits 63:32 of the imperfect filter hash table for receive filtering."]
    #[inline(always)]
    pub fn hfh(&self) -> HFH_R {
        HFH_R::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASHFILTERH")
            .field("hfh", &format_args!("{}", self.hfh().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<HASHFILTERH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bits 63:32 of the imperfect filter hash table for receive filtering."]
    #[inline(always)]
    #[must_use]
    pub fn hfh(&mut self) -> HFH_W<HASHFILTERH_SPEC, 0> {
        HFH_W::new(self)
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
#[doc = "Hash filter table MSBs register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashfilterh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashfilterh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASHFILTERH_SPEC;
impl crate::RegisterSpec for HASHFILTERH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hashfilterh::R`](R) reader structure"]
impl crate::Readable for HASHFILTERH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hashfilterh::W`](W) writer structure"]
impl crate::Writable for HASHFILTERH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HASHFILTERH to value 0"]
impl crate::Resettable for HASHFILTERH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
