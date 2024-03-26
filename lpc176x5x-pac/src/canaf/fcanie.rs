#[doc = "Register `FCANIE` reader"]
pub type R = crate::R<FCANIE_SPEC>;
#[doc = "Register `FCANIE` writer"]
pub type W = crate::W<FCANIE_SPEC>;
#[doc = "Field `FCANIE` reader - Global FullCAN Interrupt Enable. When 1, this interrupt is enabled."]
pub type FCANIE_R = crate::BitReader;
#[doc = "Field `FCANIE` writer - Global FullCAN Interrupt Enable. When 1, this interrupt is enabled."]
pub type FCANIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Global FullCAN Interrupt Enable. When 1, this interrupt is enabled."]
    #[inline(always)]
    pub fn fcanie(&self) -> FCANIE_R {
        FCANIE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCANIE")
            .field("fcanie", &format_args!("{}", self.fcanie().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<FCANIE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Global FullCAN Interrupt Enable. When 1, this interrupt is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn fcanie(&mut self) -> FCANIE_W<FCANIE_SPEC, 0> {
        FCANIE_W::new(self)
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
#[doc = "FullCAN interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcanie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcanie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCANIE_SPEC;
impl crate::RegisterSpec for FCANIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcanie::R`](R) reader structure"]
impl crate::Readable for FCANIE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fcanie::W`](W) writer structure"]
impl crate::Writable for FCANIE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCANIE to value 0"]
impl crate::Resettable for FCANIE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
