#[doc = "Register `RS485ADRMATCH` reader"]
pub type R = crate::R<RS485ADRMATCH_SPEC>;
#[doc = "Register `RS485ADRMATCH` writer"]
pub type W = crate::W<RS485ADRMATCH_SPEC>;
#[doc = "Field `ADRMATCH` reader - Contains the address match value."]
pub type ADRMATCH_R = crate::FieldReader;
#[doc = "Field `ADRMATCH` writer - Contains the address match value."]
pub type ADRMATCH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Contains the address match value."]
    #[inline(always)]
    pub fn adrmatch(&self) -> ADRMATCH_R {
        ADRMATCH_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RS485ADRMATCH")
            .field("adrmatch", &format_args!("{}", self.adrmatch().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<RS485ADRMATCH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Contains the address match value."]
    #[inline(always)]
    #[must_use]
    pub fn adrmatch(&mut self) -> ADRMATCH_W<RS485ADRMATCH_SPEC, 0> {
        ADRMATCH_W::new(self)
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
#[doc = "RS-485/EIA-485 address match. Contains the address match value for RS-485/EIA-485 mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rs485adrmatch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rs485adrmatch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RS485ADRMATCH_SPEC;
impl crate::RegisterSpec for RS485ADRMATCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rs485adrmatch::R`](R) reader structure"]
impl crate::Readable for RS485ADRMATCH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rs485adrmatch::W`](W) writer structure"]
impl crate::Writable for RS485ADRMATCH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RS485ADRMATCH to value 0"]
impl crate::Resettable for RS485ADRMATCH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
