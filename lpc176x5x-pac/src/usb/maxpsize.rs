#[doc = "Register `MAXPSIZE` reader"]
pub type R = crate::R<MAXPSIZE_SPEC>;
#[doc = "Register `MAXPSIZE` writer"]
pub type W = crate::W<MAXPSIZE_SPEC>;
#[doc = "Field `MPS` reader - The maximum packet size value."]
pub type MPS_R = crate::FieldReader<u16>;
#[doc = "Field `MPS` writer - The maximum packet size value."]
pub type MPS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
impl R {
    #[doc = "Bits 0:9 - The maximum packet size value."]
    #[inline(always)]
    pub fn mps(&self) -> MPS_R {
        MPS_R::new((self.bits & 0x03ff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAXPSIZE")
            .field("mps", &format_args!("{}", self.mps().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<MAXPSIZE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:9 - The maximum packet size value."]
    #[inline(always)]
    #[must_use]
    pub fn mps(&mut self) -> MPS_W<MAXPSIZE_SPEC, 0> {
        MPS_W::new(self)
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
#[doc = "USB MaxPacketSize\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maxpsize::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maxpsize::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAXPSIZE_SPEC;
impl crate::RegisterSpec for MAXPSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maxpsize::R`](R) reader structure"]
impl crate::Readable for MAXPSIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`maxpsize::W`](W) writer structure"]
impl crate::Writable for MAXPSIZE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAXPSIZE to value 0x08"]
impl crate::Resettable for MAXPSIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
