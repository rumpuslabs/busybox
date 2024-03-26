#[doc = "Register `CON` writer"]
pub type W = crate::W<CON_SPEC>;
#[doc = "Field `RESP` writer - Reset position counter. When set = 1, resets the position counter to all zeros. Autoclears when the position counter is cleared."]
pub type RESP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESPI` writer - Reset position counter on index. When set = 1, resets the position counter to all zeros once only the first time an index pulse occurs. Autoclears when the position counter is cleared."]
pub type RESPI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESV` writer - Reset velocity. When set = 1, resets the velocity counter to all zeros, reloads the velocity timer, and presets the velocity compare register. Autoclears when the velocity counter is cleared."]
pub type RESV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESI` writer - Reset index counter. When set = 1, resets the index counter to all zeros. Autoclears when the index counter is cleared."]
pub type RESI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CON_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Reset position counter. When set = 1, resets the position counter to all zeros. Autoclears when the position counter is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn resp(&mut self) -> RESP_W<CON_SPEC, 0> {
        RESP_W::new(self)
    }
    #[doc = "Bit 1 - Reset position counter on index. When set = 1, resets the position counter to all zeros once only the first time an index pulse occurs. Autoclears when the position counter is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn respi(&mut self) -> RESPI_W<CON_SPEC, 1> {
        RESPI_W::new(self)
    }
    #[doc = "Bit 2 - Reset velocity. When set = 1, resets the velocity counter to all zeros, reloads the velocity timer, and presets the velocity compare register. Autoclears when the velocity counter is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn resv(&mut self) -> RESV_W<CON_SPEC, 2> {
        RESV_W::new(self)
    }
    #[doc = "Bit 3 - Reset index counter. When set = 1, resets the index counter to all zeros. Autoclears when the index counter is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn resi(&mut self) -> RESI_W<CON_SPEC, 3> {
        RESI_W::new(self)
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
#[doc = "Control register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`con::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CON_SPEC;
impl crate::RegisterSpec for CON_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`con::W`](W) writer structure"]
impl crate::Writable for CON_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CON to value 0"]
impl crate::Resettable for CON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
