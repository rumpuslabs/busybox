#[doc = "Register `MWTD` writer"]
pub type W = crate::W<MWTD_SPEC>;
#[doc = "Field `WRITEDATA` writer - WRITE DATA. When written, an MII Mgmt write cycle is performed using the 16-bit data and the pre-configured PHY and Register addresses from the MII Mgmt Address register (MADR)."]
pub type WRITEDATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<MWTD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:15 - WRITE DATA. When written, an MII Mgmt write cycle is performed using the 16-bit data and the pre-configured PHY and Register addresses from the MII Mgmt Address register (MADR)."]
    #[inline(always)]
    #[must_use]
    pub fn writedata(&mut self) -> WRITEDATA_W<MWTD_SPEC, 0> {
        WRITEDATA_W::new(self)
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
#[doc = "MII Mgmt Write Data register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mwtd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MWTD_SPEC;
impl crate::RegisterSpec for MWTD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mwtd::W`](W) writer structure"]
impl crate::Writable for MWTD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MWTD to value 0"]
impl crate::Resettable for MWTD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
