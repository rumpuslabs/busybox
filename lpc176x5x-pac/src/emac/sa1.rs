#[doc = "Register `SA1` reader"]
pub type R = crate::R<SA1_SPEC>;
#[doc = "Register `SA1` writer"]
pub type W = crate::W<SA1_SPEC>;
#[doc = "Field `SADDR4` reader - STATION ADDRESS, 4th octet. This field holds the fourth octet of the station address."]
pub type SADDR4_R = crate::FieldReader;
#[doc = "Field `SADDR4` writer - STATION ADDRESS, 4th octet. This field holds the fourth octet of the station address."]
pub type SADDR4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SADDR3` reader - STATION ADDRESS, 3rd octet. This field holds the third octet of the station address."]
pub type SADDR3_R = crate::FieldReader;
#[doc = "Field `SADDR3` writer - STATION ADDRESS, 3rd octet. This field holds the third octet of the station address."]
pub type SADDR3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - STATION ADDRESS, 4th octet. This field holds the fourth octet of the station address."]
    #[inline(always)]
    pub fn saddr4(&self) -> SADDR4_R {
        SADDR4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - STATION ADDRESS, 3rd octet. This field holds the third octet of the station address."]
    #[inline(always)]
    pub fn saddr3(&self) -> SADDR3_R {
        SADDR3_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SA1")
            .field("saddr4", &format_args!("{}", self.saddr4().bits()))
            .field("saddr3", &format_args!("{}", self.saddr3().bits()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<SA1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - STATION ADDRESS, 4th octet. This field holds the fourth octet of the station address."]
    #[inline(always)]
    #[must_use]
    pub fn saddr4(&mut self) -> SADDR4_W<SA1_SPEC, 0> {
        SADDR4_W::new(self)
    }
    #[doc = "Bits 8:15 - STATION ADDRESS, 3rd octet. This field holds the third octet of the station address."]
    #[inline(always)]
    #[must_use]
    pub fn saddr3(&mut self) -> SADDR3_W<SA1_SPEC, 8> {
        SADDR3_W::new(self)
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
#[doc = "Station Address 1 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sa1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sa1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SA1_SPEC;
impl crate::RegisterSpec for SA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sa1::R`](R) reader structure"]
impl crate::Readable for SA1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sa1::W`](W) writer structure"]
impl crate::Writable for SA1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SA1 to value 0"]
impl crate::Resettable for SA1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
