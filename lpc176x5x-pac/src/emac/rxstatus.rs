#[doc = "Register `RXSTATUS` reader"]
pub type R = crate::R<RXSTATUS_SPEC>;
#[doc = "Register `RXSTATUS` writer"]
pub type W = crate::W<RXSTATUS_SPEC>;
#[doc = "Field `RXSTATUS` reader - MSBs of receive status base address."]
pub type RXSTATUS_R = crate::FieldReader<u32>;
#[doc = "Field `RXSTATUS` writer - MSBs of receive status base address."]
pub type RXSTATUS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 29, O, u32>;
impl R {
    #[doc = "Bits 3:31 - MSBs of receive status base address."]
    #[inline(always)]
    pub fn rxstatus(&self) -> RXSTATUS_R {
        RXSTATUS_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXSTATUS")
            .field("rxstatus", &format_args!("{}", self.rxstatus().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<RXSTATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 3:31 - MSBs of receive status base address."]
    #[inline(always)]
    #[must_use]
    pub fn rxstatus(&mut self) -> RXSTATUS_W<RXSTATUS_SPEC, 3> {
        RXSTATUS_W::new(self)
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
#[doc = "Receive status base address register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxstatus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxstatus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXSTATUS_SPEC;
impl crate::RegisterSpec for RXSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxstatus::R`](R) reader structure"]
impl crate::Readable for RXSTATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxstatus::W`](W) writer structure"]
impl crate::Writable for RXSTATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXSTATUS to value 0"]
impl crate::Resettable for RXSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
