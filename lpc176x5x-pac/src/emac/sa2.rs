#[doc = "Register `SA2` reader"]
pub type R = crate::R<SA2_SPEC>;
#[doc = "Register `SA2` writer"]
pub type W = crate::W<SA2_SPEC>;
#[doc = "Field `SADDR6` reader - STATION ADDRESS, 6th octet. This field holds the sixth octet of the station address."]
pub type SADDR6_R = crate::FieldReader;
#[doc = "Field `SADDR6` writer - STATION ADDRESS, 6th octet. This field holds the sixth octet of the station address."]
pub type SADDR6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SADDR5` reader - STATION ADDRESS, 5th octet. This field holds the fifth octet of the station address."]
pub type SADDR5_R = crate::FieldReader;
#[doc = "Field `SADDR5` writer - STATION ADDRESS, 5th octet. This field holds the fifth octet of the station address."]
pub type SADDR5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - STATION ADDRESS, 6th octet. This field holds the sixth octet of the station address."]
    #[inline(always)]
    pub fn saddr6(&self) -> SADDR6_R {
        SADDR6_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - STATION ADDRESS, 5th octet. This field holds the fifth octet of the station address."]
    #[inline(always)]
    pub fn saddr5(&self) -> SADDR5_R {
        SADDR5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SA2")
            .field("saddr6", &format_args!("{}", self.saddr6().bits()))
            .field("saddr5", &format_args!("{}", self.saddr5().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<SA2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - STATION ADDRESS, 6th octet. This field holds the sixth octet of the station address."]
    #[inline(always)]
    #[must_use]
    pub fn saddr6(&mut self) -> SADDR6_W<SA2_SPEC, 0> {
        SADDR6_W::new(self)
    }
    #[doc = "Bits 8:15 - STATION ADDRESS, 5th octet. This field holds the fifth octet of the station address."]
    #[inline(always)]
    #[must_use]
    pub fn saddr5(&mut self) -> SADDR5_W<SA2_SPEC, 8> {
        SADDR5_W::new(self)
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
#[doc = "Station Address 2 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sa2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sa2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SA2_SPEC;
impl crate::RegisterSpec for SA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sa2::R`](R) reader structure"]
impl crate::Readable for SA2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sa2::W`](W) writer structure"]
impl crate::Writable for SA2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SA2 to value 0"]
impl crate::Resettable for SA2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
