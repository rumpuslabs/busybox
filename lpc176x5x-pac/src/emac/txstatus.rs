#[doc = "Register `TXSTATUS` reader"]
pub type R = crate::R<TXSTATUS_SPEC>;
#[doc = "Register `TXSTATUS` writer"]
pub type W = crate::W<TXSTATUS_SPEC>;
#[doc = "Field `TXSTAT` reader - TxStatus. MSBs of transmit status base address."]
pub type TXSTAT_R = crate::FieldReader<u32>;
#[doc = "Field `TXSTAT` writer - TxStatus. MSBs of transmit status base address."]
pub type TXSTAT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 30, O, u32>;
impl R {
    #[doc = "Bits 2:31 - TxStatus. MSBs of transmit status base address."]
    #[inline(always)]
    pub fn txstat(&self) -> TXSTAT_R {
        TXSTAT_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXSTATUS")
            .field("txstat", &format_args!("{}", self.txstat().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<TXSTATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 2:31 - TxStatus. MSBs of transmit status base address."]
    #[inline(always)]
    #[must_use]
    pub fn txstat(&mut self) -> TXSTAT_W<TXSTATUS_SPEC, 2> {
        TXSTAT_W::new(self)
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
#[doc = "Transmit status base address register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txstatus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txstatus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXSTATUS_SPEC;
impl crate::RegisterSpec for TXSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txstatus::R`](R) reader structure"]
impl crate::Readable for TXSTATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txstatus::W`](W) writer structure"]
impl crate::Writable for TXSTATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXSTATUS to value 0"]
impl crate::Resettable for TXSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
