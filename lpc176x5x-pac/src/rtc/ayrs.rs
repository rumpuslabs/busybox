#[doc = "Register `AYRS` reader"]
pub type R = crate::R<AYRS_SPEC>;
#[doc = "Register `AYRS` writer"]
pub type W = crate::W<AYRS_SPEC>;
#[doc = "Field `YEAR` reader - Year value in the range of 0 to 4095."]
pub type YEAR_R = crate::FieldReader<u16>;
#[doc = "Field `YEAR` writer - Year value in the range of 0 to 4095."]
pub type YEAR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Year value in the range of 0 to 4095."]
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new((self.bits & 0x0fff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AYRS")
            .field("year", &format_args!("{}", self.year().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<AYRS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:11 - Year value in the range of 0 to 4095."]
    #[inline(always)]
    #[must_use]
    pub fn year(&mut self) -> YEAR_W<AYRS_SPEC, 0> {
        YEAR_W::new(self)
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
#[doc = "Alarm value for Year\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ayrs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ayrs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AYRS_SPEC;
impl crate::RegisterSpec for AYRS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ayrs::R`](R) reader structure"]
impl crate::Readable for AYRS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ayrs::W`](W) writer structure"]
impl crate::Writable for AYRS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AYRS to value 0"]
impl crate::Resettable for AYRS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
