#[doc = "Register `MADR` reader"]
pub type R = crate::R<MADR_SPEC>;
#[doc = "Register `MADR` writer"]
pub type W = crate::W<MADR_SPEC>;
#[doc = "Field `REGADDR` reader - REGISTER ADDRESS. This field represents the 5-bit Register Address field of Mgmt cycles. Up to 32 registers can be accessed."]
pub type REGADDR_R = crate::FieldReader;
#[doc = "Field `REGADDR` writer - REGISTER ADDRESS. This field represents the 5-bit Register Address field of Mgmt cycles. Up to 32 registers can be accessed."]
pub type REGADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `PHYADDR` reader - PHY ADDRESS. This field represents the 5-bit PHY Address field of Mgmt cycles. Up to 31 PHYs can be addressed (0 is reserved)."]
pub type PHYADDR_R = crate::FieldReader;
#[doc = "Field `PHYADDR` writer - PHY ADDRESS. This field represents the 5-bit PHY Address field of Mgmt cycles. Up to 31 PHYs can be addressed (0 is reserved)."]
pub type PHYADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - REGISTER ADDRESS. This field represents the 5-bit Register Address field of Mgmt cycles. Up to 32 registers can be accessed."]
    #[inline(always)]
    pub fn regaddr(&self) -> REGADDR_R {
        REGADDR_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - PHY ADDRESS. This field represents the 5-bit PHY Address field of Mgmt cycles. Up to 31 PHYs can be addressed (0 is reserved)."]
    #[inline(always)]
    pub fn phyaddr(&self) -> PHYADDR_R {
        PHYADDR_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MADR")
            .field("regaddr", &format_args!("{}", self.regaddr().bits()))
            .field("phyaddr", &format_args!("{}", self.phyaddr().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<MADR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4 - REGISTER ADDRESS. This field represents the 5-bit Register Address field of Mgmt cycles. Up to 32 registers can be accessed."]
    #[inline(always)]
    #[must_use]
    pub fn regaddr(&mut self) -> REGADDR_W<MADR_SPEC, 0> {
        REGADDR_W::new(self)
    }
    #[doc = "Bits 8:12 - PHY ADDRESS. This field represents the 5-bit PHY Address field of Mgmt cycles. Up to 31 PHYs can be addressed (0 is reserved)."]
    #[inline(always)]
    #[must_use]
    pub fn phyaddr(&mut self) -> PHYADDR_W<MADR_SPEC, 8> {
        PHYADDR_W::new(self)
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
#[doc = "MII Mgmt Address register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`madr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`madr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MADR_SPEC;
impl crate::RegisterSpec for MADR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`madr::R`](R) reader structure"]
impl crate::Readable for MADR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`madr::W`](W) writer structure"]
impl crate::Writable for MADR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MADR to value 0"]
impl crate::Resettable for MADR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
