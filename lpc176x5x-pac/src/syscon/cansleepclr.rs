#[doc = "Register `CANSLEEPCLR` reader"]
pub type R = crate::R<CANSLEEPCLR_SPEC>;
#[doc = "Register `CANSLEEPCLR` writer"]
pub type W = crate::W<CANSLEEPCLR_SPEC>;
#[doc = "Field `CAN1SLEEP` reader - Sleep status and control for CAN channel 1. Read: when 1, indicates that CAN channel 1 is in the sleep mode. Write: writing a 1 causes clocks to be restored to CAN channel 1."]
pub type CAN1SLEEP_R = crate::BitReader;
#[doc = "Field `CAN1SLEEP` writer - Sleep status and control for CAN channel 1. Read: when 1, indicates that CAN channel 1 is in the sleep mode. Write: writing a 1 causes clocks to be restored to CAN channel 1."]
pub type CAN1SLEEP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAN2SLEEP` reader - Sleep status and control for CAN channel 2. Read: when 1, indicates that CAN channel 2 is in the sleep mode. Write: writing a 1 causes clocks to be restored to CAN channel 2."]
pub type CAN2SLEEP_R = crate::BitReader;
#[doc = "Field `CAN2SLEEP` writer - Sleep status and control for CAN channel 2. Read: when 1, indicates that CAN channel 2 is in the sleep mode. Write: writing a 1 causes clocks to be restored to CAN channel 2."]
pub type CAN2SLEEP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - Sleep status and control for CAN channel 1. Read: when 1, indicates that CAN channel 1 is in the sleep mode. Write: writing a 1 causes clocks to be restored to CAN channel 1."]
    #[inline(always)]
    pub fn can1sleep(&self) -> CAN1SLEEP_R {
        CAN1SLEEP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sleep status and control for CAN channel 2. Read: when 1, indicates that CAN channel 2 is in the sleep mode. Write: writing a 1 causes clocks to be restored to CAN channel 2."]
    #[inline(always)]
    pub fn can2sleep(&self) -> CAN2SLEEP_R {
        CAN2SLEEP_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CANSLEEPCLR")
            .field("can1sleep", &format_args!("{}", self.can1sleep().bit()))
            .field("can2sleep", &format_args!("{}", self.can2sleep().bit()))
            .finish()
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CANSLEEPCLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 1 - Sleep status and control for CAN channel 1. Read: when 1, indicates that CAN channel 1 is in the sleep mode. Write: writing a 1 causes clocks to be restored to CAN channel 1."]
    #[inline(always)]
    #[must_use]
    pub fn can1sleep(&mut self) -> CAN1SLEEP_W<CANSLEEPCLR_SPEC, 1> {
        CAN1SLEEP_W::new(self)
    }
    #[doc = "Bit 2 - Sleep status and control for CAN channel 2. Read: when 1, indicates that CAN channel 2 is in the sleep mode. Write: writing a 1 causes clocks to be restored to CAN channel 2."]
    #[inline(always)]
    #[must_use]
    pub fn can2sleep(&mut self) -> CAN2SLEEP_W<CANSLEEPCLR_SPEC, 2> {
        CAN2SLEEP_W::new(self)
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
#[doc = "Allows clearing the current CAN channel sleep state as well as reading that state.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cansleepclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cansleepclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CANSLEEPCLR_SPEC;
impl crate::RegisterSpec for CANSLEEPCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cansleepclr::R`](R) reader structure"]
impl crate::Readable for CANSLEEPCLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cansleepclr::W`](W) writer structure"]
impl crate::Writable for CANSLEEPCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CANSLEEPCLR to value 0"]
impl crate::Resettable for CANSLEEPCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
