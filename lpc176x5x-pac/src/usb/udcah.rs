#[doc = "Register `UDCAH` reader"]
pub type R = crate::R<UDCAH_SPEC>;
#[doc = "Register `UDCAH` writer"]
pub type W = crate::W<UDCAH_SPEC>;
#[doc = "Field `UDCA_ADDR` reader - Start address of the UDCA."]
pub type UDCA_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `UDCA_ADDR` writer - Start address of the UDCA."]
pub type UDCA_ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 25, O, u32>;
impl R {
    #[doc = "Bits 7:31 - Start address of the UDCA."]
    #[inline(always)]
    pub fn udca_addr(&self) -> UDCA_ADDR_R {
        UDCA_ADDR_R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UDCAH")
            .field("udca_addr", &format_args!("{}", self.udca_addr().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<UDCAH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 7:31 - Start address of the UDCA."]
    #[inline(always)]
    #[must_use]
    pub fn udca_addr(&mut self) -> UDCA_ADDR_W<UDCAH_SPEC, 7> {
        UDCA_ADDR_W::new(self)
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
#[doc = "USB UDCA Head\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udcah::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udcah::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UDCAH_SPEC;
impl crate::RegisterSpec for UDCAH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udcah::R`](R) reader structure"]
impl crate::Readable for UDCAH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`udcah::W`](W) writer structure"]
impl crate::Writable for UDCAH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UDCAH to value 0"]
impl crate::Resettable for UDCAH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
