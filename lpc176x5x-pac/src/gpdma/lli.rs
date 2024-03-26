#[doc = "Register `LLI%s` reader"]
pub type R = crate::R<LLI_SPEC>;
#[doc = "Register `LLI%s` writer"]
pub type W = crate::W<LLI_SPEC>;
#[doc = "Field `LLI` reader - Linked list item. Bits \\[31:2\\]
of the address for the next LLI. Address bits \\[1:0\\]
are 0."]
pub type LLI_R = crate::FieldReader<u32>;
#[doc = "Field `LLI` writer - Linked list item. Bits \\[31:2\\]
of the address for the next LLI. Address bits \\[1:0\\]
are 0."]
pub type LLI_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 30, O, u32>;
impl R {
    #[doc = "Bits 2:31 - Linked list item. Bits \\[31:2\\]
of the address for the next LLI. Address bits \\[1:0\\]
are 0."]
    #[inline(always)]
    pub fn lli(&self) -> LLI_R {
        LLI_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LLI")
            .field("lli", &format_args!("{}", self.lli().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<LLI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 2:31 - Linked list item. Bits \\[31:2\\]
of the address for the next LLI. Address bits \\[1:0\\]
are 0."]
    #[inline(always)]
    #[must_use]
    pub fn lli(&mut self) -> LLI_W<LLI_SPEC, 2> {
        LLI_W::new(self)
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
#[doc = "DMA Channel 0 Linked List Item Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lli::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lli::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LLI_SPEC;
impl crate::RegisterSpec for LLI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lli::R`](R) reader structure"]
impl crate::Readable for LLI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lli::W`](W) writer structure"]
impl crate::Writable for LLI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LLI%s to value 0"]
impl crate::Resettable for LLI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
