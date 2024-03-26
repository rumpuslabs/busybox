#[doc = "Register `GPREG%s` reader"]
pub type R = crate::R<GPREG_SPEC>;
#[doc = "Register `GPREG%s` writer"]
pub type W = crate::W<GPREG_SPEC>;
#[doc = "Field `GP` reader - General purpose storage."]
pub type GP_R = crate::FieldReader<u32>;
#[doc = "Field `GP` writer - General purpose storage."]
pub type GP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - General purpose storage."]
    #[inline(always)]
    pub fn gp(&self) -> GP_R {
        GP_R::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPREG")
            .field("gp", &format_args!("{}", self.gp().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<GPREG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - General purpose storage."]
    #[inline(always)]
    #[must_use]
    pub fn gp(&mut self) -> GP_W<GPREG_SPEC, 0> {
        GP_W::new(self)
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
#[doc = "General Purpose Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpreg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpreg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPREG_SPEC;
impl crate::RegisterSpec for GPREG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpreg::R`](R) reader structure"]
impl crate::Readable for GPREG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpreg::W`](W) writer structure"]
impl crate::Writable for GPREG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPREG%s to value 0"]
impl crate::Resettable for GPREG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
