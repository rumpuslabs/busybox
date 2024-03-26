#[doc = "Register `HASHFILTERL` reader"]
pub type R = crate::R<HASHFILTERL_SPEC>;
#[doc = "Register `HASHFILTERL` writer"]
pub type W = crate::W<HASHFILTERL_SPEC>;
#[doc = "Field `HFL` reader - HashFilterL. Bits 31:0 of the imperfect filter hash table for receive filtering."]
pub type HFL_R = crate::FieldReader<u32>;
#[doc = "Field `HFL` writer - HashFilterL. Bits 31:0 of the imperfect filter hash table for receive filtering."]
pub type HFL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - HashFilterL. Bits 31:0 of the imperfect filter hash table for receive filtering."]
    #[inline(always)]
    pub fn hfl(&self) -> HFL_R {
        HFL_R::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASHFILTERL")
            .field("hfl", &format_args!("{}", self.hfl().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<HASHFILTERL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - HashFilterL. Bits 31:0 of the imperfect filter hash table for receive filtering."]
    #[inline(always)]
    #[must_use]
    pub fn hfl(&mut self) -> HFL_W<HASHFILTERL_SPEC, 0> {
        HFL_W::new(self)
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
#[doc = "Hash filter table LSBs register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashfilterl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashfilterl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASHFILTERL_SPEC;
impl crate::RegisterSpec for HASHFILTERL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hashfilterl::R`](R) reader structure"]
impl crate::Readable for HASHFILTERL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hashfilterl::W`](W) writer structure"]
impl crate::Writable for HASHFILTERL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HASHFILTERL to value 0"]
impl crate::Resettable for HASHFILTERL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
