#[doc = "Register `ADOY` reader"]
pub type R = crate::R<ADOY_SPEC>;
#[doc = "Register `ADOY` writer"]
pub type W = crate::W<ADOY_SPEC>;
#[doc = "Field `DOY` reader - Day of year value in the range of 1 to 365 (366 for leap years)."]
pub type DOY_R = crate::FieldReader<u16>;
#[doc = "Field `DOY` writer - Day of year value in the range of 1 to 365 (366 for leap years)."]
pub type DOY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
impl R {
    #[doc = "Bits 0:8 - Day of year value in the range of 1 to 365 (366 for leap years)."]
    #[inline(always)]
    pub fn doy(&self) -> DOY_R {
        DOY_R::new((self.bits & 0x01ff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADOY")
            .field("doy", &format_args!("{}", self.doy().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<ADOY_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:8 - Day of year value in the range of 1 to 365 (366 for leap years)."]
    #[inline(always)]
    #[must_use]
    pub fn doy(&mut self) -> DOY_W<ADOY_SPEC, 0> {
        DOY_W::new(self)
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
#[doc = "Alarm value for Day of Year\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adoy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adoy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADOY_SPEC;
impl crate::RegisterSpec for ADOY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adoy::R`](R) reader structure"]
impl crate::Readable for ADOY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adoy::W`](W) writer structure"]
impl crate::Writable for ADOY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADOY to value 0"]
impl crate::Resettable for ADOY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
