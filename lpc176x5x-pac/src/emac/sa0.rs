#[doc = "Register `SA0` reader"]
pub type R = crate::R<SA0_SPEC>;
#[doc = "Register `SA0` writer"]
pub type W = crate::W<SA0_SPEC>;
#[doc = "Field `SADDR2` reader - STATION ADDRESS, 2nd octet. This field holds the second octet of the station address."]
pub type SADDR2_R = crate::FieldReader;
#[doc = "Field `SADDR2` writer - STATION ADDRESS, 2nd octet. This field holds the second octet of the station address."]
pub type SADDR2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SADDR1` reader - STATION ADDRESS, 1st octet. This field holds the first octet of the station address."]
pub type SADDR1_R = crate::FieldReader;
#[doc = "Field `SADDR1` writer - STATION ADDRESS, 1st octet. This field holds the first octet of the station address."]
pub type SADDR1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - STATION ADDRESS, 2nd octet. This field holds the second octet of the station address."]
    #[inline(always)]
    pub fn saddr2(&self) -> SADDR2_R {
        SADDR2_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - STATION ADDRESS, 1st octet. This field holds the first octet of the station address."]
    #[inline(always)]
    pub fn saddr1(&self) -> SADDR1_R {
        SADDR1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SA0")
            .field("saddr2", &format_args!("{}", self.saddr2().bits()))
            .field("saddr1", &format_args!("{}", self.saddr1().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<SA0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - STATION ADDRESS, 2nd octet. This field holds the second octet of the station address."]
    #[inline(always)]
    #[must_use]
    pub fn saddr2(&mut self) -> SADDR2_W<SA0_SPEC, 0> {
        SADDR2_W::new(self)
    }
    #[doc = "Bits 8:15 - STATION ADDRESS, 1st octet. This field holds the first octet of the station address."]
    #[inline(always)]
    #[must_use]
    pub fn saddr1(&mut self) -> SADDR1_W<SA0_SPEC, 8> {
        SADDR1_W::new(self)
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
#[doc = "Station Address 0 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sa0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sa0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SA0_SPEC;
impl crate::RegisterSpec for SA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sa0::R`](R) reader structure"]
impl crate::Readable for SA0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sa0::W`](W) writer structure"]
impl crate::Writable for SA0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SA0 to value 0"]
impl crate::Resettable for SA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
