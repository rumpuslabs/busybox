#[doc = "Register `AMR` reader"]
pub type R = crate::R<AMR_SPEC>;
#[doc = "Register `AMR` writer"]
pub type W = crate::W<AMR_SPEC>;
#[doc = "Field `AMRSEC` reader - When 1, the Second value is not compared for the alarm."]
pub type AMRSEC_R = crate::BitReader;
#[doc = "Field `AMRSEC` writer - When 1, the Second value is not compared for the alarm."]
pub type AMRSEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AMRMIN` reader - When 1, the Minutes value is not compared for the alarm."]
pub type AMRMIN_R = crate::BitReader;
#[doc = "Field `AMRMIN` writer - When 1, the Minutes value is not compared for the alarm."]
pub type AMRMIN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AMRHOUR` reader - When 1, the Hour value is not compared for the alarm."]
pub type AMRHOUR_R = crate::BitReader;
#[doc = "Field `AMRHOUR` writer - When 1, the Hour value is not compared for the alarm."]
pub type AMRHOUR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AMRDOM` reader - When 1, the Day of Month value is not compared for the alarm."]
pub type AMRDOM_R = crate::BitReader;
#[doc = "Field `AMRDOM` writer - When 1, the Day of Month value is not compared for the alarm."]
pub type AMRDOM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AMRDOW` reader - When 1, the Day of Week value is not compared for the alarm."]
pub type AMRDOW_R = crate::BitReader;
#[doc = "Field `AMRDOW` writer - When 1, the Day of Week value is not compared for the alarm."]
pub type AMRDOW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AMRDOY` reader - When 1, the Day of Year value is not compared for the alarm."]
pub type AMRDOY_R = crate::BitReader;
#[doc = "Field `AMRDOY` writer - When 1, the Day of Year value is not compared for the alarm."]
pub type AMRDOY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AMRMON` reader - When 1, the Month value is not compared for the alarm."]
pub type AMRMON_R = crate::BitReader;
#[doc = "Field `AMRMON` writer - When 1, the Month value is not compared for the alarm."]
pub type AMRMON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AMRYEAR` reader - When 1, the Year value is not compared for the alarm."]
pub type AMRYEAR_R = crate::BitReader;
#[doc = "Field `AMRYEAR` writer - When 1, the Year value is not compared for the alarm."]
pub type AMRYEAR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - When 1, the Second value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrsec(&self) -> AMRSEC_R {
        AMRSEC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When 1, the Minutes value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrmin(&self) -> AMRMIN_R {
        AMRMIN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When 1, the Hour value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrhour(&self) -> AMRHOUR_R {
        AMRHOUR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When 1, the Day of Month value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrdom(&self) -> AMRDOM_R {
        AMRDOM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When 1, the Day of Week value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrdow(&self) -> AMRDOW_R {
        AMRDOW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When 1, the Day of Year value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrdoy(&self) -> AMRDOY_R {
        AMRDOY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When 1, the Month value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrmon(&self) -> AMRMON_R {
        AMRMON_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When 1, the Year value is not compared for the alarm."]
    #[inline(always)]
    pub fn amryear(&self) -> AMRYEAR_R {
        AMRYEAR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AMR")
            .field("amrsec", &format_args!("{}", self.amrsec().bit()))
            .field("amrmin", &format_args!("{}", self.amrmin().bit()))
            .field("amrhour", &format_args!("{}", self.amrhour().bit()))
            .field("amrdom", &format_args!("{}", self.amrdom().bit()))
            .field("amrdow", &format_args!("{}", self.amrdow().bit()))
            .field("amrdoy", &format_args!("{}", self.amrdoy().bit()))
            .field("amrmon", &format_args!("{}", self.amrmon().bit()))
            .field("amryear", &format_args!("{}", self.amryear().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<AMR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - When 1, the Second value is not compared for the alarm."]
    #[inline(always)]
    #[must_use]
    pub fn amrsec(&mut self) -> AMRSEC_W<AMR_SPEC, 0> {
        AMRSEC_W::new(self)
    }
    #[doc = "Bit 1 - When 1, the Minutes value is not compared for the alarm."]
    #[inline(always)]
    #[must_use]
    pub fn amrmin(&mut self) -> AMRMIN_W<AMR_SPEC, 1> {
        AMRMIN_W::new(self)
    }
    #[doc = "Bit 2 - When 1, the Hour value is not compared for the alarm."]
    #[inline(always)]
    #[must_use]
    pub fn amrhour(&mut self) -> AMRHOUR_W<AMR_SPEC, 2> {
        AMRHOUR_W::new(self)
    }
    #[doc = "Bit 3 - When 1, the Day of Month value is not compared for the alarm."]
    #[inline(always)]
    #[must_use]
    pub fn amrdom(&mut self) -> AMRDOM_W<AMR_SPEC, 3> {
        AMRDOM_W::new(self)
    }
    #[doc = "Bit 4 - When 1, the Day of Week value is not compared for the alarm."]
    #[inline(always)]
    #[must_use]
    pub fn amrdow(&mut self) -> AMRDOW_W<AMR_SPEC, 4> {
        AMRDOW_W::new(self)
    }
    #[doc = "Bit 5 - When 1, the Day of Year value is not compared for the alarm."]
    #[inline(always)]
    #[must_use]
    pub fn amrdoy(&mut self) -> AMRDOY_W<AMR_SPEC, 5> {
        AMRDOY_W::new(self)
    }
    #[doc = "Bit 6 - When 1, the Month value is not compared for the alarm."]
    #[inline(always)]
    #[must_use]
    pub fn amrmon(&mut self) -> AMRMON_W<AMR_SPEC, 6> {
        AMRMON_W::new(self)
    }
    #[doc = "Bit 7 - When 1, the Year value is not compared for the alarm."]
    #[inline(always)]
    #[must_use]
    pub fn amryear(&mut self) -> AMRYEAR_W<AMR_SPEC, 7> {
        AMRYEAR_W::new(self)
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
#[doc = "Alarm Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`amr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`amr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AMR_SPEC;
impl crate::RegisterSpec for AMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`amr::R`](R) reader structure"]
impl crate::Readable for AMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`amr::W`](W) writer structure"]
impl crate::Writable for AMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AMR to value 0"]
impl crate::Resettable for AMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
