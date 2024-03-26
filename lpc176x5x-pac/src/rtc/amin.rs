#[doc = "Register `AMIN` reader"]
pub type R = crate::R<AMIN_SPEC>;
#[doc = "Register `AMIN` writer"]
pub type W = crate::W<AMIN_SPEC>;
#[doc = "Field `MINUTES` reader - Minutes value in the range of 0 to 59"]
pub type MINUTES_R = crate::FieldReader;
#[doc = "Field `MINUTES` writer - Minutes value in the range of 0 to 59"]
pub type MINUTES_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Minutes value in the range of 0 to 59"]
    #[inline(always)]
    pub fn minutes(&self) -> MINUTES_R {
        MINUTES_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AMIN")
            .field("minutes", &format_args!("{}", self.minutes().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<AMIN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:5 - Minutes value in the range of 0 to 59"]
    #[inline(always)]
    #[must_use]
    pub fn minutes(&mut self) -> MINUTES_W<AMIN_SPEC, 0> {
        MINUTES_W::new(self)
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
#[doc = "Alarm value for Minutes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`amin::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`amin::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AMIN_SPEC;
impl crate::RegisterSpec for AMIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`amin::R`](R) reader structure"]
impl crate::Readable for AMIN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`amin::W`](W) writer structure"]
impl crate::Writable for AMIN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AMIN to value 0"]
impl crate::Resettable for AMIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
