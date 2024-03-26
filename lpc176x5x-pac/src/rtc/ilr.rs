#[doc = "Register `ILR` reader"]
pub type R = crate::R<ILR_SPEC>;
#[doc = "Register `ILR` writer"]
pub type W = crate::W<ILR_SPEC>;
#[doc = "Field `RTCCIF` reader - When one, the Counter Increment Interrupt block generated an interrupt. Writing a one to this bit location clears the counter increment interrupt."]
pub type RTCCIF_R = crate::BitReader;
#[doc = "Field `RTCCIF` writer - When one, the Counter Increment Interrupt block generated an interrupt. Writing a one to this bit location clears the counter increment interrupt."]
pub type RTCCIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTCALF` reader - When one, the alarm registers generated an interrupt. Writing a one to this bit location clears the alarm interrupt."]
pub type RTCALF_R = crate::BitReader;
#[doc = "Field `RTCALF` writer - When one, the alarm registers generated an interrupt. Writing a one to this bit location clears the alarm interrupt."]
pub type RTCALF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - When one, the Counter Increment Interrupt block generated an interrupt. Writing a one to this bit location clears the counter increment interrupt."]
    #[inline(always)]
    pub fn rtccif(&self) -> RTCCIF_R {
        RTCCIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When one, the alarm registers generated an interrupt. Writing a one to this bit location clears the alarm interrupt."]
    #[inline(always)]
    pub fn rtcalf(&self) -> RTCALF_R {
        RTCALF_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ILR")
            .field("rtccif", &format_args!("{}", self.rtccif().bit()))
            .field("rtcalf", &format_args!("{}", self.rtcalf().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<ILR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - When one, the Counter Increment Interrupt block generated an interrupt. Writing a one to this bit location clears the counter increment interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rtccif(&mut self) -> RTCCIF_W<ILR_SPEC, 0> {
        RTCCIF_W::new(self)
    }
    #[doc = "Bit 1 - When one, the alarm registers generated an interrupt. Writing a one to this bit location clears the alarm interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rtcalf(&mut self) -> RTCALF_W<ILR_SPEC, 1> {
        RTCALF_W::new(self)
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
#[doc = "Interrupt Location Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ilr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ilr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ILR_SPEC;
impl crate::RegisterSpec for ILR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ilr::R`](R) reader structure"]
impl crate::Readable for ILR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ilr::W`](W) writer structure"]
impl crate::Writable for ILR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ILR to value 0"]
impl crate::Resettable for ILR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
