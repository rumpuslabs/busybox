#[doc = "Register `CIIR` reader"]
pub type R = crate::R<CIIR_SPEC>;
#[doc = "Register `CIIR` writer"]
pub type W = crate::W<CIIR_SPEC>;
#[doc = "Field `IMSEC` reader - When 1, an increment of the Second value generates an interrupt."]
pub type IMSEC_R = crate::BitReader;
#[doc = "Field `IMSEC` writer - When 1, an increment of the Second value generates an interrupt."]
pub type IMSEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IMMIN` reader - When 1, an increment of the Minute value generates an interrupt."]
pub type IMMIN_R = crate::BitReader;
#[doc = "Field `IMMIN` writer - When 1, an increment of the Minute value generates an interrupt."]
pub type IMMIN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IMHOUR` reader - When 1, an increment of the Hour value generates an interrupt."]
pub type IMHOUR_R = crate::BitReader;
#[doc = "Field `IMHOUR` writer - When 1, an increment of the Hour value generates an interrupt."]
pub type IMHOUR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IMDOM` reader - When 1, an increment of the Day of Month value generates an interrupt."]
pub type IMDOM_R = crate::BitReader;
#[doc = "Field `IMDOM` writer - When 1, an increment of the Day of Month value generates an interrupt."]
pub type IMDOM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IMDOW` reader - When 1, an increment of the Day of Week value generates an interrupt."]
pub type IMDOW_R = crate::BitReader;
#[doc = "Field `IMDOW` writer - When 1, an increment of the Day of Week value generates an interrupt."]
pub type IMDOW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IMDOY` reader - When 1, an increment of the Day of Year value generates an interrupt."]
pub type IMDOY_R = crate::BitReader;
#[doc = "Field `IMDOY` writer - When 1, an increment of the Day of Year value generates an interrupt."]
pub type IMDOY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IMMON` reader - When 1, an increment of the Month value generates an interrupt."]
pub type IMMON_R = crate::BitReader;
#[doc = "Field `IMMON` writer - When 1, an increment of the Month value generates an interrupt."]
pub type IMMON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IMYEAR` reader - When 1, an increment of the Year value generates an interrupt."]
pub type IMYEAR_R = crate::BitReader;
#[doc = "Field `IMYEAR` writer - When 1, an increment of the Year value generates an interrupt."]
pub type IMYEAR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - When 1, an increment of the Second value generates an interrupt."]
    #[inline(always)]
    pub fn imsec(&self) -> IMSEC_R {
        IMSEC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When 1, an increment of the Minute value generates an interrupt."]
    #[inline(always)]
    pub fn immin(&self) -> IMMIN_R {
        IMMIN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When 1, an increment of the Hour value generates an interrupt."]
    #[inline(always)]
    pub fn imhour(&self) -> IMHOUR_R {
        IMHOUR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When 1, an increment of the Day of Month value generates an interrupt."]
    #[inline(always)]
    pub fn imdom(&self) -> IMDOM_R {
        IMDOM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When 1, an increment of the Day of Week value generates an interrupt."]
    #[inline(always)]
    pub fn imdow(&self) -> IMDOW_R {
        IMDOW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When 1, an increment of the Day of Year value generates an interrupt."]
    #[inline(always)]
    pub fn imdoy(&self) -> IMDOY_R {
        IMDOY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When 1, an increment of the Month value generates an interrupt."]
    #[inline(always)]
    pub fn immon(&self) -> IMMON_R {
        IMMON_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When 1, an increment of the Year value generates an interrupt."]
    #[inline(always)]
    pub fn imyear(&self) -> IMYEAR_R {
        IMYEAR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CIIR")
            .field("imsec", &format_args!("{}", self.imsec().bit()))
            .field("immin", &format_args!("{}", self.immin().bit()))
            .field("imhour", &format_args!("{}", self.imhour().bit()))
            .field("imdom", &format_args!("{}", self.imdom().bit()))
            .field("imdow", &format_args!("{}", self.imdow().bit()))
            .field("imdoy", &format_args!("{}", self.imdoy().bit()))
            .field("immon", &format_args!("{}", self.immon().bit()))
            .field("imyear", &format_args!("{}", self.imyear().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CIIR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - When 1, an increment of the Second value generates an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn imsec(&mut self) -> IMSEC_W<CIIR_SPEC, 0> {
        IMSEC_W::new(self)
    }
    #[doc = "Bit 1 - When 1, an increment of the Minute value generates an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn immin(&mut self) -> IMMIN_W<CIIR_SPEC, 1> {
        IMMIN_W::new(self)
    }
    #[doc = "Bit 2 - When 1, an increment of the Hour value generates an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn imhour(&mut self) -> IMHOUR_W<CIIR_SPEC, 2> {
        IMHOUR_W::new(self)
    }
    #[doc = "Bit 3 - When 1, an increment of the Day of Month value generates an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn imdom(&mut self) -> IMDOM_W<CIIR_SPEC, 3> {
        IMDOM_W::new(self)
    }
    #[doc = "Bit 4 - When 1, an increment of the Day of Week value generates an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn imdow(&mut self) -> IMDOW_W<CIIR_SPEC, 4> {
        IMDOW_W::new(self)
    }
    #[doc = "Bit 5 - When 1, an increment of the Day of Year value generates an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn imdoy(&mut self) -> IMDOY_W<CIIR_SPEC, 5> {
        IMDOY_W::new(self)
    }
    #[doc = "Bit 6 - When 1, an increment of the Month value generates an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn immon(&mut self) -> IMMON_W<CIIR_SPEC, 6> {
        IMMON_W::new(self)
    }
    #[doc = "Bit 7 - When 1, an increment of the Year value generates an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn imyear(&mut self) -> IMYEAR_W<CIIR_SPEC, 7> {
        IMYEAR_W::new(self)
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
#[doc = "Counter Increment Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ciir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ciir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIIR_SPEC;
impl crate::RegisterSpec for CIIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ciir::R`](R) reader structure"]
impl crate::Readable for CIIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ciir::W`](W) writer structure"]
impl crate::Writable for CIIR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CIIR to value 0"]
impl crate::Resettable for CIIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
